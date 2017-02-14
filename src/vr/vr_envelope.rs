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

use msg::Msg;
use rabble::{self, Envelope, Pid, CorrelationId};
use super::vrmsg::VrMsg;

#[derive(Debug, Clone)]
pub struct VrEnvelope {
    pub to: Pid,
    pub from: Pid,
    pub msg: VrMsg,
    pub correlation_id: CorrelationId
}

impl VrEnvelope {
    pub fn new(to: Pid, from: Pid, msg: VrMsg, correlation_id: CorrelationId) -> VrEnvelope {
        VrEnvelope {
            to: to,
            from: from,
            msg: msg,
            correlation_id: correlation_id
        }
    }
}

impl From<VrEnvelope> for Envelope<Msg> {
    fn from(vr_envelope: VrEnvelope) -> Envelope<Msg> {
        Envelope {
            to: vr_envelope.to,
            from: vr_envelope.from,
            msg: rabble::Msg::User(Msg::Vr(vr_envelope.msg)),
            correlation_id: Some(vr_envelope.correlation_id)
        }
    }
}

