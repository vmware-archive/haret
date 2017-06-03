// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use rabble::{self, Process, Pid, CorrelationId, Envelope};
use msg::Msg;
use vr::vr_msg::VrMsg;
use vr::vr_fsm::VrState;
use super::super::admin::{AdminReq, AdminRpy};

/// A replica wraps a VR FSM as a process so that it can receive messsages from inside rabble
pub struct Replica {
    pid: Pid,
    state: VrState
}

impl Replica {
    pub fn new(pid: Pid, state: VrState) -> Replica {
        Replica {
            pid: pid,
            state: state
        }
    }
}

impl Process<Msg> for Replica {
    fn handle(&mut self,
              msg: rabble::Msg<Msg>,
              from: Pid,
              cid: Option<CorrelationId>,
              output: &mut Vec<Envelope<Msg>>)
    {
        let cid = cid.map_or(CorrelationId::pid(self.pid.clone()), |c_id| c_id);
        match msg {
            rabble::Msg::User(Msg::AdminReq(AdminReq::GetReplicaState(_))) => {
                let rpy = AdminRpy::ReplicaState(self.state.clone());
                let msg = rabble::Msg::User(Msg::AdminRpy(rpy));
                let envelope = Envelope::new(from, self.pid.clone(), msg, Some(cid));
                output.push(envelope);
            },
            rabble::Msg::User(Msg::Vr(vrmsg)) => {
               self.state.next(vrmsg, from, cid, output);
            },
            _ => {
                let msg = rabble::Msg::User(Msg::Error("Invalid Msg Received".to_string()));
                let envelope = Envelope::new(from, self.pid.clone(), msg, Some(cid));
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
