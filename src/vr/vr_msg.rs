// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use uuid::Uuid;
use rabble::Pid;
use super::vr_api_messages::{VrApiReq, VrApiRsp};
use super::replica::VersionedReplicas;
use std::convert::From;

/// Generate a message struct: `$struct_name` from a set of fields
///
/// Generate `impl From<$struct_name> for VrMsg`
macro_rules! msg {
    ($struct_name:ident {
        $( $field:ident: $ty:ident ),+
    }) => {
        #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
        pub struct $struct_name {
            $( pub $field: $ty ),+
        }

        impl From<$struct_name> for VrMsg {
            fn from(msg: $struct_name) -> VrMsg {
                VrMsg::$struct_name(msg)
            }
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ClientOp {
    Request(ClientRequest),
    Reconfiguration(Reconfiguration)
}

impl From<ClientRequest> for ClientOp {
    fn from(req: ClientRequest) -> ClientOp {
        ClientOp::Request(req)
    }
}

impl From<Reconfiguration> for ClientOp {
    fn from(reconfig: Reconfiguration) -> ClientOp {
        ClientOp::Reconfiguration(reconfig)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum VrMsg {
    Tick, // A message that drives the state of the fsm during periods of inactivity
    ClientRequest(ClientRequest),
    Reconfiguration(Reconfiguration),
    ClientReply(ClientReply),
    StartViewChange(StartViewChange),
    DoViewChange(DoViewChange),
    StartView(StartView),
    Prepare(Prepare),
    PrepareOk(PrepareOk),
    Commit(Commit),
    GetState(GetState),
    NewState(NewState),
    Recovery(Recovery),
    RecoveryResponse(RecoveryResponse),
    StartEpoch(StartEpoch),
    EpochStarted(EpochStarted),
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

struct Tick;

impl From<Tick> for VrMsg {
    fn from(_: Tick) -> VrMsg {
        VrMsg::Tick
    }
}

msg!(ClientRequest {
    op: VrApiReq,
    client_id: String,
    request_num: u64
});

msg!(Reconfiguration {
    client_req_num: u64,
    epoch: u64,
    replicas: Vec<Pid>
});

msg!(ClientReply {
    epoch: u64,
    view: u64,
    request_num: u64,
    value: VrApiRsp
});

msg!(StartViewChange {
    epoch: u64,
    view: u64,
    op: u64
});

msg!(DoViewChange {
    epoch: u64,
    view: u64,
    op: u64,
    last_normal_view: u64,
    log: Vec<VrMsg>,
    commit_num: u64
});

msg!(StartView {
    epoch: u64,
    view: u64,
    op: u64,
    log: Vec<VrMsg>,
    commit_num: u64
});

msg!(Prepare {
    epoch: u64,
    view: u64,
    op: u64,
    commit_num: u64,
    msg: ClientOp
});

msg!(PrepareOk {
    epoch: u64,
    view: u64,
    op: u64
});

msg!(Commit {
    epoch: u64,
    view: u64,
    commit_num: u64
});

msg!(GetState {
    epoch: u64,
    view: u64,
    op: u64
});

msg!(NewState {
    epoch: u64,
    view: u64,
    op: u64,
    primary: Option<Pid>,
    commit_num: u64,
    log_tail: Vec<VrMsg>,
});

msg!(Recovery {
    nonce: Uuid
});

msg!(RecoveryResponse {
    epoch: u64,
    view: u64,
    nonce: Uuid,
    // The following fields are only valid when sent by the Primary
    op: Option<u64>,
    commit_num: Option<u64>,
    log: Option<Vec<VrMsg>>
});

msg!(StartEpoch {
    epoch: u64,
    op: u64,
    old_config: VersionedReplicas,
    new_config: VersionedReplicas
});

msg!(EpochStarted {
    epoch: u64
});

