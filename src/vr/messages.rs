use uuid::Uuid;
use super::replica::{Replica, VersionedReplicas};
use vr_api::{VrApiReq, VrApiRsp};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VrMsg {
    /// A message that drives the state of the fsm during periods of inactivity
    Tick,
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

#[derive(Debug, Clone)]
pub struct Envelope {
    pub to: Replica,
    pub msg: VrMsg
}

impl Envelope {
    pub fn new(to: Replica, msg: VrMsg) -> Envelope {
        Envelope {
            to: to,
            msg: msg
        }
    }
}

#[derive(Debug, Clone)]
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
