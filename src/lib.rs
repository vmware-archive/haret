#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate rand;

extern crate libc;
extern crate serde;
extern crate mio;
extern crate msgpack;
extern crate rustc_serialize;

pub mod config;
pub mod event_loop;
pub mod admin;
pub mod cluster;
pub mod state;

mod event;
mod orset;
mod resp;
mod tcpserver;
mod tcphandler;
mod membership;
