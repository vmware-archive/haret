//! This module contains common generators to be used across fuzz tests

use rand::{Rng, thread_rng};
use uuid::Uuid;

/// Create a list of n session ids
pub fn session_ids(n: usize) -> Vec<Uuid>{
    let mut session_ids = Vec::with_capacity(n);
    for _ in 0..n {
        session_ids.push(Uuid::new_v4());
    }
    session_ids
}

/// Create a list of paths
pub fn paths() -> Vec<&'static str> {
    vec!["/a",
        "/a/b",
        "/a/b/c",
        "/a/d",
        "/a/d/e",

        // Paths that can never be created (parentless paths)
        "/x/y",
        "/z/z/z"
        ]
}

pub fn oneof<T: Clone>(v: &[T]) -> T {
    thread_rng().choose(v).unwrap().clone()
}
