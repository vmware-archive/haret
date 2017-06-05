// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::mem;
use rabble::{self, Pid, CorrelationId, Envelope};
use uuid::Uuid;
use msg::Msg;
use super::utils::QuorumTracker;
use vr::vr_msg::{self, VrMsg, RecoveryResponse, ClientOp};
use vr::VrCtx;
use vr::vr_fsm::{Transition, VrState, State};
use vr::states::Backup;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryPrimary {
    pub pid: Pid,
    pub view: u64,
    pub op: u64,
    pub commit_num: u64,
    pub log: Vec<ClientOp>
}

/// The recovery state of the VR Protocol where a replica is recovering data from a quorum of
/// replicas
state!(Recovery {
    ctx: VrCtx,
    nonce: Uuid,
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
                    self.nonce = Uuid::new_v4();
                    self.ctx.broadcast(self.recovery_msg(), cid, output);
                }
                self.into()
            },
            VrMsg::RecoveryResponse(msg) => {
                self.update_recovery_state(from, msg);
                self.commit_recovery(output)
            },
            _ => self.into()
        }
    }
}

impl Recovery {
    pub fn new(ctx: VrCtx) -> Recovery {
        let quorum = ctx.quorum;
        Recovery {
            ctx: ctx,
            nonce: Uuid::new_v4(),
            primary: None,
            // Expire immediately so recovery is started on the next tick
            responses: QuorumTracker::new(quorum, 0)
        }
    }

    pub fn send_response(ctx: &VrCtx,
                         to: Pid,
                         nonce: Uuid,
                         cid: CorrelationId,
                         output: &mut Vec<Envelope<Msg>>)
    {
        let response = Recovery::response_msg(ctx, nonce);
        output.push(Envelope::new(to, ctx.pid.clone(), response, Some(cid)));
    }


    fn has_quorum(&self) -> bool {
        let current_view = self.ctx.view;
        self.responses.has_super_quorum() &&
            self.primary.as_ref().map_or(false, |p| p.view == current_view)
    }

    fn commit_recovery(mut self, output: &mut Vec<Envelope<Msg>>) -> VrState {
        if self.has_quorum() {
            let commit_num = {
                let mut primary = self.primary.as_mut().unwrap();
                self.ctx.op = primary.op;
                mem::swap(&mut self.ctx.log, &mut primary.log);
                primary.commit_num
            };
            let mut backup = Backup::new(self.ctx);
            backup.set_primary(output);
            return backup.commit(commit_num, output);
        }
        self.into()
    }

    fn update_recovery_state(&mut self, from: Pid, msg: RecoveryResponse) {
        let RecoveryResponse {epoch, view, nonce, op, commit_num, log} = msg;
        if nonce != self.nonce {
            return;
        }
        if epoch < self.ctx.epoch {
            return;
        }

        // TODO: If we get a response from a replica in a later epoch, we are in a weird state
        // We missed a reconfiguration and the namespace manager hasn't learned of the epoch
        // change yet. It started us only with knowledge of the old config. If we search the log in
        // the response backwards for the last reconfguration request, we can learn the new config.
        // We can then announce the reconfiguration to the namespace manager, and restart the
        // recovery process with that config.
        //
        // For now we're ignoring that this situation can even occur. We just return without
        // processing the message. This is clearly wrong.
        if epoch > self.ctx.epoch {
            error!(self.ctx.logger,
                   "EPOCH RECONFIGURATION DURING RECOVERY: Replica {} in a bad state",
                   self.ctx.pid);
            return;
        }

        if view > self.ctx.view {
            self.ctx.view = view;
        }

        let response_from_primary = op.is_some();
        if response_from_primary && view == self.ctx.view {
            self.primary = Some(RecoveryPrimary {
                pid: from.clone(),
                view: view,
                op: op.unwrap(),
                commit_num: commit_num.unwrap(),
                log: log.unwrap()
            });
        }
        self.responses.insert(from, ())
    }

    fn recovery_msg(&self) -> rabble::Msg<Msg> {
        vr_msg::Recovery {
            nonce: self.nonce.clone()
        }.into()
    }


    fn response_msg(ctx: &VrCtx, nonce: Uuid) -> rabble::Msg<Msg> {
        let primary = ctx.compute_primary();
        let (op, commit_num, log) =
            if primary == ctx.pid {
                (Some(ctx.op), Some(ctx.commit_num), Some(ctx.log.clone()))
            } else {
                (None, None, None)
            };
        RecoveryResponse {
            epoch: ctx.epoch,
            view: ctx.view,
            nonce: nonce,
            op: op,
            commit_num: commit_num,
            log: log
        }.into()
    }
}
