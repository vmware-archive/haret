use std::collections::HashSet;

type Version = u64;

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum NodeType {
    Blob,
    Queue,
    Set,
    Directory
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum VrApiReq {
    TreeOp(TreeOp),
    TreeCas(TreeCas),
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
// This should maintain the identical format to the vertree version. The only difference is that the
// v2r2 one is encodable and decodable
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

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Guard {
    pub path: String,
    pub version: u64
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct TreeCas {
    pub guards: Vec<Guard>,
    pub ops: Vec<TreeOp>
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum VrApiRsp {
    Ok,
    TreeOpResult(TreeOpResult),
    TreeCasResult(Vec<TreeOpResult>),
    Path(String),
    Error(VrApiError)
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum TreeOpResult {
    Ok(Option<Version>),
    Empty(Option<Version>),
    Bool(bool, Option<Version>),
    Blob(Vec<u8>, Option<Version>),
    Int(u64, Option<Version>),
    Set(Vec<Vec<u8>>, Option<Version>),
    Keys(Vec<(String, Version)>)
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
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
