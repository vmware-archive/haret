// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use protobuf::RepeatedField;
use rabble::{self, Pid, NodeId, Envelope, ConnectionMsg, ConnectionHandler, CorrelationId};
use admin::{AdminReq, AdminRpy};
use vr::{VrMsg, ClientRequest, ClientReply};
use msg::Msg;
use namespace_msg::{NamespaceMsg, NamespaceId, ClientId};
use super::internal_api_messages::{self, ApiReq, ApiError, TreeCas};
use super::messages as protobuf;
use vertree::Guard;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientRegistrationRpy {
    ClientRegistration {primary: Pid, new_registration: bool},
    Redirect {primary: Pid, api_addr: String},
    Retry(Milliseconds),
    UnknownNamespace
}

type Milliseconds = u64;

/// The connection handler for API clients
pub struct ApiConnectionHandler {
    pid: Pid,
    id: u64,
    namespace_mgr: Pid,
    waiting_for: u64,
    total_requests: u64
}

impl ApiConnectionHandler {
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

    fn handle_get_namespaces(&mut self, output: &mut Vec<ConnectionMsg<ApiConnectionHandler>>) {
        let pid = self.namespace_mgr.clone();
        let admin_msg = Msg::AdminReq(AdminReq::GetNamespaces);
        let envelope = self.make_envelope(pid, admin_msg);
        output.push(ConnectionMsg::Envelope(envelope));
    }

    fn handle_register_client(&mut self,
                              mut msg: protobuf::RegisterClient,
                              output: &mut Vec<ConnectionMsg<ApiConnectionHandler>>)
    {
        let client_id = msg.take_client_id();
        let namespace_id = msg.take_namespace_id();
        let pid = self.namespace_mgr.clone();
        let msg = Msg::Namespace(NamespaceMsg::RegisterClient(ClientId(client_id),
                                                              NamespaceId(namespace_id)));
        let envelope = self.make_envelope(pid, msg);
        output.push(ConnectionMsg::Envelope(envelope));
    }

    fn handle_consensus_request(&mut self,
                                mut msg: protobuf::ConsensusRequest,
                                output: &mut Vec<ConnectionMsg<ApiConnectionHandler>>)
    {
        let mut api_pid = msg.take_to();
        let pid = Pid {
            name: api_pid.take_name(),
            group: Some(api_pid.take_group()),
            node: NodeId {
                name: api_pid.take_node_name(),
                addr: api_pid.take_node_addr()
            }
        };
        let client_id = msg.take_client_id();
        let client_req_num = msg.get_client_request_num();
        if msg.has_tree_op() {
            return self.handle_tree_op(pid,
                                       client_id,
                                       client_req_num,
                                       msg.take_tree_op(),
                                       output);
        }
        if msg.has_tree_cas() {
            return self.handle_tree_cas(pid,
                                        client_id,
                                        client_req_num,
                                        msg.take_tree_cas(),
                                        output);
        }
        output.push(self.error(ApiError::InvalidMsg.into()));
    }

    fn handle_tree_op(&mut self,
                      pid: Pid,
                      client_id: String,
                      client_req_num: u64,
                      msg: protobuf::TreeOp,
                      output: &mut Vec<ConnectionMsg<ApiConnectionHandler>>)
    {
        match internal_api_messages::proto_to_tree_op(msg) {
            Ok(tree_op) => {
                let api_req = ApiReq::TreeOp(tree_op);
                self.send_client_request(pid, client_id, client_req_num, api_req, output);
            },
            Err(e) => output.push(self.error(e.into()))
        }
    }

    fn handle_tree_cas(&mut self,
                       pid: Pid,
                       client_id: String,
                       client_req_num: u64,
                       mut msg: protobuf::TreeCas,
                       output: &mut Vec<ConnectionMsg<ApiConnectionHandler>>)
    {
        let guards: Vec<_> = msg.take_guards().iter_mut().map(|g| {
            Guard {path: g.take_path(), version: g.get_version()}
        }).collect();

        let pb_ops = msg.take_tree_ops();
        let mut ops = Vec::with_capacity(pb_ops.len());
        for op in pb_ops.into_iter() {
            match internal_api_messages::proto_to_tree_op(op) {
                Ok(tree_op) => {
                    ops.push(tree_op)
                },
                Err(e) => {
                    output.push(self.error(e.into()));
                    return;
                }
            }
        }

        let api_req = ApiReq::TreeCas(TreeCas { guards: guards, ops: ops });
        self.send_client_request(pid, client_id, client_req_num, api_req, output);
    }

    fn send_client_request(&mut self,
                           pid: Pid,
                           client_id: String,
                           client_req_num: u64,
                           req: ApiReq,
                           output: &mut Vec<ConnectionMsg<ApiConnectionHandler>>)
    {
        let vrmsg = ClientRequest {op: req,
                                   client_id: client_id,
                                   request_num: client_req_num}.into();
        let envelope = self.make_envelope(pid, Msg::Vr(vrmsg));
        output.push(ConnectionMsg::Envelope(envelope));
    }

    fn error(&self, err: protobuf::ApiError) -> ConnectionMsg<ApiConnectionHandler> {
        let mut response = protobuf::ApiResponse::new();
        response.set_error(err);
        let mut msg = protobuf::ApiMsg::new();
        msg.set_response(response);
        ConnectionMsg::Client(msg, CorrelationId::pid(self.pid.clone()))
    }
}

impl ConnectionHandler for ApiConnectionHandler {
    type Msg = Msg;
    type ClientMsg = protobuf::ApiMsg;

    fn new(pid: Pid, id: u64) -> ApiConnectionHandler {
        let namespace_mgr = Pid {
            name: "namespace_mgr".to_string(),
            group: None,
            node: pid.node.clone()
        };
        ApiConnectionHandler {
            pid: pid,
            id: id,
            namespace_mgr: namespace_mgr,
            waiting_for: 0,
            total_requests: 0
        }
    }

    fn handle_envelope(&mut self,
                       envelope: Envelope<Msg>,
                       output: &mut Vec<ConnectionMsg<ApiConnectionHandler>>)
    {
        let Envelope {msg, correlation_id, ..} = envelope;
        let correlation_id = correlation_id.unwrap();

        // Ensure responses are recent
        if correlation_id.request.unwrap() != self.waiting_for {
            return;
        }
        self.waiting_for += 1;
        match msg {
            rabble::Msg::User(Msg::Vr(VrMsg::ClientReply(client_reply))) => {
                let ClientReply {epoch, view, request_num, value} = client_reply;
                let mut reply = protobuf::ConsensusReply::new();
                reply.set_epoch(epoch);
                reply.set_view(view);
                reply.set_request_num(request_num);
                let response = internal_api_messages::vr_api_rsp_to_proto_api_response(reply,
                                                                                       value);
                let mut msg = protobuf::ApiMsg::new();
                msg.set_response(response);
                output.push(ConnectionMsg::Client(msg, correlation_id));
            },
            rabble::Msg::User(Msg::AdminRpy(AdminRpy::Namespaces(namespaces))) => {
                let ids: Vec<_> = namespaces.map.keys().cloned().map(|k| k.0.to_string()).collect();
                let mut namespaces = protobuf::Namespaces::new();
                namespaces.set_ids(RepeatedField::from_vec(ids));
                let mut response = protobuf::ApiResponse::new();
                response.set_namespaces(namespaces);
                let mut msg = protobuf::ApiMsg::new();
                msg.set_response(response);
                output.push(ConnectionMsg::Client(msg, correlation_id));
            },
            rabble::Msg::User(Msg::ClientRegistrationRpy(reply)) => {
                let response = api_rpy_to_proto_api_response(reply);
                let mut msg = protobuf::ApiMsg::new();
                msg.set_response(response);
                output.push(ConnectionMsg::Client(msg, correlation_id));
            },
            rabble::Msg::Timeout => {
                let mut response = protobuf::ApiResponse::new();
                response.set_timeout(true);
                let mut msg = protobuf::ApiMsg::new();
                msg.set_response(response);
                output.push(ConnectionMsg::Client(msg, correlation_id));
            },
            _ => {
                let err = self.error(ApiError::InvalidMsg.into());
                output.push(err);
            }

        }
    }


    fn handle_network_msg(&mut self,
                          mut msg: protobuf::ApiMsg,
                          output: &mut Vec<ConnectionMsg<ApiConnectionHandler>>)
    {
        if !msg.has_request() {
            let err = self.error(ApiError::InvalidMsg.into());
            output.push(err);
            return;
        }
        let mut api_request = msg.take_request();
        if api_request.has_get_namespaces() {
            self.handle_get_namespaces(output);
        } else if api_request.has_register_client() {
            let msg = api_request.take_register_client();
            self.handle_register_client(msg, output);
        } else if api_request.has_consensus_request() {
            let msg = api_request.take_consensus_request();
            self.handle_consensus_request(msg, output);
        } else {
            let err = self.error(ApiError::InvalidMsg.into());
            output.push(err);
        }
    }
}

fn api_rpy_to_proto_api_response(reply: ClientRegistrationRpy) -> protobuf::ApiResponse {
    match reply {
        ClientRegistrationRpy::ClientRegistration {primary, new_registration} => {
            let mut cr = protobuf::ClientRegistration::new();
            cr.set_primary(primary.into());
            cr.set_new_registration(new_registration);
            let mut response = protobuf::ApiResponse::new();
            response.set_client_registration(cr);
            response
        },
        ClientRegistrationRpy::Redirect {primary, api_addr} => {
            let mut redirect = protobuf::Redirect::new();
            redirect.set_primary(primary.into());
            redirect.set_api_addr(api_addr);
            let mut response = protobuf::ApiResponse::new();
            response.set_redirect(redirect);
            response
        },
        ClientRegistrationRpy::Retry(milliseconds) => {
            let mut retry = protobuf::Retry::new();
            retry.set_milliseconds(milliseconds);
            let mut response = protobuf::ApiResponse::new();
            response.set_retry(retry);
            response
        },
        ClientRegistrationRpy::UnknownNamespace => {
            let mut response = protobuf::ApiResponse::new();
            response.set_unknown_namespace(true);
            response
        }
    }
}

impl From<ApiError> for protobuf::ApiError {
    fn from(error: ApiError) -> protobuf::ApiError {
        let mut api_error = protobuf::ApiError::new();
        match error {
            ApiError::NotFound(path) => {
                let mut e = protobuf::NotFound::new();
                e.set_path(path);
                api_error.set_not_found(e);
            },
            ApiError::AlreadyExists(path) => {
                let mut e = protobuf::AlreadyExists::new();
                e.set_path(path);
                api_error.set_already_exists(e);
            },
            ApiError::DoesNotExist(path) => {
                let mut e = protobuf::DoesNotExist::new();
                e.set_path(path);
                api_error.set_does_not_exist(e);
            },
            ApiError::WrongType(path, ty) => {
                let mut e = protobuf::WrongType::new();
                e.set_path(path);
                e.set_node_type(ty.into());
                api_error.set_wrong_type(e);
            },
            ApiError::PathMustEndInDirectory(path) => {
                let mut e = protobuf::PathMustEndInDirectory::new();
                e.set_path(path);
                api_error.set_path_must_end_in_directory(e);
            },
            ApiError::PathMustBeAbsolute(path) => {
                let mut e = protobuf::PathMustBeAbsolute::new();
                e.set_path(path);
                api_error.set_path_must_be_absolute(e);
            },
            ApiError::CasFailed {path, expected, actual} => {
                let mut e = protobuf::CasFailed::new();
                e.set_path(path);
                e.set_expected(expected);
                e.set_actual(actual);
                api_error.set_cas_failed(e);
            },
            ApiError::BadFormat(s) => {
                let mut e = protobuf::BadFormat::new();
                e.set_msg(s);
                api_error.set_bad_format(e);
            },
            ApiError::Io(s) => {
                let mut e = protobuf::Io::new();
                e.set_msg(s);
                api_error.set_io(e);
            },
            ApiError::EncodingError(s) => {
                let mut e = protobuf::EncodingError::new();
                e.set_msg(s);
                api_error.set_encoding(e);
            }
            ApiError::InvalidCas(s) => {
                let mut e = protobuf::InvalidCas::new();
                e.set_msg(s);
                api_error.set_invalid_cas(e);
            },
            ApiError::Msg(s) => api_error.set_msg(s),
            ApiError::CannotDeleteRoot => api_error.set_cannot_delete_root(true),
            ApiError::InvalidMsg => api_error.set_invalid_msg(true),
            ApiError::Timeout => api_error.set_timeout(true),
            ApiError::NotEnoughReplicas => api_error.set_not_enough_replicas(true),
            ApiError::BadEpoch => api_error.set_bad_epoch(true)
        }
        api_error
    }
}
