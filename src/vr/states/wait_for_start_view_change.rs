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
    if msg.view < self.ctx.view {
        return self.into();
    }
    self.handle_start_view_change(msg, output)
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
    if msg.view < self.ctx.view {
        return self.into();
    }

    WaitForDoViewChange::do_view_change(self, from, msg, output)
}

// Another replica was already elected primary for this view.
handle!(StartView, WaitForStartViewChange, {
    if msg.epoch < self.ctx.epoch {
        return self.into();
    }
    if msg.epoch == self.ctx.epoch && msg.view < self.ctx.view {
        return self.into();
    }
    // Even if the epoch is larger here, we will learn it and the new config by playing the log
    let StartView{view, op, log, commit_num, ..} = msg;
    Backup::become(view, op, log, commit_num, output)
}

handle!(Tick, WaitForStartViewChange, {
    if self.msgs.is_expired() {
        // We didn't receive quorum, increment the view and try again
        self.ctx.last_received_time = SteadyTime::now();
        self.ctx.view += 1;
        self.msgs = QuorumTracker::new(self.ctx.quorum, self.ctx.idle_timeout.clone());
        self.broadcaset_start_view_change(output);
    }
    self.into()
}

// Another replica was already elected primary for this view.
handle!(Prepare, WaitForStartViewChange, {
    if msg.epoch < self.ctx.epoch {
        return self.into();
    }
    if msg.epoch > self.ctx.epoch {
        // We missed the reconfiguration, and don't know what the new config looks like or if the
        // old replicas have shutdown. Therefore retrieve the config from the new primary.
        self.ctx.epoch = msg.epoch;
        self.ctx.view = msg.view;
        self.ctx.start_state_transfer_reconfig(from, output);
        return WaitForNewState::from(self).into();
    }
    if msg.view < self.ctx.view {
        return self.into();
    }

    // A primary has been elected in this or a later view. We missed the election.
    self.ctx.start_state_transfer_new_view(self.ctx.view, cid, output);
    WaitForNewState::from(self).into()
}

// Another replica was already elected primary for this view.
handle!(Commit, WaitForStartViewChange, {
    if msg.epoch < self.ctx.epoch {
        return self.into();
    }
    if msg.epoch > self.ctx.epoch {
        // We missed the reconfiguration, and don't know what the new config looks like or if the
        // old replicas have shutdown. Therefore retrieve the config from the new primary.
        self.ctx.epoch = msg.epoch;
        self.ctx.view = msg.view;
        self.ctx.start_state_transfer_reconfig(from, output);
        return WaitForNewState::from(self).into();
    }
    if msg.view < self.ctx.view {
        return self.into();
    }
    // A primary has been elected in this or a later view. We missed the election.
    self.ctx.start_state_transfer_new_view(self.ctx.view, cid, output);
    WaitForNewState::from(self).into()
}

impl<T: State> From<T> for WaitForStartViewChange {
    fn from(state: T) -> WaitForStartViewChange {
        let mut ctx = state.ctx();
        ctx.last_received_time = SteadyTime::now();
        ctx.view += 1;
        let tracker = QuorumTracker::new(ctx.quorum, ctx.idle_timeout.clone());
        WaitForStartViewChange {
            ctx: state.ctx,
            msgs: tracker
        }
    }
}

impl WaitForStartViewChange {
    fn handle_start_view_change(self,
                                msg: StartViewChange,
                                output: &mut Vec<FsmOutput>) -> VrStates
    {
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

    pub fn start_view_change<S: State>(state: S,
                                       msg: StartViewChange,
                                       output: &mut Vec<FsmOutput>) -> VrStates
    {
        let mut new_state = WaitForStartViewChange::from(state);
        new_state.broadcast_start_view_change(output);
        new_state.handle_start_view_change(msg, output)
    }

    pub fn broadcast_start_view_change(&mut self, output: &mut Vec<FsmOutput>) {
        self.ctx.last_received_time = SteadyTime::new();
        let msg = self.start_view_change_msg();
        let cid = CorrelationId::pid(self.pid.clone());
        self.ctx.broadcast(msg, cid, output);
    }

    pub fn start_view_change_msg(&self) -> VrMsg {
        StartViewChange {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            from: self.pid.clone()
        }.into()
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
