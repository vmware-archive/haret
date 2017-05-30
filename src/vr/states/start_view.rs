/// The part of the view change state in the VR protocol state machine where a replica is waiting
/// for a `StartView` message from the new primary. It has already sent a `DoViewChange` to the
/// proposed primary for this view.
state!(StartView {
    pub ctx: VrCtx,
    pub primary: Pid
});

handle!(StartViewChange, StartView, {
    // Old messages we want to ignore. For New ones we want to wait until a primary is elected,
    // since we know we are out of date and need to perform state transfer, which will fail until
    // a replica is in normal mode.
    if msg.epoch != self.ctx.epoch {
        return self.into();
    }
    if msg.view <= self.ctx.view {
        return self.into();
    }
    StartViewChange::start_view_change(self, msg, output)
}

handle!(DoViewChange, StartView, {
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
});

handle!(StartView, StartView, {
    if msg.epoch < self.ctx.epoch {
        return self.into();
    }
    if msg.epoch == self.ctx.epoch && msg.view < self.ctx.view {
        return self.into();
    }
    // Even if the epoch is larger here, we will learn it and the new config by playing the log
    let StartView {view, op, log, commit_num, ..} = msg;
    Backup::become_backup(view, op, log, commit_num, output)
}

handle!(Tick, StartView, {
    if self.ctx.idle_timeout() {
        // We haven't changed views yet. Transition back to StartViewChange and try again.
        self.ctx.last_received_time = SteadyTime::now();
        self.ctx.view += 1;
        let new_state = StartViewChange::from(self);
        new_state.broadcast_start_view_change(output);
        return new_state.into();
    }
    self.into()
}

handle!(Prepare, StartView, {
    up_to_date!(self, from, msg, cid, output);
    StateTransfer::start_same_view(self, cid, output)
}

handle!(Commit, StartView, {
    up_to_date!(self, from, msg, cid, output);
    StateTransfer::start_same_view(self, cid, output)
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
