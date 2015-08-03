/// Each member is globally unique. Each members name should also be globally unique. Members
/// locally register themselves and their name is also the name of the local ORSet. This denotes
/// the actor id.
use orset::ORSet;
use std::io::Result;
use rustc_serialize::Encodable;

#[derive(Debug, Clone, Eq, PartialEq, Hash, RustcEncodable, RustcDecodable)]
pub struct Member {
    /// name should be globally unique.
    pub name: String,
    pub cluster: String,
    pub ip: String
}

#[derive(Debug)]
pub struct Members {
    pub orset: ORSet<Member>
}

impl Members {
    pub fn new(name: String, cluster: String, ip: String) -> Members {
        let me = Member {
            name: name.clone(),
            cluster: cluster,
            ip: ip
        };
        let mut members = Members {
            orset: ORSet::new(name)
        };
        let _ = members.add(me);
        members
    }

    // TODO: Only allow adding member if it doesn't exist
    pub fn add(&mut self, element: Member) -> Result<()> {
        self.orset.add(element);
        Ok(())
    }
}
