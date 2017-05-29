handle!(StartViewChange, DoViewChange, {
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

handle!(DoViewChange, DoViewChange, {
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
        self.state.responses.insert(from, msg);
        if self.state.has_quorum() {
            return self.become_primary(output)
        }
        return self.into();
    }
    DoViewChange::start_do_view_change(self, from, msg, output)
}

// Another replica was already elected primary for this view.
handle!(StartView, DoViewChange, {
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

handle!(Tick, DoViewChange, {
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

handle!(Prepare, WaitForStartView, {
    up_to_date!(self, from, msg, cid, output);
    StateTransfer::start_same_view(self, cid, output)
}

handle!(Commit, WaitForStartView, {
    up_to_date!(self, from, msg, cid, output);
    StateTransfer::start_same_view(self, cid, output)
}


impl<T: State> From<T> for DoViewChange {
    fn from(state: T) -> DoViewChange {
        state.ctx.last_received_time = SteadyTime::now();
        DoViewChange {
            ctx: state.ctx
            state: DoViewChangeState::new(state.ctx.quorum, state.idle_timeout.clone())
        }
    }
}

impl DoViewChange {
    pub fn start_do_view_change<S: State>(state: S,
                                    from: Pid,
                                    msg: DoViewChange,
                                    output: &mut Vec<FsmOutput>) -> VrStates
    {
        // This is a later view in the same epoch. Other nodes have decided that we
        // should be the primary in this view via StartViewChange quorum.
        let new_state = DoViewChange::from(state);
        new_state.ctx.view = msg.view;
        new_state.state.responses.insert(from, msg);
        if new_state.state.has_quorum() {
            return new_state.become_primary(output);
        }
        new_state.into()
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
        let primary = Primary::from(self);
        output.push(primary.set_primary());
        primary.commit(latest.commit_num, output)
    }

}
