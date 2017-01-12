use funfsm::constraints::{self, Constraints};
use haret::vr::{VrCtx, VrTypes, VrMsg, VrEnvelope, FsmOutput};

pub fn constraints() -> Constraints<VrTypes> {
    let mut c = Constraints::new();
    invariants(&mut c);
    preconditions(&mut c);
    transitions(&mut c);
    c
}

fn invariants(c: &mut Constraints<VrTypes>) {
    invariant!(c, |ctx: &VrCtx| ctx.commit_num <= ctx.op);
    invariant!(c, |ctx: &VrCtx| ctx.old_config != ctx.new_config);
}

fn preconditions(c: &mut Constraints<VrTypes>) {
    precondition!(c, "primary", |ctx: &VrCtx| ctx.last_normal_view == ctx.view);
    precondition!(c, "primary", |ctx: &VrCtx| ctx.primary.as_ref() == Some(&ctx.pid));
    precondition!(c, "primary", |ctx: &VrCtx| ctx.recovery_state.is_none());
    precondition!(c, "primary", |ctx: &VrCtx| ctx.view_change_state.is_none());
}

fn transitions(c: &mut Constraints<VrTypes>) {
    transition!(c, "primary" => "primary", primary_to_primary);
}

/// All assertions pertaining to a transition from "primary" state to "primary" state
fn primary_to_primary(init_ctx: &VrCtx,
                      final_ctx: &VrCtx,
                      envelope: &VrEnvelope,
                      output: &Vec<FsmOutput>) -> Result<(), String> {
   let s = "Transition from primary to primary";

   if let Some(view) = envelope.msg.get_view() {
       check!(s, view <= final_ctx.view);
   }

   if let Some(epoch) = envelope.msg.get_epoch() {
       check!(s, epoch <= final_ctx.epoch);
   }

   check!(s, final_ctx.last_normal_view == final_ctx.view);
   check!(s, final_ctx.recovery_state.is_none());
   check!(s, final_ctx.view_change_state.is_none());
   check!(s, final_ctx.last_normal_view == final_ctx.view);
   check!(s, final_ctx.primary.as_ref() == Some(&final_ctx.pid));

   check!(s, init_ctx.epoch == final_ctx.epoch);

   if let VrMsg::DoViewChange {view, ..} = envelope.msg {
       if output.len() == 0 {
           check!(s, view <= final_ctx.view);
       } else {
           check!(s, view == final_ctx.view);
       }
   }
   Ok(())
}
