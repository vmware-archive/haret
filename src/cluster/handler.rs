use resp::Writer;
use std::io::{Error, Result, ErrorKind};
use std::collections::{HashMap};
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

pub struct ClusterHandler {
    node: String,
    state: State,
    pending_joins: HashMap<Token, Token>,
    // We only want one connection to each peer.
    ip_map: HashMap<SocketAddr, Token>,
    sender: mio::Sender<Notification>
}

impl TcpHandler for ClusterHandler {
    type Event = ClusterEvent;
    type TcpMsg = Msg;

    fn new(state: State, tx: mio::Sender<Notification>) -> ClusterHandler {
        let state2 = state.clone();
        let members = state2.members.read().unwrap();
        let myname = (*members).orset.name.clone();
        ClusterHandler {
            node: myname,
            state: state,
            pending_joins: HashMap::new(),
            ip_map: HashMap::new(),
            sender: tx
        }
    }

    fn connect(&mut self, token: Token, addr: SocketAddr) {
        if let Some(_) = self.ip_map.get(&addr) {
            // We already have a connection to this peer
            let msg = format!("Cluster connnection for {:?} on {} already exists",
                              addr,
                              self.node);
            let err = Error::new(ErrorKind::AlreadyExists, msg);
            self.sender.send(Notification::Deregister(token, err)).unwrap();
        } else {
            self.ip_map.insert(addr, token);
            self.send_members(token);
        }
    }

    fn deregister(&mut self, token: Token, addr: SocketAddr) {
        if let Some(&addr_token) = self.ip_map.get(&addr) {
            // Make sure we didn't just force the deregister in connect()
            // due to a duplicate peer connection
            if addr_token == token {
                self.ip_map.remove(&addr);
            }
        }
        if let Some(&join_token) = self.pending_joins.get(&token) {
            let err = Error::new(ErrorKind::NotConnected, "Could not connect to server");
            self.join_err(join_token, err);
            self.pending_joins.remove(&token);
        }
    }

    fn handle_event(&mut self, event: ClusterEvent) {
        match event {
            ClusterEvent::Join {token, ipstr} => self.start_join(token, ipstr),
            ClusterEvent::State(state) => self.state = state
        }
    }

    fn handle_tcp_msg(&mut self, token: Token, msg: Msg) {
        match msg {
            Msg::Members(orset) => {
                let mut members = self.state.members.write().unwrap();
                {
                    let myname = &mut (*members).orset.name;
                    println!("Got Msg::Members for {:?} from {:?} ", myname, orset.name);
                }
                (*members).orset.join_state(orset);
                if let Some(&join_token) = self.pending_joins.get(&token) {
                    println!("Ahah pending join!!!");
                    self.join_success(join_token);
                    self.pending_joins.remove(&token);
                }
                // Check to see if there are any members that we don't have connections to
                // Also should do this in a 'tick' periodically in case of
                // disconnections/partitions/errors etc...
                // self.check_connecions();
            }
        }
    }
}

impl ClusterHandler {
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

    fn send_members(&mut self, token: Token) {
        println!("Sending members for {}", self.node);
        let members = self.state.members.read().unwrap();
        let orset = (*members).orset.clone();
        let msg = Writer::encode(Msg::Members(orset));
        self.sender.send(Notification::WireMsg(token, msg)).unwrap();
    }
}
