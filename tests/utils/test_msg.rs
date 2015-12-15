/// All the messages that can be generated for the fuzz test. They are a bit more descriptive than
/// actual VrMsg messages, since some messages (like VrMsg::Tick) can signal different things.

use uuid::Uuid;
use v2r2::vr::{VrMsg, Replica};

pub trait CausalMsg {
    fn causal_id(&self) -> Option<Uuid>;
}

#[derive(Debug, Clone)]
pub enum TestMsg {
    ClientRequest(VrMsg),
    Commit,
    // Specify which backup to send the VrMsg::Tick to
    ViewChange(Replica),

    // Uuid's are causal IDs that tie crashes together with restarts. This way during shrinking if
    // we remove either a crash or restart we can remove the other and not have incorrect
    // (non-linearizable) histories
    Crash(Replica, Uuid),
    Restart(Replica, Uuid),
    // This is just an initialization state
    Null
}

impl CausalMsg for TestMsg {
    fn causal_id(&self) -> Option<Uuid> {
        match *self {
            TestMsg::Crash(_, uuid) => Some(uuid.clone()),
            TestMsg::Restart(_, uuid) => Some(uuid.clone()),
            _ => None
        }
    }
}
