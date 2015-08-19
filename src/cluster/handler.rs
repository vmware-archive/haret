use resp::Writer;
use std::io::{Error, Result, ErrorKind};
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use mio;
use mio::Token;
use event_loop::Notification;
use state::State;
use event::Event;
use tcphandler::TcpHandler;
use super::ClusterEvent;
use super::messages::Msg;
use admin::AdminEvent;
use orset::ORSet;
use membership::Member;
use time::{SteadyTime, Duration};

pub struct ClusterHandler {
    node: String,
    addr: String,
    state: State,
    pending_joins: HashMap<Token, Token>,

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
    sender: mio::Sender<Notification>
}

impl TcpHandler for ClusterHandler {
    type Event = ClusterEvent;
    type TcpMsg = Msg;

    fn new(state: State, tx: mio::Sender<Notification>) -> ClusterHandler {
        let addr = {
            let config = state.config.read().unwrap();
            config.cluster_host.parse().unwrap()
        };
        ClusterHandler {
            node: state.members.local_name(),
            addr: addr,
            state: state,
            pending_joins: HashMap::new(),
            receipts: HashMap::new(),
            unestablished_clients: HashSet::new(),
            established_by_addr: HashMap::new(),
            established_by_token: HashMap::new(),
            sender: tx
        }
    }

    fn connect(&mut self, token: Token, addr: SocketAddr) {
        self.send_members(token);
        self.receipts.insert(token, SteadyTime::now());
    }

    fn deregister(&mut self, token: Token, _: SocketAddr) {
        if let Some((_, addr)) = self.established_by_token.remove(&token) {
            self.established_by_addr.remove(&addr);
            self.state.members.disconnected(&addr);
        } else {
            self.unestablished_clients.remove(&token);
        }
        if let Some(&join_token) = self.pending_joins.get(&token) {
            let err = Error::new(ErrorKind::NotConnected, "Could not connect to server");
            self.join_err(join_token, err);
            self.pending_joins.remove(&token);
        }
        self.receipts.remove(&token);
    }

    fn handle_event(&mut self, event: ClusterEvent) {
        match event {
            ClusterEvent::Join {token, ipstr} => self.start_join(token, ipstr),
            ClusterEvent::State(state) => self.state = state
        }
    }

    fn handle_tcp_msg(&mut self, token: Token, msg: Msg) {
        match msg {
            Msg::Members(addr, orset) => {
                println!("Got Msg::Members for {:?} from {:?} ", self.node, orset.name);
                self.maybe_establish_connection(token, addr);
                self.join_members(orset, token);
                self.check_connections();
            },
            Ping => {
                self.receipts.insert(token, SteadyTime::now());
            }
        }
    }

    fn tick(&mut self) {
        self.check_liveness();
        self.send_pings();
        self.check_connections();
    }
}

fn connected_ips(established: &HashMap<String, (bool, Token)>) -> HashSet<String> {
    established.iter().map(|(ref addr, _)| addr.to_string()).collect()
}

impl ClusterHandler {

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
            self.receipts.iter().filter(|&(&token, &time)| now - time > Duration::seconds(5));
        for (&token, _) in to_remove {
            let msg = format!("Cluster connection for {:?} timed out", token);
            let err = Error::new(ErrorKind::TimedOut, msg);
            self.sender.send(Notification::Deregister(token, err)).unwrap();
        }
    }

    fn send_pings(&self) {
        let msg = Writer::encode(Msg::Ping);
        for (&token, _) in self.receipts.iter() {
            self.sender.send(Notification::WireMsg(token, msg.clone())).unwrap();
        }
    }


    fn maybe_establish_connection(&mut self, token: Token, addr: String) {
        let maybe_token = self.pick_token_to_deregister(token, &addr);
        match maybe_token {
            Some(deregister_token) => {
                let msg = format!("Peer {:?} already connected", addr);
                let err = Error::new(ErrorKind::Other, msg);
                if deregister_token == token {
                    // Deregister the incoming connection
                    self.sender.send(Notification::Deregister(token, err)).unwrap();
                } else {
                    // Deregister the already established connection
                    // Cleanup connection here and not in deregister callback, because we don't want
                    // to call self.state.members.disconnected();
                    self.established_by_token.remove(&deregister_token);
                    self.established_by_addr.remove(&addr);
                    self.sender.send(Notification::Deregister(deregister_token, err)).unwrap();
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
                let msg = format!("{:?} no longer part of cluster membership", addr);
                let err = Error::new(ErrorKind::Other, msg);
                self.sender.send(Notification::Deregister(*token, err)).unwrap();
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
        self.state.members.get_ips().difference(&connected)
            .filter(|&ip| *ip != self.addr).cloned().collect()
    }

    /// IPs that the cluster server needs to disconnect from
    fn to_disconnect(&self) -> HashSet<String> {
        let connected = connected_ips(&self.established_by_addr);
        connected.difference(&self.state.members.get_ips()).cloned().collect()
    }

    /// IPs that are connected to the cluster server that the membership system doesn't know
    /// about yet.
    fn to_membership_connect(&self) -> HashSet<String> {
        let connected = connected_ips(&self.established_by_addr);
        connected.difference(&self.state.members.get_connected_ips()).cloned().collect()
    }

    fn join_err(&self, token: Token, err: Error) {
        let reply =
            Event::ApiEvent(AdminEvent::JoinReply {token: token, reply: Err(err)});
        self.state.admin_tx.as_ref().unwrap().send(reply).unwrap();
    }

    fn join_success(&self, token: Token) {
        let reply =
            Event::ApiEvent(AdminEvent::JoinReply {token: token, reply: Ok(())});
        self.state.admin_tx.as_ref().unwrap().send(reply).unwrap();
    }

    fn start_join(&mut self, join_token: Token, ipstr: String) {
        match ipstr.parse() {
            Err(_) => {
                let msg = format!("Failed to parse ip: {}", ipstr);
                self.join_err(join_token, Error::new(ErrorKind::InvalidInput, msg));
            }
            Ok(addr) => {
                let token = self.state.next_token();
                self.pending_joins.insert(token, join_token);
                self.sender.send(Notification::Connect(token, addr)).unwrap();
            }
        }
    }

    fn join_members(&mut self, orset: ORSet<Member>, token: Token) {
        self.state.members.join(orset);
        if let Some(&join_token) = self.pending_joins.get(&token) {
            self.join_success(join_token);
            self.pending_joins.remove(&token);
        }
    }

    fn init_connect(&mut self, addr: SocketAddr) {
        let token = self.state.next_token();
        self.sender.send(Notification::Connect(token, addr)).unwrap();
    }

    fn send_members(&mut self, token: Token) {
        println!("Sending members for {}", self.node);
        let orset = self.state.members.get_orset();
        let msg = Writer::encode(Msg::Members(self.addr.clone(), orset));
        self.sender.send(Notification::WireMsg(token, msg)).unwrap();
    }
}
