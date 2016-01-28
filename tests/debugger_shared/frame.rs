use rustc_serialize::Encodable;
use super::{Action, TestMsg};

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct Frame {
    pub test_msg: TestMsg,
    pub actions: Vec<Action>
}

/// A granularity of a history that allows stepping between TestMsgs
impl Frame {
    pub fn new(test_msg: TestMsg) -> Frame {
        Frame {
            test_msg: test_msg,
            actions: Vec::new()
        }
    }

    pub fn push(&mut self, action: Action) {
        self.actions.push(action);
    }
}

