// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
mod macros;

pub mod arbitrary;

#[allow(dead_code)]
pub mod vr_invariants;

#[allow(dead_code)]
pub mod op_invariants;

#[allow(dead_code)]
pub mod scheduler;

#[allow(dead_code)]
mod model;

pub use self::model::Model;

use std::panic;

/// Taken (with slight modification) from quickcheck crate
/// Wrap tests in a closure so that we can catch panics and treat them as errors
#[allow(dead_code)]
pub fn safe<T, F>(fun: F) -> Result<T, String> where F: FnOnce() -> T {
    panic::catch_unwind(panic::AssertUnwindSafe(fun)).map_err(|any_err| {
        // Extract common types of panic payload:
        // panic and assert produce &str or String
        if let Some(&s) = any_err.downcast_ref::<&str>() {
            s.to_owned()
        } else if let Some(s) = any_err.downcast_ref::<String>() {
            s.to_owned()
        } else {
            "UNABLE TO SHOW RESULT OF PANIC.".to_owned()
        }
    })
}
