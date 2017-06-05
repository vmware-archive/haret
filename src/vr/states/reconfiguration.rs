use std::convert::{From, Into};
use rabble::{self, Pid, CorrelationId, Envelope};
use msg::Msg;
use vr::vr_fsm::{Transition, VrState, State};
use vr::VrCtx;
use vr::vr_msg::{VrMsg, EpochStarted};
use vr::states::StateTransfer;

/// The state of a replica when it first starts after becoming part of a group due to a
/// reconfiguration request
state!(Reconfiguration {
    ctx: VrCtx
});

impl Transition for Reconfiguration {
    fn handle(self,
              msg: VrMsg,
              from: Pid,
              cid: CorrelationId,
              output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        match msg {
            VrMsg::Tick => {
                let cid = CorrelationId::pid(self.ctx.pid.clone());
                let state = StateTransfer::new(self.ctx);
                state.start_reconfiguration(cid, output)
            }
            _ => self.into()
        }
    }
}

impl Reconfiguration {
    /// Ther replica gets created in Reconfiguration state when it is a new replica
    /// after reconfiguration commits. The only thing this replica does is wait for a tick
    /// so that it can start state transfer.
    pub fn new(ctx: VrCtx) -> Reconfiguration {
        Reconfiguration {
            ctx: ctx
        }
    }

    pub fn send_epoch_started(ctx: &VrCtx,
                              to: Pid,
                              cid: CorrelationId,
                              output: &mut Vec<Envelope<Msg>>)
    {
        let msg = Reconfiguration::epoch_started_msg(ctx);
        output.push(Envelope::new(to, ctx.pid.clone(), msg, Some(cid)));
    }

    pub fn broadcast_epoch_started(ctx: &VrCtx, output: &mut Vec<Envelope<Msg>>) {
        let msg = Reconfiguration::epoch_started_msg(ctx);
        let cid = CorrelationId::pid(ctx.pid.clone());
        output.extend(ctx.replicas_to_replace().iter().cloned().map(|r| {
            Envelope::new(r, ctx.pid.clone(), msg.clone(), Some(cid.clone()))
        }));
    }

    fn epoch_started_msg(ctx: &VrCtx) -> rabble::Msg<Msg> {
        EpochStarted {
            epoch: ctx.epoch
        }.into()
    }
}
