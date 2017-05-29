use std::convert::{From, Into};
#[macro_use]
use vr_fsm::{Transition, VrStates, Primary, Backup};
use vr_msg::{ClientOp, ClientRequest, Reconfiguration, ClientReply, Prepare, PrepareOk, Tick};
use vr_msg::{GetSate, Recovery, StartEpoch, ClientReply};
use vr_ctx::{VrCtx, DEFAULT_IDLE_TIMEOUT_MS};

handle!(Prepare, Backup, {
    up_to_date!(self, from, msg, cid, output);
    self.ctx.last_received_time = SteadyTime::now();
    let Prepare {op, commit_num, msg, ..} = prepare;
    if op == self.ctx.op + 1 {
        // This is the next op in order
        self.send_prepare_ok(msg, commit_num, cid, output);
        return self.commit(commit_num, output)
    } else if op > ctx.op + 1 {
        return StateTransfer::start_same_view(self, output);
    }
    self.into()
}

handle!(Commit, Backup, {
    up_to_date!(self, from, msg, cid, output);
    self.ctx.last_received_time = SteadyTime::now();
    if msg.commit_num == self.ctx.commit_num {
        // We are already up to date
        return self.into();
    } else if msg.commit_num == self.ctx.op {
        return self.commit(msg.commit_num, output);
    }
    StateTransfer::start_same_view(self, output)
}

handle!(StartViewChange, Backup, {
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
});

handle!(DoViewChange, Backup, {
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

handle!(StartView, Backup, {
    if msg.epoch < self.ctx.epoch {
        return self.into();
    }
    if msg.epoch == self.ctx.epoch && msg.view <= self.ctx.view {
        return self.into();
    }
    // A primary has been elected in a new view / epoch
    // Even if the epoch is larger here, we will learn it and the new config by playing the log
    let StartView {view, op, log, commit_num, ..} = msg;
    Backup::become_backup(view, op, log, commit_num, output)
});

handle!(Tick, Backup, {
    if self.ctx.idle_timeout() {
        self.ctx.last_received_time = SteadyTime::now();
        self.ctx.view += 1;
        let new_state = StartViewChange::from(self);
        new_state.broadcast_start_view_change(output);
        return new_state.into();
    }
    self.into()
}

handle!(GetState, Backup, {
    up_to_date!(self, from, msg, cid, output);
    let GetState {epoch, view, op} = msg;
    if epoch != self.ctx.epoch || view != self.ctx.view {
        return self.into()
    }
    output.push(self.ctx.send_new_state(op, from, cid));
    self.into()
}

handle!(Recovery, Backup, {
    output.push(self.ctx.send_recovery_response(from, msg.nonce, cid));
    self.into()
}

handle!(StartEpoch, Backup, {
    self.ctx.send_epoch_started(msg, from, cid, output);
    self.into()
}

impl Backup {
    pub fn new(ctx: VrCtx) -> Backup {
        Backup {
            ctx: ctx,
            primary: self.ctx.compute_primary()
        }
    }

    fn send_prepare_ok(&mut self,
                           msg: ClientOp, // ClientRequest | Reconfiguration
                           commit_num: u64,
                           cid: CorrelationId,
                           output: &mut Vec<FsmOutput>)
    {
        self.last_received_time = SteadyTime::now();
        self.op += 1;
        self.log.push(msg);
        output.push(self.send_to_primary(self.prepare_ok_msg(), cid));
    }

    /// Transition to a backup after receiving a `StartView` message
    pub fn become_backup<S: State>(state: S,
                            view: u64,
                            op: u64,
                            log: Vec<VrMsg>,
                            commit_num: u64,
                            output: &mut Vec<FsmOutput>) -> VrStates
    {
        state.ctx.last_received_time = SteadyTime::now();
        state.ctx.view = view;
        state.ctx.op = op;
        state.ctx.log = log;
        // TODO: This isn't correct if we transition to a new epoch
        state.ctx.last_normal_view = state.view;
        let backup = Backup::from(state);
        output.push(backup.set_primary());
        backup.commit(commit_num, output)
    }

    pub fn commit(&mut self, new_commit_num: u64, output: &mut Vec<FsmOutput>) -> VrStates {
        for i in self.commit_num..new_commit_num {
            let msg = self.log[i as usize].clone();
            match msg {
                ClientOp::Request(ClientRequest {op, ..}) => {
                    self.ctx.backend.call(op);
                },
                ClientOp::Reconfiguration(Reconfiguration {epoch, replicas, ..}) => {
                    self.ctx.epoch = epoch;
                    self.ctx.update_for_new_epoch(i+1, replicas);
                    output.push(self.ctx.announce_reconfiguration());
                    output.push(self.set_primary());

                    // If the reconfiguration is not the last in the log, we don't want to
                    // transition, as the reconfiguration has already happened.
                    if new_commit_num  == self.ctx.log.len() {
                        self.commit_num = new_commit_num;
                        return self.enter_transitioning(output);
                    }
                },
            }
        }
        self.commit_num = new_commit_num;
        self.into()
    }

    /// The backup has just committed the reconfiguration request. It must now determine whether it
    /// is the primary of view 0 in the new epoch, a backup in the new epoch, or it is being
    /// shutdown.
    fn enter_transitioning(mut self, output: &mut Vec<FsmOutput>) -> VrStates {
        if self.ctx.is_leaving() {
            return Leaving::from(self).into();
        }
        // Tell replicas that are being replaced to shutdown
        output.extend_from_slice(&ctx.broadcast_epoch_started(envelope.correlation_id));
        if self.ctx.is_primary() {
            self.reconfiguration_in_progress = false;
            // Become the primary
            Primary::from(self).into()
        }
        // Become a backup
        self.into()
    }

    fn set_primary(&mut self) -> FsmOutput {
        let primary = self.ctx.compute_primary();
        self.primary = primary.clone();
        FsmOutput::Announcement(NamespaceMsg::NewPrimary(primary), self.pid.clone())
    }
}

