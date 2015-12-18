use uuid::Uuid;
use rustc_serialize::Encodable;
use super::replica::{Replica, VersionedReplicas};
use super::ElementType;

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum VrMsg {
    /// A message that drives the state of the fsm during periods of inactivity
    Tick,
    SessionClosed(Uuid),
    ClientRequest {
        /// The opaque api operation
        op: VrApiReq,
        client_id: Uuid,
        request_num: u64
    },
    ClientReply {
        epoch: u64,
        view: u64,
        request_num: u64,
        value: VrApiRsp
    },
    StartViewChange {
        epoch: u64,
        view: u64,
        op: u64,
        from: Replica
    },
    DoViewChange {
        epoch: u64,
        view: u64,
        op: u64,
        from: Replica,
        last_normal_view: u64,
        log: Vec<VrMsg>,
        commit_num: u64
    },
    StartView {
        epoch: u64,
        view: u64,
        op: u64,
        log: Vec<VrMsg>,
        commit_num: u64
    },
    Prepare {
        epoch: u64,
        view: u64,
        op: u64,
        client_id: Uuid,
        client_request_num: u64,
        client_op: VrApiReq,
        commit_num: u64
    },
    PrepareOk {
        epoch: u64,
        view: u64,
        op: u64,
        from: Replica
    },
    Commit {
        epoch: u64,
        view: u64,
        commit_num: u64
    },
    GetState {
        epoch: u64,
        view: u64,
        op: u64,
        from: Replica
    },
    NewState {
        epoch: u64,
        view: u64,
        op: u64,
        primary: Option<Replica>,
        commit_num: u64,
        log_tail: Vec<VrMsg>,
    },
    Recovery {
        from: Replica,
        nonce: Uuid
    },
    RecoveryResponse {
        epoch: u64,
        view: u64,
        nonce: Uuid,
        from: Replica,
        // The following fields are only valid when sent by the Primary
        op: Option<u64>,
        commit_num: Option<u64>,
        log: Option<Vec<VrMsg>>
    },
    Reconfiguration {
        client_id: Uuid,
        client_req_num: u64,
        epoch: u64,
        replicas: Vec<Replica>
    },
    StartEpoch {
        epoch: u64,
        op: u64,
        old_config: VersionedReplicas,
        new_config: VersionedReplicas
    },
    EpochStarted {
        epoch: u64,
        from: Replica
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Envelope {
    pub to: Replica,
    pub from: Replica,
    pub msg: VrMsg
}

impl Envelope {
    pub fn new(to: Replica, from: Replica, msg: VrMsg) -> Envelope {
        Envelope {
            to: to,
            from: from,
            msg: msg
        }
    }
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

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum VrApiReq {
    Create {path: String, ty: ElementType},
    Put {path: String, data: Vec<u8>, cas_tag: Option<u64>},
    Get {path: String, cas: bool},
    Delete {path: String, cas_tag: Option<u64>},
    List {path: String},
    Null // used during reconfiguration
}

impl VrApiReq {
    pub fn get_path(&self) -> String {
        match *self {
            VrApiReq::Create {ref path, ..} => path.clone(),
            VrApiReq::Put {ref path, ..} => path.clone(),
            VrApiReq::Get {ref path, ..} => path.clone(),
            VrApiReq::Delete {ref path, ..} => path.clone(),
            VrApiReq::List {ref path, ..} => path.clone(),
            VrApiReq::Null => unreachable!()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum VrApiRsp {
    Element {data: Vec<u8>, cas_tag: Option<u64>},
    KeyList {keys: Vec<String>},
    Ok,
    Error {msg: String},
    Timeout,
    ParentNotFoundError,
    ElementAlreadyExistsError,
    ElementNotFoundError(String),
    CasFailedError {path: String, expected: u64, actual: u64},
}
