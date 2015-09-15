use mio::Token;
use std::net::SocketAddr;
use resp::Parse;

#[derive(Debug)]
pub enum Event<T: Parse+Send> {
    NewSock(Token, SocketAddr),
    Deregister(Token, SocketAddr),
    TcpMsg(Token, T),
    Tick,
}
