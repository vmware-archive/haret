use time::Duration;
use std::collections::HashSet;
use uuid::Uuid;
use amy::{Registrar, Notification, Timer};
use rabble::{self, Pid, Node, Envelope, CorrelationId, ServiceHandler};
use rabble::errors::ChainErr;
use fsm::{Fsm, StateFn};
use msg::Msg;
use vr::{VrMsg, Replica, VersionedReplicas};
use namespace_msg::NamespaceMsg;
use namespaces::Namespaces;
use vr::vr_fsm::{self, VrTypes};
use vr::vr_ctx::{VrCtx, DEFAULT_IDLE_TIMEOUT_MS, DEFAULT_PRIMARY_TICK_MS};
use admin::{AdminReq, AdminRpy};

const MANAGEMENT_TICK_MS: u64 = 10000; // 10s

pub struct NamespaceMgr {
    node: Node<Msg>,
    pub pid: Pid,
    /// Dispatchers on other nodes
    peers: HashSet<Pid>,
    namespaces: Namespaces,
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
    fsm_timer: Timer,
    management_timer: Timer
}

impl NamespaceMgr {
    pub fn new(node: Node<Msg>) -> NamespaceMgr {
        let pid = Pid {
            group: None,
            name: "namespace_mgr".to_string(),
            node: node.id.clone()
        };
        NamespaceMgr {
            node: node,
            pid: pid,
            peers: HashSet::new(),
            namespaces: Namespaces::new(),
            local_replicas: HashSet::new(),
            idle_timeout: Duration::milliseconds(DEFAULT_IDLE_TIMEOUT_MS as i64),
            primary_tick_ms: DEFAULT_PRIMARY_TICK_MS,
            tick_period: DEFAULT_PRIMARY_TICK_MS / 3,
            fsm_timer: Timer {id: 0, fd: 0}, // Dummy timer for now. Will be set in init()
            management_timer: Timer {id: 0, fd: 0}, // Dummy timer for now. Will be set in init()
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
            let _ = self.node.send(envelope);
        }
    }

    fn management_tick(&self) {
        for pid in self.peers.iter().cloned() {
            let envelope = Envelope {
                to: pid,
                from: self.pid.clone(),
                msg: rabble::Msg::User(Msg::Namespace(NamespaceMsg::Namespaces(self.namespaces.clone()))),
                correlation_id: None
            };
            let _ = self.node.send(envelope);
        }
    }

    fn handle_namespace_msg(&mut self, msg: NamespaceMsg, _correlation_id: Option<CorrelationId>) {
        match msg {
            NamespaceMsg::Namespaces(namespaces) =>
                self.check_namespaces(namespaces),
            NamespaceMsg::Reconfiguration {namespace_id, old_config, new_config} =>
                self.reconfigure(namespace_id, old_config, new_config),
            NamespaceMsg::Stop(pid) =>
                self.stop(&pid),
            NamespaceMsg::NewPrimary(pid) => {
                self.namespaces.primaries.insert(Uuid::parse_str(&pid.clone().group.unwrap()).unwrap(), pid);
            },
            NamespaceMsg::ClearPrimary(namespace_id) => {
                self.namespaces.primaries.remove(&namespace_id);
            }
        }
    }

    fn handle_admin_req(&mut self, req: AdminReq, correlation_id: Option<CorrelationId>) {
        let correlation_id = correlation_id.unwrap();
        match req {
            AdminReq::Join(node_id) => {
                let _ = self.node.join(&node_id);
                self.send_admin_rpy(AdminRpy::Ok, correlation_id);
            },
            AdminReq::GetNamespaces => {
                let namespaces = self.namespaces.clone();
                self.send_admin_rpy(AdminRpy::Namespaces(namespaces),
                                    correlation_id);
            },
            AdminReq::CreateNamespace(replicas) => {
                match self.create_namespace(replicas) {
                    Ok(namespace) => self.send_admin_rpy(AdminRpy::NamespaceId(namespace), correlation_id),
                    Err(e) => self.send_admin_rpy(AdminRpy::Error(e), correlation_id)
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
                let _ = self.node.cluster_status(correlation_id);
            },
            _ => {
                // TODO: Logging
                println!("Received unknown AdminReq in namespace_mgr: {:?}", req)
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
        let _ = self.node.send(envelope);
    }

    /// Receive a copy of current namespaces from another node and see if the local copy is
    /// outdated. Configure them and start/stop replicas as needed.
    pub fn check_namespaces(&mut self, namespaces: Namespaces) {
        for (namespace_id, &(ref old_config, ref new_config)) in namespaces.map.iter() {
            if self.namespaces.exists(namespace_id) {
                self.reconfigure(namespace_id.clone(), old_config.clone(), new_config.clone());
            } else {
                self.namespaces.insert(namespace_id.clone(), old_config.clone(), new_config.clone());
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

    fn create_namespace(&mut self, ungrouped_pids: Vec<Pid>) -> Result<Uuid, String> {
        let namespace = Uuid::new_v4();
        let mut new_replicas = Vec::new();
        for mut pid in ungrouped_pids {
            let mut found = false;
            let node_name = pid.node.name.clone();
            for node in self.peers.iter().map(|peer_pid| peer_pid.node.clone()) {
                if node == pid.node {
                    pid.group = Some(namespace.clone().to_string());
                    new_replicas.push(pid);
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(format!("Node {} is not a member of the cluster", node_name));
            }
        }
        new_replicas.sort();
        let old_config = VersionedReplicas::new();
        let new_config = VersionedReplicas {epoch: 1, op: 0, replicas: new_replicas};
        self.namespaces.insert(namespace, old_config.clone(), new_config.clone());
        for r in new_config.replicas.iter().cloned() {
            if self.node.id == r.node {
                self.start_replica_initial_config(r, new_config.clone());
            }
        }
        Ok(namespace)
    }

   fn reconfigure(&mut self,
                  namespace: Uuid,
                  old_config: VersionedReplicas,
                  new_config: VersionedReplicas)
    {
        let to_start = self.namespaces.reconfigure(&namespace,
                                                   old_config.clone(),
                                                   new_config.clone());

        self.peers = old_config.replicas.iter().chain(new_config.replicas.iter()).map(|pid| {
            Pid {
                group: None,
                name: "namespace_mgr".to_string(),
                node: pid.node.clone()
            }
        }).collect();

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
       let mut ctx = VrCtx::new(pid.clone(),
                                old_config.clone(),
                                new_config.clone());
       ctx.idle_timeout = self.idle_timeout.clone();
       ctx.primary_tick_ms = self.primary_tick_ms;
       let state = vr_fsm::startup_reconfiguration;
       let fsm = Fsm::<VrTypes>::new(ctx, state_fn!(state));
       let _ = self.node.spawn(&pid, Box::new(Replica::new(pid.clone(), fsm)));
       self.local_replicas.insert(pid.clone());
   }


    fn start_replica_initial_config(&mut self, pid: Pid, new_config: VersionedReplicas) {
       println!("start pid {:?}", pid);
       let mut ctx = VrCtx::new(pid.clone(),
                                VersionedReplicas::new(),
                                new_config);
       ctx.idle_timeout = self.idle_timeout.clone();
       ctx.primary_tick_ms = self.primary_tick_ms;
       let state = vr_fsm::startup_new_namespace;
       let fsm = Fsm::<VrTypes>::new(ctx, state_fn!(state));
       let _ = self.node.spawn(&pid, Box::new(Replica::new(pid.clone(), fsm)));
       self.local_replicas.insert(pid);
    }

    /// Should only be called outside this module during tests
    pub fn stop(&mut self, pid: &Pid) {
        self.local_replicas.remove(pid);
        let _ = self.node.stop(pid);
    }

    /// Should only be called outside this module during tests
    pub fn restart(&mut self, pid: Pid) {
       let uuid = Uuid::parse_str(pid.group.as_ref().unwrap()).unwrap();
       if let Some((old_config, new_config)) = self.namespaces.get_config(&uuid) {
           let mut ctx = VrCtx::new(pid.clone(),
                                    old_config.clone(),
                                    new_config.clone());
           ctx.idle_timeout = self.idle_timeout.clone();
           ctx.primary_tick_ms = self.primary_tick_ms;
           let state = vr_fsm::startup_recovery;
           let fsm = Fsm::<VrTypes>::new(ctx, state_fn!(state));
           let _ = self.node.spawn(&pid, Box::new(Replica::new(pid.clone(), fsm)));
           self.local_replicas.insert(pid);
       }
    }
}

impl ServiceHandler<Msg> for NamespaceMgr {
    fn init(&mut self, registrar: &Registrar, _: &Node<Msg>) -> rabble::Result<()> {
        self.fsm_timer = try!(registrar.set_interval(self.tick_period as usize)
                              .chain_err(|| "Failed to register request timer"));
        self.management_timer = try!(registrar.set_interval(MANAGEMENT_TICK_MS as usize)
                                     .chain_err(|| "Failed to register request timer"));
        Ok(())
    }

    fn handle_envelope(&mut self,
                       _: &Node<Msg>,
                       envelope: Envelope<Msg>,
                       _: &Registrar) -> rabble::Result<()>
    {
        let Envelope{msg, correlation_id, ..} = envelope;
        match msg {
            rabble::Msg::User(Msg::AdminReq(req)) =>
                self.handle_admin_req(req, correlation_id),
            rabble::Msg::User(Msg::Namespace(msg)) =>
                self.handle_namespace_msg(msg, correlation_id),
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

        if notification.id == self.fsm_timer.get_id() {
            self.fsm_timer.arm();
            self.fsm_tick();
            return Ok(());
        }
        if notification.id == self.management_timer.get_id() {
            self.management_timer.arm();
            self.management_tick();
            return Ok(());
        }

        Err("namespace_mgr got invalid notification: not a timer".into())
    }
}
