use uuid::Uuid;
use super::replica::{Replica, VersionedReplicas};
use super::vrmsg::VrMsg;

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum Envelope {
    Peer(PeerEnvelope),
    Announcement(Announcement),
    Client(ClientEnvelope),
    ClientReply(ClientReplyEnvelope)
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct PeerEnvelope {
    pub to: Replica,
    pub from: Replica,
    pub msg: VrMsg
}

impl PeerEnvelope {
    pub fn new(to: Replica, from: Replica, msg: VrMsg) -> PeerEnvelope {
        PeerEnvelope {
            to: to,
            from: from,
            msg: msg
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum Announcement {
    Reconfiguration {tenant: Uuid, old_config: VersionedReplicas, new_config: VersionedReplicas},
    Stop(Replica),
    NewPrimary(Replica),
    ClearPrimary(Uuid)
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct ClientReplyEnvelope {
    pub to: Uuid,
    pub msg: VrMsg
}

impl ClientReplyEnvelope {
    pub fn new(to: Uuid, msg: VrMsg) -> ClientReplyEnvelope {
        ClientReplyEnvelope {
            to: to,
            msg: msg
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct ClientEnvelope {
    pub to: Replica,

    // Note that VrMsg's that aren't of the ClientRequest variety will be dropped on receipt and the
    // connection closed.
    pub msg: VrMsg
}
