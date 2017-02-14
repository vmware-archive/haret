// Copyright 2017 VMware, Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::{HashSet, VecDeque};
use time::{Duration, SteadyTime};
use rabble::{Pid, CorrelationId};

/// Metadata for an individual prepare request
///
/// Metdata is stored in a VecDeque where the index is the operation number.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Request {
    pub correlation_id: CorrelationId,
    replies: HashSet<Pid>,
    timeout: SteadyTime
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PrepareRequests {
    quorum_size: usize,
    timeout_ms: i64,
    // The op num at index 0 of `requests` if `requests` is not empty.
    // If `requests` is empty, it's the next op to be prepared.
    pub lowest_op: u64,
    requests: VecDeque<Request>
}

impl PrepareRequests {
    pub fn new(num_replicas: usize, timeout_ms: u64 ) -> PrepareRequests {
        PrepareRequests {
            quorum_size: num_replicas/2 + 1,
            timeout_ms: timeout_ms as i64,
            lowest_op: 1,
            requests: VecDeque::new(),
        }
    }

    pub fn new_prepare(&mut self, op: u64, correlation_id: CorrelationId) {
        if self.requests.is_empty() {
            self.lowest_op = op;
        }
        self.requests.push_back(Request {
            replies: HashSet::with_capacity(self.quorum_size * 2),
            correlation_id: correlation_id,
            timeout: SteadyTime::now() + Duration::milliseconds(self.timeout_ms)
        });
    }

    // Returns true if the request exists, false otherwise
    pub fn insert(&mut self, op: u64, replica: Pid) -> bool {
        if op >= self.lowest_op && self.requests.len() != 0 {
            match self.requests.get_mut((op - self.lowest_op) as usize) {
                Some(ref mut request) => {
                    request.replies.insert(replica);
                    return true;
                },
                None => return false
            }
        }
        false
    }

    /// Do we have a quorum when including the replica making the request?
    ///
    /// Note that we include ourself in this quorum
    pub fn has_quorum(&self, op: u64) -> bool {
        debug_assert!(op > 0);
        if let Some(request) = self.requests.get((op - self.lowest_op) as usize) {
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
            Some(request) => request.timeout > SteadyTime::now(),
            None => false
        }
    }
}
