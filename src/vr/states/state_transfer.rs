use std::convert::{From, Into};
use rand::thread_rng;
use rabble::{self, Pid, CorrelationId, Envelope};
use time::{SteadyTime, Duration};
use msg::Msg;
use vr::vr_fsm::{Transition, VrState, State};
use vr::vr_msg::{ClientOp, ClientRequest, Reconfiguration, ClientReply, Prepare, PrepareOk, Tick};
use vr::vr_msg::{self, VrMsg, GetState, Recovery, StartEpoch, StartViewChange, NewState};
use vr::vr_ctx::{VrCtx, DEFAULT_IDLE_TIMEOUT_MS, DEFAULT_PRIMARY_TICK_MS};
use super::utils::QuorumTracker;
use super::Backup;

/// When a backup realizes it's behind it enters state transfer
///
/// In this state, the backup is waiting for a `NewState` message
state!(StateTransfer {
    ctx: VrCtx
});

impl Transition for StateTransfer {
    fn handle(self,
              msg: VrMsg,
              from: Pid,
              cid: CorrelationId,
              output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        match msg {
            // Replicas only respond to state transfer requests in normal mode
            // in the same epoch and view
            VrMsg::NewState(msg) => self.become_backup(msg, output),
            VrMsg::Prepare(msg)
                | VrMsg::Commit(msg)
                | VrMsg::StartViewChange(msg)
                | VrMsg::DoViewChange(msg)
                | VrMsg::StartView(msg)
                | VrMsg::GetState(msg) =>
            {
                up_to_date!(self, from, msg, cid, output);
                self.into();
            },
            VrMsg::Tick => {
                if self.ctx.idle_timeout() {
                    self.ctx.send_get_state_to_random_replica(cid, output);
                }
                self.into()
            }
        }
    }
}

impl StateTransfer {
    /// Enter state transfer
    pub fn enter(ctx: VrCtx) -> VrState {
        StateTransfer {
            ctx: ctx
        }
    }

    pub fn become_backup(&mut self, msg: NewState, output: &mut Vec<Envelope<Msg>>) -> VrState {
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
                                 output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        let msg = self.get_state_msg();
        let from = self.pid;
        output.extend(self.old_config
                      .replicas
                      .iter()
                      .cloned()
                      .chain(self.new_config.replicas.iter().cloned())
                      .filter(|pid| *pid != self.pid)
                      .map(|pid| Envelope::new(pid, from.clone(), msg.clone(), cid.clone())));
        self.into()
    }

    /// We missed the reconfiguration, and don't know what the new config looks like or if the
    /// old replicas have shutdown. Therefore retrieve the config from the new primary.
    pub fn start_epoch<S>(state: S,
                          primary: Pid,
                          cid: CorrelationId,
                          new_epoch: u64,
                          new_view: u64,
                          output: &mut Vec<Envelope<Msg>>) -> VrState
        where S: State
    {
        let mut new_state = StateTransfer { ctx: state.ctx() };
        new_state.ctx.last_received_time = SteadyTime::now();
        new_state.ctx.epoch = new_epoch;
        new_state.ctx.view = new_view;
        new_state.ctx.op = new_state.commit_num;
        new_state.ctx.log.truncate(new_state.op as usize);
        let from = new_state.ctx.pid.clone();
        let msg = new_state.get_state_msg();
        output.push(Envelope::new(primary, from, msg, cid));
        new_state.into()
    }

    pub fn start_view<S>(state: S,
                         new_view: u64,
                         cid: CorrelationId,
                         output: &mut Vec<Envelope<Msg>>) -> VrState
        where S: State
    {
        let mut new_state = StateTransfer { ctx: state.ctx() };
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
                              output: &mut Vec<Envelope<Msg>>) -> VrState
        where S: State
    {
        let new_state = StateTransfer { ctx: state.ctx() };
        new_state.send_get_state_to_random_replica(cid, output);
        new_state.into()
    }

    pub fn send_new_state(ctx: &VrCtx, op: u64, to: Pid, cid: CorrelationId) -> Envelope<Msg> {
        let msg = StateTransfer::new_state_msg(ctx, op);
        Envelope::new(to, ctx.pid.clone(), msg, cid)
    }

    fn send_get_state_to_random_replica(&mut self,
                                        cid: CorrelationId,
                                        output: &mut Vec<Envelope<Msg>>) {
        let msg = self.get_state_msg();
        let mut rng = thread_rng();
        let mut to = self.pid.clone();
        while to == self.pid {
            let index = rng.gen_range(0, self.new_config.replicas.len());
            to = self.new_config.replicas[index].clone()
        }
        output.push(Envelope::new(to, self.pid.clone(), msg, cid));
    }

    fn get_state_msg(&self) -> rabble::Msg<Msg> {
        GetState {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            from: self.pid.clone()
        }.into()
    }

    fn new_state_msg(ctx: &VrCtx, op: u64) -> rabble::Msg<Msg> {
        NewState {
            epoch: ctx.epoch,
            view: ctx.view,
            op: ctx.op,
            primary: ctx.primary.clone(),
            commit_num: ctx.commit_num,
            log_tail: (&ctx.log[op as usize..ctx.op as usize]).to_vec()
        }.into()
    }

}
