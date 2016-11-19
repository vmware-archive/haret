use uuid::Uuid;
use super::vr_api_messages::{VrApiReq, VrApiRsp};
use super::replica::{Replica, VersionedReplicas};

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum VrMsg {
    /// A message that drives the state of the fsm during periods of inactivity
    Tick,
    SessionClosed(Uuid),
    ClientRequest {
        /// The opaque api operation
        op: VrApiReq,
        request_num: u64
    },
    Reconfiguration {
        client_req_num: u64,
        epoch: u64,
        replicas: Vec<Replica>
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

impl VrMsg {
    // This is only used for filtering messages in vr_fsm. However we don't ever want to filter
    // client requests from the primary, so we exclude `VrMsg::Reconfiguration`
    pub fn get_epoch(&self) -> Option<u64> {
        match *self {
            VrMsg::ClientReply {epoch, ..} => Some(epoch),
            VrMsg::StartViewChange {epoch, ..} => Some(epoch),
            VrMsg::DoViewChange {epoch, ..} => Some(epoch),
            VrMsg::StartView {epoch, ..} => Some(epoch),
            VrMsg::Prepare {epoch, ..} => Some(epoch),
            VrMsg::PrepareOk {epoch, ..} => Some(epoch),
            VrMsg::Commit {epoch, ..} => Some(epoch),
            VrMsg::GetState {epoch, ..} => Some(epoch),
            VrMsg::NewState {epoch, ..} => Some(epoch),
            VrMsg::RecoveryResponse {epoch, ..} => Some(epoch),
            VrMsg::StartEpoch {epoch, ..} => Some(epoch),
            VrMsg::EpochStarted {epoch, ..} => Some(epoch),
            _ => None
        }
    }

    pub fn get_view(&self) -> Option<u64> {
        match *self {
            VrMsg::ClientReply {view, ..} => Some(view),
            VrMsg::StartViewChange {view, ..} => Some(view),
            VrMsg::DoViewChange {view, ..} => Some(view),
            VrMsg::StartView {view, ..} => Some(view),
            VrMsg::Prepare {view, ..} => Some(view),
            VrMsg::PrepareOk {view, ..} => Some(view),
            VrMsg::Commit {view, ..} => Some(view),
            VrMsg::GetState {view, ..} => Some(view),
            VrMsg::NewState {view, ..} => Some(view),
            VrMsg::RecoveryResponse {view, ..} => Some(view),
            _ => None
        }
    }
}
