#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![feature(mpsc_select)]

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate rand;

extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate mio;
extern crate msgpack;
extern crate rustc_serialize;
extern crate time;

pub mod config;
pub mod event_loop;
pub mod admin;
pub mod cluster;
pub mod state;
pub mod resp;

mod event;
mod orset;
mod tcphandler;
mod membership;
