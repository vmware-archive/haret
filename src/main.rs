#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate rand;

extern crate libc;
extern crate serde;
extern crate mio;

mod config;
mod orset;
mod resp;
mod event_loop;
mod tcpserver;
mod admin_server;

use config::Config;
use tcpserver::TcpServer;
use admin_server::AdminServer;
use event_loop::EventLoop;
use std::sync::{Arc, RwLock};

fn main() {
    let config = Arc::new(RwLock::new(Config::read()));
    let mut event_loop = EventLoop::new(AdminServer::run(config.clone()));
    event_loop.run();
}
