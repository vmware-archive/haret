// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::convert::{From, Into};
use rabble::{self, Pid, CorrelationId, Envelope};
use time::SteadyTime;
use msg::Msg;
use vr::vr_fsm::{Transition, VrState, State};
use vr::vr_msg::{self, VrMsg, ClientOp};
use vr::vr_ctx::VrCtx;
use vr::states::common::view_change;
use super::utils::QuorumTracker;
use super::{Primary, StateTransfer, StartViewChange};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Latest {
    pub last_normal_view: u64,
    pub commit_num: u64,
    pub op: u64,
    pub log_start: u64,
    pub log_tail: Vec<ClientOp>
}

impl Latest {
    fn new() -> Latest {
        Latest {
            last_normal_view: 0,
            commit_num: 0,
            op: 0,
            log_start: 0,
            log_tail: Vec::new()
        }
    }
}

impl From<vr_msg::DoViewChange> for Latest {
    fn from(msg: vr_msg::DoViewChange) -> Latest {
        Latest {
            last_normal_view: msg.last_normal_view,
            commit_num: msg.commit_num,
            op: msg.op,
            log_start: msg.log_start,
            log_tail: msg.log_tail
        }
    }
}

/// The part of the view change state in the VR protocol state machine the proposed primary is
/// waiting for a quorum of `DoViewChange` messages.
state!(DoViewChange {
    ctx: VrCtx,
    responses: QuorumTracker<vr_msg::DoViewChange>,
    latest: Latest
});

impl Transition for DoViewChange {
    fn handle(self,
              msg: VrMsg,
              from: Pid,
              cid: CorrelationId,
              output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        match msg {
            VrMsg::StartViewChange(msg) => self.handle_start_view_change(msg, from, output),
            VrMsg::DoViewChange(msg) => self.handle_do_view_change(msg, from, output),
            VrMsg::StartView(msg) => view_change::handle_start_view(self, msg, cid, output),
            VrMsg::Tick => view_change::handle_tick(self, output),
            VrMsg::Prepare(msg) => {
                up_to_date!(self, from, msg, cid, output);
                StateTransfer::start_same_view(self.ctx, cid, output)
            },
            VrMsg::Commit(msg) => {
                up_to_date!(self, from, msg, cid, output);
                StateTransfer::start_same_view(self.ctx, cid, output)
            },
            _ => self.into()
        }
    }
}

impl DoViewChange {
    fn new(mut ctx: VrCtx) -> DoViewChange {
        ctx.last_received_time = SteadyTime::now();
        DoViewChange {
            responses: QuorumTracker::new(ctx.quorum, ctx.idle_timeout_ms),
            ctx: ctx,
            latest: Latest::new()
        }
    }
    /// Enter the DoViewChange state
    pub fn enter(ctx: VrCtx) -> VrState {
        DoViewChange::new(ctx).into()
    }

    pub fn start_do_view_change<S: State>(state: S,
                                          from: Pid,
                                          msg: vr_msg::DoViewChange,
                                          output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        // This is a later view in the same epoch. Other nodes have decided that we
        // should be the primary in this view via StartViewChange quorum.
        let mut new_state = DoViewChange::new(state.ctx());
        new_state.ctx.view = msg.view;
        new_state.responses.insert(from, msg);
        if new_state.responses.has_quorum() {
            return new_state.become_primary(output);
        }
        new_state.into()
    }

    fn become_primary(mut self, output: &mut Vec<Envelope<Msg>>) -> VrState {
        let latest = self.compute_latest_state();
        self.ctx.op = latest.op;
        self.ctx.last_normal_view = self.ctx.view;
        self.ctx.last_received_time = SteadyTime::now();
        self.ctx.append_log_tail(latest.log_start, latest.log_tail);
        self.broadcast_start_view_msg(latest.commit_num, output);
        let mut primary = Primary::new(self.ctx);
        primary.set_primary(output);
        primary.commit(latest.commit_num, output)
    }

    fn handle_start_view_change(self,
                                msg: vr_msg::StartViewChange,
                                from: Pid,
                                output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        // Old messages we want to ignore. For New ones we want to wait until a primary is elected,
        // since we know we are out of date and need to perform state transfer, which will fail until
        // a replica is in normal mode.
        if msg.epoch != self.ctx.epoch {
            return self.into();
        }
        if msg.view <= self.ctx.view {
            return self.into();
        }
        StartViewChange::start_view_change(self.ctx, from, msg, output)
    }

    fn handle_do_view_change(mut self,
                             msg: vr_msg::DoViewChange,
                             from: Pid,
                             output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        // Old messages we want to ignore. We don't want to become the primary here either, since we
        // didn't participate in reconfiguration, and therefore haven't yet learned about how many
        // replicas we need to get quorum. We just want to wait until another replica is elected
        // primary and then transfer state from it.
        if msg.epoch != self.ctx.epoch {
            return self.into();
        }
        if msg.view < self.ctx.view {
            return self.into();
        }

        if msg.view == self.ctx.view {
            self.responses.insert(from, msg);
            if self.responses.has_quorum() {
                return self.become_primary(output)
            }
            return self.into();
        }
        DoViewChange::start_do_view_change(self, from, msg, output)
    }

    fn compute_latest_state(&mut self) -> Latest {
        let current = Latest {
            last_normal_view: self.ctx.last_normal_view,
            commit_num: self.ctx.commit_num,
            op: self.ctx.op,
            log_start: self.ctx.global_min_accept,
            log_tail: self.ctx.get_log_tail()
        };

        self.responses.drain()
            .map(|(_, msg)| msg.into())
            .fold(current, |mut latest, other: Latest| {
                if (other.last_normal_view > latest.last_normal_view) ||
                    (other.last_normal_view == latest.last_normal_view && other.op > latest.op)
                {
                   latest.last_normal_view = other.last_normal_view;
                   latest.op = other.op;
                   latest.log_start = other.log_start;
                   latest.log_tail = other.log_tail;
                } else if other.last_normal_view == latest.last_normal_view
                       && other.op == latest.op
                       && other.log_start > latest.log_start
                {
                           latest.log_start = other.log_start;
                           latest.log_tail = other.log_tail;
                }
                if other.commit_num > latest.commit_num {
                    latest.commit_num = other.commit_num;
                }
                latest
            })
    }

    fn broadcast_start_view_msg(&self, new_commit_num: u64, output: &mut Vec<Envelope<Msg>>) {
        let msg = self.start_view_msg(new_commit_num);
        let cid = CorrelationId::pid(self.ctx.pid.clone());
        self.ctx.broadcast(msg, cid, output);
    }

    fn start_view_msg(&self, new_commit_num: u64) -> rabble::Msg<Msg> {
        vr_msg::StartView {
            epoch: self.ctx.epoch,
            view: self.ctx.view,
            op: self.ctx.op,
            log_start: self.ctx.global_min_accept,
            log_tail: self.ctx.get_log_tail(),
            commit_num: new_commit_num
        }.into()
    }

}
