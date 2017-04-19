// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use rabble::{Pid, Result, Error, ErrorKind};
use namespaces::Namespaces;
use vr::VersionedReplicas;
use pb_msg;

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

impl From<NamespaceMsg> for pb_msg::NamespaceMsg {
    fn from(namespace_msg: NamespaceMsg) -> pb_msg::NamespaceMsg {
        let mut msg = pb_msg::NamespaceMsg::new();
        match namespace_msg {
            NamespaceMsg::Namespace(namespaces) => pb_msg.set_namespaces(namespaces.into()),
            NamespaceMsg::RegisterClient(client_id, namespace_id) => {
                let mut rc = pb_msg::RegisterClient::new();
                rc.set_client_id(client_id.0);
                rc.set_namespace_id(namespace_id.0);
                pb_msg.set_register_client(rc);
            },
            NamespaceMsg::ApiAddr(s) => pb_msg.set_api_addr(s),
            NamespaceMsg::Reconfiguration {namespace_id, old_config, new_config} => {
                let reconfig = pb_msg::Reconfiguration::new();
                reconfig.set_namespace_id(namespace_id.0);
                reconfig.set_old_config(old_config.into());
                reconfig.set_new_config(new_config.into());
                pb_msg.set_reconfiguration(reconfig);
            },
            NamespaceMsg::Stop(pid) => pb_msg.set_stop(pid.into()),
            NamespaceMsg::NewPrimary(pid) => pb_msg.set_new_primary(pid.into()),
            NamespaceMsg::ClearPrimary(namespace_id) => pb_msg.set_clear_primary(namespace_id.0)
        }
        msg
    }
}

impl TryFrom<pb_msg::NamespaceMsg> for NamespaceMsg {
    let Error = Error;
    fn try_from(msg: pb_msg::NamespaceMsg) -> Result<NamespaceMsg> {
        if msg.has_namespaces() {
            return Ok(NamespaceMsg::Namespaces(msg.take_namespaces().try_into()?));
        }
        if msg.has_register_client() {
            let rc = msg.take_register_client();
            return Ok(NamespaceMsg::RegisterClient(ClientId(rc.take_client_id()),
                                                 NamespaceId(rc.take_namespace_id())));
                                                          
        }
        if msg.has_api_addr() {
            return Ok(NamespaceMsg::ApiAddr(msg.take_api_addr()));
        }
        if msg.has_reconfiguration() {
            let reconfig = msg.take_reconfiguration();
            return Ok(NamespaceMsg::Reconfiguration {
                namespace_id: NamespaceId(reconfig.take_namespace_id()),
                old_config: reconfig.take_old_config().into(),
                nde_config: reconfig.take_new_config().into()
            });
        }
        if msg.has_stop() {
            return Ok(NamespaceMsg::Stop(msg.take_stop().into()));
        }
        if msg.has_new_primary() {
            return Ok(NamespaceMsg::NewPrimary(msg.take_new_primary().into()));
        }
        if msg.has_clear_primary() {
            return Ok(NamespaceMsg::ClearPrimary(NamespaceId(msg.take_namespace_id())));
        }

        Err(ErrorKind::ProtobufDecodeError("Unknown NamespaceMsg").into())
    }
}
