// Copyright 2017 VMware, Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod replica;
mod backend;
mod quorum_tracker;
mod prepare_requests;
mod vrmsg;
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
