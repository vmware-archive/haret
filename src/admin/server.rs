use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{channel, sync_channel, Sender, Receiver};
use std::collections::VecDeque;
use std::error::Error;
use msgpack::{Encoder, from_msgpack};
use mio::{self, Token};
use time::{SteadyTime, Duration};
use uuid::Uuid;
use state::State;
use super::{AdminClientReq, AdminClientRpy, AdminReq, AdminRpy};
use vr::event_loop::{EventLoop, OutControlMsg, OutDataMsg, IncomingMsg};
use vr::{DispatchMsg, RawReplica, Replica};
use debug_sender::DebugSender;

const EVENT_LOOP_QUEUE_SIZE: usize = 1000;
const TIMEOUT: i64 = 5; // seconds
const TICK_TIME: u64 = 1000; // milliseconds

pub struct AdminServer {
   state: State,
   client_requests: VecDeque<(Token, SteadyTime)>,
   event_loop_tx: Option<mio::Sender<IncomingMsg>>,
   dispatcher_tx: Sender<DispatchMsg>,
   cluster_tx: Sender<AdminReq>,
   // To be cloned and used for responses to the admin server
   reply_tx: DebugSender<AdminRpy>,
   // Receives replies to outgoing requests from both the dispatcher and cluster server
   reply_rx: Option<Receiver<AdminRpy>>
}

impl AdminServer {
    pub fn new(state: State,
               dispatcher_tx: Sender<DispatchMsg>,
               cluster_tx: Sender<AdminReq>) -> AdminServer {
        let (reply_tx, reply_rx) = channel();
        AdminServer {
            state: state,
            client_requests: VecDeque::new(),
            event_loop_tx: None,
            dispatcher_tx: dispatcher_tx,
            cluster_tx: cluster_tx,
            reply_tx: DebugSender::new(reply_tx),
            reply_rx: Some(reply_rx),
        }
    }

    pub fn run(mut self) -> Vec<JoinHandle<()>> {
        let host = {
            let config = self.state.config.read().unwrap();
            config.admin_host.parse().unwrap()
        };
        let mut event_loop = EventLoop::new(host, 5000);
        let (data_tx, data_rx) = sync_channel(EVENT_LOOP_QUEUE_SIZE/2);
        let (control_tx, control_rx) = channel();
        let state = self.state.clone();
        self.event_loop_tx = Some(event_loop.sender());
        let mut handles = Vec::new();
        handles.push(thread::Builder::new().name("admin_server_event_loop".to_string()).spawn(move || {
            event_loop.run(state, data_tx, control_tx);
        }).unwrap());
        self.register_tick();
        let reply_rx = self.reply_rx.take().unwrap();
        handles.push(thread::Builder::new().name("admin_server".to_string()).spawn(move || {
            loop {
                select! {
                    msg = data_rx.recv() => self.handle_data_msg(msg.unwrap()),
                    msg = control_rx.recv() => self.handle_control_msg(msg.unwrap()),
                    msg = reply_rx.recv() => self.handle_reply(msg.unwrap())
                }
            }
        }).unwrap());
        handles
    }

    fn handle_data_msg(&mut self, msg: OutDataMsg) {
        match msg {
            OutDataMsg::TcpMsg(token, data) => {
                match from_msgpack(&data) {
                    Ok(req) => self.handle_client_request(token, req),
                    Err(_) => {
                        let reason =
                            "Received improperly encoded msgpack data at admin server".to_string();
                        self.deregister_client(token, reason);
                    }
                }
            },
            OutDataMsg::Tick(_) => {
                self.maybe_timeout_client_requests();
            }
        }
    }

    fn handle_control_msg(&mut self, msg: OutControlMsg) {
        match msg {
            OutControlMsg::NewSock(_token) => (),
            OutControlMsg::Deregister(token) => self.remove_client_request(&token)
        }
    }

    fn handle_reply(&mut self, msg: AdminRpy) {
        match msg {
            AdminRpy::JoinReply {token, reply} => {
                // We may have already timed out and returned a Timeout
                if self.request_exists(&token) {
                    match reply {
                        Ok(()) => self.send_client_reply(token, &AdminClientRpy::Ok),
                        Err(e) => self.send_client_reply(token, &AdminClientRpy::Error(e))
                    }
                }
            },
            AdminRpy::Tenants {token, tenants} => {
                // We may have already timed out and returned a Timeout
                if self.request_exists(&token) {
                    self.send_client_reply(token, &AdminClientRpy::VrTenants(tenants));
                }
            },
            AdminRpy::Replica {token, state, ctx} => {
                // We may have already timed out and returned a Timeout
                if self.request_exists(&token) {
                    let ctx = format!("{:#?}", ctx);
                    self.send_client_reply(token, &AdminClientRpy::VrReplica(state.to_string(), ctx));
                }
            },
            AdminRpy::ReplicaNotFound {token, replica} => {
                // We may have already timed out and returned a Timeout
                if self.request_exists(&token) {
                    let msg = format!("No such replica {} on node {}", replica,
                                      self.state.members.me.name);
                    self.send_client_reply(token, &AdminClientRpy::Error(msg));
                }
            },
            AdminRpy::TenantId {token, id} => {
                if self.request_exists(&token) {
                    self.send_client_reply(token, &AdminClientRpy::VrTenantId(id));
                }
            },
            AdminRpy::Error(token, err) => {
                if self.request_exists(&token) {
                    self.send_client_reply(token, &AdminClientRpy::Error(err));
                }
            },
            AdminRpy::VrStats {token, stats} => {
                if self.request_exists(&token) {
                    self.send_client_reply(token, &AdminClientRpy::VrStats(stats));
                }
            },
            AdminRpy::Ok(token) => {
                if self.request_exists(&token) {
                    self.send_client_reply(token, &AdminClientRpy::Ok);
                }
            }
        }
    }

    // Note that there can be only one outstanding client request per each client, although we don't
    // enforce this with a check.
    fn handle_client_request(&mut self, token: Token, req: AdminClientReq) {
        match req {
            AdminClientReq::ConfigSet(key, val) => self.set_config(token, key, val),
            AdminClientReq::ConfigGet(key) => self.get_config(token, key),
            AdminClientReq::ClusterJoin(ipstr) => self.join(token, ipstr),
            AdminClientReq::ClusterMembers => self.cluster_members(token),
            AdminClientReq::ClusterStatus => self.cluster_status(token),
            AdminClientReq::VrCreateTenant(raw_replicas) => self.create_tenant(token, raw_replicas),
            AdminClientReq::VrTenants => self.get_tenants(token),
            AdminClientReq::VrReplica(replica) => self.get_replica_state(token, replica),
            AdminClientReq::VrStats => self.get_vr_stats(token)
        }
    }

    fn join(&mut self, token: Token, ipstr: String) {
        self.client_requests.push_back((token, SteadyTime::now()));
        let msg = AdminReq::Join {token: token, ipstr: ipstr, reply_tx: self.reply_tx.clone()};
        self.cluster_tx.send(msg).unwrap();
    }

    fn cluster_members(&mut self, token: Token) {
        let members = self.state.members.to_string();
        self.send_client_reply(token, &AdminClientRpy::Members(members));
    }

    fn cluster_status(&mut self, token: Token) {
        let status = self.state.members.status();
        self.send_client_reply(token, &AdminClientRpy::MemberStatus(status));
    }

    fn create_tenant(&mut self, token: Token, raw_replicas: String) {
        match parse_raw_replicas(raw_replicas) {
            Ok(parsed_raw_replicas) => {
                let tenant = Uuid::new_v4();
                let req = AdminReq::CreateTenant {token: token,
                                                  tenant: tenant,
                                                  replicas: parsed_raw_replicas,
                                                  reply_tx: self.reply_tx.clone()};
                self.send_async_dispatcher_msg(token, req);
            },
            Err(e) => self.send_client_reply(token, &AdminClientRpy::Error(e))
        };
    }

    fn get_tenants(&mut self, token: Token) {
        let msg = AdminReq::GetTenants {token: token.clone(),
                                        reply_tx: self.reply_tx.clone()};
        self.send_async_dispatcher_msg(token, msg);
    }

    fn get_replica_state(&mut self, token: Token, replica: Replica) {
        let msg = AdminReq::GetReplica {token: token.clone(),
                                        replica: replica,
                                        reply_tx: self.reply_tx.clone()};
        self.send_async_dispatcher_msg(token, msg);
    }

    fn get_vr_stats(&mut self, token: Token) {
        let msg = AdminReq::GetVrStats {token: token.clone(),
                                        reply_tx: self.reply_tx.clone()};
        self.send_async_dispatcher_msg(token, msg);
    }

    fn send_async_dispatcher_msg(&mut self, token: Token, req: AdminReq) {
        let msg = DispatchMsg::Admin(req);
        self.dispatcher_tx.send(msg).unwrap();
        self.client_requests.push_back((token, SteadyTime::now()));
    }

    fn set_config(&mut self, token: Token, key: String, val: String) {
        let reply = {
            let mut config = self.state.config.write().unwrap();
            match config.set(key, val) {
                Ok(()) => AdminClientRpy::Ok,
                Err(e) => {
                    let err = Error::description(&e).to_string();
                    AdminClientRpy::Error(err)
                }
            }
        };
        self.send_client_reply(token, &reply);
    }

    fn get_config(&mut self, token: Token, key: String) {
        let reply = {
            let config = self.state.config.read().unwrap();
            match config.get(key.clone()) {
                Ok(val) => AdminClientRpy::Config(key, val),
                Err(e) => {
                    let err = Error::description(&e).to_string();
                    AdminClientRpy::Error(err)
                }
            }
        };
        self.send_client_reply(token, &reply);
    }

    fn deregister_client(&mut self, token: Token, reason: String) {
        let dereg_msg = IncomingMsg::Deregister(token.clone(), reason);
        self.event_loop_tx.as_ref().unwrap().send(dereg_msg).unwrap();
    }

    fn register_tick(&mut self) {
        let msg = IncomingMsg::SetTimeout(Uuid::new_v4(), TICK_TIME);
        self.event_loop_tx.as_ref().unwrap().send(msg).unwrap();
    }

    fn remove_client_request(&mut self, token: &Token) {
        self.client_requests.retain(|&(ref saved_token, _)| *saved_token != *token);
    }

    fn request_exists(&self, token: &Token) -> bool {
        self.client_requests.iter().any(|&(ref saved_token, _)| *saved_token == *token)
    }

    fn maybe_timeout_client_requests(&mut self) {
        // Find the first request that did not timeout
        let pos = self.client_requests.iter().position(|&(_, ref time)| {
            SteadyTime::now() - *time < Duration::seconds(TIMEOUT)
        });
        match pos {
            Some(0) => (), //No requests timed out
            Some(n) => {
                // requests up to index n have timed out
                for _ in 0..n {
                    let (token, _) = self.client_requests.pop_front().unwrap();
                    self.send_client_timeout(token);
                }
            },
            None => {
                // All requests have timed out
                for _ in 0..self.client_requests.len() {
                    let (token, _) = self.client_requests.pop_front().unwrap();
                    self.send_client_timeout(token);
                }
            }
        }
    }

    fn send_client_reply(&mut self, token: Token, reply: &AdminClientRpy) {
        let _ = self.remove_client_request(&token);
        let encoded = Encoder::to_msgpack(reply).unwrap();
        let msg = IncomingMsg::WireMsg(token, encoded);
        self.event_loop_tx.as_ref().unwrap().send(msg).unwrap();
    }

    fn send_client_timeout(&mut self, token: Token) {
        let encoded = Encoder::to_msgpack(&AdminClientRpy::Timeout).unwrap();
        let msg = IncomingMsg::WireMsg(token, encoded);
        self.event_loop_tx.as_ref().unwrap().send(msg).unwrap();
    }
}

fn parse_raw_replicas(raw_str: String) -> Result<Vec<RawReplica>, String> {
    let mut replicas = Vec::new();
    let split: Vec<&str> = raw_str.split(",").collect();
    if split.len() < 3 {
        return Err("There must be at least 3 replicas to create a tenant".to_string())
    }
    for s in split {
        let raw_replica = try!(s.parse());
        replicas.push(raw_replica);
    }
    Ok(replicas)
}
