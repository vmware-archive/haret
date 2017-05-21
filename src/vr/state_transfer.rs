use std::convert::{From, Into};
use vr_fsm::{WaitForNewState, Transition, VrStates, Backup};

impl Transition<NewState> for WaitForNewState {
    fn next(mut self,
            msg: NewState,
            _: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>)
    {
        self.become_backup(msg, output)
    }
}

impl Transition<Tick> for WaitForNewState {
    fn next(mut self,
            _: Tick,
            _: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>)
    {
        if self.ctx.idle_timeout() {
            output.push(self.ctx.send_get_state_to_random_replica(cid));
        }
        self.into()
    }
}

impl WaitForNewState {
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
}
