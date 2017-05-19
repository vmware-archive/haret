use std::convert::{From, Into};
use rabble::{self, Pid, CorrelationId, Envelope};
use msg::Msg;
use NamespaceMsg;
use vr::vr_fsm::{Transition, VrState, State};
use vr::vr_ctx::VrCtx;
use vr::vr_msg::{VrMsg, EpochStarted, StartEpoch};
use vr::states::Shutdown;
use vr::states::utils::QuorumTracker;

/// The state where a replica is in the process of shutting down
///
/// The replica received a reconfiguration request in its log and it is not in the new
/// configuration.
/// The replica is waiting for a quorum of EpochStarted messages from the new config so that it can
/// shut down.
state!(Leaving {
    ctx: VrCtx,
    msgs: QuorumTracker<EpochStarted>
});

impl Transition for Leaving {
    fn handle(mut self,
              msg: VrMsg,
              from: Pid,
              _: CorrelationId,
              output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        match msg {
            VrMsg::EpochStarted(msg) => {
                if msg.epoch > self.ctx.epoch {
                    // There has already been *another* reconfiguration,
                    // so just transition to shutdown state
                    return Shutdown::enter(self.ctx);
                }
                self.msgs.insert(from, msg);
                if self.msgs.has_quorum() {
                    let ns_msg = NamespaceMsg::Stop(self.ctx.pid.clone());
                    output.push(self.ctx.namespace_mgr_envelope(ns_msg));
                    return Shutdown::enter(self.ctx);
                }
                self.into()
            },
            VrMsg::Tick => {
                if self.msgs.is_expired() {
                    self.broadcast_start_epoch(output);
                }
                self.into()
            },
            _ => self.into()
        }
    }
}

impl Leaving {
    pub fn leave(ctx: VrCtx) -> VrState {
        let quorum = ctx.quorum;
        Leaving {
            msgs: QuorumTracker::new(quorum, ctx.idle_timeout_ms),
            ctx: ctx
        }.into()
    }

    pub fn broadcast_start_epoch(&self, output: &mut Vec<Envelope<Msg>>) {
        let msg = self.start_epoch_msg();
        let cid = CorrelationId::pid(self.ctx.pid.clone());
        self.ctx.broadcast(msg, cid, output);
    }

    fn start_epoch_msg(&self) -> rabble::Msg<Msg> {
        StartEpoch {
            epoch: self.ctx.epoch,
            op: self.ctx.op,
            old_config: self.ctx.old_config.clone(),
            new_config: self.ctx.new_config.clone()
        }.into()
    }
}
