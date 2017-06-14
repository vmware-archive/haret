// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::convert::{From, Into};
use rabble::{self, Pid, CorrelationId, Envelope};
use time::{SteadyTime, Duration};
use msg::Msg;
use namespace_msg::NamespaceMsg;
use vr::vr_fsm::{Transition, VrState, State};
use vr::vr_msg::{self, VrMsg, ClientOp, ClientRequest, ClientReply, Prepare, PrepareOk, Commit};
use vr::vr_ctx::{VrCtx, DEFAULT_PRIMARY_TICK_MS};
use vr::vr_api_messages::{VrApiRsp, VrApiError};
use super::utils::PrepareRequests;
use super::common::normal;
use super::StateTransfer;

/// The primary state of the VR protocol operating in normal mode
state!(Primary {
    ctx: VrCtx,
    prepare_requests: PrepareRequests,
    tick_ms: i64,
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
            VrMsg::GetState(msg) => normal::handle_get_state(self, msg, from, cid, output),
            VrMsg::Recovery(msg) => normal::handle_recovery(self, msg, from, cid, output),
            VrMsg::StartEpoch(_) => normal::handle_start_epoch(self, from, cid, output),
            VrMsg::StartViewChange(msg) =>
                normal::handle_start_view_change(self, msg, from, output),
            VrMsg::DoViewChange(msg) => normal::handle_do_view_change(self, msg, from, output),
            VrMsg::StartView(msg) => normal::handle_start_view(self, msg, output),
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
    pub fn new(ctx: VrCtx, tick_ms: i64) -> Primary {
        let size = ctx.new_config.replicas.len();
        Primary {
            prepare_requests: PrepareRequests::new(size, ctx.idle_timeout_ms),
            ctx: ctx,
            tick_ms: tick_ms,
            reconfiguration_in_progress: false
        }
    }

    /// Enter Primary state
    pub fn enter(ctx: VrCtx) -> VrState {
        Primary::new(ctx, DEFAULT_PRIMARY_TICK_MS).into()
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
        self.ctx.log.push(msg);
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
                        if let Some(req) = iter.next() {
                            // This primary prepared this request, and wasn't elected after it was
                            // prepared but before it was committed.
                            let cid = req.correlation_id;
                            let from = cid.pid.clone();
                            output.push(self.client_reply(rsp, request_num, &from, &cid));
                        }
                    }
                },
                ClientOp::Reconfiguration(vr_msg::Reconfiguration {client_req_num,
                                                                   epoch,
                                                                   replicas, ..}) =>
                {
                    if i >= lowest_prepared_op - 1 {
                        if let Some(req) = iter.next() {
                            // This replica prepared this request as primary, and wasn't elected
                            // after it was prepared but before this replica committed it.
                            let cid = req.correlation_id;
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
                    }

                    self.ctx.epoch = epoch + 1;
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
                        return normal::enter_transitioning(self, output);
                    }
                }
            }
        }
        self.into()
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
            commit_num: self.ctx.commit_num
        }.into();
        self.ctx.broadcast_old(msg, CorrelationId::pid(self.ctx.pid.clone()), output)
    }

    pub fn set_primary(&mut self, output: &mut Vec<Envelope<Msg>>) {
        let primary = self.ctx.compute_primary();
        info!(self.ctx.logger, "Elected primary"; "primary" => format!("{:?}", primary),
                                                  "view" => self.ctx.view);
        let msg = NamespaceMsg::NewPrimary(primary);
        output.push(self.ctx.namespace_mgr_envelope(msg));
    }

    fn idle_timeout(&self) -> bool {
        SteadyTime::now() - self.ctx.last_received_time >
            Duration::milliseconds(self.tick_ms as i64)
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
    use vr::{VersionedReplicas, VrApiReq, TreeOp, NodeType};
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
    fn prepare_client_request_succeeds() {
        let pids = pids();
        let mut ctx = VrCtx::new(logger(),
                                 pids[0].clone(),
                                 VersionedReplicas::new(),
                                 new_config());
        ctx.idle_timeout_ms = 0;
        let primary = Primary::new(ctx, 0);
        let client_req = ClientRequest {
            op: VrApiReq::TreeOp(TreeOp::CreateNode {path: "/a".to_owned(), ty: NodeType::Blob}),
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
    }
}
