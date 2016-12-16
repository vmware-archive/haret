use rabble::{self, Pid, Envelope, ConnectionMsg, ConnectionHandler, CorrelationId};
use super::vr_api_messages::{VrClientMsg, VrApiReq, VrApiRsp, NamespaceId};
use admin::{AdminReq, AdminRpy};
use super::vrmsg::VrMsg;
use msg::Msg;
use namespace_msg::NamespaceMsg;

/// The connection handler for VR API clients
pub struct VrConnectionHandler {
    pid: Pid,
    id: u64,
    namespace_mgr: Pid,
    waiting_for: u64,
    total_requests: u64,
    output: Vec<ConnectionMsg<VrConnectionHandler>>
}

impl VrConnectionHandler {
    pub fn make_envelope(&mut self, pid: Pid, msg: Msg) -> Envelope<Msg> {
        let c_id = CorrelationId::request(self.pid.clone(), self.id, self.total_requests);
        self.waiting_for = self.total_requests;
        self.total_requests += 1;
        Envelope {
            to: pid,
            from: self.pid.clone(),
            msg: rabble::Msg::User(msg),
            correlation_id: Some(c_id)
        }
    }
}

impl ConnectionHandler for VrConnectionHandler {
    type Msg = Msg;
    type ClientMsg = VrClientMsg;

    fn new(pid: Pid, id: u64) -> VrConnectionHandler {
        let namespace_mgr = Pid {
            name: "namespace_mgr".to_string(),
            group: None,
            node: pid.node.clone()
        };
        VrConnectionHandler {
            pid: pid,
            id: id,
            namespace_mgr: namespace_mgr,
            waiting_for: 0,
            total_requests: 0,
            output: Vec::new()
        }
    }

    fn handle_envelope(&mut self, envelope: Envelope<Msg>) ->
        &mut Vec<ConnectionMsg<VrConnectionHandler>>
    {
        let Envelope {msg, correlation_id, ..} = envelope;
        let correlation_id = correlation_id.unwrap();

        // Ensure responses are recent
        if correlation_id.request.unwrap() != self.waiting_for {
            return &mut self.output;
        }
        self.waiting_for += 1;
        match msg {
            rabble::Msg::User(Msg::Vr(VrMsg::ClientReply {epoch, view, request_num, value})) => {
                let msg = VrClientMsg::Rsp {
                    epoch: epoch,
                    view: view,
                    request_num: request_num,
                    value: value
                };
                self.output.push(ConnectionMsg::Client(msg, correlation_id));
            },
            rabble::Msg::User(Msg::AdminRpy(AdminRpy::Namespaces(namespaces))) => {
                let ids: Vec<_> = namespaces.map.keys().cloned()
                    .map(|k| NamespaceId(k.to_string())).collect();
                let msg = VrClientMsg::Rsp {
                    epoch: 0,
                    view: 0,
                    request_num: 0,
                    value: VrApiRsp::Namespaces(ids)
                };
                self.output.push(ConnectionMsg::Client(msg, correlation_id));
            },
            rabble::Msg::User(Msg::VrApiRsp(rsp)) => {
                let msg = VrClientMsg::Rsp {
                    epoch: 0,
                    view: 0,
                    request_num: correlation_id.request.unwrap(),
                    value: rsp
                };
                self.output.push(ConnectionMsg::Client(msg, correlation_id));
            },
            rabble::Msg::Timeout => {
                let msg = VrClientMsg::Rsp {
                    epoch: 0,
                    view: 0,
                    // ASSUME the request num from the client matches the request count on
                    // THIS CONNECTION. This is NOT TRUE if the client maintains the request count
                    // across connections. TODO: I'm leaning toward removing the request count
                    // altogether from clients as it provides little value without a global client
                    // table, which itself is trouble because of garbage collection issues.
                    request_num: correlation_id.request.unwrap(),
                    value: VrApiRsp::Timeout
                };
                self.output.push(ConnectionMsg::Client(msg, correlation_id));
            },
            _ => {
                // TODO: Log Error
            }

        }
        &mut self.output
    }


    fn handle_network_msg(&mut self, msg: VrClientMsg) ->
        &mut Vec<ConnectionMsg<VrConnectionHandler>>
    {
        if let VrClientMsg::Req {pid, op, request_num} = msg {
            if op == VrApiReq::GetNamespaces {
                let pid = self.namespace_mgr.clone();
                let admin_msg = Msg::AdminReq(AdminReq::GetNamespaces);
                let envelope = self.make_envelope(pid, admin_msg);
                self.output.push(ConnectionMsg::Envelope(envelope));
                return &mut self.output;
            }
            if let VrApiReq::RegisterClient(client_id, namespace_id) = op {
                let pid = self.namespace_mgr.clone();
                let msg = Msg::Namespace(NamespaceMsg::RegisterClient(client_id, namespace_id));
                let envelope = self.make_envelope(pid, msg);
                self.output.push(ConnectionMsg::Envelope(envelope));
                return &mut self.output;
            }
            if pid.is_none() {
                let err = VrApiRsp::Error {msg: "Pid required for this message".to_string()};
                let msg = VrClientMsg::Rsp {epoch: 0, view: 0, request_num: 0, value: err};
                // CorrelationId doesn't matter here
                self.output.push(ConnectionMsg::Client(msg, CorrelationId::pid(self.pid.clone())));
            } else {
                let vrmsg = VrMsg::ClientRequest {op: op, request_num: request_num};
                let envelope = self.make_envelope(pid.unwrap(), Msg::Vr(vrmsg));
                self.output.push(ConnectionMsg::Envelope(envelope));
            }
        } else {
            let err = VrApiRsp::Error {msg: "Invalid VR Client Msg".to_string()};
            let msg = VrClientMsg::Rsp {epoch: 0, view: 0, request_num: 0, value: err};
            // CorrelationId doesn't matter here
            self.output.push(ConnectionMsg::Client(msg, CorrelationId::pid(self.pid.clone())));
        }
        &mut self.output
    }
}
