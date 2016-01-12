use std::collections::{HashMap, HashSet, VecDeque};
use time::{SteadyTime, Duration};
use super::replica::Replica;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PrepareRequests {
    num_replicas: usize,
    quorum_size: usize,
    timeout: Duration,
    timeouts: VecDeque<SteadyTime>,
    // The smallest op number for all outstanding Prepare requests
    lowest_op: u64,
    requests: HashMap<u64, HashSet<Replica>>
}

impl PrepareRequests {
    pub fn new(num_replicas: usize, timeout_ms: u64 ) -> PrepareRequests {
        PrepareRequests {
            num_replicas: num_replicas,
            quorum_size: num_replicas/2 + 1,
            timeout: Duration::milliseconds(timeout_ms as i64),
            timeouts: VecDeque::new(),
            lowest_op: 0,
            requests: HashMap::new()
        }
    }

    pub fn new_prepare(&mut self, op: u64) {
        if self.timeouts.is_empty() {
            self.lowest_op = op;
        }
        self.requests.insert(op, HashSet::with_capacity(self.num_replicas));
        self.timeouts.push_back(SteadyTime::now() + self.timeout);
    }

    // Returns true if the id exists, false otherwise
    pub fn insert(&mut self, op: u64, replica: Replica) -> bool {
        if let Some(replicas) = self.requests.get_mut(&op) {
            replicas.insert(replica);
            true
        } else {
            false
        }
    }

    /// Do we have a quorum when including the replica making the request?
    pub fn has_quorum(&self, op: u64) -> bool {
        if let Some(replicas) = self.requests.get(&op) {
            replicas.len() >= (self.quorum_size - 1)
        } else {
            false
        }
    }

    pub fn remove(&mut self, op: u64) {
        if self.timeouts.is_empty() { return; }
        for i in self.lowest_op..op + 1 {
            self.requests.remove(&i);
            // Note that removes always oare always in order (*op is monitonic)
            // We therefore remove the first timeout in, so it doesn't cause an expiration
            self.timeouts.pop_front();
        }
        self.lowest_op = op + 1;
    }

    pub fn expired(&self) -> bool {
        match self.timeouts.front() {
            Some(&time) => time > SteadyTime::now(),
            // If there are no outstanding requests, we don't step down
            None => false
        }
    }
}
