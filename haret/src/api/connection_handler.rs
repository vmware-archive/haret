// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use std::iter::FromIterator;
use protobuf::RepeatedField;
use rabble::{self, Pid, NodeId, Envelope, ConnectionMsg, ConnectionHandler, CorrelationId};
use admin::{AdminReq, AdminRpy};
use vr::{self, VrMsg, VrApiReq, VrApiRsp, VrApiError, ClientRequest, ClientReply};
use msg::Msg;
use namespace_msg::{NamespaceMsg, NamespaceId, ClientId};
use super::messages::*;

type Milliseconds = u64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiRpy {
    ClientRegistration {primary: Pid, new_registration: bool},
    Redirect {primary: Pid, api_addr: String},
    Retry(Milliseconds),
    UnknownNamespace
}

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
                              mut msg: RegisterClient,
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
                                mut msg: ConsensusRequest,
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
        unreachable!()
    }

    fn proto_tree_op_to_vr_tree_op(&self, mut msg: TreeOp) -> vr::TreeOp {
        if msg.has_create_node() {
            self.to_vr_create_node(msg.take_create_node())
        } else if msg.has_delete_node() {
            self.to_vr_delete_node(msg.take_delete_node())
        } else if msg.has_list_keys() {
            self.to_vr_list_keys(msg.take_list_keys())
        } else if msg.has_blob_put() {
            self.to_vr_blob_put(msg.take_blob_put())
        } else if msg.has_blob_get() {
            self.to_vr_blob_get(msg.take_blob_get())
        } else if msg.has_blob_size() {
            self.to_vr_blob_size(msg.take_blob_size())
        } else if msg.has_queue_push() {
            self.to_vr_queue_push(msg.take_queue_push())
        } else if msg.has_queue_pop() {
            self.to_vr_queue_pop(msg.take_queue_pop())
        } else if msg.has_queue_front() {
            self.to_vr_queue_front(msg.take_queue_front())
        } else if msg.has_queue_back() {
            self.to_vr_queue_back(msg.take_queue_back())
        } else if msg.has_queue_len() {
            self.to_vr_queue_len(msg.take_queue_len())
        } else if msg.has_set_insert() {
            self.to_vr_set_insert(msg.take_set_insert())
        } else if msg.has_set_remove() {
            self.to_vr_set_remove(msg.take_set_remove())
        } else if msg.has_set_contains() {
            self.to_vr_set_contains(msg.take_set_contains())
        } else if msg.has_set_union() {
            self.to_vr_set_union(msg.take_set_union())
        } else if msg.has_set_intersection() {
            self.to_vr_set_intersection(msg.take_set_intersection())
        } else if msg.has_set_difference() {
            self.to_vr_set_difference(msg.take_set_difference())
        } else if msg.has_set_symmetric_difference() {
            self.to_vr_set_symmetric_difference(msg.take_set_symmetric_difference())
        } else if msg.has_set_subset_path() {
            self.to_vr_set_subset_path(msg.take_set_subset_path())
        } else if msg.has_set_subset_set() {
            self.to_vr_set_subset_set(msg.take_set_subset_set())
        } else if msg.has_set_superset_path() {
            self.to_vr_set_superset_path(msg.take_set_superset_path())
        } else if msg.has_set_superset_set() {
            self.to_vr_set_superset_set(msg.take_set_superset_set())
        } else {
            unreachable!();
        }
    }

    fn handle_tree_op(&mut self,
                      pid: Pid,
                      client_id: String,
                      client_req_num: u64,
                      msg: TreeOp,
                      output: &mut Vec<ConnectionMsg<ApiConnectionHandler>>)
    {
        let vr_api_req = VrApiReq::TreeOp(self.proto_tree_op_to_vr_tree_op(msg));
        self.send_client_request(pid, client_id, client_req_num, vr_api_req, output)
    }

    fn handle_tree_cas(&mut self,
                       pid: Pid,
                       client_id: String,
                       client_req_num: u64,
                       mut msg: TreeCas,
                       output: &mut Vec<ConnectionMsg<ApiConnectionHandler>>)
    {
        let guards: Vec<_> = msg.take_guards().iter_mut().map(|g| {
            vr::Guard {path: g.take_path(), version: g.get_version()}
        }).collect();
        let ops: Vec<_> = msg.take_tree_ops().into_iter()
            .map(|op| self.proto_tree_op_to_vr_tree_op(op)).collect();
        let vr_api_req = VrApiReq::TreeCas(vr::TreeCas { guards: guards, ops: ops });
        self.send_client_request(pid, client_id, client_req_num, vr_api_req, output)
    }

    fn send_client_request(&mut self,
                           pid: Pid,
                           client_id: String,
                           client_req_num: u64,
                           req: VrApiReq,
                           output: &mut Vec<ConnectionMsg<ApiConnectionHandler>>)
    {
        let vrmsg = ClientRequest {op: req,
                                   client_id: client_id,
                                   request_num: client_req_num}.into();
        let envelope = self.make_envelope(pid, Msg::Vr(vrmsg));
        output.push(ConnectionMsg::Envelope(envelope));
    }

    fn to_vr_create_node(&self, mut msg: CreateNode) -> vr::TreeOp {
        let path = msg.take_path();
        let node_type = match msg.get_node_type() {
            NodeType::BLOB => vr::NodeType::Blob,
            NodeType::QUEUE => vr::NodeType::Queue,
            NodeType::SET => vr::NodeType::Set,
            NodeType::DIRECTORY => vr::NodeType::Directory
        };
        vr::TreeOp::CreateNode{path: path, ty: node_type}
    }

    fn to_vr_delete_node(&self, mut msg: DeleteNode) -> vr::TreeOp {
        let path = msg.take_path();
        vr::TreeOp::DeleteNode {path: path}
    }

    fn to_vr_list_keys(&self, mut msg: ListKeys) -> vr::TreeOp {
        let path = msg.take_path();
        vr::TreeOp::ListKeys {path: path}
    }

    fn to_vr_blob_put(&self, mut msg: BlobPut) -> vr::TreeOp {
        let path = msg.take_path();
        let val = msg.take_val();
        vr::TreeOp::BlobPut {path: path, val: val}
    }

    fn to_vr_blob_get(&self, mut msg: BlobGet) -> vr::TreeOp {
        let path = msg.take_path();
        vr::TreeOp::BlobGet {path: path}
    }

    fn to_vr_blob_size(&self, mut msg: BlobSize) -> vr::TreeOp {
        let path = msg.take_path();
        vr::TreeOp::BlobSize {path: path}
    }

    fn to_vr_queue_push(&self, mut msg: QueuePush) -> vr::TreeOp {
        let path = msg.take_path();
        let val = msg.take_val();
        vr::TreeOp::QueuePush {path: path, val: val}
    }

    fn to_vr_queue_pop(&self, mut msg: QueuePop) -> vr::TreeOp {
        let path = msg.take_path();
        vr::TreeOp::QueuePop {path: path}
    }

    fn to_vr_queue_front(&self, mut msg: QueueFront) -> vr::TreeOp {
        let path = msg.take_path();
        vr::TreeOp::QueueFront {path: path}
    }

    fn to_vr_queue_back(&self, mut msg: QueueBack) -> vr::TreeOp {
        let path = msg.take_path();
        vr::TreeOp::QueueBack {path: path}
    }

    fn to_vr_queue_len(&self, mut msg: QueueLen) -> vr::TreeOp {
        let path = msg.take_path();
        vr::TreeOp::QueueLen{path: path}
    }

    fn to_vr_set_insert(&self, mut msg: SetInsert) -> vr::TreeOp {
        let path = msg.take_path();
        let val = msg.take_val();
        vr::TreeOp::SetInsert {path: path, val: val}
    }

    fn to_vr_set_remove(&self, mut msg: SetRemove) -> vr::TreeOp {
        let path = msg.take_path();
        let val = msg.take_val();
        vr::TreeOp::SetRemove {path: path, val: val}
    }

    fn to_vr_set_contains(&self, mut msg: SetContains) -> vr::TreeOp {
        let path = msg.take_path();
        let val = msg.take_val();
        vr::TreeOp::SetContains {path: path, val: val}
    }

    fn to_vr_set_union(&self, mut msg: SetUnion) -> vr::TreeOp {
        let paths = msg.take_paths().into_vec();
        let sets: Vec<_> = msg.take_sets().into_iter().map(|mut s| {
            HashSet::from_iter(s.take_val().into_iter())
        }).collect();
        vr::TreeOp::SetUnion {paths: paths, sets: sets}
    }

    fn to_vr_set_intersection(&self, mut msg: SetIntersection) -> vr::TreeOp {
        let path1 = msg.take_path1();
        let path2 = msg.take_path2();
        vr::TreeOp::SetIntersection {path1: path1, path2: path2}
    }

    fn to_vr_set_difference(&self, mut msg: SetDifference) -> vr::TreeOp {
        let path1 = msg.take_path1();
        let path2 = msg.take_path2();
        vr::TreeOp::SetDifference {path1: path1, path2: path2}
    }

    fn to_vr_set_symmetric_difference(&self, mut msg: SetSymmetricDifference) -> vr::TreeOp {
        let path1 = msg.take_path1();
        let path2 = msg.take_path2();
        vr::TreeOp::SetSymmetricDifference {path1: path1, path2: path2}
    }

    fn to_vr_set_subset_path(&self, mut msg: SetSubsetPath) -> vr::TreeOp {
        let path1 = msg.take_path1();
        let path2 = msg.take_path2();
        vr::TreeOp::SetSubsetPath {path1: path1, path2: path2}
    }

    fn to_vr_set_subset_set(&self, mut msg: SetSubsetSet) -> vr::TreeOp {
        let path = msg.take_path();
        let set = HashSet::from_iter(msg.take_set().take_val().into_iter());
        vr::TreeOp::SetSubsetSet {path: path, set: set}
    }

    fn to_vr_set_superset_path(&self, mut msg: SetSupersetPath) -> vr::TreeOp {
        let path1 = msg.take_path1();
        let path2 = msg.take_path2();
        vr::TreeOp::SetSupersetPath {path1: path1, path2: path2}
    }

    fn to_vr_set_superset_set(&self, mut msg: SetSupersetSet) -> vr::TreeOp {
        let path = msg.take_path();
        let set = HashSet::from_iter(msg.take_set().take_val().into_iter());
        vr::TreeOp::SetSupersetSet {path: path, set: set}
    }

    fn error(&self, err: ApiError) -> ConnectionMsg<ApiConnectionHandler> {
        let mut response = ApiResponse::new();
        response.set_error(err);
        let mut msg = ApiMsg::new();
        msg.set_response(response);
        ConnectionMsg::Client(msg, CorrelationId::pid(self.pid.clone()))
    }
}

impl ConnectionHandler for ApiConnectionHandler {
    type Msg = Msg;
    type ClientMsg = ApiMsg;

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
                let mut reply = ConsensusReply::new();
                reply.set_epoch(epoch);
                reply.set_view(view);
                reply.set_request_num(request_num);
                let response = vr_api_rsp_to_proto_api_response(reply, value);
                let mut msg = ApiMsg::new();
                msg.set_response(response);
                output.push(ConnectionMsg::Client(msg, correlation_id));
            },
            rabble::Msg::User(Msg::AdminRpy(AdminRpy::Namespaces(namespaces))) => {
                let ids: Vec<_> = namespaces.map.keys().cloned().map(|k| k.0.to_string()).collect();
                let mut namespaces = Namespaces::new();
                namespaces.set_ids(RepeatedField::from_vec(ids));
                let mut response = ApiResponse::new();
                response.set_namespaces(namespaces);
                let mut msg = ApiMsg::new();
                msg.set_response(response);
                output.push(ConnectionMsg::Client(msg, correlation_id));
            },
            rabble::Msg::User(Msg::ApiRpy(reply)) => {
                let response = api_rpy_to_proto_api_response(reply);
                let mut msg = ApiMsg::new();
                msg.set_response(response);
                output.push(ConnectionMsg::Client(msg, correlation_id));
            },
            rabble::Msg::Timeout => {
                let mut response = ApiResponse::new();
                response.set_timeout(true);
                let mut msg = ApiMsg::new();
                msg.set_response(response);
                output.push(ConnectionMsg::Client(msg, correlation_id));
            },
            _ => {
                let err = self.error(VrApiError::InvalidMsg.into());
                output.push(err);
            }

        }
    }


    fn handle_network_msg(&mut self,
                          mut msg: ApiMsg,
                          output: &mut Vec<ConnectionMsg<ApiConnectionHandler>>)
    {
        if !msg.has_request() {
            let err = self.error(VrApiError::InvalidMsg.into());
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
            let err = self.error(VrApiError::InvalidMsg.into());
            output.push(err);
        }
    }
}

fn api_rpy_to_proto_api_response(reply: ApiRpy) -> ApiResponse {
    match reply {
        ApiRpy::ClientRegistration {primary, new_registration} => {
            let pid = pid_to_api_pid(primary);
            let mut cr = ClientRegistration::new();
            cr.set_primary(pid);
            cr.set_new_registration(new_registration);
            let mut response = ApiResponse::new();
            response.set_client_registration(cr);
            response
        },
        ApiRpy::Redirect {primary, api_addr} => {
            let pid = pid_to_api_pid(primary);
            let mut redirect = Redirect::new();
            redirect.set_primary(pid);
            redirect.set_api_addr(api_addr);
            let mut response = ApiResponse::new();
            response.set_redirect(redirect);
            response
        },
        ApiRpy::Retry(milliseconds) => {
            let mut retry = Retry::new();
            retry.set_milliseconds(milliseconds);
            let mut response = ApiResponse::new();
            response.set_retry(retry);
            response
        },
        ApiRpy::UnknownNamespace => {
            let mut response = ApiResponse::new();
            response.set_unknown_namespace(true);
            response
        }
    }
}

impl From<VrApiError> for ApiError {
    fn from(error: VrApiError) -> ApiError {
        let mut api_error = ApiError::new();
        match error {
            VrApiError::NotFound(path) => {
                let mut e = NotFound::new();
                e.set_path(path);
                api_error.set_not_found(e);
            },
            VrApiError::AlreadyExists(path) => {
                let mut e = AlreadyExists::new();
                e.set_path(path);
                api_error.set_already_exists(e);
            },
            VrApiError::DoesNotExist(path) => {
                let mut e = DoesNotExist::new();
                e.set_path(path);
                api_error.set_does_not_exist(e);
            },
            VrApiError::WrongType(path, ty) => {
                let mut e = WrongType::new();
                e.set_path(path);
                e.set_node_type(ty.into());
                api_error.set_wrong_type(e);
            },
            VrApiError::PathMustEndInDirectory(path) => {
                let mut e = PathMustEndInDirectory::new();
                e.set_path(path);
                api_error.set_path_must_end_in_directory(e);
            },
            VrApiError::PathMustBeAbsolute(path) => {
                let mut e = PathMustBeAbsolute::new();
                e.set_path(path);
                api_error.set_path_must_be_absolute(e);
            },
            VrApiError::CasFailed {path, expected, actual} => {
                let mut e = CasFailed::new();
                e.set_path(path);
                e.set_expected(expected);
                e.set_actual(actual);
                api_error.set_cas_failed(e);
            },
            VrApiError::BadFormat(s) => {
                let mut e = BadFormat::new();
                e.set_msg(s);
                api_error.set_bad_format(e);
            },
            VrApiError::Io(s) => {
                let mut e = Io::new();
                e.set_msg(s);
                api_error.set_io(e);
            },
            VrApiError::EncodingError(s) => {
                let mut e = EncodingError::new();
                e.set_msg(s);
                api_error.set_encoding(e);
            }
            VrApiError::InvalidCas(s) => {
                let mut e = InvalidCas::new();
                e.set_msg(s);
                api_error.set_invalid_cas(e);
            },
            VrApiError::Msg(s) => api_error.set_msg(s),
            VrApiError::CannotDeleteRoot => api_error.set_cannot_delete_root(true),
            VrApiError::InvalidMsg => api_error.set_invalid_msg(true),
            VrApiError::Timeout => api_error.set_timeout(true),
            VrApiError::NotEnoughReplicas => api_error.set_not_enough_replicas(true),
            VrApiError::BadEpoch => api_error.set_bad_epoch(true)
        }
        api_error
    }
}

impl From<vr::NodeType> for NodeType {
    fn from(ty: vr::NodeType) -> NodeType {
        match ty {
            vr::NodeType::Blob => NodeType::BLOB,
            vr::NodeType::Queue => NodeType::QUEUE,
            vr::NodeType::Set => NodeType::SET,
            vr::NodeType::Directory => NodeType::DIRECTORY
        }
    }
}

impl From<vr::TreeOpResult> for TreeOpResult {
    fn from(result: vr::TreeOpResult) -> TreeOpResult {
        let (mut result, version) = match result {
            vr::TreeOpResult::Ok(version) =>  {
                let mut result = TreeOpResult::new();
                result.set_ok(true);
                (result, version)
            },
            vr::TreeOpResult::Empty(version) =>  {
                let mut result = TreeOpResult::new();
                result.set_empty(true);
                (result, version)
            },
            vr::TreeOpResult::Bool(b, version) => {
                let mut result = TreeOpResult::new();
                result.set_bool(b);
                (result, version)
            },
            vr::TreeOpResult::Blob(blob, version) => {
                let mut result = TreeOpResult::new();
                result.set_blob(blob);
                (result, version)
            },
            vr::TreeOpResult::Int(i, version) => {
                let mut result = TreeOpResult::new();
                result.set_int(i);
                (result, version)
            },
            vr::TreeOpResult::Set(set, version) => {
                let val = RepeatedField::from_vec(set);
                let mut set = Set::new();
                set.set_val(val);
                let mut result = TreeOpResult::new();
                result.set_set(set);
                (result, version)
            },
            vr::TreeOpResult::Keys(keys) => {
                let key_vec: Vec<_>  = keys.into_iter().map(|(name, version)| {
                    let mut key = Key::new();
                    key.set_name(name);
                    key.set_version(version);
                    key
                }).collect();
                let mut keys = Keys::new();
                keys.set_keys(RepeatedField::from_vec(key_vec));
                let mut result = TreeOpResult::new();
                result.set_keys(keys);
                (result, None)
            }
        };
        if let Some(version) = version {
            result.set_optional_version(version);
        }
        result
    }
}

fn vr_api_rsp_to_proto_api_response(mut reply: ConsensusReply, value: VrApiRsp) -> ApiResponse {
    match value {
        VrApiRsp::Ok => reply.set_ok(true),
        VrApiRsp::TreeOpResult(result) =>  reply.set_tree_op_result(result.into()),
        VrApiRsp::TreeCasResult(results) => {
            let results: Vec<_> = results.into_iter().map(|op| op.into()).collect();
            let mut result = TreeCasResult::new();
            result.set_results(RepeatedField::from_vec(results));
            reply.set_tree_cas_result(result);
        },
        VrApiRsp::Path(path) => reply.set_path(path),
        VrApiRsp::Error(error) => reply.set_error(error.into())
    }
    let mut response = ApiResponse::new();
    response.set_consensus_reply(reply);
    response
}

fn pid_to_api_pid(pid: Pid) -> ApiPid {
    let mut api_pid = ApiPid::new();
    api_pid.set_name(pid.name);
    api_pid.set_node_name(pid.node.name);
    api_pid.set_node_addr(pid.node.addr);
    if pid.group.is_some() {
        api_pid.set_group(pid.group.unwrap());
    }
    api_pid
}

