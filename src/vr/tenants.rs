use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use uuid::Uuid;
use super::replica::{Replica, VersionedReplicas};

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Tenants {
    pub map: HashMap<Uuid, (VersionedReplicas, VersionedReplicas)>,
    pub primaries: HashMap<Uuid, Replica>
}

impl Tenants {
    pub fn new() -> Tenants {
        Tenants {
            map: HashMap::new(),
            primaries: HashMap::new()
        }
    }

    pub fn insert(&mut self, tenant: Uuid, old: VersionedReplicas, new: VersionedReplicas) {
        self.map.insert(tenant, (old, new));
    }

    pub fn exists(&self, tenant: &Uuid) -> bool {
        self.map.contains_key(tenant)
    }

    pub fn get_config(&self, tenant: &Uuid) -> Option<(VersionedReplicas, VersionedReplicas)> {
        match self.map.get(&tenant) {
            Some(&(ref old_config, ref new_config)) =>
                Some((old_config.clone(), new_config.clone())),
            None => None
        }
    }

    /// Save the new tenant configuration and return the new replicas that need starting
    pub fn reconfigure(&mut self, tenant: &Uuid, old: VersionedReplicas, new: VersionedReplicas) ->
        Vec<Replica>
    {
        if let Some(&mut(ref mut saved_old_config, ref mut saved_new_config)) =
            self.map.get_mut(&tenant)
        {
            // This is an old reconfig message, nothing to start
            if new.epoch <= saved_new_config.epoch { return Vec::new() }
            let new_set = HashSet::<Replica>::from_iter(new.replicas.clone());
            // We want to use the actual running nodes here because we are trying to determine which
            // nodes to start locally
            let old_set = HashSet::<Replica>::from_iter(saved_new_config.replicas.clone());
            let to_start: Vec<Replica> = new_set.difference(&old_set).cloned().collect();
            *saved_old_config = old;
            *saved_new_config = new;
            to_start
        } else {
            Vec::new()
        }
    }
}
