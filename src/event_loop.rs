use std::io::{Read, Write};
use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::mpsc::{Sender, SyncSender};
use uuid::Uuid;
use state::State;
use mio;
use mio::{Token, Handler, PollOpt, EventSet};
use mio::tcp::{TcpListener, TcpStream};
use super::frame::{ReadState, WriteState};

const ACCEPTOR: Token = Token(0);

#[derive(Debug, Clone)]
/// Data Messages can be dropped when the event loop detects overload
pub enum OutDataMsg{
    TcpMsg(Token, Vec<u8>),
    Tick(Uuid)
}

#[derive(Debug, Clone)]
/// Control messages are always sent, even when in overload. They are required for cross thread
/// bookkeeping.
pub enum OutControlMsg {
    NewSock(Token),
    Deregister(Token)
}

#[derive(Debug, Clone)]
pub enum IncomingMsg {
    Connect(Token, SocketAddr),
    Deregister(Token, String),
    WireMsg(Token, Vec<u8>),
    // Timeout is (timeout id, timeout in ms)
    SetTimeout(Uuid, u64),
    #[allow(dead_code)]
    CancelTimeout(Uuid),
    #[allow(dead_code)]
    Stop
}

pub struct EventLoop {
    addr: SocketAddr,
    event_loop: mio::EventLoop<Context>
}

impl EventLoop {
    pub fn new(listen_address: SocketAddr, queue_size: usize) -> EventLoop {
        let mut config = mio::EventLoopConfig::new();
        config.notify_capacity(queue_size);
        EventLoop {
            addr: listen_address,
            event_loop: mio::EventLoop::configured(config).unwrap()
        }
    }

    /// This call will block the current thread
    pub fn run(&mut self,
               state: State,
               data_tx: SyncSender<OutDataMsg>,
               control_tx: Sender<OutControlMsg>) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        self.event_loop.register(&listener, ACCEPTOR,
                                 EventSet::readable(),
                                 PollOpt::edge()).unwrap();
        let mut ctx = Context::new(listener, state, data_tx, control_tx);
        self.event_loop.run(&mut ctx).unwrap();
    }

    pub fn sender(&self) -> mio::Sender<IncomingMsg> {
        self.event_loop.channel()
    }
}

struct Conn {
    sock: TcpStream,
    // We use an option simply so we can steal the inner value and own it elsewhere.
    // The value will never actually be `None` for either ReadState or WriteState
    read_state: Option<ReadState>,
    write_state: Option<WriteState>
}

impl Conn {
    fn new(sock: TcpStream) -> Conn {
        Conn {
            sock: sock,
            read_state: Some(ReadState::new()),
            write_state: Some(WriteState::new())
        }
    }
}

struct Context {
    state: State,
    overloaded: bool,
    overload_timer_id: Option<Uuid>,
    timeouts: HashMap<Uuid, u64>,
    conns: HashMap<Token, Conn>,
    listener: TcpListener,
    data_tx: SyncSender<OutDataMsg>,
    control_tx: Sender<OutControlMsg>
}

/// EventLoop Callbacks and Associated Types
impl Handler for Context {
    type Timeout = Uuid;
    type Message = IncomingMsg;

    fn ready(&mut self, event_loop: &mut mio::EventLoop<Context>, token: Token, event: EventSet)
    {
        if event.is_readable() {
            match token {
                ACCEPTOR => { self.accept(event_loop); }
                _ => {
                    if self.overloaded {
                        // We just disconnect any client that tries to send requests while we are in
                        // overload.
                        return self.deregister(event_loop, token, "Overloaded".to_string());
                    }
                    match self.read(event_loop, token) {
                        Ok(()) => (),
                        Err(e) => {
                            println!("Error reading from vr socket with token: {:?}: on {:?}, {}", token, self.state.members.me.name, e);
                            self.deregister(event_loop, token, e);
                        }

                    }
                }
            }
        }

        // We still want to allow writes in overload, since that doesn't add any actual load to the
        // dispatcher/fsms. It's just outputting completed operations.
        if event.is_writable() {
            if let Err(err) = self.write(event_loop, token) {
                println!("Got a write error: {} for token {:?}", err, token);
                self.deregister(event_loop, token, err);
            }

        }
    }

    fn notify(&mut self, event_loop: &mut mio::EventLoop<Context>, msg: IncomingMsg) {
        match msg {
            IncomingMsg::Deregister(token, err) => self.deregister(event_loop, token, err),
            IncomingMsg::WireMsg(token, msg) => self.push_outgoing(event_loop, token, msg),
            IncomingMsg::Connect(token, addr) => self.connect(event_loop, token, addr),
            IncomingMsg::Stop => event_loop.shutdown(),
            IncomingMsg::SetTimeout(id, timeout) => {
                event_loop.timeout_ms(id, timeout).unwrap();
                self.timeouts.insert(id, timeout);
            },
            IncomingMsg::CancelTimeout(id) => {
                // Note that it's still possible a tick will come after the removal
                self.timeouts.remove(&id);
            }
        }
    }

    fn timeout(&mut self, event_loop: &mut mio::EventLoop<Context>, timeout_id: Uuid) {
        if let Some(overload_timer_id) = self.overload_timer_id {
            if timeout_id == overload_timer_id {
                return self.clear_overload();
            }
        }
        // Satisfy the borrow checker with this stupid variable
        let mut overloaded = false;
        if let Some(timeout) = self.timeouts.get(&timeout_id) {
            if !self.overloaded {
                if let Err(_) = self.data_tx.try_send(OutDataMsg::Tick(timeout_id)) {
                    overloaded = true;
                }
            }
            event_loop.timeout_ms(timeout_id, *timeout).unwrap();
        }
        if overloaded { self.set_overload(event_loop); }
    }
}

impl Context {
    fn new(listener: TcpListener,
           state: State,
           data_tx: SyncSender<OutDataMsg>,
           control_tx: Sender<OutControlMsg>) -> Context {
        Context {
            state: state,
            overloaded: false,
            overload_timer_id: None,
            timeouts: HashMap::new(),
            conns: HashMap::new(),
            listener: listener,
            data_tx: data_tx,
            control_tx: control_tx
        }
    }

    fn accept(&mut self, event_loop: &mut mio::EventLoop<Context>) {
        // Don't accept any connections while in overload
        if self.overloaded { return; }
        match self.listener.accept() {
            Ok(None) => (), // EWOULDBLOCK
            Ok(Some((sock, _))) => {
                let token = self.state.next_token();
                self.register(event_loop, token, sock);
            },
            Err(err) => println!("Error accepting connection: {}", err)
        }
    }

    fn set_overload(&mut self, event_loop: &mut mio::EventLoop<Context>) {
        self.overloaded = true;
        self.overload_timer_id = Some(Uuid::new_v4());
        event_loop.timeout_ms(self.overload_timer_id.as_ref().unwrap().clone(), 1000).unwrap();;
    }

    fn clear_overload(&mut self) {
        self.overload_timer_id = None;
        self.overloaded = false;
    }

    fn connect(&mut self, event_loop: &mut mio::EventLoop<Context>, token: Token, addr: SocketAddr) {
        if self.overloaded {
            return self.control_tx.send(OutControlMsg::Deregister(token)).unwrap();
        }
        match TcpStream::connect(&addr) {
            Ok(sock) => self.register(event_loop, token, sock),
            Err(_e) => self.control_tx.send(OutControlMsg::Deregister(token)).unwrap()
        }
    }

    fn read(&mut self, event_loop: &mut mio::EventLoop<Context>, token: Token) -> Result<(), String> {
        let res = match self.conns.get_mut(&token) {
            Some(ref mut conn) => {
                let read_state = conn.read_state.take().unwrap();
                match read_state.read(&mut conn.sock) {
                    (new_read_state, Ok(r)) => {
                        event_loop.reregister(&conn.sock,
                                              token,
                                              EventSet::readable(),
                                              PollOpt::edge() | PollOpt::oneshot()).unwrap();
                        conn.read_state = Some(new_read_state);
                        r
                    },
                    (_, Err(e)) => {
                        return Err(format!("{}", e))
                    }
                }
            },
            None => None
        };
        if let Some(buf) = res {
            if let Err(_) = self.data_tx.try_send(OutDataMsg::TcpMsg(token, buf)) {
                self.deregister(event_loop, token, "Overloaded".to_string());
                self.set_overload(event_loop);
            }

        }
        Ok(())
    }

    fn push_outgoing(&mut self, event_loop: &mut mio::EventLoop<Context>, token: Token, msg: Vec<u8>) {

        if let Some(ref mut conn) = self.conns.get_mut(&token) {
            let write_state = conn.write_state.take().unwrap();
            let new_write_state = write_state.push(msg);
            conn.write_state = Some(new_write_state);
            event_loop.reregister(&conn.sock,
                                  token,
                                  EventSet::writable() | EventSet::readable(),
                                  PollOpt::edge() | PollOpt::oneshot()).unwrap();
        }
    }

    fn write(&mut self, event_loop: &mut mio::EventLoop<Context>, token: Token)
        -> Result<(), String> {
        // It's possible we already deregistered and one last write gets triggered
        self.conns.get_mut(&token).map(|ref mut conn| {
            let write_state = conn.write_state.take().unwrap();
            match write_state.write(&mut conn.sock) {
                Ok((true, new_write_state)) => {
                    event_loop.reregister(&conn.sock,
                                          token,
                                          EventSet::writable() | EventSet::readable(),
                                          PollOpt::edge() | PollOpt::oneshot()).unwrap();
                    conn.write_state = Some(new_write_state);
                    return Ok(())
                },
                Ok((false, new_write_state)) => {
                    event_loop.reregister(&conn.sock,
                                          token,
                                          EventSet::readable(),
                                          PollOpt::edge() | PollOpt::oneshot()).unwrap();
                    conn.write_state = Some(new_write_state);
                    return Ok(());
                },
                Err(e) => return Err(e)
            }
        });
        Ok(())
    }

    fn register(&mut self, event_loop: &mut mio::EventLoop<Context>, token: Token, sock: TcpStream) {
        self.control_tx.send(OutControlMsg::NewSock(token)).unwrap();
        event_loop.register(&sock, token,
                            EventSet::readable() | EventSet::writable(),
                            PollOpt::edge() | PollOpt::oneshot()).unwrap();
        let conn = Conn::new(sock);
        self.conns.insert(token, conn);
    }

    fn deregister(&mut self, event_loop: &mut mio::EventLoop<Context>, token: Token, err: String) {
        if let Some(conn) = self.conns.remove(&token) {
            let _ = event_loop.deregister(&conn.sock);
            println!("Deregistered vr socket for token {:?} with error: {} on {}", token, err,
                     self.state.members.me.name);
            self.control_tx.send(OutControlMsg::Deregister(token)).unwrap();
        } else {
            println!("Error: Tried to deregister a token with no corresponding socket");
        }
    }
}

