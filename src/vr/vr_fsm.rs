// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use time::SteadyTime;
use namespace_msg::NamespaceMsg;
use super::vrmsg::VrMsg;
use super::vr_ctx::VrCtx;
use super::fsm_output::FsmOutput;
use super::vr_envelope::VrEnvelope;
use std::convert::From;

pub trait Transition<Msg> {
    fn next(self,
            msg: Msg,
            from: Pid,
            correlation_id: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates;
}

/// Generate a state struct: `$struct_name` from a set of fields
///
/// Generate `impl From<$struct_name> for VrStates`
macro_rules! state {
    ($struct_name:ident {
        $( $field:ident: $ty:ident ),*
    }) => {
        #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
        pub struct $struct_name {
            $( pub $field: $ty ),*
        }

        impl From<$struct_name> for VrStates {
            fn from(msg: $struct_name) -> VrStates {
                VrMsg::$struct_name(msg)
            }
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum VrStates {
    Primary(Primary),
    Backup(Backup),
    WaitForNewState(WaitForNewState),
    WaitForStartViewChange(WaitForStartViewChange),
    WaitForDoViewChange(WaitForDoViewChange),
    WaitForStartView(WaitForStartView),
    Recovery(Recovery),
    ReconfigurationWaitForPrepareOk(ReconfigurationWaitForPrepareOk),
    ReconfigurationWaitForNewState(ReconfigurationWaitForNewState),
    Leaving(Leaving),
    Shutdown(Shutdown)
}

/// The primary state of the VR protocol operating in normal mode
state!(Primary {
    pub ctx: VrCtx,
    pub prepare_requests: PrepareRequests,
    /// If the primary doesn't receive a new client request in `primary_tick_ms` it sends a commit
    /// message to the backups. `idle_timeout` should be at least twice as large as this value.
    pub tick_ms: u64
});

/// The backup state of the VR protocol operating in normal mode
state!(Backup {
    pub ctx: VrCtx,
    pub primary: Pid,
    /// Backups wait `idle_timeout` between messages from the primary before initiating a view
    /// change.
    pub idle_timeout: Duration
});

/// When a backup realizes it's behind it enters state transfer
///
/// In this state, the backup is waiting for a `NewState` message
state!(WaitForNewState {
    pub ctx: VrCtx,
    pub idle_timeout: Duration
});

/// The part of the view change state in the VR protocol state machine where a replica is waiting
/// for a quorum of `StartViewChange` messages.
state!(WaitForStartViewChange {
    pub ctx: VrCtx,
    pub state: ViewChangeState
});

/// The part of the view change state in the VR protocol state machine the proposed primary is
/// waiting for a quorum of `DoViewChange` messages.
state!(WaitForDoViewChange {
    pub ctx: VrCtx,
    pub state: ViewChangeState
});

/// The part of the view change state in the VR protocol state machine where a replica is waiting
/// for a `StartView` message from the new primary. It has already sent a `DoViewChange` to the
/// proposed primary for this view.
state!(WaitForStartView {
    pub ctx: VrCtx,
    pub proposed_primary: Pid
});

/// The recovery state of the VR Protocol where a replica is recovering data from a quorum of
/// replicas
state!(Recovery {
    pub ctx: VrCtx,
    pub state: RecoveryState
});

/// The first step for the primary in the reconfiguration part of the VR protocol
///
/// The primary has sent a `Prepare` containing the reconfiguration and is waiting for a quorum of
/// `PrepareOk` messages
state!(ReconfigurationWaitForPrepareOk {
    pub ctx: VrCtx,
    pub prepare_requests: PrepareRequests,
    /// If the primary doesn't receive a new client request in `primary_tick_ms` it sends a commit
    /// message to the backups. `idle_timeout` should be at least twice as large as this value.
    pub tick_ms: u64
});

/// The state where a new replica is added to the group as part of Reconfiguration
///
/// In this state the replica is waiting for a `NewState` msg
state!(ReconfigurationWaitForNewState {
    pub ctx: VrCtx,
    pub idle_timeout: Duration
});

/// The state where a replica is in the process of shutting down
///
/// The replica received a reconfiguration request in its log and it is not in the new
/// configuration.
/// The replica is waiting for a quorum of EpochStarted messages from the new config so that it can
/// shut down.
state!(Leaving {
    pub ctx: VrCtx,
    pub msgs: QuorumTracker<EpochStarted>
});

/// The replica has left and has told the namespace manager to shut it down.
///
/// It doesn't respond to any messages from here on out
state!(Shutdown {});


macro_rules! check_epoch {
    ($ctx:ident, $envelope:ident, $state:ident) => {
        if let Some(epoch) = $envelope.msg.get_epoch() {
            if epoch < $ctx.epoch {
                return next!($state);
            }
            if epoch == $ctx.epoch + 1 {
                return start_reconfiguration($ctx, $envelope, epoch);
            }
            if epoch > $ctx.epoch + 1 {
                // We have been partitioned for one or multiple reconfigurations. We need to catch up.
                // We don't know what the old or new configs are to ask for the latest state. Therefore we wait
                // for a normal message in the new epoch with a 'from' field. We can then begin state transfer
                // using that replica.
                return learn_config($ctx, epoch);
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
    ($ctx:ident, $envelope:ident, $state:ident) => {
        check_epoch!($ctx, $envelope, $state);
        check_view!($ctx, $envelope, $state);
    }
}

/// A new epoch was just seen in the message. If it's a commit message with a `commit_num` equal to
/// the `op`, it means that it's the reconfiguration being committed and this backup is up to
/// date. If it's not an up to date config message then we don't know the new nodes. Let's try to
/// learn those in learn_config before doing state transfer.
///
/// Note that this function will never run on newly created replicas in the new group. New
/// replicas are started knowing the old and new configuration and immediately start state transfer,
/// thus skipping the need to run this function.
///
/// Since this runs only on old nodes
pub fn start_reconfiguration(ctx: &mut VrCtx, envelope: VrEnvelope, epoch: u64) -> Transition {
    ctx.epoch = epoch;
    match envelope.msg {
        VrMsg::Commit {commit_num, ..} if commit_num == ctx.op => {
            ctx.last_received_time = SteadyTime::now();
            // We only handle requests when we have the reconfiguration in the log. Backup commit
            // will play it and we'll learn the new members.
            ctx.view = 0;
            let mut output = ctx.backup_commit(commit_num);
            if ctx.is_leaving() {
                ctx.clear_epoch_started_msgs();
                return next!(leaving, output);
            }
            output.extend_from_slice(&ctx.broadcast_epoch_started(envelope.correlation_id));
            if ctx.is_primary() {
                return next!(primary, output);
            }
            next!(backup, output)
        },

        // TODO: Fix this to do state transfer from the old group
        //
        // We have been partitioned and missed the reconfiguration. At least a quorum of new nodes
        // is normally processing requests, and the leaving replicas may have shut down.
        // We don't yet know what the 'new_config' is though.
        msg => {
            println!("start_reconfiguration msg = {:#?}, ctx.op = {}", msg, ctx.op);
            return learn_config(ctx, epoch);
        }
    }
}

pub fn handle_later_view(ctx: &mut VrCtx, envelope: VrEnvelope, new_view: u64) -> Transition {
    debug!(ctx.logger,
           "handle_later_view: epoch = {}, new_view = {}, from = {}",
           ctx.epoch, new_view, envelope.from);
    match envelope.msg.clone() {
        VrMsg::StartViewChange {from, ..} => {
            debug!(ctx.logger,
                   "handle_later_view: StartViewChange: epoch = {}, new_view = {}, from = {}",
                   ctx.epoch, new_view, from);
            let mut output = ctx.reset_view_change_state(new_view);
            output.extend(ctx.start_view_change());
            ctx.insert_view_change_message(from, envelope.msg);
            if ctx.has_view_change_quorum() {
                let computed_primary = ctx.compute_primary();
                if computed_primary == ctx.pid {
                    let view = ctx.view;
                    output.extend(ctx.reset_view_change_state(view));
                    return next!(wait_for_do_view_change, output)
                }
                output.push(ctx.send_do_view_change(computed_primary));
                return next!(wait_for_start_view, output);
            }
            next!(wait_for_start_view_change, output)
        },
        VrMsg::DoViewChange {from, ..} => {
            debug!(ctx.logger,
                   "handle_later_view: DoViewChange : epoch = {}, new_view = {}, from = {}",
                   ctx.epoch, new_view, from);
            let output = ctx.reset_view_change_state(new_view);
            ctx.insert_view_change_message(from, envelope.msg);
            if ctx.has_view_change_quorum() {
                let output = ctx.become_primary();
                return next!(primary, output);
            }
            next!(wait_for_do_view_change, output)
        },
        VrMsg::StartView{op, log, commit_num, ..} => {
            debug!(ctx.logger, "Start View: DoViewChange : epoch = {}, new_view = {}, from = {}",
                   ctx.epoch, new_view, envelope.from);
            let output = ctx.become_backup(new_view, op, log, commit_num);
            next!(backup, output)
        },
        _ => {
            debug!(ctx.logger,
                   "handle_later_view: Non-ViewChange msg: epoch = {}, new_view = {}, from = {}",
                   ctx.epoch, new_view, envelope.from);
            // We've missed the view change and are likely out of date
            let output = ctx.start_state_transfer_new_view(new_view, envelope.correlation_id);
            next!(state_transfer, output)
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// FSM STATES
///////////////////////////////////////////////////////////////////////////////////////////////////
// Startup states always receive a tick message to kick things off

/// Start this FSM as part of a newly created namespace
pub fn startup_new_namespace(ctx: &mut VrCtx, _: VrEnvelope) -> Transition {
    let output = ctx.reset_and_start_view_change();
    next!(wait_for_start_view_change, output)
}

/// Start this FSM in recovery mode
pub fn startup_recovery(ctx: &mut VrCtx, _: VrEnvelope) -> Transition {
    let output = ctx.start_recovery();
    next!(recovery, output)
}

/// This node was started as part of a reconfiguration. It's a new member to the group.
pub fn startup_reconfiguration(ctx: &mut VrCtx, _: VrEnvelope) -> Transition {
    let output = ctx.start_state_transfer_reconfiguration();
    next!(reconfiguration_wait_for_new_state, output)
}

/// This replica is currently the primary replica operating normally
pub fn primary(ctx: &mut VrCtx, envelope: VrEnvelope) -> Transition {
    handle_common!(ctx, envelope, primary);
    match envelope.msg {
        VrMsg::ClientRequest {..} => {
            debug!(ctx.logger, "primary: got client request");
            let output =  ctx.send_prepare(envelope);
            next!(primary, output)
        },
        VrMsg::PrepareOk {op, ref from, ..} if op > ctx.commit_num => {
            debug!(ctx.logger, "primary: got PrepareOk: op = {}", op);
            if ctx.has_commit_quorum(op, from.clone()) {
                debug!(ctx.logger, "primary: has commit quorum: op = {}", op);
                // We'll never actually commit a Reconfiguration here. That always happens in
                // wait_for_reconfiguration_prepare_ok.
                let output = ctx.primary_commit(op);
                return next!(primary, output);
            }
            next!(primary)
        },
        VrMsg::Tick => {
            if ctx.prepare_requests.expired() {
                debug!(ctx.logger, "primary: prepare expired");
                let output = ctx.reset_and_start_view_change();
                return next!(wait_for_start_view_change, output);
            }
            if ctx.primary_idle_timeout() {
                let output = ctx.broadcast_commit_msg();
                return next!(primary, output);
            }
            next!(primary)
        },
        VrMsg::GetState {op, from, ..} => {
            let output = ctx.send_new_state(op, from, envelope.correlation_id);
            next!(primary, vec![output])
        },
        VrMsg::Recovery {from, nonce} => {
            let output = ctx.send_recovery_response(from, nonce, envelope.correlation_id);
            next!(primary, vec![output])
        },
        VrMsg::Reconfiguration {..} => {
            if let Some(err_envelope) = ctx.validate_reconfig(&envelope) {
                return next!(primary, vec![err_envelope]);
            }
            let output = ctx.send_prepare(envelope);
            next!(wait_for_reconfiguration_prepare_ok, output)
        },
        VrMsg::StartEpoch {..} => {
            let output = ctx.send_epoch_started(envelope);
            next!(primary, output)
        },
        _ => next!(primary)
    }
}

/// This replica is currently a backup operating normally
pub fn backup(ctx: &mut VrCtx, envelope: VrEnvelope) -> Transition {
    handle_common!(ctx, envelope, backup);
    match envelope.msg {
        VrMsg::Prepare {op, commit_num, ref msg, ..} if op == ctx.op + 1 => {
            ctx.last_received_time = SteadyTime::now();
            let prepare_ok_envelope = ctx.send_prepare_ok((**msg).clone(),
                                                          commit_num,
                                                          envelope.correlation_id);
            next!(backup, prepare_ok_envelope)
        },
        VrMsg::Prepare {op, ..} if op > ctx.op + 1 => {
            ctx.last_received_time = SteadyTime::now();
            let output = vec![ctx.send_get_state_to_random_replica(envelope.correlation_id)];
            next!(state_transfer, output)
        },
        VrMsg::Commit {commit_num, ..} if commit_num == ctx.commit_num => {
            ctx.last_received_time = SteadyTime::now();
            next!(backup)
        },
        VrMsg::Commit {commit_num, ..} if commit_num == ctx.op => {
            ctx.last_received_time = SteadyTime::now();
            let output = ctx.backup_commit(commit_num);
            next!(backup, output)
        },
        VrMsg::Commit {commit_num, ..} if commit_num > ctx.op => {
            ctx.last_received_time = SteadyTime::now();
            // Note that a primary cannot have a commit_num smaller than a backup by protocol
            // invariants
            let output = vec![ctx.send_get_state_to_random_replica(envelope.correlation_id)];
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
            let output = vec![ctx.send_new_state(op, from, envelope.correlation_id)];
            next!(backup, output)
        },
        VrMsg::Recovery {from, nonce} => {
            if *ctx.primary.as_ref().unwrap() == from {
                let output = ctx.reset_and_start_view_change();
                return next!(wait_for_start_view_change, output);
            }
            let output = ctx.send_recovery_response(from, nonce, envelope.correlation_id);
            next!(backup, vec![output])
        },
        VrMsg::StartEpoch {..} => {
            let output = ctx.send_epoch_started(envelope);
            next!(backup, output)
        },
        _ => {
            println!("Re-entering backup state: {}", ctx.pid.name);
            next!(backup)
        }
    }
}

/// The first phase of a view change. This replica is waiting for a quorum of StartViewChange
/// messages.
pub fn wait_for_start_view_change(ctx: &mut VrCtx, envelope: VrEnvelope) -> Transition {
    debug!(ctx.logger, "wait_for_start_view_change: epoch = {}, view = {}", ctx.epoch, ctx.view);
    handle_common!(ctx, envelope, wait_for_start_view_change);
    match envelope.msg.clone() {
        VrMsg::StartViewChange{from, ..} => {
            debug!(ctx.logger, "received StartViewChange: epoch = {}, view = {}",
                ctx.epoch, ctx.view);
            ctx.last_received_time = SteadyTime::now();
            ctx.insert_view_change_message(from, envelope.msg);
            if ctx.has_view_change_quorum() {
                let computed_primary = ctx.compute_primary();
                if computed_primary == ctx.pid {
                    let view = ctx.view;
                    let output = ctx.reset_view_change_state(view);
                    return next!(wait_for_do_view_change, output)
                }
                let output = ctx.send_do_view_change(computed_primary);
                return next!(wait_for_start_view, vec![output]);
            }
            next!(wait_for_start_view_change)
        },
        VrMsg::DoViewChange {view, from, ..} => {
            // Another replica got quorum of StartViewChange messages for this view and computed
            // that we are the primary for this view.
            let output = ctx.reset_view_change_state(view);
            ctx.insert_view_change_message(from, envelope.msg);
            if ctx.has_view_change_quorum() {
                let output = ctx.become_primary();
                return next!(primary, output);
            }
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
            let primary = ctx.compute_primary();
            if primary == *from {
                let output = ctx.reset_and_start_view_change();
                return next!(wait_for_start_view_change, output);
            }
            next!(wait_for_start_view_change)
        },
        VrMsg::Prepare {..} => {
            // Another replica was already elected primary for this view.
            let view = ctx.view;
            let output = ctx.start_state_transfer_new_view(view, envelope.correlation_id);
            next!(state_transfer, output)
        },
        VrMsg::Commit {..} => {
            // Another replica was already elected primary for this view.
            let view = ctx.view;
            let output = ctx.start_state_transfer_new_view(view, envelope.correlation_id);
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
            ctx.insert_view_change_message(from, envelope.msg);
            if ctx.has_view_change_quorum() {
                let output = ctx.become_primary();
                return next!(primary, output);
            }
            next!(wait_for_do_view_change)
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
    debug!(ctx.logger, "wait_for_start_view: epoch = {}, view = {}", ctx.epoch, ctx.view);
    handle_common!(ctx, envelope, wait_for_start_view);
    let view = ctx.view;
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
            let output = ctx.start_state_transfer_new_view(view, envelope.correlation_id);
            next!(state_transfer, output)
        },
        VrMsg::Commit {..} => {
            // We missed the StartView message
            let output = ctx.start_state_transfer_new_view(view, envelope.correlation_id);
            next!(state_transfer, output)
        }
        _ => next!(wait_for_start_view)
    }
}


/// When a backup realizes it's behind it enters this state
pub fn state_transfer(ctx: &mut VrCtx, envelope: VrEnvelope) -> Transition {
    handle_common!(ctx, envelope, state_transfer);
    match envelope.msg {
        VrMsg::NewState {..} => {
            let output = ctx.set_from_new_state_msg(envelope.msg);
            next!(backup, output)
        },
        VrMsg::Tick => {
            // If we haven't gotten a NewState in time then re-broadcast GetState
            if ctx.idle_timeout() {
                let output = vec![ctx.send_get_state_to_random_replica(envelope.correlation_id)];
                return next!(state_transfer, output);
            }
            next!(state_transfer)
        },
        _ => next!(state_transfer)
    }
}

/// This is the state of the primary after it has sent a Prepare message containing the client
/// reconfiguration request.
pub fn wait_for_reconfiguration_prepare_ok(ctx: &mut VrCtx, envelope: VrEnvelope) -> Transition {
    handle_common!(ctx, envelope, wait_for_reconfiguration_prepare_ok);
    match envelope.msg {
        VrMsg::PrepareOk {op, ref from, ..} if op == ctx.op => {
            if ctx.has_commit_quorum(op, from.clone()) {
                let output = ctx.primary_commit(op);
                // We committed the reconfiguration request
                if ctx.is_primary() {
                    return next!(primary, output);
                }
                return next!(backup, output)
            }
            next!(wait_for_reconfiguration_prepare_ok)
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

pub fn reconfiguration_wait_for_new_state(ctx: &mut VrCtx, envelope: VrEnvelope) -> Transition {
    handle_common!(ctx, envelope, reconfiguration_wait_for_new_state);
    match envelope.msg {
        VrMsg::NewState {op, ..} if op >= ctx.new_config.op => {
            let mut output = ctx.set_from_new_state_msg(envelope.msg);
            if ctx.is_leaving() {
                ctx.clear_epoch_started_msgs();
                return next!(leaving, output)
            }
            output.extend_from_slice(&ctx.broadcast_epoch_started(envelope.correlation_id));
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
            ctx.commit_recovery().map_or(next!(recovery), |output| next!(backup, output))
        },
        VrMsg::Tick => {
            if ctx.recovery_expired() {
                let output = ctx.start_recovery();
                next!(recovery, output)
            } else {
                next!(recovery)
            }
        },
        _ => next!(recovery)
    }
}


/// A replica is in this state after it has a reconfiguration request in its log and it is not in
/// the new configuration. It is waiting for f' + 1 EpochStarted messages so that it can shutdown.
pub fn leaving(ctx: &mut VrCtx, envelope: VrEnvelope) -> Transition {
    match envelope.msg {
        // TODO: What if epoch > ctx.epoch ?
        VrMsg::EpochStarted {epoch, ref from, ..} if epoch == ctx.epoch => {
            ctx.epoch_started_msgs.insert(from.clone(), envelope.msg.clone());
            if ctx.epoch_started_msgs.has_quorum() {
                ctx.clear_epoch_started_msgs();
                let output = vec![FsmOutput::Announcement(NamespaceMsg::Stop(ctx.pid.clone()),
                                                          ctx.pid.clone())];
                return next!(shutdown, output)
            }
            next!(leaving)
        },
        VrMsg::Tick => {
            if ctx.epoch_started_msgs.is_expired() {
                let output = ctx.broadcast_start_epoch();
                return next!(leaving, output);
            }
            next!(leaving)
        }
        _ => next!(leaving)
    }
}

/// This replica has already told the dispatcher to shut it down. It just waits in this state and
/// doesn't respond to any messages from this point out.
pub fn shutdown(_ctx: &mut VrCtx, _: VrEnvelope) -> Transition {
    next!(shutdown)
}


///////////////////////////////////////////////////////////////////////////////////////////////////
// END OF FSM STATES
///////////////////////////////////////////////////////////////////////////////////////////////////


fn learn_config(_ctx: &mut VrCtx, _epoch: u64) -> Transition {
    unimplemented!();
}
