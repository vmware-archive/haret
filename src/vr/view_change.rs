use std::convert::{From, Into};
use vr_fsm::{Transition, VrStates, Primary, Backup};
use vr_msg::{ClientOp, ClientRequest, Reconfiguration, ClientReply, Prepare, PrepareOk, Tick};
use vr_msg::{GetSate, Recovery, StartEpoch, ClientReply};
use vr_ctx::{VrCtx, DEFAULT_IDLE_TIMEOUT_MS, DEFAULT_PRIMARY_TICK_MS};

impl Transition<StartViewChange> for WaitForStartViewChange {
    fn next(mut self,
            msg: StartViewChange,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        self.ctx.last_received_time = SteadyTime::now();
        self.insert_view_change_message(from, msg.into());
        if self.has_view_change_quorum() {
            let computed_primary = self.ctx.compute_primary();
            if compute_primary == self.ctx.pid {
                self.ctx.reset_view_change_state(ctx.view, output);
                return WaitForDoViewChange::from(self).into();
            }
            output.push(self.send_do_view_change(computed_primary));
            return WaitForStartView::from(self).into();
        }
        self.into()
    }
}

// Another replica got quorum of StartViewChange messages for this view and computed
// that we are the primary for this view.
impl Transition<DoViewChange> for WaitForStartViewChange {
    fn next(mut self,
            msg: DoViewChange,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        self.ctx.last_received_time = SteadyTime::now();
        self.ctx.reset_view_change_state(ctx.view, output);
        self.insert_view_change_message(from, msg.into());
        if self.has_view_change_quorum() {
            return self.become_primary(output)
        }
        WaitForDoViewChange::from(self).into()
    }
}

// Another replica was already elected primary for this view.
impl Transition<StartView> for WaitForStartViewChange {
    fn next(mut self,
            msg: StartView,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        let StartView{view, op, log, commit_num, ..} = msg;
        self.become_backup(view, op, log, commit_num, output)
    }
}

impl Transition<Tick> for WaitForStartViewChange {
    fn next(mut self,
            msg: Tick,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        if self.view_change_expired() {
            self.ctx.reset_and_start_view_change(output);
        }
        self.into()
    }
}

// Another replica was already elected primary for this view.
impl Transition<Prepare> for WaitForStartViewChange {
    fn next(mut self,
            _: Prepare,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        self.ctx.start_state_transfer_new_view(self.ctx.view, cid, output);
        WaitForNewState::from(self).into()
    }
}

// Another replica was already elected primary for this view.
impl Transition<Commit> for WaitForStartViewChange {
    fn next(mut self,
            _: Commit,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        self.ctx.start_state_transfer_new_view(self.ctx.view, cid, output);
        WaitForNewState::from(self).into()
    }
}

impl Transition<DoViewChange> for WaitForDoViewChange {
    fn next(mut self,
            msg: DoViewChange,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        self.insert_view_change_message(from, msg);
        if self.has_view_change_quorum() {
            return self.become_primary(output)
        }
        WaitForDoViewChange::from(self).into()
    }
}

impl Transition<Tick> for WaitForDoViewChange {
    fn next(mut self,
            _: Tick,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        if self.view_change_expired() {
            // We haven't changed views yet. Try again.
            self.ctx.reset_and_start_view_change(output);
            return WaitForStartViewChange::from(self).into();
        }
        self.into()
    }
}

impl Transition<StartView> for WaitForStartView {
    fn next(mut self,
            msg: StartView,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        let StartView {view, op, log, commit_num, ..} = msg;
        self.become_backup(view, op, log, commit_num, output)
    }
}

impl Transition<Tick> for WaitForStartView {
    fn next(mut self,
            msg: Tick,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        if self.view_change_expired() {
            // We haven't changed views yet. Try again.
            self.ctx.reset_and_start_view_change(output);
            return WaitForStartViewChange::from(self).into();
        }
        self.into()
    }
}

impl Transition<Prepare> for WaitForStartView {
    fn next(mut self,
            _: Prepare,
            _: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        self.ctx.start_state_transfer_new_view(self.ctx.view, cid, output);
        WaitForNewState::from(self).into()
    }
}

impl Transition<Commit> for WaitForStartView {
    fn next(mut self,
            _: Commit,
            _: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        self.ctx.start_state_transfer_new_view(self.ctx.view, cid, output);
        WaitForNewState::from(self).into()
    }
}

impl WaitForStartViewChange {
    pub fn become_backup(&mut self,
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

    fn insert_view_change_message(&mut self, from: Pid, msg: VrMsg) {
        self.view_change_state.responses.insert(from, msg);
    }

    pub fn view_change_expired(&self) -> bool {
        self.view_change_state.map_or(false, |s| s.responses.is_expired())
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
        self.op = latest.op;
        self.log = latest.log;
        self.last_normal_view = self.view;
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

    pub fn start_state_transfer_new_view(&mut self,
                                         new_view: u64,
                                         cid: CorrelationId,
                                         output: &mut Vec<FsmOutput>)
    {
        self.last_received_time = SteadyTime::now();
        self.view = new_view;
        self.op = self.commit_num;
        self.log.truncate(self.op as usize);
        output.push(self.ctx.send_get_state_to_random_replica(cid));
    }



}
