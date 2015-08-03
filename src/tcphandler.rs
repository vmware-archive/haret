use mio::{Token, Sender};
use event_loop::Notification;
use state::State;
use std::net::SocketAddr;
use resp::Parse;

pub trait TcpHandler {
    type Event;
    type TcpMsg: Parse;

    fn new(state: State, tx: Sender<Notification>) -> Self;
    fn connect(&mut self, token: Token, addr: SocketAddr) {}
    fn deregister(&mut self, token: Token, addr: SocketAddr) {}
    fn handle_tcp_msg(&mut self, token: Token, msg: Self::TcpMsg);
    fn handle_event(&mut self, Self::Event);
}
