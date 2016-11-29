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
            self.recovery_state.responses.insert(from)
        }
        unreachable!()
    }

    pub fn commit_recovery(&mut self) -> Some<FsmOutput> {
        if self.recovery_state.has_quorum(self.view) {
            let output = self.set_primary();
            let primary_state = self.recovery_state.primary.as_ref().unwrap();
            self.op = primary_state.op;
            self.log = primary_state.log;
            self.backup_commit(primary_state.commit_num);
            self.recovery_state = Some(new_recovery_state())
            Some(output)
        }
        None
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

    pub fn start_state_transfer_reconfiguration(&mut self) -> Vec<FsmOutput> {
        self.view = 0;
        self.last_received_time = SteadyTime::now();
        self.op = self.commit_num;
        self.log.truncate(self.op as usize);
        broadcast_old_and_new(self, self.get_state_msg())
    }

    /// For a valid VrMsg::ClientRequest | VrMsg::Reconfiguration, broadcast a prepare msg
    pub fn send_prepare(&mut self, envelope: VrEnvelope) -> Vec<VrEnvelope> {
        let (op, request_num) = get_prepare_client_data(envelope.msg.clone());
        self.last_received_time = SteadyTime::now();
        self.op += 1;
        let prepare = self.prepare_msg(request_num, op);
        self.log.push(envelope.msg);
        self.prepare_requests.new_prepare(self.op);
        broadcast(self, prepare, envelope.correlation_id)
    }

    pub fn send_prepare_ok(&mut self, client_op: VrApiReq, commit_num: u64, request_num: u64)
        -> VrEnvelope
    {
        self.last_received_time = SteadyTime::now();
        self.op += 1;
        // Reconstruct Client Request, since the log stores VrMsgs
        let client_req = VrMsg::ClientRequest {op: client_op, request_num: client_request_num};
        self.log.push(client_req);
        let output = self.send_to_primary(self.prepare_ok_msg());
        self.commit(commit_num);
        return output;
    }

    pub fn send_new_state(&mut self, op: u64, from: Pid) -> VrEnvelope {
        let new_state = self.new_state_msg(op);
        vec![Envelope::Peer(PeerEnvelope::new(from, self.me.clone(), new_state))];
    }

    pub fn send_primary_recovery_response(&self, msg: VrMsg) -> Vec<Envelope> {
        if let VrMsg::Recovery {from, nonce} = msg {
            let response = self.recovery_response_msg(nonce);
            return vec![Envelope::Peer(PeerEnvelope::new(from, self.me.clone(), response))];
        }
        unreachable!()
    }

    pub fn send_backup_recovery_response(&self, from: Pid, nonce: Uuid) -> VrEnvelope {
        let response = self.recovery_response_msg(nonce);
        let output = vec![Envelope::Peer(PeerEnvelope::new(from, ctx.me.clone(), response))];
    }

    pub fn send_get_state_to_random_replica(&self) -> VrEnvelope {
        self.send_to_random_replica(self.get_state_msg(), &self.new_config)
    }

    fn send_to_random_replica(&self, msg: VrMsg, config: &VersionedReplicas) -> VrEnvelope {
        let mut rng = thread_rng();
        let mut to = self.me.clone();
        while to == self.me {
            let index = rng.gen_range(0, config.replicas.len());
            to = config.replicas[index].clone()
        }
        vec![Envelope::Peer(PeerEnvelope::new(to, self.me.clone(), msg))]
    }

    pub fn broadcast_start_epoch(&self) -> Vec<VrEnvelope> {
        self.broadcast(self.start_epoch_msg())
    }

    fn broadcast_start_view_msg(&self) -> Vec<VrEnvelope> {
        self.broadcast(self.start_view_msg())
    }

    fn broadcast_commit_msg(&self) -> Vec<Envelope> {
        self.broadcast(self.commit_msg())
    }

    fn broadcast_commit_msg_old(&self) -> Vec<Envelope> {
        self.broadcast_old(self.commit_msg())
    }

    pub fn rebroadcast_reconfig(&self) -> Vec<Envelope> {
        let reconfig = self.log[(self.op - 1) as usize].clone();
        if let VrMsg::Reconfiguration {client_req_num, ..} = reconfig {
            let prepare = self.prepare_msg(client_req_num, VrApiReq::Null);
            return broadcast(self, prepare);
        }
        unreachable!();
    }

    // During reconfiguration if we are not up to date we need to send a get state request to all
    // replicas to ensure we get the latest results.
    pub fn broadcast_old_and_new(&self, msg: VrMsg) -> Vec<FsmOutput> {
        let mut output = Vec::new();
        for r in self.old_config.replicas.iter().cloned() {
            if self.me != r {
                output.push(Envelope::Peer(PeerEnvelope::new(r, self.me.clone(), msg.clone())));
            }
        }
        for r in self.new_config.replicas.iter().cloned() {
            if self.me != r {
                output.push(Envelope::Peer(PeerEnvelope::new(r, self.me.clone(), msg.clone())));
            }
        }
        output
    }

    // TODO: This code before the rabble refactor had self.new_config. Is that correct?
    pub fn broadcast_old(&self, msg: VrMsg) -> Vec<Envelope> {
        let mut output = Vec::new();
        for replica in self.new_config.replicas.iter().cloned() {
            if self.me != replica {
                output.push(Envelope::Peer(PeerEnvelope::new(replica, self.me.clone(), msg.clone())));
            }
        }
        output
    }

    /// Wrap a VrMsg in an envelope and send to all replicas
    pub fn broadcast(&self, msg: VrMsg, correlation_id: CorrelationId) -> Vec<VrEnvelope> {
        self.new_config.replicas.iter().cloned().filter(|pid| pid != self.me).map(|pid| {
            VrEnvelope::new(pid, self.me.clone(), msg.clone(), correlation_id)
        }).collect()
    }

/// Returns true if the op was committed
fn maybe_become_primary(&mut self, from: Pid, msg: VrMsg) -> Transition {
    self.quorum_tracker.insert(from, msg);
    if self.quorum_tracker.has_quorum() {
        let mut output = Vec::new();
        let last_commit_num = self.commit_num;
        set_latest_state(self);
        output.extend_from_slice(&self.broadcast_start_view_msg());
        output.extend_from_slice(&self.backup_commit(self, last_commit_num));
        clear_quorum_tracker(self);
        output.push(set_primary(self));
        println!("Elected {:?} as primary of view {}", self.primary, self.view);
        // We may have just committed a Reconfiguration request. Check to see if we are still the
        // primary
        if self.me == compute_primary(self) {
            next!(primary, output)
        } else {
            next!(backup, output)
        }
    } else {
        next!(wait_for_do_view_change)
    }
}


    pub fn become_backup(ctx: &mut VrCtx, view: u64, op: u64, log: Vec<VrMsg>, commit_num: u64)
        -> Vec<VrEnvelope>
    {
        ctx.view = view;
        ctx.op = op;
        ctx.log = log;
        ctx.prepare_requests = PrepareRequests::new(ctx.new_config.replicas.len(),
                                                    ctx.idle_timeout_ms);
        ctx.quorum_tracker = QuorumTracker::new(ctx.quorum as usize);
        let mut output = Vec::new();
        output.push(set_primary(ctx));
        output.extend_from_slice(&send_prepare_ok_for_uncommitted_ops(ctx));
        ctx.backup_commit(commit_num);
        ctx.last_normal_view = ctx.view;
        output
    }

    pub fn backup_commit(&mut self, new_commit_num: u64) {
        for i in self.commit_num..new_commit_num {
            let msg = self.log[i as usize].clone();
            match msg {
                VrMsg::ClientRequest {op, ..} => {
                    self.backend.call(i+1, op);
                },
                VrMsg::Reconfiguration {replicas, ..} => {
                    self.old_config = self.new_config.clone();
                    self.new_config = VersionedReplicas {epoch: self.epoch, op: i+1, replicas: replicas};
                    self.backend.call(i+1, VrApiReq::Null);
                },
                _ => ()
            }
        }
        self.commit_num = new_commit_num;
    }


    pub fn announce_reconfiguration(&self) -> FsmOutput {
        FsmOutput::Announcement(NamespaceMsg::Reconfiguration {
            namespace_id: self.me.namespace_id.clone(),
            old_config: self.old_config.clone(),
            new_config: self.new_config.clone()
        }, self.me.clone())
    }

    pub fn send_epoch_started(&mut self) -> Vec<Envelope>{
        let mut output = Vec::new();
        let msg = self.epoch_started_msg();
        for r in replicas_to_replace(self) {
            output.push(Envelope::Peer(PeerEnvelope::new(r, self.me.clone(), msg.clone())));
        }
        output
    }

    pub fn replicas_to_replace(&self) -> Vec<Pid> {
        let new_set = HashSet::<Pid>::from_iter(self.new_config.replicas.clone());
        let old_set = HashSet::<Pid>::from_iter(self.old_config.replicas.clone());
        old_set.difference(&new_set).cloned().collect()
    }

    pub fn send_to_primary(&self, msg: VrMsg) -> Envelope {
        Envelope::Peer(PeerEnvelope::new(self.primary.as_ref().unwrap().clone(), self.me.clone(), msg))
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

    pub fn maybe_commit(&mut self, op: u64, from: Pid, correlation_id: CorrelationId)
        -> Vec<VrEnvelope>
        {
            self.prepare_requests.insert(op, from.clone());
            if self.prepare_requests.has_quorum(op) {
                let last_commit_num = self.commit_num;
                self.commit_num = op;
                primary_commit_known_committed_ops(self, last_commit_num)
            } else {
                Vec::new()
            }

        }


    pub fn primary_commit_known_committed_ops(&mut self, last_commit_num: u64) -> Vec<Envelope> {
    let mut output = Vec::new();
    let iter = self.prepare_requests.remove(self.commit_num);
    for i in last_commit_num..self.commit_num {
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
            VrMsg::Reconfiguration {client_req_num, epoch, ..} {
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
                                            request.correlation_id));
                self.update_self_for_new_epoch(i+1);
                output.push(self.announce_reconfiguration(self));
                output.extend_from_slice(&self.broadcast_commit_msg_old());
                output.extend_from_slice(&self.send_epoch_started());
            }
    }
    output
}

fn update_self_for_new_epoch(&mut self, op: u64) {
    self.epoch += 1;
    self.view = 0;
    self.old_config = self.new_config.clone();
    // Get the reconfiguration from the log. This always succeeds.
    if let VrMsg::Reconfiguration {ref replicas, ..} = self.log[(op-1) as usize] {
        self.new_config = VersionedReplicas {epoch: self.epoch,
                                            op: self.op,
                                            replicas: replicas.clone()};
    }
    unreachable!();
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

    pub fn set_state(&mut self, msg: VrMsg) -> Envelope {
        if let VrMsg::NewState {view, op, commit_num, log_tail, ..} = msg {
            self.view = view;
            self.op = op;
            clear_quorum_tracker(self);
            for m in log_tail {
                self.log.push(m);
            }
            self.backup_commit(commit_num);
            return set_primary(self);
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

    fn reset_quorum(&mut self, view: u64, from: Pid, msg: VrMsg) {
        self.last_received_time = SteadyTime::now();
        self.view = view;
        self.prepare_requests = PrepareRequests::new(self.new_config.replicas.len(),
                                                    self.idle_timeout_ms);
        self.quorum_tracker = QuorumTracker::new(self.quorum as usize);
        self.quorum_tracker.insert(from, msg);
    }

    fn reset_and_start_view_change(&mut self) -> Transition {
        self.last_received_time = SteadyTime::now();
        self.view += 1;
        self.prepare_requests = PrepareRequests::new(self.new_config.replicas.len(),
        self.idle_timeout_ms);
        self.quorum_tracker = QuorumTracker::new(self.quorum as usize);
        let envelope = clear_primary(self);
        self.start_view_change(vec![envelope])
    }

    fn start_view_change(self: &mut VrCtx, mut output: Vec<Envelope>) -> Transition {
        let msg = self.start_view_change_msg();
        output.extend(broadcast(self, msg));
        maybe_send_do_view_change(self, output)
    }

    pub fn maybe_send_do_view_change(&mut self, mut output: Vec<Envelope>) -> Transition {
        if self.quorum_tracker.has_quorum() {
            output.push(self.set_primary());
            let primary = self.primary.clone().unwrap();
            if primary == self.me {
                return next!(wait_for_do_view_change, output);
            }
            let do_view_change = self.do_view_change_msg();
            output.push(Envelope::Peer(PeerEnvelope::new(primary, self.me.clone(), do_view_change)));
        }
        next!(view_change, output)
    }


    fn set_latest_state(&mut self) {
        let mut largest_last_normal_view = 0;
        let mut largest_commit_num = 0;
        let mut largest_op = 0;
        let mut largest_log = Vec::new();
        // Find the latest values contained in received DoViewChange messages
        for (_, m) in self.quorum_tracker.drain() {
            if let VrMsg::DoViewChange{op, last_normal_view, commit_num, log, ..} = m {
                if last_normal_view > largest_last_normal_view {
                    largest_last_normal_view = last_normal_view;
                    largest_log = log;
                    largest_op = op;
                } else if last_normal_view == largest_last_normal_view && op > largest_op {
                    largest_last_normal_view = last_normal_view;
                    largest_log = log;
                    largest_op = op;
                }
                if commit_num > largest_commit_num {
                    largest_commit_num = commit_num;
                }
            }
        }
        // Compare received DoViewChange messages to the local state
        if largest_last_normal_view > self.last_normal_view {
            self.log = largest_log;
            self.op = largest_op;
        } else if largest_last_normal_view == self.last_normal_view && largest_op > self.op {
            self.log = largest_log;
            self.op = largest_op;
        }
        if largest_commit_num > self.commit_num {
            self.commit_num = largest_commit_num;
        }

        self.last_normal_view = self.view;
        self.clear_quorum_tracker();
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

