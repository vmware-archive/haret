// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use std::iter::FromIterator;
use std::fmt;
use rabble::{self, Pid, Envelope, CorrelationId};
use slog::{self, Logger};
use time::{Duration, SteadyTime};
use vr::vr_msg::ClientOp;
use vr::VersionedReplicas;
use vr::VrBackend;
use msg::Msg;
use namespace_msg::NamespaceMsg;

pub const DEFAULT_IDLE_TIMEOUT_MS: i64 = 2000;
pub const DEFAULT_PRIMARY_IDLE_TIMEOUT_MS: i64 = 500;

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

    /// The minimum accepted prepare request at all replicas
    /// Used for the view change optimizations detailed in RFC 2
    pub global_min_accept: u64,

    // Note that deserialization will return the wrong time.
    // This is only used in debuggin goutput though.
    #[serde(skip_serializing, skip_deserializing, default = "now")]
    pub last_received_time: SteadyTime,

    pub last_normal_view: u64,

    /// The number of replicas needed to provide quorum
    pub quorum: u64,

    /// This is the time a backup waits before starting a view change.
    /// It is also used in other states as a timeout
    /// It should be smaller than primary_idle_timeout_ms;
    pub idle_timeout_ms: i64,

    /// This is the time a primary waits for client requests before sending commit messages
    /// It should be larger than idle_timeout_ms;
    ///
    /// It's only in this module instead of the primary state because its configuration
    /// should travel across state transitions, and the VrCtx is the only part of the VR state that
    /// allows this.
    pub primary_idle_timeout_ms: i64,

    /// This is either 0 or the op number of the last truncated entry in the log
    pub log_start: u64,

    /// The tail of the log starting with the entry in `log_start`.
    pub log: Vec<ClientOp>,

    #[serde(skip_serializing, skip_deserializing)]
    pub backend: VrBackend,

    pub old_config: VersionedReplicas,
    pub new_config: VersionedReplicas,
}

impl fmt::Display for VrCtx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pid: {}\n", self.pid)?;
        write!(f, "epoch: {}, view: {}, op: {}, commit: {}, global_min_accept: {}\n",
               self.epoch, self.view, self.op, self.commit_num, self.global_min_accept)?;
        write!(f, "last_normal_view: {}, quorum: {}, log_start: {}, log len: {}\n",
               self.last_normal_view, self.quorum, self.log_start, self.log.len())
    }
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
            global_min_accept: 0,
            last_received_time: SteadyTime::now(),
            last_normal_view: 0,
            quorum: quorum as u64,
            idle_timeout_ms: DEFAULT_IDLE_TIMEOUT_MS,
            primary_idle_timeout_ms: DEFAULT_PRIMARY_IDLE_TIMEOUT_MS,
            log_start: 0,
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

    pub fn replicas_to_replace(&self) -> Vec<Pid> {
        let new_set = HashSet::<Pid>::from_iter(self.new_config.replicas.clone());
        let old_set = HashSet::<Pid>::from_iter(self.old_config.replicas.clone());
        old_set.difference(&new_set).cloned().collect()
    }

    /// We use a cast to i64 until the stdlib Duration that takes u64 is stabilized; It doesn't matter
    /// here since the values are so small.
    pub fn idle_timeout(&self) -> bool {
        SteadyTime::now() - self.last_received_time
            > Duration::milliseconds(self.idle_timeout_ms)
    }

    pub fn is_leaving(&self) -> bool {
        self.replicas_to_replace().contains(&self.pid)
    }

    pub fn append_log_tail(&mut self, log_start: u64, log_tail: Vec<ClientOp>) {
        if self.log_start > log_start {
            // On this replica some operations have already committed and been truncated.
            // This replica must have been the previous primary.
            // Only append later log entries.
           let start = (self.log_start - log_start) as usize;
           self.log.extend_from_slice(&log_tail[start..]);
        } else {
            let keep = (log_start - self.log_start) as usize;
            self.global_min_accept = log_start;
            self.log.truncate(keep);
            self.log.extend(log_tail);
        }
    }

    pub fn get_log_tail(&self) -> Vec<ClientOp> {
        let start = (self.global_min_accept - self.log_start) as usize;
        self.log[start..].to_vec()
    }
}

#[cfg(test)]
mod tests {
    extern crate slog;
    extern crate slog_stdlog;
    extern crate slog_term;
    extern crate slog_envlogger;

    use super::*;
    use rabble::{Pid, NodeId};
    use vr::{VersionedReplicas, VrApiReq, TreeOp, NodeType};
    use vr::vr_msg::{ClientOp, ClientRequest};
    use slog::{DrainExt, Logger};

    /// Set up logging to go to the terminal and be configured via `RUST_LOG`
    fn logger() -> Logger {
        let drain = slog_term::streamer().async().full().build();
        Logger::root(slog_envlogger::EnvLogger::new(drain.fuse()), o!())
    }

    fn test_pid() -> Pid {
        Pid {
            group: None,
            name: "test-pid".to_owned(),
            node: node_id()
        }
    }

    fn node_id() -> NodeId {
        NodeId {
            name: "node1".to_owned(),
            addr: "127.0.0.1:9999".to_owned()
        }
    }

    fn blob_create() -> VrApiReq {
        VrApiReq::TreeOp(TreeOp::CreateNode {path: "/a".to_owned(), ty: NodeType::Blob})
    }

    fn client_request(i: u64) -> ClientOp {
        ClientRequest {
            op: blob_create(),
            client_id: "test-client".to_owned(),
            request_num: i
        }.into()
    }

    #[test]
    fn append_and_get_log_tail() {
        let replicas = VersionedReplicas::new();
        let mut ctx = VrCtx::new(logger(), test_pid(), replicas.clone(), replicas);
        // Client request num 1 - 5
        let log: Vec<ClientOp> = (1..6).map(|i| client_request(i).into()).collect();
        ctx.log = log.clone();

        // This mimics a do_view_change or start_view operation where the tail is retrieved
        // from the log in the sender and then appended in the receiver. We ensure that any time we
        // take some entries from some log and append those entries to the same log we end up with the
        // same log.
        //
        // This could be a quickcheck property, but this is faster and inductively correct
        for i in 0..5 {
            ctx.global_min_accept = i;
            let tail = ctx.get_log_tail();
            ctx.append_log_tail(i, tail.clone());
            assert_eq!(tail, ctx.get_log_tail());
            assert_eq!(log, ctx.log);
        }
    }
}
