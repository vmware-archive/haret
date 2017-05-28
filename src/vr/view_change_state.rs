// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use time::Duration;
use super::vrmsg::DoViewChange;
use super::quorum_tracker::QuorumTracker;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Latest {
    pub last_normal_view: u64,
    pub commit_num: u64,
    pub op: u64,
    pub log: Vec<ClientOp>
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

impl From<DoViewChange> for Latest {
    fn from(msg: DoViewChange) -> Latest {
    let VrMsg::DoViewChange{op, last_normal_view, commit_num, log, ..} = msg;
    Latest {
        last_normal_view: last_normal_view,
        commit_num: commit_num,
        op: op,
        log: log
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DoViewChangeState {
    pub responses: QuorumTracker<DoViewChange>,
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
            .map(|(_, msg)| msg.into())
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
