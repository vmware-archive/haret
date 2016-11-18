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

pub mod config;
pub mod event_loop;
pub mod admin;
pub mod vr;
pub mod frame;
mod error;

mod membership;
mod debug_sender;
mod timer_wheel;
mod requests;
mod timeout;
mod shared_messages;

pub use self::membership::Member;
pub use self::timeout::Timeout;
pub use self::shared_messages::{
    NewSessionRequest,
    NewSessionReply
};
