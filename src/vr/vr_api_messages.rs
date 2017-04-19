// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use rabble::{Pid, Error};
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
    fn from(self) -> pb_msg::NodeType {
        match self {
            NodeType::Blob => pb_msg::NodeType::BlobType,
            NodeType::Queue => pb_msg::NodeType::QueueType,
            NodeType::Set => pb_msg::NodeType::SetType,
            NodeType::Directory => pb_msg::NodeType::DirectoryType
        }
    }
}

impl From<pb_msg::NodeType> for NodeType {
    fn from(self) -> NodeType {
        match self {
            pb_msg::NodeType::BlobType => NodeType::Blob,
            pb_msg::NodeType::QueueType => NodeType::Queue,
            pb_msg::NodeType::SetType => NodeType::Set,
            pb_msg::NodeType::DirectoryType => NodeType::Directory
        }
    }
}

impl From<TreeOp> for pb_msg::TreeOp {
    fn from(self) -> pb_msg::TreeOp {
        let mut op = pb_msg::TreeOp::new();
        match self {
            TreeOp::Snapshot {directory} => op.set_snapshot(directory),
            TreeOp::CreateNode {path, ty} => {
                let mut msg = pb_msg::CreateNode::new();
                msg.set_path(path),
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
    fn try_from(self) -> Result<TreeOp> {
        if self.has_snapshot() {
            return Ok(TreeOp::Snapshot {
                directory: self.take_snapshot()
            });
        }
        if self.has_create_node() {
            let msg = self.take_create_node();
            return Ok(TreeOp::CreateNode {
                path: msg.take_path(),
                ty: msg.take_node_type().into()
            });
        }
        if self.has_delete_node() {
            return Ok(TreeOp::DeleteNode {
                path: self.take_delete_node()
            });
        }
        if self.has_list_keys() {
            return Ok(TreeOp::ListKeys {
                path: self.take_list_keys()
            });
        }
        if self.has_blob_put() {
            let msg = self.take_blob_put();
            return Ok(TreeOp::BlobPut {
                path: msg.take_path(),
                val: msg.take_val()
            });
        }
        if self.has_blob_get() {
            return Ok(TreeOp::BlobGet {
                path: self.take_path()
            });
        }
        if self.has_blob_size() {
            return Ok(TreeOp::BlobSize {
                path: self.take_path()
            });
        }
        if self.has_queue_push() {
            let msg = self.take_queue_push();
            return Ok(TreeOp::QueuePush {
                path: msg.take_path(),
                val: msg.take_val()
            });
        }
        if self.has_queue_pop() {
            return Ok(TreeOp::QueuePop {
                path: self.take_path()
            });
        }
        if self.has_queue_front() {
            return Ok(TreeOp::QueueFront {
                path: self.take_path()
            });
        }
        if self.has_queue_back() {
            return Ok(TreeOp::QueueBack {
                path: self.take_path()
            });
        }
        if self.has_queue_len() {
            return Ok(TreeOp::QueueLen {
                path: self.take_path()
            });
        }
        if self.has_set_insert() {
            let msg = self.take_set_insert();
            return Ok(TreeOp::SetInsert {
                path: msg.take_path(),
                val: msg.take_val()
            });
        }
        if self.has_set_remove() {
            let msg = self.take_set_remove();
            return Ok(TreeOp::SetRemove {
                path: msg.take_path(),
                val: msg.take_val()
            });
        }
        if self.has_set_contains() {
            let msg = self.take_set_contains();
            return Ok(TreeOp::SetContains {
                path: msg.take_path(),
                val: msg.take_val()
            });
        }
        if self.has_set_union() {
            let msg = self.take_set_union();
            return Ok(TreeOp::SetUnion {
                paths: self.take_paths().into_iter().collect(),
                sets: self.take_sets().into_iter().map(|s| s.into_iter().collect()).collect()
            });
        }
        if self.has_set_intersection() {
            let msg = self.take_set_intersection();
            return Ok(TreeOp::SetIntersection {
                path1: msg.take_path1(),
                path2: msg.take_path2()
            });
        }
        if self.has_set_difference() {
            let msg = self.take_set_difference();
            return Ok(TreeOp::SetDifference {
                path1: msg.take_path1(),
                path2: msg.take_path2()
            });
        }
        if self.has_set_symmetric_difference() {
            let msg = self.take_set_symmetric_difference();
            return Ok(TreeOp::SetSymmetricDifference {
                path1: msg.take_path1(),
                path2: msg.take_path2()
            });
        }
        if self.has_subset_path() {
            let msg = self.take_set_subset_path();
            return Ok(TreeOp::SetSubsetPath {
                path1: msg.take_path1(),
                path2: msg.take_path2()
            });
        }
        if self.has_subset_set() {
            let msg = self.take_set_subset_set();
            return Ok(TreeOp::SetSubsetSet {
                path: msg.take_path(),
                set: msg.take_set().into_iter().collect()
            });
        }
        if self.has_superset_path() {
            let msg = self.take_set_superset_path();
            return Ok(TreeOp::SetSupersetPath {
                path1: msg.take_path1(),
                path2: msg.take_path2()
            });
        }
        if self.has_superset_set() {
            let msg = self.take_set_superset_set();
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
    fn from(self) -> pb_msg::TreeCas {
        let mut pb_tree_cas = pb_msg::TreeCas::new();
        pb_tree_cas.set_guards(self.guards.into_iter().map(|g| g.into()).collect());
        pb_tree_cas.set_ops(self.ops.into_iter().map(|op| op.into()).collect());
        pb_tree_cas
    }
}

impl TryFrom<pb_msg::TreeCas> for TreeCas {
    type Error = Error;
    fn try_from(self) -> Result<TreeCas> {
        Ok(TreeCas {
            guards: self.take_guards().map(|g| g.into()).collect(),
            ops: self.take_ops().map(|op| op.try_into()?).collect()
        })
    }
}

impl From<VrApiReq> for pb_msg::VrApiReq {
    fn from(self) -> pb_msg::VrApiReq {
        let mut req = pb_msg::VrApiReq::new();
        match self {
            VrApiReq::TreeOp(op) => req.set_tree_op(op.into()),
            VrApiReq::TreeCas(cas) => req.set_tree_cas(cas.into())
        }
        req
    }
}

impl TryFrom<pb_msg::VrApiReq> for VrApiReq {
    fn try_from(self) -> Result<VrApiReq> {
        type Error = Error;
        if self.has_tree_op() {
            return Ok(VrApiReq::TreeOp(self.take_tree_op().try_into()?));
        }
        if self.has_tree_cas() {
            return Ok(VrApiReq::TreeCas(self.take_tree_cas().try_into()?));
        }
        Err(ErrorKind::ProtobufDecodeError("Unknown VrApiReq received").into())
    }
}

impl From<TreeOpResult> for pb_msg::TreeOpResult {
    fn from(self) -> pb_msg::TreeOpResult {
        let mut res = pb_msg::TreeOpResult::new();
        match self {
            TreeOpResult::Ok(Some(version)) => {
                res.set_version(version);
                res.set_ok(true);
            },
            TreeOpResult::Ok(None) => res.set_ok(true),
            TreeOpResult::Empty(Some(version)) => {
                res.set_version(version);
                res.set_empty(true);
            },
            TreeOpResult::Empty(None) => res.set_ok(true),
            TreeOpResult::Bool(b, Some(version)) => {
                res.set_version(version);
                res.set_bool(b);
            },
            TreeOpResult::Bool(b, None) => res.set_bool(b),
            TreeOpResult::Blob(b, Some(version)) => {
                res.set_version(version);
                res.set_blob(b);
            },
            TreeOpResult::Blob(b, None) => res.set_blob(b),
            TreeOpResult::Int(i, Some(version)) => {
                res.set_version(version);
                res.set_int(i);
            },
            TreeOpResult::Int(i, None) => res.set_int(i),
            TreeOpResult::Set(s, Some(version)) => {
                res.set_version(version);
                res.set_set(s.into());
            },
            TreeOpResult::Set(i, None) => res.set_set(s.into()),
            TreeOpResult::Keys(keys) => {
                res.set_keys(keys.into())
            }
        }
        res
    }
}

impl TryFrom<pb_msg::TreeOpResult> for TreeOpResult {
    fn try_from(self) -> Result<TreeOpResult> {
        let version = if self.has_version() {
            Some(self.get_version())
        } else {
            None
        };
        if self.has_ok() {
            return Ok(TreeOpResult::Ok(version));
        }
        if self.has_empty() {
            return Ok(TreeOpResult::Empty(version));
        }
        if self.has_bool() {
            return Ok(TreeOpResult::Bool(self.get_bool(), version));
        }
        if self.has_blob() {
            return Ok(TreeOpResult::Blob(self.take_blob(), version));
        }
        if self.has_int() {
            return Ok(TreeOpResult::Int(self.get_int(), version));
        }
        if self.has_set() {
            return Ok(TreeOpResult::Set(self.take_set().into(), version));
        }
        if self.has_keys() {
            return Ok(TreeOpResult::Keys(self.take_keys().into()));
        }
        Err(ErrorKind::ProtobufDecodeError("Unknown TreeOpResult received").into())
    }
}

impl From<Vec<Vec<u8>>> for pb_msg::Set {
    fn from(self) -> pb_msg::Set {
        let mut set = pb_msg::Set::new();
        set.set_values(self);
        set
    }
}

impl From<pb_msg::Set> for Vec<Vec<u8>> {
    fn from(self) -> Vec<Vec<u8>> {
        self.take_values().collect()
    }
}

impl From<Vec<(String, Version)>> for pb_msg::Keys {
    fn from(self) -> pb_msg::Keys {
        let mut keys = pb_msg::Keys::new();
        keys.set_keys(self.into_iter().map(|s, v| {
            let mut entry = pb_msg::Keys_KeysEntry::new();
            entry.set_key(s);
            entry.set_value(v);
            entry
        }).collect());
        keys
    }
}

impl From<pb_msg::Keys> for Vec<(String, Version)> {
    fn from(self) -> pb_msg::Keys {
        self.take_keys().into_iter().map(|entry| {
            (entry.take_key(), entry.get_version())
        }).collect()
    }
}

impl From<VrApiRsp> for pb_msg::VrApiRsp {
    fn from(self) -> pb_msg::VrApiRsp {
        let mut rsp = pb_msg::VrApiRsp::new();
        match self {
            VrApiRsp::Ok => rsp.set_ok(true),
            VrApiRsp::TreeOpResult(result) => rsp.set_tree_op_result(result.into()),
            VrApiRsp::TreeCasResult(results) =>
                rsp.set_tree_cas_result(results.into_iter().map(|r| r.into()).collect()),
            VrApiRsp::Path(s) => rsp.set_path(s),
            VrApiRsp::Error(e) => rsp.set_error(e.into())
        }
        rsp
    }
}

impl TryFrom<pb_msg::VrApiRsp> for VrApiRsp {
    type Error = Error;
    fn try_from(self) -> Result<VrApiRsp> {
        if self.has_ok() {
            return Ok(VrApiRsp::Ok);
        }
        if self.has_tree_op_result() {
            return Ok(VrApiRsp::TreeOpResult(self.take_tree_op_result().try_into()?));
        }
        if self.has_tree_cas_result() {
            return Ok(VrApiRsp::TreeCasResult(self.take_tree_cas_result().take_result().map(|r| {
                r.try_into()?
            })));
        }
        if self.has_path() {
            return Ok(VrApiRsp::Path(self.take_path()));
        }
        if self.has_error() {
            return Ok(VrApiRsp::Error(self.take_error().try_into()?));
        }

        Err(ErrorKind::ProtobufDecodeError("Unknown VrApiRsp").into())
    }
}

impl From<VrApiError> for pb_msg::VrApiError {
    fn from(self) -> pb_msg::VrApiError {
        let mut rsp = pb_msg::VrApiError::new();
        match self {
            VrApiError::NotFound(s) => rsp.set_not_found(s),
            VrApiError::AlreadyExists(s) => rsp.set_already_exists(s),
            VrApiError::DoesNotExist(s) => rsp.set_does_not_exist(s),
            VrApiError::WrongType(s, ty) => {
                let mut wrong_type = pb_msg::WrongType::new();
                wrong_type.set_path(s);
                wrong_type.set_node_type(ty);
                rsp.set_wrong_type(wrong_type);
            },
            VrApiError::PathMustEndInDirectory(s) => rsp.set_path_must_end_in_directory(s),
            VrApiError::PathMustBeAbsolute(s) => rsp.set_path_must_be_absolute(s),
            VrApiError::CasFailed {path, expected, actual} => {
                let mut cas_failed = pb_msg::CasFailed::new();
                cas_failed.set_path(path);
                cas_failed.set_expected(expected);
                cas_failed.set_actual(actual);
                rsp.set_cas_failed(cas_failed);
            },
            VrApiError::BadFormat(s) => rsp.set_bad_format(s),
            VrApiError::Io(s) => rsp.set_io(s),
            VrApiError::EncodingError(s) => rsp.set_encoding_error(s),
            VrApiError::InvalidCas(s) => rsp.set_invalid_cas(s),
            VrApiError::Msg(s) => rsp.set_msg(s),
            VrApiError::CannotDeleteRoot => rsp.set_cannot_delete_root(true),
            VrApiError::InvalidMsg => rsp.set_invalid_msg(true),
            VrApiError::Timeout => rsp.set_timeout(true),
            VrApiError::NotEnoughReplicas => rsp.set_not_enough_replicas(true),
            VrApiError::BadEpoch => rsp.set_bad_epoch(true)
        }
    }
}

impl TryFrom<pb_msg::VrApiError> for VrApiError {
    type Error = Error;
    fn try_from(self) -> Result<VrApiError> {
        if self.has_not_found() {
            return Ok(VrApiError::NotFound(self.take_not_found()));
        }
        if self.has_already_exists() {
            return Ok(VrApiError::AlreadyExists(self.take_already_exists()));
        }
        if self.has_does_not_exist() {
            return Ok(VrApiError::DoesNotExist(self.does_not_exist()));
        }
        if self.has_wrong_type() {
            let msg = self.take_wrong_type();
            return Ok(VrApiError::WrongType(msg.take_path(), msg.take_node_type().into()));
        }
        if self.has_path_must_end_in_directory() {
            return Ok(VrApiError::PathMustEndInDirectory(self.take_path_must_end_in_directory()));
        }
        if self.has_path_must_be_absolute() {
            return Ok(VrApiError::PathMustBeAbsolute(self.take_path_must_be_absolute()));
        }
        if self.has_cas_failed() {
            let msg = self.take_cas_failed();
            return Ok(VrApiError::CasFailed {
                path: msg.take_path(),
                expected: msg.get_expected(),
                actual: msg.get_actual()
            });
        }
        if self.has_bad_format() {
            return Ok(VrApiError::BadFormat(self.take_bad_format()));
        }
        if self.has_io() {
            return Ok(VrApiError::Io(self.take_io()));
        }
        if self.has_encoding_error() {
            return Ok(VrApiError::EncodingError(self.take_encoding_error()));
        }
        if self.has_invalid_cas() {
            return Ok(VrApiError::InvalidCas(self.take_invalid_cas()));
        }
        if self.has_msg() {
            return Ok(VrApiError::Msg(self.take_msg()));
        }
        if self.has_cannot_delete_root() {
            return Ok(VrApiError::CannotDeleteRoot);
        }
        if self.has_invalid_msg() {
            return Ok(VrApiError::InvalidMsg);
        }
        if self.has_timeout() {
            return Ok(VrApiError::Timeout);
        }
        if self.has_not_enough_replicas() {
            return Ok(VrApiError::NotEnoughReplicas);
        }
        if self.has_bad_epoch() {
            return Ok(VrApiError::BadEpoch);
        }

        Err(ErrorKind::ProtobufDecodeError("Unknown VrApiError").into())
    }
}
