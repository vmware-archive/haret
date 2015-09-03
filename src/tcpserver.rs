use mio::{Token, Sender};
use event_loop::Notification;
use state::State;
use std::net::SocketAddr;
use resp::Parse;

pub trait TcpServer {
    type Event;
    type TcpMsg: Parse;

    fn new(state: State, event_loop_sender: Sender<Notification>) -> Self;
    fn state(&self) -> State;
    fn host(state: &State) -> String;
    fn new_sock(&self, token: Token, addr: SocketAddr);
    fn handle_tcp_msg(&self, token: Token, msg: Self::TcpMsg);
    fn deregister(&self, token: Token, addr: SocketAddr);
    fn tick(&self) {}
}
