use std::convert::{From, Into};
use vr_fsm::{WaitForNewState, Transition, VrStates, Backup};

handle!(NewState, WaitForNewState, {
    // Replicas only respond to state transfer requests in normal mode in the same epoch and view
    self.become_backup(msg, output)
}

handle!(Prepare, WaitForNewState, {

}

handle!(Tick, WaitForNewState, {
    if self.ctx.idle_timeout() {
        output.push(self.ctx.send_get_state_to_random_replica(cid));
    }
    self.into()
}

/// Convert any State to WaitForNewState by taking its ctx
impl<T: State> From<T> for WaitForNewState {
    fn from(state: T) -> WaitForNewState {
        WaitForNewState {
            ctx: state.ctx()
        }
    }
}

impl WaitForNewState {
    // TODO: Ensure that if we
    pub fn become_backup(&mut self, msg: NewState, output: &mut Vec<FsmOutput>) -> VrStates {
        self.last_received_time = SteadyTime::now();
        let NewState {view, op, commit_num, log_tail, ..} = msg;
        self.view = view;
        self.op = op;
        for m in log_tail {
            self.log.push(m);
        }
        let backup = Backup::from(self);
        output.push(backup.set_primary());
        backup.commit(commit_num, output)
    }

    pub fn start_state_transfer_new_view<S: State>(state: S,
                                                   new_view: u64,
                                                   cid: CorrelationId,
                                                   output: &mut Vec<FsmOutput>) -> VrStates
    {
        let new_state = WaitForNewState::from(state);
        new_state.ctx.last_received_time = SteadyTime::now();
        new_state.ctx.view = new_view;
        new_state.ctx.op = new_state.commit_num;
        new_state.ctx.log.truncate(new_state.op as usize);
        new_state.send_get_state_to_random_replica(cid, output);
        new_state.into()
    }

    pub fn start_state_transfer_same_view<S: State>(state: S,
                                                    cid: CorrelationId,
                                                    output: &mut Vec<FsmOutput>) -> VrStates
    {
        let new_state = WaitForNewState::from(state);
        new_state.send_get_state_to_random_replica(cid, output);
        new_state.into()
    }

    fn send_get_state_to_random_replica(&mut self,
                                        cid: CorrelationId,
                                        output: &mut Vec<FsmOutput>) {
        let msg = self.get_state_msg();
        let mut rng = thread_rng();
        let mut to = self.pid.clone();
        while to == self.pid {
            let index = rng.gen_range(0, self.new_config.replicas.len());
            to = self.new_config.replicas[index].clone()
        }
        output.push(FsmOutput::Vr(VrEnvelope::new(to, self.pid.clone(), msg, cid)));
    }

    fn get_state_msg(&self) -> VrMsg {
        GetState {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            from: self.pid.clone()
        }.into()
    }


}
