// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use rabble::{self, Process, Pid, CorrelationId, Envelope};
use msg::Msg;
use vr::vr_fsm::VrState;
use vr::vr_ctx::VrCtx;
use vr::states::{Primary, Backup, Recovery, Reconfiguration};
use disk_msgs::{DiskReq, DiskRpy};
use super::super::admin::{AdminReq, AdminRpy};

/// A replica wraps a VR FSM as a process so that it can receive messsages from inside rabble
pub struct Replica {
    pid: Pid,
    // Becomes none after the initial state is created
    ctx: Option<VrCtx>,
    // Starts in None state then is only optional to allow updates duing handle calls
    state: Option<VrState>
}

impl Replica {
    pub fn new(pid: Pid, ctx: VrCtx) -> Replica {
        Replica {
            pid: pid,
            ctx: Some(ctx),
            state: None
        }
    }

    fn start_initial(&mut self) {
       let ctx = self.ctx.take().unwrap();
       if self.pid == ctx.compute_primary() {
           self.state = Some(VrState::Primary(Primary::new(ctx)));
       } else {
           self.state = Some(VrState::Backup(Backup::new(ctx)));
       };
    }

    fn start_reconfiguration(&mut self) {
        let ctx = self.ctx.take().unwrap();
        self.state = Some(VrState::Reconfiguration(Reconfiguration::new(ctx)));
    }

    fn recover(&mut self, nonce: u64) {
        let ctx = self.ctx.take().unwrap();
        self.state = Some(VrState::Recovery(Recovery::new(ctx, nonce)));
    }
}

impl Process<Msg> for Replica {
    fn init(&mut self, _: Pid) -> Vec<Envelope<Msg>> {
        // Try to learn if this replica is in recovery or not
        let to = Pid {
            group: None,
            name: "disk_mgr".to_owned(),
            node: self.pid.node.clone()
        };
        let from = self.pid.clone();
        let msg = DiskReq::ReadNonce.into();
        let cid = CorrelationId::pid(from.clone());
        vec![Envelope::new(to, from, msg, Some(cid))]
    }

    fn handle(&mut self,
              msg: rabble::Msg<Msg>,
              from: Pid,
              cid: Option<CorrelationId>,
              output: &mut Vec<Envelope<Msg>>)
    {
        let cid = cid.map_or(CorrelationId::pid(self.pid.clone()), |c_id| c_id);
        match msg {
            rabble::Msg::User(Msg::AdminReq(AdminReq::GetReplicaState(_))) => {
                if self.state.is_none() {
                    // We are still waiting for a DiskRpy with the nonce
                    // TODO: Send a response to the admin client
                    return;
                }
                let rpy = AdminRpy::ReplicaState(self.state.as_ref().unwrap().clone());
                let msg = rabble::Msg::User(Msg::AdminRpy(rpy));
                let envelope = Envelope::new(from, self.pid.clone(), msg, Some(cid));
                output.push(envelope);
            }
            rabble::Msg::User(Msg::Vr(vrmsg)) => {
                if self.state.is_none() {
                    // We are still waiting for a DiskRpy with the nonce
                    return;
                }
                let state = self.state.take().unwrap();
                self.state = Some(state.next(vrmsg, from, cid, output));
            }
            rabble::Msg::User(Msg::DiskRpy(DiskRpy::Nonce(nonce))) => self.recover(nonce),
            rabble::Msg::User(Msg::DiskRpy(DiskRpy::NotFound)) => {
                if self.ctx.as_ref().unwrap().old_config.epoch == 0 {
                    return self.start_initial();
                }
                self.start_reconfiguration();
            }
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
