// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! This module contains the backend code that executes commited consensus operations.
//! It represents the "state machine" in a "replicated state machine".
//!
//! Specifically, this backend wraps vertree and provides a generic public interface
//! to the consensus code so that the consensus code can be independent of any specific
//! api and it's tightly coupled backend implementation.

use std::convert::From;
use super::internal_api_messages::{ApiReq, ApiRsp, ApiError, TreeOp, TreeCas, TreeOpResult};
use vertree::{self, Tree};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Backend {
    pub tree: Tree
}

impl Default for Backend {
    fn default() -> Backend {
        Backend::new()
    }
}

impl Backend {
    pub fn new() -> Backend {
        Backend {
            tree: Tree::new()
        }
    }

    /// The sole method for executing backend operations provided to the consensus system
    pub fn call(&mut self, msg: ApiReq) -> ApiRsp {
        match msg {
            ApiReq::TreeOp(tree_op) => {
                match self.run_tree_op(tree_op) {
                    Ok(vr_api_rsp) => vr_api_rsp,
                    Err(vertree_error) => vertree_error.into()
                }
            },
            ApiReq::TreeCas(tree_cas) => {
                match self.run_tree_cas(tree_cas) {
                    Ok(vr_api_rsp) => vr_api_rsp,
                    Err(vertree_error) => vertree_error.into()
                }
            }
        }
    }

    fn run_tree_op(&mut self, tree_op: TreeOp) -> Result<ApiRsp, vertree::Error> {
        let val = match tree_op {
            TreeOp::Snapshot {directory} => {
                let s = self.tree.snapshot(&directory)?;
                ApiRsp::Path(s)
            },
            TreeOp::CreateNode {path, ty} => {
                self.tree = self.tree.create(&path, ty.into())?;
                ApiRsp::Ok
            },
            TreeOp::DeleteNode {path} => {
                let (_, tree) = self.tree.delete(&path)?;
                self.tree = tree;
                ApiRsp::Ok
            },
            TreeOp::ListKeys {..} => {
                // Use the path variable when vertree supports it
                ApiRsp::TreeOpResult(TreeOpResult::Keys(self.tree.keys()))
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
                let reply = self.tree.set_superset(path, None, Some(set))?;
                reply.into()
            }
        };
        Ok(val)
    }

    fn run_tree_cas(&mut self, tree_cas: TreeCas) -> Result<ApiRsp, vertree::Error> {
        let TreeCas {guards, ops} = tree_cas;
        // All cas operations must be writes for now
        if ops.iter().any(|s| !s.is_write()) {
            let err = ApiError::InvalidCas("All cas operations must be writes".to_string());
            // Returning Ok here because we already have a ApiRsp (even though it's an error)
            return Ok(ApiRsp::Error(err));
        }
        let guards = guards.into_iter().map(vertree::Guard::from).collect();
        let ops = ops.into_iter().map(vertree::WriteOp::from).collect();
        let (replies, tree) = self.tree.multi_cas(guards, ops)?;
        self.tree = tree;
        let replies = replies.into_iter().map(|r| r.into()).collect();
        Ok(ApiRsp::TreeCasResult(replies))
    }
}

