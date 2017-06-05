use std::convert::{From, Into};
use rabble::{self, Pid, CorrelationId, Envelope};
use time::{SteadyTime, Duration};
use msg::Msg;
use vr::vr_fsm::{Transition, VrState, State};
use vr::vr_msg::{self, ClientOp, ClientRequest, ClientReply, Prepare, PrepareOk, Commit};
use vr::vr_msg::{VrMsg, GetState};
use vr::vr_ctx::{VrCtx, DEFAULT_IDLE_TIMEOUT_MS, DEFAULT_PRIMARY_TICK_MS};
use vr::vr_api_messages::{VrApiRsp, VrApiError};
use NamespaceMsg;
use super::utils::PrepareRequests;
use super::{StateTransfer, Recovery, Reconfiguration, Backup, Leaving};

/// The primary state of the VR protocol operating in normal mode
state!(Primary {
    ctx: VrCtx,
    prepare_requests: PrepareRequests,
    // If the primary doesn't receive a new client request in `primary_tick_ms` it sends a commit
    // message to the backups. `idle_timeout` should be at least twice as large as this value.
    tick_ms: u64,
    reconfiguration_in_progress: bool
});

impl Transition for Primary {
    fn handle(self,
              msg: VrMsg,
              from: Pid,
              cid: CorrelationId,
              output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        match msg {
            VrMsg::ClientRequest(msg) => self.handle_client_request(msg, cid, output),
            VrMsg::Reconfiguration(msg) => self.handle_reconfig(msg, from, cid, output),
            VrMsg::PrepareOk(msg) => self.handle_prepare_ok(msg, from, cid, output),
            VrMsg::Tick => self.handle_tick(output),
            VrMsg::GetState(msg) => self.handle_get_state(msg, from, cid, output),
            VrMsg::Recovery(msg) => self.handle_recovery(msg, from, cid, output),
            VrMsg::StartEpoch(_) => self.handle_start_epoch(from, cid, output),
            VrMsg::Prepare(msg) => {
                up_to_date!(self, from, msg, cid, output);
                // The primary should never receive these messages in the same epoch/view
                unreachable!()
            }
            VrMsg::Commit(msg) => {
                up_to_date!(self, from, msg, cid, output);
                // The primary should never receive these messages in the same epoch/view
                unreachable!()
            }
            VrMsg::StartView(msg) => {
                up_to_date!(self, from, msg, cid, output);
                // The primary should never receive these messages in the same epoch/view
                unreachable!()
            }
            VrMsg::StartViewChange(msg) => {
                up_to_date!(self, from, msg, cid, output);
                // Ignore old messages for same epoch/view as this replica is already Primary
                self.into()
            }
            VrMsg::DoViewChange(msg) => {
                up_to_date!(self, from, msg, cid, output);
                // Ignore old messages for same epoch/view as this replica is already Primary
                self.into()
            },
            _ => self.into()
        }
    }
}

impl Primary {
    pub fn new(ctx: VrCtx) -> Primary {
        let size = ctx.new_config.replicas.len();
        Primary {
            ctx: ctx,
            prepare_requests: PrepareRequests::new(size,
                                                   DEFAULT_IDLE_TIMEOUT_MS),
            tick_ms: DEFAULT_PRIMARY_TICK_MS,
            reconfiguration_in_progress: false
        }
    }

    /// Enter Primary state
    pub fn enter(ctx: VrCtx) -> VrState {
        Primary::new(ctx).into()
    }

    fn handle_client_request(mut self,
                             msg: ClientRequest,
                             cid: CorrelationId,
                             output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        self.send_prepare(msg.into(), cid, output);
        self.into()
    }

    fn handle_reconfig(mut self,
                       msg: vr_msg::Reconfiguration,
                       from: Pid,
                       cid: CorrelationId,
                       output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        if msg.epoch != self.ctx.epoch {
            // TODO: Inform client about the change in epoch
            // Redirect to the primary with the correct epoch
            // This code will cause a Timeout
            return self.into();
        }
        if let Some(err_envelope) = self.validate_reconfig(&from, &msg, &cid) {
            output.push(err_envelope);
            return self.into();
        }
        self.reconfiguration_in_progress = true;
        self.send_prepare(msg.into(), cid, output);
        self.into()
    }

    fn handle_prepare_ok(mut self,
                         msg: PrepareOk,
                         from: Pid,
                         cid: CorrelationId,
                         output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        up_to_date!(self, from, msg, cid, output);
        let PrepareOk {op, ..} = msg;
        if op <= self.ctx.commit_num {
            return self.into();
        }
        if self.has_commit_quorum(op, from) {
            debug!(self.ctx.logger, "primary: has commit quorum: op = {}", op);
            return self.commit(op, output);
        }
        self.into()
    }

    fn handle_tick(mut self, output: &mut Vec<Envelope<Msg>>) -> VrState {
        if self.idle_timeout() {
            self.ctx.last_received_time = SteadyTime::now();
            self.broadcast_commit_msg(output);
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
        StateTransfer::send_new_state(&self.ctx, msg.op, from, cid, output);
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


    fn send_prepare(&mut self,
                    msg: ClientOp,
                    cid: CorrelationId,
                    output: &mut Vec<Envelope<Msg>>)
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

    fn has_commit_quorum(&mut self, op: u64, from: Pid) -> bool {
        self.prepare_requests.insert(op, from);
        self.prepare_requests.has_quorum(op)
    }

    pub fn commit(mut self, new_commit_num: u64, output: &mut Vec<Envelope<Msg>>) -> VrState {
        let lowest_prepared_op = self.prepare_requests.lowest_op;
        let mut iter = self.prepare_requests.remove(new_commit_num).into_iter();
        let commit_num = self.ctx.commit_num;
        for i in commit_num..new_commit_num {
            let msg = self.ctx.log[i as usize].clone();
            self.ctx.commit_num = i + 1;
            match msg {
                ClientOp::Request(ClientRequest {op, request_num, ..}) => {
                    let rsp = self.ctx.backend.call(op);
                    if i >= lowest_prepared_op - 1 {
                        // This primary prepared this request, and wasn't elected after it was
                        // prepared but before it was committed.
                        let cid = iter.next().unwrap().correlation_id;
                        let from = cid.pid.clone();
                        output.push(self.client_reply(rsp, request_num, &from, &cid));
                    }
                },
                ClientOp::Reconfiguration(vr_msg::Reconfiguration {client_req_num,
                                                                   epoch,
                                                                   replicas, ..}) =>
                {
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
                        output.push(Envelope::new(to, from, reply, Some(cid)));
                    }

                    self.ctx.epoch = epoch;
                    self.ctx.update_for_new_epoch(i+1, replicas);
                    self.ctx.announce_reconfiguration(output);
                    self.set_primary(output);

                    // If the Reconfiguration is the last entry in the log, then this is either the
                    // primary that received the client request, or a view change occurred and this
                    // is a new primary in the epoch of the reconfiguration request processing the
                    // reconfiguration
                    //
                    // If the reconfiguration is not the last in the log, we don't want to
                    // transition, as the reconfiguration has already happened.
                    if new_commit_num == self.ctx.log.len() as u64 {
                        self.broadcast_commit_msg_old(output);
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
    fn enter_transitioning(mut self, output: &mut Vec<Envelope<Msg>>) -> VrState {
        if self.ctx.is_leaving() {
            return Leaving::leave(self.ctx);
        }
        // Tell replicas that are being replaced to shutdown
        Reconfiguration::broadcast_epoch_started(&self.ctx, output);
        if self.ctx.is_primary() {
            self.reconfiguration_in_progress = false;
            // Stay a primary
            return self.into();
        }
        // Become a backup
        Backup::enter(self.ctx)
    }

    /// Return Some(error msg) if the reconfiguration request is invalid. Return None on success.
    fn validate_reconfig(&self,
                         from: &Pid,
                         msg: &vr_msg::Reconfiguration,
                         cid: &CorrelationId) -> Option<Envelope<Msg>>
    {
        let &vr_msg::Reconfiguration {client_req_num, epoch, ref replicas} = msg;
        let err = if self.reconfiguration_in_progress {
            Some(VrApiError::Msg("Reconfiguration in progress".to_owned()))
        } else if replicas.len() < 3 {
            Some(VrApiError::NotEnoughReplicas)
        } else if epoch != self.ctx.epoch {
            Some(VrApiError::BadEpoch)
        } else if *replicas == self.ctx.new_config.replicas {
            Some(VrApiError::Msg("No change to existing configuration".to_owned()))
        } else {
            None
        };
        err.map(|e| self.client_reply(VrApiRsp::Error(e), client_req_num, from, cid))
    }

    fn client_reply(&self,
                    rsp: VrApiRsp,
                    client_req_num: u64,
                    to: &Pid,
                    cid: &CorrelationId) -> Envelope<Msg>
    {
        let reply = self.client_reply_msg(client_req_num, rsp);
        Envelope::new(to.clone(), self.ctx.pid.clone(), reply, Some(cid.clone()))
    }

    fn client_reply_msg(&self, client_req_num: u64, value: VrApiRsp) -> rabble::Msg<Msg> {
        ClientReply {
            epoch: self.ctx.epoch,
            view: self.ctx.view,
            request_num: client_req_num,
            value: value
        }.into()
    }

    fn commit_msg(&self) -> rabble::Msg<Msg> {
        Commit {
            epoch: self.ctx.epoch,
            view: self.ctx.view,
            commit_num: self.ctx.commit_num
        }.into()
    }

    fn broadcast_commit_msg(&self, output: &mut Vec<Envelope<Msg>>) {
        self.ctx.broadcast(self.commit_msg(),
                           CorrelationId::pid(self.ctx.pid.clone()),
                           output);
    }

    fn broadcast_commit_msg_old(&self, output: &mut Vec<Envelope<Msg>>) {
        self.ctx.broadcast_old(self.commit_msg(),
                               CorrelationId::pid(self.ctx.pid.clone()),
                               output)
    }

    pub fn set_primary(&mut self, output: &mut Vec<Envelope<Msg>>) {
        let primary = self.ctx.compute_primary();
        info!(self.ctx.logger, "Elected primary";
              "primary" => format!("{:?}", primary),
              "view" => self.ctx.view);
        let msg = NamespaceMsg::NewPrimary(primary);
        output.push(self.ctx.namespace_mgr_envelope(msg));
    }

    fn idle_timeout(&self) -> bool {
        SteadyTime::now() - self.ctx.last_received_time >
            Duration::milliseconds(self.tick_ms as i64)
    }
}
