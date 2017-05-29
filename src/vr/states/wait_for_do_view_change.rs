handle!(StartViewChange, WaitForDoViewChange, {
    // Old messages we want to ignore. For New ones we want to wait until a primary is elected,
    // since we know we are out of date and need to perform state transfer, which will fail until
    // a replica is in normal mode.
    if msg.epoch != self.ctx.epoch {
        return self.into();
    }
    if msg.view <= self.ctx.view {
        return self.into();
    }

    WaitForStartViewChange::start_view_change(self, msg, output)
}

handle!(DoViewChange, WaitForDoViewChange, {
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

    self.state.responses.insert(from, msg);
    if self.state.has_quorum() {
        return self.become_primary(output)
    }
    self.into()
}

// Another replica was already elected primary for this view.
handle!(StartView, WaitForDoViewChange, {
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

handle!(Tick, WaitForDoViewChange, {
    if self.state.responses.is_expired() {
        // We haven't changed views yet. Transition back to WaitForStartViewChange and try again.
        let new_state = WaitForStartViewChange::from(self);
        new_state.broadcast_start_view_change(output);
        return new_state.into();
    }
    self.into()
}

handle!(Prepare, WaitForStartView, {
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

handle!(Commit, WaitForStartView, {
    if msg.epoch < self.ctx.epoch {
        return self.into();
    }
    if msg.epoch > self.ctx.epoch {
        // We missed the reconfiguration, and don't know what the new config looks like or if the
        // old replicas have shutdown. Therefore retrieve the config from the new primary.
        self.ctx.epoch = msg.epoch;
        self.ctx.view = msg.view;
        self.ctx.start_state_transfer_primary_in_new_epoch(from, output);
        return WaitForNewState::from(self).into();
    }
    if msg.view < self.ctx.view {
        return self.into();
    }
    // A primary has been elected in this or a later view. We missed the election.
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

impl WaitForDoViewChange {
    pub fn do_view_Change<S: State>(state: S,
                                    from: Pid,
                                    msg: DoViewChange,
                                    output: &mut Vec<FsmOutput>) -> VrStates
    {
        // This is a later view in the same epoch. Other nodes have decided that we
        // should be the primary in this view via StartViewChange quorum.
        let new_state = WaitForDoViewChange::from(state);
        new_state.state.responses.insert(from, msg);
        if new_state.state.has_quorum() {
            return new_state.become_primary(output);
        }
        new_state.into()
    }
}
