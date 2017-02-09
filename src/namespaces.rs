// Copyright 2017 VMware, Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use vr::VersionedReplicas;
use rabble::{Pid, NodeId};
use namespace_msg::NamespaceId;

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
