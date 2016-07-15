//! This module contains code that acts as the dispatcher for testing purposes, in that it manages
//! fsm lifetimes and sends and receives their messages. It's used for testing a single tenant
//! with all replicas running on a single node.

use std::iter::FromIterator;
use std::collections::{HashSet, HashMap, VecDeque};
use uuid::Uuid;
use fsm::{Fsm, StateFn};
use v2r2::Member;
use v2r2::vr::{vr_fsm, Replica, VrMsg, Envelope, PeerEnvelope, Announcement, ClientEnvelope,
              ClientReplyEnvelope, VrCtx, VrTypes, VersionedReplicas};
use super::DynamicOp;

#[derive(Clone)]
pub struct Scheduler {
    pub primary: Option<Replica>,
    pub envelopes: VecDeque<Envelope>,
    pub tenant_id: Uuid,
    pub fsms: HashMap<Replica, Fsm<VrTypes>>,
    pub old_config: VersionedReplicas,
    pub new_config: VersionedReplicas
}

impl Iterator for Scheduler {
    type Item = Envelope;

    /// Return the next envelope to be sent. It can be manually sent with self.send_once().
    fn next(&mut self) -> Option<Envelope> {
        self.envelopes.pop_front()
    }
}

impl Scheduler {
    pub fn new(num_replicas: u64) -> Scheduler {
        let replicas = create_replicas(num_replicas);
        let new_config = VersionedReplicas {
            epoch: 1,
            op: 0,
            replicas: replicas.clone()
        };
        let fsms = create_fsms(replicas, new_config.clone());
        Scheduler {
            primary: None,
            envelopes: VecDeque::new(),
            tenant_id: Uuid::nil(),
            fsms: fsms,
            old_config: VersionedReplicas::new(),
            new_config: new_config
        }
    }

    // We always start in view 1 with an elected primary
    // We return the view change operation that caused the transition
    pub fn elect_initial_leader(&mut self) -> DynamicOp {
        let replica = self.fsms.iter().next().as_ref().unwrap().0.clone();
        self.send(&replica, VrMsg::Tick);
        DynamicOp::ViewChange(replica)
    }

    pub fn send(&mut self, to: &Replica, msg: VrMsg) -> Vec<ClientReplyEnvelope> {
        if let Some(ref mut fsm) = self.fsms.get_mut(&to) {
            self.envelopes.extend(fsm.send(msg));
        }
        self.send_until_empty()
    }

    #[cfg(test)]
    pub fn get_state(&self, replica: &Replica) -> Option<(&'static str, VrCtx)> {
        match self.fsms.get(replica) {
            Some(fsm) => Some(fsm.get_state()),
            None => None
        }
    }

    pub fn stop(&mut self, replica: &Replica) {
        self.fsms.remove(&replica);
    }

    pub fn restart(&mut self, replica: &Replica) {
        let mut ctx = VrCtx::new(replica.clone(),
                                 self.old_config.clone(),
                                 self.new_config.clone());
        ctx.idle_timeout_ms = 0;
        ctx.primary_tick_ms = 0;
        let fun = vr_fsm::startup_recovery;
        let mut fsm = Fsm::<VrTypes>::new(ctx, state_fn!(fun));
        self.envelopes.extend(fsm.send(VrMsg::Tick));
        println!("self.envelopes.len() = {}", self.envelopes.len());
        self.fsms.insert(replica.clone(), fsm);
        self.send_until_empty();
    }

    pub fn run(&mut self, op: &DynamicOp) {
        match *op {
            DynamicOp::ClientRequest(ref primary, ref msg) => {
                if let Some(ref mut fsm) = self.fsms.get_mut(primary) {
                    self.envelopes.extend(fsm.send(msg.clone()));
                }
            },
            DynamicOp::Commit(ref replica) | DynamicOp::ViewChange(ref replica) => {
                if let Some(ref mut fsm) = self.fsms.get_mut(replica) {
                    self.envelopes.extend(fsm.send(VrMsg::Tick));
                }
            },
            DynamicOp::Crash(ref replica) => self.stop(replica),
            DynamicOp::Restart(ref replica) => self.restart(replica),
            DynamicOp::Reconfiguration(_) => ()
        }
    }

    pub fn send_once(&mut self, envelope: Envelope) -> Option<ClientReplyEnvelope> {
        match envelope {
            Envelope::Peer(PeerEnvelope {to, msg, ..}) => self.send_direct(&to, msg),
            Envelope::Client(ClientEnvelope {to, msg}) => self.send_direct(&to, msg),
            Envelope::Announcement(announcement) => self.handle_announcement(announcement),
            Envelope::ClientReply(client_reply_envelope) => return Some(client_reply_envelope)
        }
        None
    }

    /// Directly send a vr message to an FSM
    pub fn send_direct(&mut self, to: &Replica, msg: VrMsg) {
        if let Some(ref mut fsm) = self.fsms.get_mut(&to) {
            let output = fsm.send(msg);
            self.envelopes.extend(output);
        }
    }

    pub fn send_until_empty(&mut self) -> Vec<ClientReplyEnvelope> {
        let mut replies = Vec::new();
        loop {
            match self.next() {
                Some(envelope) => {
                    if let Some(client_reply_envelope) = self.send_once(envelope) {
                        replies.push(client_reply_envelope);
                    }
                }
                None => return replies
            }
        }
    }

    /// Send all envelopes received, except those ones marked for dropping. Return any client
    /// replies.
    #[cfg(test)]
    pub fn send_until_empty_with_drop(&mut self, drop_target: &Replica) -> Vec<ClientReplyEnvelope> {
        let mut replies = Vec::new();
        loop {
            match self.next() {
                Some(envelope) => {
                    // Drop any messages from peers destined for our drop target replica
                    if let Envelope::Peer(PeerEnvelope {ref to, ..}) = envelope {
                        if *to == *drop_target {
                            continue;
                        }
                    }

                    if let Some(client_reply_envelope) = self.send_once(envelope) {
                        replies.push(client_reply_envelope);
                    }
                }
                None => return replies
            }
        }
    }

    fn handle_announcement(&mut self, announcement: Announcement) {
        match announcement {
            Announcement::Reconfiguration {old_config, new_config, ..} => {
                self.reconfigure(old_config, new_config);
            },
            Announcement::Stop(replica) => {
                self.stop(&replica);
            },
            Announcement::NewPrimary(replica) => {
                self.primary = Some(replica);
            },
            Announcement::ClearPrimary(_tenant_id) => {
                self.primary = None;
            }
        }
    }

    fn reconfigure(&mut self, old: VersionedReplicas, new: VersionedReplicas) {
        self.old_config = old.clone();
        self.new_config = new.clone();
        let new_set = HashSet::<Replica>::from_iter(new.replicas.clone());
        let old_set = HashSet::<Replica>::from_iter(old.replicas.clone());
        let mut to_start: Vec<Replica> = new_set.difference(&old_set).cloned().collect();
        for replica in to_start.drain(..) {
            let mut ctx = VrCtx::new(replica.clone(), old.clone(), new.clone());
            ctx.idle_timeout_ms = 0;
            ctx.primary_tick_ms = 0;
            let fun = vr_fsm::startup_reconfiguration;
            let fsm = Fsm::<VrTypes>::new(ctx, state_fn!(fun));
            self.fsms.insert(replica, fsm);
        }
    }
}

fn create_fsms(mut replicas: Vec<Replica>,
               new_config: VersionedReplicas) -> HashMap<Replica, Fsm<VrTypes>> {
        let mut fsms = HashMap::new();
        for replica in replicas.drain(..) {
            let mut ctx = VrCtx::new(replica.clone(),
                                     VersionedReplicas::new(),
                                     new_config.clone());
            // Set timeouts to zero, since the only time we send a tick message is when we want the
            // event to occur driven by the timeout.
            ctx.idle_timeout_ms = 0;
            ctx.primary_tick_ms = 0;
            let fun = vr_fsm::startup_new_tenant;
            let fsm = Fsm::<VrTypes>::new(ctx, state_fn!(fun));
            fsms.insert(replica, fsm);
        }
        fsms
}

fn create_replicas(num_replicas: u64) -> Vec<Replica> {
    let mut replicas = Vec::new();
    let node = Member::new_test_node("test_node");
    for i in 1..num_replicas + 1 {
        replicas.push(Replica {
            tenant: Uuid::nil(),
            name: format!("r{}", i),
            node: node.clone()
        });
    }
    replicas
}
