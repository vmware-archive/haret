handle!(NewState, ReconfigurationWaitForNewState, {
    if msg.op < self.ctx.new_config.op {
        // This NewState msg doesn't have the reconfiguration request. Go back to waiting.
        return self.into();
    }
    let new_state = WaitForNewState::from(self);
    new_state.become_backup(msg, output)
});

handle!(Tick, ReconfigurationWaitForNewState, {
    if self.ctx.idle_timeout() {
        self.ctx.start_state_transfer_reconfiguration(output);
    }
    self.into()
});
