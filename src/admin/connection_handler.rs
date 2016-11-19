use std::collections::HashMap;
use rabble::{Pid, Envelope, ConnectionMsg, ConnectionHandler, ClusterStatus};
use msg::Msg;

/// The connection handler for Admin Clients
pub struct AdminConnectionHandler {
    pid: Pid,
    id: usize,
    namespace_mgr: Pid,
    total_requests: usize,
    output: Vec<ConnectionMsg<AdminConnectionHandler>>,

    // The next reply we are waiting for
    waiting_for: usize,

    // Map of request ids to received responses. Responses received out of order are saved here.
    out_of_order_replies: HashMap<usize, AdminRpy>
}

impl AdminConnectionHandler {
    pub fn make_envelope(&self, pid: Pid, req: AdminReq) -> Envelope {
        let c_id = CorrelationId::Request(self.pid.clone(), self.id, self.total_requests);
        self.total_requests++;
        Envelope {
            to: pid,
            from: self.pid.clone(),
            msg: rabble::Msg::User(Msg::AdminReq(req)),
            correlation_id: c_id
        }
    }

    pub fn write_successive_replies(&mut self) {
        self.waiting_for += 1;
        while self.waiting_for != self.total_requests {
            if let Some(rpy) = self.out_of_order_replies.remove(self.waiting_for) {
                self.output.push(ConnectionMsg::ClientMsg(AdminMsg::Rpy(rpy))),
            } else {
                break;
            }
        }
    }
}

impl ConnectionHandler for AdminConnectionHandler {
    type Msg = Msg;
    type ClientMsg = AdminMsg;

    fn new(pid: Pid, id: usize) -> AdminConnectionHandler {
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
            output: Vec::new(),
            waiting_for: 0,
            out_of_order_replies: HashMap::new()
        }
    }

    fn handle_envelope(&mut self, envelope: Envelope<Msg>) ->
        &mut Vec<ConnectionMsg<AdminConnectionHandler>>
    {
        let Envelope {msg, correlation_id, ..} = envelope;
        let correlation_id = correlation_id.unwrap();
        let rpy = match msg {
            rabble::Msg::User(Msg::AdminRpy(rpy)) => rpy
            rabble::Msg::ClusteStatus(status) => AdminRpy::ClusteStatus(status),
            rabble::Msg::Timeout => AdminRpy::Timeout,
            _ => unreachable!()
        };
        if correlation_id.request == self.waiting_for {
            self.output.push(ConnectionMsg::ClientMsg(AdminMsg::Rpy(rpy))),
            self.write_successive_replies();
        } else {
            self.out_of_order_replies.insert(correlation_id.request, rpy);
        }
        &mut self.output
    }

    fn handle_network_msg(&mut self, msg: AdminMsg) ->
        &mut Vec<ConnectionMsg<AdminConnectionHandler>>
    {
        if let AdminMsg::Req(req) = msg {
            let envelope = if let AdminReq::GetReplicaState(pid) = req {
                self.make_envelope(pid.clone(), AdminReq::GetReplicaState(pid));
            } else {
                self.make_envelope(self.namespace_mgr.clone(), req);
            }
            self.output.push(ConnectionMsg::Envelope(envelope));
        } else {
            let msg = AdminMsg::Rpy(AdminRpy::Error("Invalid Admin Request".to_string()));
            // CorrelationId doesn't matter here
            self.output.push(ConnectionMsg::ClientMsg(msg, CorrelationId::Pid(self.pid)));
        }
        &mut self.output
    }
}
