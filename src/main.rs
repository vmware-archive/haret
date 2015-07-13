#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate rand;

extern crate mio;

mod orset;
mod resp;
mod event_loop;
mod tcpserver;
mod admin_server;

use tcpserver::TcpServer;
use admin_server::AdminServer;
use event_loop::EventLoop;

fn main() {
    let mut event_loop = EventLoop::new("127.0.0.1:5000", AdminServer::run());
    event_loop.run();
}
