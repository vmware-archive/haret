use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::Receiver;
use event_loop::EventLoop;
use state::State;
use admin::AdminReq;
use super::{ClusterHandler};
use tcphandler::TcpHandler;

pub fn run(state: State, admin_rx: Receiver<AdminReq>)
    -> Vec<JoinHandle<()>> {

    let mut handler = ClusterHandler::new(state.clone(), admin_rx);
    let mut event_loop = EventLoop::<ClusterHandler>::new(state.clone(), handler.event_loop_sender());
    handler.set_event_loop_tx(event_loop.sender());
    let handle1 = event_loop.run();
    let handle2 = thread::Builder::new().name("cluster_server".to_string()).spawn(move || {
        handler.recv_loop()
    }).unwrap();
    vec![handle1, handle2]
}
