#![feature(custom_derive, plugin)]
#![feature(btree_range, collections_bound)]

// Crates we don't manage
extern crate rand;
extern crate libc;
extern crate rustc_serialize;
extern crate time;
extern crate uuid;
extern crate stats;

// Crates we manage
#[macro_use]
extern crate fsm;
extern crate orset;
extern crate rabble;
extern crate amy;

pub mod config;
pub mod admin;
pub mod vr;
mod error;

mod session_msgs;
mod namespace_mgr;
mod namespace_msg;
mod namespaces;
mod msg;

pub use msg::Msg;
pub use namespace_mgr::NamespaceMgr;
