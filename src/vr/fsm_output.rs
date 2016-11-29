use rabble::{Pid, CorrelationId, Envelope};
use msg::Msg;
use namespace_msg::NamespaceMsg;
use super::vr_envelope::VrEnvelope;

pub enum FsmOutput {
    Vr(VrEnvelope),
    Announcement(NamespaceMsg, Pid)
}

/// Convert from FsmOuptput to Envelope with "self.into()" or Envelope::from(self)
impl From<FsmOutput> for Envelope {
    fn from(fsm_output: FsmOutput) -> Envelope {
        match fsm_output {
            Vr(vr_envelope) => vr_envelope.into(),
            Announcement(namespace_msg, pid) => Envelope {
                to: Pid {group: None, name: "namespace_mgr".to_string(), node: pid.node.clone()},
                from: pid.clone(),
                msg: rabble::Msg::User(Msg::Namespace(namespace_msg)),
                correlation_id: CorrelationId::Pid(pid)
            }
        }
    }
}

