/// The state where a replica is in the process of shutting down
///
/// The replica received a reconfiguration request in its log and it is not in the new
/// configuration.
/// The replica is waiting for a quorum of EpochStarted messages from the new config so that it can
/// shut down.
state!(Leaving {
    pub ctx: VrCtx,
    pub msgs: QuorumTracker<EpochStarted>
});

/// A replica is in this state after it has a reconfiguration request in its log and it is not in
/// the new configuration. It is waiting for f' + 1 EpochStarted messages so that it can shutdown.
handle!(EpochStarted, Leaving, {
    if msg.epoch > ctx.epoch {
        // There has already been *another* reconfiguration, so just transition to shutdown state
        return Shutdown::from(self).into();
    }
    self.msgs.insert(msg.from.clone(), msg);
    if self.msgs.has_quorum() {
        output.push(self.ctx.namespace_mgr_envelope(NamespaceMsg::Stop(self.ctx.pid.clone())));
        return Shutdown::from(self).into();
    }
    self.into()
});

handle!(Tick, Leaving, {
    if self.msgs.is_expired() {
      self.broadcast_start_epoch(output);
    }
    self.into()
});

impl Leaving {
    pub fn broadcast_start_epoch(&self, output: &mut Vec<Envelope>) {
        let msg = self.start_epoch_msg();
        let cid = CorrelationId::pid(self.ctx.pid.clone());
        self.ctx.broadcast(msg, cid, output);
    }

    fn start_epoch_msg(&self) -> rabble::Msg {
        rabble::Msg::User(Msg::Vr(StartEpoch {
            epoch: self.ctx.epoch,
            op: self.ctx.op,
            old_config: self.ctx.old_config.clone(),
            new_config: self.ctx.new_config.clone()
        }.into()))
    }
}
