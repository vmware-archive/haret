use rabble::Pid;
use namespaces::Namespaces;
use uuid::Uuid;
use shared_messages::NewSessionReply;
use vr::{RawReplica, VrCtx};

#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub enum AdminReq {
    Join(ipstr: String),
    CreateNamespace {
        namespace: Uuid,
        replicas: Vec<RawReplica>,
    },
    GetNamespaces,
    GetReplicaState(Pid),
    GetPrimary(Uuid),
    GetNewSessionReply(Uuid)
}

#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub enum AdminRpy {
    Ok,
    Error(String),
    NamespaceId(Uuid),
    Namespaces(Namespaces),
    ReplicaState {state: &'static str, ctx: VrCtx},
    ReplicaNotFound(Pid),
    Primary(Option<Pid>),
    NewSessionReply(NewSessionReply)
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum AdminClientReq {
    ConfigSet(String, String),
    ConfigGet(String),
    ClusterJoin(String),
    ClusterMembers,
    ClusterStatus,
    VrCreateTenant(String),
    VrTenants,
    VrReplica(Replica),
    VrStats,
    VrPrimary(Uuid)
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum AdminClientRpy {
    Ok,
    Error(String),
    Timeout,
    Config(String, String),
    MemberStatus(String),
    Members(String),
    VrTenantId(Uuid),
    VrTenants(Tenants),
    VrReplica(String, String),
    VrStats(String),
    VrPrimary(Option<Replica>)
}

