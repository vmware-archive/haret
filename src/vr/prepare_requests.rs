use std::collections::{HashMap, HashSet, VecDeque};
use time::{SteadyTime, Duration};
use super::replica::Replica;

/// Metadata for an individual prepare request
///
/// Metdata is stored in a VecDeque where the index is the operation number.
pub struct Request {
    replies: HashSet<Pid>,
    correlation_id: CorrelationId,
    timeout: SteadyTime
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PrepareRequests {
    quorum_size: usize,
    timeout: Duration,
    // The smallest op number for all outstanding Prepare requests
    lowest_op: u64,
    request: VecDeque<Request>
}

impl PrepareRequests {
    pub fn new(num_replicas: usize, timeout_ms: u64 ) -> PrepareRequests {
        PrepareRequests {
            quorum_size: num_replicas/2 + 1,
            timeout: Duration::milliseconds(timeout_ms as i64),
            lowest_op: 0,
            request: VecDeque::new(),
        }
    }

    pub fn new_prepare(&mut self, op: u64, correlation_id: CorrelationId) {
        if self.metadata.is_empty() {
            self.lowest_op = op;
        }
        self.requests.push_back(Request {
            replies: HashSet::with_capacity(self.quorum_size),
            correlation_id: CorrelationId,
            timeout: SteadyTime::now() + self.timeout
        });
    }

    // Returns true if the id exists, false otherwise
    pub fn insert(&mut self, op: u64, replica: Pid) -> bool {
        if op >= self.lowest_op && self.requests.len() != 0 {
            let request = self.requests[op - lowest_op];
            request.replies.insert(replica);
            return true;
        }
        false
    }

    /// Do we have a quorum when including the replica making the request?
    ///
    /// Note that we include ourself in this quorum
    pub fn has_quorum(&self, op: u64) -> bool {
        if let Some(request) = self.requests.get(&op) {
            request.replies.len() >= (self.quorum_size - 1)
        } else {
            false
        }
    }

    pub fn remove(&mut self, op: u64) -> Drain<Request> {
        let lowest_op = self.lowest_op;
        self.lowest_op = op + 1;
        self.requests.drain(0..(op - lowest_op + 1))
    }

    pub fn expired(&self) -> bool {
        match self.requests.front() {
            Some(&request) => request.timeout > SteadyTime::now(),
            None => false
        }
    }
}
