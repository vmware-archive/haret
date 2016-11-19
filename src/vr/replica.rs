use rabble::{self, Pid, CorrelationId, Envelope};
use msg::Msg;

/// A replica wraps a VR FSM as a process so that it can receive messsages from inside rabble
/// It also takes care to only pass messages of type VrMsg to the FSM.
struct Replica {
    pid: pid,
    fsm: Fsm<VrTypes>,
    output: Vec<Envelope<Msg>>
}

impl Replica {
    fn new(pid: Pid, fsm: Fsm<VrTypes>) -> Replica {
        Replica {
            pid: Pid,
            fsm: fsm,
            output: Vec::with_capacity(1)
        }
    }
}

impl Process for Replica {
    type Msg = Msg;

    fn init(&mut self, _: Pid) -> Vec<Envelope<Msg>> {
        self.fsm.send(VrMsg::Tick)
    }

    fn handle(&mut self,
              msg: rabble::Msg<Msg>,
              from: Pid,
              correlation_id: Option<CorrelationId>) -> &mut Vec<Envelope<Msg>>
    {
        let correlation_id = correlation_id.unwrap();
        match msg {
            rabble::Msg::User(Msg::AdminReq(AdminReq::GetReplicaState(_))) => {
                let (state, ctx) = self.fsm.get_state();
                let rpy = AdminRpy::ReplicaState {state: state, ctx: ctx};
                let msg = rabble::Msg::User(Msg::AdminRpy(rpy));
                let envelope = Envelope::new(from, self.pid.clone(), msg, Some(correlation_id));
                self.output.push(envelope);
            },
            rabble::Msg::User(Msg::Vr(vrmsg)) =>
                self.output.extend(self.fsm.send((vrmsg, correlation_id))),
            _ => {
                let msg = rabble::Msg::User(Msg::Error("Invalid Msg Received".to_string()));
                let envelope = Envelope::new(from, self.pid.clone(), msg, Some(correlation_id));
                self.output.push(envelope);
            }
        }
        &mut self.output
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
