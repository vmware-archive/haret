use std::sync::mpsc::{Sender, Receiver};
use mio::Token;
use super::messages::{Req, Rsp};
use element::Version; // TODO: should this be in it's own file?
use super::backend::VrBackend;

pub enum VrReq {
    Write(Token, Req),
    Read(Token, Req)
}

pub struct VrResp(pub Token, pub Rsp);

pub struct MockVr {
    version: Version,
    backend: VrBackend,
    tx: Sender<VrResp>,
    rx: Receiver<VrReq>
}

impl MockVr {
    pub fn new(tx: Sender<VrResp>, rx: Receiver<VrReq>) -> MockVr {
        MockVr {
            backend: VrBackend::new(),
            version: Version::new(),
            tx: tx,
            rx: rx
        }
    }

    pub fn recv_loop(&mut self) {
        loop {
            match self.rx.recv().unwrap() {
                VrReq::Write(token, msg) => self.handle_write(token, msg),
                VrReq::Read(token, msg) => self.handle_read(token, msg)
            }
        }
    }

    fn handle_write(&mut self, token: Token, msg: Req) {
        let version = self.version.inc();
        let resp = self.backend.call(version, msg);
        self.tx.send(VrResp(token, resp)).unwrap();
    }

    fn handle_read(&mut self, token: Token, msg: Req) {
        let version = self.version.clone();
        let resp = self.backend.call(version, msg);
        self.tx.send(VrResp(token, resp)).unwrap();
    }
}
