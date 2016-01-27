//! This is a deterministic scheduler which records all test messages as well as all messages sent
//! to and from each FSM via the dispatcher. It will save the state of the world (FSMs + Dispatcher)
//! after each `TestMsg` execution for use via an interactive debugger. This will allow backwards
//! debugging via a snapshot + replay method. This is more efficient in terms of memory usage than
//! tracking every single state in each fsm after a VrMsg is received and allowing
//! stepping backwards by removing VrMsgs one at a time and reverting state. However, forward
//! stepping will be as fine grained as triggering individual sends of VRMsgs and inspecting state
//! of the FSM.

use v2r2::vr::{Dispatcher, DispatcherState, Envelope};
use debugger_shared::{Action};

pub struct Scheduler {
    pub dispatcher: Dispatcher
}

impl Scheduler {
    pub fn new(dispatcher: Dispatcher) -> Scheduler {
        Scheduler {
            dispatcher: dispatcher
        }
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
