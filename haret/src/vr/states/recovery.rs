// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use rabble::{self, Pid, CorrelationId, Envelope};
use msg::Msg;
use super::utils::QuorumTracker;
use vr::vr_msg::{self, VrMsg, RecoveryResponse, ClientOp};
use vr::{VrCtx, VrBackend};
use vr::vr_fsm::{Transition, VrState, State};
use vr::states::Backup;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryPrimary {
    pub pid: Pid,
    pub view: u64,
    pub op: u64,
    pub commit_num: u64,
    pub state: VrBackend,
    pub log_start: u64,
    pub log_tail: Vec<ClientOp>
}

/// The recovery state of the VR Protocol where a replica is recovering data from a quorum of
/// replicas
state!(Recovery {
    ctx: VrCtx,
    nonce: u64,
    // Primary from the latest view we've heard from
    primary: Option<RecoveryPrimary>,
    responses: QuorumTracker<()>
});

impl Transition for Recovery {
    fn handle(mut self,
              msg: VrMsg,
              from: Pid,
              _: CorrelationId,
              output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        match msg {
            VrMsg::Tick => {
                if self.responses.is_expired() {
                    let cid = CorrelationId::pid(self.ctx.pid.clone());
                    self.responses = QuorumTracker::new(self.ctx.quorum, self.ctx.idle_timeout_ms);
                    self.primary = None;
                    self.ctx.broadcast(self.recovery_msg(), cid, output);
                }
                self.into()
            },
            VrMsg::RecoveryResponse(msg) => {
                self.update_recovery_state(from, msg, output);
                self.commit_recovery(output)
            },
            _ => self.into()
        }
    }
}

impl Recovery {
    pub fn new(ctx: VrCtx, nonce: u64) -> Recovery {
        let quorum = ctx.quorum;
        Recovery {
            ctx: ctx,
            nonce: nonce,
            primary: None,
            // Expire immediately so recovery is started on the next tick
            responses: QuorumTracker::new(quorum, 0)
        }
    }

    fn has_quorum(&self) -> bool {
        let current_view = self.ctx.view;
        self.responses.has_super_quorum() &&
            self.primary.as_ref().map_or(false, |p| p.view == current_view)
    }

    fn commit_recovery(mut self, output: &mut Vec<Envelope<Msg>>) -> VrState {
        if self.has_quorum() {
            let commit_num = {
                let primary = self.primary.take().unwrap();
                self.ctx.op = primary.op;
                self.ctx.backend = primary.state;
                self.ctx.log_start = primary.log_start;
                self.ctx.log = primary.log_tail;
                // Don't attempt to commit operations that are already part of the backend state
                // They don't exist in the log anyway.
                self.ctx.commit_num = primary.log_start;
                primary.commit_num
            };
            let mut backup = Backup::new(self.ctx);
            backup.set_primary(output);
            // This isn't in the VR protocol, but we send a PrepareOk here so that
            // the primary can update it's min_accept table in case it committed operations while
            // this replica was down.
            let cid = CorrelationId::pid(backup.ctx.pid.clone());
            backup.send_prepare_ok(cid, output);
            return backup.commit(commit_num, output);
        }
        self.into()
    }

    fn update_recovery_state(&mut self,
                             from: Pid,
                             msg: RecoveryResponse,
                             output: &mut Vec<Envelope<Msg>>)
    {
        if msg.nonce != self.nonce {
            return;
        }
        if msg.epoch < self.ctx.epoch {
            return;
        }

        // If we get a response from a replica in a later epoch, we learn the config from the
        // message and try again with the new group. If this replica isn't a member of the new group
        // it shuts down.
        if msg.epoch > self.ctx.epoch {
            let cid = CorrelationId::pid(self.ctx.pid.clone());
            self.ctx.epoch = msg.epoch;
            self.ctx.view = msg.view;
            self.ctx.old_config = msg.old_config.unwrap();
            self.ctx.new_config = msg.new_config.unwrap();
            self.ctx.quorum = self.ctx.new_config.replicas.len() as u64 / 2 + 1;
            self.primary = None;
            self.responses = QuorumTracker::new(self.ctx.quorum, self.ctx.idle_timeout_ms);
            self.ctx.broadcast(self.recovery_msg(), cid, output);
            return;
        }

        if msg.view > self.ctx.view {
            self.ctx.view = msg.view;
        }

        let response_from_primary = msg.op.is_some();
        if response_from_primary && msg.view == self.ctx.view {
            self.ctx.global_min_accept = msg.global_min_accept;
            self.primary = Some(RecoveryPrimary {
                pid: from.clone(),
                view: msg.view,
                op: msg.op.unwrap(),
                commit_num: msg.commit_num.unwrap(),
                state: msg.state.unwrap(),
                log_start: msg.log_start.unwrap(),
                log_tail: msg.log_tail.unwrap()
            });
        }
        self.responses.insert(from, ())
    }

    fn recovery_msg(&self) -> rabble::Msg<Msg> {
        vr_msg::Recovery {
            epoch: self.ctx.epoch,
            nonce: self.nonce,
        }.into()
    }
}
