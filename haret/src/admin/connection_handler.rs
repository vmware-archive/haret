// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use rabble::{self, Pid, Envelope, ConnectionMsg, ConnectionHandler, CorrelationId};
use msg::Msg;
use super::messages::{AdminMsg, AdminReq, AdminRpy};

/// The connection handler for Admin Clients
pub struct AdminConnectionHandler {
    pid: Pid,
    id: u64,
    namespace_mgr: Pid,
    total_requests: u64,

    // The next reply we are waiting for
    waiting_for: u64,

    // Map of request ids to received responses. Responses received out of order are saved here.
    out_of_order_replies: HashMap<u64, AdminRpy>
}

impl AdminConnectionHandler {
    fn make_envelope(&mut self, pid: Pid, req: AdminReq) -> Envelope<Msg> {
        let c_id = CorrelationId::request(self.pid.clone(), self.id, self.total_requests);
        self.total_requests += 1;
        Envelope {
            to: pid,
            from: self.pid.clone(),
            msg: rabble::Msg::User(Msg::AdminReq(req)),
            correlation_id: Some(c_id)
        }
    }

    fn make_rabble_msg_envelope(&mut self, pid: Pid, msg: rabble::Msg<Msg>) -> Envelope<Msg> {
        let c_id = CorrelationId::request(self.pid.clone(), self.id, self.total_requests);
        self.total_requests += 1;
        Envelope {
            to: pid,
            from: self.pid.clone(),
            msg: msg,
            correlation_id: Some(c_id)
        }
    }

    fn write_successive_replies(&mut self,
                                output: &mut Vec<ConnectionMsg<AdminConnectionHandler>>)
    {
        self.waiting_for += 1;
        while self.waiting_for != self.total_requests {
            if let Some(rpy) = self.out_of_order_replies.remove(&self.waiting_for) {
                let c_id = CorrelationId::request(self.pid.clone(), self.id, self.waiting_for);
                output.push(ConnectionMsg::Client(AdminMsg::Rpy(rpy), c_id));
                self.waiting_for += 1;
            } else {
                break;
            }
        }
    }
}

impl ConnectionHandler for AdminConnectionHandler {
    type Msg = Msg;
    type ClientMsg = AdminMsg;

    fn new(pid: Pid, id: u64) -> AdminConnectionHandler {
        let namespace_mgr = Pid {
            name: "namespace_mgr".to_string(),
            group: None,
            node: pid.node.clone()
        };
        AdminConnectionHandler {
            pid: pid,
            id: id,
            namespace_mgr: namespace_mgr,
            total_requests: 0,
            waiting_for: 0,
            out_of_order_replies: HashMap::new()
        }
    }

    fn handle_envelope(&mut self,
                       envelope: Envelope<Msg>,
                       output: &mut Vec<ConnectionMsg<AdminConnectionHandler>>)
    {
        let Envelope {msg, correlation_id, ..} = envelope;
        let correlation_id = correlation_id.unwrap();
        let rpy = match msg {
            rabble::Msg::User(Msg::AdminRpy(rpy)) => rpy,
            rabble::Msg::ClusterStatus(status) => AdminRpy::ClusterStatus(status),
            rabble::Msg::Timeout => AdminRpy::Timeout,
            rabble::Msg::Metrics(metrics) => AdminRpy::Metrics(metrics),
            _ => unreachable!()
        };
        if correlation_id.request == Some(self.waiting_for) {
            output.push(ConnectionMsg::Client(AdminMsg::Rpy(rpy), correlation_id));
            self.write_successive_replies(output);
        } else {
            self.out_of_order_replies.insert(correlation_id.request.unwrap(), rpy);
        }
    }

    fn handle_network_msg(&mut self,
                          msg: AdminMsg,
                          output: &mut Vec<ConnectionMsg<AdminConnectionHandler>>)
    {
        if let AdminMsg::Req(req) = msg {
            let envelope = match req {
                AdminReq::GetReplicaState(pid) =>
                    self.make_envelope(pid.clone(), AdminReq::GetReplicaState(pid)),
                AdminReq::GetMetrics(pid) =>
                    self.make_rabble_msg_envelope(pid.clone(), rabble::Msg::GetMetrics),
                _ => {
                    let pid = self.namespace_mgr.clone();
                    self.make_envelope(pid, req)
                }
            };
            output.push(ConnectionMsg::Envelope(envelope));
        } else {
            let msg = AdminMsg::Rpy(AdminRpy::Error("Invalid Admin Request".to_string()));
            // CorrelationId doesn't matter here
            output.push(ConnectionMsg::Client(msg, CorrelationId::pid(self.pid.clone())));
        }
    }
}
