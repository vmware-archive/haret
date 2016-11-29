use msg::Msg;
use rabble::{Pid, CorrelationId};
use super::vrmsg::VrMsg;

pub struct VrEnvelope {
    to: Pid,
    from: Pid,
    msg: VrMsg,
    correlation_id: CorrelationId
}

impl VrEnvelope {
    fn new(to: Pid, from: Pid, msg: VrMsg, correlation_id: CorrelationId) -> VrEnvelope {
        VrEnvelope {
            to: to,
            from: from,
            msg: msg,
            correlation_id: correlation_id
        }
    }
}

impl From<VrEnvelope> for Envelope {
    fn from(vr_envelope: VrEnvelope) -> Envelope {
        Envelope {
            to: vr_envelope.to,
            from: vr_envelope.from,
            msg: rabble::Msg::User(Msg::Vr(vr_envelope.msg)),
            correlation_id: correlation_id
        }
    }
}

