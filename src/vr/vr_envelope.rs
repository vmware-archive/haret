// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

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

