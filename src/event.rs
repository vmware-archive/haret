use mio::Token;
use mio::tcp::TcpStream;
use std::net::SocketAddr;
use std::sync::Arc;
use resp::Parse;

#[derive(Debug)]
pub enum Event<T: Send, P: Parse> {
    NewSock(Token, SocketAddr),
    Deregister(Token, SocketAddr),
    TcpMsg(Token, P),
    ApiEvent(T)
}
