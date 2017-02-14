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

use rabble::Pid;
use uuid::Uuid;
use time::Duration;
use super::quorum_tracker::QuorumTracker;
use super::vrmsg::VrMsg;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPrimary {
    pub pid: Pid,
    pub view: u64,
    pub op: u64,
    pub commit_num: u64,
    pub log: Vec<VrMsg>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryState {
    pub nonce: Uuid,
    // Primary from the latest view we've heard from
    pub primary: Option<RecoveryPrimary>,
    pub responses: QuorumTracker<()>,
}

impl RecoveryState {
    pub fn new(quorum: u64, timeout: Duration) -> RecoveryState {
        RecoveryState {
            nonce: Uuid::new_v4(),
            primary: None,
            responses: QuorumTracker::new(quorum as usize, &timeout)
        }
    }

    pub fn has_quorum(&self, current_view: u64) -> bool {
        self.responses.has_super_quorum() && self.primary.as_ref().map_or(false, |p| p.view == current_view)
    }
}


