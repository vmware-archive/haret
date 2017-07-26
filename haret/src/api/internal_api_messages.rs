// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! This module contains all messages that are embedded in consensus requests
//! and perform API operations on the consensus backend after being committed.
//!
//! The messages are independent of any specific consensus protocol and therefore
//! isolated from the consensus code.
//!
//! The API messages are however coupled tightly to the specific backend that allows
//! performing those operations.

use std::collections::HashSet;
use std::iter::FromIterator;
use protobuf::RepeatedField;
use rabble::Pid;
use vertree::{self, Value, ErrorKind, NodeType, Guard};
use super::messages as protobuf;

type Version = u64;

/// Requests that get passed through consensus and executed on the backend vertree
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ApiReq {
    TreeOp(TreeOp),
    TreeCas(TreeCas),
}

/// Replies after execution on backend vertree
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ApiRsp {
    Ok,
    TreeOpResult(TreeOpResult),
    TreeCasResult(Vec<TreeOpResult>),
    Path(String),
    Error(ApiError)
}

/// An operation on a vertree
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TreeOp {
    Snapshot {directory: String},
    CreateNode {path: String, ty: NodeType},
    DeleteNode {path: String},
    ListKeys {path: String},
    BlobPut {path: String, val: Vec<u8>},
    BlobGet {path: String},
    BlobSize {path: String},
    QueuePush {path: String, val: Vec<u8>},
    QueuePop {path: String},
    QueueFront {path: String},
    QueueBack {path: String},
    QueueLen {path: String},
    SetInsert {path: String, val: Vec<u8>},
    SetRemove {path: String, val: Vec<u8>},
    SetContains {path: String, val: Vec<u8>},
    SetUnion {paths: Vec<String>, sets: Vec<HashSet<Vec<u8>>>},
    SetIntersection {path1: String, path2: String},
    SetDifference {path1: String, path2: String},
    SetSymmetricDifference {path1: String, path2: String},
    SetSubsetPath {path1: String, path2: String},
    SetSubsetSet {path: String,  set: HashSet<Vec<u8>>},
    SetSupersetPath {path1: String, path2: String},
    SetSupersetSet {path: String,  set: HashSet<Vec<u8>>}
}

impl TreeOp {
    pub fn is_write(&self) -> bool {
        match *self {
            TreeOp::Snapshot {..} |
            TreeOp::CreateNode {..} |
            TreeOp::DeleteNode {..} |
            TreeOp::BlobPut {..} |
            TreeOp::QueuePush {..} |
            TreeOp::QueuePop {..} |
            TreeOp::SetInsert {..} |
            TreeOp::SetRemove {..} => true,
            _ => false
        }
    }
}

/// A compare and swap operation on a vertree
///
/// All guards must succeed in order to run all ops
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TreeCas {
    pub guards: Vec<Guard>,
    pub ops: Vec<TreeOp>
}

/// The result of an individual `TreeOp` run against a vertree
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TreeOpResult {
    Ok(Option<Version>),
    Empty(Option<Version>),
    Bool(bool, Option<Version>),
    Blob(Vec<u8>, Option<Version>),
    Int(u64, Option<Version>),
    Set(Vec<Vec<u8>>, Option<Version>),
    Keys(Vec<(String, Version)>)
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ApiError {
    NotFound(String),
    AlreadyExists(String),
    DoesNotExist(String),
    WrongType(String, NodeType),
    PathMustEndInDirectory(String),
    PathMustBeAbsolute(String),
    CasFailed {path: String, expected: u64, actual: u64},
    BadFormat(String),
    Io(String),
    EncodingError(String),
    InvalidCas(String),
    Msg(String),
    CannotDeleteRoot,
    InvalidMsg,
    Timeout,
    NotEnoughReplicas,
    BadEpoch
}

// When the `TryFrom` trait is stabilized it should be used here
// We aren't using it because we want to keep haret on Rust stable
// https://github.com/rust-lang/rust/issues/33417
pub fn proto_to_tree_op(mut msg: protobuf::TreeOp) -> Result<TreeOp, ApiError> {
        if msg.has_create_node() {
            Ok(proto_to_vr_create_node(msg.take_create_node()))
        } else if msg.has_delete_node() {
            Ok(proto_to_vr_delete_node(msg.take_delete_node()))
        } else if msg.has_list_keys() {
            Ok(proto_to_vr_list_keys(msg.take_list_keys()))
        } else if msg.has_blob_put() {
            Ok(proto_to_vr_blob_put(msg.take_blob_put()))
        } else if msg.has_blob_get() {
            Ok(proto_to_vr_blob_get(msg.take_blob_get()))
        } else if msg.has_blob_size() {
            Ok(proto_to_vr_blob_size(msg.take_blob_size()))
        } else if msg.has_queue_push() {
            Ok(proto_to_vr_queue_push(msg.take_queue_push()))
        } else if msg.has_queue_pop() {
            Ok(proto_to_vr_queue_pop(msg.take_queue_pop()))
        } else if msg.has_queue_front() {
            Ok(proto_to_vr_queue_front(msg.take_queue_front()))
        } else if msg.has_queue_back() {
            Ok(proto_to_vr_queue_back(msg.take_queue_back()))
        } else if msg.has_queue_len() {
            Ok(proto_to_vr_queue_len(msg.take_queue_len()))
        } else if msg.has_set_insert() {
            Ok(proto_to_vr_set_insert(msg.take_set_insert()))
        } else if msg.has_set_remove() {
            Ok(proto_to_vr_set_remove(msg.take_set_remove()))
        } else if msg.has_set_contains() {
            Ok(proto_to_vr_set_contains(msg.take_set_contains()))
        } else if msg.has_set_union() {
            Ok(proto_to_vr_set_union(msg.take_set_union()))
        } else if msg.has_set_intersection() {
            Ok(proto_to_vr_set_intersection(msg.take_set_intersection()))
        } else if msg.has_set_difference() {
            Ok(proto_to_vr_set_difference(msg.take_set_difference()))
        } else if msg.has_set_symmetric_difference() {
            Ok(proto_to_vr_set_symmetric_difference(msg.take_set_symmetric_difference()))
        } else if msg.has_set_subset_path() {
            Ok(proto_to_vr_set_subset_path(msg.take_set_subset_path()))
        } else if msg.has_set_subset_set() {
            Ok(proto_to_vr_set_subset_set(msg.take_set_subset_set()))
        } else if msg.has_set_superset_path() {
            Ok(proto_to_vr_set_superset_path(msg.take_set_superset_path()))
        } else if msg.has_set_superset_set() {
            Ok(proto_to_vr_set_superset_set(msg.take_set_superset_set()))
        } else {
            Err(ApiError::InvalidMsg)
        }
}

fn proto_to_vr_create_node(mut msg: protobuf::CreateNode) -> TreeOp {
    let path = msg.take_path();
    let node_type = match msg.get_node_type() {
        protobuf::NodeType::BLOB => NodeType::Blob,
        protobuf::NodeType::QUEUE => NodeType::Queue,
        protobuf::NodeType::SET => NodeType::Set,
        protobuf::NodeType::DIRECTORY => NodeType::Directory
    };
    TreeOp::CreateNode{path: path, ty: node_type}
}

fn proto_to_vr_delete_node(mut msg: protobuf::DeleteNode) -> TreeOp {
    let path = msg.take_path();
    TreeOp::DeleteNode {path: path}
}

fn proto_to_vr_list_keys(mut msg: protobuf::ListKeys) -> TreeOp {
    let path = msg.take_path();
    TreeOp::ListKeys {path: path}
}

fn proto_to_vr_blob_put(mut msg: protobuf::BlobPut) -> TreeOp {
    let path = msg.take_path();
    let val = msg.take_val();
    TreeOp::BlobPut {path: path, val: val}
}

fn proto_to_vr_blob_get(mut msg: protobuf::BlobGet) -> TreeOp {
    let path = msg.take_path();
    TreeOp::BlobGet {path: path}
}

fn proto_to_vr_blob_size(mut msg: protobuf::BlobSize) -> TreeOp {
    let path = msg.take_path();
    TreeOp::BlobSize {path: path}
}

fn proto_to_vr_queue_push(mut msg: protobuf::QueuePush) -> TreeOp {
    let path = msg.take_path();
    let val = msg.take_val();
    TreeOp::QueuePush {path: path, val: val}
}

fn proto_to_vr_queue_pop(mut msg: protobuf::QueuePop) -> TreeOp {
    let path = msg.take_path();
    TreeOp::QueuePop {path: path}
}

fn proto_to_vr_queue_front(mut msg: protobuf::QueueFront) -> TreeOp {
    let path = msg.take_path();
    TreeOp::QueueFront {path: path}
}

fn proto_to_vr_queue_back(mut msg: protobuf::QueueBack) -> TreeOp {
    let path = msg.take_path();
    TreeOp::QueueBack {path: path}
}

fn proto_to_vr_queue_len(mut msg: protobuf::QueueLen) -> TreeOp {
    let path = msg.take_path();
    TreeOp::QueueLen{path: path}
}

fn proto_to_vr_set_insert(mut msg: protobuf::SetInsert) -> TreeOp {
    let path = msg.take_path();
    let val = msg.take_val();
    TreeOp::SetInsert {path: path, val: val}
}

fn proto_to_vr_set_remove(mut msg: protobuf::SetRemove) -> TreeOp {
    let path = msg.take_path();
    let val = msg.take_val();
    TreeOp::SetRemove {path: path, val: val}
}

fn proto_to_vr_set_contains(mut msg: protobuf::SetContains) -> TreeOp {
    let path = msg.take_path();
    let val = msg.take_val();
    TreeOp::SetContains {path: path, val: val}
}

fn proto_to_vr_set_union(mut msg: protobuf::SetUnion) -> TreeOp {
    let paths = msg.take_paths().into_vec();
    let sets: Vec<_> = msg.take_sets().into_iter().map(|mut s| {
        HashSet::from_iter(s.take_val().into_iter())
    }).collect();
    TreeOp::SetUnion {paths: paths, sets: sets}
}

fn proto_to_vr_set_intersection(mut msg: protobuf::SetIntersection) -> TreeOp {
    let path1 = msg.take_path1();
    let path2 = msg.take_path2();
    TreeOp::SetIntersection {path1: path1, path2: path2}
}

fn proto_to_vr_set_difference(mut msg: protobuf::SetDifference) -> TreeOp {
    let path1 = msg.take_path1();
    let path2 = msg.take_path2();
    TreeOp::SetDifference {path1: path1, path2: path2}
}

fn proto_to_vr_set_symmetric_difference(mut msg: protobuf::SetSymmetricDifference) -> TreeOp {
    let path1 = msg.take_path1();
    let path2 = msg.take_path2();
    TreeOp::SetSymmetricDifference {path1: path1, path2: path2}
}

fn proto_to_vr_set_subset_path(mut msg: protobuf::SetSubsetPath) -> TreeOp {
    let path1 = msg.take_path1();
    let path2 = msg.take_path2();
    TreeOp::SetSubsetPath {path1: path1, path2: path2}
}

fn proto_to_vr_set_subset_set(mut msg: protobuf::SetSubsetSet) -> TreeOp {
    let path = msg.take_path();
    let set = HashSet::from_iter(msg.take_set().take_val().into_iter());
    TreeOp::SetSubsetSet {path: path, set: set}
}

fn proto_to_vr_set_superset_path(mut msg: protobuf::SetSupersetPath) -> TreeOp {
    let path1 = msg.take_path1();
    let path2 = msg.take_path2();
    TreeOp::SetSupersetPath {path1: path1, path2: path2}
}

fn proto_to_vr_set_superset_set(mut msg: protobuf::SetSupersetSet) -> TreeOp {
    let path = msg.take_path();
    let set = HashSet::from_iter(msg.take_set().take_val().into_iter());
    TreeOp::SetSupersetSet {path: path, set: set}
}

impl From<vertree::Reply> for TreeOpResult {
    fn from(reply: vertree::Reply) -> TreeOpResult {
        let vertree::Reply {version, value, ..} = reply;
        match value {
            Value::Blob(vec) => TreeOpResult::Blob(vec, version),
            Value::Set(set) => TreeOpResult::Set(set.data.into_iter().collect(), version),
            Value::Int(i) => TreeOpResult::Int(i, version),
            Value::Bool(b) => TreeOpResult::Bool(b, version),
            Value::Empty => TreeOpResult::Empty(version),
            Value::None => TreeOpResult::Ok(version)
        }
    }
}

impl From<vertree::Reply> for ApiRsp {
    fn from(reply: vertree::Reply) -> ApiRsp {
        ApiRsp::TreeOpResult(reply.into())
    }
}

impl From<vertree::Error> for ApiRsp {
    fn from(error: vertree::Error) -> ApiRsp {
        let vertree::Error(kind, _) = error;
        let err = match kind {
            ErrorKind::AlreadyExists(path) => ApiError::AlreadyExists(path),
            ErrorKind::DoesNotExist(path) => ApiError::DoesNotExist(path),
            ErrorKind::WrongType(path, ty) => ApiError::WrongType(path, ty.into()),
            ErrorKind::InvalidPathContent(path) => ApiError::PathMustEndInDirectory(path),
            ErrorKind::CasFailed{path, expected, actual} =>
                ApiError::CasFailed {path: path, expected: expected, actual: actual},
            ErrorKind::BadPath(msg) => ApiError::BadFormat(msg),
            ErrorKind::Io(e) => ApiError::Io(e.to_string()),
            ErrorKind::MsgPackEncodeError(e) => ApiError::EncodingError(e.to_string()),
            ErrorKind::MsgPackDecodeError(e) => ApiError::EncodingError(e.to_string()),
            ErrorKind::CannotDeleteRoot => ApiError::CannotDeleteRoot,
            ErrorKind::PathMustBeAbsolute(path) => ApiError::PathMustBeAbsolute(path),
            ErrorKind::Msg(msg) => ApiError::Msg(msg)
        };
        ApiRsp::Error(err)
    }
}

impl From<TreeOp> for vertree::WriteOp {
    fn from(op: TreeOp) -> vertree::WriteOp {
        match op {
            TreeOp::Snapshot {directory} => vertree::WriteOp::Snapshot {directory: directory},
            TreeOp::CreateNode {path, ty} => vertree::WriteOp::CreateNode {path: path, ty: ty.into()},
            TreeOp::DeleteNode {path} => vertree::WriteOp::DeleteNode {path: path},
            TreeOp::BlobPut {path, val} => vertree::WriteOp::BlobPut {path: path, val: val},
            TreeOp::QueuePush {path, val} => vertree::WriteOp::QueuePush {path: path, val: val},
            TreeOp::QueuePop {path} => vertree::WriteOp::QueuePop {path: path},
            TreeOp::SetInsert {path, val} => vertree::WriteOp::SetInsert {path: path, val: val},
            TreeOp::SetRemove {path, val} => vertree::WriteOp::SetRemove {path: path, val: val},
            _ => unreachable!()
        }
    }
}

impl From<NodeType> for protobuf::NodeType {
    fn from(ty: NodeType) -> protobuf::NodeType {
        match ty {
            NodeType::Blob => protobuf::NodeType::BLOB,
            NodeType::Queue => protobuf::NodeType::QUEUE,
            NodeType::Set => protobuf::NodeType::SET,
            NodeType::Directory => protobuf::NodeType::DIRECTORY
        }
    }
}

impl From<TreeOpResult> for protobuf::TreeOpResult {
    fn from(result: TreeOpResult) -> protobuf::TreeOpResult {
        let (mut result, version) = match result {
            TreeOpResult::Ok(version) =>  {
                let mut result = protobuf::TreeOpResult::new();
                result.set_ok(true);
                (result, version)
            },
            TreeOpResult::Empty(version) =>  {
                let mut result = protobuf::TreeOpResult::new();
                result.set_empty(true);
                (result, version)
            },
            TreeOpResult::Bool(b, version) => {
                let mut result = protobuf::TreeOpResult::new();
                result.set_bool(b);
                (result, version)
            },
            TreeOpResult::Blob(blob, version) => {
                let mut result = protobuf::TreeOpResult::new();
                result.set_blob(blob);
                (result, version)
            },
            TreeOpResult::Int(i, version) => {
                let mut result = protobuf::TreeOpResult::new();
                result.set_int(i);
                (result, version)
            },
            TreeOpResult::Set(set, version) => {
                let val = RepeatedField::from_vec(set);
                let mut set = protobuf::Set::new();
                set.set_val(val);
                let mut result = protobuf::TreeOpResult::new();
                result.set_set(set);
                (result, version)
            },
            TreeOpResult::Keys(keys) => {
                let key_vec: Vec<_>  = keys.into_iter().map(|(name, version)| {
                    let mut key = protobuf::Key::new();
                    key.set_name(name);
                    key.set_version(version);
                    key
                }).collect();
                let mut keys = protobuf::Keys::new();
                keys.set_keys(RepeatedField::from_vec(key_vec));
                let mut result = protobuf::TreeOpResult::new();
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

pub fn vr_api_rsp_to_proto_api_response(mut reply: protobuf::ConsensusReply,
                                        value: ApiRsp) -> protobuf::ApiResponse
{
    match value {
        ApiRsp::Ok => reply.set_ok(true),
        ApiRsp::TreeOpResult(result) =>  reply.set_tree_op_result(result.into()),
        ApiRsp::TreeCasResult(results) => {
            let results: Vec<_> = results.into_iter().map(|op| op.into()).collect();
            let mut result = protobuf::TreeCasResult::new();
            result.set_results(RepeatedField::from_vec(results));
            reply.set_tree_cas_result(result);
        },
        ApiRsp::Path(path) => reply.set_path(path),
        ApiRsp::Error(error) => reply.set_error(error.into())
    }
    let mut response = protobuf::ApiResponse::new();
    response.set_consensus_reply(reply);
    response
}


impl From<Pid> for protobuf::ApiPid {
    fn from(pid: Pid) -> protobuf::ApiPid {
        let mut api_pid = protobuf::ApiPid::new();
        api_pid.set_name(pid.name);
        api_pid.set_node_name(pid.node.name);
        api_pid.set_node_addr(pid.node.addr);
        if pid.group.is_some() {
            api_pid.set_group(pid.group.unwrap());
        }
        api_pid
    }
}

