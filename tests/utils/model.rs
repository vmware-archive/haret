//! This is a model that represents the state of a VR replica group. It's used to validate
//! actual state of the replicas during fuzz testing. All the other assertions during fuzz testing
//! operate on the current state of the replicas and result of the most recent operation. The model
//! maintains a history of operations to allow for even more checks against the current replica
//! states to ensure consistent operation.

use std::collections::{HashMap, HashSet};
use rand::{thread_rng, ThreadRng};
use rand::distributions::{IndependentSample, Range};
use uuid::Uuid;
use v2r2::vr::{Dispatcher, Replica, VrMsg, ClientReplyEnvelope};
use v2r2::vr_api::{Element, VrBackend, VrApiReq};
use super::TestMsg;

#[derive(Debug, Clone)]
struct BackupState {
    view: u64,
    op: u64,
    state: &'static str
}

impl BackupState {
    pub fn new() -> BackupState {
        BackupState {
            view: 1,
            op: 0,
            state: "backup"
        }
    }
}

#[derive(Debug, Clone)]
pub struct Model {
    pub replicas: Vec<Replica>,
    pub primary: Option<Replica>,
    pub crashed_replicas: Vec<(Replica, Uuid)>,
    pub live_replicas: Vec<Replica>,
    // backend, view, op, commit_num are all the values on the primary
    // No need to re-implement the backend for the model
    backend: VrBackend,
    view: u64,
    op: u64,
    commit_num: u64,
    quorum: usize,
    last_msg: TestMsg,
    backup_states: HashMap<Replica, BackupState>
}

impl Model {
    pub fn new(replicas: Vec<Replica>, view: u64) -> Model {
        let primary = compute_primary(1, &replicas);
        let backups = init_backups(&primary, &replicas);
        let quorum = replicas.len() / 2 + 1;
        Model {
            replicas: replicas.clone(),
            primary: Some(primary),
            crashed_replicas: Vec::new(),
            live_replicas: replicas,
            backend: VrBackend::new(),
            view: view,
            op: 0,
            commit_num: 0,
            quorum: quorum,
            last_msg: TestMsg::Null,
            backup_states: backups
        }
    }

    /// Client requests and commits only arrive when there is a primary. We can enforce this by
    /// always having tests get the current primary from the model. Furthermore, other messages only
    /// arrive at alive replicas. This simplifies the test code with no apparent loss of protocol
    /// coverage, since messages to a crashed replica would just be dropped anyway. When testing a
    /// client of course, not using an oracle to find the primary would be the proper course of
    /// action.
    pub fn update(&mut self, msg: TestMsg) {
        match msg {
            TestMsg::ClientRequest(VrMsg::ClientRequest {ref op, ..}) => {
                self.op += 1;
                self.commit_num += 1;
                self.backend.call(self.op, op.clone());
                self.do_backup_prepare();
            },
            TestMsg::Commit => {},
            TestMsg::ViewChange(_) => self.do_view_change(),
            TestMsg::Crash(ref replica, uuid) => {
                let to_crash = self.live_replicas.pop().unwrap();
                assert_eq!(to_crash, *replica);
                self.crashed_replicas.push((to_crash, uuid));
                if self.primary.is_some() && *replica == *(self.primary.as_ref().unwrap()) {
                    self.primary = None;
                } else {
                    self.backup_states.remove(&replica);
                }
            },
            TestMsg::Restart(ref replica, ref uuid) => {
                match self.crashed_replicas.pop() {
                    Some((to_restart, crash_uuid)) => {
                        assert_eq!(crash_uuid, *uuid);
                        assert_eq!(to_restart, *replica);
                        self.live_replicas.push(to_restart);
                        let mut restarted = BackupState::new();
                        // We are currently in a view change (trying to elect the restarting node),
                        // therefore it can't recover yet, so it sits in the recovery state.
                        if self.primary.is_none() {
                            // The restarted replica is the current primary. In this case the backups
                            // will start a view change when they see the recovery message
                            // TODO: FIXME FIXME UNCOMMENT - this is just to test shrinking
/*                            if *replica == compute_primary(self.view, &self.replicas) {
                                self.do_view_change();
                            }
                            */
                            restarted.view = 0;
                            restarted.op = 0;
                            restarted.state = "recovery";
                        } else {
                            // We have recovered since there are no dropped messages and there is a
                            // current primary.
                            restarted.view = self.view;
                            restarted.op = self.op;
                            restarted.state = "backup";
                        }
                        self.backup_states.insert(replica.clone(), restarted);
                    },
                    None => ()
                }
            },
            _ => unreachable!()
        };
        self.last_msg = msg;
    }

    fn do_view_change(&mut self) {
        let new_primary = compute_primary(self.view + 1, &self.replicas);
        if self.is_crashed_or_recovering(&new_primary) {
            self.save_primary_as_backup();
            self.do_backup_view_change(None);
            self.primary = None;
        } else {
            self.save_primary_as_backup();
            self.do_backup_view_change(Some(&new_primary));
            self.backup_states.remove(&new_primary);
            self.primary = Some(new_primary);
        }
        self.view += 1;
    }

    pub fn check(&self, dispatcher: &Dispatcher) -> Result<(), String> {
        match self.last_msg {
            TestMsg::ClientRequest(VrMsg::ClientRequest {ref op, ..}) =>
                self.assert_last_op_matches_primary_state(op.clone(), dispatcher),
            TestMsg::Commit => Ok(()),
            TestMsg::ViewChange(_) => self.assert_view_change(dispatcher),
            TestMsg::Crash(_, _) => {
                // A crash doesn't cause any messages/side effects on it's own.
                // It may affect the next message though.
                Ok(())
            },
            TestMsg::Restart(ref replica, _) => self.assert_replica_state(dispatcher, replica),
            TestMsg::Null => Ok(()),
            _ => unreachable!()
        }
    }

    /// During shrinking some messages are invalid in a history and should be dropped before
    /// processing. This message determines what to drop.
    pub fn should_drop(&self, msg: &TestMsg) -> bool {
        match *msg {
            TestMsg::ClientRequest(_) => self.primary.is_none(),
            TestMsg::Commit => self.primary.is_none(),
            TestMsg::ViewChange(ref replica) => {
                // Don't send a view change tick to the primary
                self.primary.is_some() && *(self.primary.as_ref().unwrap()) == *replica
            },
            // Crashes and restarts are already covered by  causal ids
            _ => false
        }
    }

    fn assert_last_op_matches_primary_state(&self,
                                            op: VrApiReq,
                                            dispatcher: &Dispatcher) -> Result<(), String>
    {
        let path = op.get_path();
        let model_element = self.backend.tree.get(&path);
        let (state, ctx) = dispatcher.get_state(&self.primary.as_ref().unwrap()).unwrap();
        let _ = safe_assert_eq!(state, "primary");
        let primary_element = ctx.backend.tree.get(&path);
        match primary_element {
            None => safe_assert!(model_element.is_none()),
            Some(element) => safe_assert_eq!(*model_element.unwrap(), *element)
        }
    }

    fn assert_replica_state(&self, dispatcher: &Dispatcher, replica: &Replica)
        -> Result<(), String>
    {
        let backup_state = self.backup_states.get(&replica).clone().unwrap();
        let (state, ctx) = dispatcher.get_state(replica).unwrap();
        safe_assert_eq!(backup_state.state, state)
    }

    fn assert_view_change(&self, dispatcher: &Dispatcher) -> Result<(), String> {
        for (replica, backup_state) in self.backup_states.iter() {
            let (state, ctx) = dispatcher.get_state(replica).unwrap();
            if self.primary.is_some() && state == "backup" {
                let _ = safe_assert_eq!(ctx.primary, self.primary);
            }
            let _ = safe_assert_eq!(state, backup_state.state, ctx.me.name);
            let _ = safe_assert_eq!(ctx.view, backup_state.view, ctx.me.name);
            safe_assert_eq!(ctx.op, backup_state.op);
        }
        Ok(())
    }

    /// Assume no dropped messages for now (all messages commit)
    fn do_backup_prepare(&mut self) {
        for (_, state) in self.backup_states.iter_mut() {
            if (state.state != "recovery") {
                state.op += 1;
            }
        }
    }

    /// Assume no dropped messages for now (all messages commit)
    fn do_backup_view_change(&mut self, primary: Option<&Replica>) {
        for (replica, state) in self.backup_states.iter_mut() {
            if state.state != "recovery" {
                state.view += 1;
            }
            // If we don't have a primary the view change is unsuccessful
            if primary.is_none() {
                // A view change doesn't take a replica out of recovery
                if state.state != "recovery" {
                    state.state = "view_change"
                }
            } else {
                // A view change doesn't take a replica out of recovery
                if state.state != "recovery" {
                    state.state = "backup"
                }
            }
        }
    }

    fn save_primary_as_backup(&mut self) {
        if self.primary == None { return; }
        let state = BackupState {
            view: self.view,
            op: self.op,
            state: "backup"
        };
        self.backup_states.insert(self.primary.clone().unwrap(), state);
    }

    /// Choose a backup that isn't crashed or in recovery
    pub fn choose_live_backup(&self) -> Replica {
        loop {
            let range = Range::new(0, self.backup_states.len());
            let index = range.ind_sample(&mut thread_rng());
            let v: Vec<_> = self.backup_states.iter().collect();
            if (v[index].1.state != "recovery") {
                return v[index].0.clone();
            }
        }
    }

    pub fn choose_live_replica(&self) -> Option<Replica> {
        if self.live_replicas.len() > self.quorum {
            self.live_replicas.iter().cloned().last()
        } else {
            None
        }
    }

    pub fn choose_crashed_replica(&self) -> Option<(Replica, Uuid)> {
        self.crashed_replicas.iter().cloned().last()
    }

    fn is_crashed(&self, replica: &Replica) -> bool {
        for &(ref r, _) in &self.crashed_replicas {
            if *r == *replica { return true; }
        }
        false
    }

    fn is_crashed_or_recovering(&self, replica: &Replica) -> bool {
        if self.is_crashed(replica) { return true; }
        match self.backup_states.get(replica) {
            None => {
                let msg = format!("Replica {} not a backup in model", replica.name);
                assert!(false, msg);
                false
            },
            Some(state) => state.state == "recovery"
        }
    }
}

fn compute_primary(view: u64, replicas: &Vec<Replica>) -> Replica {
    let index = view as usize % replicas.len();
    replicas[index].clone()
}

fn init_backups(primary: &Replica, replicas: &Vec<Replica>) -> HashMap<Replica, BackupState> {
    let mut backups = HashMap::new();
    for r in replicas {
        if r != primary {
            backups.insert(r.clone(), BackupState::new());
        }
    }
    backups
}
