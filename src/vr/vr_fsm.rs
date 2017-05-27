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
            output: &mut Vec<FsmOutput>) -> VrStates {
        self.into()
    };
}

pub trait State {
    fn ctx(self) -> VrCtx;
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

        impl State for $struct_name {
            fn ctx(self) -> VrCtx {
                self.ctx
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
            VrStates::Primary(s) => match_msg(s, msg, from, cid, output),
            VrStates::Backup(s) => match_msg(s, msg, from, cid, output),
            VrStates::WaitForNewState(s) => match_msg(s, msg, from, cid, output),
            VrStates::WaitForStartViewChange(s) => match_msg(s, msg, from, cid, output),
            VrStates::WaitForDoViewChange(s) => match_msg(s, msg, from, cid, output),
            VrStates::WaitForStartView(s) => match_msg(s, msg, from, cid, output),
            VrStates::Recovery(s) => match_msg(s, msg, from, cid, output),
            VrStates::Reconfiguration(s) => match_msg(s, msg, from, cid, output),
            VrStates::Leaving(s) => match_msg(s, msg, from, cid, output),
            VrStates::Shutdown(s) => match_msg(s, msg, from, cid, output)
        }
    }

    fn match_msg<S: State>(state: S,
                           msg: VrMsg,
                           from: Pid,
                           cid: CorrelationId,
                           output: &mut Vec<FsmOutput>) -> VrStates {
        match msg {
            VrMsg::Tick => state.handle(Tick, from, cid, output),
            VrMsg::ClientRequest(msg) => state.handle(msg, from, cid, output),
            VrMsg::Reconfiguration(msg) => state.handle(msg, from, cid, output),
            VrMsg::ClientReply(msg) => state.handle(msg, from, cid, output),
            VrMsg::StartViewChange(msg) => state.handle(msg, from, cid, output),
            VrMsg::DoViewChange(msg) => state.handle(msg, from, cid, output),
            VrMsg::StartView(msg) => state.handle(msg, from, cid, output),
            VrMsg::Prepare(msg) => state.handle(msg, from, cid, output),
            VrMsg::PrepareOk(msg) => state.handle(msg, from, cid, output),
            VrMsg::Commit(msg) => state.handle(msg, from, cid, output)
            VrMsg::GetState(msg) => state.handle(msg, from, cid, output),
            VrMsg::NewState(msg) => state.handle(msg, from, cid, output),
            VrMsg::Recovery(msg) => state.handle(msg, from, cid, output),
            VrMsg::RecoveryResponse(msg) => state.handle(msg, from, cid, output),
            VrMsg::StartEpoch(msg) => state.handle(msg, from, cid, output),
            VrMsg::EpochStarted(msg) => state.handle(msg, from, cid, output)
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

