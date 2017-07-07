// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::convert::{From, Into};
use rand::{thread_rng, Rng};
use rabble::{self, Pid, CorrelationId, Envelope};
use time::SteadyTime;
use msg::Msg;
use vr::vr_fsm::{Transition, VrState, State};
use vr::vr_msg::{VrMsg, GetState, NewState};
use vr::vr_ctx::VrCtx;
use super::Backup;

/// When a backup realizes it's behind it enters state transfer
///
/// In this state, the backup is waiting for a `NewState` message
state!(StateTransfer {
    ctx: VrCtx
});

impl Transition for StateTransfer {
    fn handle(mut self,
              msg: VrMsg,
              from: Pid,
              cid: CorrelationId,
              output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        match msg {
            // Replicas only respond to state transfer requests in normal mode
            // in the same epoch and view
            VrMsg::NewState(msg) => self.become_backup(msg, output),
            VrMsg::Prepare(msg) => {
                up_to_date!(self, from, msg, cid, output);
                self.into()
            }
            VrMsg::Commit(msg) => {
                up_to_date!(self, from, msg, cid, output);
                self.into()
            }
            VrMsg::StartViewChange(msg) => {
                up_to_date!(self, from, msg, cid, output);
                self.into()
            }
            VrMsg::DoViewChange(msg) => {
                up_to_date!(self, from, msg, cid, output);
                self.into()
            }
            VrMsg::StartView(msg) => {
                up_to_date!(self, from, msg, cid, output);
                self.into()
            }
            VrMsg::GetState(msg) => {
                up_to_date!(self, from, msg, cid, output);
                self.into()
            },
            VrMsg::Tick => {
                if self.ctx.idle_timeout() {
                    self.send_get_state_to_random_replica(cid, output);
                }
                self.into()
            },
            _ => self.into()
        }
    }
}

impl StateTransfer {
    pub fn new(ctx: VrCtx) -> StateTransfer {
        StateTransfer {
            ctx: ctx
        }
    }

    /// Enter state transfer
    pub fn enter(ctx: VrCtx) -> VrState {
        StateTransfer::new(ctx).into()
    }

    pub fn become_backup(mut self, msg: NewState, output: &mut Vec<Envelope<Msg>>) -> VrState {
        self.ctx.last_received_time = SteadyTime::now();
        let NewState {view, op, commit_num, log_tail, ..} = msg;
        self.ctx.view = view;
        self.ctx.op = op;
        for m in log_tail {
            self.ctx.log.push(m);
        }
        let cid = CorrelationId::pid(self.ctx.pid.clone());
        let mut backup = Backup::new(self.ctx);
        backup.set_primary(output);
        backup.send_prepare_ok(cid, output);
        backup.commit(commit_num, output)
    }

    /// When a new replica starts after reconfiguration it needs to send a get state request to all
    /// replicas to ensure it gets the latest state.
    pub fn start_reconfiguration(self,
                                 cid: CorrelationId,
                                 output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        {
            let msg = self.get_state_msg();
            let from = &self.ctx.pid;
            output.extend(self.ctx.old_config
                          .replicas
                          .iter()
                          .cloned()
                          .chain(self.ctx.new_config.replicas.iter().cloned())
                          .filter(|pid| *pid != self.ctx.pid)
                          .map(|pid| Envelope::new(pid,
                                                   from.clone(),
                                                   msg.clone(),
                                                   Some(cid.clone()))));
        }
        self.into()
    }

    /// We missed the reconfiguration, and don't know what the new config looks like or if the
    /// old replicas have shutdown. Therefore retrieve the config from the new primary.
    pub fn start_epoch<S>(state: S,
                          primary: Pid,
                          cid: CorrelationId,
                          new_epoch: u64,
                          new_view: u64,
                          output: &mut Vec<Envelope<Msg>>) -> VrState
        where S: State
    {
        let mut new_state = StateTransfer { ctx: state.ctx() };
        new_state.ctx.last_received_time = SteadyTime::now();
        new_state.ctx.epoch = new_epoch;
        new_state.ctx.view = new_view;
        new_state.ctx.op = new_state.ctx.commit_num;
        let end = StateTransfer::truncation_point(&new_state.ctx);
        new_state.ctx.log.truncate(end);
        let from = new_state.ctx.pid.clone();
        let msg = new_state.get_state_msg();
        output.push(Envelope::new(primary, from, msg, Some(cid)));
        new_state.into()
    }

    pub fn start_view<S>(state: S,
                         new_view: u64,
                         cid: CorrelationId,
                         output: &mut Vec<Envelope<Msg>>) -> VrState
        where S: State
    {
        let mut new_state = StateTransfer { ctx: state.ctx() };
        new_state.ctx.last_received_time = SteadyTime::now();
        new_state.ctx.view = new_view;
        new_state.ctx.op = new_state.ctx.commit_num;
        let end = StateTransfer::truncation_point(&new_state.ctx);
        new_state.ctx.log.truncate(end);
        new_state.send_get_state_to_random_replica(cid, output);
        new_state.into()
    }

    // Send a state transfer message
    pub fn start_same_view(ctx: VrCtx,
                           cid: CorrelationId,
                           output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        let mut new_state = StateTransfer { ctx: ctx };
        new_state.send_get_state_to_random_replica(cid, output);
        new_state.into()
    }

    pub fn send_new_state(ctx: &VrCtx,
                          op: u64,
                          to: Pid,
                          cid: CorrelationId,
                          output: &mut Vec<Envelope<Msg>>)
    {
        let msg = StateTransfer::new_state_msg(ctx, op);
        output.push(Envelope::new(to, ctx.pid.clone(), msg, Some(cid)))
    }

    fn send_get_state_to_random_replica(&mut self,
                                        cid: CorrelationId,
                                        output: &mut Vec<Envelope<Msg>>) {
        let msg = self.get_state_msg();
        let mut rng = thread_rng();
        let mut to = self.ctx.pid.clone();
        while to == self.ctx.pid {
            let index = rng.gen_range(0, self.ctx.new_config.replicas.len());
            to = self.ctx.new_config.replicas[index].clone()
        }
        output.push(Envelope::new(to, self.ctx.pid.clone(), msg, Some(cid)));
    }

    fn get_state_msg(&self) -> rabble::Msg<Msg> {
        GetState {
            epoch: self.ctx.epoch,
            view: self.ctx.view,
            op: self.ctx.op
        }.into()
    }

    fn new_state_msg(ctx: &VrCtx, op: u64) -> rabble::Msg<Msg> {
        let start = (op - ctx.log_start) as usize;
        let end = (ctx.op - ctx.log_start) as usize;
        NewState {
            epoch: ctx.epoch,
            view: ctx.view,
            op: ctx.op,
            commit_num: ctx.commit_num,
            log_tail: (&ctx.log[start..end]).to_vec()
        }.into()
    }

    /// We don't want to truncate past the global_min_accept value.  We also know that operations up
    /// to the global_min_accept value cannot be reordered since they exist on all replicas.
    /// Therefore if the global_min_accept > commit_num we only truncate to the global_min_accept
    /// entry. Otherwise we truncate to the latest commited entry.
    fn truncation_point(ctx: &VrCtx) -> usize {
        if ctx.global_min_accept > ctx.commit_num {
            return (ctx.global_min_accept - ctx.log_start) as usize;
        }
        (ctx.commit_num - ctx.log_start) as usize
    }
}
