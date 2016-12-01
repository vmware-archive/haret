use time::{SteadyTime, Duration};
use rabble::{Pid, CorrelationId};
use super::vr_envelope::VrEnvelope;

/// The internal state of the VR FSM. Note that fields are only made public for visibility,
/// debugging and testing purposes.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VrCtx {
    pub me: Pid,
    pub primary: Option<Pid>,
    pub epoch: u64,
    pub view: u64,
    pub op: u64,
    pub commit_num: u64,
    pub last_received_time: SteadyTime,
    pub last_normal_view: u64,

    /// The number of replicas needed to provide quorum
    pub quorum: u64,

    /// Used by the replica in primary state tracking quorums of Prepare requests
    pub prepare_requests: PrepareRequests,

    pub log: Vec<VrMsg>,
    pub backend: VrBackend,
    pub old_config: VersionedReplicas,
    pub new_config: VersionedReplicas,

    /// Only used during recovery
    pub recovery_state: Option<RecoveryState>,

    /// Only used during view change
    pub view_change_state: Option<ViewChangeState>,

    /// Backups wait `idle_timeout` between messages from the primary before initiating a view
    /// change.
    pub idle_timeout: Duration,

    /// If the primary doesn't receive a new client request in `primary_tick_ms` it sends a commit
    /// message to the backups. `idle_timeout_ms` should be at least twice as large as this value.
    pub primary_tick_ms: u64,
}

impl VrCtx {
    pub fn new(me: Pid, old_config: VersionedReplicas, new_config: VersionedReplicas) -> VrCtx {
        let quorum = new_config.replicas.len() / 2 + 1;
        VrCtx {
            me: me,
            primary: None,
            epoch: new_config.epoch,
            view: 0,
            op: 0,
            commit_num: 0,
            last_received_time: SteadyTime::now(),
            last_normal_view: 0,
            quorum: quorum as u64,
            prepare_requests: PrepareRequests::new(new_config.replicas.len(),
                                                   DEFAULT_IDLE_TIMEOUT_MS),
            log: Vec::new(),
            backend: VrBackend::new(),
            old_config: old_config,
            new_config: new_config,
            recovery_state: None,
            view_change_state: None,
            idle_timeout_ms: DEFAULT_IDLE_TIMEOUT_MS,
            primary_tick_ms: DEFAULT_PRIMARY_TICK_MS
        }
    }

    pub fn is_primary(&self) -> bool {
        self.primary.as_ref().map_or(false, |p| *p == self.me)
    }

    fn new_recovery_state(&self) -> RecoveryState {
        RecoveryState::new(self.quorum, self.idle_timeout)
    }

    pub fn start_recovery(&mut self) -> Vec<VrEnvelope> {
        self.recovery_state = Some(self.new_recovery_state());
        self.broadcast(self.recovery_msg())
    }

    pub fn update_recovery_state(&mut self, msg: VrMsg) {
        if let VrMsg::RecoveryResponse {epoch, view, nonce, from, op, commit_num, log} = msg {
            if nonce != self.recovery_state.nonce { return; }
            if epoch < self.epoch { return; }

            // TODO: If we get a response from a replica in a later epoch, we are in a weird state
            // We missed a reconfiguration and the namespace manager hasn't learned of the epoch
            // change yet. What we really want is to wait for the namespace manager to learn of the
            // replicas in the later epoch and restart the replica. For now we're ignoring that this
            // situation can even occur. We just return without processing the message.. This is
            // clearly wrong.
            if epoch > self.epoch {
                println!("EPOCH RECONFIGURATION DURING RECOVERY: Replica {} in a bad state",
                         self.me);
                return;
            }

            if view > self.view {
                self.view = view;
            }

            let response_from_primary = op.is_some();
            if response_from_primary && view == self.view {
                self.recovery_state.as_mut().primary = RecoveryPrimary {
                    pid: Pid,
                    view: view,
                    op: op,
                    commit_num: commit_num,
                    log: log
                };
            }
            self.recovery_state.responses.insert(from, ())
        }
        unreachable!()
    }

    pub fn commit_recovery(&mut self) -> Some<FsmOutput> {
        if self.recovery_state.has_quorum(self.view) {
            let output = self.set_primary();
            let primary_state = self.recovery_state.primary.as_ref().unwrap();
            self.op = primary_state.op;
            self.log = primary_state.log;
            output.extend(&self.backup_commit(primary_state.commit_num));
            self.recovery_state = Some(new_recovery_state())
            Some(output)
        }
        None
    }

    pub fn view_change_expired(&self) -> bool {
        self.view_change_state.responses.is_expired()
    }

    pub fn recovery_expired(&self) -> bool {
        self.recovery_state.responses.is_expired()
    }

    pub fn clear_quorum_tracker(&mut self) {
        self.quorum_tracker = QuorumTracker::new(self.quorum as usize);
    }

    pub fn compute_primary(&self) -> Pid {
        let index = self.view as usize % self.new_config.replicas.len();
        self.new_config.replicas[index].clone()
    }

    pub fn start_state_transfer_new_view(&mut self,
                                         new_view: u64,
                                         c_id: CorrelationId) -> Vec<FsmOutput>
    {
        ctx.last_received_time = SteadyTime::now();
        ctx.view = new_view;
        self.op = self.commit_num;
        self.log.truncate(self.op as usize);
        vec![ctx.send_get_state_to_random_replica(c_id)]
    }

    pub fn start_state_transfer_reconfiguration(&mut self) -> Vec<FsmOutput> {
        self.last_received_time = SteadyTime::now();
        self.view = 0;
        self.op = self.commit_num;
        self.log.truncate(self.op as usize);
        broadcast_old_and_new(self, self.get_state_msg())
    }

    /// For a valid VrMsg::ClientRequest | VrMsg::Reconfiguration, broadcast a prepare msg
    pub fn send_prepare(&mut self, envelope: VrEnvelope) -> Vec<FsmOutput> {
        self.last_received_time = SteadyTime::now();
        self.op += 1;
        let (api_op, request_num) = get_prepare_client_data(envelope.msg.clone());
        let prepare = self.prepare_msg(request_num, api_op);
        self.log.push(envelope.msg);
        self.prepare_requests.new_prepare(self.op, envelope.correlation_id.clone());
        self.broadcast(prepare, envelope.correlation_id)
    }

    pub fn send_prepare_ok(&mut self,
                           client_op: VrApiReq,
                           commit_num: u64,
                           request_num: u64,
                           correlation_id: CorrelationId) -> Vec<FsmOutput> {
        self.last_received_time = SteadyTime::now();
        self.op += 1;
        // Reconstruct Client Request, since the log stores VrMsgs
        let client_req = VrMsg::ClientRequest {op: client_op, request_num: client_request_num};
        self.log.push(client_req);
        let mut output = self.backup_commit(commit_num);
        output.push(self.send_to_primary(self.prepare_ok_msg(), correlation_id));
        output
    }

    pub fn send_do_view_change(&self) -> FsmOutput {
        let c_id = CorrelationId::Pid(self.pid.clone());
        FsmOutput::Vr(VrEnvelope::new(from, self.me.clone(), self.do_view_change_msg(op), c_id))
    }

    pub fn send_new_state(&mut self, op: u64, from: Pid, c_id: CorrelationId) -> FsmOutput {
        FsmOutput::Vr(VrEnvelope::new(from, self.me.clone(), self.new_state_msg(op), c_id))
    }

    pub fn send_recovery_response(&self,
                                  from: Pid,
                                  nonce: Uuid,
                                  c_id: CorrelationId) -> FsmOutput
    {
        let response = self.recovery_response_msg(nonce);
        FsmOutput::Vr(VrEnvelope::new(from, self.me.clone(), response, c_id));
    }

    pub fn send_get_state_to_random_replica(&self, c_id: CorrelationId) -> FsmOutput {
        self.send_to_random_replica(self.get_state_msg(), &self.new_config)
    }

    fn send_to_random_replica(&self, msg: VrMsg, c_id: CorrelationId) -> FsmOutpu {
        let mut rng = thread_rng();
        let mut to = self.me.clone();
        while to == self.me {
            let index = rng.gen_range(0, self.new_config.replicas.len());
            to = self.new_config.replicas[index].clone()
        }
        FsmOutput::Vr(VrEnvelope::new(to, self.me.clone(), msg, c_id))
    }

    pub fn broadcast_start_epoch(&self) -> Vec<FsmOutput> {
        self.broadcast(self.start_epoch_msg(), CorrelationId::Pid(self.pid))
    }

    fn broadcast_start_view_msg(&self) -> Vec<FsmOutput> {
        self.broadcast(self.start_view_msg(), CorrelationId::Pid(self.pid))
    }

    pub fn broadcast_commit_msg(&self) -> Vec<FsmOutput> {
        self.broadcast(self.commit_msg(), CorrelationId::Pid(self.pid))
    }

    pub fn rebroadcast_reconfig(&self) -> Vec<FsmOutput> {
        let reconfig = self.log[(self.op - 1) as usize].clone();
        if let VrMsg::Reconfiguration {client_req_num, ..} = reconfig {
            let prepare = self.prepare_msg(client_req_num, VrApiReq::Null);
            return broadcast(self, prepare);
        }
        unreachable!();
    }

    // During reconfiguration if we are not up to date we need to send a get state request to all
    // replicas to ensure we get the latest results.
    pub fn broadcast_old_and_new(&self, msg: VrMsg, c_id: CorrelationId) -> Vec<FsmOutput> {
        let mut output = Vec::new();
        self.old_config.replicas.iter().cloned().chain(self.new_config.replicas.iter().cloned())
            .filter(|pid| pid != self.me)
            .map(|pid| vr_new(pid, self.me.clone(), msg.clone(), correlation_id.clone()))
            .collect()
    }

    /// Wrap a VrMsg in an envelope and send to all replicas
    pub fn broadcast(&self, msg: VrMsg, correlation_id: CorrelationId) -> Vec<FsmOutput> {
        self.new_config.replicas.iter().cloned()
            .filter(|pid| pid != self.me)
            .map(|pid| vr_new(pid, self.me.clone(), msg.clone(), correlation_id.clone()))
            .collect()
    }

    pub fn has_view_change_quorum(&self) -> bool {
        self.view_change_state.map_or(false, |s| s.has_quorum())
    }

    fn become_primary(&mut self) -> Vec<FsmOutput> {
        let last_commit_num = self.commit_num;
        self.set_state_to_become_primary();
        let mut output = self.broadcast_start_view_msg();
        println!("Elected {:?} as primary of view {}", self.primary, self.view);
        output.extend(&self.new_primary_commit(self, last_commit_num));
        output
    }

    pub fn new_primary_commit(&mut self, last_commit_num: u64) -> Vec<FsmOutput> {
        let mut output = Vec::new();
        for i in last_commit_num..self.commit_num {
            let msg = self.log[i as usize].clone();
            match msg {
                VrMsg::ClientRequest {ref op, request_num} {
                    // The client likely hasn't reconnected, don't bother sending a reply here
                    self.backend.call(i+1, op.clone());
                },
                VrMsg::Reconfiguration {client_req_num, epoch, replicas ..} {
                    let rsp = self.backend.call(i+1, VrApiReq::Null);
                    self.update_for_new_epoch(i+1, replicas);
                    output.push(self.announce_reconfiguration());
                    output.push(self.set_primary());
                    output.extend_from_slice(&self.broadcast_commit_msg_old());
                    // TODO: If we tracked VrEnvelope in the log instead of VrMsg, we would have a
                    // proper correlation_id here.
                    output.extend_from_slice(&self.send_epoch_started(CorrelationId::Pid(self.me.clone()));
                }
            }
            output
        }
    }

    fn set_state_to_become_primary(&mut self) {
        let current = Latest {
            last_normal_view: self.last_normal_view,
            commit_num: self.commit_num,
            op: self.op,
            // TODO: FIXME: Cloning the log is expensive
            log: self.log.clone()
        };
        let latest = self.view_change_state.compute_latest_state(current);
        self.commit_num = latest.commit_num;
        self.op = latest.op;
        self.log = latest.log;
        self.last_normal_view = self.view;
        self.view_change_state = None;
    }


    pub fn become_backup(ctx: &mut VrCtx,
                         view: u64,
                         op: u64,
                         log: Vec<VrMsg>,
                         commit_num: u64) -> Vec<FsmOutput>
    {
        ctx.view = view;
        ctx.op = op;
        ctx.log = log;
        let mut output = vec![ctx.set_primary()];
        output.extend(&ctx.backup_commit(commit_num));
        ctx.last_normal_view = ctx.view;
        output
    }

    pub fn backup_commit(&mut self, new_commit_num: u64) -> Vec<FsmOutput> {
        let mut output = Vec::new();
        for i in self.commit_num..new_commit_num {
            let msg = self.log[i as usize].clone();
            match msg {
                VrMsg::ClientRequest {op, ..} => {
                    self.backend.call(i+1, op);
                },
                VrMsg::Reconfiguration {replicas, ..} => {
                    self.backend.call(i+1, VrApiReq::Null);
                    self.update_for_new_epoch(i+1, replicas);
                    output.push(self.announce_reconfiguration(self));
                    output.push(self.set_primary());
                },
                _ => ()
            }
        }
        self.commit_num = new_commit_num;
        output
    }


    pub fn announce_reconfiguration(&self) -> FsmOutput {
        FsmOutput::Announcement(NamespaceMsg::Reconfiguration {
            namespace_id: self.me.namespace_id.clone(),
            old_config: self.old_config.clone(),
            new_config: self.new_config.clone()
        }, self.me.clone())
    }

    fn send_epoch_started(&mut self, c_id: CorrelationId) -> Vec<FsmOutput>{
        let msg = self.epoch_started_msg();
        self.replicas_to_replace().iter().map(|r| {
            FsmOutput::Vr(VrEnvelope::new(r, self.me.clone(), msg.clone(), c_id.clone()))
        }).collect()
    }

    pub fn replicas_to_replace(&self) -> Vec<Pid> {
        let new_set = HashSet::<Pid>::from_iter(self.new_config.replicas.clone());
        let old_set = HashSet::<Pid>::from_iter(self.old_config.replicas.clone());
        old_set.difference(&new_set).cloned().collect()
    }

    pub fn send_to_primary(&self, msg: VrMsg, correlation_id: CorrelationId) -> FsmOutput {
        FsmOutput::Vr(
            VrEnvelope::new(self.primary.as_ref().unwrap.clone(), self.me.clone(), msg, c_id)
        )
    }

    /// Return Some(error msg) if the reconfiguration request is invalid. Return None on success.
    pub fn validate_reconfig(&self, envelope: &VrEnvelope) -> Option<VrEnvelope> {
        if let VrMsg::Reconfiguration {client_req_num, epoch, ref replicas} = envelope.msg {
            if replicas.len() < 3 {
                let errmsg = "New config must contain at least 3 replicas".to_string();
                let reply = self.client_reply_msg(client_req_num, VrApiRsp::Error {msg: errmsg});
                return Some(VrEnvelope::new(envelope.from,
                                            self.pid.clone(),
                                            reply,
                                            envelope.correlation_id.clone()));
            }
            if epoch < ctx.epoch || epoch > ctx.epoch {
                let errmsg = "Reconfiguration attempted with incorrect epoch".to_string();
                let reply = self.client_reply_msg(client_req_num, VrApiRsp::Error {msg: errmsg});
                return Some(VrEnvelope::new(envelope.from,
                                            self.pid.clone(),
                                            reply,
                                            envelope.correlation_id));
            }
            return None;
        }
        unreachable!()
    }

    pub fn has_commit_quorum(&mut self, op: u64, from: Pid) -> bool {
        self.prepare_requests.insert(op, from);
        self.prepare_requests.has_quorum(op)
    }

    pub fn primary_commit(&mut self, op: u64) -> Vec<Envelope> {
        let mut output = Vec::new();
        let iter = self.prepare_requests.remove(self.commit_num);
        for i in self.commit_num..op {
            let request = iter.next().unwrap();
            let msg = self.log[i as usize].clone();
            match msg {
                VrMsg::ClientRequest {ref op, request_num} {
                    let rsp = self.backend.call(i+1, op.clone());
                    let reply = self.client_reply_msg(request_num, rsp);
                    output.push(VrEnvelope::new(request.correlation_id.pid,
                                                self.pid.clone(),
                                                reply,
                                                request.correlation_id));
                },
                VrMsg::Reconfiguration {client_req_num, epoch, replicas ..} {
                    let rsp = self.backend.call(i+1, VrApiReq::Null);
                    let reply = VrMsg::ClientReply {
                        epoch: epoch,
                        view: 0,
                        request_num: client_req_num,
                        value: rsp
                    };
                    output.push(VrEnvelope::new(request.correlation_id.pid,
                                                self.pid.clone(),
                                                reply,
                                                request.correlation_id.clone()));
                    self.update_for_new_epoch(i+1, replicas);
                    output.push(self.announce_reconfiguration(self));
                    output.push(self.set_primary());
                    output.extend_from_slice(&self.broadcast_commit_msg_old());
                    output.extend_from_slice(&self.send_epoch_started(request.correlation_id));
                }
            }
        }
        self.commit_num = op;
        output
    }

    fn update_for_new_epoch(&mut self, op: u64, replicas: Vec<Pid>) {
        self.last_received_time = SteadyTime::now();
        self.epoch += 1;
        self.view = 0;
        self.last_normal_view = 0;
        mem::swap(&self.old_config, &self.new_config);
        self.new_config = VersionedReplicas {epoch: self.epoch, op: op, replicas: replicas};
    }


    #[inline]
    /// We use a cast to i64 until the stdlib Duration that takes u64 is stabilized; It doesn't matter
    /// here since the values are so small.
    fn idle_timeout(&self) -> bool {
        SteadyTime::now() - self.last_received_time > Duration::milliseconds(self.idle_timeout_ms as i64)
    }

    #[inline]
    /// We use a cast to i64 until the stdlib Duration that takes u64 is stabilized; It doesn't matter
    /// here since the values are so small.
    fn primary_idle_timeout(&self) -> bool {
        SteadyTime::now() - self.last_received_time > Duration::milliseconds(self.primary_tick_ms as i64)
    }

    pub fn last_log_entry_is_latest_reconfiguration(&self, epoch: u64, op: u64) -> bool {
        if let VrMsg::Reconfiguration {epoch: log_epoch, ..} = self.log[(op-1) as usize] {
            if log_epoch + 1 == epoch {
                return true;
            }
        }
        false
    }

    pub fn is_leaving(&self) -> bool {
        self.replicas_to_replace().contains(&self.me)
    }

    pub fn set_state_new_epoch(&mut self,
                               old_config: VersionedReplicas,
                               new_config: VersionedReplicas) {

        self.last_received_time = SteadyTime::now();
        self.view = 0;
        self.old_config = old_config;
        self.new_config = new_config;
    }

    pub fn set_from_new_state_msg(&mut self, msg: VrMsg) -> Vec<FsmOutput> {
        if let VrMsg::NewState {view, op, commit_num, log_tail, ..} = msg {
            self.view = view;
            self.op = op;
            clear_quorum_tracker(self);
            for m in log_tail {
                self.log.push(m);
            }
            let mut output = self.backup_commit(commit_num);
            ouptut.push(self.set_primary());
            output
        }
        unreachable!();
    }


    fn get_prepare_client_data(msg: VrMsg) -> (VrApiReq, u64) {
        match msg {
            VrMsg::ClientRequest {op, request_num} => (op, request_num)
            VrMsg::Reconfiguration {client_req_num} => (VrApiReq::Null, client_req_num)
            _ => unreachable!()
        }
    }

    fn clear_primary(&mut self) -> FsmOutput {
        self.primary = None;
        FsmOutput::Announcement(NamespaceMsg::ClearPrimary(self.me.namespace_id), self.me.clone())
    }

    fn set_primary(&mut self) -> FsmOutput {
        let primary = self.compute_primary();
        self.primary = Some(primary.clone());
        FsmOutput::Announcement(NamespaceMsg::NewPrimary(primary), self.me.clone())
    }

    fn insert_view_change_message(&mut self, from: Pid, msg: VrMsg) {
        self.view_change_state.as_ref().unwrap().responses.insert(from, msg);
    }

    fn reset_view_change_state(&mut self, view: u64) -> Vec<FsmOutput> {
        self.last_received_time = SteadyTime::now();
        self.view = view;
        self.view_change_state = Some(ViewChangeState::new(self.quorum, self.idle_timeout));
        vec![self.clear_primary()]
    }

    fn reset_and_start_view_change(&mut self) -> Vec<FsmOutput> {
        let mut output = self.reset_view_change_state(self.view + 1);
        output.extend(self.start_view_change());
        output
    }

    fn start_view_change(self: &mut VrCtx) -> Vec<FsmOutput> {
        self.broadcast(self.start_view_change_msg())
    }

    /*************************************************************************/
    /*******   CONSTRUCT VR MESSAGES   ****/
    /************************************************************************/

    pub fn prepare_msg(&self, client_req_num: u64, client_op: VrApiReq) -> VrMsg {
        VrMsg::Prepare {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            client_request_num: client_req_num,
            client_op: client_op,
            commit_num: self.commit_num
        }
    }

    pub fn prepare_ok_msg(&self) -> VrMsg {
        VrMsg::PrepareOk {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            from: self.me.clone()
        };
    }

    pub fn new_state_msg(&self, op: u64) -> VrMsg {
        VrMsg::NewState {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            primary: self.primary.clone(),
            commit_num: self.commit_num,
            log_tail: (&self.log[op as usize..self.op as usize]).clone()
        }
    }

    pub fn recovery_msg(&self) -> VrMsg {
        VrMsg::Recovery {
            from: self.me.clone(),
            nonce: self.recovery_nonce.clone().unwrap()
        }
    }

    pub fn recovery_response_msg(&self, nonce: Uuid) -> VrMsg {
        let (op, commit_num, log) =
            if self.primary.is_some() && self.primary.as_ref().unwrap() == self.me {
                (Some(self.op), Some(self.commit_num), Some(self.log.clone())
            } else {
                (None, None, None)
            };
        VrMsg::RecoveryResponse {
            epoch: self.epoch,
            view: self.view,
            nonce: nonce,
            from: self.me.clone(),
            op: op,
            commit_num: commit_num,
            log: log
        }
    }

    pub fn get_state_msg(&self) -> VrMsg {
        VrMsg::GetState {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            from: self.me.clone()
        }
    }

    pub fn commit_msg(&self) -> VrMsg {
        VrMsg::Commit {
            epoch: self.epoch,
            view: self.view,
            commit_num: self.commit_num
        }
    }

    pub fn start_epoch_msg(&self) -> VrMsg {
        VrMsg::StartEpoch {
            epoch: self.epoch,
            op: self.op,
            old_config: self.old_config.clone(),
            new_config: self.new_config.clone()
        }
    }

    pub fn start_view_msg(&self) -> VrMsg {
        VrMsg::StartView {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            log: self.log.clone(),
            commit_num: self.commit_num
        }
    }

    pub fn start_view_change_msg(&self) -> VrMsg {
        VrMsg::StartViewChange {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            from: self.me.clone()
        }
    }

    pub fn do_view_change_msg(&self) -> VrMsg {
        let do_view_change = VrMsg::DoViewChange {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            from: self.me.clone(),
            last_normal_view: self.last_normal_view,
            log: self.log.clone(),
            commit_num: self.commit_num
        }
    }

    pub fn epoch_started_msg(&self) -> VrMsg {
        VrMsg::EpochStarted {
            epoch: self.epoch,
            from: self.me.clone()
        }
    }

    pub fn client_reply_msg(&self, client_req_num: u64, value: VrApiRsp) -> VrMsg {
        VrMsg::ClientReply {
            epoch: self.epoch,
            view: self.view,
            request_num: client_req_num,
            value: value
        }
    }

    /*************************************************************************/
    /* End of VrMsg constructors */
    /*************************************************************************/
}

/// Create a new `FsmOut::Vr(..)` variant
fn vr_new(to: Pid, from: Pid, msg: VrMsg, c_id: CorrelationId) -> FsmOut {
    FsmOut::Vr(VrEnvelope::new(pid, self.me.clone(), msg.clone(), correlation_id))
}
