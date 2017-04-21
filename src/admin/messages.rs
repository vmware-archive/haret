// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::convert::{TryFrom, TryInto};
use rabble::{Pid, NodeId, Metric, Result, Error, ErrorKind};
use config::Config;
use namespaces::Namespaces;
use namespace_msg::NamespaceId;
use vr::VrCtxSummary;
use pb_msg;
use rabble::pb_messages as rabble_msg;

#[derive(Debug, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub enum AdminMsg {
    Req(AdminReq),
    Rpy(AdminRpy)
}

#[derive(Debug, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub enum AdminReq {
    GetConfig,
    Join(NodeId),
    CreateNamespace(Vec<Pid>),
    GetNamespaces,
    GetReplicaState(Pid),
    GetPrimary(NamespaceId),
    GetClusterStatus,
    GetMetrics(Pid)
}

#[derive(Debug, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub enum AdminRpy {
    Ok,
    Timeout,
    Error(String),
    Config(Config),
    NamespaceId(NamespaceId),
    Namespaces(Namespaces),
    ReplicaState(VrCtxSummary),
    ReplicaNotFound(Pid),
    Primary(Option<Pid>),
    Metrics(Vec<(String, Metric)>)
}

impl From<AdminReq> for pb_msg::AdminReq {
    fn from(req: AdminReq) -> pb_msg::AdminReq {
        let mut msg = pb_msg::AdminReq::new();
        match req {
            AdminReq::GetConfig => msg.set_get_config(true),
            AdminReq::Join(node_id) => msg.set_join(node_id.into()),
            AdminReq::CreateNamespace(pids) => {
                let mut pb_pids = rabble_msg::Pids::new();
                pb_pids.set_pids(pids.into_iter().map(|pid| pid.into()).collect());
                msg.set_create_namespace(pb_pids);
            },
            AdminReq::GetNamespaces => msg.set_get_namespaces(true),
            AdminReq::GetReplicaState(pid) => msg.set_get_replica_state(pid.into()),
            AdminReq::GetPrimary(ns_id) => msg.set_get_primary(ns_id.0),
            AdminReq::GetClusterStatus => msg.set_get_cluster_status(true),
            AdminReq::GetMetrics(pid) => msg.set_get_metrics(pid.into())
        }
        msg
    }
}

impl TryFrom<pb_msg::AdminReq> for AdminReq {
    type Error = Error;
    fn try_from(msg: pb_msg::AdminReq) -> Result<AdminReq> {
        if msg.has_get_config() {
            return Ok(AdminReq::GetConfig);
        }
        if msg.has_join() {
            return Ok(AdminReq::Join(msg.take_join().into()));
        }
        if msg.has_create_namespace() {
            let pids = msg.take_create_namespace().take_pids().into_iter().map(|p| p.into()).collect();
            return Ok(AdminReq::CreateNamespace(pids));
        }
        if msg.has_get_namespaces() {
            return Ok(AdminReq::GetNamespaces);
        }
        if msg.has_get_replica_state() {
            return Ok(AdminReq::GetReplicaState(msg.take_get_replica_state().into()));
        }
        if msg.has_get_primary() {
            return Ok(AdminReq::GetPrimary(NamespaceId(msg.take_get_primary())));
        }
        if msg.has_get_cluster_status() {
            return Ok(AdminReq::GetClusterStatus);
        }
        if msg.has_get_metrics() {
            return Ok(AdminReq::GetMetrics(msg.take_get_metrics().into()));
        }

        Err(ErrorKind::ProtobufDecodeError("Unknown AdminReq").into())
    }
}

impl From<AdminRpy> for pb_msg::AdminRpy {
    fn from(rpy: AdminRpy) -> pb_msg::AdminRpy {
        let mut msg = pb_msg::AdminRpy::new();
        match rpy {
            AdminRpy::Ok => msg.set_ok(true),
            AdminRpy::Timeout => msg.set_timeout(true),
            AdminRpy::Error(s) => msg.set_error(s),
            AdminRpy::Config(config) => msg.set_config(config.into()),
            AdminRpy::NamespaceId(ns_id) => msg.set_namespace_id(ns_id.0),
            AdminRpy::Namespaces(namespaces) => msg.set_namespaces(namespaces.into()),
            AdminRpy::ReplicaState(state) => msg.set_replica_state(state.into()),
            AdminRpy::ReplicaNotFound(pid) => msg.set_replica_not_found(pid.into()),
            AdminRpy::Primary(Some(pid)) => {
                let mut primary = pb_msg::Primary::new();
                primary.set_primary(pid.into());
                msg.set_primary(primary);
            },
            AdminRpy::Primary(None) => msg.set_primary(pb_msg::Primary::new()),
            AdminRpy::Metrics(metrics) => msg.set_metrics(metrics.into()),
        }
        msg
    }
}

impl TryFrom<pb_msg::AdminRpy> for AdminRpy {
    type Error = Error;

    fn try_from(msg: pb_msg::AdminRpy) -> Result<AdminRpy> {
        if msg.has_ok() {
            return Ok(AdminRpy::Ok);
        }
        if msg.has_timeout() {
            return Ok(AdminRpy::Timeout);
        }
        if msg.has_error() {
            return Ok(AdminRpy::Error(msg.take_error()));
        }
        if msg.has_config() {
            return Ok(AdminRpy::Config(msg.take_config().into()));
        }
        if msg.has_namespace_id() {
            return Ok(AdminRpy::NamespaceId(NamespaceId(msg.take_namespace_id())));
        }
        if msg.has_namespaces() {
            return Ok(AdminRpy::Namespaces(msg.take_namespaces().into()));
        }
        if msg.has_replica_state() {
            return Ok(AdminRpy::ReplicaState(msg.take_replica_state().into()));
        }
        if msg.has_replica_not_found() {
            return Ok(AdminRpy::ReplicaNotFound(msg.take_replica_not_found().into()));
        }
        if msg.has_primary() {
            let primary = msg.take_primary();
            if primary.has_primary() {
                return Ok(AdminRpy::Primary(Some(primary.take_primary().into())));
            } 
            return Ok(AdminRpy::Primary(None));
        }
        if msg.has_metrics() {
            return Ok(AdminRpy::Metrics(msg.take_metrics().try_into()?));
        }

        Err(ErrorKind::ProtobufDecodeError("Unknown AdminRpy").into())
    }
}
