use std::collections::HashSet;
use std::collections::hash_map::Drain;
use time::{SteadyTime, Duration};
use rabble::Pid;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuorumTracker {
    quorum_size: usize,
    expiration: SteadyTime,
    replies: HashSet<Pid>
}

/// Do we have a quorum when including the replica making the request?
impl QuorumTracker {
    pub fn new(quorum_size: usize, timeout: &Duration) -> QuorumTracker {
        QuorumTracker {
            quorum_size: quorum_size,
            expiration: SteadyTime::now() + *timeout,
            replies: HashSet::with_capacity(quorum_size * 2)
        }
    }

    pub fn insert(&mut self, replica: Pid) {
        self.replies.insert(replica);
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
        SteadyTime::now() > self.expiration
    }

    pub fn drain(&mut self) -> Drain<Pid> {
        self.replies.drain()
    }
}
