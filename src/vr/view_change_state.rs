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

use time::Duration;
use super::vrmsg::VrMsg;
use super::quorum_tracker::QuorumTracker;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Latest {
    pub last_normal_view: u64,
    pub commit_num: u64,
    pub op: u64,
    pub log: Vec<VrMsg>
}

impl Latest {
    fn new() -> Latest {
        Latest {
            last_normal_view: 0,
            commit_num: 0,
            op: 0,
            log: Vec::new()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ViewChangeState {
    pub responses: QuorumTracker<VrMsg>,
    latest: Latest,
}

impl ViewChangeState {
    pub fn new(quorum: u64, timeout: Duration) -> ViewChangeState {
        ViewChangeState {
            responses: QuorumTracker::new(quorum as usize, &timeout),
            latest: Latest::new()
        }
    }

    pub fn has_quorum(&self) -> bool {
        self.responses.has_quorum()
    }

    pub fn compute_latest_state(&mut self, current: Latest) -> Latest {
        self.responses.drain()
            .map(|(_, msg)| convert_do_view_change_msg_to_latest(msg))
            .fold(current, |mut latest, other| {
                if (other.last_normal_view > latest.last_normal_view) ||
                    (other.last_normal_view == latest.last_normal_view && other.op > latest.op)
                {
                   latest.last_normal_view = other.last_normal_view;
                   latest.op = other.op;
                   latest.log = other.log;
                }
                if other.commit_num > latest.commit_num {
                    latest.commit_num = other.commit_num;
                }
                latest
            })
    }
}

fn convert_do_view_change_msg_to_latest(msg: VrMsg) -> Latest {
    if let VrMsg::DoViewChange{op, last_normal_view, commit_num, log, ..} = msg {
        return Latest {
            last_normal_view: last_normal_view,
            commit_num: commit_num,
            op: op,
            log: log
        };
    }
    unreachable!()
}
