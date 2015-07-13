use mio::Token;
use mio::tcp::TcpStream;

pub enum Event {
    NewSock(Token, TcpStream),
    Readable(Token),
    Writable(Token)
}

pub trait TcpServer {
    fn run() -> Self;
    fn new_sock(&mut self, token: Token, sock: TcpStream);
    fn readable(&mut self, token: Token);
    fn writable(&mut self, token: Token);
}

