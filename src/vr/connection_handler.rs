use std::collections::HashMap;
use rabble::{self, Pid, Envelope, ConnectionMsg, ConnectionHandler, CorrelationId};
use super::vr_api_messages::{VrClientMsg, VrApiReq, VrApiRsp};
use super::vrmsg::VrMsg;
use super::super::msg::Msg;

/// The connection handler for VR API clients
pub struct VrConnectionHandler {
    pid: Pid,
    id: usize,
    total_requests: usize,
    output: Vec<ConnectionMsg<VrConnectionHandler>>
}

impl VrConnectionHandler {
    pub fn make_envelope(&self, pid: Pid, msg: VrMsg) -> Envelope<Msg> {
        let c_id = CorrelationId::Request(self.pid.clone(), self.id, self.total_requests);
        self.total_requests += 1;
        Envelope {
            to: pid,
            from: self.pid.clone(),
            msg: rabble::Msg::User(Msg::Vr(msg)),
            correlation_id: c_id
        }
    }
}

impl ConnectionHandler for VrConnectionHandler {
    type Msg = Msg;
    type ClientMsg = VrClientMsg;

    fn new(pid: Pid, id: usize) -> VrConnectionHandler {
        VrConnectionHandler {
            pid: pid,
            id: id,
            total_requests: 0,
            output: Vec::new()
        }
    }

    fn handle_envelope(&mut self, envelope: Envelope<Msg>) ->
        &mut Vec<ConnectionMsg<VrConnectionHandler>>
    {
        let Envelope {msg, correlation_id, ..} = envelope;
        let correlation_id = correlation_id.unwrap();
        match msg {
            rabble::Msg::User(Msg::Vr(VrMsg::ClientReply {epoch, view, request_num, value})) => {
                let msg = VrClientMsg::Rpy {
                    epoch: epoch,
                    view: view,
                    request_num: request_num,
                    value: value
                };
                self.output.push(ConnectionMsg::ClientMsg(msg));
            },
            rabble::Msg::Timeout => {
                let msg = VrClientMsg::Rpy {
                    epoch: 0,
                    view: 0,
                    // ASSUME the request num from the client matches the request count on
                    // THIS CONNECTION. This is NOT TRUE if the client maintains the request count
                    // across connections. TODO: I'm leaning toward removing the request count
                    // altogether from clients as it provides little value without a global client
                    // table, which itself is trouble because of garbage collection issues.
                    request_num: correlation_id.request,
                    value: VrApiRsp::Timeout
                };
                self.output.push(ConnectionMsg::ClientMsg(msg, correlation_id));
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
            let vrmsg = VrMsg::ClientRequest {op: op, request_num: request_num};
            let envelope = self.make_envelope(pid, vrmsg);
            self.output.push(ConnectionMsg::Envelope(envelope));
        } else {
            let err = VrApiRsp::Error {msg: "Invalid VR Client Msg".to_string()};
            let msg = VrClientMsg::Rpy {epoch: 0, view: 0, request_num: 0, value: err};
            // CorrelationId doesn't matter here
            self.output.push(ConnectionMsg::ClientMsg(msg, CorrelationId::Pid(self.pid)));
        }
        &mut self.output
    }
}
