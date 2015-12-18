#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![feature(mpsc_select)]
#![feature(btree_range, collections_bound)]

#[cfg(test)]
extern crate quickcheck;

extern crate rand;
extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate mio;
extern crate msgpack;
extern crate rustc_serialize;
extern crate time;
extern crate uuid;
#[macro_use]
extern crate fsm;

pub mod config;
pub mod event_loop;
pub mod admin;
pub mod cluster;
pub mod state;
pub mod resp;
pub mod vr_api;
pub mod vr;

mod event;
mod orset;
mod tcphandler;
mod membership;

pub use self::membership::Member;
