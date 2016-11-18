use rabble::{self, Pid, CorrelationId};
use msg::Msg;

/// A replica wraps a VR FSM as a process so that it can receive messsages from inside rabble
/// It also takes care to only pass messages of type VrMsg to the FSM.
struct Replica {
    pid: pid,
    fsm: Fsm<VrTypes>,
}

impl Replica {
    fn new(pid: Pid, fsm: Fsm<VrTypes>) -> Replica {
        Replica {
            pid: Pid,
            fsm: fsm
        }
    }
}

impl Process for Replica {
    type Msg = Msg;

    fn init(&mut self, _: Pid) -> Vec<Envelope<Msg>> {
        fsm.send(VrMsg::Tick)
    }

    fn handle(&mut self,
              msg: rabble::Msg<Msg>,
              from: Pid,
              correlation_id: Option<CorrelationId>) -> &mut Vec<Envelope<Msg>>
    {
    }

}


/// When we create a namesapce, the initial group of replicas is at epoch 1. When a reconfiguration
/// occurs we bump the epoch so when we gossip the information around to start the fsms, we can
/// chose the latest version.
#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct VersionedReplicas {
    pub epoch: u64,
    pub op: u64,
    pub replicas: Vec<Pid>
}

impl VersionedReplicas {
    pub fn new() -> VersionedReplicas {
        VersionedReplicas {
            epoch: 0,
            op: 0,
            replicas: Vec::new()
        }
    }
}
