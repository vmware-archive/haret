// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::convert::{From, Into};
use rabble::{self, Pid, CorrelationId, Envelope};
use time::{SteadyTime, Duration};
use msg::Msg;
use vr::vr_fsm::{Transition, VrState, State};
use vr::vr_msg::{ClientOp, ClientRequest, Reconfiguration, ClientReply, Prepare, PrepareOk, Tick};
use vr::vr_msg::{self, VrMsg, GetState, Recovery, StartEpoch};
use vr::vr_ctx::{VrCtx, DEFAULT_IDLE_TIMEOUT_MS, DEFAULT_PRIMARY_TICK_MS};
use super::utils::QuorumTracker;
use super::{Primary, Backup, StateTransfer, DoViewChange, StartViewChange};

/// The part of the view change state in the VR protocol state machine where a replica is waiting
/// for a `StartView` message from the new primary. It has already sent a `DoViewChange` to the
/// proposed primary for this view.
state!(StartView {
    ctx: VrCtx,
    primary: Pid
});

impl Transition for StartView {
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

impl From<StartViewChange> for StartView {
    fn from(state: StartViewChange) -> StartView {
        let primary = state.ctx.compute_primary();
        StartView {
            ctx: state.ctx,
            primary: primary
        }
    }
}

impl StartView {
    fn handle_start_view_change(self,
                                msg: vr_msg::StartViewChange,
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
        if msg.view <= self.ctx.view {
            return self.into();
        }
        DoViewChange::start_do_view_change(self, from, msg, output)
    }

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
        // Even if the epoch is larger here, we will learn it and the new config by playing the log
        let vr_msg::StartView {view, op, log, commit_num, ..} = msg;
        Backup::become_backup(self.ctx, view, op, log, commit_num, output)
    }

    fn handle_tick(mut self, output: &mut Vec<Envelope<Msg>>) -> VrState {
        if self.ctx.idle_timeout() {
            // We haven't changed views yet. Transition back to StartViewChange and try again.
            self.ctx.last_received_time = SteadyTime::now();
            self.ctx.view += 1;
            let mut new_state = StartViewChange::new(self.ctx);
            new_state.broadcast_start_view_change(output);
            return new_state.into();
        }
        self.into()
    }
}
