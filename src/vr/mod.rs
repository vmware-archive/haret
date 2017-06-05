// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod replica;
mod backend;
mod vr_msg;
mod vr_api_messages;

#[macro_use]
pub mod vr_fsm;
pub mod states;
pub mod vr_ctx;

pub use self::vr_fsm::{
    VrState
};

pub use self::vr_ctx::{
    VrCtx
};

pub use self::replica::{
    Replica,
    VersionedReplicas
};

pub use self::vr_msg::{
    VrMsg,
    ClientRequest,
    ClientReply
};

pub use self::vr_api_messages::{
    VrApiReq,
    VrApiRsp,
    VrApiError,
    TreeOp,
    TreeCas,
    NodeType,
    Guard,
    TreeOpResult
};

pub use self::backend::VrBackend;
