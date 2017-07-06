// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

type Version = u64;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    Blob,
    Queue,
    Set,
    Directory
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum VrApiReq {
    TreeOp(TreeOp),
    TreeCas(TreeCas),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Guard {
    pub path: String,
    pub version: u64
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TreeCas {
    pub guards: Vec<Guard>,
    pub ops: Vec<TreeOp>
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum VrApiRsp {
    Ok,
    TreeOpResult(TreeOpResult),
    TreeCasResult(Vec<TreeOpResult>),
    Path(String),
    Error(VrApiError)
}

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
