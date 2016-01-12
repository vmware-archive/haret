use std::thread;
use std::thread::JoinHandle;
use std::io::{Error, ErrorKind};
use std::sync::mpsc::{channel, sync_channel, Sender, Receiver};
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use time::{SteadyTime, Duration};
use uuid::Uuid;
use mio::{self, Token};
use msgpack::{Encoder, from_msgpack};
use event_loop::{EventLoop, OutControlMsg, OutDataMsg, IncomingMsg};
use membership::Member;
use orset::ORSet;
use state::State;
use admin::{AdminReq, AdminRpy};
use super::ClusterMsg;
use debug_sender::DebugSender;

const EVENT_LOOP_QUEUE_SIZE: usize = 1000;
const TICK_TIME: u64 = 1000; // milliseconds

pub struct ClusterServer {
    state: State,
    addr: String,
    pending_joins: HashMap<Token, (Token, DebugSender<AdminRpy>)>,

    // Keep track of the time of the last received message on each socket
    // We don't want just another field in `established_by_token`, because it's possible that we
    // have a connection where we never get a message after connection and we need that to timeout
    // as well.
    receipts: HashMap<Token, SteadyTime>,

    // We only want one connection to each peer. An established channel is one in which a Members
    // msg is received. In order to tear down connections deterministically, we always deregister
    // the connection from the client with the lower ip:port. `unestablished_clients` maintains the
    // connections that this node is the connecting client of and that have not yet been
    // established.
    unestablished_clients: HashSet<Token>,
    established_by_addr: HashMap<String, (bool, Token)>,
    established_by_token: HashMap<Token, (bool, String)>,

    event_loop_tx: Option<mio::Sender<IncomingMsg>>
}

impl ClusterServer {
    pub fn new(state: State) -> ClusterServer {
        let addr = {
            let config = state.config.read().unwrap();
            config.cluster_host.parse().unwrap()
        };
        ClusterServer {
            state: state,
            addr: addr,
            pending_joins: HashMap::new(),
            receipts: HashMap::new(),
            unestablished_clients: HashSet::new(),
            established_by_addr: HashMap::new(),
            established_by_token: HashMap::new(),
            event_loop_tx: None
        }
    }

    pub fn run(mut self, admin_rx: Receiver<AdminReq>) -> Vec<JoinHandle<()>> {
        let host = {
            let config = self.state.config.read().unwrap();
            config.cluster_host.parse().unwrap()

        };
        let mut event_loop = EventLoop::new(host, 5000);
        let (data_tx, data_rx) = sync_channel(EVENT_LOOP_QUEUE_SIZE/2);
        let (control_tx, control_rx) = channel();
        let state = self.state.clone();
        self.event_loop_tx = Some(event_loop.sender());
        let mut handles = Vec::new();
        handles.push(
            thread::Builder::new().name("cluster_server_event_loop".to_string()).spawn(move || {
                event_loop.run(state, data_tx, control_tx);
            }).unwrap());
        self.register_tick();
        handles.push(
            thread::Builder::new().name("cluster_server".to_string()).spawn(move || {
                loop {
                    select! {
                        msg = data_rx.recv() => self.handle_data_msg(msg.unwrap()),
                        msg = control_rx.recv() => self.handle_control_msg(msg.unwrap()),
                        msg = admin_rx.recv() => self.handle_admin_msg(msg.unwrap())
                    }
                }
            }).unwrap());
        handles
    }

    fn register_tick(&mut self) {
        let msg = IncomingMsg::SetTimeout(Uuid::new_v4(), TICK_TIME);
        self.event_loop_tx.as_ref().unwrap().send(msg).unwrap();
    }

    fn handle_admin_msg(&mut self, msg: AdminReq) {
        match msg {
            AdminReq::Join {token, ipstr, reply_tx} => self.start_join(token, ipstr, reply_tx),
            _ => println!("Received unknown AdminReq in cluster handler {:?}", msg)
        }
    }

    fn handle_data_msg(&mut self, msg: OutDataMsg) {
        match msg {
            OutDataMsg::TcpMsg(token, data) => {
                match from_msgpack(&data) {
                    Ok(ClusterMsg::Members(addr, orset)) => {
                        self.maybe_establish_connection(token, addr);
                        self.join_members(orset, token);
                        self.check_connections();
                    },
                    Ok(ClusterMsg::Ping) => {
                        self.receipts.insert(token, SteadyTime::now());
                    },
                    Err(_) => {
                        let reason =
                            "Received improperly encoded msgpack data at cluster server".to_string();
                        self.send_deregister_to_event_loop(token, reason);
                    }
                }
            },
            OutDataMsg::Tick(_) => {
                self.check_liveness();
                self.send_pings();
                self.check_connections();
            }
        }
    }

    fn handle_control_msg(&mut self, msg: OutControlMsg) {
        match msg {
            OutControlMsg::NewSock(token) => self.connect(token),
            OutControlMsg::Deregister(token) => self.deregister(token)
        }
    }

    fn connect(&mut self, token: Token) {
        self.send_members(token);
        self.receipts.insert(token, SteadyTime::now());
    }

    fn deregister(&mut self, token: Token) {
        if let Some((_, addr)) = self.established_by_token.remove(&token) {
            self.established_by_addr.remove(&addr);
            self.state.members.disconnected(&addr);
        } else {
            self.unestablished_clients.remove(&token);
        }
        let res = self.pending_joins.remove(&token);
        if let Some((join_token, reply_tx)) = res {
            let err = Error::new(ErrorKind::NotConnected, "Could not connect to server");
            self.join_err(join_token, err.to_string(), reply_tx);
        }
        self.receipts.remove(&token);
    }

    fn send_deregister_to_event_loop(&self, token: Token, reason: String) {
        let dereg_msg = IncomingMsg::Deregister(token.clone(), reason);
        self.event_loop_tx.as_ref().unwrap().send(dereg_msg).unwrap();
    }

    /// The current tick comes in at one second intervals, meaning each node should send a `Ping`
    /// every second. If we haven't received a Ping within 5 seconds, the connection should be
    /// considered dead and closed.
    // TODO: As an optimization, we should only check connections that we know will expire in the
    // next tick.
    // TODO: need a good way to test this: Either network partition, or test code that doesn't send
    // Pings
    fn check_liveness(&mut self) {
        let now = SteadyTime::now();
        let to_remove =
            self.receipts.iter().filter(|&(_, &time)| now - time > Duration::seconds(5));
        for (&token, _) in to_remove {
            let reason = format!("Cluster connection for {:?} timed out", token);
            self.send_deregister_to_event_loop(token, reason);
        }
    }

    fn send_pings(&self) {
        let encoded = Encoder::to_msgpack(&ClusterMsg::Ping).unwrap();
        let event_loop_tx = self.event_loop_tx.as_ref().unwrap();
        for (&token, _) in self.receipts.iter() {
            let msg = IncomingMsg::WireMsg(token, encoded.clone());
            event_loop_tx.send(msg).unwrap();
        }
    }


    fn maybe_establish_connection(&mut self, token: Token, addr: String) {
        let maybe_token = self.pick_token_to_deregister(token, &addr);
        match maybe_token {
            Some(deregister_token) => {
                let reason = format!("Peer {:?} already connected", addr);
                if deregister_token == token {
                    // Deregister the incoming connection
                    self.send_deregister_to_event_loop(token, reason);
                } else {
                    // Deregister the already established connection
                    // Cleanup connection here and not in deregister callback, because we don't want
                    // to call self.state.members.disconnected();
                    self.established_by_token.remove(&deregister_token);
                    self.established_by_addr.remove(&addr);
                    self.send_deregister_to_event_loop(deregister_token, reason);
                    self.establish_connection(token, addr);
                }
            },
            None => self.establish_connection(token, addr)
        }
    }

    // This function wouldn't be necessary if the damn borrow checker didn't match lifetimes to
    // scope.
    // See 'Borrow checker improvements here: http://blog.rust-lang.org/2015/08/14/Next-year.html
    fn pick_token_to_deregister(&self, token: Token, addr: &String) -> Option<Token>{
        if let Some(&(ref is_client, ref established_token)) = self.established_by_addr.get(addr) {
            if (*is_client && self.addr < *addr) || (!*is_client && self.addr > *addr) {
                Some(*established_token)
            } else {
                Some(token)
            }
        } else {
            None
        }
    }

    fn establish_connection(&mut self, token: Token, addr: String) {
        let is_client = self.unestablished_clients.remove(&token);
        self.established_by_addr.insert(addr.clone(), (is_client, token));
        self.established_by_token.insert(token, (is_client, addr));
    }

    // TODO: Where do we check the validity of ip addresses and report errors? Here they are just
    // excluded via the flat_map().
    /// Ensure connections are correct based on membership state
    fn check_connections(&mut self) {
        let addrs: Vec<SocketAddr> = self.to_connect().iter().flat_map(|ip| ip.parse()).collect();
        for addr in addrs {
            self.init_connect(addr);
        }

        let addrs: Vec<String> = self.to_disconnect().iter().cloned().map(|ip| ip).collect();
        for addr in addrs {
            if let Some(&(_, ref token)) = self.established_by_addr.get(&addr) {
                let reason = format!("{:?} no longer part of cluster membership", addr);
                self.send_deregister_to_event_loop(*token, reason);
            }
        }

        let addrs: Vec<String> = self.to_membership_connect().iter().cloned().map(|ip| ip).collect();
        for addr in addrs {
            self.state.members.connected(&addr)
        }
    }

    /// IPs that the cluster server needs to establish connections with
    fn to_connect(&self) -> HashSet<String> {
        let connected = connected_ips(&self.established_by_addr);
        self.state.members.get_cluster_hosts().difference(&connected)
            .filter(|&ip| *ip != self.addr).cloned().collect()
    }

    /// IPs that the cluster server needs to disconnect from
    fn to_disconnect(&self) -> HashSet<String> {
        let connected = connected_ips(&self.established_by_addr);
        connected.difference(&self.state.members.get_cluster_hosts()).cloned().collect()
    }

    /// IPs that are connected to the cluster server that the membership system doesn't know
    /// about yet.
    fn to_membership_connect(&self) -> HashSet<String> {
        let connected = connected_ips(&self.established_by_addr);
        connected.difference(&self.state.members.get_connected_cluster_hosts()).cloned().collect()
    }

    fn join_err(&self, token: Token, err: String, reply_tx: DebugSender<AdminRpy>) {
        let reply = AdminRpy::JoinReply {token: token, reply: Err(err)};
        reply_tx.send(reply);
    }

    fn join_success(&self, token: Token, reply_tx: DebugSender<AdminRpy>) {
        let reply = AdminRpy::JoinReply {token: token, reply: Ok(())};
        reply_tx.send(reply);
    }

    fn start_join(&mut self, join_token: Token, ipstr: String, reply_tx: DebugSender<AdminRpy> ) {
        match ipstr.parse() {
            Err(_) => {
                let msg = format!("Failed to parse ip: {}", ipstr);
                self.join_err(join_token,
                              Error::new(ErrorKind::InvalidInput, msg).to_string(),
                              reply_tx);
            }
            Ok(addr) => {
                let token = self.state.next_token();
                self.pending_joins.insert(token, (join_token, reply_tx));
                self.event_loop_tx.as_ref().unwrap().send(IncomingMsg::Connect(token, addr)).unwrap();
            }
        }
    }

    fn join_members(&mut self, orset: ORSet<Member>, token: Token) {
        self.state.members.join(orset);
        let res = self.pending_joins.remove(&token);
        if let Some((join_token, reply_tx)) = res {
            self.join_success(join_token, reply_tx);
        }
    }

    fn init_connect(&mut self, addr: SocketAddr) {
        let token = self.state.next_token();
        self.event_loop_tx.as_ref().unwrap().send(IncomingMsg::Connect(token, addr)).unwrap();
    }

    fn send_members(&mut self, token: Token) {
        let orset = self.state.members.get_orset();
        let encoded = Encoder::to_msgpack(&ClusterMsg::Members(self.addr.clone(), orset)).unwrap();
        let msg = IncomingMsg::WireMsg(token, encoded);
        self.event_loop_tx.as_ref().unwrap().send(msg).unwrap();
    }
}

fn connected_ips(established: &HashMap<String, (bool, Token)>) -> HashSet<String> {
    established.iter().map(|(ref addr, _)| addr.to_string()).collect()
}
