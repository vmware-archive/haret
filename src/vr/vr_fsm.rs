use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use uuid::Uuid;
use time::{SteadyTime, Duration};
use rand::{thread_rng, Rng};
use fsm::{FsmTypes, StateFn};
use rabble::{self, Envelope, Pid, CorrelationId};
use namespace_msg::NamespaceMsg;
use super::replica::VersionedReplicas;
use super::VrBackend;
use super::quorum_tracker::QuorumTracker;
use super::prepare_requests::PrepareRequests;
use super::vrmsg::VrMsg;
use super::vr_api_messages::{VrApiReq, VrApiRsp};

pub const DEFAULT_IDLE_TIMEOUT_MS: u64 = 2000;
pub const DEFAULT_PRIMARY_TICK_MS: u64 = 500;

pub type Transition = (StateFn<VrTypes>, Vec<FsmOutput>);

#[derive(Debug, Clone)]
pub struct VrTypes;

impl FsmTypes for VrTypes {
    type Context = VrCtx;
    type Msg = VrEnvelope;
    type Output = FsmOutput;
}

macro_rules! check_epoch {
    ($ctx:ident, $msg:ident, $state:ident) => {
        if let Some(epoch) = $msg.get_epoch() {
            if epoch < $ctx.epoch {
                return next!($state);
            }
            if epoch > $ctx.epoch {
                return start_reconfiguration($ctx, $msg, epoch);
            }
        }
    }
}

macro_rules! check_view {
    ($ctx:ident, $msg:ident, $state:ident) => {
        if let Some(view) = $msg.get_view() {
            if view < $ctx.view {
                return next!($state);
            }
            if view > $ctx.view {
                return super::states::view_change::handle_later_view($ctx, $msg, view);
            }
        }
    }
}

macro_rules! handle_common {
    ($ctx:ident, $msg:ident, $state:ident) => {
        check_epoch!($ctx, $msg, $state);
        check_view!($ctx, $msg, $state);
    }
}

pub fn start_reconfiguration(ctx: &mut VrCtx, msg: VrMsg, epoch: u64) -> Transition {
    if epoch == ctx.epoch + 1 {
        match msg {
            VrMsg::Commit {commit_num, ..} if commit_num != ctx.op => {
                ctx.last_received_time = SteadyTime::now();
                ctx.epoch = epoch;
                ctx.view = 0;
                let output = ctx.start_state_transfer();
                return next!(reconfiguration_wait_for_new_state, output);
            },

            VrMsg::Commit {commit_num, ..} => {
                ctx.last_received_time = SteadyTime::now();
                ctx.epoch = epoch;
                ctx.view = 0;
                if ctx.backup_commit_replace_replica(commit_num) {
                    return next!(leaving);
                }
                let output = ctx.send_epoch_started(ctx);
                if ctx.is_primary() {
                    return next!(primary, output);
                }
                next!(backup, output)
            },

            VrMsg::StartEpoch {op, old_config, new_config, ..}
                if ctx.op == op && ctx.last_log_entry_is_latest_reconfiguration(epoch, op) =>
            {
                ctx.set_state_new_epoch(epoch, old_config, new_config);
                ctx.backup_commit(op);
                let output = ctx.send_epoch_started();
                if ctx.is_primary() {
                    return next!(primary, output)
                }
                return next!(backup, output)
            },

            VrMsg::StartEpoch {op, old_config, new_config, ..} => {
                ctx.set_state_new_epoch(epoch, old_config, new_config);
                let output = ctx.start_state_transfer();
                next!(reconfiguration_wait_for_new_state, output)
            },
            // We have been partitioned and missed the reconfiguration. At least a quorum of new nodes
            // is normally processing requests, and the leaving replicas may have shut down.
            // We don't yet know what the 'new_config' is though.
            _ => {
                return learn_config(ctx, epoch);
            }
        }
    }
    // We have been partitioned for one or multiple reconfigurations. We need to catch up.
    // We don't know what the old or new configs are to ask for the latest state. Therefore we wait
    // for a normal message in the new epoch with a 'from' field. We can then begin state transfer
    // using that replica.
    learn_config(ctx, epoch)
}

pub fn handle_later_view(ctx: &mut VrCtx, msg: VrMsg, new_view: u64) -> Transition {
    match msg.clone() {
        VrMsg::StartViewChange {ref from, ..} => {
            ctx.reset_quorum(new_view, from.clone(), msg);
            return start_view_change(ctx, Vec::new());
        },
        VrMsg::DoViewChange {ref from, ..} => {
            ctx.reset_quorum(new_view, from.clone(), msg.clone());
            return ctx.maybe_become_primary(ctx, from.clone(), msg);
        },
        VrMsg::StartView{view, op, log, commit_num, ..} => {
            let output = ctx.become_backup(ctx, msg);
            return next!(backup, output);
        },
        _ => ctx.start_state_transfer_new_view(ctx, new_view)
    }
}


///////////////////////////////////////////////////////////////////////////////////////////////////
// FSM STATES
///////////////////////////////////////////////////////////////////////////////////////////////////
// Startup states always receive a tick message to kick things off

pub fn startup_new_namespace(ctx: &mut VrCtx, _: VrEnvelope) -> Transition {
    ctx.view += 1;
    super::view_change::start(ctx, Vec::new())
}

pub fn startup_recovery(ctx: &mut VrCtx, _: VrEnvelope) -> Transition {
    ctx.start_recovery();
}

pub fn startup_reconfiguration(ctx: &mut VrCtx, _: VrEnvelope) -> Transition {
    let output = ctx.start_state_transfer();
    return next!(reconfiguration_wait_for_new_state, output);
}

/// This replica is currently the primary replica operating normally
// @StateFn
pub fn primary(ctx: &mut VrCtx, envelope: VrEnvelope) -> Transition {
    handle_common!(ctx, envelope.msg, primary);
    match envelope.msg {
        VrMsg::ClientRequest {..} => {
            let output =  ctx.send_prepare(envelope);
            next!(primary, output)
        },
        VrMsg::PrepareOk {op, ref from, ..} op > ctx.commit_num => {
            let output = ctx.maybe_commit(op, from.clone(), envelope.correlation_id)
            next!(primary, output)
        },
        VrMsg::Tick => {
            if ctx.prepare_requests.expired() e
                return ctx.reset_and_start_view_change();
            }
            if primary_idle_timeout(ctx) {
                let output = ctx.broadcast_commit_msg();
                return next!(primary, output);
            }
            next!(primary)
        },
        VrMsg::GetState{..} => {
            let output = ctx.send_new_state(msg);
            next!(primary, output)
        },
        VrMsg::Recovery{..} => {
            let output = ctx.send_primary_recovery_response(msg);
            next!(primary, output)
        },
        VrMsg::Reconfiguration{..} => {
            if let Some(err_envelope) = ctx.validate_reconfig(&envelope) {
                return next!(primary, vec![err_envelope]);
            }
            let output = ctx.send_prepare(envelope.clone());
            next!(wait_for_reconfiguration_prepare_ok, output)
        },
        _ => next!(primary)
    }
}

/// This replica is currently a backup operating normally
// @StateFn
pub fn backup(ctx: &mut VrCtx, envelope: VrEnvelope) -> Transition {
    handle_common!(ctx, msg, backup);
    ctx.last_received_time = SteadyTime::now();
    match envelope.msg {
        VrMsg::Prepare {op, client_op, commit_num, client_request_num, ...} if op == ctx.op + 1 => {
            let prepare_ok_envelope = ctx.send_prepare_ok(client_op, commit_num, request_num);
            next!(backup, vec![prepare_ok_envelope])
        },
        VrMsg::Prepare {op, ..} if op > ctx.op + 1 => {
            let output = ctx.send_get_state();
            next!(state_transfer, output)
        },
        VrMsg::Commit {commit_num, ..} => if commit_num == ctx.commit_num {
            next!(backup)
        },
        VrMsg::Commit {commit_num, ..} => if commit_num == ctx.op {
            ctx.backup_commit(ctx);
            next!(backup)
        },
        VrMsg::Commit {commit_num, ..} => if commit_num > ctx.op {
            // Note that a primary cannot have a commit_num smaller than a backup by protocol
            // invariants
            let output = ctx.send_get_state();
            next!(state_transfer, output)
        },
        VrMsg::Tick => {
            if idle_timeout(ctx) {
                ctx.reset_and_start_view_change()
            } else {
                next!(backup)
            }
        },
        VrMsg::GetState{op, from, ..} => {
            let output = ctx.send_new_state(op, from);
            next!(backup, output)
        },
        VrMsg::Recovery{from, nonce} => {
            if *ctx.primary.as_ref().unwrap() == from {
                return ctx.reset_and_start_view_change()
            }
            let output = ctx.send_backup_recovery_response(from, nonce);
            return next!(backup, vec![output]);
        },
        _ => {
            println!("Re-entering backup state {:?}", ctx.me.name);
            next!(backup)
        }
    }
}

/// A replica is in this state during view change. The replica remains in this state until view
/// change is completed unless it is the proposed primary, which transitions to the `wait_for_do_view_change`
/// state on receipt of a `DoViewChange` message.
// @StateFn
pub fn view_change(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    handle_common!(ctx, msg, view_change);
    match msg.clone() {
        VrMsg::StartViewChange{ref from, ..} => {
            ctx.quorum_tracker.insert(from.clone(), msg.clone());
            ctx.maybe_send_do_view_change(ctx, Vec::new())
        },
        VrMsg::DoViewChange {view, ref from, ..} => {
            ctx.reset_quorum(view, from.clone(), msg.clone());
            return ctx.maybe_become_primary(from.clone(), msg);
        },
        VrMsg::StartView{view, op, log, commit_num, ..} => {
            let output = ctx.become_backup(view, op, log, commit_num);
            return next!(backup, output);
        },
        VrMsg::Tick => {
            // We haven't changed views yet. The new primary must be down. Try again.
            if ctx.quorum_tracker.is_expired(&u64_to_duration(ctx.idle_timeout_ms)) {
                ctx.reset_and_start_view_change()
            } else {
                next!(view_change)
            }
        },
        VrMsg::Recovery {ref from, ..}  => {
            // We don't handle recovery messages in view_change state. However, if we receive one
            // from the primary for this view we know the election will never complete and trigger a
            // new view change immediately rather than waiting. It's possible this is an old
            // recovery message lost in the network, but an extra view change is still safe.  It's
            // impossible that the recovery message is old when using TCP as transport.
            if let Some(primary) = ctx.primary.clone() {
                if primary == *from {
                    return ctx.reset_and_start_view_change(ctx);
                }
            }
            next!(view_change)
        },
        VrMsg::Prepare {view, ..} => ctx.start_state_transfer_new_view(view),
        VrMsg::Commit {view, ..} => ctx.start_state_transfer_new_view(view),
        _ => next!(view_change)
    }
}

/// At least one replica has received a quorum of `StartViewChange` messages for a given view.
/// It has sent a `DoViewChange` message to the proposed primary for that view. In this state the
/// proposed primary is waiting for a quorum of `DoViewChange` messages so that it can become the
/// primary for that view.
// @StateFn
pub fn wait_for_do_view_change(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    handle_common!(ctx, msg, wait_for_do_view_change);
    match msg.clone() {
        VrMsg::DoViewChange{from, ..} => ctx.maybe_become_primary(from, msg),
        _ => next!(wait_for_do_view_change)
    }
}


/// When a backup realizes it's behind it enters this state
// @StateFn
pub fn state_transfer(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    handle_common!(ctx, msg, state_transfer);
    match msg {
        VrMsg::NewState {..} => {
            let output = vec![ctx.set_state(msg)];
            next!(backup, output)
        },
        _ => next!(state_transfer)
    }
}

/// This is the state of the primary after it has sent a Prepare message containing the client
/// reconfiguration request.
// @StateFn
pub fn wait_for_reconfiguration_prepare_ok(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    handle_common!(ctx, msg, wait_for_reconfiguration_prepare_ok);
    match msg {
        VrMsg::PrepareOk {op, ref from, ..} if op == ctx.op => {
            let output = maybe_commit(ctx, op, from);
            if output.is_empty() {
                return next!(wait_for_reconfiguration_prepare_ok);
            }
            // We committed the reconfiguration request
            if ctx.is_primary() {
                return next!(primary, output);
            }
            next!(backup, output)
        },
        VrMsg::Tick => {
            // If we haven't received quorum of PrepareOk in time then re-broadcast Prepare
            if ctx.prepare_requests.expired() {
                let output = ctx.rebroadcast_reconfig();
                return next!(wait_for_reconfiguration_prepare_ok, output);
            }
            next!(wait_for_reconfiguration_prepare_ok)
        }
        _ => next!(wait_for_reconfiguration_prepare_ok)
    }
}

// @StateFn
pub fn reconfiguration_wait_for_new_state(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    handle_common!(ctx, msg, reconfiguration_wait_for_new_state);
    match msg {
        VrMsg::NewState {op, ..} if op >= ctx.new_config.op => {
            let mut output = vec![ctx.set_state(msg)];
            if ctx.replicas_to_replace().contains(&ctx.me) {
                return next!(leaving, output)
            }
            output.extend_from_slice(&ctx.send_epoch_started(ctx));
            if ctx.is_primary() {
                return next!(primary, output)
            }
            next!(backup, output)
        },
        VrMsg::Tick => {
            // If we haven't gotten a NewState in time then re-broadcast GetState
            if idle_timeout(ctx) {
                let output = ctx.start_state_transfer();
                return next!(reconfiguration_wait_for_new_state, output);
            }
            next!(reconfiguration_wait_for_new_state)
        },
        _ => next!(reconfiguration_wait_for_new_state)
    }
}


/// A replica has just started back up and needs to recover its log from other replicas
// @StateFn
pub fn recovery(ctx: &mut VrCtx, envelope: VrEnvelope) -> Transition {
    check_epoch!(ctx, envelope.msg, recovery);
    match envelope.msg {
        VrMsg::RecoveryResponse {view,
                                 ref op,
                                 ref nonce,
                                 ref from, ..} if Some(*nonce) == ctx.recovery_nonce => {
            if let Some(output) =
                ctx.maybe_recover(ctx, msg.clone(), view, op.is_some(), from.clone()) {
                return next!(backup, output)
            }
            next!(recovery)
        },
        VrMsg::Tick => {
            let timeout = u64_to_duration(ctx.idle_timeout_ms);
            if ctx.quorum_tracker.is_expired(&timeout) {
                ctx.start_recovery()
            } else {
                next!(recovery)
            }
        },
        _ => next!(recovery)
    }
}


/// A replica is in this state after it has a reconfiguration request in its log and it is not in
/// the new configuration. It is waiting for f' + 1 EpochStarted messages so that it can shutdown.
// @StateFn
pub fn leaving(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    match msg {
        // TODO: What if epoch > ctx.epoch ?
        VrMsg::EpochStarted {epoch, ref from, ..} if epoch == ctx.epoch => {
            ctx.quorum_tracker.insert(from.clone(), msg.clone());
            if ctx.quorum_tracker.has_quorum() {
                clear_quorum_tracker(ctx);
                let output = vec![FsmOutput::Announcement(NamespaceMsg::Stop(ctx.me.clone()),
                                                          ctx.me.clone())];
                return next!(shutdown, output)
            }
            next!(leaving)
        },
        VrMsg::Tick => {
            let timeout = u64_to_duration(ctx.idle_timeout_ms);
            if ctx.quorum_tracker.is_expired(&timeout) {
                let output = broadcast_start_epoch(ctx);
                return next!(leaving, output);
            }
            next!(leaving)
        }
        _ => next!(leaving)
    }
}

/// This replica has already told the dispatcher to shut it down. It just waits in this state and
/// doesn't respond to any messages from this point out.
// @StateFn
pub fn shutdown(_ctx: &mut VrCtx, _: VrMsg) -> Transition {
    next!(shutdown)
}


///////////////////////////////////////////////////////////////////////////////////////////////////
// END OF FSM STATES
///////////////////////////////////////////////////////////////////////////////////////////////////


fn learn_config(_ctx: &mut VrCtx, _epoch: u64) -> Transition {
    // TODO: I think this is just an instance of state transfer to a new view, using the replica that informed us of a later epoch. Basically, we set our epoch than call start_state_transfer_new_view(). Talk to Justin about this. How is start_state_transfer_new_view() different from start_state_transfer_transitioning()?
    unimplemented!();
}

pub fn u64_to_duration(timeout: u64) -> Duration {
    Duration::milliseconds(timeout as i64)
}

