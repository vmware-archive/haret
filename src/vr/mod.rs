// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod replica;
mod backend;
mod quorum_tracker;
mod prepare_requests;
mod vr_msg;
mod vr_api_messages;
mod fsm_output;
mod vr_envelope;
mod view_change_state;
mod recovery_state;
mod vr_ctx_summary;

pub mod vr_fsm;
pub mod vr_ctx;

pub use self::vr_fsm::{
    VrTypes
};

pub use self::vr_ctx::{
    VrCtx
};

pub use self::vr_ctx_summary::VrCtxSummary;

pub use self::replica::{
    Replica,
    VersionedReplicas
};

pub use self::vrmsg::VrMsg;
pub use self::vr_envelope::VrEnvelope;
pub use self::fsm_output::FsmOutput;

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
