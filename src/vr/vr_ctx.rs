// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use rabble::{self, Pid, Envelope, CorrelationId};
use slog::{self, Logger};
use std::collections::HashSet;
use std::mem;
use std::iter::FromIterator;
use time::{Duration, SteadyTime};
use vr::vr_msg::{self, ClientOp};
use vr::VersionedReplicas;
use vr::VrBackend;
use msg::Msg;
use namespace_msg::{NamespaceMsg, NamespaceId};

pub const DEFAULT_IDLE_TIMEOUT_MS: i64 = 2000;
pub const DEFAULT_PRIMARY_TICK_MS: i64 = 500;


/// The internal state of the VR FSM shared among all states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VrCtx {
    #[serde(skip_serializing, skip_deserializing, default = "empty_logger")]
    pub logger: Logger,
    pub pid: Pid,
    pub epoch: u64,
    pub view: u64,
    pub op: u64,
    pub commit_num: u64,

    // Note that deserialization will return the wrong time.
    // This is only used in debuggin goutput though.
    #[serde(skip_serializing, skip_deserializing, default = "now")]
    pub last_received_time: SteadyTime,

    pub last_normal_view: u64,

    /// The number of replicas needed to provide quorum
    pub quorum: u64,

    pub idle_timeout_ms: i64,

    pub log: Vec<ClientOp>,

    #[serde(skip_serializing, skip_deserializing)]
    pub backend: VrBackend,

    pub old_config: VersionedReplicas,
    pub new_config: VersionedReplicas,
}

fn now() -> SteadyTime {
    SteadyTime::now()
}

fn empty_logger() -> Logger {
    Logger::root(slog::Discard, o!())
}

impl VrCtx {
    pub fn new(logger: Logger,
               me: Pid,
               old_config: VersionedReplicas,
               new_config: VersionedReplicas) -> VrCtx
    {
        let quorum = new_config.replicas.len() / 2 + 1;
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
            idle_timeout_ms: DEFAULT_IDLE_TIMEOUT_MS,
            log: Vec::new(),
            backend: VrBackend::new(),
            old_config: old_config,
            new_config: new_config,
        }
    }

    pub fn namespace_mgr_envelope(&self, msg: NamespaceMsg) -> Envelope<Msg> {
        let to = Pid {
            group: None,
            name: "namespace_mgr".to_owned(),
            node: self.pid.node.clone()
        };
        let from = self.pid.clone();
        let cid = Some(CorrelationId::pid(self.pid.clone()));
        let msg = rabble::Msg::User(Msg::Namespace(msg));
        Envelope::new(to, from, msg, cid)
    }

    pub fn is_primary(&self) -> bool {
        self.compute_primary() == self.pid
    }

    pub fn compute_primary(&self) -> Pid {
        let index = self.view as usize % self.new_config.replicas.len();
        self.new_config.replicas[index].clone()
    }

    /// Wrap a VrMsg in an envelope and send to all old replicas
    pub fn broadcast_old(&self,
                         msg: rabble::Msg<Msg>,
                         cid: CorrelationId,
                         output: &mut Vec<Envelope<Msg>>)
    {
        output.extend(self.old_config.replicas.iter().cloned()
            .filter(|pid| *pid != self.pid)
            .map(|pid| Envelope::new(pid, self.pid.clone(), msg.clone(), Some(cid.clone()))));
    }

    /// Wrap a VrMsg in an envelope and send to all new replicas
    pub fn broadcast(&self,
                     msg: rabble::Msg<Msg>,
                     cid: CorrelationId,
                     output: &mut Vec<Envelope<Msg>>)
    {
        output.extend(self.new_config.replicas.iter().cloned()
                      .filter(|pid| *pid != self.pid)
                      .map(|pid| Envelope::new(pid,
                                               self.pid.clone(),
                                               msg.clone(),
                                               Some(cid.clone()))));
    }

    pub fn announce_reconfiguration(&self, output: &mut Vec<Envelope<Msg>>) {
        output.push(self.namespace_mgr_envelope(NamespaceMsg::Reconfiguration {
            namespace_id: NamespaceId(self.pid.group.as_ref().unwrap().to_owned()),
            old_config: self.old_config.clone(),
            new_config: self.new_config.clone()
        }));
    }

    pub fn replicas_to_replace(&self) -> Vec<Pid> {
        let new_set = HashSet::<Pid>::from_iter(self.new_config.replicas.clone());
        let old_set = HashSet::<Pid>::from_iter(self.old_config.replicas.clone());
        old_set.difference(&new_set).cloned().collect()
    }

    pub fn update_for_new_epoch(&mut self, op: u64, replicas: Vec<Pid>) {
        self.last_received_time = SteadyTime::now();
        self.view = 0;
        self.last_normal_view = 0;
        mem::swap(&mut self.old_config, &mut self.new_config);
        self.new_config = VersionedReplicas {epoch: self.epoch, op: op, replicas: replicas};
        self.quorum = self.new_config.replicas.len() as u64 / 2 + 1;
    }

    /// We use a cast to i64 until the stdlib Duration that takes u64 is stabilized; It doesn't matter
    /// here since the values are so small.
    pub fn idle_timeout(&self) -> bool {
        SteadyTime::now() - self.last_received_time
            > Duration::milliseconds(self.idle_timeout_ms)
    }

    pub fn last_log_entry_is_latest_reconfiguration(&self, epoch: u64, op: u64) -> bool {
        if let ClientOp::Reconfiguration(vr_msg::Reconfiguration {epoch: log_epoch, ..})
            = self.log[(op-1) as usize]
        {
            if log_epoch + 1 == epoch {
                return true;
            }
        }
        false
    }

    pub fn is_leaving(&self) -> bool {
        self.replicas_to_replace().contains(&self.pid)
    }
}
