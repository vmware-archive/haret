// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::mem;
use rabble::{self, Pid, CorrelationId, Envelope};
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

    pub fn send_response(ctx: &VrCtx,
                         to: Pid,
                         msg: vr_msg::Recovery,
                         cid: CorrelationId,
                         output: &mut Vec<Envelope<Msg>>)
    {
        let vr_msg::Recovery {epoch, nonce} = msg;
        let response = Recovery::response_msg(ctx, epoch, nonce);
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
        let RecoveryResponse {epoch,
                              view,
                              nonce,
                              op,
                              commit_num,
                              log,
                              global_min_accept,
                              old_config,
                              new_config} = msg;

        if nonce != self.nonce {
            return;
        }
        if epoch < self.ctx.epoch {
            return;
        }

        // If we get a response from a replica in a later epoch, we learn the config from the
        // message and try again with the new group. If this replica isn't a member of the new group
        // it shuts down.
        if epoch > self.ctx.epoch {
            let cid = CorrelationId::pid(self.ctx.pid.clone());
            self.ctx.epoch = epoch;
            self.ctx.view = view;
            self.ctx.old_config = old_config.unwrap();
            self.ctx.new_config = new_config.unwrap();
            self.ctx.quorum = self.ctx.new_config.replicas.len() as u64 / 2 + 1;
            self.primary = None;
            self.responses = QuorumTracker::new(self.ctx.quorum, self.ctx.idle_timeout_ms);
            self.ctx.broadcast(self.recovery_msg(), cid, output);
            return;
        }

        if view > self.ctx.view {
            self.ctx.view = view;
        }

        let response_from_primary = op.is_some();
        if response_from_primary && view == self.ctx.view {
            self.ctx.global_min_accept = global_min_accept;
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
            epoch: self.ctx.epoch,
            nonce: self.nonce.clone()
        }.into()
    }


    fn response_msg(ctx: &VrCtx, epoch: u64, nonce: u64) -> rabble::Msg<Msg> {
        let primary = ctx.compute_primary();

        let (op, commit_num, log) = if primary == ctx.pid {
            (Some(ctx.op), Some(ctx.commit_num), Some(ctx.log.clone()))
        } else {
            (None, None, None)
        };

        let (old_config, new_config) = if ctx.epoch > epoch {
            (Some(ctx.old_config.clone()), Some(ctx.new_config.clone()))
        } else {
            (None, None)
        };

        RecoveryResponse {
            epoch: ctx.epoch,
            view: ctx.view,
            nonce: nonce,
            global_min_accept: ctx.global_min_accept,
            op: op,
            commit_num: commit_num,
            log: log,
            old_config: old_config,
            new_config: new_config
        }.into()
    }
}
