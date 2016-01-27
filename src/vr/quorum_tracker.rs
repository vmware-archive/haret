use std::collections::HashMap;
use std::collections::hash_map::Drain;
use time::{SteadyTime, Duration};
use super::replica::Replica;
use super::VrMsg;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuorumTracker {
    quorum_size: usize,
    start_time: SteadyTime,
    replies: HashMap<Replica, VrMsg>
}

/// Do we have a quorum when including the replica making the request?
impl QuorumTracker {
    pub fn new(quorum_size: usize) -> QuorumTracker {
        QuorumTracker {
            quorum_size: quorum_size,
            start_time: SteadyTime::now(),
            replies: HashMap::with_capacity(quorum_size)
        }
    }

    pub fn insert(&mut self, replica: Replica, msg: VrMsg) {
        self.replies.insert(replica, msg);
    }

    pub fn has_quorum(&self) -> bool {
        self.replies.len() >= (self.quorum_size - 1)
    }

    pub fn has_super_quorum(&self) -> bool {
        self.replies.len() >= self.quorum_size
    }

    pub fn is_expired(&self, timeout: &Duration) -> bool {
        SteadyTime::now() - self.start_time > *timeout
    }

    pub fn drain(&mut self) -> Drain<Replica, VrMsg> {
        self.replies.drain()
    }
}
