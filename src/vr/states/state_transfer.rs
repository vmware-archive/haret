use std::convert::{From, Into};
use vr_fsm::{StateTransfer, Transition, VrState, Backup};

/// When a backup realizes it's behind it enters state transfer
///
/// In this state, the backup is waiting for a `NewState` message
state!(StateTransfer {
    pub ctx: VrCtx
});

handle!(NewState, StateTransfer, {
    // Replicas only respond to state transfer requests in normal mode in the same epoch and view
    self.become_backup(msg, output)
}

handle!(Prepare, StateTransfer, {
    up_to_date!(self, from, msg, cid, output);
    self.into();
}

handle!(Commit, StateTransfer, {
    up_to_date!(self, from, msg, cid, output);
    self.into();
}

handle!(StartViewChange, Primary, {
    up_to_date!(self, from, msg, cid, output);
    state.into()
});

handle!(DoViewChange, Primary, {
    up_to_date!(self, from, msg, cid, output);
    self.into()
});

handle!(StartView, Primary {
    up_to_date!(self, from, msg, cid, output);
    self.into()
}):

handle!(GetState, Primary, {
    up_to_date!(self, from, msg, cid, output);
    self.into()
});

handle!(Tick, StateTransfer, {
    if self.ctx.idle_timeout() {
        output.push(self.ctx.send_get_state_to_random_replica(cid));
    }
    self.into()
}

/// Convert any State to StateTransfer by taking its ctx
impl<T: State> From<T> for StateTransfer {
    fn from(state: T) -> StateTransfer {
        StateTransfer {
            ctx: state.ctx()
        }
    }
}

impl StateTransfer {
    pub fn become_backup(&mut self, msg: NewState, output: &mut Vec<Envelope>) -> VrState {
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

    /// When a new replica starts after reconfiguration it needs to send a get state request to all
    /// replicas to ensure it gets the latest state.
    pub fn start_reconfiguration(self,
                                 cid: CorrelationId,
                                 new_epoch: u64,
                                 output: &mut Vec<Envelope>) -> VrState
    {
        output.extend(self.old_config
                      .replicas
                      .iter()
                      .cloned()
                      .chain(self.new_config.replicas.iter().cloned())
                      .filter(|pid| *pid != self.pid)
                      .map(|pid| self.ctx.vr_new(pid, msg.clone(), cid.clone())));
        self.into()
    }

    /// We missed the reconfiguration, and don't know what the new config looks like or if the
    /// old replicas have shutdown. Therefore retrieve the config from the new primary.
    pub fn start_epoch<S>(state: S,
                          primary: Pid,
                          cid: CorrelationId,
                          new_epoch: u64,
                          new_view: u64,
                          output: &mut Vec<Envelope>) -> VrState
        where S: State
    {
        let mut new_state = StateTransfer::from(state);
        new_state.ctx.last_received_time = SteadyTime::now();
        new_state.ctx.epoch = new_epoch;
        new_state.ctx.view = new_view;
        new_state.ctx.op = new_state.commit_num;
        new_state.ctx.log.truncate(new_state.op as usize);
        let from = self.pid.clone();
        let msg = self.get_state_msg();
        output.push(Envelope::new(primary, from, msg, cid));
        new_state.into()
    }

    pub fn start_view<S>(state: S,
                         new_view: u64,
                         cid: CorrelationId,
                         output: &mut Vec<Envelope>) -> VrState
        where S: State
    {
        let new_state = StateTransfer::from(state);
        new_state.ctx.last_received_time = SteadyTime::now();
        new_state.ctx.view = new_view;
        new_state.ctx.op = new_state.commit_num;
        new_state.ctx.log.truncate(new_state.op as usize);
        new_state.send_get_state_to_random_replica(cid, output);
        new_state.into()
    }

    // Send a state transfer message
    pub fn start_same_view<S>(state: S,
                              cid: CorrelationId,
                              output: &mut Vec<Envelope>) -> VrState
        where S: State
    {
        let new_state = StateTransfer::from(state);
        new_state.send_get_state_to_random_replica(cid, output);
        new_state.into()
    }

    pub fn send_new_state(ctx: &VrCtx, op: u64, to: Pid, cid: CorrelationId) -> Envelope {
        let msg = StateTransfer::new_state_msg(ctx, op);
        Envelope::new(to, ctx.pid.clone(), msg, cid)
    }

    fn send_get_state_to_random_replica(&mut self,
                                        cid: CorrelationId,
                                        output: &mut Vec<Envelope>) {
        let msg = self.get_state_msg();
        let mut rng = thread_rng();
        let mut to = self.pid.clone();
        while to == self.pid {
            let index = rng.gen_range(0, self.new_config.replicas.len());
            to = self.new_config.replicas[index].clone()
        }
        output.push(Envelope::new(to, self.pid.clone(), msg, cid));
    }

    fn get_state_msg(&self) -> rabble::Msg {
        rabble::Msg::User(Msg::Vr(GetState {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            from: self.pid.clone()
        }.into()))
    }

    fn new_state_msg(ctx: &VrCtx, op: u64) -> rabble::Msg {
        rabble::Msg::User(Msg::Vr(NewState {
            epoch: ctx.epoch,
            view: ctx.view,
            op: ctx.op,
            primary: ctx.primary.clone(),
            commit_num: ctx.commit_num,
            log_tail: (&ctx.log[op as usize..ctx.op as usize]).to_vec()
        }.into()))
    }

}
