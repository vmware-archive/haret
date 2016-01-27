use msgpack::Encoder;
use v2r2::vr::{Dispatcher, Replica, VrMsg};
use debugger_shared::{TestMsg, Frame, Action};

pub struct Recorder {
    pub dispatcher: Dispatcher,
    recording: bool,
    history: Vec<Frame>,
}

impl Recorder {
    pub fn new(dispatcher: Dispatcher) -> Recorder {
        Recorder {
            recording: false,
            history: Vec::new(),
            dispatcher: dispatcher
        }
    }

    pub fn serialize_history(&self) -> Vec<u8> {
        Encoder::to_msgpack(&self.history).unwrap()
    }

    pub fn record(&mut self) {
        self.recording = true;
    }


    pub fn send(&mut self, test_msg: TestMsg, replica: &Replica, msg: VrMsg) {
        if self.recording {
            let action = Action::Send(replica.clone(), msg.clone());
            let mut frame = Frame::new(test_msg);
            frame.push(action);
            self.history.push(frame);
        }
        self.dispatcher.send(replica, msg);
        self.dispatcher.dispatch_all_received_messages();
    }

    pub fn stop_replica(&mut self, test_msg: TestMsg, replica: &Replica) {
        if self.recording {
            let mut frame = Frame::new(test_msg);
            let action = Action::Stop(replica.clone());
            frame.push(action);
            self.history.push(frame);
        }
        self.dispatcher.stop(replica);
    }

    pub fn restart_replica(&mut self, test_msg: TestMsg, replica: &Replica) {
        if self.recording {
            let mut frame = Frame::new(test_msg);
            let action = Action::Restart(replica.clone());
            frame.push(action);
            self.history.push(frame);
        }
        self.dispatcher.restart(replica.clone());
        self.dispatcher.dispatch_all_received_messages();
    }

}
