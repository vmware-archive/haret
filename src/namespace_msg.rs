// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use rabble::Pid;
use namespaces::Namespaces;
use vr::VersionedReplicas;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClientId(pub String);

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct NamespaceId(pub String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NamespaceMsg {
    /// Namespaces are gossiped between namespace managers,
    Namespaces(Namespaces),

    /// Register a client with the primary of a namespace
    RegisterClient(ClientId, NamespaceId),

    /// API Addresses are published from the node they live on to all other nodes.
    ApiAddr(String),

    /// The following four messages are sent from an FSM to indicate a
    /// change in membership state for a given namespace
    Reconfiguration {
        namespace_id: NamespaceId,
        old_config: VersionedReplicas,
        new_config: VersionedReplicas
    },
    Stop(Pid),
    NewPrimary(Pid),
    ClearPrimary(NamespaceId)
}

