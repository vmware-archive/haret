// Copyright 2017 VMware, Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
