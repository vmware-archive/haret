use std::convert::{From, Into};
use rabble::{self, Pid, CorrelationId, Envelope};
use time::SteadyTime;
use msg::Msg;
use vr::vr_fsm::{Transition, VrState, State};
use vr::vr_msg::{self, VrMsg};
use vr::vr_ctx::VrCtx;
use namespace_msg::{NamespaceId, NamespaceMsg};
use super::common::view_change;
use super::utils::QuorumTracker;
use super::{StateTransfer, DoViewChange, StartView};

/// The part of the view change state in the VR protocol state machine where a replica is waiting
/// for a quorum of `StartViewChange` messages.
state!(StartViewChange {
    ctx: VrCtx,
    msgs: QuorumTracker<vr_msg::StartViewChange>
});

impl Transition for StartViewChange {
    fn handle(self,
              msg: VrMsg,
              from: Pid,
              cid: CorrelationId,
              output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        match msg {
            VrMsg::StartViewChange(msg) => self.handle_start_view_change(msg, from, output),
            VrMsg::DoViewChange(msg) => self.handle_do_view_change(msg, from, output),
            VrMsg::StartView(msg) => view_change::handle_start_view(self, msg, output),
            VrMsg::Tick => view_change::handle_tick(self, output),
            VrMsg::Prepare(msg) => {
                up_to_date!(self, from, msg, cid, output);
                // Another replica was already elected primary for this view.
                StateTransfer::start_same_view(self.ctx, cid, output)
            }
            VrMsg::Commit(msg) => {
                up_to_date!(self, from, msg, cid, output);
                // Another replica was already elected primary for this view.
                StateTransfer::start_same_view(self.ctx, cid, output)
            },
            _ => self.into()
        }
    }
}

impl StartViewChange {
    pub fn new(ctx: VrCtx) -> StartViewChange {
        let tracker = QuorumTracker::new(ctx.quorum, ctx.idle_timeout_ms);
        StartViewChange {
            ctx: ctx,
            msgs: tracker
        }
    }

    /// Enter StartViewChangeState
    pub fn enter(mut ctx: VrCtx, output: &mut Vec<Envelope<Msg>>) -> VrState {
        ctx.last_received_time = SteadyTime::now();
        ctx.view += 1;
        let mut state = StartViewChange::new(ctx);
        state.broadcast_start_view_change(output);
        state.into()
    }

    pub fn start_view_change(mut ctx: VrCtx,
                             from: Pid,
                             msg: vr_msg::StartViewChange,
                             output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        ctx.last_received_time = SteadyTime::now();
        ctx.view = msg.view;
        let mut state = StartViewChange::new(ctx);
        state.clear_primary(output);
        state.broadcast_start_view_change(output);
        state.check_quorum(from, msg, output)
    }

    pub fn clear_primary(&mut self, output: &mut Vec<Envelope<Msg>>) {
        let namespace_id = NamespaceId(self.ctx.pid.group.as_ref().unwrap().to_owned());
        let msg = NamespaceMsg::ClearPrimary(namespace_id);
        output.push(self.ctx.namespace_mgr_envelope(msg));
    }

    fn check_quorum(mut self,
                    from: Pid,
                    msg: vr_msg::StartViewChange,
                    output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        self.ctx.last_received_time = SteadyTime::now();
        self.msgs.insert(from, msg);
        if self.msgs.has_quorum() {
            let computed_primary = self.ctx.compute_primary();
            if computed_primary == self.ctx.pid {
                return DoViewChange::enter(self.ctx)
            }
            output.push(self.send_do_view_change(computed_primary));
            return StartView::from(self).into();
        }
        self.into()
    }

    pub fn broadcast_start_view_change(&mut self, output: &mut Vec<Envelope<Msg>>) {
        self.ctx.last_received_time = SteadyTime::now();
        let msg = self.start_view_change_msg();
        let cid = CorrelationId::pid(self.ctx.pid.clone());
        self.ctx.broadcast(msg, cid, output);
    }

    fn start_view_change_msg(&self) -> rabble::Msg<Msg> {
        vr_msg::StartViewChange {
            epoch: self.ctx.epoch,
            view: self.ctx.view,
            op: self.ctx.op
        }.into()
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
        if msg.view < self.ctx.view {
            return self.into();
        }
        if msg.view == self.ctx.view {
            return self.check_quorum(from, msg, output)
        }
        StartViewChange::start_view_change(self.ctx, from, msg, output)
    }

    // Another replica got quorum of StartViewChange messages for this view and computed
    // that we are the primary for this view.
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
        if msg.view < self.ctx.view {
            return self.into();
        }
        DoViewChange::start_do_view_change(self, from, msg, output)
    }

    fn send_do_view_change(&self, new_primary: Pid) -> Envelope<Msg> {
        let cid = CorrelationId::pid(self.ctx.pid.clone());
        let msg = self.do_view_change_msg();
        Envelope::new(new_primary, self.ctx.pid.clone(), msg, Some(cid))
    }

    fn do_view_change_msg(&self) -> rabble::Msg<Msg> {
        vr_msg::DoViewChange {
            epoch: self.ctx.epoch,
            view: self.ctx.view,
            op: self.ctx.op,
            last_normal_view: self.ctx.last_normal_view,
            log: self.ctx.log.clone(),
            commit_num: self.ctx.commit_num
        }.into()
    }
}
