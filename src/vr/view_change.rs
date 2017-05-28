use std::convert::{From, Into};
use vr_fsm::{Transition, VrStates, Primary, Backup};
use vr_msg::{ClientOp, ClientRequest, Reconfiguration, ClientReply, Prepare, PrepareOk, Tick};
use vr_msg::{GetSate, Recovery, StartEpoch, ClientReply};
use vr_ctx::{VrCtx, DEFAULT_IDLE_TIMEOUT_MS, DEFAULT_PRIMARY_TICK_MS};

handle!(StartViewChange, WaitForStartViewChange, {
    // Old messages we want to ignore. For New ones we want to wait until a primary is elected,
    // since we know we are out of date and need to perform state transfer, which will fail until
    // a replica is in normal mode.
    if msg.epoch != self.ctx.epoch {
        return self.into();
    }

    self.ctx.last_received_time = SteadyTime::now();
    self.msgs.insert(from, msg);
    if self.msgs.has_quorum() {
        let computed_primary = self.ctx.compute_primary();
        if compute_primary == self.ctx.pid {
            return WaitForDoViewChange::from(self).into();
        }
        output.push(self.send_do_view_change(computed_primary));
        return WaitForStartView::from(self).into();
    }
    self.into()
}

// Another replica got quorum of StartViewChange messages for this view and computed
// that we are the primary for this view.
handle!(DoViewChange, WaitForStartViewChange, {
    // Old messages we want to ignore. We don't want to become the primary here either, since we
    // didn't participate in reconfiguration, and therefore haven't yet learned about how many
    // replicas we need to get quorum. We just want to wait until another replica is elected
    // primary and then transfer state from it.
    if msg.epoch != self.ctx.epoch {
        return self.into();
    }

    let new_state = WaitForDoViewChange::from(self);
    new_state.state.responses.insert(from, msg);
    if new_state.state.has_quorum() {
        return new_state.become_primary(output)
    }
    new_state.into()
}

// Another replica was already elected primary for this view.
handle!(StartView, WaitForStartViewChange, {
    let StartView{view, op, log, commit_num, ..} = msg;
    self.become_backup(view, op, log, commit_num, output)
}

handle!(Tick, WaitForStartViewChange, {
    if self.msgs.is_expired() {
        // We didn't receive quorum, increment the view and try again
        self.ctx.last_received_time = SteadyTime::now();
        self.ctx.view += 1;
        self.msgs = QuorumTracker::new(self.ctx.quorum, self.ctx.idle_timeout.clone());
        self.ctx.start_view_change(output);
    }
    self.into()
}

// Another replica was already elected primary for this view.
handle!(Prepare, WaitForStartViewChange, {
    self.ctx.start_state_transfer_new_view(self.ctx.view, cid, output);
    WaitForNewState::from(self).into()
}

// Another replica was already elected primary for this view.
handle!(Commit, WaitForStartViewChange, {
    self.ctx.start_state_transfer_new_view(self.ctx.view, cid, output);
    WaitForNewState::from(self).into()
}

handle!(DoViewChange, WaitForDoViewChange, {
    self.state.responses.insert(from, msg);
    if self.state.has_quorum() {
        return self.become_primary(output)
    }
    self.into()
}

handle!(Tick, WaitForDoViewChange, {
    if self.state.responses.is_expired() {
        // We haven't changed views yet. Transition back to WaitForStartViewChange and try again.
        let new_state = WaitForStartViewChange::from(self);
        new_state.ctx.start_view_change(output);
        return new_state.into();
    }
    self.into()
}

handle!(StartView, WaitForStartView, {
    let StartView {view, op, log, commit_num, ..} = msg;
    self.become_backup(view, op, log, commit_num, output)
}

handle!(Tick, WaitForStartView, {
    if self.ctx.idle_timeout() {
        // We haven't changed views yet. Transition back to WaitForStartViewChange and try again.
        let new_state = WaitForStartViewChange::from(self);
        new_state.ctx.start_view_change(output);
        return new_state.into();
    }
    self.into()
}

handle!(Prepare, WaitForStartView, {
    self.ctx.start_state_transfer_new_view(self.ctx.view, cid, output);
    WaitForNewState::from(self).into()
}

handle!(Commit, WaitForStartView, {
    self.ctx.start_state_transfer_new_view(self.ctx.view, cid, output);
    WaitForNewState::from(self).into()
}


impl From<WaitForStartViewChange> for WaitForDoViewChange {
    fn from(state: WaitForStartViewChange) -> WaitForDoViewChange {
        state.ctx.last_received_time = SteadyTime::now();
        WaitForDoViewChange {
            ctx: state.ctx
            state: DoViewChangeState::new(state.ctx.quorum, state.idle_timeout.clone())
        }
    }
}

impl From<WaitForStartViewChange> for WaitForStartView {
    fn from(state: WaitForStartViewChange) -> WaitForStartView {
        let primary = state.ctx.compute_primary();
        WaitForStartView {
            ctx: state.ctx,
            primary: primary
        }
    }
}

// Quorum of DoViewChange messages was not received, so try again in the next view
impl From<WaitForDoViewChange> for WaitForStartViewChange {
    fn from(state: WaitForDoViewChange) -> WaitForStartViewChange {
        state.ctx.last_received_time = SteadyTime::now();
        state.ctx.view += 1;
        WaitForStartViewChange {
            ctx: state.ctx,
            msgs: QuorumTracker::new(self.ctx.quorum, self.ctx.idle_timeout.clone());
        }
    }
}

// Quorum of DoViewChange messages was not received, so try again in the next view
impl From<WaitForDoViewChange> for WaitForStartViewChange {
    fn from(state: WaitForDoViewChange) -> WaitForStartViewChange {
        state.ctx.last_received_time = SteadyTime::now();
        state.ctx.view += 1;
        WaitForStartViewChange {
            ctx: state.ctx,
            msgs: QuorumTracker::new(self.ctx.quorum, self.ctx.idle_timeout.clone());
        }
    }
}

// A StartView message was not received from the new primary, so try again in the next view
impl From<WaitForStartView>> for WaitForStartViewChange {
    fn from(state: WaitForStartView)) -> WaitForStartViewChange {
        state.ctx.last_received_time = SteadyTime::now();
        state.ctx.view += 1;
        WaitForStartViewChange {
            ctx: state.ctx,
            msgs: QuorumTracker::new(self.ctx.quorum, self.ctx.idle_timeout.clone());
        }
    }
}

impl From<Backup> for WaitForStartViewChange {
    fn from(state: Backup) -> WaitForStartViewChange {
        state.ctx.last_received_time = SteadyTime::now();
        state.ctx.view += 1;
        WaitForStartViewChange {
            ctx: state.ctx,
            msgs: QuorumTracker::new(self.ctx.quorum, self.ctx.idle_timeout.clone());
        }
    }
}


impl WaitForStartViewChange {
    pub fn become_backup(self,
                         view: u64,
                         op: u64,
                         log: Vec<VrMsg>,
                         commit_num: u64,
                         output: &mut Vec<FsmOutput>) -> VrStates
    {
        self.ctx.last_received_time = SteadyTime::now();
        self.ctx.view = view;
        self.ctx.op = op;
        self.ctx.log = log;
        self.ctx.last_normal_view = self.view;
        let backup = Backup::from(self);
        output.push(backup.set_primary());
        backup.commit(commit_num, output)
    }


    fn send_do_view_change(&self, new_primary: Pid) -> FsmOutput {
        let cid = CorrelationId::pid(self.pid.clone());
        let msg = self.do_view_change_msg();
        FsmOutput::Vr(VrEnvelope::new(new_primary, self.pid.clone(), msg, cid))
    }

    fn do_view_change_msg(&self) -> VrMsg {
        DoViewChange {
            epoch: self.ctx.epoch,
            view: self.ctx.view,
            op: self.ctx.op,
            from: self.ctx.pid.clone(),
            last_normal_view: self.ctx.last_normal_view,
            log: self.ctx.log.clone(),
            commit_num: self.ctx.commit_num
        }.into()
    }

    fn become_primary(&mut self, output: &mut Vec<FsmOutput>) -> VrStates {
        let current = Latest {
            last_normal_view: self.last_normal_view,
            commit_num: self.commit_num,
            op: self.op,
            // TODO: FIXME: Cloning the log is expensive
            log: self.log.clone()
        };
        let latest = self.view_change_state.compute_latest_state(current);
        self.ctx.op = latest.op;
        self.ctx.log = latest.log;
        self.ctx.last_normal_view = self.view;
        self.broadcast_start_view_msg(latest.commit_num, output);
        info!(self.logger, "Elected primary";
              "primary" => format!("{:?}", self.primary),
              "view" => self.view);
        let primary = Primary::from(self);
        output.push(primary.set_primary());
        primary.commit(latest.commit_num, output)
    }

    fn broadcast_start_view_msg(&self, new_commit_num: u64, output: &mut Vec<FsmOutput>) {
        let msg = self.start_view_msg(new_commit_num);
        let cid = CorrelationId::pid(self.pid.clone());
        self.ctx.broadcast(msg, cid, output);
    }

    fn start_view_msg(&self, new_commit_num: u64) -> VrMsg {
        StartView {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            log: self.log.clone(),
            commit_num: new_commit_num
        }.into()
    }
}
