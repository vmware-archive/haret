// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::collections::hash_map::Drain;
use time::{SteadyTime, Duration};
use rabble::Pid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct QuorumTracker<T> {
    quorum_size: u64,
    // Note that deserialization will return the wrong time.
    // This is only used in debuggin goutput though.
    #[serde(skip_serializing, skip_deserializing, default = "now")]
    expiration: SteadyTime,
    replies: HashMap<Pid, T>
}

fn now() -> SteadyTime {
    SteadyTime::now()
}

/// Do we have a quorum when including the replica making the request?
impl<T> QuorumTracker<T> {
   pub fn new(quorum_size: u64, timeout_ms: i64) -> QuorumTracker<T> {
        QuorumTracker {
            quorum_size: quorum_size,
            expiration: SteadyTime::now() + Duration::milliseconds(timeout_ms),
            replies: HashMap::with_capacity(quorum_size as usize * 2)
        }
    }

    pub fn insert(&mut self, replica: Pid, val: T) {
        self.replies.insert(replica, val);
    }

    /// Quorum including this replica
    pub fn has_quorum(&self) -> bool {
        self.replies.len() as u64 >= (self.quorum_size - 1)
    }

    /// Quorum not including this replica
    pub fn has_super_quorum(&self) -> bool {
        self.replies.len() as u64 >= self.quorum_size
    }

    pub fn is_expired(&self) -> bool {
        SteadyTime::now() > self.expiration
    }

    pub fn drain(&mut self) -> Drain<Pid, T> {
        self.replies.drain()
    }
}
