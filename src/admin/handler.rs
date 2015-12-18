use resp::Writer;
use event_loop::Notification;
use mio;
use mio::Token;
use state::State;
use tcphandler::TcpHandler;
use super::messages::{Msg, Req, Res};
use event::Event;
use super::AdminMsg;
use std::error;
use std::sync::mpsc::{channel, Sender, Receiver};

/// Handle all events from the event loop
/// Maintain the internal state of the admin server
pub struct AdminHandler {
    state: State,
    cluster_tx: Sender<AdminMsg>,
    cluster_rx: Option<Receiver<AdminMsg>>,
    event_loop_tx: Option<mio::Sender<Notification>>,
    event_loop_rx: Option<Receiver<Event<Msg>>>,
    // To be given away to the event loop so it can send us messages
    event_loop_sender: Sender<Event<Msg>>
}

impl TcpHandler for AdminHandler {
    type TcpMsg = Msg;

    fn set_event_loop_tx(&mut self, tx: mio::Sender<Notification>) {
        self.event_loop_tx = Some(tx);
    }

    fn event_loop_sender(&self) -> Sender<Event<Msg>> {
        self.event_loop_sender.clone()
    }

    fn host(state: &State) -> String {
        let config = state.config.read().unwrap();
        config.admin_host.clone()
    }

    // Main handler loop
    fn recv_loop(&mut self) {
        let cluster_rx = self.cluster_rx.take().unwrap();
        let event_loop_rx = self.event_loop_rx.take().unwrap();
        loop {
            select! {
                msg = cluster_rx.recv() => self.handle_admin_msg(msg.unwrap()),
                event = event_loop_rx.recv() => self.handle_event(event.unwrap())
            }
        }
    }
}

impl AdminHandler {
    pub fn new(state: State, cluster_tx: Sender<AdminMsg>, cluster_rx: Receiver<AdminMsg>)
        -> AdminHandler {

        let (tx, rx) = channel();
        AdminHandler {
            state: state,
            cluster_tx: cluster_tx,
            cluster_rx: Some(cluster_rx),
            event_loop_tx: None,
            event_loop_rx: Some(rx),
            event_loop_sender: tx
        }
    }

    fn handle_event(&mut self, event: Event<Msg>) {
        match event {
            Event::TcpMsg(token, msg) => self.handle_tcp_msg(token, msg),
            _ => ()
        }
    }

    fn handle_admin_msg(&mut self, msg: AdminMsg) {
        match msg {
            AdminMsg::JoinReply {token, reply} => {
                let msg = match reply {
                    Ok(()) => Writer::encode(Msg::Res(Res::Ok)),
                    Err(e) => {
                        let string = format!("{}", e);
                        Writer::encode(Msg::Res(Res::Err(string)))
                    }
                };
                let msg = Notification::WireMsg(token, msg);
                self.event_loop_tx.as_ref().unwrap().send(msg).unwrap();
            },
            _ => ()
        }
    }

    fn handle_tcp_msg(&mut self, token: Token, msg: Msg) {
        match msg {
            Msg::Req(Req::ConfigSet(key, val)) => self.set_config(token, key, val),
            Msg::Req(Req::ConfigGet(key)) => self.get_config(token, key),
            Msg::Req(Req::ClusterJoin(ipstr)) => self.join(token, ipstr),
            Msg::Req(Req::ClusterMembers) => self.cluster_members(token),
            Msg::Req(Req::ClusterStatus) => self.cluster_status(token),
            _ => ()
        }
    }
    /// Send a join to the cluster server. An AdminEvent::JoinReply will come asynchronously in
    /// response and be handled in handler.handle_event(). Note that we don't use a nonce or make
    /// any attempt to order asynchronous responses. Therefore, admin client commands cannot be
    /// pipelined. Currently, `cluster join` is the only 'off thread' asynchronous operation.
    fn join(&self, token: Token, ipstr: String) {
        let msg = AdminMsg::Join {token: token, ipstr: ipstr};
        self.cluster_tx.send(msg).unwrap();
    }

    fn cluster_members(&self, token: Token) {
        let msg = format!("{}", self.state.members);
        let msg = Writer::encode(Msg::Res(Res::Simple(msg)));
        self.event_loop_tx.as_ref().unwrap().send(Notification::WireMsg(token, msg)).unwrap();
    }

    fn cluster_status(&self, token: Token) {
        let msg = Writer::encode(Msg::Res(Res::Simple(self.state.members.status())));
        self.event_loop_tx.as_ref().unwrap().send(Notification::WireMsg(token, msg)).unwrap();
    }

    fn set_config(&self, token: Token, key: String, val: String) {
        let mut config = self.state.config.write().unwrap();
        let msg = match config.set(key, val) {
            Ok(()) => Writer::encode(Msg::Res(Res::Ok)),
            Err(e) => Writer::encode(Msg::Res(Res::Err(error::Error::description(&e).to_string())))
        };
        self.event_loop_tx.as_ref().unwrap().send(Notification::WireMsg(token, msg)).unwrap();
    }

    fn get_config(&self, token: Token, key: String) {
        let config = self.state.config.read().unwrap();
        let msg = match config.get(key) {
            Ok(string) => Writer::encode(Msg::Res(Res::Simple(string))),
            Err(e) => Writer::encode(Msg::Res(Res::Err(error::Error::description(&e).to_string())))
        };
        self.event_loop_tx.as_ref().unwrap().send(Notification::WireMsg(token, msg)).unwrap();
    }
}
