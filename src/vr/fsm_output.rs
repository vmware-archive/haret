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

