use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::iter::FromIterator;
use uuid::Uuid;
use fsm::{Fsm, Msg};
use membership::Member;
use super::replica::{RawReplica, Replica, VersionedReplicas};
use super::vr_fsm::{StartupState, VrCtx, VrHandler, DEFAULT_IDLE_TIMEOUT_MS, DEFAULT_PRIMARY_TICK_MS};
use super::messages::*;

/// Messages sent to the Dispatcher. They are sent to the local dispatcher from an FSM on the same
/// node. They are also gossiped between dispatchers on different nodes.
#[derive(Debug, Clone)]
pub enum DispatchMsg {
    Reconfiguration {tenant: Uuid, old_config: VersionedReplicas, new_config: VersionedReplicas},
    Stop(Replica)
}

/// The Dispatcher is in charge of both starting VR FSMs and routing requests to them.
pub struct Dispatcher {
    pub node: Member,
    pub tenants: HashMap<Uuid, (VersionedReplicas, VersionedReplicas)>,
    pub local_replicas: HashMap<Replica, Fsm<VrHandler>>,

    /// Timeout configuration for VR Fsms
    idle_timeout_ms: i64,
    primary_tick_ms: i64,

    /// The amount of time between VrMsg::Tick messages being sent to replicas. By default this
    /// value is set at 1/3 * primary_tick_ms for the following reasons:
    /// 1) We want to send timeout messages as close to the tick timeout as possible (therfore use
    ///    smaller tick values)
    /// 2) We don't want to unnecessarily spam the FSMs with ticks and chew CPU cycles (therefore
    ///    use larger tick values)
    /// This setting allows us to maximally send a commit message 1/3 primary_tick_ms late. If we
    /// always ensure that `idle_timeout_ms` is at least double `primary_tick_ms` we can minimize
    /// unnecessary view changes. Note that for backups, the ticks are higher frequency than
    /// necessary, but this is the tradeoff made for having a single Tick message.
    tick_frequency: i64,

    /// Receives output messages from local VrHandlers
    fsm_receiver: Receiver<Envelope>,
    /// To be cloned and passed into the constructor of VrCtx
    fsm_sender: Sender<Envelope>,

    /// Receives client responses that can either be used directly in testing or forwarded to the
    /// event_loop and then actual client in production.
    client_reply_receiver: Receiver<ClientReplyEnvelope>,

    /// To be cloned and passed into the constructor of VrCtx
    client_reply_sender: Sender<ClientReplyEnvelope>,

    /// To be cloned and passed into the constructor of VrCtx
    dispatch_sender: Sender<DispatchMsg>,

    /// Receives requests destined for the dispatcher
    dispatch_receiver: Receiver<DispatchMsg>,

    // TODO: Add a channel to the event loop that sends messages to remote replicas
}

impl Dispatcher {
    pub fn new(node: Member) -> Dispatcher {
        let (tx, rx) = channel();
        let (cli_tx, cli_rx) = channel();
        let (dispatch_tx, dispatch_rx) = channel();
        Dispatcher {
            node: node,
            tenants: HashMap::new(),
            local_replicas: HashMap::new(),
            fsm_receiver: rx,
            fsm_sender: tx,
            client_reply_receiver: cli_rx,
            client_reply_sender: cli_tx,
            dispatch_sender: dispatch_tx,
            dispatch_receiver: dispatch_rx,
            idle_timeout_ms: DEFAULT_IDLE_TIMEOUT_MS,
            primary_tick_ms: DEFAULT_PRIMARY_TICK_MS,
            tick_frequency: DEFAULT_PRIMARY_TICK_MS/ 3,
        }
    }

    /// Directly set the view change timeout. Other timeouts and ticks will be set in relation to
    /// this value. This is to prevent breaking timeout invariants such as making the primary send
    /// commits less frequently than the view change idle timeout.
    pub fn set_idle_timeout_ms(&mut self, timeout: i64) {
        self.idle_timeout_ms = timeout;
        self.primary_tick_ms = timeout / 4;
        self.tick_frequency = self.primary_tick_ms / 3;
    }

    pub fn create_test_tenant(&mut self, raw_replicas: Vec<RawReplica>) -> Uuid {
        let tenant = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
        self.create_tenant_(tenant, raw_replicas);
        tenant
    }

    pub fn create_tenant(&mut self, raw_replicas: Vec<RawReplica>) -> Uuid {
        let tenant = Uuid::new_v4();
        self.create_tenant_(tenant, raw_replicas);
        tenant
    }

    fn create_tenant_(&mut self, tenant: Uuid, raw_replicas: Vec<RawReplica>) {
        let mut new_replicas = Vec::new();
        for r in raw_replicas {
            new_replicas.push(Replica::new(tenant.clone(), r));
        }
        new_replicas.sort();
        let old_config = VersionedReplicas::new();
        let new_config = VersionedReplicas {epoch: 1, op: 0, replicas: new_replicas};
        self.tenants.insert(tenant, (old_config.clone(), new_config.clone()));
        for r in new_config.replicas.iter().cloned() {
            if self.node == r.node {
                self.register(r, new_config.clone());
            }
        }
    }

    fn register(&mut self, replica: Replica, new_config: VersionedReplicas) {
       let mut ctx = VrCtx::new(replica.clone(),
                                VersionedReplicas::new(),
                                new_config,
                                StartupState::InitialConfig,
                                self.fsm_sender.clone(),
                                self.client_reply_sender.clone(),
                                self.dispatch_sender.clone());
       ctx.idle_timeout_ms = self.idle_timeout_ms;
       ctx.primary_tick_ms = self.primary_tick_ms;
       let fsm = Fsm::<VrHandler>::new(ctx);
       self.local_replicas.insert(replica, fsm);
    }

    /// Should only be called outside this module during tests
    pub fn stop(&mut self, replica: &Replica) {
        // TODO: Does this actually stop a threaded fsm?
        self.local_replicas.remove(&replica);
    }

    /// Should only be called outside this module during tests
    pub fn restart(&mut self, replica: Replica) {
       if let Some(&(ref old_config, ref new_config)) = self.tenants.get(&replica.tenant) {
           let mut ctx = VrCtx::new(replica.clone(),
                                    old_config.clone(),
                                    new_config.clone(),
                                    StartupState::Recovery,
                                    self.fsm_sender.clone(),
                                    self.client_reply_sender.clone(),
                                    self.dispatch_sender.clone());
           ctx.idle_timeout_ms = self.idle_timeout_ms;
           ctx.primary_tick_ms = self.primary_tick_ms;
           let mut fsm = Fsm::<VrHandler>::new(ctx);
           // Send an initial tick to transition to proper state
           fsm.send_msg(VrMsg::Tick);
           self.local_replicas.insert(replica, fsm);
       }
    }

    pub fn send_broadcast(&mut self, replicas: &Vec<Replica>, msg: VrMsg) {
        for r in replicas {
            self.send(r, msg.clone());
        }
    }

    pub fn send(&mut self, to: &Replica, msg: VrMsg) {
        if self.node == to.node {
            self.send_local(to, msg);
        } else {
            self.send_remote(to, Box::new(msg) as Msg);
        }
    }

    pub fn send_local(&mut self, to: &Replica, msg: VrMsg) {
        if let Some(ref mut fsm) = self.local_replicas.get_mut(to) {
            fsm.send_msg(msg);
        }
    }

    pub fn send_remote(&mut self, _to: &Replica, _msg: Msg) {
        unimplemented!();
    }

    pub fn handle_dispatch_msg(&mut self, msg: DispatchMsg) {
        match msg {
            DispatchMsg::Reconfiguration {tenant, old_config, new_config} => {
                if let Some(&mut(ref mut saved_old_config, ref mut saved_new_config)) =
                    self.tenants.get_mut(&tenant) {

                    // This is an old reconfig message
                    if new_config.epoch <= saved_new_config.epoch { 
                        return; 
                    }
                    let new_set = HashSet::<Replica>::from_iter(new_config.replicas.clone());
                    // We want to use the actual running nodes here because we are trying to determine which
                    // nodes to start locally
                    let old_set = HashSet::<Replica>::from_iter(saved_new_config.replicas.clone());
                    let to_start: Vec<Replica> = new_set.difference(&old_set).cloned().collect();
                    *saved_old_config = old_config;
                    *saved_new_config = new_config;
                    for replica in to_start {
                        if self.node == replica.node {
                            let mut ctx = VrCtx::new(replica.clone(),
                                                     saved_old_config.clone(),
                                                     saved_new_config.clone(),
                                                     StartupState::Reconfiguration,
                                                     self.fsm_sender.clone(),
                                                     self.client_reply_sender.clone(),
                                                     self.dispatch_sender.clone());
                            ctx.idle_timeout_ms = self.idle_timeout_ms;
                            ctx.primary_tick_ms = self.primary_tick_ms;
                            let fsm = Fsm::<VrHandler>::new(ctx);
                            self.local_replicas.insert(replica, fsm);
                        }
                    }
                }
            // TODO: We need to gossip this message around to the other node's dispatchers so that they
            // can start their own new local replicas and reconfigure their state.
            },
            DispatchMsg::Stop(replica) => self.stop(&replica)
        }
    }

    pub fn try_recv_dispatch_msg(&self) -> Result<DispatchMsg, ()> {
        match self.dispatch_receiver.try_recv() {
            Ok(msg) => Ok(msg),
            // We ignore disconnects. Since we hold a reference to our corresponding Sender it will
            // never disconnect.
            Err(_) => Err(())
        }
    }

    pub fn try_recv_client_reply(&self) -> Result<ClientReplyEnvelope, ()> {
        match self.client_reply_receiver.try_recv() {
            Ok(envelope) => Ok(envelope),
            // We ignore disconnects. Since we hold a reference to our corresponding Sender it will
            // never disconnect.
            Err(_) => Err(())
        }
    }

    /// Try to receive any messages sent by one of the local FSMs.
    /// This method is used to receive envelopes and dispatch them.
    /// It is made public so that it can be used in testing to track all messages sent
    /// by local FSMs.
    pub fn try_recv(&mut self) -> Result<Envelope, ()> {
        match self.fsm_receiver.try_recv() {
            Ok(envelope) => Ok(envelope),
            // We ignore disconnects. Since we hold a reference to our corresponding Sender it will
            // never disconnect.
            Err(_) => Err(())
        }
    }

    /// This will call Fsm::get_state(&self) for the given replica and return the internal state
    /// of the handler Fsm. This call will block waiting for receipt of the state for threaded_fsms.
    /// It should not be used for replicas not on the local node. In that case it will return
    /// `None`. Note also that for large logs, this could be expensive.
    pub fn get_state(&self, replica: &Replica) -> Option<(&'static str, VrCtx)> {
        if self.node != replica.node {
            return None;
        }
        if let Some(ref fsm) = self.local_replicas.get(&replica) {
            Some(fsm.get_state())
        } else {
            None
        }
    }

    /// This function will block the currently running thread until all messages in the fsm_receiver
    /// have been dispatched. Completion may never occur since each message sent to an fsm may
    /// result in more messages being received by the dispatcher.
    pub fn dispatch_all_received_messages(&mut self) {
        loop {
            match self.try_recv() {
                Ok(Envelope {to, msg, ..}) => self.send(&to, msg),
                Err(()) => break
            }
        }
    }

    // This is useful when testing operations that include view changes. After a view change, th
    //  new primary replays any uncommitted client operations. This allows us to just drop them and
    // get a response to the latest client request.
    pub fn drop_all_client_replies(&self) {
        while let Ok(_envelope) = self.client_reply_receiver.try_recv() {};
    }

    #[cfg(debug_assertions)]
    pub fn save_state(&self) -> DispatcherState {
        let mut replica_states = HashMap::new();
        for (replica, fsm) in self.local_replicas.iter() {
            replica_states.insert(replica.clone(), fsm.clone());
        }
        DispatcherState {
            node: self.node.clone(),
            tenants: self.tenants.clone(),
            local_replicas: replica_states
        }
    }

    #[cfg(debug_assertions)]
    pub fn restore_state(&mut self, state: &DispatcherState) {
        self.tenants = state.tenants.clone();
        let mut local_replicas = HashMap::new();
        for (replica, fsm) in state.local_replicas.iter() {
            local_replicas.insert(replica.clone(), fsm.clone());
        }
        self.local_replicas = local_replicas;
    }
}

/// This structure is used to save and restore dispatcher state during debugging
/// Note that states are only stored/retrieved after all messages have been dispatched. Therefore
/// there should be nothing left in the channels that requires saving.
#[cfg(debug_assertions)]
#[derive(Clone)]
pub struct DispatcherState {
    pub node: Member,
    pub tenants: HashMap<Uuid, (VersionedReplicas, VersionedReplicas)>,
    pub local_replicas: HashMap<Replica, Fsm<VrHandler>>
}

