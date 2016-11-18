#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![feature(mpsc_select)]
#![feature(btree_range, collections_bound)]

// Crates we don't manage
extern crate rand;
extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate mio;
extern crate msgpack;
extern crate rustc_serialize;
extern crate time;
extern crate uuid;
extern crate stats;

// Crates we manage
#[macro_use]
extern crate fsm;
extern crate orset;
extern crate rabble;

pub mod config;
pub mod admin;
pub mod vr;
mod error;

mod debug_sender;
mod session_msgs;
mod session;
mod namespace_mgr;
mod namespace_msg;
mod namespaces;

pub use self::session_messages::{
    NewSessionRequest,
    NewSessionReply
};
