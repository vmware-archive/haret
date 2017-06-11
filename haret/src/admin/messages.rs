// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use rabble::{Pid, NodeId, ClusterStatus, Metric};
use config::Config;
use namespaces::Namespaces;
use namespace_msg::NamespaceId;
use vr::VrCtxSummary;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdminMsg {
    Req(AdminReq),
    Rpy(AdminRpy)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    ClusterStatus(ClusterStatus),
    Metrics(Vec<(String, Metric)>)
}

