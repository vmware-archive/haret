// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use rabble::{self, Pid};
use msg::Msg;
use super::vr_api_messages::{VrApiReq, VrApiRsp};
use super::replica::VersionedReplicas;
use std::convert::From;

/// Generate a message struct: `$struct_name` from a set of fields
///
/// Generate `impl From<$struct_name> for VrMsg`
macro_rules! msg {
    ($struct_name:ident {
        $( $field:ident: $ty:ty),+
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

        impl From<$struct_name> for rabble::Msg<Msg> {
            fn from(msg: $struct_name) -> rabble::Msg<Msg> {
                rabble::Msg::User(Msg::Vr(msg.into()))
            }
        }
    }
}

impl From<VrMsg> for rabble::Msg<Msg> {
    fn from(msg: VrMsg) -> rabble::Msg<Msg> {
        rabble::Msg::User(Msg::Vr(msg))
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

pub struct Tick;

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
    epoch: u64,
    client_req_num: u64,
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
    log_start: u64,
    log_tail: Vec<ClientOp>,
    commit_num: u64
});

msg!(StartView {
    epoch: u64,
    view: u64,
    op: u64,
    log_start: u64,
    log_tail: Vec<ClientOp>,
    commit_num: u64
});

msg!(Prepare {
    epoch: u64,
    view: u64,
    op: u64,
    commit_num: u64,
    global_min_accept: u64,
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
    commit_num: u64,
    global_min_accept: u64
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
    commit_num: u64,
    log_tail: Vec<ClientOp>
});

msg!(Recovery {
    epoch: u64,
    nonce: u64
});

msg!(RecoveryResponse {
    epoch: u64,
    view: u64,
    nonce: u64,
    global_min_accept: u64,

    // The following fields are only valid when sent by the Primary
    op: Option<u64>,
    commit_num: Option<u64>,
    log: Option<Vec<ClientOp>>,

    // The following fields aren't in the paper, but they allow recovery in a later epoch
    // This is required because a replica may be started from an old config via gossip.
    // When a recovery response with a greater epoch than the epoch from when the replica was started
    // is received, it will use these configurations and restart recovery so that it can properly
    // recover from a quorum. This also allows the replica to shutdown if it isn't in the new
    // config.
    old_config: Option<VersionedReplicas>,
    new_config: Option<VersionedReplicas>
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

