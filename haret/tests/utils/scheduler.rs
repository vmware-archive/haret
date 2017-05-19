// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! This module contains code that manages fsm lifetimes and sends and receives their messages. It's
//! used for testing a single namespace with all replicas running on a single node.

use std::iter::FromIterator;
use std::collections::{HashSet, HashMap};
use uuid::Uuid;
use slog::{DrainExt, Logger};
use slog_term;
use slog_envlogger;
use haret::{NamespaceMsg, NamespaceId, Msg};
use haret::vr::{VrState, VrMsg, VrCtx, VersionedReplicas};
use haret::vr::states::{Primary, Backup, Recovery, Reconfiguration};
use rabble::{self, Pid, NodeId, CorrelationId, Envelope};

pub struct Scheduler {
    pub logger: Logger,
    pub pid: Pid,
    pub nonce: u64,
    pub namespace_mgr: Pid,
    pub primary: Option<Pid>,
    pub fsm_output: Vec<Envelope<Msg>>,
    pub crashed: Vec<Pid>,
    pub namespace_id: NamespaceId,
    pub fsms: HashMap<Pid, Option<VrState>>,
    pub old_config: VersionedReplicas,
    pub new_config: VersionedReplicas
}

impl Iterator for Scheduler {
    type Item = Envelope<Msg>;

    /// Return the next message to be sent. It can be manually sent with self.send_once().
    /// Use a vec as a queue. We could have used a vecdeque easier, but that is not what each
    /// state transition takes as an argument
    fn next(&mut self) -> Option<Envelope<Msg>> {
        if self.fsm_output.is_empty() {
            return None;
        }
        Some(self.fsm_output.remove(0))
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
        let namespace_mgr = Pid {
            group: None,
            name: "namespace_mgr".to_string(),
            node: node_id.clone()
        };
        let logger = logger();
        let primary = pids[0].clone();
        let fsms = create_fsms(pids, new_config.clone(), &logger);
        let pid = Pid {name: "scheduler".to_string(), group: None, node: node_id};
        Scheduler {
            logger: logger,
            pid: pid,
            nonce: 1,
            namespace_mgr: namespace_mgr,
            primary: Some(primary),
            fsm_output: Vec::new(),
            crashed: Vec::new(),
            namespace_id: NamespaceId(Uuid::nil().to_string()),
            fsms: fsms,
            old_config: VersionedReplicas::new(),
            new_config: new_config
        }
    }

    pub fn quorum(&self) -> usize {
        self.new_config.replicas.len() / 2 + 1
    }

    pub fn send_to_primary(&mut self, msg: VrMsg) -> Vec<Envelope<Msg>> {
        if let Some(ref pid) = self.primary.clone() {
            debug!(self.logger, "Send to primary:"; "pid" => pid.to_string(), "msg" => format!("{:?}", msg));
            return self.send(pid, msg);
        }
        Vec::new()
    }

    pub fn restart_crashed_node(&mut self) -> Option<Pid> {
        // Always start the most recently crashed node - TODO: randomize this?
        if let Some(pid) = self.crashed.pop() {
            self.restart(&pid);
            return Some(pid);
        }
        None
    }

    pub fn stop_primary(&mut self) {
        // We never want a quorum of stopped nodes, as VR cannot recover from that
        if self.fsms.len() == self.quorum() {
            return;
        }
        if let Some(primary) = self.primary.take() {
            self.fsms.remove(&primary);
            self.crashed.push(primary);
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
        debug!(self.logger, "Crash backup:"; "pid" => backup.to_string());
        self.fsms.remove(&backup);
        self.crashed.push(backup);
    }

    pub fn send_to_backup(&mut self, msg: VrMsg) -> Vec<Envelope<Msg>> {
        let to = if self.primary.is_some() {
            self.fsms.iter().find(|&(& ref pid, _)| {
                *pid != *self.primary.as_ref().unwrap()
            }).unwrap().0.clone()
        } else {
            self.fsms.iter().next().unwrap().0.clone()
        };
        debug!(self.logger, "Send to backup:"; "pid" => to.to_string(),
                                               "msg" => format!("{:?}", msg));
        self.send(&to, msg)
    }

    pub fn send(&mut self, to: &Pid, msg: VrMsg) -> Vec<Envelope<Msg>> {
        self.send_msg(to, msg);
        self.send_until_empty()
    }

    pub fn get_state(&self, replica: &Pid) -> Option<VrState> {
        match self.fsms.get(replica) {
            Some(state) => Some(state.clone().unwrap()),
            None => None
        }
    }

    pub fn get_states(&self) -> Vec<VrState> {
        self.fsms.iter().map(|(_, state)| state.clone().unwrap()).collect()
    }

    pub fn stop(&mut self, replica: &Pid) {
        self.fsms.remove(&replica);
        self.crashed.push(replica.clone());
    }

    pub fn restart(&mut self, replica: &Pid) -> Vec<Envelope<Msg>> {
        let mut ctx = VrCtx::new(self.logger.clone(),
                                 replica.clone(),
                                 self.old_config.clone(),
                                 self.new_config.clone());
        ctx.idle_timeout_ms = 0;
        let state = VrState::Recovery(Recovery::new(ctx, self.nonce));
        self.nonce += 1;
        let from = self.pid.clone();
        let cid = CorrelationId::pid(self.pid.clone());
        let state = state.next(VrMsg::Tick, from, cid, &mut self.fsm_output);
        self.fsms.insert(replica.clone(), Some(state));
        self.send_until_empty()
    }

    /// Send a message to an fsm
    pub fn send_msg(&mut self, to: &Pid, msg: VrMsg) {
        let from = self.pid.clone();
        let cid = CorrelationId::pid(self.pid.clone());
        if let Some(state) = self.fsms.get_mut(to) {
            let vr_state = state.take().unwrap();
            let vr_state = vr_state.next(msg, from, cid, &mut self.fsm_output);
            *state = Some(vr_state);
        }
    }

    /// Send a msg to an fsm after extracting the relevant parts from an envelope
    pub fn send_envelope(&mut self, envelope: Envelope<Msg>) {
        if let Some(state) = self.fsms.get_mut(&envelope.to) {
            let vr_state = state.take().unwrap();
            if let rabble::Msg::User(Msg::Vr(vrmsg)) = envelope.msg {
                let vr_state = vr_state.next(vrmsg,
                                             envelope.from,
                                             envelope.correlation_id.unwrap(),
                                             &mut self.fsm_output);
                *state = Some(vr_state);
            } else {
                unreachable!()
            }
        }
    }

    pub fn send_until_empty(&mut self) -> Vec<Envelope<Msg>> {
        let mut replies = Vec::new();
        while let Some(envelope) = self.next() {
            debug!(self.logger, "send_until_empty"; "envelope" => format!("{:#?}", envelope));
            let to = envelope.to.clone();
            match to {
                ref pid if *pid == self.pid => replies.push(envelope),
                ref pid if *pid == self.namespace_mgr => self.handle_announcement(envelope.msg),
                _ => self.send_envelope(envelope)
            }
        }
        replies
    }

    /// Send all envelopes received, except those ones marked for dropping. Return any client
    /// replies.
    #[cfg(test)]
    pub fn send_until_empty_with_drop(&mut self, drop_target: &Pid) -> Vec<Envelope<Msg>> {
        let mut replies = Vec::new();
        while let Some(envelope) = self.next() {
            let to = envelope.to.clone();
            match to {
                ref pid if *pid == self.pid => replies.push(envelope),
                ref pid if *pid == *drop_target => (), // Drop any messages from peers destined for drop target
                ref pid if *pid == self.namespace_mgr => self.handle_announcement(envelope.msg),
                _ => self.send_envelope(envelope)
            }
        }
        replies
    }

    fn handle_announcement(&mut self, msg: rabble::Msg<Msg>) {
        if let rabble::Msg::User(Msg::Namespace(namespace_msg)) = msg {
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
        } else {
            unreachable!()
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
            ctx.idle_timeout_ms = 0;
            if self.fsms.contains_key(&replica) { return; }
            self.fsms.insert(replica, Some(VrState::Reconfiguration(Reconfiguration::new(ctx))));
        }
    }
}

fn create_fsms(pids: Vec<Pid>,
               new_config: VersionedReplicas,
               logger: &Logger) -> HashMap<Pid, Option<VrState>>
{
    let mut fsms = HashMap::new();
    for pid in pids {
        let mut ctx =
            VrCtx::new(logger.clone(), pid.clone(), VersionedReplicas::new(), new_config.clone());
        // Set timeouts to zero, since the only time we send a tick message is when we want the
        // event to occur driven by the timeout.
        ctx.idle_timeout_ms = 0;
        if pid == ctx.compute_primary() {
            fsms.insert(pid, Some(Primary::new(ctx, 0).into()));
        } else {
            fsms.insert(pid, Some(Backup::new(ctx).into()));
        }
    }
    fsms
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

