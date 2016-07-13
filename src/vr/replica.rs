use std::cmp::{Ordering, PartialOrd};
use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;
use membership::Member;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, RustcEncodable, RustcDecodable)]
/// A replica not yet assigned a tenant
pub struct RawReplica {
    pub name: String,
    pub node: Member
}

impl Display for RawReplica {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}::{}", self.name, self.node.name)
    }
}

impl FromStr for RawReplica {
    type Err = String;

    fn from_str(s: &str) -> Result<RawReplica, String> {
        let v: Vec<&str> = s.split("::").collect();
        if v.len() != 2 {
            return Err(
                "Invalid RawReplica format. Must be of type ReplicaName::NodeName".to_string());
        }
        // Note that generation cannot correctly populate cluster_host and vr_host. Therefore this
        // code should only be used to check membership/ do lookups on a member and NOT to create one.
        Ok(RawReplica {
            name: v[0].to_string(),
            node: Member {
                name: v[1].to_string(),
                cluster_host: "".to_string(),
                vr_host: "".to_string(),
                vr_api_host: "".to_string()
            }
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, RustcEncodable, RustcDecodable)]
pub struct Replica {
    pub tenant: Uuid,
    pub name: String,
    pub node: Member
}

impl Display for Replica {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}::{}::{}", self.tenant, self.name, self.node.name)
    }
}

impl FromStr for Replica {
    type Err = String;

    fn from_str(s: &str) -> Result<Replica, String> {
        let v: Vec<&str> = s.split("::").collect();
        if v.len() != 3 {
            return Err("Invalid Replica format. Must be of type TenantUuid::ReplicaName::NodeName::NodeIP:NodePort".to_string());
        }
        match Uuid::parse_str(v[0]) {
            Ok(tenant_id) => {
                // Note that generation cannot correctly populate cluster_host and vr_host.
                // Therefore this code should only be used to check membership/ do lookups on a
                // member and NOT to create one.
                Ok(Replica {
                    tenant: tenant_id,
                    name: v[1].to_string(),
                    node: Member {
                        name: v[2].to_string(),
                        cluster_host: "".to_string(),
                        vr_host: "".to_string(),
                        vr_api_host: "".to_string()
                    }
                })
            },
            Err(_) => Err("Tenant Id must be a Uuid".to_string())
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_replica_to_str() {
        let s = "r1::node1";
        let raw_replica: RawReplica = s.parse().unwrap();
        assert_eq!(&raw_replica.to_string(), s);
    }

    #[test]
    fn replica_to_str() {
        let s = "f378dc44-f58b-4a6d-af63-c8791c44043c::r1::node1";
        let replica: Replica = s.parse().unwrap();
        assert_eq!(&replica.to_string(), s);
    }

}
