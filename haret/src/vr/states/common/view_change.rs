// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use time::SteadyTime;
use rabble::{Envelope, CorrelationId};
use msg::Msg;
use vr::vr_fsm::{State, VrState};
use vr::vr_msg::StartView;
use vr::states::{Backup, StartViewChange};

pub fn handle_start_view<T: State>(state: T,
                                   msg: StartView,
                                   cid: CorrelationId,
                                   output: &mut Vec<Envelope<Msg>>) -> VrState
{
    if msg.epoch < state.borrow_ctx().epoch {
        return state.into();
    }
    if msg.epoch == state.borrow_ctx().epoch && msg.view < state.borrow_ctx().view {
        return state.into();
    }
    // Even if the epoch is larger here, we will learn it and the new config by playing the log
    let StartView {view, op, log_start, log_tail, commit_num, ..} = msg;
    Backup::become_backup(state.ctx(), view, op, log_start, log_tail, commit_num, cid, output)
}

pub fn handle_tick<T: State>(state: T, output: &mut Vec<Envelope<Msg>>) -> VrState {
    if state.borrow_ctx().idle_timeout() {
        // We haven't changed views yet. Transition back to StartViewChange and try again.
        let mut new_state = StartViewChange::new(state.ctx());
        new_state.ctx.last_received_time = SteadyTime::now();
        new_state.ctx.view += 1;
        new_state.broadcast_start_view_change(output);
        return new_state.into();
    }
    state.into()
}
