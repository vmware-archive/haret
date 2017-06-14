// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::convert::From;
use super::vr_api_messages::{VrApiReq, VrApiRsp, VrApiError, TreeOp, TreeCas, TreeOpResult};
use super::vr_api_messages::{Guard, NodeType};
use vertree::{self, Tree, Value, ErrorKind};

#[derive(Debug, Clone)]
pub struct VrBackend {
    pub tree: Tree
}

impl Default for VrBackend {
    fn default() -> VrBackend {
        VrBackend::new()
    }
}

impl VrBackend {
    pub fn new() -> VrBackend {
        VrBackend {
            tree: Tree::new()
        }
    }

    pub fn call(&mut self, msg: VrApiReq) -> VrApiRsp {
        match msg {
            VrApiReq::TreeOp(tree_op) => {
                match self.run_tree_op(tree_op) {
                    Ok(vr_api_rsp) => vr_api_rsp,
                    Err(vertree_error) => vertree_error.into()
                }
            },
            VrApiReq::TreeCas(tree_cas) => {
                match self.run_tree_cas(tree_cas) {
                    Ok(vr_api_rsp) => vr_api_rsp,
                    Err(vertree_error) => vertree_error.into()
                }
            }
        }
    }

    fn run_tree_op(&mut self, tree_op: TreeOp) -> Result<VrApiRsp, vertree::Error> {
        let val = match tree_op {
            TreeOp::Snapshot {directory} => {
                let s = self.tree.snapshot(&directory)?;
                VrApiRsp::Path(s)
            },
            TreeOp::CreateNode {path, ty} => {
                self.tree = self.tree.create(&path, ty.into())?;
                VrApiRsp::Ok
            },
            TreeOp::DeleteNode {path} => {
                let (_, tree) = self.tree.delete(&path)?;
                self.tree = tree;
                VrApiRsp::Ok
            },
            TreeOp::ListKeys {..} => {
                // Use the path variable when vertree supports it
                VrApiRsp::TreeOpResult(TreeOpResult::Keys(self.tree.keys()))
            },
            TreeOp::BlobPut {path, val} => {
                let (reply, tree) = self.tree.blob_put(path, val)?;
                self.tree = tree;
                reply.into()
            },
            TreeOp::BlobGet {path} => {
                let reply = self.tree.blob_get(path)?;
                reply.into()
            },
            TreeOp::BlobSize {path} => {
                let reply = self.tree.blob_size(path)?;
                reply.into()
            },
            TreeOp::QueuePush {path, val} => {
                let (reply, tree) = self.tree.queue_push(path, val)?;
                self.tree = tree;
                reply.into()
            },
            TreeOp::QueuePop {path} => {
                let (reply, tree) = self.tree.queue_pop(path)?;
                self.tree = tree;
                reply.into()
            },
            TreeOp::QueueFront {path} => {
                let reply = self.tree.queue_front(path)?;
                reply.into()
            },
            TreeOp::QueueBack {path} => {
                let reply = self.tree.queue_back(path)?;
                reply.into()
            },
            TreeOp::QueueLen {path} => {
                let reply = self.tree.queue_len(path)?;
                reply.into()
            },
            TreeOp::SetInsert {path, val} => {
                let (reply, tree) = self.tree.set_insert(path, val)?;
                self.tree = tree;
                reply.into()
            },
            TreeOp::SetRemove {path, val} => {
                let (reply, tree) = self.tree.set_remove(path, val)?;
                self.tree = tree;
                reply.into()
            },
            TreeOp::SetContains {path, val} => {
                let reply= self.tree.set_contains(path, val)?;
                reply.into()
            },
            TreeOp::SetUnion {paths, sets} => {
                let reply = self.tree.set_union(paths, sets)?;
                reply.into()
            },
            TreeOp::SetIntersection {path1, path2} => {
                let reply = self.tree.set_intersection(&path1, &path2)?;
                reply.into()
            },
            TreeOp::SetDifference {path1, path2} => {
                let reply = self.tree.set_difference(&path1, &path2)?;
                reply.into()
            },
            TreeOp::SetSymmetricDifference {path1, path2} => {
                let reply = self.tree.set_symmetric_difference(&path1, &path2)?;
                reply.into()
            },
            TreeOp::SetSubsetPath {path1, path2} => {
                let reply = self.tree.set_subset(path1, Some(path2), None)?;
                reply.into()
            },
            TreeOp::SetSubsetSet {path,  set} => {
                let reply = self.tree.set_subset(path, None, Some(set))?;
                reply.into()
            },
            TreeOp::SetSupersetPath {path1, path2} => {
                let reply = self.tree.set_superset(path1, Some(path2), None)?;
                reply.into()
            },
            TreeOp::SetSupersetSet {path, set} => {
                let reply = self.tree.set_subset(path, None, Some(set))?;
                reply.into()
            }
        };
        Ok(val)
    }

    fn run_tree_cas(&mut self, tree_cas: TreeCas) -> Result<VrApiRsp, vertree::Error> {
        let TreeCas {guards, ops} = tree_cas;
        // All cas operations must be writes for now
        if ops.iter().any(|s| !s.is_write()) {
            let err = VrApiError::InvalidCas("All cas operations must be writes".to_string());
            // Returning Ok here because we already have a VrApiRsp (even though it's an error)
            return Ok(VrApiRsp::Error(err));
        }
        let guards = guards.into_iter().map(|g| vertree::Guard::from(g)).collect();
        let ops = ops.into_iter().map(|op| vertree::WriteOp::from(op)).collect();
        let (replies, tree) = self.tree.multi_cas(guards, ops)?;
        self.tree = tree;
        let replies = replies.into_iter().map(|r| r.into()).collect();
        Ok(VrApiRsp::TreeCasResult(replies))
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

impl From<NodeType> for vertree::NodeType {
    fn from(n: NodeType) -> vertree::NodeType {
        match n {
            NodeType::Blob => vertree::NodeType::Blob,
            NodeType::Queue => vertree::NodeType::Queue,
            NodeType::Set => vertree::NodeType::Set,
            NodeType::Directory => vertree::NodeType::Directory
        }
    }
}

impl From<vertree::NodeType> for NodeType {
    fn from(n: vertree::NodeType) -> NodeType {
        match n {
            vertree::NodeType::Blob => NodeType::Blob,
            vertree::NodeType::Queue =>NodeType::Queue,
            vertree::NodeType::Set => NodeType::Set,
            vertree::NodeType::Directory => NodeType::Directory
        }
    }
}


impl From<Guard> for vertree::Guard {
    fn from(g: Guard) -> vertree::Guard {
        vertree::Guard {
            path: g.path,
            version: g.version
        }
    }
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

impl From<vertree::Reply> for VrApiRsp {
    fn from(reply: vertree::Reply) -> VrApiRsp {
        VrApiRsp::TreeOpResult(reply.into())
    }
}

impl From<vertree::Error> for VrApiRsp {
    fn from(error: vertree::Error) -> VrApiRsp {
        let vertree::Error(kind, _) = error;
        let err = match kind {
            ErrorKind::AlreadyExists(path) => VrApiError::AlreadyExists(path),
            ErrorKind::DoesNotExist(path) => VrApiError::DoesNotExist(path),
            ErrorKind::WrongType(path, ty) => VrApiError::WrongType(path, ty.into()),
            ErrorKind::InvalidPathContent(path) => VrApiError::PathMustEndInDirectory(path),
            ErrorKind::CasFailed{path, expected, actual} =>
                VrApiError::CasFailed {path: path, expected: expected, actual: actual},
            ErrorKind::BadPath(msg) => VrApiError::BadFormat(msg),
            ErrorKind::Io(e) => VrApiError::Io(e.to_string()),
            ErrorKind::MsgPackValueReadError(e) => VrApiError::EncodingError(e.to_string()),
            ErrorKind::MsgPackValueWriteError(e) => VrApiError::EncodingError(e.to_string()),
            ErrorKind::CannotDeleteRoot => VrApiError::CannotDeleteRoot,
            ErrorKind::PathMustBeAbsolute(path) => VrApiError::PathMustBeAbsolute(path),
            ErrorKind::Msg(msg) => VrApiError::Msg(msg)
        };
        VrApiRsp::Error(err)
    }
}
