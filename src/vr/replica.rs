// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use rabble::{self, Process, Pid, CorrelationId, Envelope};
use funfsm::Fsm;
use msg::Msg;
use super::vr_fsm::VrTypes;
use super::vr_envelope::VrEnvelope;
use super::vrmsg::VrMsg;
use super::vr_ctx_summary::VrCtxSummary;
use super::super::admin::{AdminReq, AdminRpy};

/// A replica wraps a VR FSM as a process so that it can receive messsages from inside rabble
pub struct Replica {
    pid: Pid,
    state: VrStates
}

impl Replica {
    pub fn new(pid: Pid, state: VrStates) -> Replica {
        Replica {
            pid: pid,
            state: VrStates
        }
    }
}

impl Process<Msg> for Replica {
    fn handle(&mut self,
              msg: rabble::Msg<Msg>,
              from: Pid,
              correlation_id: Option<CorrelationId>,
              output: &mut Vec<Envelope<Msg>>)
    {
        let correlation_id = correlation_id.map_or(CorrelationId::pid(self.pid.clone()),
                                                   |c_id| c_id);
        match msg {
            rabble::Msg::User(Msg::AdminReq(AdminReq::GetReplicaState(_))) => {
                let (state, ctx) = self.state.get_state();
                let summary = VrCtxSummary::new(state, ctx);
                let rpy = AdminRpy::ReplicaState(summary);
                let msg = rabble::Msg::User(Msg::AdminRpy(rpy));
                let envelope = Envelope::new(from, self.pid.clone(), msg, Some(correlation_id));
                output.push(envelope);
            },
            rabble::Msg::User(Msg::Vr(vrmsg)) => {
               let vr_envelope = VrEnvelope::new(self.pid.clone(), from, vrmsg, correlation_id);
               output.extend(self.fsm.send(vr_envelope).into_iter().map(|e| e.into()));
            },
            _ => {
                let msg = rabble::Msg::User(Msg::Error("Invalid Msg Received".to_string()));
                let envelope = Envelope::new(from, self.pid.clone(), msg, Some(correlation_id));
                output.push(envelope);
            }
        }
    }

}


/// When we create a namesapce, the initial group of replicas is at epoch 1. When a reconfiguration
/// occurs we bump the epoch so when we gossip the information around to start the fsms, we can
/// chose the latest version.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct VersionedReplicas {
    pub epoch: u64,
    pub op: u64,
    pub replicas: Vec<Pid>
}

impl VersionedReplicas {
    pub fn new() -> VersionedReplicas {
        VersionedReplicas {
            epoch: 0,
            op: 0,
            replicas: Vec::new()
        }
    }
}
