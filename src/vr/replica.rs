use std::cmp::{Ordering, PartialOrd};
use rustc_serialize::Encodable;
use membership::Member;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, RustcEncodable, RustcDecodable)]
/// A replica not yet assigned a tenant
pub struct RawReplica {
    pub name: String,
    pub node: Member
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, RustcEncodable, RustcDecodable)]
pub struct Replica {
    pub tenant: Uuid,
    pub name: String,
    pub node: Member
}

// Ignore Uuid. Only allows for comparing replicas in the same tenant.
impl PartialOrd for Replica {
    fn partial_cmp(&self, other: &Replica) -> Option<Ordering> {
        if self.name < other.name { return Some(Ordering::Less) }
        if self.name == other.name && self.node < other.node { return Some(Ordering::Less) }
        if self.name > other.name { return Some(Ordering::Greater) }
        if self.name == other.name && self.node > other.node { return Some(Ordering::Greater) }
        Some(Ordering::Equal)
    }
}

impl Ord for Replica {
    fn cmp(&self, other: &Replica) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Replica {
    pub fn new(tenant: Uuid, raw: RawReplica) -> Replica {
        Replica {
            tenant: tenant,
            name: raw.name,
            node: raw.node
        }
    }
}

/// When we create a tenant, the initial group of replicas is at epoch 1. When a reconfiguration
/// occurs we bump the epoch so when we gossip the information around to start the fsms, we can
/// chose the latest version.
#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct VersionedReplicas {
    pub epoch: u64,
    pub op: u64,
    pub replicas: Vec<Replica>
}

impl VersionedReplicas {
    pub fn new() -> VersionedReplicas {
        VersionedReplicas {
            epoch: 0,
            op: 0,
            replicas: Vec::new()
        }
    }
}
