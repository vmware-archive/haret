use std::convert::{From, Into};
use vr_fsm::{Transition, VrStates, Primary, Backup};
use vr_msg::{ClientOp, ClientRequest, Reconfiguration, ClientReply, Prepare, PrepareOk, Tick};
use vr_msg::{GetSate, Recovery, StartEpoch, ClientReply};
use vr_ctx::{VrCtx, DEFAULT_IDLE_TIMEOUT_MS};

impl Transition<Prepare> for Backup {
    fn next(mut self,
            prepare: Prepare,
            _: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        ctx.last_received_time = SteadyTime::now();
        let Prepare {op, commit_num, msg, ..} = prepare;
        if op == self.ctx.op + 1 {
            // This is the next op in order
            self.send_prepare_ok(msg, commit_num, cid, output);
            return self.commit(commit_num, output)
        } else if op > ctx.op + 1 {
            output.push(self.ctx.send_get_state_to_random_replica(cid));
            return WaitForNewState::from(self).into();
        }
        self.into()
    }
}

impl Transition<Commit> for Backup {
    fn next(mut self,
            msg: Commit,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        ctx.last_received_time = SteadyTime::now();
        let Commit {commit_num, ..} = msg;
        if commit_num == ctx.commit_num {
            // We are already up to date
            return self.into();
        } else if commit_num == ctx.op {
            return self.commit(commit_num, output);
        }
        output.push(self.ctx.send_get_state_to_random_replica(cid));
        WaitForNewState::from(self).into();
    }
}

impl Transition<Tick> for Backup {
    fn next(mut self,
            _: Tick,
            _: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        if self.ctx.idle_timeout() {
            self.ctx.reset_and_start_view_change(output);
            return WaitForStartViewChange::from(self).into();
        }
        self.into()
    }
}

impl Transition<GetState> for Backup {
    fn next(mut self,
            msg: GetState,
            _: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        let GetState {op, from, ..} = msg;
        output.push(self.ctx.send_new_state(op, from, cid));
        self.into()
    }
}

impl Transition<Recovery> for Backup {
    fn next(mut self,
            msg: Recovery,
            _: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        let Recovery {from, nonce} = msg;
        output.push(self.ctx.send_recovery_response(from, nonce, cid));
        self.into()
    }
}

impl Transition<StartEpoch> for Backup {
    fn next(mut self,
            msg: StartEpoch,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        self.ctx.send_epoch_started(msg, from, cid, output);
        self.into()
    }
}

impl Backup {
    pub fn new(ctx: VrCtx) -> Backup {
        Backup {
            ctx: ctx,
            primary: self.ctx.compute_primary(),
            idle_timeout: Duration::milliseconds(DEFAULT_IDLE_TIMEOUT_MS as i64)
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

    pub fn commit(&mut self, new_commit_num: u64, output: &mut Vec<FsmOutput>) -> VrStates {
        for i in self.commit_num..new_commit_num {
            let msg = self.log[i as usize].clone();
            match msg {
                ClientOp::Request(ClientRequest {op, ..}) => {
                    self.ctx.backend.call(op);
                },
                ClientOp::Reconfiguration(Reconfiguration {replicas, ..}) => {
                    self.ctx.update_for_new_epoch(i+1, replicas);
                    self.commit_num = i + 1;
                    output.push(self.ctx.announce_reconfiguration());
                    output.push(self.set_primary());
                    return self.enter_transitioning(output);
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
