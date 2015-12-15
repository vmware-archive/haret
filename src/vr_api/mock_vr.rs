use std::sync::mpsc::{Sender, Receiver};
use mio::Token;
use super::messages::{VrApiReq, VrApiRsp};
use super::backend::VrBackend;

pub enum VrReq {
    Write(Token, VrApiReq),
    Read(Token, VrApiReq)
}

pub struct VrResp(pub Token, pub VrApiRsp);

pub struct MockVr {
    op: u64,
    backend: VrBackend,
    tx: Sender<VrResp>,
    rx: Receiver<VrReq>
}

impl MockVr {
    pub fn new(tx: Sender<VrResp>, rx: Receiver<VrReq>) -> MockVr {
        MockVr {
            backend: VrBackend::new(),
            op: 0,
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

    fn handle_write(&mut self, token: Token, msg: VrApiReq) {
        self.op += 1;
        let resp = self.backend.call(self.op, msg);
        self.tx.send(VrResp(token, resp)).unwrap();
    }

    fn handle_read(&mut self, token: Token, msg: VrApiReq) {
        let resp = self.backend.call(self.op, msg);
        self.tx.send(VrResp(token, resp)).unwrap();
    }
}
