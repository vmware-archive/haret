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

/// Generate an impl of Transition<$Msg> for $State
#[macro_export]
macro_rules! handle {
    ($Msg:ty, $State:ty, $Code:block) => {
        #[allow(unused_variables)]
        impl Transition<$Msg> for $State {
            fn next(self,
                    msg: $Msg,
                    from: Pid,
                    cid: CorrelationId,
                    output: &mut Vec<FsmOutput>) -> VrStates $Code
        }
    }
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
    ReconfigurationWaitForNewState(ReconfigurationWaitForNewState),
    Leaving(Leaving),
    Shutdown(Shutdown)
}

impl VrStates {
    fn next(self,
            msg: VrMsg,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<FsmOutput>) -> VrStates
    {
        match self {
            VrStates::Primary(s) => next!(s, msg, from, cid, output),
            VrStates::Backup(s) => next!(s, msg, from, cid, output),
            VrStates::WaitForNewState(s) => next!(s, msg, from, cid, output)
        }
    }
}

/// The primary state of the VR protocol operating in normal mode
state!(Primary {
    pub ctx: VrCtx,
    pub prepare_requests: PrepareRequests,
    /// If the primary doesn't receive a new client request in `primary_tick_ms` it sends a commit
    /// message to the backups. `idle_timeout` should be at least twice as large as this value.
    pub tick_ms: u64,
    pub reconfiguration_in_progress: bool
});

/// The backup state of the VR protocol operating in normal mode
state!(Backup {
    pub ctx: VrCtx,
    pub primary: Pid
});

/// When a backup realizes it's behind it enters state transfer
///
/// In this state, the backup is waiting for a `NewState` message
state!(WaitForNewState {
    pub ctx: VrCtx
});

/// The part of the view change state in the VR protocol state machine where a replica is waiting
/// for a quorum of `StartViewChange` messages.
state!(WaitForStartViewChange {
    pub ctx: VrCtx,
    pub msgs: QuorumTracker<StartViewChange>
});

/// The part of the view change state in the VR protocol state machine the proposed primary is
/// waiting for a quorum of `DoViewChange` messages.
state!(WaitForDoViewChange {
    pub ctx: VrCtx,
    pub state: DoViewChangeState
});

/// The part of the view change state in the VR protocol state machine where a replica is waiting
/// for a `StartView` message from the new primary. It has already sent a `DoViewChange` to the
/// proposed primary for this view.
state!(WaitForStartView {
    pub ctx: VrCtx,
    pub primary: Pid
});

/// The recovery state of the VR Protocol where a replica is recovering data from a quorum of
/// replicas
state!(Recovery {
    pub ctx: VrCtx,
    pub state: RecoveryState
});

/// The state where a new replica is added to the group as part of Reconfiguration
///
/// In this state the replica is waiting for a `NewState` msg
state!(ReconfigurationWaitForNewState {
    pub ctx: VrCtx
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

