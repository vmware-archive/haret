/// Each member is globally unique. Each members name should also be globally unique. Members
/// locally register themselves and their name is also the name of the local ORSet. This denotes
/// the actor id.
use orset::ORSet;
use std::result;
use std::io::Result;
use std::fmt::{Display, Error, Formatter};
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, RustcEncodable, RustcDecodable)]
pub struct Member {
    /// name should be globally unique.
    pub name: String,
    pub cluster_host: String,
    pub vr_host: String,
    pub vr_api_host: String
}

impl Member {
    #[cfg(debug_assertions)]
    pub fn new_test_node(name: &str) -> Member {
        Member {
            name: name.to_string(),
            cluster_host: "".to_string(),
            vr_host: "".to_string(),
            vr_api_host: "".to_string()
        }
    }
}

/// We only want to do comparisons on the name of the Member. Note however that this can cause
/// problems when merging ORSets of members with the same name and different hosts. However, we
/// prevent this in the Members wrapper at join time by doing a full equality check. This allows us
/// to prevent duplicate members and also allow enumeration and membership checks based on name
/// only.
impl PartialEq for Member {
    fn eq(&self, other: &Member) -> bool {
        self.name == other.name
    }
}

impl Hash for Member {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.name.hash(state);
    }
}

#[derive(Debug, Clone)]
// TODO: Move to an internal `data` member and use a single Lock around both internal members?
// For now just always take the locks in member order: `orset` first, then `connected`
pub struct Members {
    pub me: Member,
    orset: Arc<RwLock<ORSet<Member>>>,
    connected: Arc<RwLock<HashSet<Member>>>
}

impl Display for Members {
    fn fmt(&self, fmt: &mut Formatter) -> result::Result<(), Error> {
        let orset = self.orset.read().unwrap();
        let mut members = (*orset).elements();
        members.sort();
        for member in members {
            try!(fmt.write_fmt(format_args!("{} \n", member.name)));
        }
        Ok(())
    }
}

impl Members {
    pub fn new(name: String, cluster_host: String, vr_host: String, vr_api_host: String) -> Members {
        let me = Member {
            name: name.clone(),
            cluster_host: cluster_host,
            vr_host: vr_host,
            vr_api_host: vr_api_host
        };
        let mut orset = ORSet::new(name);
        orset.add(me.clone());
        Members {
            me: me,
            orset: Arc::new(RwLock::new(orset)),
            connected: Arc::new(RwLock::new(HashSet::new()))
        }
    }

    pub fn status(&self) -> String {
        let orset = self.orset.read().unwrap();
        let connected = self.connected.read().unwrap();
        let all: HashSet<Member> =
            (*orset).elements().iter().filter(|&m| *m != self.me).cloned().collect();
        let mut disconnected: Vec<Member> = all.difference(&(*connected)).cloned().collect();
        disconnected.sort();
        let mut connected: Vec<Member> = (*connected).iter().cloned().collect();
        connected.sort();
        let connected_str = connected.iter().fold(String::new(), |mut acc, m| {
            acc.push_str(&m.name);
            acc.push_str(" (Connected)\n");
            acc
        });
        let disconnected_str = disconnected.iter().fold(String::new(), |mut acc, m| {
            acc.push_str(&m.name);
            acc.push_str(" (Disconnected)\n");
            acc
        });
        format!("{} (Self)\n{}{}", self.me.name, connected_str, disconnected_str)
    }

    pub fn join(&mut self, other: ORSet<Member>) {
        let mut orset = self.orset.write().unwrap();
        (*orset).join_state(other);
    }

    pub fn get_orset(&self) -> ORSet<Member> {
        let orset = self.orset.read().unwrap();
        (*orset).clone()
    }

    // TODO: Only allow adding member if it doesn't exist
    #[allow(dead_code)]
    pub fn add(&mut self, element: Member) -> Result<()> {
        let mut orset = self.orset.write().unwrap();
        (*orset).add(element);
        Ok(())
    }

    pub fn connected(&mut self, ip: &str) {
        if let Some(member) = self.member_by_cluster_host(ip) {
            let mut connected = self.connected.write().unwrap();
            (*connected).insert(member);
        }
    }

    pub fn disconnected(&mut self, ip: &str) {
        if let Some(member) = self.member_by_cluster_host(ip) {
            let mut connected = self.connected.write().unwrap();
            (*connected).remove(&member);
        }
    }

    // TODO: Memoize this
    pub fn get_cluster_hosts(&self) -> HashSet<String> {
        let orset = self.orset.read().unwrap();
        (*orset).elements().iter().map(|ref m| (*m).cluster_host.clone()).collect()
    }

    pub fn get_connected_cluster_hosts(&self) -> HashSet<String> {
        let connected = self.connected.read().unwrap();
        connected.iter().map(|m| m.cluster_host.clone()).collect()
    }

    // TODO: This is just a simple linear search. For larger orsets we probably want to do a binary
    // search on a sorted elements vector. The vector should be sorted and memoized as part of the
    // orset.
    pub fn member_by_cluster_host(&self, ip: &str) -> Option<Member> {
        let orset = self.orset.read().unwrap();
        (*orset).elements().iter().find(|&m| m.cluster_host == ip).map(|ref m| (*m).clone())
    }
}
