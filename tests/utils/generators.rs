//! This module contains common generators to be used across fuzz tests

use rand::{ThreadRng};
use uuid::Uuid;
use rand::distributions::{IndependentSample, Range};

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

pub fn oneof<T: Clone>(rng: &mut ThreadRng, v: &Vec<T>) -> T {
    // Not sure if creating a new Range here throws off sampling uniformity
    let range = Range::new(0, v.len());
    let index = range.ind_sample(rng);
    v[index].clone()
}
