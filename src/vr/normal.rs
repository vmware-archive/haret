use vr_fsm::{Transition, VrStates, Primary, Backup};
use vr_msg::{ClientOp, ClientRequest, Reconfiguration, ClientReply, Prepare, PrepareOk};

impl Primary {
    fn send_prepare(self, msg: Msg, from: Pid, cid: CorrelationId, output: &mut Vec<FsmOutput>) {
        self.ctx.last_received_time = SteadyTime::now();
        self.ctx.op += 1;
        let prepare = Prepare {
            epoch: self.ctx.epoch,
            view: self.ctx.view,
            op: self.ctx.op,
            commit_num: self.ctx.commit_num,
            msg: msg.clone().into()
        };
        self.ctx.log.push(msg.into());
        self.prepare_requests.new_prepare(self.ctx.op, cid.clone());
        self.ctx.broadcast(prepare, cid, output);
    }
}

impl Transition<ClientRequest> for Primary {
    fn next(self,
            msg: Msg,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        self.send_prepare(msg, from, cid, output);
        Primary.into()
    }
}
