use std::io::{Error, Result};
use resp::Writer;
use config::Config;
use event_loop::Notification;
use mio;
use mio::Token;
use mio::tcp::TcpStream;
use state::State;
use super::messages::{Msg, Req, Res};
use tcphandler;
use tcphandler::{TcpHandler};
use event::Event;
use cluster::ClusterEvent;
use super::AdminEvent;
use std::net::SocketAddr;
use std::error;

/// Handle all events from the event loop
/// Maintain the internal state of the admin server
pub struct AdminHandler {
    node: String,
    state: State,
    sender: mio::Sender<Notification>,
}

impl TcpHandler for AdminHandler {
    type Event = AdminEvent;
    type TcpMsg = Msg;

    fn new(state: State, tx: mio::Sender<Notification>) -> AdminHandler {
        let state2 = state.clone();
        let members = state2.members.read().unwrap();
        let myname = (*members).orset.name.clone();
        let handler =
            AdminHandler {
                node: myname,
                state: state,
                sender: tx
            };
        handler.init_cluster_state();
        handler
    }

    fn handle_event(&mut self, event: AdminEvent) {
        match event {
            AdminEvent::JoinReply {token, reply} => {
                let msg = match reply {
                    Ok(()) => Writer::encode(Msg::Res(Res::Ok)),
                    Err(e) => {
                        let string = format!("{}", e);
                        Writer::encode(Msg::Res(Res::Err(string)))
                    }
                };
                self.sender.send(Notification::WireMsg(token, msg)).unwrap();
            }
        }
    }

    fn handle_tcp_msg(&mut self, token: Token, msg: Msg) {
        match msg {
            Msg::Req(Req::ConfigSet(key, val)) => self.set_config(token, key, val),
            Msg::Req(Req::ConfigGet(key)) => self.get_config(token, key),
            Msg::Req(Req::ClusterJoin(ipstr)) => self.join(token, ipstr),
            _ => ()
        }
    }
}

impl AdminHandler {
    /// We set state.admin_tx after constructing the AdminServer. We need to update the state in
    /// the cluster server so send it a message.
    fn init_cluster_state(&self) {
        let tx = self.state.cluster_tx.as_ref().unwrap().clone();
        tx.send(Event::ApiEvent(ClusterEvent::State(self.state.clone()))).unwrap();
    }

    /// Send a join to the cluster server. An AdminEvent::JoinReply will come asynchronously in
    /// response and be handled in handler.handle_event(). Note that we don't use a nonce or make
    /// any attempt to order asynchronous responses. Therefore, admin client commands cannot be
    /// pipelined. Currently, `cluster join` is the only 'off thread' asynchronous operation.
    fn join(&self, token: Token, ipstr: String) {
        let msg = Event::ApiEvent(ClusterEvent::Join {token: token, ipstr: ipstr});
        self.state.cluster_tx.as_ref().unwrap().send(msg).unwrap();
    }

    fn set_config(&self, token: Token, key: String, val: String) {
        let mut config = self.state.config.write().unwrap();
        let msg = match config.set(key, val) {
            Ok(()) => Writer::encode(Msg::Res(Res::Ok)),
            Err(e) => Writer::encode(Msg::Res(Res::Err(error::Error::description(&e).to_string())))
        };
        self.sender.send(Notification::WireMsg(token, msg)).unwrap();
    }

    fn get_config(&self, token: Token, key: String) {
        let config = self.state.config.read().unwrap();
        let msg = match config.get(key) {
            Ok(string) => Writer::encode(Msg::Res(Res::Simple(string))),
            Err(e) => Writer::encode(Msg::Res(Res::Err(error::Error::description(&e).to_string())))
        };
        self.sender.send(Notification::WireMsg(token, msg)).unwrap();
    }
}
