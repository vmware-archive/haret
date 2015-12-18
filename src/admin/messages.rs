use mio::Token;
use uuid::Uuid;
use vr::{Replica, RawReplica, Tenants, VrCtx, VrStats};
use debug_sender::DebugSender;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdminReq {
    Join {token: Token, ipstr: String, reply_tx: DebugSender<AdminRpy>},
    CreateTenant {
        token: Token,
        tenant: Uuid,
        replicas: Vec<RawReplica>,
        reply_tx: DebugSender<AdminRpy>
    },
    GetTenants {token: Token, reply_tx: DebugSender<AdminRpy>},
    GetReplica {token: Token, replica: Replica, reply_tx: DebugSender<AdminRpy>},
    GetVrStats {token: Token, reply_tx: DebugSender<AdminRpy>}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdminRpy {
    Ok(Token),
    Error(Token, String),
    JoinReply {token: Token, reply: Result<(), String>},
    TenantId {token: Token, id: Uuid},
    Tenants {token: Token, tenants: Tenants},
    Replica {token: Token, state: &'static str, ctx: VrCtx},
    ReplicaNotFound {token: Token, replica: Replica},
    VrStats {token: Token, stats: String}
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
    VrStats
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
    VrStats(String)
}

