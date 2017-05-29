// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use time::Duration;
use std::collections::{HashSet, HashMap};
use slog::Logger;
use amy::{Registrar, Notification};
use rabble::{self, Pid, NodeId, Node, Envelope, CorrelationId, ServiceHandler};
use rabble::errors::ChainErr;
use funfsm::{Fsm, StateFn};
use msg::Msg;
use config::Config;
use vr::{VrMsg, Replica, VersionedReplicas};
use namespace_msg::{NamespaceMsg, ClientId, NamespaceId};
use namespaces::Namespaces;
use vr::vr_fsm::{self, VrTypes};
use vr::vr_ctx::{VrCtx, DEFAULT_IDLE_TIMEOUT_MS, DEFAULT_PRIMARY_TICK_MS};
use admin::{AdminReq, AdminRpy};
use api::ApiRpy;

const MANAGEMENT_TICK_MS: u64 = 10000; // 10s

pub struct NamespaceMgr {
    config: Config,
    node: Node<Msg>,
    logger: Logger,
    pub pid: Pid,
    /// Dispatchers on other nodes
    peers: HashSet<Pid>,
    namespaces: Namespaces,
    /// This node's api address
    api_addr: String,
    /// Other nodes' api addresses. Each node publishes its own configuration changes only.
    api_addrs: HashMap<NodeId, String>,
    local_replicas: HashSet<Pid>,

    /**************************************************************************/
    // It's possible these ticks will be replaced by per fsm timeouts scheduled by the FSMs
    // themselves. For now stick to the original strategy in the old dispatcher code.

    /// Timeout configuration for VR Fsms
    idle_timeout: Duration,
    primary_tick_ms: u64,

    /// The amount of time between VrMsg::Tick messages being sent to replicas. By default this
    /// value is set at 1/3 * primary_tick_ms for the following reasons:
    /// 1) We want to send timeout messages as close to the tick timeout as possible (therfore use
    ///    smaller tick values)
    /// 2) We don't want to unnecessarily spam the FSMs with ticks and chew CPU cycles (therefore
    ///    use larger tick values)
    /// This setting allows us to maximally send a commit message 1/3 primary_tick_ms late. If we
    /// always ensure that `idle_timeout` is at least double `primary_tick_ms` we can minimize
    /// unnecessary view changes. Note that for backups, the ticks are higher frequency than
    /// necessary, but this is the tradeoff made for having a single Tick message.
    tick_period: u64,
    /*************************************************************************/
    fsm_timer_id: usize,
    management_timer_id: usize
}

impl NamespaceMgr {
    pub fn new(node: Node<Msg>, config: Config, logger: Logger) -> NamespaceMgr {
        let pid = Pid {
            group: None,
            name: "namespace_mgr".to_string(),
            node: node.id.clone()
        };
        let api_addr = config.api_host.clone();
        NamespaceMgr {
            config: config,
            node: node,
            logger: logger,
            pid: pid,
            peers: HashSet::new(),
            namespaces: Namespaces::new(),
            api_addr: api_addr,
            api_addrs: HashMap::new(),
            local_replicas: HashSet::new(),
            idle_timeout: Duration::milliseconds(DEFAULT_IDLE_TIMEOUT_MS as i64),
            primary_tick_ms: DEFAULT_PRIMARY_TICK_MS,
            tick_period: DEFAULT_PRIMARY_TICK_MS / 3,
            fsm_timer_id: 0, // Dummy timer for now. Will be set in init()
            management_timer_id: 0, // Dummy timer for now. Will be set in init()
        }
    }

    fn fsm_tick(&self) {
        for pid in self.local_replicas.iter().cloned() {
            let envelope = Envelope {
                to: pid,
                from: self.pid.clone(),
                msg: rabble::Msg::User(Msg::Vr(VrMsg::Tick)),
                correlation_id: None
            };
            self.node.send(envelope).unwrap();
        }
    }

    fn management_tick(&self) {
        for pid in self.peers.iter().cloned() {
            let envelope = self.namespaces_msg(pid.clone());
            self.node.send(envelope).unwrap();
            let envelope = self.api_addr_msg(pid);
            self.node.send(envelope).unwrap();
        }
    }

    fn api_addr_msg(&self, pid: Pid) -> Envelope<Msg> {
        Envelope {
            to: pid,
            from: self.pid.clone(),
            msg: rabble::Msg::User(Msg::Namespace(NamespaceMsg::ApiAddr(self.api_addr.clone()))),
            correlation_id: None
        }
    }

    fn namespaces_msg(&self, pid: Pid) -> Envelope<Msg> {
        Envelope {
            to: pid,
            from: self.pid.clone(),
            msg: rabble::Msg::User(Msg::Namespace(
                    NamespaceMsg::Namespaces(self.namespaces.clone()))),
            correlation_id: None
        }
    }

    fn handle_namespace_msg(&mut self,
                            from: Pid,
                            msg: NamespaceMsg,
                            c_id: Option<CorrelationId>)
    {
        match msg {
            NamespaceMsg::Namespaces(namespaces) =>
                self.check_namespaces(namespaces),
            NamespaceMsg::ApiAddr(addr) => {
                self.api_addrs.insert(from.node.clone(), addr);
            }
            NamespaceMsg::Reconfiguration {namespace_id, old_config, new_config} =>
                self.reconfigure(namespace_id, old_config, new_config),
            NamespaceMsg::Stop(pid) =>
                self.stop(&pid),
            NamespaceMsg::NewPrimary(pid) => {
                let namespace_id = NamespaceId(pid.group.clone().unwrap());
                self.namespaces.primaries.insert(namespace_id, pid);
            },
            NamespaceMsg::ClearPrimary(namespace_id) => {
                self.namespaces.primaries.remove(&namespace_id);
            },
            NamespaceMsg::RegisterClient(client_id, namespace_id) =>
                self.register_client(client_id, namespace_id, c_id.unwrap())
        }
    }

    // TODO: Register the client with the VR client table (once it exists).
    fn register_client(&mut self,
                       _: ClientId,
                       namespace_id: NamespaceId,
                       c_id: CorrelationId)
    {
        let msg = match self.namespaces.primaries.get(&namespace_id) {
            Some(replica) => {
                if replica.node == self.node.id {
                    // TODO: Actually return a valid new_registration value when the
                    // client table exists
                    ApiRpy::ClientRegistration {primary: replica.clone(), new_registration: true}
                } else {
                    match self.api_addrs.get(&replica.node) {
                        Some(addr) =>
                            ApiRpy::Redirect {primary: replica.clone(), api_addr: addr.clone()},
                        None =>
                            ApiRpy::Retry(10000)
                    }
                }
            },
            None => ApiRpy::UnknownNamespace
        };
        let envelope = Envelope {
            to: c_id.pid.clone(),
            from: self.pid.clone(),
            msg: rabble::Msg::User(Msg::ApiRpy(msg)),
            correlation_id: Some(c_id)
        };
        self.node.send(envelope).unwrap();
    }

    fn handle_admin_req(&mut self, req: AdminReq, correlation_id: Option<CorrelationId>) {
        let correlation_id = correlation_id.unwrap();
        match req {
            AdminReq::GetConfig => {
                let config = self.config.clone();
                self.send_admin_rpy(AdminRpy::Config(config), correlation_id);
            },
            AdminReq::Join(node_id) => {
                self.node.join(&node_id).unwrap();
                self.send_admin_rpy(AdminRpy::Ok, correlation_id);
            },
            AdminReq::GetNamespaces => {
                let namespaces = self.namespaces.clone();
                self.send_admin_rpy(AdminRpy::Namespaces(namespaces),
                                    correlation_id);
            },
            AdminReq::CreateNamespace(replicas) => {
                let mut s: String = replicas.iter().map(|r| {
                    let mut s = r.to_string();
                    s.push(',');
                    s
                }).collect();
                s = s.trim_matches(',').to_string();
                info!(self.logger,  "Attempt Create namespace"; "replicas" => s.clone());
                match self.create_namespace(replicas) {
                    Ok(namespace_id) => {
                        info!(self.logger, "Namespace created"; "namespace" => namespace_id.0);
                        self.send_admin_rpy(AdminRpy::Ok, correlation_id)
                    },
                    Err(e) => {
                        warn!(self.logger, "Namespace failed to be created";
                              "replicas" => s, "error" => e.to_string());
                        self.send_admin_rpy(AdminRpy::Error(e.to_string()), correlation_id)
                    }
                }
            },
            AdminReq::GetPrimary(namespace_id) => {
                let primary = match self.namespaces.primaries.get(&namespace_id) {
                    Some(replica) => Some(replica.clone()),
                    None => None
                };
                self.send_admin_rpy(AdminRpy::Primary(primary), correlation_id);
            },
            AdminReq::GetClusterStatus => {
                self.node.cluster_status(correlation_id).unwrap();
            },
            _ => {
                self.send_admin_rpy(AdminRpy::Error("Unknown Admin Message".to_string()),
                                    correlation_id);
            }
        }
    }


    fn send_admin_rpy(&mut self, reply: AdminRpy, correlation_id: CorrelationId) {
        let envelope = Envelope {
            to: correlation_id.pid.clone(),
            from: self.pid.clone(),
            msg: rabble::Msg::User(Msg::AdminRpy(reply)),
            correlation_id: Some(correlation_id)
        };
        self.node.send(envelope).unwrap();
    }

    /// Receive a copy of current namespaces from another node and see if the local copy is
    /// outdated. Configure them and start/stop replicas as needed.
    pub fn check_namespaces(&mut self, namespaces: Namespaces) {
        for (namespace_id, &(ref old_config, ref new_config)) in namespaces.map.iter() {
            if self.namespaces.exists(namespace_id) {
                self.reconfigure(namespace_id.clone(), old_config.clone(), new_config.clone());
            } else {
                self.namespaces.insert(namespace_id.clone(), old_config.clone(), new_config.clone());
                for r in old_config.replicas.iter().chain(new_config.replicas.iter()) {
                    self.peers.insert(Pid {
                        group: None,
                        name: "namespace_mgr".to_string(),
                        node: r.node.clone()
                    });
                }
                for r in new_config.replicas.iter().cloned() {
                    if self.node.id == r.node{
                        if old_config.epoch == 0 {
                            self.start_replica_initial_config(r, new_config.clone());
                        } else {
                            self.start_replica_reconfig(&r, old_config, new_config);
                        }
                    }
                }
            }
        }
    }

    fn create_namespace(&mut self, mut new_replicas: Vec<Pid>) -> rabble::Result<NamespaceId> {
        let namespace_id = validate_group_pids(&new_replicas)?;
        new_replicas.sort();
        let old_config = VersionedReplicas::new();
        let new_config = VersionedReplicas {epoch: 1, op: 0, replicas: new_replicas};
        self.namespaces.insert(namespace_id.clone(), old_config.clone(), new_config.clone());
        for r in new_config.replicas.iter().cloned() {
            self.peers.insert(Pid {
                group: None,
                name: "namespace_mgr".to_string(),
                node: r.node.clone()
            });
            if self.node.id == r.node {
                self.start_replica_initial_config(r, new_config.clone());
            }
        }
        Ok(namespace_id)
    }

   fn reconfigure(&mut self,
                  namespace: NamespaceId,
                  old_config: VersionedReplicas,
                  new_config: VersionedReplicas)
    {
        let (changed, to_start) =
            self.namespaces.reconfigure(&namespace, old_config.clone(), new_config.clone());

        if changed {
            self.peers = self.namespaces.nodes().iter().map(|node| {
                Pid {
                    group: None,
                    name: "namespace_mgr".to_string(),
                    node: node.clone()
                }
            }).collect();
        }

        for replica in to_start {
            if self.node.id == replica.node {
                self.start_replica_reconfig(&replica, &old_config, &new_config);
            }
        }
    }

   fn start_replica_reconfig(&mut self,
                             pid: &Pid,
                             old_config: &VersionedReplicas,
                             new_config: &VersionedReplicas) {
       // The same reconfigure announcement can occur from multiple replicas on the same node
       if self.local_replicas.contains(pid) { return; }
       let mut ctx = VrCtx::new(self.logger.clone(),
                                pid.clone(),
                                old_config.clone(),
                                new_config.clone());
       ctx.idle_timeout = self.idle_timeout.clone();
       ctx.primary_tick_ms = self.primary_tick_ms;
       let fsm = Fsm::<VrTypes>::new(ctx, state_fn!(vr_fsm::startup_reconfiguration));
       self.node.spawn(&pid, Box::new(Replica::new(pid.clone(), fsm))).unwrap();
       self.local_replicas.insert(pid.clone());
   }


    fn start_replica_initial_config(&mut self, pid: Pid, new_config: VersionedReplicas) {
       println!("start pid {:?}", pid);
       let mut ctx = VrCtx::new(self.logger.clone(),
                                pid.clone(),
                                VersionedReplicas::new(),
                                new_config);
       ctx.idle_timeout = self.idle_timeout.clone();
       ctx.primary_tick_ms = self.primary_tick_ms;
       let state = if pid == ctx.compute_primary() {
           VrStates::Primary(Primary::new(ctx))
       } else {
           VrStates::Backup(Backup::new(ctx));
       };
       self.node.spawn(&pid, Box::new(Replica::new(pid.clone(), state))).unwrap();
       self.local_replicas.insert(pid);
    }

    /// Should only be called outside this module during tests
    pub fn stop(&mut self, pid: &Pid) {
        self.local_replicas.remove(pid);
        self.node.stop(pid).unwrap();
    }

    /// Should only be called outside this module during tests
    pub fn restart(&mut self, pid: Pid) {
       let namespace_id = NamespaceId(pid.group.clone().unwrap());
       if let Some((old_config, new_config)) = self.namespaces.get_config(&namespace_id) {
           let mut ctx = VrCtx::new(self.logger.clone(),
                                    pid.clone(),
                                    old_config.clone(),
                                    new_config.clone());
           ctx.idle_timeout = self.idle_timeout.clone();
           ctx.primary_tick_ms = self.primary_tick_ms;
           let fsm = Fsm::<VrTypes>::new(ctx, state_fn!(vr_fsm::startup_recovery));
           self.node.spawn(&pid, Box::new(Replica::new(pid.clone(), fsm))).unwrap();
           self.local_replicas.insert(pid);
       }
    }
}

impl ServiceHandler<Msg> for NamespaceMgr {
    fn init(&mut self, registrar: &Registrar, _: &Node<Msg>) -> rabble::Result<()> {
        self.fsm_timer_id = registrar.set_interval(self.tick_period as usize)
                              .chain_err(|| "Failed to register request timer")?;
        self.management_timer_id = registrar.set_interval(MANAGEMENT_TICK_MS as usize)
                                     .chain_err(|| "Failed to register request timer")?;
        Ok(())
    }

    fn handle_envelope(&mut self,
                       _: &Node<Msg>,
                       envelope: Envelope<Msg>,
                       _: &Registrar) -> rabble::Result<()>
    {
        let Envelope{from, msg, correlation_id, ..} = envelope;
        match msg {
            rabble::Msg::User(Msg::AdminReq(req)) =>
                self.handle_admin_req(req, correlation_id),
            rabble::Msg::User(Msg::Namespace(msg)) =>
                self.handle_namespace_msg(from, msg, correlation_id),
            _ => unreachable!()
        }

        Ok(())
    }

    /// Handle any poll notifications
    fn handle_notification(&mut self,
                           _: &Node<Msg>,
                           notification: Notification,
                           _: &Registrar) -> rabble::Result<()>
    {

        if notification.id == self.fsm_timer_id {
            self.fsm_tick();
            return Ok(());
        }
        if notification.id == self.management_timer_id {
            self.management_tick();
            return Ok(());
        }

        Err("namespace_mgr got invalid notification: not a timer".into())
    }
}

/// Ensure all pids have an identical `group` value and that group value is not `None`
fn validate_group_pids(pids: &Vec<Pid>) -> rabble::Result<NamespaceId> {
    let mut namespace_id = None;
    for pid in pids {
        match pid.group {
            Some(ref s) => {
                if namespace_id.is_some() && namespace_id != Some(NamespaceId(s.to_string())) {
                    return Err("Pids in the same group cannot have different group IDs".into());
                }
                if namespace_id.is_none() {
                    namespace_id = Some(NamespaceId(s.to_string()));
                }
            },
            None => {
                return Err("All pids in a group must have a group ID".into());
            }
        }
    }
    Ok(namespace_id.unwrap())
}

