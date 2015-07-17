use mio::Token;
use mio::tcp::TcpStream;
use std::sync::{Arc, RwLock};
use config::Config;

pub enum Event {
    NewSock(Token, TcpStream),
    Readable(Token),
    Writable(Token)
}

pub trait TcpServer {
    fn run(config: Arc<RwLock<Config>>) -> Self;
    fn host(&self) -> String;
    fn new_sock(&mut self, token: Token, sock: TcpStream);
    fn readable(&mut self, token: Token);
    fn writable(&mut self, token: Token);
}

