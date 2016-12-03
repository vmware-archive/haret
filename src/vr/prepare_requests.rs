use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::vec_deque::Drain;
use time::Duration;
use rabble::{Pid, CorrelationId};
use super::encodable_steady_time::EncodableSteadyTime;

/// Metadata for an individual prepare request
///
/// Metdata is stored in a VecDeque where the index is the operation number.
#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Request {
    pub correlation_id: CorrelationId,
    replies: HashSet<Pid>,
    timeout: EncodableSteadyTime
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct PrepareRequests {
    quorum_size: usize,
    timeout_ms: i64,
    // The smallest op number for all outstanding Prepare requests
    lowest_op: u64,
    requests: VecDeque<Request>
}

impl PrepareRequests {
    pub fn new(num_replicas: usize, timeout_ms: u64 ) -> PrepareRequests {
        PrepareRequests {
            quorum_size: num_replicas/2 + 1,
            timeout_ms: timeout_ms as i64,
            lowest_op: 0,
            requests: VecDeque::new(),
        }
    }

    pub fn new_prepare(&mut self, op: u64, correlation_id: CorrelationId) {
        if self.requests.is_empty() {
            self.lowest_op = op;
        }
        self.requests.push_back(Request {
            replies: HashSet::with_capacity(self.quorum_size),
            correlation_id: correlation_id,
            timeout: EncodableSteadyTime(EncodableSteadyTime::now().0 +
                                         Duration::milliseconds(self.timeout_ms))
        });
    }

    // Returns true if the id exists, false otherwise
    pub fn insert(&mut self, op: u64, replica: Pid) -> bool {
        if op >= self.lowest_op && self.requests.len() != 0 {
            let ref mut request = self.requests[(op - self.lowest_op) as usize];
            request.replies.insert(replica);
            return true;
        }
        false
    }

    /// Do we have a quorum when including the replica making the request?
    ///
    /// Note that we include ourself in this quorum
    pub fn has_quorum(&self, op: u64) -> bool {
        if let Some(request) = self.requests.get(op as usize) {
            request.replies.len() >= (self.quorum_size - 1)
        } else {
            false
        }
    }

    pub fn remove(&mut self, op: u64) -> Vec<Request> {
        let lowest_op = self.lowest_op;
        self.lowest_op = op + 1;
        self.requests.drain(0..(op - lowest_op + 1) as usize).collect()
    }

    pub fn expired(&self) -> bool {
        match self.requests.front() {
            Some(request) => request.timeout.0 > EncodableSteadyTime::now().0,
            None => false
        }
    }
}
