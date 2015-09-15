use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Sender, Receiver};
use event_loop::EventLoop;
use state::State;
use super::{AdminMsg, AdminHandler};
use tcphandler::TcpHandler;

pub fn run(state: State, cluster_tx: Sender<AdminMsg>, cluster_rx: Receiver<AdminMsg>)
    -> Vec<JoinHandle<()>> {

    let mut handler = AdminHandler::new(state.clone(), cluster_tx, cluster_rx);
    let mut event_loop = EventLoop::<AdminHandler>::new(state.clone(), handler.event_loop_sender());
    handler.set_event_loop_tx(event_loop.sender());
    let handle1 = event_loop.run();
    let handle2 = thread::Builder::new().name("admin_server".to_string()).spawn(move || {
        handler.recv_loop()
    }).unwrap();
    vec![handle1, handle2]
}
