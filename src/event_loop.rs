use std::thread;
use std::io::{Error, Result, Read, Write};
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread::JoinHandle;
use mio::{EventLoop, Token, Handler, PollOpt, Interest, ReadHint };
use mio::tcp::{TcpListener, TcpSocket, TcpStream};
use tcpserver::TcpServer;
use state::State;
use resp::{Parse, Reader, Writer};
use std::net::SocketAddr;
use std::collections::HashMap;

const ACCEPTOR: Token = Token(0);

#[derive(Debug)]
pub enum Notification {
    Connect(Token, SocketAddr),
    Deregister(Token, Error),
    WireMsg(Token, Vec<Vec<u8>>)
}

pub fn run<T: TcpServer>(state: State) -> (State, JoinHandle<()>) {
    let (state_tx, state_rx) = channel();
    let handle = thread::spawn(move || {
        let addr = T::host(&state).parse().unwrap();
        let sock = TcpSocket::v4().unwrap();
        sock.set_reuseaddr(true).unwrap();
        sock.bind(&addr).unwrap();
        let listener = sock.listen(1024).unwrap();
        let mut event_loop = EventLoop::new().unwrap();
        event_loop.register_opt(&listener, ACCEPTOR,
                                Interest::readable(),
                                PollOpt::edge()).unwrap();
        let tx = event_loop.channel();
        let server = T::new(state, tx);
        state_tx.send(server.state());
        let mut ctx = Context::new(server.state(), listener, server);
        event_loop.run(&mut ctx).unwrap();
    });
    let state = state_rx.recv().unwrap();
    (state, handle)
}

// TODO: We don't want to do parsing and messge construction in the event loop.
// This is currently here for expediency in a large refactor. We will want a separate parser
// for each server.
struct Conn<T: Parse> {
    sock: TcpStream,
    reader: Reader<T>,
    writer: Writer,
    outgoing: Vec<Vec<Vec<u8>>>,
    addr: SocketAddr
}

impl<T: Parse> Conn<T> {
    fn new(sock: TcpStream, addr: SocketAddr) -> Conn<T> {
        Conn {
            sock: sock,
            reader: Reader::new(),
            writer: Writer::new(),
            outgoing: Vec::new(),
            addr: addr
        }
    }
}

struct Context<T: TcpServer> {
    node: String,
    state: State,
    conns: HashMap<Token, Conn<T::TcpMsg>>,
    listener: TcpListener,
    server: T
}

impl<T: TcpServer> Context<T> {
    fn new(state: State, listener: TcpListener, server: T) -> Context<T> {
        let state2 = state.clone();
        let members = state2.members.read().unwrap();
        let myname = (*members).orset.name.clone();
        Context {
            node: myname,
            state: state,
            conns: HashMap::new(),
            listener: listener,
            server: server
        }
    }

    fn accept(&mut self, event_loop: &mut EventLoop<Context<T>>) {
        match self.listener.accept() {
            Ok(None) => (), // EWOULDBLOCK
            Ok(Some(sock)) => {
                // TODO: Should probably not unwrap here, since the connection could close
                // immediately
                let addr = sock.peer_addr().unwrap();
                println!("Connection Accepted for {} from {:?}", self.node, addr);
                let token = self.state.next_token();
                self.register(event_loop, token, sock, addr);
            },
            Err(err) => println!("Error accepting connection for node {}: {}", self.node, err)
        }
    }
}

impl<T: TcpServer> Handler for Context<T> {
    type Timeout = ();
    type Message = Notification;

    fn readable(&mut self, event_loop: &mut EventLoop<Context<T>>, token: Token, _: ReadHint)
    {
        match token {
            ACCEPTOR => { self.accept(event_loop); }
            _i => {
                println!("Got Readable for {}", self.node);
                match self.read(event_loop, token) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("Error reading from clsuter socket for {} with token: {:?}: {}",
                                 self.node, token, e);
                        self.deregister(event_loop, token, e);
                    }
                }
            }
        }
    }

    fn writable(&mut self, event_loop: &mut EventLoop<Context<T>>, token: Token) {
        println!("Got Writable for {}", self.node);
        if let Err(err) = self.writer_ready(event_loop, token) {
            println!("Got a write error: {} for token {:?} on {}", err, token, self.node);
            self.deregister(event_loop, token, err);
        }
    }

    fn notify(&mut self, event_loop: &mut EventLoop<Context<T>>, msg: Notification) {
        println!("Event loop got notification {:?} for {}", msg, self.node);
        match msg {
            Notification::Deregister(token, err) => self.deregister(event_loop, token, err),
            Notification::WireMsg(token, msg) => if let Err(err) = self.push_outgoing(event_loop,
                                                                                      token,
                                                                                      msg) {
                println!("Got a write error: {} for token {:?} on {}", err, token, self.node);
                self.deregister(event_loop, token, err);
            },
            Notification::Connect(token, addr) => self.connect(event_loop, token, addr)
        }
    }
}

impl<T: TcpServer> Context<T> {
    fn connect(&mut self, event_loop: &mut EventLoop<Context<T>>, token: Token, addr: SocketAddr) {
        match TcpStream::connect(&addr) {
            Ok(sock) => self.register(event_loop, token, sock, addr),
            Err(e) => self.server.deregister(token, addr)
        }
    }

    fn read(&mut self, event_loop: &mut EventLoop<Context<T>>, token: Token) -> Result<()>{
        if let Some(msg) = try!(self.conns.get_mut(&token).map_or(Ok(None), |ref mut conn| {
            let res = conn.reader.read(&mut conn.sock);
            event_loop.reregister(&conn.sock,
                                  token,
                                  Interest::readable(),
                                  PollOpt::edge() | PollOpt::oneshot()).unwrap();
            res
        })) {
            self.server.handle_tcp_msg(token, msg);
        };
        Ok(())
    }

    fn push_outgoing(&mut self, event_loop: &mut EventLoop<Context<T>>, token: Token,
                     msg: Vec<Vec<u8>>) -> Result<bool> {
        if let Some(ref mut conn) = self.conns.get_mut(&token) {
            conn.outgoing.push(msg);
            if try!(write::<T>(token, *conn)) {
                event_loop.reregister(&conn.sock,
                                      token,
                                      Interest::writable(),
                                      PollOpt::edge() | PollOpt::oneshot()).unwrap();
            };
        }
        Ok(false)
    }

    fn writer_ready(&mut self, event_loop: &mut EventLoop<Context<T>>, token: Token) -> Result<()> {
        // It's possible we already deregistered and one last write gets triggered
        self.conns.get_mut(&token).map(|ref mut conn| {
            conn.writer.ready(true);
            match write::<T>(token, *conn) {
                Ok(true) => {
                    event_loop.reregister(&conn.sock,
                                          token,
                                          Interest::writable(),
                                          PollOpt::edge() | PollOpt::oneshot()).unwrap();
                    Ok(())
                },
                Ok(false) => Ok(()),
                Err(e) => Err(e)
            }
        });
        Ok(())
    }

    fn register(&mut self, event_loop: &mut EventLoop<Context<T>>, token: Token, sock: TcpStream,
                addr: SocketAddr) {
        event_loop.register_opt(&sock, token,
                                Interest::readable() | Interest::writable(),
                                PollOpt::edge() | PollOpt::oneshot()).unwrap();
        let conn = Conn::new(sock, addr.clone());
        self.conns.insert(token, conn);
        self.server.new_sock(token, addr);
    }

    fn deregister(&mut self, event_loop: &mut EventLoop<Context<T>>, token: Token, err: Error) {
        if let Some(conn) = self.conns.remove(&token) {
            event_loop.deregister(&conn.sock).unwrap();
            println!("Deregistered cluster socket for token {:?} with error: {}", token, err);
            self.server.deregister(token, conn.addr);
        } else {
            println!("Error: Tried to deregister a token with no corresponding socket");
        }
    }
}

/// Return Ok(true) if we need to reregister the socket. We only need to do this when the OS
/// tells us we would block. Otherwise we either have nothing to write, or are already waiting on a
/// writable notification.
fn write<T: TcpServer>(token: Token, conn: &mut Conn<T::TcpMsg>) -> Result<bool> {
    while conn.writer.is_ready() {
        if conn.writer.is_empty() {
            if conn.outgoing.len() > 0 {
                let msg = conn.outgoing.remove(0);
                conn.writer.set_data(msg);
            } else {
                return Ok(false);
            }
        }
        match conn.writer.write(&mut conn.sock) {
            Ok(None) =>  { // EWOULDBLOCK
                conn.writer.ready(false);
                return Ok(true);
            },
            Ok(Some(())) => (), // We completed writing the message
            Err(err) => {
                return Err(err);
            }
        }
    }
    Ok(false)
}
