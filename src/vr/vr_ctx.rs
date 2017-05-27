// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use uuid::Uuid;
use rabble::{Pid, CorrelationId};
use rand::{thread_rng, Rng};
use slog::Logger;
use std::collections::HashSet;
use std::mem;
use std::iter::FromIterator;
use time::{Duration, SteadyTime};
use super::vrmsg::{VrMsg, ClientOp};
use super::replica::VersionedReplicas;
use super::fsm_output::FsmOutput;
use super::vr_envelope::VrEnvelope;
use super::VrBackend;
use super::prepare_requests::PrepareRequests;
use super::quorum_tracker::QuorumTracker;
use super::view_change_state::{ViewChangeState, Latest};
use super::recovery_state::{RecoveryState, RecoveryPrimary};
use super::vr_api_messages::{VrApiRsp, VrApiError};
use namespace_msg::{NamespaceMsg, NamespaceId};

pub const DEFAULT_IDLE_TIMEOUT_MS: u64 = 2000;
pub const DEFAULT_PRIMARY_TICK_MS: u64 = 500;


/// The internal state of the VR FSM shared among all states
#[derive(Debug, Clone)]
pub struct VrCtx {
    pub logger: Logger,
    pub pid: Pid,
    pub epoch: u64,
    pub view: u64,
    pub op: u64,
    pub commit_num: u64,
    pub last_received_time: SteadyTime,
    pub last_normal_view: u64,

    /// The number of replicas needed to provide quorum
    pub quorum: u64,

    pub log: Vec<ClientOp>,
    pub backend: VrBackend,
    pub old_config: VersionedReplicas,
    pub new_config: VersionedReplicas,
}

impl VrCtx {
    pub fn new(logger: Logger,
               me: Pid,
               old_config: VersionedReplicas,
               new_config: VersionedReplicas) -> VrCtx
    {
        let quorum = new_config.replicas.len() / 2 + 1;
       let idle_timeout = Duration::milliseconds(DEFAULT_IDLE_TIMEOUT_MS as i64);
        VrCtx {
            logger: logger.new(o!("component" => "vr_ctx", "node_id" => me.node.to_string())),
            pid: me,
            epoch: new_config.epoch,
            view: 0,
            op: 0,
            commit_num: 0,
            last_received_time: SteadyTime::now(),
            last_normal_view: 0,
            quorum: quorum as u64,
            log: Vec::new(),
            backend: VrBackend::new(),
            old_config: old_config,
            new_config: new_config,
        }
    }

    pub fn is_primary(&self) -> bool {
        self.primary.as_ref().map_or(false, |p| *p == self.pid)
    }

    pub fn clear_epoch_started_msgs(&mut self) {
        self.epoch_started_msgs = QuorumTracker::new(self.quorum as usize, &self.idle_timeout);
    }

    pub fn compute_primary(&self) -> Pid {
        let index = self.view as usize % self.new_config.replicas.len();
        self.new_config.replicas[index].clone()
    }

    pub fn start_state_transfer_reconfiguration(&mut self) -> Vec<FsmOutput> {
        self.last_received_time = SteadyTime::now();
        self.view = 0;
        self.op = self.commit_num;
        self.log.truncate(self.op as usize);
        self.broadcast_old_and_new(self.get_state_msg(), CorrelationId::pid(self.pid.clone()))
    }

    pub fn start_state_transfer_new_view(&mut self,
                                         new_view: u64,
                                         cid: CorrelationId,
                                         output: &mut Vec<FsmOutput>)
    {
        self.last_received_time = SteadyTime::now();
        self.view = new_view;
        self.op = self.commit_num;
        self.log.truncate(self.op as usize);
        output.push(self.send_get_state_to_random_replica(cid));
    }



    pub fn send_new_state(&mut self, op: u64, from: Pid, cid: CorrelationId) -> FsmOutput {
        FsmOutput::Vr(VrEnvelope::new(from, self.pid.clone(), self.new_state_msg(op), cid))
    }

    pub fn send_recovery_response(&self,
                                  from: Pid,
                                  nonce: Uuid,
                                  cid: CorrelationId) -> FsmOutput
    {
        let response = self.recovery_response_msg(nonce);
        FsmOutput::Vr(VrEnvelope::new(from, self.pid.clone(), response, cid))
    }

    pub fn send_get_state_to_random_replica(&self, cid: CorrelationId) -> FsmOutput {
        self.send_to_random_replica(self.get_state_msg(), cid)
    }

    fn send_to_random_replica(&self, msg: VrMsg, cid: CorrelationId) -> FsmOutput {
        let mut rng = thread_rng();
        let mut to = self.pid.clone();
        while to == self.pid {
            let index = rng.gen_range(0, self.new_config.replicas.len());
            to = self.new_config.replicas[index].clone()
        }
        FsmOutput::Vr(VrEnvelope::new(to, self.pid.clone(), msg, cid))
    }

    pub fn broadcast_start_epoch(&self, output: &mut Vec<FsmOutput>) {
        self.broadcast(self.start_epoch_msg(), CorrelationId::pid(self.pid.clone()), output)
    }

    // During reconfiguration if we are not up to date we need to send a get state request to all
    // replicas to ensure we get the latest results.
    pub fn broadcast_old_and_new(&self, msg: VrMsg,
                                 cid: CorrelationId,
                                 output: &mut Vec<FsmOutput>)
    {
        output.extend(self.old_config
                      .replicas
                      .iter()
                      .cloned()
                      .chain(self.new_config.replicas.iter().cloned())
                      .filter(|pid| *pid != self.pid)
                      .map(|pid| self.vr_new(pid, msg.clone(), cid.clone())));
    }

    /// Wrap a VrMsg in an envelope and send to all old replicas
    pub fn broadcast_old(&self,
                         msg: VrMsg,
                         cid: CorrelationId,
                         output: &mut Vec<FsmOutput>)
    {
        output.extend(self.old_config.replicas.iter().cloned()
            .filter(|pid| *pid != self.pid)
            .map(|pid| self.vr_new(pid, msg.clone(), cid.clone())));
    }

    /// Wrap a VrMsg in an envelope and send to all new replicas
    pub fn broadcast(&self,
                     msg: VrMsg,
                     cid: CorrelationId,
                     output: &mut Vec<FsmOutput>)
    {
        output.extend(self.new_config.replicas.iter().cloned()
                      .filter(|pid| *pid != self.pid)
                      .map(|pid| self.vr_new(pid, msg.clone(), cid.clone())));
    }

    pub fn has_view_change_quorum(&self) -> bool {
        self.view_change_state.as_ref().map_or(false, |s| s.has_quorum())
    }

    pub fn new_primary_commit(&mut self, last_commit_num: u64) -> Vec<FsmOutput> {
        let mut output = Vec::new();
        for i in last_commit_num..self.commit_num {
            let msg = self.log[i as usize].clone();
            match msg {
                VrMsg::ClientRequest {ref op, .. } => {
                    // The client likely hasn't reconnected, don't bother sending a reply here
                    self.backend.call(op.clone());
                },
                VrMsg::Reconfiguration {replicas, ..} => {
                    self.epoch += 1;
                    self.update_for_new_epoch(i+1, replicas);
                    output.push(self.announce_reconfiguration());
                    output.push(self.set_primary());
                    output.extend_from_slice(&self.broadcast_commit_msg_old());
                    // TODO: If we tracked VrEnvelope in the log instead of VrMsg, we would have a
                    // proper cid here.
                    let pid = self.pid.clone();
                    output.extend_from_slice(&self.broadcast_epoch_started(CorrelationId::pid(pid)));
                },
                _ => unreachable!()
            }
        }
        output
    }


    pub fn announce_reconfiguration(&self) -> FsmOutput {
        FsmOutput::Announcement(NamespaceMsg::Reconfiguration {
            namespace_id: NamespaceId(self.pid.group.as_ref().unwrap().to_string()),
            old_config: self.old_config.clone(),
            new_config: self.new_config.clone()
        }, self.pid.clone())
    }

    pub fn send_epoch_started(&mut self, envelope: VrEnvelope, output: &mut Vec<FsmOutput>) {
        let msg = self.epoch_started_msg();
        let cid = envelope.cid;
        output.push(FsmOutput::Vr(VrEnvelope::new(envelope.from, self.pid.clone(), msg, cid)));
    }

    pub fn broadcast_epoch_started(&mut self, cid: CorrelationId, output: &mut Vec<FsmOutput>) {
        let msg = self.epoch_started_msg();
        output.extend(self.replicas_to_replace().iter().cloned().map(|r| {
            FsmOutput::Vr(VrEnvelope::new(r, self.pid.clone(), msg.clone(), cid.clone()))
        }));
    }

    pub fn replicas_to_replace(&self) -> Vec<Pid> {
        let new_set = HashSet::<Pid>::from_iter(self.new_config.replicas.clone());
        let old_set = HashSet::<Pid>::from_iter(self.old_config.replicas.clone());
        old_set.difference(&new_set).cloned().collect()
    }

    pub fn send_to_primary(&self, msg: VrMsg, cid: CorrelationId) -> FsmOutput {
        FsmOutput::Vr(
            VrEnvelope::new(self.primary.as_ref().unwrap().clone(), self.pid.clone(), msg, cid)
        )
    }

    fn update_for_new_epoch(&mut self, op: u64, replicas: Vec<Pid>) {
        self.last_received_time = SteadyTime::now();
        self.view = 0;
        self.last_normal_view = 0;
        mem::swap(&mut self.old_config, &mut self.new_config);
        self.new_config = VersionedReplicas {epoch: self.epoch, op: op, replicas: replicas};
    }

    #[inline]
    /// We use a cast to i64 until the stdlib Duration that takes u64 is stabilized; It doesn't matter
    /// here since the values are so small.
    pub fn idle_timeout(&self) -> bool {
        SteadyTime::now() - self.last_received_time > self.idle_timeout
    }

    pub fn last_log_entry_is_latest_reconfiguration(&self, epoch: u64, op: u64) -> bool {
        if let VrMsg::Reconfiguration {epoch: log_epoch, ..} = self.log[(op-1) as usize] {
            if log_epoch + 1 == epoch {
                return true;
            }
        }
        false
    }

    pub fn is_leaving(&self) -> bool {
        self.replicas_to_replace().contains(&self.pid)
    }


    fn clear_primary(&mut self) -> FsmOutput {
        self.primary = None;
        let namespace_id = NamespaceId(self.pid.group.clone().unwrap());
        FsmOutput::Announcement(NamespaceMsg::ClearPrimary(namespace_id), self.pid.clone())
    }

    fn set_primary(&mut self) -> FsmOutput {
        let primary = self.compute_primary();
        self.primary = Some(primary.clone());
        FsmOutput::Announcement(NamespaceMsg::NewPrimary(primary), self.pid.clone())
    }

    pub fn reset_view_change_state(&mut self, view: u64, output: &mut Vec<FsmOutput>) {
        self.last_received_time = SteadyTime::now();
        self.view = view;
        self.view_change_state =
            Some(ViewChangeState::new(self.quorum, self.idle_timeout.clone()));
        output.push(self.clear_primary());
    }

    pub fn reset_and_start_view_change(&mut self, output: &mut Vec<FsmOutput>) {
        let view = self.view;
        self.reset_view_change_state(view + 1, output);
        self.start_view_change(output);
    }

    pub fn start_view_change(&mut self, output: &mut Vec<FsmOutput>) {
        let msg = self.start_view_change_msg();
        let cid = CorrelationId::pid(self.pid.clone());
        self.broadcast(msg, cid, output);
    }

    /// Create a new `FsmOut::Vr(..)` variant
    fn vr_new(&self, to: Pid, msg: VrMsg, cid: CorrelationId) -> FsmOutput {
        FsmOutput::Vr(VrEnvelope::new(to, self.pid.clone(), msg.clone(), cid))
    }

    /*************************************************************************/
    /*******   CONSTRUCT VR MESSAGES   ****/
    /************************************************************************/

    pub fn prepare_msg(&self, msg: VrMsg) -> VrMsg {
        Prepare {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            commit_num: self.commit_num,
            msg: Box::new(msg)
        }.into()
    }

    pub fn prepare_ok_msg(&self) -> VrMsg {
        PrepareOk {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            from: self.pid.clone()
        }.into()
    }

    pub fn new_state_msg(&self, op: u64) -> VrMsg {
        NewState {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            primary: self.primary.clone(),
            commit_num: self.commit_num,
            log_tail: (&self.log[op as usize..self.op as usize]).to_vec()
        }.into()
    }

    pub fn recovery_response_msg(&self, nonce: Uuid) -> VrMsg {
        let (op, commit_num, log) =
            if self.primary.is_some() && self.primary == Some(self.pid.clone()) {
                (Some(self.op), Some(self.commit_num), Some(self.log.clone()))
            } else {
                (None, None, None)
            };
        RecoveryResponse {
            epoch: self.epoch,
            view: self.view,
            nonce: nonce,
            from: self.pid.clone(),
            op: op,
            commit_num: commit_num,
            log: log
        }.into()
    }

    pub fn get_state_msg(&self) -> VrMsg {
        GetState {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            from: self.pid.clone()
        }.into()
    }


    pub fn start_epoch_msg(&self) -> VrMsg {
        StartEpoch {
            epoch: self.epoch,
            op: self.op,
            old_config: self.old_config.clone(),
            new_config: self.new_config.clone()
        }.into()
    }

    pub fn start_view_change_msg(&self) -> VrMsg {
        StartViewChange {
            epoch: self.epoch,
            view: self.view,
            op: self.op,
            from: self.pid.clone()
        }.into()
    }


    pub fn epoch_started_msg(&self) -> VrMsg {
        EpochStarted {
            epoch: self.epoch,
            from: self.pid.clone()
        }.into()
    }

    /*************************************************************************/
    /* End of VrMsg constructors */
    /*************************************************************************/
}
