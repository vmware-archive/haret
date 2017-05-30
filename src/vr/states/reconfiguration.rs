/// The state of a replica when it first starts after becoming part of a group due to a
/// reconfiguration request
state!(Reconfiguration {
    pub ctx: VrCtx
});

handle!(Tick, Recovery, {
    let cid = CorrelationId::Pid(self.ctx.pid.clone());
    let state = StateTransfer::from(self);
    state.start_reconfiguration(cid, self.new_config.epoch, output)
});

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
                              output: &mut Vec<Envelope>)
    {
        let msg = Reconfiguration::epoch_started_msg(ctx);
        output.push(Envelope::new(to, ctx.pid.clone(), msg, cid)):
    }

    pub fn broadcast_epoch_started(ctx: &VrCtx, output: &mut Vec<Envelope>) {
        let msg = Reconfiguration::epoch_started_msg(ctx);
        let cid = CorrelationId::pid(ctx.pid.clone());
        output.extend(ctx.replicas_to_replace().iter().cloned().map(|r| {
            Envelope::new(r, ctx.pid.clone(), msg.clone(), cid.clone()))
        });
    }

    fn epoch_started_msg(ctx: &VrCtx) -> rabble::Msg {
        rabble::Msg::User(Msg::Vr(EpochStarted {
            epoch: ctx.epoch,
            from: ctx.pid.clone()
        }.into()))
    }
}
