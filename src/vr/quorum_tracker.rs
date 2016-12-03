use std::collections::HashMap;
use std::collections::hash_map::Drain;
use time::SteadyTime;
use rabble::Pid;
use super::encodable_steady_time::{EncodableSteadyTime, EncodableDuration};


#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct QuorumTracker<T> {
    quorum_size: usize,
    expiration: EncodableSteadyTime,
    replies: HashMap<Pid, T>
}

/// Do we have a quorum when including the replica making the request?
impl<T> QuorumTracker<T> {
    pub fn new(quorum_size: usize, timeout: &EncodableDuration) -> QuorumTracker<T> {
        QuorumTracker {
            quorum_size: quorum_size,
            expiration: EncodableSteadyTime(SteadyTime::now() + (*timeout).0),
            replies: HashMap::with_capacity(quorum_size * 2)
        }
    }

    pub fn insert(&mut self, replica: Pid, val: T) {
        self.replies.insert(replica, val);
    }

    /// Quorum including this replica
    pub fn has_quorum(&self) -> bool {
        self.replies.len() >= (self.quorum_size - 1)
    }

    /// Quorum not including this replica
    pub fn has_super_quorum(&self) -> bool {
        self.replies.len() >= self.quorum_size
    }

    pub fn is_expired(&self) -> bool {
        SteadyTime::now() > self.expiration.0
    }

    pub fn drain(&mut self) -> Drain<Pid, T> {
        self.replies.drain()
    }
}
