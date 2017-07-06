// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use rabble::Pid;
use haret::vr::{ClientOp, VrState};
use super::arbitrary::{Op, ClientRequest};

/// This is a model of the state of a VR replica set under test. It doesn't represent a particular
/// replica, but rather properties of the system as it transitions through various states and
/// messages.
pub struct Model {
    client_req_num: u64,
    replicas: Vec<Pid>,
    quorum: u64,
    crashed: u64,
    possible_views: Vec<u64>,

    possible_primary_commit_num: Vec<u64>,
    possible_backup_commit_num: Vec<u64>,
    possible_primary_min_accept: Vec<u64>,
    possible_backup_min_accept: Vec<u64>,

    // This is the log at every alive replica, since messages are not dropped and there are no
    // partitions in the current model
    log: Vec<ClientOp>
}

impl Model {

    /// Create a new model
    pub fn new(mut replicas: Vec<Pid>) -> Model {
        replicas.sort();
        Model {
            client_req_num: 0,
            quorum: replicas.len() as u64 / 2 + 1,
            replicas: replicas,
            crashed: 0,
            possible_views: vec![0],
            possible_primary_commit_num: vec![0],
            possible_backup_commit_num: vec![0],
            possible_primary_min_accept: vec![0],
            possible_backup_min_accept: vec![0],
            log: Vec::new()
        }
    }

    /// Update the model as a result of an operation
    pub fn update(&mut self, op: Op) {
        match op {
            Op::ClientRequest(ClientRequest(mut req)) => {
                self.client_req_num += 1;
                req.request_num = self.client_req_num;
                self.log.push(req.into());

                self.possible_backup_commit_num = self.possible_primary_commit_num.clone();

                // In our current model we always commit operations
                self.possible_primary_commit_num = vec![self.log.len() as u64];

                self.possible_backup_min_accept = self.possible_primary_min_accept.clone();

                if self.crashed == 0 {
                    self.possible_primary_min_accept = vec![self.log.len() as u64];
                }
            }
            Op::Commit => {
                self.possible_backup_commit_num = self.possible_primary_commit_num.clone();

                // A prepare will be rebroadcast and commit if there are uncommitted ops
                self.possible_primary_commit_num = vec![self.log.len() as u64];
                self.possible_backup_min_accept = self.possible_primary_min_accept.clone();
            }
            Op::ViewChange => {
                self.update_view();
                self.possible_backup_commit_num.extend(&self.possible_primary_commit_num);
                self.possible_backup_commit_num.sort();
                self.possible_backup_commit_num.dedup();
                self.possible_primary_commit_num = self.possible_backup_commit_num.clone();

                self.possible_backup_min_accept.extend(&self.possible_primary_min_accept);
                self.possible_backup_min_accept.sort();
                self.possible_backup_min_accept.dedup();
                self.possible_primary_min_accept = self.possible_backup_min_accept.clone();
            }
            Op::CrashBackup => {
                if self.replicas.len() as u64 - self.crashed != self.quorum {
                    self.crashed += 1;
                }
            }
            Op::CrashPrimary => {
                if self.replicas.len() as u64 - self.crashed != self.quorum {
                    self.crashed += 1;
                    self.update_view();
                    self.possible_primary_commit_num = self.possible_backup_commit_num.clone();
                    self.possible_primary_min_accept = self.possible_backup_min_accept.clone();
                }
            }
            Op::Restart => {
                // A restarting replica will learn of the primary commit num. Therefore backups may
                // diverge.
                self.possible_backup_commit_num.extend(&self.possible_primary_commit_num);
                self.possible_backup_commit_num.sort();
                self.possible_backup_commit_num.dedup();

                // A restarting replica will learn of the primary min accept, therefore backups may
                // diverge.
                self.possible_backup_min_accept.extend(&self.possible_primary_min_accept);
                self.possible_backup_min_accept.sort();
                self.possible_backup_min_accept.dedup();

                if self.crashed == 1 {
                    // Recovering replicas send PrepareOk to the primary allowing it to learn of the
                    // latest accepted prepare
                    self.possible_primary_min_accept = vec![self.log.len() as u64];
                }

                if self.crashed != 0 {
                    self.crashed -= 1;
                }
            }
        }
    }

    /// Compare the model with the actual states of the replicas.
    ///
    /// Note that this method is mutable because some members contain multiple possible values that
    /// can be collapsed during checking to known values.
    pub fn check(&mut self, states: &[VrState]) -> Result<(), String> {
        for state in states {
           safe_assert!(self.possible_views.contains(&state.ctx().view))?;
           self.possible_views = vec![state.ctx().view];
           safe_assert_eq!(self.log, state.ctx().log)?;
           match *state {
               VrState::Primary(_) => {
                   assert_contains!(self.possible_primary_commit_num, &state.ctx().commit_num)?;
                   // Reset the possible primary value with the known result
                   self.possible_primary_commit_num = vec![state.ctx().commit_num];

                   assert_contains!(self.possible_primary_min_accept,
                                    &state.ctx().global_min_accept)?;
                   // Reset the possible primary value with the known result
                   self.possible_primary_min_accept = vec![state.ctx().global_min_accept];
               },
               VrState::Backup(_) => {
                   assert_contains!(self.possible_backup_commit_num, &state.ctx().commit_num)?;
                   assert_contains!(self.possible_backup_min_accept,
                                    &state.ctx().global_min_accept)?;
               },
               _ => fail!()
           }
        }
        Ok(())
    }

    /// With no partitions, at each update there is only going to be one view,
    /// because at each call to check, the possible views will be collapsed to a known
    /// view.
    fn update_view(&mut self) {
        assert_eq!(self.possible_views.len(), 1);
        let mut view = self.possible_views.pop().unwrap();
        view += 1;
        self.possible_views.push(view);
        for _ in 0..self.crashed {
            view += 1;
            self.possible_views.push(view);
        }
    }
}
