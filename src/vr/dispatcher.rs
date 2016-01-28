use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::mpsc::{channel, sync_channel, Sender, Receiver};
use std::thread::{self, JoinHandle};
use rustc_serialize::Encodable;
use msgpack::{Encoder, from_msgpack};
use uuid::Uuid;
use mio::{self, Token};
use fsm::Fsm;
use membership::Member;
use admin::{AdminReq, AdminRpy};
use state::State;
use fsm::StateFn;
use super::replica::{RawReplica, Replica, VersionedReplicas};
use super::vr_fsm::{self, VrCtx, VrTypes, DEFAULT_IDLE_TIMEOUT_MS, DEFAULT_PRIMARY_TICK_MS};
use shared_messages::{NewSessionRequest, NewSessionReply};
use event_loop::{EventLoop, OutControlMsg, OutDataMsg, IncomingMsg};
use super::tenants::Tenants;
use super::vrmsg::VrMsg;
use super::vr_api_messages::VrApiRsp;
use super::envelope::{Envelope, Announcement, PeerEnvelope, ClientEnvelope, ClientReplyEnvelope};
use requests::Requests;
use super::vr_stats::VrStats;

const EVENT_LOOP_QUEUE_SIZE: usize = 1024*128;
const MANAGEMENT_TICK: u64 = 10000; // 10s
const CLIENT_TICK: u64 = 250; // ms
const REQUEST_TIMEOUT: u64 = 5000; // 5s

/// Messages sent to and from peer event loop as serialized Vec<u8>
#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum PeerMsg {
    VrMsg(Replica, VrMsg),
    Peer(Member),
    // Gossiped between dispatchers to ensure all replicas are correctly started on each
    // node and can process VR requests
    Tenants(Tenants)
}

struct Session {
    pub id: Uuid,
    pub tenant_id: Uuid,
    /// A list of Replicas that this client has talked to
    pub replicas: HashSet<Replica>
}

impl Session {
    fn new(session_id: Uuid, tenant_id: Uuid) -> Session {
        Session {
            id: session_id,
            tenant_id: tenant_id,
            replicas: HashSet::new()
        }
    }
}

impl Iterator for Dispatcher {
    type Item = Envelope;

    /// Return the next envelope to be sent. It can be manually sent with self.open_envelope().
    fn next(&mut self) -> Option<Envelope> {
        self.envelopes.pop_front()
    }
}

/// The Dispatcher is in charge of both starting VR FSMs and routing requests to them.
pub struct Dispatcher {
    pub node: Member,
    pub tenants: Tenants,
    pub local_replicas: HashMap<Replica, Fsm<VrTypes>>,

    pub envelopes: VecDeque<Envelope>,

    state: State,
    stats: VrStats,

    /// Timeout configuration for VR Fsms
    idle_timeout_ms: u64,
    primary_tick_ms: u64,

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
    tick_period: u64,

    fsm_tick_id: Uuid,

    /// To be cloned and passed into the constructor of the admin server
    pub admin_tx: Sender<AdminReq>,

    /// Receives requests from the admin server
    admin_rx: Option<Receiver<AdminReq>>,

    /// Processes API requests from v2r2 clients
    client_event_loop_thread: Option<JoinHandle<()>>,

    /// Send messages to the client event loop
    client_event_loop_tx: Option<mio::Sender<IncomingMsg>>,

    client_requests: Requests<Uuid, (Token, u64)>,
    sessions: HashMap<Token, Session>,

    /// The peer event loop processes VR protocol messages sent between dispatchers on different
    /// nodes.
    peer_event_loop_thread: Option<JoinHandle<()>>,
    /// Send messages to the peer event loop
    peer_event_loop_tx: Option<mio::Sender<IncomingMsg>>,

    /// Connections to other nodes' peer event loops. Note that we connect only to other nodes
    /// that sort lower than this node as Member. This allows us to deterministically create a full
    /// mesh of connections with only a single connection between each pair of nodes.
    peer_connections_by_member: HashMap<Member, Token>,
    peer_connections_by_token: HashMap<Token, Member>,

    /// These are connections from unidentified peer nodes. We should receive a message on this
    /// connection that identifies the node. At that point we will move the connection from
    /// `unestablished_peer_connections` to `peer_connections`.
    unestablished_peer_connections: HashSet<Token>,

    /// We periodically need to check our peer_connections for added and removed nodes. We do
    /// this check on a management tick.
    management_tick_id: Uuid
}

impl Dispatcher {
    pub fn new(state: &State) -> Dispatcher {
        let (admin_tx, admin_rx) = channel();
        Dispatcher {
            node: state.members.me.clone(),
            tenants: Tenants::new(),
            local_replicas: HashMap::new(),
            envelopes: VecDeque::new(),
            state: state.clone(),
            stats: VrStats::new(),
            client_event_loop_thread: None,
            client_event_loop_tx: None,
            client_requests: Requests::new(CLIENT_TICK, REQUEST_TIMEOUT),
            sessions: HashMap::new(),
            idle_timeout_ms: DEFAULT_IDLE_TIMEOUT_MS,
            primary_tick_ms: DEFAULT_PRIMARY_TICK_MS,
            tick_period: DEFAULT_PRIMARY_TICK_MS / 3,
            fsm_tick_id: Uuid::new_v4(),
            admin_tx: admin_tx,
            admin_rx: Some(admin_rx),
            peer_event_loop_thread: None,
            peer_event_loop_tx: None,
            peer_connections_by_member: HashMap::new(),
            peer_connections_by_token: HashMap::new(),
            unestablished_peer_connections: HashSet::new(),
            management_tick_id: Uuid::new_v4()
        }
    }

    pub fn run(mut self) -> Vec<JoinHandle<()>> {
        let (peer_data_rx, peer_control_rx) = self.start_peer_loop();
        let (client_data_rx, client_control_rx) = self.start_client_loop();
        self.register_management_tick();
        self.register_fsm_tick();
        self.register_client_tick();
        self.rx_loop(peer_data_rx, peer_control_rx, client_data_rx, client_control_rx)
    }

    /// Start the VR protocol event loop thread
    fn start_peer_loop(&mut self) -> (Receiver<OutDataMsg>, Receiver<OutControlMsg>) {
        let config = self.state.config.read().unwrap();
        let host = config.vr_host.parse().unwrap();
        let mut peer_event_loop = EventLoop::new(host, EVENT_LOOP_QUEUE_SIZE);
        let (data_tx, data_rx) = sync_channel(EVENT_LOOP_QUEUE_SIZE/2);
        let (control_tx, control_rx) = channel();
        let state = self.state.clone();
        self.peer_event_loop_tx = Some(peer_event_loop.sender());
        self.peer_event_loop_thread =
            Some(thread::Builder::new().name("vr_peer_event_loop".to_string()).spawn(move || {
                    peer_event_loop.run(state, data_tx, control_tx)
                }).unwrap());
        (data_rx, control_rx)
    }

    /// Start the V2R2 API client event loop thread
    fn start_client_loop(&mut self) -> (Receiver<OutDataMsg>, Receiver<OutControlMsg>) {
        let config = self.state.config.read().unwrap();
        let host = config.vr_api_host.parse().unwrap();
        let mut client_event_loop = EventLoop::new(host, EVENT_LOOP_QUEUE_SIZE);
        let (data_tx, data_rx) = sync_channel(EVENT_LOOP_QUEUE_SIZE/2);
        let (control_tx, control_rx) = channel();
        let state = self.state.clone();
        self.client_event_loop_tx = Some(client_event_loop.sender());
        self.client_event_loop_thread =
            Some(thread::Builder::new().name("vr_client_event_loop".to_string()).spawn(move || {
                    client_event_loop.run(state, data_tx, control_tx)
                }).unwrap());
        (data_rx, control_rx)
    }

    /// We loop infinitely and attempt to receive data from all our channels/queues and dispatch it
    /// as appropriate. For now this is all done in a single-threaded manner for expediency. It
    /// should transition to a more sophisticated scheduling strategy in the future.
    fn rx_loop(mut self,
               peer_data_rx: Receiver<OutDataMsg>,
               peer_control_rx: Receiver<OutControlMsg>,
               client_data_rx: Receiver<OutDataMsg>,
               client_control_rx: Receiver<OutControlMsg>) -> Vec<JoinHandle<()>> {
        let mut handles = Vec::new();
        let admin_rx = self.admin_rx.take().unwrap();
        handles.push(self.peer_event_loop_thread.take().unwrap());
        handles.push(self.client_event_loop_thread.take().unwrap());
        handles.push(thread::Builder::new().name("dispatcher_loop".to_string()).spawn(move || {
            loop {
                select! {
                    msg = admin_rx.recv() => self.handle_admin_request(msg.unwrap()),
                    msg = peer_data_rx.recv() => self.handle_peer_data_msg(msg.unwrap()),
                    msg = peer_control_rx.recv() => self.handle_peer_control_msg(msg.unwrap()),
                    msg = client_data_rx.recv() => self.handle_client_data_msg(msg.unwrap()),
                    msg = client_control_rx.recv() => self.handle_client_control_msg(msg.unwrap())
                }
            }
        }).unwrap());
        handles
    }

    fn register_management_tick(&mut self) {
        let msg = IncomingMsg::SetTimeout(self.management_tick_id, MANAGEMENT_TICK);
        self.peer_event_loop_tx.as_ref().unwrap().send(msg).unwrap();
    }

    fn register_fsm_tick(&mut self) {
        let msg = IncomingMsg::SetTimeout(self.fsm_tick_id, self.tick_period);
        self.peer_event_loop_tx.as_ref().unwrap().send(msg).unwrap();
    }

    fn register_client_tick(&mut self) {
        // There's only one tick. No need to worry about the id.
        let msg = IncomingMsg::SetTimeout(Uuid::new_v4(), CLIENT_TICK);
        self.client_event_loop_tx.as_ref().unwrap().send(msg).unwrap();
    }

    /// Directly set the view change timeout. Other timeouts and ticks will be set in relation to
    /// this value. This is to prevent breaking timeout invariants such as making the primary send
    /// commits less frequently than the view change idle timeout.
    pub fn set_idle_timeout_ms(&mut self, timeout: u64) {
        self.idle_timeout_ms = timeout;
        self.primary_tick_ms = timeout / 4;
        self.tick_period = self.primary_tick_ms / 3;
    }

    pub fn create_tenant(&mut self, raw_replicas: Vec<RawReplica>) -> Uuid {
        let tenant = Uuid::new_v4();
        let _ = self.create_tenant_(tenant, raw_replicas);
        tenant
    }

    fn create_tenant_(&mut self, tenant: Uuid, raw_replicas: Vec<RawReplica>) -> Result<(), String> {
        let mut new_replicas = Vec::new();
        let orset = self.state.members.get_orset();
        for raw in raw_replicas {
            let mut found = false;
            for member in orset.elements().iter() {
                if *member == raw.node {
                    let replica = Replica::new(tenant.clone(),
                                               RawReplica {name: raw.name, node: member.clone()});

                    new_replicas.push(replica);
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(format!("Node {} is not a member of the cluster", raw.node.name));
            }
        }
        new_replicas.sort();
        let old_config = VersionedReplicas::new();
        let new_config = VersionedReplicas {epoch: 1, op: 0, replicas: new_replicas};
        self.tenants.insert(tenant, old_config.clone(), new_config.clone());
        for r in new_config.replicas.iter().cloned() {
            if self.node == r.node {
                self.start_replica_initial_config(r, new_config.clone());
            }
        }
        Ok(())
    }

    fn start_replica_initial_config(&mut self, replica: Replica, new_config: VersionedReplicas) {
       println!("start replica {:?}", replica);
       let mut ctx = VrCtx::new(replica.clone(),
                                VersionedReplicas::new(),
                                new_config);
       ctx.idle_timeout_ms = self.idle_timeout_ms;
       ctx.primary_tick_ms = self.primary_tick_ms;
       let fun = vr_fsm::startup_new_tenant;
       let mut fsm = Fsm::<VrTypes>::new(ctx, state_fn!(fun));
       self.envelopes.extend(fsm.send(VrMsg::Tick));
       self.local_replicas.insert(replica, fsm);
    }

    /// Should only be called outside this module during tests
    pub fn stop(&mut self, replica: &Replica) {
        self.local_replicas.remove(&replica);
    }

    /// Should only be called outside this module during tests
    pub fn restart(&mut self, replica: Replica) {
       if let Some((old_config, new_config)) = self.tenants.get_config(&replica.tenant) {
           let mut ctx = VrCtx::new(replica.clone(),
                                    old_config.clone(),
                                    new_config.clone());
           ctx.idle_timeout_ms = self.idle_timeout_ms;
           ctx.primary_tick_ms = self.primary_tick_ms;
           let fun = vr_fsm::startup_recovery;
           let mut fsm = Fsm::<VrTypes>::new(ctx, state_fn!(fun));
           // Send an initial tick to transition to proper state
           self.envelopes.extend(fsm.send(VrMsg::Tick));
           self.local_replicas.insert(replica, fsm);
       }
    }

    // We only send one message. We don't send any envelopes we receive.
    fn send_once(&mut self, to: &Replica, msg: VrMsg) {
        if self.node == to.node {
            if let Some(ref mut fsm) = self.local_replicas.get_mut(to) {
                self.envelopes.extend(fsm.send(msg));
            }
        } else {
            self.send_remote(to, msg);
        }
    }

    fn send(&mut self, to: &Replica, msg: VrMsg) {
        if self.node == to.node {
            self.send_local(to, msg);
        } else {
            self.send_remote(to, msg);
        }
    }

    fn send_local(&mut self, to: &Replica, msg: VrMsg) {
        if let Some(ref mut fsm) = self.local_replicas.get_mut(to) {
            self.envelopes.extend(fsm.send(msg));
        }
        self.send_until_empty();
    }

    fn send_remote(&mut self, to: &Replica, msg: VrMsg) {
        if let Some(token) = self.peer_connections_by_member.get(&to.node) {
            let encoded = Encoder::to_msgpack(&PeerMsg::VrMsg(to.clone(), msg)).unwrap();
            let msg = IncomingMsg::WireMsg(token.clone(), encoded);
            self.peer_event_loop_tx.as_ref().unwrap().send(msg).unwrap();
        }
    }

    fn handle_client_data_msg(&mut self, msg: OutDataMsg) {
        match msg {
            OutDataMsg::TcpMsg(token, data) => {
                if let Ok(NewSessionRequest(tenant_id)) = from_msgpack(&data) {
                    return self.new_session(token, tenant_id);
                }

                match from_msgpack(&data) {
                    Ok(ClientEnvelope {to, msg: vrmsg @ VrMsg::ClientRequest {..}}) => {
                        if let VrMsg::ClientRequest {session_id, request_num, ..} = vrmsg.clone() {
                            let valid_session = match self.sessions.get(&token) {
                                Some(ref session) if session.id == session_id => true,
                                _ => false
                            };
                            if valid_session {
                                self.stats.client_req_count += 1;
                                self.client_requests.insert(session_id.clone(), (token, request_num));
                                self.send(&to, vrmsg);
                                if let Some(session) = self.sessions.get_mut(&token) {
                                    session.replicas.insert(to);
                                }
                            } else {
                                let reason = format!("Invalid session id: {}", session_id);
                                self.deregister_client(token, reason);
                            }
                        }
                    },
                    Ok(_) => {
                        let reason = "Msg other than ClientRequest received".to_string();
                        self.deregister_client(token, reason);
                    },
                    Err(_) => {
                        let reason = "Received improperly encoded msgpack data".to_string();
                        self.deregister_client(token, reason);
                    }
                }
            },
            OutDataMsg::Tick(_) => {
                self.stats.client_tick_count += 1;
                // Clone the sender and counter to get around the borrow checker.
                let tx = self.client_event_loop_tx.as_ref().unwrap().clone();
                let mut count = self.stats.client_reply_count;
                self.client_requests.expire(|_session_id, (token, request_num)| {
                    let timeout = VrMsg::ClientReply {epoch: 0,
                                                      view: 0,
                                                      request_num: request_num,
                                                      value: VrApiRsp::Timeout};
                    count += 1;
                    send_client_reply(&tx, token, timeout);
                });
                self.stats.client_reply_count = count;
            }
        }
    }

    fn handle_client_control_msg(&mut self, msg: OutControlMsg) {
        match msg {
            OutControlMsg::NewSock(_token) => self.stats.client_new_sock_count += 1,
            OutControlMsg::Deregister(token) => {
                self.stats.client_deregister_count += 1;
                if let Some(session) = self.sessions.remove(&token) {
                    let _ = self.client_requests.remove(&session.id);
                    for to in session.replicas.iter() {
                        self.send(to, VrMsg::SessionClosed(session.id.clone()));
                    }
                }
            }
        }
    }

    fn new_session_reply(&mut self, tenant_id: &Uuid) -> NewSessionReply {
        match self.tenants.primaries.get(&tenant_id) {
            Some(primary) => {
                NewSessionReply::Redirect {host: primary.node.vr_api_host.clone()}
            },
            None => {
                if self.tenants.exists(&tenant_id) {
                    NewSessionReply::Retry(1000)
                } else {
                    NewSessionReply::NoSuchTenant
                }
            }
        }
    }

    fn invalid_client_session_attempt(&mut self, token: Token, tenant_id: Uuid) {
        let reply = self.new_session_reply(&tenant_id);
        self.send_client_error_to_peer(token, reply);
    }

    fn new_session(&mut self, token: Token, tenant_id: Uuid) {
        let session_id = Uuid::new_v4();
        let reply = match self.tenants.primaries.get(&tenant_id) {
            Some(primary) => {
                if primary.node == self.node {
                    self.sessions.insert(token, Session::new(session_id.clone(), tenant_id.clone()));
                    NewSessionReply::SessionId {session_id: session_id, primary: primary.clone()}
                } else {
                    NewSessionReply::Redirect {host: primary.node.vr_api_host.clone()}
                }
            },
            None => {
                if self.tenants.exists(&tenant_id) {
                    NewSessionReply::Retry(1000)
                } else {
                    NewSessionReply::NoSuchTenant
                }
            }
        };
        self.send_client_reply(token, reply);
    }

    fn deregister_client(&mut self, token: Token, reason: String) {
        let dereg_msg = IncomingMsg::Deregister(token.clone(), reason);
        self.client_event_loop_tx.as_ref().unwrap().send(dereg_msg).unwrap();
    }

    fn handle_admin_request(&mut self, req: AdminReq) {
        match req {
            AdminReq::GetTenants {token, reply_tx} => {
                reply_tx.send(AdminRpy::Tenants {token: token, tenants: self.tenants.clone()});
            },
            AdminReq::GetReplica {token, replica, reply_tx} => {
                match self.get_state(&replica) {
                    Some((state, ctx)) => {
                        reply_tx.send(AdminRpy::Replica {token: token, state: state, ctx: ctx});
                    },
                    None => {
                        reply_tx.send(AdminRpy::ReplicaNotFound {token: token, replica: replica});
                    }
                }
            },
            AdminReq::CreateTenant {token, tenant, replicas, reply_tx} => {
                match self.create_tenant_(tenant.clone(), replicas) {
                    Ok(()) => reply_tx.send(AdminRpy::TenantId {token: token, id: tenant}),
                    Err(e) => reply_tx.send(AdminRpy::Error(token, e))
                }
            },
            AdminReq::GetVrStats {token, reply_tx} => {
                reply_tx.send(AdminRpy::VrStats {token: token, stats: self.stats.to_string()});
            },
            AdminReq::GetPrimary {token, tenant_id, reply_tx} => {
                let val = match self.tenants.primaries.get(&tenant_id) {
                    Some(replica) => Some(replica.clone()),
                    None => None
                };
                reply_tx.send(AdminRpy::Primary {token: token, replica: val});
            },
            AdminReq::GetNewSessionReply {token, tenant_id, reply_tx} => {
                let reply = self.new_session_reply(&tenant_id);
                reply_tx.send(AdminRpy::NewSessionReply {token: token, reply: reply});
            },
            _ => println!("Received unknown AdminReq in dispatcher: {:?}", req)
        }
    }

    fn handle_peer_data_msg(&mut self, msg: OutDataMsg) {
        match msg {
            OutDataMsg::TcpMsg(token, data) => {
                // Make sure that a client is not trying to connect to the peer server.
                if let Ok(NewSessionRequest(tenant_id)) = from_msgpack(&data) {
                    return self.invalid_client_session_attempt(token, tenant_id);
                }

                match from_msgpack(&data) {
                    Ok(PeerMsg::VrMsg(to, vrmsg)) => {
                        self.send_local(&to, vrmsg);
                    }
                    Ok(PeerMsg::Peer(member)) => {
                        self.establish_connection(token, member);
                    },
                    Ok(PeerMsg::Tenants(tenants)) => self.check_tenants(tenants),
                    Err(_) => {
                        // Note that we will get an OutControlMsg::Deregister as a result of this
                        // which will allow us to clean up our state just like if the event loop
                        // lost the connection.
                        let reason = "Received improperly encoded msgpack data".to_string();
                        let dereg_msg = IncomingMsg::Deregister(token, reason);
                        self.peer_event_loop_tx.as_ref().unwrap().send(dereg_msg).unwrap();
                    }
                }
            },
            OutDataMsg::Tick(uuid) => {
                if uuid == self.management_tick_id {
                    self.management_tick()
                } else {
                    self.fsm_tick()
                }
            }
        }
    }

    fn handle_peer_control_msg(&mut self, msg: OutControlMsg) {
        match msg {
            OutControlMsg::NewSock(token) => {
                if !self.peer_connections_by_token.contains_key(&token) {
                    self.unestablished_peer_connections.insert(token);
                } else {
                    // We are the connecting client. Let's tell the peer who we are.
                    let encoded = Encoder::to_msgpack(&PeerMsg::Peer(self.node.clone())).unwrap();
                    let peer_msg = IncomingMsg::WireMsg(token.clone(), encoded);
                    self.peer_event_loop_tx.as_ref().unwrap().send(peer_msg).unwrap();
                }
            },
            OutControlMsg::Deregister(token) => {
                match self.peer_connections_by_token.remove(&token) {
                    Some(member) => {
                        self.peer_connections_by_member.remove(&member);
                    },
                    None => {
                        self.unestablished_peer_connections.remove(&token);
                    }
                }
            }
        }
    }

    fn management_tick(&mut self) {
        let members = self.state.members.get_orset().elements();
        self.connect_new_members(&members);
        self.disconnect_old_members(&members);
        self.gossip_tenants();
    }

    fn connect_new_members(&mut self, members: &Vec<Member>) {
        for m in members {
            if *m < self.node {
                if !self.peer_connections_by_member.contains_key(&m) {
                    self.maybe_connect(&m);
                }
            }
        }
    }

    fn disconnect_old_members(&mut self, members: &Vec<Member>) {
        for (member, token) in self.peer_connections_by_member.iter() {
            if !members.contains(member) {
                let reason = format!("Member {:?} no longer part of cluster", member);
                let dereg_msg = IncomingMsg::Deregister(token.clone(), reason);
                self.peer_event_loop_tx.as_ref().unwrap().send(dereg_msg).unwrap();
            }
        }
    }

    fn maybe_connect(&mut self, member: &Member) {
        match member.vr_host.parse() {
            Err(_) => println!("Failed to parse ip: {}", member.vr_host),
            Ok(addr) => {
                let token = self.state.next_token();
                let msg = IncomingMsg::Connect(token, addr);
                self.peer_event_loop_tx.as_ref().unwrap().send(msg).unwrap();
                self.establish_connection(token, member.clone());
            }
        }
    }

    /// Send all tenant info to other dispatchers so that they can correctly start replicas as
    /// needed. Note that this could be made more efficient by only gossiping versions and then
    /// allowing individual dispatchers to request replica info for outdated tenants.
    fn gossip_tenants(&self) {
        let encoded = Encoder::to_msgpack(&PeerMsg::Tenants(self.tenants.clone())).unwrap();
        for (token, _) in self.peer_connections_by_token.iter() {
            let msg = IncomingMsg::WireMsg(token.clone(), encoded.clone());
            self.peer_event_loop_tx.as_ref().unwrap().send(msg).unwrap();
        }
    }

    fn fsm_tick(&mut self) {
        for (_, fsm) in self.local_replicas.iter_mut() {
            self.envelopes.extend(fsm.send(VrMsg::Tick));
        }
        self.send_until_empty();
    }

    fn establish_connection(&mut self, token: Token, peer: Member) {
        let _ = self.unestablished_peer_connections.remove(&token);
        self.peer_connections_by_token.insert(token.clone(), peer.clone());
        self.peer_connections_by_member.insert(peer, token);
    }

    /// When we receive a copy of current tenants from another node we need to see if our tenants
    /// are outdated. We then configure them and start/stop replicas as needed.
    fn check_tenants(&mut self, tenants: Tenants) {
        for (tenant_id, &(ref old_config, ref new_config)) in tenants.map.iter() {
            if self.tenants.exists(tenant_id) {
                self.reconfigure(tenant_id.clone(), old_config.clone(), new_config.clone());
            } else {
                self.tenants.insert(tenant_id.clone(), old_config.clone(), new_config.clone());
                for r in new_config.replicas.iter().cloned() {
                    if self.node == r.node {
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

   fn reconfigure(&mut self,
                  tenant: Uuid,
                  old_config: VersionedReplicas,
                  new_config: VersionedReplicas) {
        let to_start = self.tenants.reconfigure(&tenant,
                                                old_config.clone(),
                                                new_config.clone());
        for replica in to_start {
            if self.node == replica.node {
                self.start_replica_reconfig(&replica, &old_config, &new_config);
            }
        }
    }

   fn start_replica_reconfig(&mut self,
                             replica: &Replica,
                             old_config: &VersionedReplicas,
                             new_config: &VersionedReplicas) {
       let mut ctx = VrCtx::new(replica.clone(),
                                old_config.clone(),
                                new_config.clone());
       ctx.idle_timeout_ms = self.idle_timeout_ms;
       ctx.primary_tick_ms = self.primary_tick_ms;
       let fun = vr_fsm::startup_reconfiguration;
       let mut fsm = Fsm::<VrTypes>::new(ctx, state_fn!(fun));
       self.envelopes.extend(fsm.send(VrMsg::Tick));
       self.local_replicas.insert(replica.clone(), fsm);
       self.send_until_empty();
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

    pub fn send_until_empty(&mut self) {
        loop {
            match self.next() {
                Some(envelope) => self.open_envelope(envelope),
                None => return
            }
        }
    }

    pub fn open_envelope(&mut self, envelope: Envelope) {
        match envelope {
            Envelope::Peer(PeerEnvelope {to, msg, ..}) => self.send_once(&to, msg),
            Envelope::Announcement(announcement) => self.handle_announcement(announcement),
            Envelope::Client(ClientEnvelope {to, msg}) => self.send_once(&to, msg),
            Envelope::ClientReply(ClientReplyEnvelope {to, msg}) => {
                if let Some((token, _)) = self.client_requests.remove(&to) {
                    self.send_client_reply(token, msg);
                }
            }
        }
    }

    fn handle_announcement(&mut self, announcement: Announcement) {
        match announcement {
            Announcement::Reconfiguration {tenant, old_config, new_config} =>
                self.reconfigure(tenant, old_config, new_config),
            Announcement::Stop(replica) => self.stop(&replica),
            Announcement::NewPrimary(replica) => {
                self.tenants.primaries.insert(replica.tenant.clone(), replica);
            },
            Announcement::ClearPrimary(tenant_id) => {
                self.tenants.primaries.remove(&tenant_id);
            }
        }
    }

    fn send_client_reply<T: Encodable>(&mut self, token: Token, msg: T) {
        self.stats.client_reply_count += 1;
        let encoded = Encoder::to_msgpack(&msg).unwrap();
        let msg = IncomingMsg::WireMsg(token, encoded);
        self.client_event_loop_tx.as_ref().unwrap().send(msg).unwrap();
    }

    /// When a client improperly connects to the peer server it will send a NewSessionRequest. In
    /// this case we respond with a NewSessionReply indicating a redirect to the primary for the
    /// given tenant, or an error.
    fn send_client_error_to_peer(&mut self, token: Token, msg: NewSessionReply) {
        self.stats.client_reply_count += 1;
        let encoded = Encoder::to_msgpack(&msg).unwrap();
        let msg = IncomingMsg::WireMsg(token, encoded);
        self.peer_event_loop_tx.as_ref().unwrap().send(msg).unwrap();
    }
}

fn send_client_reply(tx: &mio::Sender<IncomingMsg>, token: Token, msg: VrMsg) {
    let encoded = Encoder::to_msgpack(&msg).unwrap();
    let msg = IncomingMsg::WireMsg(token, encoded);
    tx.send(msg).unwrap();
}
