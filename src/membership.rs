/// Each member is globally unique. Each members name should also be globally unique. Members
/// locally register themselves and their name is also the name of the local ORSet. This denotes
/// the actor id.
use orset::ORSet;
use std::result;
use std::io::Result;
use std::fmt::{Display, Error, Formatter};
use rustc_serialize::Encodable;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, RustcEncodable, RustcDecodable)]
pub struct Member {
    /// name should be globally unique.
    pub name: String,
    pub cluster: String,
    pub ip: String
}

#[derive(Debug, Clone)]
// TODO: Move to an internal `data` member and use a single Lock around both internal members?
// For now just always take the locks in member order: `orset` first, then `connected`
pub struct Members {
    me: Member,
    orset: Arc<RwLock<ORSet<Member>>>,
    connected: Arc<RwLock<HashSet<Member>>>
}

impl Display for Members {
    fn fmt(&self, fmt: &mut Formatter) -> result::Result<(), Error> {
        let orset = self.orset.read().unwrap();
        let mut members = (*orset).elements();
        members.sort();
        for member in members {
            try!(fmt.write_fmt(format_args!("{} {}\n", member.name, member.ip)));
        }
        Ok(())
    }
}

impl Members {
    pub fn new(name: String, cluster: String, ip: String) -> Members {
        let me = Member {
            name: name.clone(),
            cluster: cluster,
            ip: ip
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
            acc.push(' ');
            acc.push_str(&m.ip);
            acc.push_str(" (Connected)\n");
            acc
        });
        let disconnected_str = disconnected.iter().fold(String::new(), |mut acc, m| {
            acc.push_str(&m.name);
            acc.push(' ');
            acc.push_str(&m.ip);
            acc.push_str(" (Disconnected)\n");
            acc
        });
        format!("{} {} (Self)\n{}{}", self.me.name, self.me.ip, connected_str, disconnected_str)
    }

    pub fn local_name(&self) -> String {
        let orset = self.orset.read().unwrap();
        orset.name.clone()
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
    pub fn add(&mut self, element: Member) -> Result<()> {
        let mut orset = self.orset.write().unwrap();
        (*orset).add(element);
        Ok(())
    }

    pub fn connected(&mut self, ip: &str) {
        if let Some(member) = self.member_by_ip(ip) {
            let mut connected = self.connected.write().unwrap();
            (*connected).insert(member);
        }
    }

    pub fn disconnected(&mut self, ip: &str) {
        if let Some(member) = self.member_by_ip(ip) {
            let mut connected = self.connected.write().unwrap();
            (*connected).remove(&member);
        }
    }

    // TODO: Memoize this
    pub fn get_ips(&self) -> HashSet<String> {
        let orset = self.orset.read().unwrap();
        (*orset).elements().iter().map(|ref m| (*m).ip.clone()).collect()
    }

    pub fn get_connected_ips(&self) -> HashSet<String> {
        let connected = self.connected.read().unwrap();
        connected.iter().map(|m| m.ip.clone()).collect()
    }

    // TODO: This is just a simple linear search. For larger orsets we probably want to do a binary
    // search on a sorted elements vector. The vector should be sorted and memoized as part of the
    // orset.
    pub fn member_by_ip(&self, ip: &str) -> Option<Member> {
        let orset = self.orset.read().unwrap();
        (*orset).elements().iter().find(|&m| m.ip == ip).map(|ref m| (*m).clone())
    }
}
