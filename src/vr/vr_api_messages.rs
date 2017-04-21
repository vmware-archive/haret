// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::convert::TryFrom;
use std::collections::HashSet;
use rabble::{Pid, Error, ErrorKind, Result};
use pb_msg;

type Version = u64;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeType {
    Blob,
    Queue,
    Set,
    Directory
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VrApiReq {
    TreeOp(TreeOp),
    TreeCas(TreeCas),
}

#[derive(Debug, Clone, Eq, PartialEq)]
// This should maintain the identical format to the vertree version. The only 
// difference is that the haret one is encodable and decodable
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
            TreeOp::Snapshot {..} => true,
            TreeOp::CreateNode {..} => true,
            TreeOp::DeleteNode {..} => true,
            TreeOp::BlobPut {..} => true,
            TreeOp::QueuePush {..} => true,
            TreeOp::QueuePop {..} => true,
            TreeOp::SetInsert {..} => true,
            TreeOp::SetRemove {..} => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Guard {
    pub path: String,
    pub version: u64
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TreeCas {
    pub guards: Vec<Guard>,
    pub ops: Vec<TreeOp>
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VrApiRsp {
    Ok,
    TreeOpResult(TreeOpResult),
    TreeCasResult(Vec<TreeOpResult>),
    Path(String),
    Error(VrApiError)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TreeOpResult {
    Ok(Option<Version>),
    Empty(Option<Version>),
    Bool(bool, Option<Version>),
    Blob(Vec<u8>, Option<Version>),
    Int(u64, Option<Version>),
    Set(Vec<Vec<u8>>, Option<Version>),
    Keys(Vec<(String, Version)>)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VrApiError {
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

impl From<NodeType> for pb_msg::NodeType {
    fn from(ty: NodeType) -> pb_msg::NodeType {
        match ty {
            NodeType::Blob => pb_msg::NodeType::BlobType,
            NodeType::Queue => pb_msg::NodeType::QueueType,
            NodeType::Set => pb_msg::NodeType::SetType,
            NodeType::Directory => pb_msg::NodeType::DirectoryType
        }
    }
}

impl From<pb_msg::NodeType> for NodeType {
    fn from(msg: pb_msg::NodeType) -> NodeType {
        match msg {
            pb_msg::NodeType::BlobType => NodeType::Blob,
            pb_msg::NodeType::QueueType => NodeType::Queue,
            pb_msg::NodeType::SetType => NodeType::Set,
            pb_msg::NodeType::DirectoryType => NodeType::Directory
        }
    }
}

impl From<TreeOp> for pb_msg::TreeOp {
    fn from(tree_op: TreeOp) -> pb_msg::TreeOp {
        let mut op = pb_msg::TreeOp::new();
        match tree_op {
            TreeOp::Snapshot {directory} => op.set_snapshot(directory),
            TreeOp::CreateNode {path, ty} => {
                let mut msg = pb_msg::CreateNode::new();
                msg.set_path(path);
                msg.set_node_type(ty.into());
                op.set_create_node(msg)
            },
            TreeOp::DeleteNode {path} => op.set_delete_node(path),
            TreeOp::ListKeys {path} => op.set_list_keys(path),
            TreeOp::BlobPut {path, val} => {
                let mut msg = pb_msg::BlobPut::new();
                msg.set_path(path);
                msg.set_val(val);
                op.set_blob_put(msg);
            },
            TreeOp::BlobGet {path} => op.set_blob_get(path),
            TreeOp::BlobSize {path} => op.set_blob_size(path),
            TreeOp::QueuePush {path, val} => {
                let mut msg = pb_msg::QueuePush::new();
                msg.set_path(path);
                msg.set_val(val);
                op.set_queue_push(msg);
            },
            TreeOp::QueuePop {path} => op.set_queue_pop(path),
            TreeOp::QueueFront {path} => op.set_queue_front(path),
            TreeOp::QueueBack {path} => op.set_queue_back(path),
            TreeOp::QueueLen {path} => op.set_queue_len(path),
            TreeOp::SetInsert {path, val} => {
                let mut msg = pb_msg::SetInsert::new();
                msg.set_path(path);
                msg.set_val(val);
                op.set_set_insert(msg);
            },
            TreeOp::SetRemove {path, val} => {
                let mut msg = pb_msg::SetRemove::new();
                msg.set_path(path);
                msg.set_val(val);
                op.set_set_remove(msg);
            },
            TreeOp::SetContains {path, val} => {
                let mut msg = pb_msg::SetContains::new();
                msg.set_path(path);
                msg.set_val(val);
                op.set_set_contains(msg);
            },
            TreeOp::SetUnion {paths, sets} => {
                let mut msg = pb_msg::SetUnion::new();
                msg.set_paths(paths);
                msg.set_sets(sets.into_iter().map(|s| s.into_iter().collect()).collect());
                op.set_set_union(msg);
            },
            TreeOp::SetIntersection {path1, path2} => {
                let mut msg = pb_msg::SetIntersection::new();
                msg.set_path1(path1);
                msg.set_path2(path2);
                op.set_set_intersection(msg);
            },
            TreeOp::SetDifference {path1, path2} => {
                let mut msg = pb_msg::SetDifference::new();
                msg.set_path1(path1);
                msg.set_path2(path2);
                op.set_set_difference(msg);
            },
            TreeOp::SetSymmetricDifference {path1, path2} => {
                let mut msg = pb_msg::SetSymmetricDifference::new();
                msg.set_path1(path1);
                msg.set_path2(path2);
                op.set_set_symmetric_difference(msg);
            },
            TreeOp::SetSubsetPath {path1, path2} => {
                let mut msg = pb_msg::SetSubsetPath::new();
                msg.set_path1(path1);
                msg.set_path2(path2);
                op.set_set_subset_path(msg);
            },
            TreeOp::SetSubsetSet {path, set} => {
                let mut msg = pb_msg::SetSubsetSet::new();
                msg.set_path(path);
                msg.set_set(set.into_iter().collect());
                op.set_set_subset_set(msg);
            },
            TreeOp::SetSupersetPath {path1, path2} => {
                let mut msg = pb_msg::SetSupersetPath::new();
                msg.set_path1(path1);
                msg.set_path2(path2);
                op.set_set_superset_path(msg);
            },
            TreeOp::SetSupersetSet{path, set} => {
                let mut msg = pb_msg::SetSupersetSet::new();
                msg.set_path(path);
                msg.set_set(set.into_iter().collect());
                op.set_set_superset_set(msg);
            }
        }
        op
    }
}

impl TryFrom<pb_msg::TreeOp> for TreeOp {
    type Error = Error;
    fn try_from(pb_op: pb_msg::TreeOp) -> Result<TreeOp> {
        if pb_op.has_snapshot() {
            return Ok(TreeOp::Snapshot {
                directory: pb_op.take_snapshot()
            });
        }
        if pb_op.has_create_node() {
            let msg = pb_op.take_create_node();
            return Ok(TreeOp::CreateNode {
                path: msg.take_path(),
                ty: msg.take_node_type().into()
            });
        }
        if pb_op.has_delete_node() {
            return Ok(TreeOp::DeleteNode {
                path: pb_op.take_delete_node()
            });
        }
        if pb_op.has_list_keys() {
            return Ok(TreeOp::ListKeys {
                path: pb_op.take_list_keys()
            });
        }
        if pb_op.has_blob_put() {
            let msg = pb_op.take_blob_put();
            return Ok(TreeOp::BlobPut {
                path: msg.take_path(),
                val: msg.take_val()
            });
        }
        if pb_op.has_blob_get() {
            return Ok(TreeOp::BlobGet {
                path: pb_op.take_path()
            });
        }
        if pb_op.has_blob_size() {
            return Ok(TreeOp::BlobSize {
                path: pb_op.take_path()
            });
        }
        if pb_op.has_queue_push() {
            let msg = pb_op.take_queue_push();
            return Ok(TreeOp::QueuePush {
                path: msg.take_path(),
                val: msg.take_val()
            });
        }
        if pb_op.has_queue_pop() {
            return Ok(TreeOp::QueuePop {
                path: pb_op.take_path()
            });
        }
        if pb_op.has_queue_front() {
            return Ok(TreeOp::QueueFront {
                path: pb_op.take_path()
            });
        }
        if pb_op.has_queue_back() {
            return Ok(TreeOp::QueueBack {
                path: pb_op.take_path()
            });
        }
        if pb_op.has_queue_len() {
            return Ok(TreeOp::QueueLen {
                path: pb_op.take_path()
            });
        }
        if pb_op.has_set_insert() {
            let msg = pb_op.take_set_insert();
            return Ok(TreeOp::SetInsert {
                path: msg.take_path(),
                val: msg.take_val()
            });
        }
        if pb_op.has_set_remove() {
            let msg = pb_op.take_set_remove();
            return Ok(TreeOp::SetRemove {
                path: msg.take_path(),
                val: msg.take_val()
            });
        }
        if pb_op.has_set_contains() {
            let msg = pb_op.take_set_contains();
            return Ok(TreeOp::SetContains {
                path: msg.take_path(),
                val: msg.take_val()
            });
        }
        if pb_op.has_set_union() {
            let msg = pb_op.take_set_union();
            return Ok(TreeOp::SetUnion {
                paths: pb_op.take_paths().into_iter().collect(),
                sets: pb_op.take_sets().into_iter().map(|s| s.into_iter().collect()).collect()
            });
        }
        if pb_op.has_set_intersection() {
            let msg = pb_op.take_set_intersection();
            return Ok(TreeOp::SetIntersection {
                path1: msg.take_path1(),
                path2: msg.take_path2()
            });
        }
        if pb_op.has_set_difference() {
            let msg = pb_op.take_set_difference();
            return Ok(TreeOp::SetDifference {
                path1: msg.take_path1(),
                path2: msg.take_path2()
            });
        }
        if pb_op.has_set_symmetric_difference() {
            let msg = pb_op.take_set_symmetric_difference();
            return Ok(TreeOp::SetSymmetricDifference {
                path1: msg.take_path1(),
                path2: msg.take_path2()
            });
        }
        if pb_op.has_subset_path() {
            let msg = pb_op.take_set_subset_path();
            return Ok(TreeOp::SetSubsetPath {
                path1: msg.take_path1(),
                path2: msg.take_path2()
            });
        }
        if pb_op.has_subset_set() {
            let msg = pb_op.take_set_subset_set();
            return Ok(TreeOp::SetSubsetSet {
                path: msg.take_path(),
                set: msg.take_set().into_iter().collect()
            });
        }
        if pb_op.has_superset_path() {
            let msg = pb_op.take_set_superset_path();
            return Ok(TreeOp::SetSupersetPath {
                path1: msg.take_path1(),
                path2: msg.take_path2()
            });
        }
        if pb_op.has_superset_set() {
            let msg = pb_op.take_set_superset_set();
            return Ok(TreeOp::SetSupersetSet {
                path: msg.take_path(),
                set: msg.take_set().into_iter().collect()
            });
        }

        Err(ErrorKind::ProtobufDecodeError("Unknown TreeOp received").into())
    }
}

impl From<Guard> for pb_msg::Guard {
    fn from(guard: Guard) -> pb_msg::Guard {
        let mut pb_guard = pb_msg::Guard::new();
        pb_guard.set_path(guard.path);
        pb_guard.set_version(guard.version);
        pb_guard
    }
}

impl From<pb_msg::Guard> for Guard {
    fn from(pb_guard: pb_msg::Guard) -> Guard {
        Guard {
            path: pb_guard.take_path(),
            version: pb_guard.get_version()
        }
    }
}

impl From<TreeCas> for pb_msg::TreeCas {
    fn from(cas: TreeCas) -> pb_msg::TreeCas {
        let mut pb_tree_cas = pb_msg::TreeCas::new();
        pb_tree_cas.set_guards(cas.guards.into_iter().map(|g| g.into()).collect());
        pb_tree_cas.set_ops(cas.ops.into_iter().map(|op| op.into()).collect());
        pb_tree_cas
    }
}

impl TryFrom<pb_msg::TreeCas> for TreeCas {
    type Error = Error;
    fn try_from(msg: pb_msg::TreeCas) -> Result<TreeCas> {
        Ok(TreeCas {
            guards: msg.take_guards().map(|g| g.into()).collect(),
            ops: msg.take_ops().map(|op| op.try_into()?).collect()
        })
    }
}

impl From<VrApiReq> for pb_msg::VrApiReq {
    fn from(req: VrApiReq) -> pb_msg::VrApiReq {
        let mut msg = pb_msg::VrApiReq::new();
        match req {
            VrApiReq::TreeOp(op) => msg.set_tree_op(op.into()),
            VrApiReq::TreeCas(cas) => msg.set_tree_cas(cas.into())
        }
        req
    }
}

impl TryFrom<pb_msg::VrApiReq> for VrApiReq {
    type Error = Error;
    fn try_from(msg: pb_msg::VrApiReq) -> Result<VrApiReq> {
        if msg.has_tree_op() {
            return Ok(VrApiReq::TreeOp(msg.take_tree_op().try_into()?));
        }
        if msg.has_tree_cas() {
            return Ok(VrApiReq::TreeCas(msg.take_tree_cas().try_into()?));
        }
        Err(ErrorKind::ProtobufDecodeError("Unknown VrApiReq received").into())
    }
}

impl From<TreeOpResult> for pb_msg::TreeOpResult {
    fn from(res: TreeOpResult) -> pb_msg::TreeOpResult {
        let mut msg = pb_msg::TreeOpResult::new();
        match res {
            TreeOpResult::Ok(Some(version)) => {
                msg.set_version(version);
                msg.set_ok(true);
            },
            TreeOpResult::Ok(None) => msg.set_ok(true),
            TreeOpResult::Empty(Some(version)) => {
                msg.set_version(version);
                msg.set_empty(true);
            },
            TreeOpResult::Empty(None) => msg.set_ok(true),
            TreeOpResult::Bool(b, Some(version)) => {
                msg.set_version(version);
                msg.set_bool(b);
            },
            TreeOpResult::Bool(b, None) => msg.set_bool(b),
            TreeOpResult::Blob(b, Some(version)) => {
                msg.set_version(version);
                msg.set_blob(b);
            },
            TreeOpResult::Blob(b, None) => msg.set_blob(b),
            TreeOpResult::Int(i, Some(version)) => {
                msg.set_version(version);
                msg.set_int(i);
            },
            TreeOpResult::Int(i, None) => msg.set_int(i),
            TreeOpResult::Set(s, Some(version)) => {
                msg.set_version(version);
                msg.set_set(s.into());
            },
            TreeOpResult::Set(s, None) => msg.set_set(s.into()),
            TreeOpResult::Keys(keys) => {
                msg.set_keys(keys.into())
            }
        }
        msg 
    }
}

impl TryFrom<pb_msg::TreeOpResult> for TreeOpResult {
    type Error = Error;
    fn try_from(msg: pb_msg::TreeOpResult) -> Result<TreeOpResult> {
        let version = if msg.has_version() {
            Some(msg.get_version())
        } else {
            None
        };
        if msg.has_ok() {
            return Ok(TreeOpResult::Ok(version));
        }
        if msg.has_empty() {
            return Ok(TreeOpResult::Empty(version));
        }
        if msg.has_bool() {
            return Ok(TreeOpResult::Bool(msg.get_bool(), version));
        }
        if msg.has_blob() {
            return Ok(TreeOpResult::Blob(msg.take_blob(), version));
        }
        if msg.has_int() {
            return Ok(TreeOpResult::Int(msg.get_int(), version));
        }
        if msg.has_set() {
            return Ok(TreeOpResult::Set(msg.take_set().into(), version));
        }
        if msg.has_keys() {
            return Ok(TreeOpResult::Keys(msg.take_keys().into()));
        }
        Err(ErrorKind::ProtobufDecodeError("Unknown TreeOpResult received").into())
    }
}

impl From<Vec<Vec<u8>>> for pb_msg::Set {
    fn from(set: Vec<Vec<u8>>) -> pb_msg::Set {
        let mut msg = pb_msg::Set::new();
        msg.set_values(set);
        msg 
    }
}

impl From<pb_msg::Set> for Vec<Vec<u8>> {
    fn from(msg: pb_msg::Set) -> Vec<Vec<u8>> {
        msg.take_values().collect()
    }
}

impl From<Vec<(String, Version)>> for pb_msg::Keys {
    fn from(keys: Vec<(String, Version)>) -> pb_msg::Keys {
        let mut msg = pb_msg::Keys::new();
        msg.set_keys(keys.into_iter().map(|s, v| {
            let mut entry = pb_msg::Keys_KeysEntry::new();
            entry.set_key(s);
            entry.set_value(v);
            entry
        }).collect());
       msg 
    }
}

impl From<pb_msg::Keys> for Vec<(String, Version)> {
    fn from(msg: pb_msg::Keys) -> Vec<(String, Version)> {
        msg.take_keys().into_iter().map(|entry| {
            (entry.take_key(), entry.get_version())
        }).collect()
    }
}

impl From<VrApiRsp> for pb_msg::VrApiRsp {
    fn from(rsp: VrApiRsp) -> pb_msg::VrApiRsp {
        let mut msg = pb_msg::VrApiRsp::new();
        match rsp {
            VrApiRsp::Ok => msg.set_ok(true),
            VrApiRsp::TreeOpResult(result) => msg.set_tree_op_result(result.into()),
            VrApiRsp::TreeCasResult(results) =>
                msg.set_tree_cas_result(results.into_iter().map(|r| r.into()).collect()),
            VrApiRsp::Path(s) => msg.set_path(s),
            VrApiRsp::Error(e) => msg.set_error(e.into())
        }
        rsp
    }
}

impl TryFrom<pb_msg::VrApiRsp> for VrApiRsp {
    type Error = Error;
    fn try_from(msg: pb_msg::VrApiRsp) -> Result<VrApiRsp> {
        if msg.has_ok() {
            return Ok(VrApiRsp::Ok);
        }
        if msg.has_tree_op_result() {
            return Ok(VrApiRsp::TreeOpResult(msg.take_tree_op_result().try_into()?));
        }
        if msg.has_tree_cas_result() {
            return Ok(VrApiRsp::TreeCasResult(msg.take_tree_cas_result().take_result().map(|r| {
                r.try_into()?
            })));
        }
        if msg.has_path() {
            return Ok(VrApiRsp::Path(msg.take_path()));
        }
        if msg.has_error() {
            return Ok(VrApiRsp::Error(msg.take_error().try_into()?));
        }

        Err(ErrorKind::ProtobufDecodeError("Unknown VrApiRsp").into())
    }
}

impl From<VrApiError> for pb_msg::VrApiError {
    fn from(error: VrApiError) -> pb_msg::VrApiError {
        let mut msg = pb_msg::VrApiError::new();
        match error {
            VrApiError::NotFound(s) => msg.set_not_found(s),
            VrApiError::AlreadyExists(s) => msg.set_already_exists(s),
            VrApiError::DoesNotExist(s) => msg.set_does_not_exist(s),
            VrApiError::WrongType(s, ty) => {
                let mut wrong_type = pb_msg::WrongType::new();
                wrong_type.set_path(s);
                wrong_type.set_node_type(ty);
                msg.set_wrong_type(wrong_type);
            },
            VrApiError::PathMustEndInDirectory(s) => msg.set_path_must_end_in_directory(s),
            VrApiError::PathMustBeAbsolute(s) => msg.set_path_must_be_absolute(s),
            VrApiError::CasFailed {path, expected, actual} => {
                let mut cas_failed = pb_msg::CasFailed::new();
                cas_failed.set_path(path);
                cas_failed.set_expected(expected);
                cas_failed.set_actual(actual);
                msg.set_cas_failed(cas_failed);
            },
            VrApiError::BadFormat(s) => msg.set_bad_format(s),
            VrApiError::Io(s) => msg.set_io(s),
            VrApiError::EncodingError(s) => msg.set_encoding_error(s),
            VrApiError::InvalidCas(s) => msg.set_invalid_cas(s),
            VrApiError::Msg(s) => msg.set_msg(s),
            VrApiError::CannotDeleteRoot => msg.set_cannot_delete_root(true),
            VrApiError::InvalidMsg => msg.set_invalid_msg(true),
            VrApiError::Timeout => msg.set_timeout(true),
            VrApiError::NotEnoughReplicas => msg.set_not_enough_replicas(true),
            VrApiError::BadEpoch => msg.set_bad_epoch(true)
        }
    }
}

impl TryFrom<pb_msg::VrApiError> for VrApiError {
    type Error = Error;
    fn try_from(msg: pb_msg::VrApiError) -> Result<VrApiError> {
        if msg.has_not_found() {
            return Ok(VrApiError::NotFound(msg.take_not_found()));
        }
        if msg.has_already_exists() {
            return Ok(VrApiError::AlreadyExists(msg.take_already_exists()));
        }
        if msg.has_does_not_exist() {
            return Ok(VrApiError::DoesNotExist(msg.does_not_exist()));
        }
        if msg.has_wrong_type() {
            let msg = msg.take_wrong_type();
            return Ok(VrApiError::WrongType(msg.take_path(), msg.take_node_type().into()));
        }
        if msg.has_path_must_end_in_directory() {
            return Ok(VrApiError::PathMustEndInDirectory(msg.take_path_must_end_in_directory()));
        }
        if msg.has_path_must_be_absolute() {
            return Ok(VrApiError::PathMustBeAbsolute(msg.take_path_must_be_absolute()));
        }
        if msg.has_cas_failed() {
            let msg = msg.take_cas_failed();
            return Ok(VrApiError::CasFailed {
                path: msg.take_path(),
                expected: msg.get_expected(),
                actual: msg.get_actual()
            });
        }
        if msg.has_bad_format() {
            return Ok(VrApiError::BadFormat(msg.take_bad_format()));
        }
        if msg.has_io() {
            return Ok(VrApiError::Io(msg.take_io()));
        }
        if msg.has_encoding_error() {
            return Ok(VrApiError::EncodingError(msg.take_encoding_error()));
        }
        if msg.has_invalid_cas() {
            return Ok(VrApiError::InvalidCas(msg.take_invalid_cas()));
        }
        if msg.has_msg() {
            return Ok(VrApiError::Msg(msg.take_msg()));
        }
        if msg.has_cannot_delete_root() {
            return Ok(VrApiError::CannotDeleteRoot);
        }
        if msg.has_invalid_msg() {
            return Ok(VrApiError::InvalidMsg);
        }
        if msg.has_timeout() {
            return Ok(VrApiError::Timeout);
        }
        if msg.has_not_enough_replicas() {
            return Ok(VrApiError::NotEnoughReplicas);
        }
        if msg.has_bad_epoch() {
            return Ok(VrApiError::BadEpoch);
        }

        Err(ErrorKind::ProtobufDecodeError("Unknown VrApiError").into())
    }
}
