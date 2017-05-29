handle!(RecoveryResponse, Recovery, {
    self.update_recovery_state(msg);
    self.commit_recovery(output)
});

handle!(Tick, Recovery, {
    if self.state.responses.is_expired() {
        let cid = CorrelationId::Pid(self.ctx.pid.clone());
        self.state = RecoveryState::new(self.ctx.quorum, self.ctx.idle_timeout.clone());

        self.ctx.broadcast(self.recovery_msg(), cid, output);
    }
    self.into()
});

impl Recovery {
    pub fn commit_recovery(&mut self, output: &mut Vec<FsmOutput>) -> VrStates {
        if self.state.has_quorum(self.ctx.view) {
            let commit_num = {
                let mut primary_state = self.state.primary.as_ref().unwrap();
                self.ctx.op = primary_state.op;
                mem::swap(&mut self.ctx.log, &mut primary_state.log);
                primary_state.commit_num
            };
            let backup = Backup::from(self);
            output.push(backup.set_primary());
            backup.commit(commit_num, output)
            return backup.into();
        }
        self.into()
    }

    pub fn update_recovery_state(&mut self, msg: RecoveryResponse) {
        let RecoveryResponse {epoch, view, nonce, from, op, commit_num, log} = msg;
        if nonce != self.state.nonce {
            return;
        }
        if epoch < self.ctx.epoch {
            return;
        }

        // TODO: If we get a response from a replica in a later epoch, we are in a weird state
        // We missed a reconfiguration and the namespace manager hasn't learned of the epoch
        // change yet. What we really want is to wait for the namespace manager to learn of the
        // replicas in the later epoch and restart the replica. For now we're ignoring that this
        // situation can even occur. We just return without processing the message.. This is
        // clearly wrong.
        if epoch > self.ctx.epoch {
            error!(ctx.logger,
                   "EPOCH RECONFIGURATION DURING RECOVERY: Replica {} in a bad state",
                   self.pid);
            return;
        }

        if view > self.ctx.view {
            self.ctx.view = view;
        }

        let response_from_primary = op.is_some();
        if response_from_primary && view == self.ctx.view {
            self.recovery_state.primary = Some(RecoveryPrimary {
                pid: from.clone(),
                view: view,
                op: op.unwrap(),
                commit_num: commit_num.unwrap(),
                log: log.unwrap()
            });
        }
        self.recovery_state.responses.insert(from, ())
    }

    fn recovery_msg(&self) -> VrMsg {
        Recovery {
            from: self.ctx.pid.clone(),
            nonce: self.state.nonce.clone()
        }.into()
    }

}
