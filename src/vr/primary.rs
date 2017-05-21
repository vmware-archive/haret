use std::convert::{From, Into};
use vr_fsm::{Transition, VrStates, Primary, Backup};
use vr_msg::{ClientOp, ClientRequest, Reconfiguration, ClientReply, Prepare, PrepareOk, Tick};
use vr_msg::{GetSate, Recovery, StartEpoch, ClientReply};
use vr_ctx::{VrCtx, DEFAULT_IDLE_TIMEOUT_MS, DEFAULT_PRIMARY_TICK_MS};

impl Transition<ClientRequest> for Primary {
    fn next(mut self,
            msg: ClientRequest,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        self.send_prepare(msg, from, cid, output);
        self.into()
    }
}

impl Transition<PrepareOk> for Primary {
    fn next(mut self,
            msg: PrepareOk,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        let PrepareOk {op, from, ..} = msg;
        if op <= ctx.commit_num {
            return self.into();
        }
        if self.has_commit_quorum(op, from) {
            debug!(self.ctx.logger, "primary: has commit quorum: op = {}", op);
            return self.commit(op, output);
        }
        self.into()
    }
}

impl Transition<Tick> for Primary {
    fn next(mut self,
            _: Tick,
            _: Pid,
            _: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    if self.idle_timeout() {
        self.broadcast_commit_msg(output);
    }
    self.into()
}

impl Transition<GetState> for Primary {
    fn next(mut self,
            msg: GetState,
            _: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        let GetState {op, from, ..} = msg;
        output.push(self.ctx.send_new_state(op, from, cid));
        self.into()
    }
}

impl Transition<Recovery> for Primary {
    fn next(mut self,
            msg: Recovery,
            _: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        let Recovery {from, nonce} = msg;
        output.push(self.ctx.send_recovery_response(from, nonce, cid));
        self.into()
    }
}

impl Transition<Reconfiguration> for Primary {
    fn next(mut self,
            msg: Reconfiguration,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        if let Some(err_envelope) = self.validate_reconfig(&msg, &from, &cid) {
            output.push(err_envelope);
            return self.into();
        }
        self.reconfiguration_in_progress = true;
        self.send_prepare(msg, from, cid, output);
        self.into()
    }
}

impl Transition<StartEpoch> for Primary {
    fn next(mut self,
            msg: StartEpoch,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        self.ctx.send_epoch_started(msg, from, cid, output);
        self.into()
    }
}


impl Primary {
    fn new(ctx: VrCtx) -> Primary {
        Primary {
            ctx: ctx,
            prepare_requests: PrepareRequests::new(new_config.replicas.len(),
                                                   DEFAULT_IDLE_TIMEOUT_MS),
            tick_ms: DEFAULT_PRIMARY_TICK_MS,
            reconfiguration_in_progress: false
        }
    }

    fn send_prepare(&mut self,
                    msg: Msg,
                    from: Pid,
                    cid: CorrelationId,
                    output: &mut Vec<FsmOutput>)
    {
        self.ctx.last_received_time = SteadyTime::now();
        self.ctx.op += 1;
        let prepare = Prepare {
            epoch: self.ctx.epoch,
            view: self.ctx.view,
            op: self.ctx.op,
            commit_num: self.ctx.commit_num,
            msg: msg.clone().into()
        };
        self.ctx.log.push(msg.into());
        self.prepare_requests.new_prepare(self.ctx.op, cid.clone());
        self.ctx.broadcast(prepare.into(), cid, output);
    }

    pub fn has_commit_quorum(&mut self, op: u64, from: Pid) -> bool {
        self.prepare_requests.insert(op, from);
        self.prepare_requests.has_quorum(op)
    }

    pub fn commit(self, new_commit_num: u64, output: &mut Vec<FsmOutput>) -> VrStates {
        let lowest_prepared_op = self.prepare_requests.lowest_op;
        let mut iter = self.prepare_requests.remove(new_commit_num).into_iter();
        let commit_num = self.ctx.commit_num;
        for i in commit_num..new_commit_num {
            let msg = self.ctx.log[i as usize].clone();
            self.commit_num = i + 1;
            match msg {
                ClientOp::Request(ClientRequest {op, request_num, ..}) => {
                    let rsp = self.ctx.backend.call(op);
                    if i >= lowest_prepared_op - 1 {
                        // This primary prepared this request, and wasn't elected after it was
                        // prepared but before it committed it.
                        let cid = iter.next().unwrap().correlation_id;
                        let from = cid.pid.clone();
                        output.push(self.client_reply(rsp, request_num, &from, &cid));
                    }
                },
                ClientOp::Reconfiguration(Reconfiguration {client_req_num, epoch, replicas, ..})=>
                {
                    let cid =
                        if i >= lowest_prepared_op - 1 {
                            // This replica prepared this request as primary, and wasn't elected
                            // after it was prepared but before this replica committed it.
                            let cid = iter.next().unwrap().correlation_id;
                            let to = cid.pid.clone();
                            let from = self.ctx.pid.clone();
                            let reply = ClientReply {
                                epoch: epoch,
                                view: 0,
                                request_num: client_req_num,
                                value: VrApiRsp::Ok
                            }.into();
                            let cid2 = cid.clone();
                            output.push(FsmOutput::Vr(VrEnvelope::new(to, from, reply, cid2)));
                            cid
                        } else {
                            CorrelationId::pid(self.ctx.pid.clone())
                        };

                    // If the Reconfiguration is the last entry in the log, then this is either the
                    // primary that received the client request, or a view change occurred and this
                    // is a new primary in the epoch of the reconfiguration request processing the
                    // reconfiguration
                    if new_commit_num == self.ctx.log.len() {
                        self.ctx.epoch += 1;
                        self.ctx.update_for_new_epoch(i+1, replicas);
                        output.push(self.ctx.announce_reconfiguration());
                        output.push(self.set_primary());
                        output.extend(self.broadcast_commit_msg_old());
                        // We don't bother sending 'StartEpoch` requests to the new replicas here,
                        // since they aren't yet online.
                        return self.enter_transitioning(output);
                    }
                }
            }
        }
        self.into()
    }

    /// The primary has just committed the reconfiguration request. It must now determine whether it
    /// is the primary of view 0 in the new epoch, a backup in the new epoch, or it is being
    /// shutdown.
    fn enter_transitioning(mut self, output: &mut Vec<FsmOutput>) -> VrStates {
        if self.ctx.is_leaving() {
            return Leaving::from(self).into();
        }
        // Tell replicas that are being replaced to shutdown
        output.extend_from_slice(&ctx.broadcast_epoch_started(envelope.correlation_id));
        if self.ctx.is_primary() {
            self.reconfiguration_in_progress = false;
            // Stay a primary
            return self.into();
        }
        // Become a backup
        Backup::from(self).into()
    }

    /// Return Some(error msg) if the reconfiguration request is invalid. Return None on success.
    fn validate_reconfig(&self
                         from: &Pid,
                         msg: &Reconfiguration,
                         cid: &CorrelationId) -> Option<FsmOutput>
    {
        let Reconfiguration {client_req_num, epoch, ref replicas} = msg;
        let err = if self.reconfiguration_in_progress {
            Some(VrApiError::Msg("Reconfiguration in progress".to_owned()))
        } else if replicas.len() < 3 {
            Some(VrApiError::NotEnoughReplicas)
        } else if epoch < self.epoch || epoch > self.epoch {
            Some(VrApiError::BadEpoch)
        } else if *replicas == self.new_config.replicas {
            Some(VrApiError::Msg("No change to existing configuration".to_owned()))
        } else {
            None
        };
        err.map(|e| self.client_reply(VrApiRsp::Error(err), client_req_num, from, cid))
    }

    fn client_reply(&self,
                    rsp: VrApiRsp,
                    client_req_num: u64,
                    to: &Pid,
                    cid: &CorrelationId) -> FsmOutput
    {
        let reply = self.ctx.client_reply_msg(client_req_num, rsp);
        FsmOutput::Vr(VrEnvelope::new(to.clone(), self.ctx.pid.clone(), reply, cid.clone()))
    }

    fn client_reply_msg(&self, client_req_num: u64, value: VrApiRsp) -> VrMsg {
        ClientReply {
            epoch: self.ctx.epoch,
            view: self.ctx.view,
            request_num: client_req_num,
            value: value
        }.into()
    }

    fn commit_msg(&self) -> VrMsg {
        Commit {
            epoch: self.ctx.epoch,
            view: self.ctx.view,
            commit_num: self.ctx.commit_num
        }.into()
    }

    fn broadcast_commit_msg(&self, output: &mut Vec<FsmOutput>) {
        self.ctx.broadcast(self.commit_msg(), CorrelationId::pid(self.ctx.pid.clone()), output);
    }

    fn broadcast_commit_msg_old(&self, output: &mut Vec<FsmOutput>) {
        self.ctx.broadcast_old(self.commit_msg(), CorrelationId::pid(self.ctx.pid.clone()))
    }

    fn set_primary(&mut self) -> FsmOutput {
        let primary = self.ctx.compute_primary();
        FsmOutput::Announcement(NamespaceMsg::NewPrimary(primary), self.ctx.pid.clone())
    }


    fn idle_timeout(&self) -> bool {
        SteadyTime::now() - self.ctx.last_received_time >
            Duration::milliseconds(self.tick_ms as i64)
    }
}
