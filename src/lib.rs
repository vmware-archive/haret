// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![feature(custom_derive, plugin)]

// Crates we don't manage
extern crate rand;
extern crate libc;
extern crate time;
extern crate uuid;
#[macro_use]
extern crate slog;
extern crate protobuf;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


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
pub use namespace_msg::{NamespaceMsg, ClientId, NamespaceId};
