//! This is a deterministic scheduler which records all test messages as well as all messages sent
//! to and from each FSM via the dispatcher. It will save the state of the world (FSMs + Dispatcher)
//! after each `TestMsg` execution for use via an interactive debugger. This will allow backwards
//! debugging via a snapshot + replay method. This is more efficient in terms of memory usage than
//! tracking every single state in each fsm after a VrMsg is received and allowing
//! stepping backwards by removing VrMsgs one at a time and reverting state. However, forward
//! stepping will be as fine grained as triggering individual sends of VRMsgs and inspecting state
//! of the FSM.

use std::collections::HashMap;
use rustc_serialize::Encodable;
use msgpack::{Encoder, from_msgpack};
use v2r2::vr::{Dispatcher, DispatcherState, Replica, VrMsg, VrCtx, Envelope};
use super::TestMsg;

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub enum Action {
    Send(Replica, VrMsg),
    Stop(Replica),
    Restart(Replica)
}

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct Frame {
    pub test_msg: TestMsg,
    pub actions: Vec<Action>
}

/// A granularity of a history that allows stepping between TestMsgs
impl Frame {
    fn new(test_msg: TestMsg) -> Frame {
        Frame {
            test_msg: test_msg,
            actions: Vec::new()
        }
    }

    fn push(&mut self, action: Action) {
        self.actions.push(action);
    }
}

pub struct Scheduler {
    recording: bool,
    history: Vec<Frame>,
    pub dispatcher: Dispatcher
}

impl Scheduler {
    pub fn new(dispatcher: Dispatcher) -> Scheduler {
        Scheduler {
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

    pub fn get_state(&self) -> DispatcherState {
        self.dispatcher.save_state()
    }

    pub fn set_state(&mut self, state: &DispatcherState) {
        self.dispatcher.restore_state(state);
    }

    pub fn run_action(&mut self, action: &Action) {
        match *action {
            Action::Send(ref replica, ref msg) => self.dispatcher.send(replica, msg.clone()),
            Action::Stop(ref replica) => self.dispatcher.stop(replica),
            Action::Restart(ref replica) => self.dispatcher.restart(replica.clone())
        }
    }

    pub fn dispatch_one_msg(&mut self) -> Option<Envelope> {
        match self.dispatcher.try_recv() {
            Ok(Envelope {to, from, msg}) => {
                self.dispatcher.send(&to, msg.clone());
                Some(Envelope {to: to, from: from, msg: msg})
            },
            _ => None
        }
    }

    pub fn dispatch(&mut self) {
        self.dispatcher.dispatch_all_received_messages();
    }
}
