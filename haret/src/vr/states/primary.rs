// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::convert::{From, Into};
use std::collections::HashMap;
use rabble::{self, Pid, CorrelationId, Envelope};
use time::{SteadyTime, Duration};
use msg::Msg;
use namespace_msg::NamespaceMsg;
use vr::vr_fsm::{Transition, VrState, State};
use vr::vr_msg::{self, VrMsg, ClientOp, ClientRequest, ClientReply, Prepare, PrepareOk, Commit};
use vr::vr_msg::{Recovery, RecoveryResponse};
use vr::vr_ctx::VrCtx;
use api::{ApiRsp, ApiError};
use super::utils::PrepareRequests;
use super::common::normal;
use super::StateTransfer;

/// The primary state of the VR protocol operating in normal mode
state!(Primary {
    ctx: VrCtx,
    prepare_requests: PrepareRequests,
    reconfiguration_in_progress: bool,

    // A mapping of each replica in the new_config to it's min commit
    min_accepts: HashMap<Pid, u64>
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
            VrMsg::GetState(msg) => normal::handle_get_state(self, msg, from, cid, output),
            VrMsg::Recovery(msg) => self.handle_recovery(msg, from, cid, output),
            VrMsg::StartEpoch(_) => normal::handle_start_epoch(self, from, cid, output),
            VrMsg::StartViewChange(msg) =>
                normal::handle_start_view_change(self, msg, from, output),
            VrMsg::DoViewChange(msg) => normal::handle_do_view_change(self, msg, from, output),
            VrMsg::StartView(msg) => normal::handle_start_view(self, msg, cid, output),
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
            _ => self.into()
        }
    }
}

impl Primary {
    pub fn new(ctx: VrCtx) -> Primary {
        let size = ctx.new_config.replicas.len();
        let min_accepts = ctx.new_config.replicas
                                        .iter()
                                        .filter(|&pid| *pid != ctx.pid)
                                        .map(|pid| (pid.clone(), ctx.global_min_accept))
                                        .collect();
        Primary {
            prepare_requests: PrepareRequests::new(size, ctx.idle_timeout_ms),
            ctx: ctx,
            reconfiguration_in_progress: false,
            min_accepts: min_accepts
        }
    }

    /// Enter Primary state
    pub fn enter(ctx: VrCtx) -> VrState {
        Primary::new(ctx).into()
    }

    pub fn set_primary(&mut self, output: &mut Vec<Envelope<Msg>>) {
        let primary = self.ctx.compute_primary();
        info!(self.ctx.logger, "Elected primary"; "primary" => format!("{:?}", primary),
                                                  "view" => self.ctx.view);
        let msg = NamespaceMsg::NewPrimary(primary);
        output.push(self.ctx.namespace_mgr_envelope(msg));
    }

    pub fn commit(mut self, new_commit_num: u64, output: &mut Vec<Envelope<Msg>>) -> VrState {
        let lowest_prepared_op = self.prepare_requests.lowest_op;
        let mut iter = self.prepare_requests.remove(new_commit_num).into_iter();
        let start = self.ctx.commit_num - self.ctx.log_start;
        let end = new_commit_num - self.ctx.log_start;
        for i in start..end {
            let msg = self.ctx.log[i as usize].clone();
            match msg {
                ClientOp::Request(ClientRequest {op, request_num, ..}) => {
                    let rsp = self.ctx.backend.call(op);
                    if (i + self.ctx.log_start) >= lowest_prepared_op - 1 {
                        if let Some(req) = iter.next() {
                            // This primary prepared this request, and wasn't elected after it was
                            // prepared but before it was committed.
                            let cid = req.correlation_id;
                            let from = cid.pid.clone();
                            // This wasn't a prepare from a new primary.
                            // Send a response to the client
                            if cid.pid != self.ctx.pid {
                                output.push(self.client_reply(rsp, request_num, &from, &cid));
                            }
                        }
                    }
                },
                ClientOp::Reconfiguration(vr_msg::Reconfiguration {client_req_num,
                                                                   epoch,
                                                                   replicas, ..}) =>
                {
                    if (i + self.ctx.log_start) >= lowest_prepared_op - 1 {
                        if let Some(req) = iter.next() {
                            // This replica prepared this request as primary, and wasn't elected
                            // after it was prepared but before this replica committed it.
                            let cid = req.correlation_id;
                            // This wasn't a rebroadcast prepare from a new primary.
                            // Send a response to the client
                            if cid.pid != self.ctx.pid {
                                self.send_reconfig_client_reply(epoch, client_req_num, cid, output);
                            }
                        }
                    }

                    normal::commit_reconfiguration(&mut self.ctx, epoch, i+1, replicas, output);
                    self.set_primary(output);

                    // If the Reconfiguration is the last entry in the log, then this is either the
                    // primary that received the client request, or a view change occurred and this
                    // is a new primary in the epoch of the reconfiguration request processing the
                    // reconfiguration
                    //
                    // If the reconfiguration is not the last in the log, we don't want to
                    // transition, as the reconfiguration has already happened.
                    if i + 1 == end && end == self.ctx.log.len() as u64 {
                        self.ctx.commit_num = new_commit_num;
                        normal::gc_log(&mut self.ctx);
                        self.broadcast_commit_msg_old(output);
                        // We don't bother sending 'StartEpoch` requests to the new replicas here,
                        // since they aren't yet online.
                        return normal::enter_transitioning(self, output);
                    }
                }
            }
        }
        self.ctx.commit_num = new_commit_num;
        normal::gc_log(&mut self.ctx);
        self.into()
    }

    fn send_reconfig_client_reply(&self,
                                  epoch: u64,
                                  client_req_num: u64,
                                  cid: CorrelationId,
                                  output: &mut Vec<Envelope<Msg>>)
    {
        let to = cid.pid.clone();
        let from = self.ctx.pid.clone();
        let reply = ClientReply {
            epoch: epoch,
            view: 0,
            request_num: client_req_num,
            value: ApiRsp::Ok
        }.into();
        output.push(Envelope::new(to, from, reply, Some(cid)));
    }

    fn handle_client_request(mut self,
                             msg: ClientRequest,
                             cid: CorrelationId,
                             output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        self.broadcast_prepare(msg.into(), cid, output);
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
        self.broadcast_prepare(msg.into(), cid, output);
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

        // We can receive valid PrepareOk messages after we commit an operation, so this
        // clause must come before the next clause which compares the op to the commit_num
        if let Some(old_op) = self.min_accepts.get_mut(&from) {
            if op > *old_op {
                *old_op = op;
            }
        }
        self.update_global_min_accept();
        if op <= self.ctx.commit_num {
            return self.into();
        }
        self.track_prepare_ok(op, from);
        if self.has_commit_quorum(op) {
            debug!(self.ctx.logger, "primary: has commit quorum: op = {}", op);
            return self.commit(op, output);
        }
        self.into()
    }

    fn handle_recovery(self,
                       msg: Recovery,
                       to: Pid,
                       cid: CorrelationId,
                       output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        let vr_msg::Recovery {epoch, nonce} = msg;
        let response = self.recovery_response_msg(epoch, nonce);
        output.push(Envelope::new(to, self.ctx.pid.clone(), response, Some(cid)));
        self.into()
    }


    fn handle_tick(mut self, output: &mut Vec<Envelope<Msg>>) -> VrState {
        if self.idle_timeout() {
            self.ctx.last_received_time = SteadyTime::now();
            if self.ctx.op != self.ctx.commit_num {
                self.rebroadcast_prepare(output);
            }
            self.broadcast_commit_msg(output);
        }
        self.into()
    }

    fn rebroadcast_prepare(&mut self, output: &mut Vec<Envelope<Msg>>) {
        // This isn't the original request cid, but it doesn't matter
        // as it's already maintained in self.prepare_requests.
        let cid = CorrelationId::pid(self.ctx.pid.clone());
        let index = (self.ctx.op - self.ctx.log_start - 1) as usize;
        let msg = self.ctx.log[index].clone();
        if self.prepare_requests.is_empty() {
            // This is a new primary that didn't prepare this request orignally.
            // Therefore, no client reply will be required. Use this primary's pid for the cid then
            // filter before replying when the request is committed.
            self.prepare_requests.new_prepare(self.ctx.op, cid.clone());
        }
        self.ctx.broadcast(self.prepare_msg(msg), cid, output);
    }

    fn broadcast_prepare(&mut self,
                    msg: ClientOp,
                    cid: CorrelationId,
                    output: &mut Vec<Envelope<Msg>>)
    {
        self.ctx.last_received_time = SteadyTime::now();
        self.ctx.op += 1;
        self.ctx.log.push(msg.clone());
        self.prepare_requests.new_prepare(self.ctx.op, cid.clone());
        self.ctx.broadcast(self.prepare_msg(msg), cid, output);
    }

    fn prepare_msg(&self, msg: ClientOp) -> rabble::Msg<Msg> {
        Prepare {
            epoch: self.ctx.epoch,
            view: self.ctx.view,
            op: self.ctx.op,
            commit_num: self.ctx.commit_num,
            global_min_accept: self.ctx.global_min_accept,
            msg: msg
        }.into()
    }

    fn recovery_response_msg(&self, epoch: u64, nonce: u64) -> rabble::Msg<Msg> {
        let (old_config, new_config) = if self.ctx.epoch > epoch {
            (Some(self.ctx.old_config.clone()), Some(self.ctx.new_config.clone()))
        } else {
            (None, None)
        };

        RecoveryResponse {
            epoch: self.ctx.epoch,
            view: self.ctx.view,
            nonce: nonce,
            global_min_accept: self.ctx.global_min_accept,
            op: Some(self.ctx.op),
            commit_num: Some(self.ctx.commit_num),
            state: Some(self.ctx.backend.clone()),
            log_start: Some(self.ctx.log_start),
            log_tail: Some(self.ctx.log.clone()),
            old_config: old_config,
            new_config: new_config
        }.into()
    }

    fn track_prepare_ok(&mut self, op: u64, from: Pid) {
        if self.prepare_requests.is_empty() && op > self.ctx.commit_num {
            // We were just elected primary and haven't sent any prepare requests yet.
            // This is a PrepareOk message for uncommitted operations from a backup
            let cid = CorrelationId::pid(self.ctx.pid.clone());
            self.prepare_requests.new_prepare(op, cid.clone());
        }
        self.prepare_requests.insert(op, from);
    }

    fn has_commit_quorum(&mut self, op: u64) -> bool {
        self.prepare_requests.has_quorum(op)
    }

    /// Return Some(error msg) if the reconfiguration request is invalid. Return None on success.
    fn validate_reconfig(&self,
                         from: &Pid,
                         msg: &vr_msg::Reconfiguration,
                         cid: &CorrelationId) -> Option<Envelope<Msg>>
    {
        let &vr_msg::Reconfiguration {client_req_num, epoch, ref replicas} = msg;
        let err = if self.reconfiguration_in_progress {
            Some(ApiError::Msg("Reconfiguration in progress".to_owned()))
        } else if replicas.len() < 3 {
            Some(ApiError::NotEnoughReplicas)
        } else if epoch != self.ctx.epoch {
            Some(ApiError::BadEpoch)
        } else if *replicas == self.ctx.new_config.replicas {
            Some(ApiError::Msg("No change to existing configuration".to_owned()))
        } else {
            None
        };
        err.map(|e| self.client_reply(ApiRsp::Error(e), client_req_num, from, cid))
    }

    fn client_reply(&self,
                    rsp: ApiRsp,
                    client_req_num: u64,
                    to: &Pid,
                    cid: &CorrelationId) -> Envelope<Msg>
    {
        let reply = self.client_reply_msg(client_req_num, rsp);
        Envelope::new(to.clone(), self.ctx.pid.clone(), reply, Some(cid.clone()))
    }

    fn client_reply_msg(&self, client_req_num: u64, value: ApiRsp) -> rabble::Msg<Msg> {
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
            commit_num: self.ctx.commit_num,
            global_min_accept: self.ctx.global_min_accept
        }.into()
    }

    fn update_global_min_accept(&mut self) {
        let (_, global_min_accept) =
            self.min_accepts.iter().min_by(|&(_,&x), &(_,&y)| x.cmp(&y)).unwrap();
        self.ctx.global_min_accept = *global_min_accept;
    }

    fn broadcast_commit_msg(&mut self, output: &mut Vec<Envelope<Msg>>) {
        self.ctx.broadcast(self.commit_msg(),
                           CorrelationId::pid(self.ctx.pid.clone()),
                           output);
    }

    fn broadcast_commit_msg_old(&self, output: &mut Vec<Envelope<Msg>>) {
        // We need to commit the message in the old epoch. If we commit in the new epoch, which view
        // would we commit in? The backups would not know if the commit was from the last view they
        // were in in the old epoch and therefore would not know if the entry being committed had
        // been reordered. This would require the backup to unsafely truncate it's log to the last
        // known commit point and participate in state transfer. In addition to potential unsafety
        // this means that reconfiguration would take longer than necessary. Therefore we always
        // commit in the old epoch. Even if the commit fails or a partition occurs, this is safe
        // because a quorum has already acknowledged the reconfiguration. In this case a new
        // primary would be elected in the old epoch and commit the reconfiguration to complete the
        // transition.
        let msg = Commit {
            epoch: self.ctx.epoch - 1,
            view: self.ctx.view,
            commit_num: self.ctx.commit_num,
            global_min_accept: self.ctx.global_min_accept
        }.into();
        self.broadcast_old(msg, CorrelationId::pid(self.ctx.pid.clone()), output)
    }

    fn idle_timeout(&self) -> bool {
        SteadyTime::now() - self.ctx.last_received_time >
            Duration::milliseconds(self.ctx.primary_idle_timeout_ms as i64)
    }

    /// Wrap a VrMsg in an envelope and send to all old replicas
    pub fn broadcast_old(&self,
                         msg: rabble::Msg<Msg>,
                         cid: CorrelationId,
                         output: &mut Vec<Envelope<Msg>>)
    {
        output.extend(self.ctx.old_config.replicas.iter().cloned()
            .filter(|pid| *pid != self.ctx.pid)
            .map(|pid| Envelope::new(pid, self.ctx.pid.clone(), msg.clone(), Some(cid.clone()))));
    }

}

#[cfg(test)]
mod tests {

    extern crate slog;
    extern crate slog_stdlog;
    extern crate slog_term;
    extern crate slog_envlogger;

    use super::*;
    use rabble::{Pid, NodeId, CorrelationId};
    use vertree::NodeType;
    use vr::VersionedReplicas;
    use api::{ApiReq, TreeOp};
    use msg::Msg;
    use slog::{DrainExt, Logger};

    /// Set up logging to go to the terminal and be configured via `RUST_LOG`
    fn logger() -> Logger {
        let drain = slog_term::streamer().async().full().build();
        Logger::root(slog_envlogger::EnvLogger::new(drain.fuse()), o!())
    }

    fn node_id() -> NodeId {
        NodeId {
            name: "node1".to_owned(),
            addr: "127.0.0.1:9999".to_owned()
        }
    }

    fn test_pid() -> Pid {
        Pid {
            group: None,
            name: "test-pid".to_owned(),
            node: node_id()
        }
    }

    fn pids() -> Vec<Pid> {
        (1..4).into_iter().map(|i| Pid {
            group: None,
            name: format!("r{}", i),
            node: node_id()
        }).collect()
    }

    fn new_config() -> VersionedReplicas {
        VersionedReplicas {
            epoch: 1,
            op: 0,
            replicas: pids()
        }
    }

    fn take_primary(state: VrState) -> Primary {
        if let VrState::Primary(primary) = state {
            return primary;
        }
        panic!("Not in primary state: {:?}", state);
    }

    // Encapsulate a VrMsg in a rabble Msg
    macro_rules! m {
        ($msg:pat) => {
            rabble::Msg::User(Msg::Vr($msg))
        }
    }

    /// Have the primary handle a client request
    /// Mock the prepare ok from a backup and handle it
    /// Ensure that the client request commits and a reply is sent
    #[test]
    fn prepare_and_commit_client_request_succeeds() {
        let pids = pids();
        let mut ctx = VrCtx::new(logger(),
                                 pids[0].clone(),
                                 VersionedReplicas::new(),
                                 new_config());
        ctx.idle_timeout_ms = 0;
        ctx.primary_idle_timeout_ms = 0;
        let primary = Primary::new(ctx);
        let client_req = ClientRequest {
            op: ApiReq::TreeOp(TreeOp::CreateNode {path: "/a".to_owned(), ty: NodeType::Blob}),
            client_id: "client1".to_owned(),
            request_num: 1

        };
        let cid = CorrelationId::pid(test_pid());
        let mut output = Vec::new();

        // Assure that handling the client request does the right thing
        let state = primary.handle_client_request(client_req, cid.clone(), &mut output);
        let primary = take_primary(state);
        assert_eq!(primary.ctx.log.len(), 1);
        assert_eq!(primary.ctx.op, 1);
        // broadcast of prepare to the 2 backups
        assert_eq!(output.len(), 2);
        for envelope in output.drain(..) {
            assert_matches!(envelope.msg, m!(VrMsg::Prepare(_)));
        }

        // Create a mock PrepareOk and handle it
        let msg = PrepareOk {
            epoch: 1,
            view: 0,
            op: 1
        };
        assert_eq!(primary.ctx.commit_num, 0);
        let state = primary.handle_prepare_ok(msg, pids[1].clone(), cid.clone(), &mut output);
        let primary = take_primary(state);
        assert_eq!(primary.ctx.commit_num, 1);
        // The client reply
        assert_eq!(output.len(), 1);
        let envelope = output.pop().unwrap();
        assert_eq!(envelope.to, test_pid());
        assert_eq!(envelope.correlation_id, Some(cid));
        assert_matches!(envelope.msg, m!(VrMsg::ClientReply(_)));

        // Ensure that a commit broadcast gets sent
        let state = primary.handle_tick(&mut output);
        let primary = take_primary(state);
        // The log doesn't get garbage collected after the commit because only one backup replied
        assert_eq!(primary.ctx.log.len(), 1);
        assert_eq!(primary.ctx.log_start, 0);
        assert_eq!(primary.ctx.op, 1);
        assert_eq!(primary.ctx.commit_num, 1);
        // broadcast of commit to the 2 backups
        assert_eq!(output.len(), 2);
        for envelope in output.drain(..) {
            assert_matches!(envelope.msg, m!(VrMsg::Commit(_)));
        }

    }
}
