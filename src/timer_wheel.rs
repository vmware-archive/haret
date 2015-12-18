//! This module implements a very simple timer wheel optimized for large numbers of timers using
//! identical timeout lengths. This type of timer wheel is useful for things like client timeouts,
//! which are all identical, but not useful for storing a variety of different timeout lengths. On
//! the other hand it works without tracking actual time and provides O(1) `lookup`, `remove` and
//! `expire` functions. It just requires a single approximately fixed length tick to drive the
//! wheel.
//!
//! Each timer, of say, 5 seconds, is inserted into the `current_write` slot. On each tick the slot
//! pointers will advance. Timers inserted in the `current_read` spot will then all expire and be
//! returned for processing.
use std::hash::Hash;
use std::collections::HashSet;

pub struct TimerWheel<T: Eq+Hash> {
    size: usize,
    data: Vec<HashSet<T>>,
    current_read: usize,
    current_write: usize
}

impl<T: Eq+Hash> TimerWheel<T> {
    pub fn new(size: usize) -> TimerWheel<T> {
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(HashSet::new());
        }
        TimerWheel {
            size: size,
            current_write: size - 1,
            current_read: 0,
            data: data
        }
    }

    // Insert a new connection to the current writing wheel slot
    // Return the number slot so the write can be removed in O(1) time later.
    pub fn insert(&mut self, key: T) -> usize {
        let slot = self.data.get_mut(self.current_write).unwrap();
        slot.insert(key);
        self.current_write
    }

    pub fn remove(&mut self, key: &T, slot: usize) {
        let set = self.data.get_mut(slot).unwrap();
        let _ = set.remove(key);
    }

    pub fn expire(&mut self) -> HashSet<T> {
        self.data.push(HashSet::new());
        let set = self.data.swap_remove(self.current_read);
        self.update_pointers();
        set
    }

    fn update_pointers(&mut self) {
        self.current_write = (self.current_write + 1) % self.size;
        self.current_read = (self.current_read + 1) % self.size;
    }
}
