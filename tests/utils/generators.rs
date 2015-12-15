//! This module contains common generators to be used across fuzz tests

use rand::{ThreadRng};
use uuid::Uuid;
use rand::distributions::{IndependentSample, Range};

/// Create a list of n clients
pub fn clients(n: usize) -> Vec<Uuid>{
    let mut clients = Vec::new();
    for _ in 0..n {
        clients.push(Uuid::new_v4());
    }
    clients
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
