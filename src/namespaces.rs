// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use vr::VersionedReplicas;
use rabble::{Pid, NodeId, Result, Error, ErrorKind};
use namespace_msg::NamespaceId;
use pb_msg;

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Namespaces {
    pub map: HashMap<NamespaceId, (VersionedReplicas, VersionedReplicas)>,
    pub primaries: HashMap<NamespaceId, Pid>
}

impl Namespaces {
    pub fn new() -> Namespaces {
        Namespaces {
            map: HashMap::new(),
            primaries: HashMap::new()
        }
    }

    /// Return the nodes used in all namespaces
    /// Note that this can be expensive with a large number of namespaces.
    /// However, It's only used during an actual reconfiguration.
    pub fn nodes(&self) -> HashSet<NodeId> {
        self.map.iter().flat_map(|(_, &(ref old_config, ref new_config))| {
            old_config.replicas.iter().chain(new_config.replicas.iter()).map(|r| {
                r.node.clone()
            })
        }).collect()
    }

    pub fn insert(&mut self, namespace: NamespaceId, old: VersionedReplicas, new: VersionedReplicas) {
        self.map.insert(namespace, (old, new));
    }

    pub fn exists(&self, namespace: &NamespaceId) -> bool {
        self.map.contains_key(namespace)
    }

    pub fn get_config(&self, namespace: &NamespaceId) -> Option<(VersionedReplicas, VersionedReplicas)> {
        match self.map.get(&namespace) {
            Some(&(ref old_config, ref new_config)) =>
                Some((old_config.clone(), new_config.clone())),
            None => None
        }
    }

    /// Save the new namespace configuration and return whether there actually was a configuration
    /// change as well as the new replicas that need starting.
    pub fn reconfigure(&mut self,
                       namespace: &NamespaceId,
                       old: VersionedReplicas,
                       new: VersionedReplicas) -> (bool, Vec<Pid>)
    {
        if let Some(&mut(ref mut saved_old_config, ref mut saved_new_config)) =
            self.map.get_mut(&namespace)
        {
            // This is an old reconfig message, nothing to start
            if new.epoch <= saved_new_config.epoch { return (false, Vec::new()) }
            let new_set = HashSet::<Pid>::from_iter(new.replicas.clone());
            // We want to use the actual running nodes here because we are trying to determine which
            // nodes to start locally
            let old_set = HashSet::<Pid>::from_iter(saved_new_config.replicas.clone());
            let to_start: Vec<Pid> = new_set.difference(&old_set).cloned().collect();
            *saved_old_config = old;
            *saved_new_config = new;
            (true, to_start)
        } else {
            (false, Vec::new())
        }
    }
}

impl From<Namespaces> for pb_msg::Namespaces {
    fn from(namespaces: Namespaces) -> pb_msg::Namespaces {
        let msg = pb_msg::Namespaces::new();
        msg.set_map(namespaces.map.into_iter().map(|(ns_id, (old_config, new_config))| {
            let mut entry = pb_msg::Namespaces_MapEntry::new();
            entry.set_key(ns_id.0);
            let mut config = pb_msg::ReplicaConfig::new();
            config.set_old(old_config.into());
            config.set_new(new_config.into());
            entry.set_value(config);
            entry
        }).collect());
        msg.set_primaries(namespaces.primaries.into_iter().map(|(ns_id, pid)| {
            let mut entry = pb_msg::Namespaces_PrimariesEntry::new();
            entry.set_key(ns_id.0);
            entry.set_value(pid.into());
            entry
        }).collect());
        msg
    }
}

impl From<pb_msg::Namespaces> for Namespaces {
    fn from(msg: pb_msg::Namespaces) -> Namespaces {
        let map = msg.take_map().into_iter().map(|entry| {
            let ns_id = NamespaceId(entry.take_key());
            let replica_config = entry.take_value();
            (ns_id, (replica_config.take_old().into(), replica_config.take_new().into()))
        }).collect();
        let primaries = msg.take_primaries().into_iter().map(|entry| {
            (NamespaceId(entry.take_key()), entry.take_value().into())
        }).collect();

        Namespaces {
            map: map,
            primaries: primaries
        }
    }
}
