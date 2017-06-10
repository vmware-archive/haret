// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use rabble::{self, Pid, CorrelationId, Envelope};
use msg::Msg;
use namespace_msg::NamespaceMsg;
use super::vr_envelope::VrEnvelope;

#[derive(Debug, Clone)]
pub enum FsmOutput {
    Vr(VrEnvelope),
    Announcement(NamespaceMsg, Pid)
}

/// Convert from FsmOuptput to Envelope with "self.into()" or Envelope::from(self)
impl From<FsmOutput> for Envelope<Msg> {
    fn from(fsm_output: FsmOutput) -> Envelope<Msg> {
        match fsm_output {
            FsmOutput::Vr(vr_envelope) => vr_envelope.into(),
            FsmOutput::Announcement(namespace_msg, pid) => Envelope {
                to: Pid {group: None, name: "namespace_mgr".to_string(), node: pid.node.clone()},
                from: pid.clone(),
                msg: rabble::Msg::User(Msg::Namespace(namespace_msg)),
                correlation_id: Some(CorrelationId::pid(pid))
            }
        }
    }
}

