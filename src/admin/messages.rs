use rabble::{Pid, NodeId, ClusterStatus};
use config::Config;
use namespaces::Namespaces;
use uuid::Uuid;
use vr::VrCtxSummary;

#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub enum AdminMsg {
    Req(AdminReq),
    Rpy(AdminRpy)
}

#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub enum AdminReq {
    GetConfig,
    Join(NodeId),
    CreateNamespace(Vec<Pid>),
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
    Config(Config),
    NamespaceId(Uuid),
    Namespaces(Namespaces),
    ReplicaState(VrCtxSummary),
    ReplicaNotFound(Pid),
    Primary(Option<Pid>),
    ClusterStatus(ClusterStatus)
}

