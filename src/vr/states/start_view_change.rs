use std::convert::{From, Into};
use vr_fsm::{Transition, VrState, Primary, Backup};
use vr_msg::{ClientOp, ClientRequest, Reconfiguration, ClientReply, Prepare, PrepareOk, Tick};
use vr_msg::{GetSate, Recovery, StartEpoch, ClientReply};
use vr_ctx::{VrCtx, DEFAULT_IDLE_TIMEOUT_MS, DEFAULT_PRIMARY_TICK_MS};

/// The part of the view change state in the VR protocol state machine where a replica is waiting
/// for a quorum of `StartViewChange` messages.
state!(StartViewChange {
    pub ctx: VrCtx,
    pub msgs: QuorumTracker<StartViewChange>
});

handle!(StartViewChange, StartViewChange, {
    // Old messages we want to ignore. For New ones we want to wait until a primary is elected,
    // since we know we are out of date and need to perform state transfer, which will fail until
    // a replica is in normal mode.
    if msg.epoch != self.ctx.epoch {
        return self.into();
    }
    if msg.view < self.ctx.view {
        return self.into();
    }
    if msg.view == self.ctx.view {
        return self.handle_start_view_change(msg, output)
    }
    self.start_view_change(self, msg, output)
}

// Another replica got quorum of StartViewChange messages for this view and computed
// that we are the primary for this view.
handle!(DoViewChange, StartViewChange, {
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
    DoViewChange::start_do_view_change(self, from, msg, output)
}

// Another replica was already elected primary for this view.
handle!(StartView, StartViewChange, {
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

handle!(Tick, StartViewChange, {
    if self.msgs.is_expired() {
        // We didn't receive quorum, increment the view and try again
        self.ctx.last_received_time = SteadyTime::now();
        self.ctx.view += 1;
        self.msgs = QuorumTracker::new(self.ctx.quorum, self.ctx.idle_timeout.clone());
        self.broadcast_start_view_change(output);
    }
    self.into()
}

// Another replica was already elected primary for this view.
handle!(Prepare, StartViewChange, {
    up_to_date!(self, from, msg, cid, output);
    StateTransfer::start_same_view(self, cid, output)
}

// Another replica was already elected primary for this view.
handle!(Commit, StartViewChange, {
    up_to_date!(self, from, msg, cid, output);
    StateTransfer::start_same_view(self, cid, output)
}

impl<T: State> From<T> for StartViewChange {
    fn from(state: T) -> StartViewChange {
        let mut ctx = state.ctx();
        ctx.last_received_time = SteadyTime::now();
        let tracker = QuorumTracker::new(ctx.quorum, ctx.idle_timeout.clone());
        StartViewChange {
            ctx: state.ctx,
            msgs: tracker
        }
    }
}

impl StartViewChange {
    fn handle_start_view_change(self,
                                msg: StartViewChange,
                                output: &mut Vec<Envelope>) -> VrState
    {
        self.ctx.last_received_time = SteadyTime::now();
        self.msgs.insert(from, msg);
        if self.msgs.has_quorum() {
            let computed_primary = self.ctx.compute_primary();
            if compute_primary == self.ctx.pid {
                return DoViewChange::from(self).into();
            }
            output.push(self.send_do_view_change(computed_primary));
            return StartView::from(self).into();
        }
        self.into()
    }

    pub fn start_view_change<S: State>(state: S,
                                       msg: StartViewChange,
                                       output: &mut Vec<Envelope>) -> VrState
    {
        let mut new_state = StartViewChange::from(state);
        new_state.view = msg.view;
        new_state.broadcast_start_view_change(output);
        new_state.handle_start_view_change(msg, output)
    }

    pub fn broadcast_start_view_change(&mut self, output: &mut Vec<Envelope>) {
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

    fn send_do_view_change(&self, new_primary: Pid) -> Envelope {
        let cid = CorrelationId::pid(self.pid.clone());
        let msg = self.do_view_change_msg();
        Envelope::new(new_primary, self.pid.clone(), msg, cid)
    }

    fn do_view_change_msg(&self) -> rabble::Msg {
        rabble::Msg::User(Msg::Vr(DoViewChange {
            epoch: self.ctx.epoch,
            view: self.ctx.view,
            op: self.ctx.op,
            from: self.ctx.pid.clone(),
            last_normal_view: self.ctx.last_normal_view,
            log: self.ctx.log.clone(),
            commit_num: self.ctx.commit_num
        }.into()))
    }

    fn broadcast_start_view_msg(&self, new_commit_num: u64, output: &mut Vec<Envelope>) {
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
