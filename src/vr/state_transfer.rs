use std::convert::{From, Into};
use vr_fsm::{WaitForNewState, Transition, VrStates, Backup};

handle!(NewState, WaitForNewState, {
    self.become_backup(msg, output)
}

handle!(Tick, WaitForNewState, {
    if self.ctx.idle_timeout() {
        output.push(self.ctx.send_get_state_to_random_replica(cid));
    }
    self.into()
}

impl From<WaitForStartViewChange> for WaitForNewState {
    fn from(state: WaitForStartViewChange) -> WaitForNewState {
        WaitForNewState {
            ctx: state.ctx
        }
    }
}

impl From<WaitForStartView> for WaitForNewState {
    fn from(state: WaitForStartView) -> WaitForNewState {
        WaitForNewState {
            ctx: state.ctx
        }
    }
}

impl From<Backup> for WaitForNewState {
    fn from(state: Backup) -> WaitForNewState {
        WaitForNewState {
            ctx: state.ctx
        }
    }
}

impl From<ReconfigurationWaitForNewState> for WaitForNewState {
    fn from(state: ReconfigurationWaitForNewState) -> WaitForNewState {
        WaitForNewState {
            ctx: state.ctx
        }
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
