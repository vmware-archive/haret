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
use super::vr_ctx::VrCtx;
use super::fsm_output::FsmOutput;
use super::vr_envelope::VrEnvelope;

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
    ($ctx:ident, $envelope:ident, $state:ident) => {
        if let Some(epoch) = $envelope.msg.get_epoch() {
            if epoch < $ctx.epoch {
                return next!($state);
            }
            if epoch == $ctx.epoch + 1 {
                return start_reconfiguration($ctx, $envelope.msg, epoch);
            }
            if epoch > $ctx.epoch + 1 {
                // We have been partitioned for one or multiple reconfigurations. We need to catch up.
                // We don't know what the old or new configs are to ask for the latest state. Therefore we wait
                // for a normal message in the new epoch with a 'from' field. We can then begin state transfer
                // using that replica.
                return learn_config(ctx, epoch);
            }
        }
    }
}

macro_rules! check_view {
    ($ctx:ident, $envelope:ident, $state:ident) => {
        if let Some(view) = $envelope.msg.get_view() {
            if view < $ctx.view {
                return next!($state);
            }
            if view > $ctx.view {
                return handle_later_view($ctx, $envelope, view);
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
    ctx.last_received_time = SteadyTime::now();
    ctx.epoch = epoch;
    match msg {
        VrMsg::Commit {commit_num, ..} if commit_num != ctx.op => {
            let output = ctx.start_state_transfer_reconfiguration();
            return next!(reconfiguration_wait_for_new_state, output);
        },

        VrMsg::Commit {commit_num, ..} => {
            ctx.view = 0;
            ctx.backup_commit(commit_num);
            if ctx.is_leaving() {
                return next!(leaving);
            }
            let output = ctx.send_epoch_started(ctx);
            if ctx.is_primary() {
                return next!(primary, output);
            }
            next!(backup, output)
        },

        VrMsg::StartEpoch {op, old_config, new_config, ..} if ctx.op == op &&
            ctx.last_log_entry_is_latest_reconfiguration(epoch, op) =>
        {
            ctx.set_state_new_epoch(old_config, new_config);
            ctx.backup_commit(op);
            let output = ctx.send_epoch_started();
            if ctx.is_primary() {
                return next!(primary, output)
            }
            return next!(backup, output)
        },

        VrMsg::StartEpoch {op, old_config, new_config, ..} => {
            ctx.set_state_new_epoch(old_config, new_config);
            let output = ctx.start_state_transfer_reconfiguration();
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

pub fn handle_later_view(ctx: &mut VrCtx, envelope: VrEnvelope, new_view: u64) -> Transition {
    match envelope.msg.clone() {
        VrMsg::StartViewChange {from, ..} => {
            let output = ctx.reset_view_change_state(view);
            ctx.insert_view_change_message(from, envelope.msg);
            output.extend(ctx.start_view_change());
            next!(wait_for_start_view_change, output)
        },
        VrMsg::DoViewChange {from, ..} => {
            let output = ctx.reset_view_change_state(view);
            ctx.insert_view_change_message(from, envelope.msg);
            next!(wait_for_do_view_change, output);
        },
        VrMsg::StartView{view, op, log, commit_num, ..} => {
            let output = ctx.become_backup(view, op, log, commit_num);
            return next!(backup, output);
        },
        _ => {
            // We've missed the view change and are likely out of date
            ctx.view = new_view;
            let output = vec![ctx.send_get_state_to_random_replica(envelopes.correlation_id)];
            next!(state_transfer, output)
        }
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
    let envelopes = ctx.start_recovery();
    next!(recovery, envelopes.map(|e| e.into()))
}

pub fn startup_reconfiguration(ctx: &mut VrCtx, _: VrEnvelope) -> Transition {
    let output = ctx.start_state_transfer_reconfiguration();
    return next!(reconfiguration_wait_for_new_state, output);
}

/// This replica is currently the primary replica operating normally
pub fn primary(ctx: &mut VrCtx, envelope: VrEnvelope) -> Transition {
    handle_common!(ctx, envelope.msg, primary);
    match envelope.msg {
        VrMsg::ClientRequest {..} => {
            let output =  ctx.send_prepare(envelope);
            next!(primary, output)
        },
        VrMsg::PrepareOk {op, from, ..} op > ctx.commit_num => {
            let output = ctx.maybe_commit(op, from, envelope.correlation_id)
            next!(primary, output)
        },
        VrMsg::Tick => {
            if ctx.prepare_requests.expired() {
                let output = ctx.reset_and_start_view_change();
                next!(wait_for_start_view_change, output)
            }
            if ctx.primary_idle_timeout() {
                let output = ctx.broadcast_commit_msg();
                return next!(primary, output);
            }
            next!(primary)
        },
        VrMsg::GetState{op, from, ..} => {
            let output = ctx.send_new_state(op, from, envelope.correlation_id);
            next!(primary, output)
        },
        VrMsg::Recovery{from, nonce} => {
            let output = ctx.send_recovery_response(from, nonce, envelope.correlation_id);
            next!(primary, vec![output])
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
pub fn backup(ctx: &mut VrCtx, envelope: VrEnvelope) -> Transition {
    handle_common!(ctx, envelope, backup);
    ctx.last_received_time = SteadyTime::now();
    match envelope.msg {
        VrMsg::Prepare {op, client_op, commit_num, client_request_num, ...} if op == ctx.op + 1 => {
            let prepare_ok_envelope = ctx.send_prepare_ok(client_op, commit_num, request_num,
                                                          envelope.correlation_id);
            next!(backup, vec![prepare_ok_envelope])
        },
        VrMsg::Prepare {op, ..} if op > ctx.op + 1 => {
            let output = vec![ctx.send_get_state_to_random_replica(envelope.correlation_id)];
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
            let output = ctx.send_get_state_to_random_replica(envelope.correlation_id);
            next!(state_transfer, output)
        },
        VrMsg::Tick => {
            if ctx.idle_timeout() {
                let output = ctx.reset_and_start_view_change();
                next!(wait_for_start_view_change, output)
            } else {
                next!(backup)
            }
        },
        VrMsg::GetState{op, from, ..} => {
            let output = ctx.send_new_state(op, from, envelope.correlation_id);
            next!(backup, output)
        },
        VrMsg::Recovery{from, nonce} => {
            if *ctx.primary.as_ref().unwrap() == from {
                let output = ctx.reset_and_start_view_change();
                next!(wait_for_start_view_change, output)
            }
            let output = ctx.send_recovery_response(from, nonce, envelope.correlation_id);
            return next!(backup, vec![output]);
        },
        _ => {
            println!("Re-entering backup state {:?}", ctx.me.name);
            next!(backup)
        }
    }
}

/// The first phase of a view change. Here a replica is waiting for a quorum of StartViewChange
/// messages.
pub fn wait_for_start_view_change(ctx: &mut VrCtx, envelope: VrEnvelope) -> Transition {
    handle_common!(ctx, envelope, view_change);
    match envelope.msg.clone() {
        VrMsg::StartViewChange{ref from, ..} => {
            ctx.insert_view_change_message(from, envelope.msg);
            if ctx.has_view_change_quorum() {
                if ctx.compute_primary() == ctx.me {
                    return next!(wait_for_do_view_change)
                }
                let output = ctx.send_do_view_change();
                return next!(wait_for_start_view, vec![output]);
            }
            next!(wait_for_start_view_change)
        },
        VrMsg::DoViewChange {view, from, ..} => {
            // Another replica got quorum of StartViewChange messages for this view and computed
            // that we are the primary for this view.
            let output = ctx.reset_view_change_state(view);
            ctx.insert_view_change_message(from, envelope.msg);
            next!(wait_for_do_view_change, output)
        },
        VrMsg::StartView{view, op, log, commit_num, ..} => {
            // Another replica was already elected primary for this view.
            let output = ctx.become_backup(view, op, log, commit_num);
            return next!(backup, output);
        },
        VrMsg::Tick => {
            if ctx.view_change_expired() {
                // We haven't changed views yet. Try again.
                let output = ctx.reset_and_start_view_change();
                next!(wait_for_start_view_change, output)
            } else {
                next!(wait_for_start_view_change)
            }
        },
        VrMsg::Recovery {ref from, ..}  => {
            // We don't handle recovery messages in view_change state. However, if we receive one
            // from the primary for this view we know the election will never complete and trigger a
            // new view change immediately rather than waiting. It's possible this is an old
            // recovery message lost in the network, but an extra view change is still safe.  It's
            // impossible that the recovery message is old when using TCP as transport.
            if let Some(primary) = ctx.compute_primary() {
                if primary == *from {
                    let output = ctx.reset_and_start_view_change();
                    return next!(wait_for_start_view_change, output);
                }
            }
            next!(wait_for_start_view_change)
        },
        VrMsg::Prepare {..} => {
            // Another replica was already elected primary for this view.
            let output = vec![ctx.send_get_state_to_random_replica(envelope.correlation_id)];
            next!(state_transfer, output)
        },
        VrMsg::Commit {..} => {
            // Another replica was already elected primary for this view.
            let output = vec![ctx.send_get_state_to_random_replica(envelope.correlation_id)];
            next!(state_transfer, output)
        }
        _ => next!(wait_for_start_view_change)
    }
}

/// The second phase of a ViewChange for the proposed primary of the new view. At least one replica
/// has received a quorum of `StartViewChange` messages for a given view.  It has sent a
/// `DoViewChange` message to the proposed primary for that view. In this state the proposed primary
/// is waiting for a quorum of `DoViewChange` messages so that it can become the primary for that
/// view.
pub fn wait_for_do_view_change(ctx: &mut VrCtx, envelope: VrEnvelope) -> Transition {
    handle_common!(ctx, envelope, wait_for_do_view_change);
    match envelope.msg.clone() {
        VrMsg::DoViewChange{from, ..} => {
            ctx.insert_view_change_message(from, envelope.msg)
            if ctx.has_view_change_quorum() {
                let output = ctx.become_primary()
                return next!(primary, output);
            }
            next!(wait_for_do_view_change, output)
        },
        VrMsg::Tick => {
            if ctx.view_change_expired() {
                // We haven't changed views yet. Try again.
                let output = ctx.reset_and_start_view_change();
                next!(wait_for_start_view_change, output)
            } else {
                next!(wait_for_start_view_change)
            }
        },
        _ => next!(wait_for_do_view_change)
    }
}

/// The second phase of a ViewChange for a proposed backup in the new view. In this state, this
/// replica has sent a `DoViewChange` message to the proposed primary for the new view. When the
/// proposed primary has gotten a quorum of `DoViewChange` messages it will broadcast a `StartView`
/// message to all new backups. This replica is waiting for that `StartView` message.
pub fn wait_for_start_view(ctx: &mut VrCtx, envelope: VrEnvelope) -> Transition {
    handle_common!(ctx, envelope, wait_for_start_view);
    match envelope.msg.clone() {
        VrMsg::StartView {view, op, log, commit_num, ..} => {
            let output = ctx.become_backup(view, op, log, commit_num);
            return next!(backup, output);
        },
        VrMsg::Tick => {
            if ctx.view_change_expired() {
                // We haven't changed views yet. Try again.
                let output = ctx.reset_and_start_view_change();
                next!(wait_for_start_view_change, output)
            } else {
                next!(wait_for_start_view_change)
            }
        },
        VrMsg::Prepare {..} => {
            // We missed the StartView message
            let output = vec![ctx.send_get_state_to_random_replica(envelope.correlation_id)];
            next!(state_transfer, output)
        },
        VrMsg::Commit {..} => {
            // We missed the StartView message
            let output = vec![ctx.send_get_state_to_random_replica(envelope.correlation_id)];
            next!(state_transfer, output)
        }
        _ => next!(wait_for_start_view)
    }
}


/// When a backup realizes it's behind it enters this state
pub fn state_transfer(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    handle_common!(ctx, envelope, state_transfer);
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
pub fn wait_for_reconfiguration_prepare_ok(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    handle_common!(ctx, envelope, wait_for_reconfiguration_prepare_ok);
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

pub fn reconfiguration_wait_for_new_state(ctx: &mut VrCtx, msg: VrMsg) -> Transition {
    handle_common!(ctx, envelope, reconfiguration_wait_for_new_state);
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
            if ctx.idle_timeout() {
                let output = ctx.start_state_transfer_reconfiguration();
                return next!(reconfiguration_wait_for_new_state, output);
            }
            next!(reconfiguration_wait_for_new_state)
        },
        _ => next!(reconfiguration_wait_for_new_state)
    }
}


/// A replica has just started back up and needs to recover its log from other replicas
pub fn recovery(ctx: &mut VrCtx, envelope: VrEnvelope) -> Transition {
    match envelope.msg {
        VrMsg::RecoveryResponse {..} => {
            ctx.update_recovery_state(envelope.msg);
            ctx.commit_recovery().map_or(next!(recovery), |output| next!(backup, vec![output]))
        },
        VrMsg::Tick => {
            if ctx.recovery_expired() {
                let envelopes = ctx.start_recovery();
                next!(recovery, envelopes.map(|e| e.into()))
            } else {
                next!(recovery)
            }
        },
        _ => next!(recovery)
    }
}


/// A replica is in this state after it has a reconfiguration request in its log and it is not in
/// the new configuration. It is waiting for f' + 1 EpochStarted messages so that it can shutdown.
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
            if ctx.quorum_tracker.is_expired(&ctx.idle_timeout) {
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
