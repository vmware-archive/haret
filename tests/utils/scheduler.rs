// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! This module contains code that manages fsm lifetimes and sends and receives their messages. It's
//! used for testing a single namespace with all replicas running on a single node.

use std::iter::FromIterator;
use std::collections::{HashSet, HashMap, VecDeque};
use time::Duration;
use uuid::Uuid;
use slog::{DrainExt, Logger};
use slog_term;
use slog_envlogger;
use funfsm::StateFn;
use funfsm::fsm_check::Checker;
use haret::{NamespaceMsg, NamespaceId};
use haret::vr::{vr_fsm, VrMsg, VrCtx, VrTypes, VersionedReplicas, VrEnvelope, FsmOutput};
use rabble::{Pid, NodeId, CorrelationId};
use super::vr_fsm_constraints;

pub struct Scheduler {
    pub logger: Logger,
    pub pid: Pid,
    pub primary: Option<Pid>,
    pub fsm_output: VecDeque<FsmOutput>,
    pub crashed_nodes: Vec<Pid>,
    pub namespace_id: NamespaceId,
    pub fsms: HashMap<Pid, Checker<VrTypes>>,
    pub old_config: VersionedReplicas,
    pub new_config: VersionedReplicas
}

impl Iterator for Scheduler {
    type Item = FsmOutput;

    /// Return the next message to be sent. It can be manually sent with self.send_once().
    fn next(&mut self) -> Option<FsmOutput> {
        self.fsm_output.pop_front()
    }
}

impl Scheduler {
    pub fn new(num_replicas: u64) -> Scheduler {
        let node_id = NodeId {
            name: "test_node".to_string(),
            addr: "127.0.0.1:1999".to_string()
        };
        let pids = create_pids(num_replicas, node_id.clone());
        let new_config = VersionedReplicas {
            epoch: 1,
            op: 0,
            replicas: pids.clone()
        };
        let logger = logger();
        let fsms = create_fsms(pids, new_config.clone(), &logger);
        let pid = Pid {name: "scheduler".to_string(), group: None, node: node_id};
        Scheduler {
            logger: logger,
            pid: pid,
            primary: None,
            fsm_output: VecDeque::new(),
            crashed_nodes: Vec::new(),
            namespace_id: NamespaceId(Uuid::nil().to_string()),
            fsms: fsms,
            old_config: VersionedReplicas::new(),
            new_config: new_config
        }
    }

    pub fn quorum(&self) -> usize {
        self.new_config.replicas.len() / 2 + 1
    }

    // We always start in view 1 with an elected primary
    // We return the view change operation that caused the transition
    pub fn elect_initial_leader(&mut self) {
        let replica = self.fsms.iter().next().as_ref().unwrap().0.clone();
        let _ = self.send(&replica, VrMsg::Tick);
    }

    pub fn make_vr_envelope(&self, to: Pid, msg: VrMsg) -> VrEnvelope {
        VrEnvelope {
            to: to,
            from: self.pid.clone(),
            msg: msg,
            correlation_id: CorrelationId::pid(self.pid.clone())
        }
    }

    pub fn send_to_primary(&mut self, msg: VrMsg) -> Result<Vec<VrEnvelope>, String> {
        if let Some(ref pid) = self.primary.clone() {
            debug!(self.logger, "Send to primary:"; "msg" => format!("{:?}", msg));
            return self.send(pid, msg);
        }
        Ok(Vec::new())
    }

    pub fn restart_crashed_node(&mut self) -> Result<(), String> {
        if self.primary.is_none() {
            // Trigger a view change so the restarting node can recover
            debug!(self.logger, format!("View change: self.fsms.len() = {}", self.fsms.len()));
            let pid = self.fsms.iter().next().unwrap().0.clone();
            {
                let (state, _) = self.get_state(&pid).unwrap();
                debug!(self.logger, format!("first fsm state = {}", state));
            }
            self.send(&pid, VrMsg::Tick)?;
        }
        // Always start the most recently crashed node - TODO: randomize this?
        if let Some(pid) = self.crashed_nodes.pop() {
            self.restart(&pid)?;
        }
        Ok(())
    }

    pub fn stop_primary(&mut self) {
        // We never want a quorum of stopped nodes, as VR cannot recover from that
        if self.fsms.len() == self.quorum() {
            return;
        }
        if let Some(primary) = self.primary.take() {
            self.fsms.remove(&primary);
            self.crashed_nodes.push(primary);
        }
    }

    pub fn stop_backup(&mut self) {
        // We never want a quorum of stopped nodes, as VR cannot recover from that
        if self.fsms.len() == self.quorum() {
            return;
        }
        let backup = if let Some(primary) = self.primary.clone() {
            self.fsms.iter().find(|&(& ref pid, _)| *pid != primary).unwrap().0.clone()
        } else {
            self.fsms.iter().next().unwrap().0.clone()
        };
        self.fsms.remove(&backup);
        self.crashed_nodes.push(backup);
    }

    pub fn send_to_backup(&mut self, msg: VrMsg) -> Result<Vec<VrEnvelope>, String> {
        let to = if let Some(primary) = self.primary.clone() {
            self.fsms.iter().find(|&(& ref pid, _)| *pid != primary).unwrap().0.clone()
        } else {
            self.fsms.iter().next().unwrap().0.clone()
        };
        self.send(&to, msg)
    }

    pub fn send(&mut self, to: &Pid, msg: VrMsg) -> Result<Vec<VrEnvelope>, String> {
        self.send_msg(to, msg)?;
        self.send_until_empty()
    }

    pub fn get_state(&self, replica: &Pid) -> Option<(&'static str, &VrCtx)> {
        match self.fsms.get(replica) {
            Some(checker) => Some(checker.fsm.get_state()),
            None => None
        }
    }

    pub fn get_states(&self) -> Vec<(&'static str, VrCtx)> {
        self.fsms.iter().map(|(_, checker)| {
            let (state, ctx) = checker.fsm.get_state();
            (state, ctx.clone())
        }).collect()
    }

    pub fn stop(&mut self, replica: &Pid) {
        self.fsms.remove(&replica);
        self.crashed_nodes.push(replica.clone());
    }

    pub fn restart(&mut self, replica: &Pid) -> Result<Vec<VrEnvelope>, String> {
        let mut ctx = VrCtx::new(self.logger.clone(),
                                 replica.clone(),
                                 self.old_config.clone(),
                                 self.new_config.clone());
        ctx.idle_timeout = Duration::zero();
        ctx.primary_tick_ms = 0;
        // A `Checker` wraps the fsm, so invariants can be checked during operation
        let mut checker = new_checker(ctx, state_fn!(vr_fsm::startup_recovery));
        let envelope = self.make_vr_envelope(replica.clone(), VrMsg::Tick);
        self.fsm_output.extend(checker.check(envelope)?);
        self.fsms.insert(replica.clone(), checker);
        self.send_until_empty()
    }

    /// Wrap a VrMsg in a VrEnvelope sent from the scheduler and send to an fsm
    pub fn send_msg(&mut self, to: &Pid, msg: VrMsg) -> Result<(), String> {
        let envelope = self.make_vr_envelope(to.clone(), msg);
        if let Some(ref mut checker) = self.fsms.get_mut(to) {
            self.fsm_output.extend(checker.check(envelope)?);
        }
        Ok(())
    }

    /// Send a VrEnvelope to an fsm
    pub fn send_envelope(&mut self, envelope: VrEnvelope) -> Result<(), String> {
        if let Some(ref mut checker) = self.fsms.get_mut(&envelope.to) {
            self.fsm_output.extend(checker.check(envelope)?);
        }
        Ok(())
    }

    pub fn send_until_empty(&mut self) -> Result<Vec<VrEnvelope>, String> {
        let mut replies = Vec::new();
        loop {
            let msg = self.next();
            debug!(self.logger, "send_until_empty"; "msg" => format!("{:#?}", msg));
            match msg {
                Some(FsmOutput::Vr(ref envelope)) if envelope.to == self.pid =>
                    replies.push(envelope.clone()),
                Some(FsmOutput::Vr(envelope)) =>
                    self.send_envelope(envelope)?,
                Some(FsmOutput::Announcement(namespace_msg, _)) => {
                    self.handle_announcement(namespace_msg);
                },
                None => return Ok(replies)
            }
        }
    }

    /// Send all envelopes received, except those ones marked for dropping. Return any client
    /// replies.
    #[cfg(test)]
    pub fn send_until_empty_with_drop(&mut self,
                                      drop_target: &Pid) -> Result<Vec<VrEnvelope>, String>
    {
        let mut replies = Vec::new();
        loop {
            match self.next() {
                Some(FsmOutput::Vr(ref envelope)) if envelope.to == self.pid =>
                    replies.push(envelope.clone()),
                Some(FsmOutput::Vr(ref envelope)) if envelope.to == *drop_target =>
                    // Drop any messages from peers destined for our drop target replica
                    (),
                Some(FsmOutput::Vr(envelope)) =>
                    self.send_envelope(envelope)?,
                Some(FsmOutput::Announcement(namespace_msg, _)) =>
                    self.handle_announcement(namespace_msg),
                None => return Ok(replies)
            }
        }
    }

    fn handle_announcement(&mut self, namespace_msg: NamespaceMsg) {
        match namespace_msg {
            NamespaceMsg::Reconfiguration {old_config, new_config, ..} => {
                self.reconfigure(old_config, new_config);
            },
            NamespaceMsg::Stop(replica) => {
                self.stop(&replica);
            },
            NamespaceMsg::NewPrimary(replica) => {
                self.primary = Some(replica);
            },
            NamespaceMsg::ClearPrimary(_tenant_id) => {
                self.primary = None;
            },
            _ => unreachable!()
        }
    }

    fn reconfigure(&mut self, old: VersionedReplicas, new: VersionedReplicas) {
        self.old_config = old.clone();
        self.new_config = new.clone();
        let new_set = HashSet::<Pid>::from_iter(new.replicas.clone());
        let old_set = HashSet::<Pid>::from_iter(old.replicas.clone());
        let to_start: Vec<Pid> = new_set.difference(&old_set).cloned().collect();
        for replica in to_start {
            let mut ctx =
                VrCtx::new(self.logger.clone(), replica.clone(), old.clone(), new.clone());
            ctx.idle_timeout = Duration::zero();
            ctx.primary_tick_ms = 0;
            if self.fsms.contains_key(&replica) { return; }
            let checker = new_checker(ctx, state_fn!(vr_fsm::startup_reconfiguration));
            self.fsms.insert(replica, checker);
        }
    }
}

fn create_fsms(pids: Vec<Pid>,
               new_config: VersionedReplicas,
               logger: &Logger) -> HashMap<Pid, Checker<VrTypes>>
{
    let mut fsms = HashMap::new();
    for pid in pids {
        let mut ctx =
            VrCtx::new(logger.clone(), pid.clone(), VersionedReplicas::new(), new_config.clone());
        // Set timeouts to zero, since the only time we send a tick message is when we want the
        // event to occur driven by the timeout.
        ctx.idle_timeout = Duration::zero();
        ctx.primary_tick_ms = 0;
        let checker = new_checker(ctx, state_fn!(vr_fsm::startup_new_namespace));
        fsms.insert(pid, checker);
    }
    fsms
}

// A `Checker` wraps an fsm, so invariants can be checked during operation
fn new_checker(ctx: VrCtx, fun: StateFn<VrTypes>) -> Checker<VrTypes> {
    let constraints = vr_fsm_constraints::constraints();
    Checker::<VrTypes>::new(ctx, fun, constraints)
}

fn create_pids(num_replicas: u64, node_id: NodeId) -> Vec<Pid> {
    (1..num_replicas+1).into_iter().map(|i| Pid {
        group: Some(Uuid::nil().to_string()),
        name: format!("r{}", i),
        node: node_id.clone()
    }).collect()
}

/// Set up logging to go to the terminal and be configured via `RUST_LOG`
fn logger() -> Logger {
    let drain = slog_term::streamer().async().full().build();
    Logger::root(slog_envlogger::EnvLogger::new(drain.fuse()), o!())
}

