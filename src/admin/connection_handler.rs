use rabble::{Pid, Envelope, ConnectionMsg, ConnectionHandler};
use vr::VrMsg;

pub struct AdminConnectionHandler {
    pid: Pid,
    id: usize,
    total_requests: usize
}

impl ConnectionHandler for AdminConnectionHandler {
    type Msg = VrMsg;
    type ClientMsg = AdminClientMsg;
}
