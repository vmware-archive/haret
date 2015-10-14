use resp::Writer;
use event_loop::Notification;
use mio;
use mio::Token;
use state::State;
use tcphandler::TcpHandler;
use super::messages::{VrApiReq, VrApiRsp};
use event::Event;
use std::error;
use std::sync::mpsc::{channel, Sender, Receiver};
use rustc_serialize::Encodable;
use super::mock_vr::{VrReq, VrResp};

pub struct VrApiHandler {
    node: String,
    state: State,
    vr_tx: Sender<VrReq>,
    vr_rx: Option<Receiver<VrResp>>,
    event_loop_tx: Option<mio::Sender<Notification>>,
    event_loop_rx: Option<Receiver<Event<VrApiReq>>>,
    // To be given away to the event loop so it can send us messages
    event_loop_sender: Sender<Event<VrApiReq>>
}

impl TcpHandler for VrApiHandler {
    type TcpMsg = VrApiReq;

    fn set_event_loop_tx(&mut self, tx: mio::Sender<Notification>) {
        self.event_loop_tx = Some(tx);
    }

    fn event_loop_sender(&self) -> Sender<Event<VrApiReq>> {
        self.event_loop_sender.clone()
    }

    fn host(state: &State) -> String {
        let config = state.config.read().unwrap();
        config.vr_api_host.clone()
    }

    // Main handler loop
    fn recv_loop(&mut self) {
        let event_loop_rx = self.event_loop_rx.take().unwrap();
        let vr_rx = self.vr_rx.take().unwrap();
        loop {
            select! {
                msg = vr_rx.recv() => self.handle_vr_resp(msg.unwrap()),
                event = event_loop_rx.recv() => self.handle_event(event.unwrap())
            }
        }
    }
}

impl VrApiHandler {
    pub fn new(state: State, vr_tx: Sender<VrReq>, vr_rx: Receiver<VrResp> ) -> VrApiHandler {
        let (tx, rx) = channel();
        VrApiHandler {
            node: state.members.local_name(),
            state: state,
            vr_tx: vr_tx,
            vr_rx: Some(vr_rx),
            event_loop_tx: None,
            event_loop_rx: Some(rx),
            event_loop_sender: tx
        }
    }

    fn handle_event(&mut self, event: Event<VrApiReq>) {
        match event {
            Event::TcpMsg(token, msg) => self.handle_tcp_msg(token, msg),
            _ => ()
        }
    }

    fn handle_tcp_msg(&mut self, token: Token, msg: VrApiReq) {
        match msg {
            VrApiReq::Create {..} => self.vr_write(token, msg),
            VrApiReq::Put {..} => self.vr_write(token, msg),
            VrApiReq::Delete {..} => self.vr_write(token, msg),
            VrApiReq::Get {..} => self.vr_read(token, msg),
            VrApiReq::List {..} => self.vr_read(token, msg),
            VrApiReq::Null => ()
        };
    }

    fn handle_vr_resp(&self, resp: VrResp) {
        let VrResp(token, msg) = resp;
        let msg = Writer::encode(msg);
        self.event_loop_tx.as_ref().unwrap().send(Notification::WireMsg(token, msg)).unwrap();
    }

    fn vr_write(&self, token: Token, msg: VrApiReq) {
        self.vr_tx.send(VrReq::Write(token, msg)).unwrap();
    }

    fn vr_read(&self, token: Token, msg: VrApiReq) {
        self.vr_tx.send(VrReq::Read(token, msg)).unwrap();
    }
}

