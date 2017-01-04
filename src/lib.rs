#![feature(custom_derive, plugin)]

// Crates we don't manage
extern crate rand;
extern crate libc;
extern crate rustc_serialize;
extern crate time;
extern crate uuid;
#[macro_use]
extern crate slog;
extern crate protobuf;

// Crates we manage
#[macro_use]
extern crate funfsm;
extern crate orset;
extern crate rabble;
extern crate amy;
extern crate vertree;

pub mod config;
pub mod admin;
pub mod vr;
pub mod api;
mod error;

mod namespace_mgr;
mod namespace_msg;
mod namespaces;
mod msg;

pub use msg::Msg;
pub use namespace_mgr::NamespaceMgr;
pub use namespace_msg::{
    NamespaceMsg,
    ClientId,
    NamespaceId
};
