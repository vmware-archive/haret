use std::convert::{From, Into};
use rabble::{self, Pid, CorrelationId, Envelope};
use time::{SteadyTime};
use msg::Msg;
use NamespaceMsg;
use vr::vr_fsm::{Transition, VrState, State};
use vr::vr_msg::{ClientOp, ClientRequest, Prepare, PrepareOk, Commit};
use vr::vr_msg::{self, VrMsg, GetState, StartView};
use vr::vr_ctx::VrCtx;
use super::{Primary, StateTransfer, Recovery, Reconfiguration, Leaving};
use super::{StartViewChange, DoViewChange};


/// The backup state of the VR protocol operating in normal mode
state!(Backup {
    ctx: VrCtx,
    primary: Pid
});

impl Transition for Backup {
    fn handle(self,
              msg: VrMsg,
              from: Pid,
              cid: CorrelationId,
              output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        match msg {
            VrMsg::Prepare(msg) => self.handle_prepare(msg, from, cid, output),
            VrMsg::Commit(msg) => self.handle_commit(msg, from, cid, output),
            VrMsg::StartViewChange(msg) => self.handle_start_view_change(msg, from, output),
            VrMsg::DoViewChange(msg) => self.handle_do_view_change(msg, from, output),
            VrMsg::StartView(msg) => self.handle_start_view(msg, output),
            VrMsg::Tick => self.handle_tick(output),
            VrMsg::GetState(msg) => self.handle_get_state(msg, from, cid, output),
            VrMsg::Recovery(msg) => self.handle_recovery(msg, from, cid, output),
            VrMsg::StartEpoch(_) => self.handle_start_epoch(from, cid, output),
            _ => self.into()
        }
    }
}

impl Backup {
    pub fn new(ctx: VrCtx) -> Backup {
        let primary = ctx.compute_primary();
        Backup {
            ctx: ctx,
            primary: primary
        }
    }

    // Enter the backup state
    pub fn enter(ctx: VrCtx) -> VrState {
        Backup::new(ctx).into()
    }

    fn send_prepare_ok(&mut self,
                           msg: ClientOp, // ClientRequest | Reconfiguration
                           cid: CorrelationId,
                           output: &mut Vec<Envelope<Msg>>)
    {
        self.ctx.last_received_time = SteadyTime::now();
        self.ctx.op += 1;
        self.ctx.log.push(msg);
        output.push(self.send_to_primary(self.prepare_ok_msg(), cid));
    }

    /// Transition to a backup after receiving a `StartView` message
    pub fn become_backup(mut ctx: VrCtx,
                         view: u64,
                         op: u64,
                         log: Vec<ClientOp>,
                         commit_num: u64,
                         output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        ctx.last_received_time = SteadyTime::now();
        ctx.op = op;
        ctx.log = log;
        // TODO: This isn't correct if we transition to a new epoch
        ctx.last_normal_view = ctx.view;
        ctx.view = view;
        let mut backup = Backup::new(ctx);
        backup.set_primary(output);
        backup.commit(commit_num, output)
    }

    pub fn commit(mut self, new_commit_num: u64, output: &mut Vec<Envelope<Msg>>) -> VrState {
        for i in self.ctx.commit_num..new_commit_num {
            let msg = self.ctx.log[i as usize].clone();
            match msg {
                ClientOp::Request(ClientRequest {op, ..}) => {
                    self.ctx.backend.call(op);
                },
                ClientOp::Reconfiguration(vr_msg::Reconfiguration {epoch,
                                                                   replicas, ..}) =>
                {
                    self.ctx.epoch = epoch + 1;
                    self.ctx.update_for_new_epoch(i+1, replicas);
                    self.ctx.announce_reconfiguration(output);
                    self.set_primary(output);

                    // If the reconfiguration is not the last in the log, we don't want to
                    // transition, as the reconfiguration has already happened.
                    if new_commit_num == self.ctx.log.len() as u64 {
                        self.ctx.commit_num = new_commit_num;
                        return self.enter_transitioning(output);
                    }
                },
            }
        }
        self.ctx.commit_num = new_commit_num;
        self.into()
    }

    fn handle_prepare(mut self,
                      msg: Prepare,
                      from: Pid,
                      cid: CorrelationId,
                      output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        up_to_date!(self, from, msg, cid, output);
        self.ctx.last_received_time = SteadyTime::now();
        let Prepare {op, commit_num, msg, ..} = msg;
        if op == self.ctx.op + 1 {
            // This is the next op in order
            self.send_prepare_ok(msg, cid, output);
            return self.commit(commit_num, output)
        } else if op > self.ctx.op + 1 {
            return StateTransfer::start_same_view(self.ctx, cid, output);
        }
        self.into()
    }

    fn handle_commit(mut self,
                     msg: Commit,
                     from: Pid,
                     cid: CorrelationId,
                     output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        up_to_date!(self, from, msg, cid, output);
        self.ctx.last_received_time = SteadyTime::now();
        if msg.commit_num == self.ctx.commit_num {
            // We are already up to date
            return self.into();
        } else if msg.commit_num == self.ctx.op {
            return self.commit(msg.commit_num, output);
        }
        StateTransfer::start_same_view(self.ctx, cid, output)
    }

    fn handle_start_view_change(self,
                                msg: vr_msg::StartViewChange,
                                from: Pid,
                                output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        // Old messages we want to ignore. For New ones we want to wait until a primary is elected,
        // since we know we are out of date and need to perform state transfer, which will fail until
        // a replica is in normal mode.
        if msg.epoch != self.ctx.epoch {
            return self.into();
        }
        if msg.view <= self.ctx.view {
            return self.into();
        }

        StartViewChange::start_view_change(self.ctx, from, msg, output)
    }

    fn handle_do_view_change(self,
                             msg: vr_msg::DoViewChange,
                             from: Pid,
                             output: &mut Vec<Envelope<Msg>>) -> VrState
    {
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
    }

    fn handle_start_view(self,
                         msg: StartView,
                         output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        if msg.epoch < self.ctx.epoch {
            return self.into();
        }
        if msg.epoch == self.ctx.epoch && msg.view <= self.ctx.view {
            return self.into();
        }
        // A primary has been elected in a new view / epoch
        // Even if the epoch is larger here, we will learn it and the new config by playing the log
        let StartView {view, op, log, commit_num, ..} = msg;
        Backup::become_backup(self.ctx, view, op, log, commit_num, output)
    }

    fn handle_tick(self, output: &mut Vec<Envelope<Msg>>) -> VrState {
        if self.ctx.idle_timeout() {
            return StartViewChange::enter(self.ctx, output);
        }
        self.into()
    }

    fn handle_get_state(self,
                        msg: GetState,
                        from: Pid,
                        cid: CorrelationId,
                        output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        up_to_date!(self, from, msg, cid, output);
        let GetState {epoch, view, op} = msg;
        if epoch != self.ctx.epoch || view != self.ctx.view {
            return self.into()
        }
        StateTransfer::send_new_state(&self.ctx, op, from, cid, output);
        self.into()
    }

    fn handle_recovery(self,
                       msg: vr_msg::Recovery,
                       from: Pid,
                       cid: CorrelationId,
                       output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        Recovery::send_response(&self.ctx, from, msg.nonce, cid, output);
        self.into()
    }

    fn handle_start_epoch(self,
                          from: Pid,
                          cid: CorrelationId,
                          output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        Reconfiguration::send_epoch_started(&self.ctx, from, cid, output);
        self.into()
    }

    /// The backup has just committed the reconfiguration request. It must now determine whether it
    /// is the primary of view 0 in the new epoch, a backup in the new epoch, or it is being
    /// shutdown.
    fn enter_transitioning(self, output: &mut Vec<Envelope<Msg>>) -> VrState {
        if self.ctx.is_leaving() {
            return Leaving::leave(self.ctx)
        }
        // Tell replicas that are being replaced to shutdown
        Reconfiguration::broadcast_epoch_started(&self.ctx, output);
        if self.ctx.is_primary() {
            // Become the primary
            return Primary::enter(self.ctx);
        }
        // Become a backup
        self.into()
    }

    pub fn set_primary(&mut self, output: &mut Vec<Envelope<Msg>>) {
        let primary = self.ctx.compute_primary();
        self.primary = primary.clone();
        output.push(self.ctx.namespace_mgr_envelope(NamespaceMsg::NewPrimary(primary)));
    }

    fn send_to_primary(&self, msg: rabble::Msg<Msg>, cid: CorrelationId) -> Envelope<Msg> {
        Envelope::new(self.primary.clone(), self.ctx.pid.clone(), msg, Some(cid))
    }

    pub fn prepare_ok_msg(&self) -> rabble::Msg<Msg> {
        PrepareOk {
            epoch: self.ctx.epoch,
            view: self.ctx.view,
            op: self.ctx.op
        }.into()
    }
}
