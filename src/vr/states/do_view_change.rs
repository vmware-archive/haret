// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::convert::{From, Into};
use rabble::{self, Pid, CorrelationId, Envelope};
use time::{SteadyTime, Duration};
use msg::Msg;
use vr::vr_fsm::{Transition, VrState, State};
use vr::vr_msg::{ClientOp, ClientRequest, Reconfiguration, ClientReply, Prepare, PrepareOk, Tick};
use vr::vr_msg::{self, VrMsg, GetState, Recovery, StartEpoch, StartViewChange};
use vr::vr_ctx::{VrCtx, DEFAULT_IDLE_TIMEOUT_MS, DEFAULT_PRIMARY_TICK_MS};
use super::utils::QuorumTracker;
use super::{Primary, Backup, StateTransfer, StartView};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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

/// The part of the view change state in the VR protocol state machine the proposed primary is
/// waiting for a quorum of `DoViewChange` messages.
state!(DoViewChange {
    ctx: VrCtx,
    responses: QuorumTracker<DoViewChange>,
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
            VrMsg::StartViewChange(msg) => self.handle_start_view_change(msg, from, cid, output),
            VrMsg::DoViewChange(msg) => self.handle_do_view_change(msg, from, cid, output),
            VrMsg::StartView(msg) => self.handle_start_view(msg, from, cid, output),
            VrMsg::Tick => self.handle_tick(output),
            VrMsg::Prepare(msg) => {
                up_to_date!(self, from, msg, cid, output);
                StateTransfer::start_same_view(self, cid, output)
            },
            VrMsg::Commit(msg) => {
                up_to_date!(self, from, msg, cid, output);
                StateTransfer::start_same_view(self, cid, output)
            },
            _ => self.into()
        }
    }
}

impl DoViewChange {
    /// Enter the DoViewChange state
    pub fn enter(mut ctx: VrCtx) -> VrState {
        ctx.last_received_time = SteadyTime::now();
        let quorum = ctx.quorum;
        let idle_timeout = Duration::milliseconds(DEFAULT_IDLE_TIMEOUT_MS as i64);
        DoViewChange {
            ctx: ctx,
            responses: QuorumTracker::new(quorum, &idle_timeout),
            latest: Latest::new()
        }.into()
    }

    pub fn start_do_view_change<S: State>(state: S,
                                    from: Pid,
                                    msg: DoViewChange,
                                    output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        // This is a later view in the same epoch. Other nodes have decided that we
        // should be the primary in this view via StartViewChange quorum.
        let new_state = DoViewChange::from(state);
        new_state.ctx.view = msg.view;
        new_state.responses.insert(from, msg);
        if new_state.responses.has_quorum() {
            return new_state.become_primary(output);
        }
        new_state.into()
    }

    fn become_primary(&mut self, output: &mut Vec<Envelope<Msg>>) -> VrState {
        let current = Latest {
            last_normal_view: self.last_normal_view,
            commit_num: self.commit_num,
            op: self.op,
            // TODO: FIXME: Cloning the log is expensive
            log: self.log.clone()
        };
        let latest = self.compute_latest_state(current);
        self.ctx.op = latest.op;
        self.ctx.log = latest.log;
        self.ctx.last_normal_view = self.view;
        self.broadcast_start_view_msg(latest.commit_num, output);
        let primary = Primary::from(self);
        primary.set_primary(output);
        primary.commit(latest.commit_num, output)
    }

    fn handle_start_view_change(self,
                                msg: StartViewChange,
                                from: Pid,
                                cid: CorrelationId,
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

    fn handle_do_view_change(self,
                             msg: vr_msg::DoViewChange,
                             from: Pid,
                             cid: CorrelationId,
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

    // Another replica was already elected primary for this view.
    fn handle_start_view(self,
                         msg: vr_msg::StartView,
                         from: Pid,
                         cid: CorrelationId,
                         output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        if msg.epoch < self.ctx.epoch {
            return self.into();
        }
        if msg.epoch == self.ctx.epoch && msg.view < self.ctx.view {
            return self.into();
        }
        // A primary has been elected in a new view / epoch
        // Even if the epoch is larger here, we will learn it and the new config by playing the log
        let StartView{view, op, log, commit_num, ..} = msg;
        Backup::become_backup(view, op, log, commit_num, output)
    }

    fn handle_tick(self, output: &mut Vec<Envelope<Msg>>) -> VrState {
        if self.state.responses.is_expired() {
            // We haven't changed views yet. Transition back to StartViewChange and try again.
            self.ctx.last_received_time = SteadyTime::now();
            self.ctx.view += 1;
            let new_state = StartViewChange::from(self);
            new_state.broadcast_start_view_change(output);
            return new_state.into();
        }
        self.into()
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
