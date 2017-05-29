/// A replica is in this state after it has a reconfiguration request in its log and it is not in
/// the new configuration. It is waiting for f' + 1 EpochStarted messages so that it can shutdown.
handle!(EpochStarted, Leaving, {
    if msg.epoch > ctx.epoch {
        // There has already been *another* reconfiguration, so just transition to shutdown state
        return Shutdown::from(self).into();
    }
    self.msgs.insert(msg.from.clone(), msg);
    if self.msgs.has_quorum() {
        output.push(FsmOutput::Announcement(
                NamespaceMsg::Stop(ctx.pid.clone()),
                ctx.pid.clone()));
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
