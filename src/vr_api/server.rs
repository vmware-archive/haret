use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::channel;
use event_loop::EventLoop;
use state::State;
use super::VrApiHandler;
use super::mock_vr::MockVr;
use tcphandler::TcpHandler;

pub fn run(state: State)
    -> Vec<JoinHandle<()>> {

    let (req_tx, req_rx) = channel();
    let (rpy_tx, rpy_rx) = channel();

    let mut handler = VrApiHandler::new(state.clone(), req_tx, rpy_rx);
    let mut mock_vr = MockVr::new(rpy_tx, req_rx);
    let mut event_loop = EventLoop::<VrApiHandler>::new(state.clone(), handler.event_loop_sender());
    handler.set_event_loop_tx(event_loop.sender());
    let handle1 = event_loop.run();
    let handle2 = thread::Builder::new().name("vr_api_server".to_string()).spawn(move || {
        handler.recv_loop()
    }).unwrap();
    let handle3 = thread::Builder::new().name("mock_vr".to_string()).spawn(move || {
        mock_vr.recv_loop()
    }).unwrap();

    vec![handle1, handle2, handle3]
}
