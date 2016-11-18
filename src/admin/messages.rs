use rabble::{Pid, NodeId};
use namespaces::Namespaces;
use uuid::Uuid;
use session_messages::NewSessionReply;
use vr::VrCtx;

#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub enum AdminMsg {
    Req(AdminReq),
    Rpy(AdminRpy)
}

#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub enum AdminReq {
    Join(NodeId),
    CreateNamespace {
        namespace: Uuid,
        replicas: Vec<Pid>,
    },
    GetNamespaces,
    GetReplicaState(Pid),
    GetPrimary(Uuid),
    GetClusterStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub enum AdminRpy {
    Ok,
    Timeout,
    Error(String),
    NamespaceId(Uuid),
    Namespaces(Namespaces),
    ReplicaState {state: &'static str, ctx: VrCtx},
    ReplicaNotFound(Pid),
    Primary(Option<Pid>)
}

