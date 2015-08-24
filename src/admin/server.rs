use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Sender, channel};
use mio;
use mio::{Token};
use mio::tcp::TcpStream;
use event_loop::Notification;
use state::State;
use event::Event;
use super::{AdminEvent, AdminHandler};
use super::messages::*;
use tcphandler::TcpHandler;
use tcpserver::TcpServer;
use std::net::SocketAddr;
use std::sync::Arc;

pub struct AdminServer {
    // We could use a thread pool here and make the size configurable per server
    thread: JoinHandle<()>,
    sender: Sender<Event<AdminEvent, Msg>>,
    state: State
}

impl TcpServer for AdminServer {
    type Event = AdminEvent;
    type TcpMsg = Msg;

    fn new(state: State, event_loop_sender: mio::Sender<Notification>) -> AdminServer {
        let (tx, rx) = channel();
        let mut state = state;
        let mut state2 = state.clone();
        state.admin_tx = Some(tx.clone());
        let handle = thread::Builder::new().name("admin_server".to_string()).spawn(move || {
            let mut handler = AdminHandler::new(state, event_loop_sender);
            loop {
                match rx.recv().unwrap() {
                    Event::NewSock(token, addr) => handler.connect(token, addr),
                    Event::Deregister(token, addr) => handler.deregister(token, addr),
                    Event::TcpMsg(token, msg) => handler.handle_tcp_msg(token, msg),
                    Event::ApiEvent(event) => handler.handle_event(event)
                }
            }
        }).unwrap();

        state2.admin_tx = Some(tx.clone());

        AdminServer {
            thread: handle,
            sender: tx,
            state: state2
        }
    }

    fn state(&self) -> State {
        self.state.clone()
    }

    fn host(state: &State) -> String {
        let config = state.config.read().unwrap();
        config.admin_host.clone()
    }

    fn new_sock(&self, token: Token, addr: SocketAddr) {
        self.sender.send(Event::NewSock(token, addr)).unwrap();
    }

    fn deregister(&self, token: Token, addr: SocketAddr) {
        self.sender.send(Event::Deregister(token, addr)).unwrap();
    }

    fn handle_tcp_msg(&self, token: Token, msg: Msg) {
        self.sender.send(Event::TcpMsg(token, msg)).unwrap();
    }
}
