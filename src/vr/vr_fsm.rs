// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use time::SteadyTime;
use namespace_msg::NamespaceMsg;
use super::vrmsg::VrMsg;
use super::vr_ctx::VrCtx;
use super::vr_envelope::VrEnvelope;
use std::convert::From;

pub trait Transition<Msg> {
    fn next(self,
            msg: Msg,
            from: Pid,
            correlation_id: CorrelationId,
            output: &mut Vec<Envelope>) -> VrState
    {
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
        impl Transition<vr_msg::$Msg> for $State {
            fn next(self,
                    msg: vr_msg::$Msg,
                    from: Pid,
                    cid: CorrelationId,
                    output: &mut Vec<Envelope>) -> VrState $Code
        }
    }
}

/// Generate a state struct: `$struct_name` from a set of fields
///
/// Generate `impl From<$struct_name> for VrState`
macro_rules! state {
    ($struct_name:ident {
        $( $field:ident: $ty:ident ),*
    }) => {
        #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
        pub struct $struct_name {
            $( pub $field: $ty ),*
        }

        impl From<$struct_name> for VrState {
            fn from(msg: $struct_name) -> VrState {
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

macro_rules! up_to_date {
    ($Self:ident, $From:ident, $Msg:ident, $Cid:ident, $Output:ident) => {
        if $Msg.epoch < $Self.ctx.epoch {
            return $Self.into();
        }
        if $Msg.epoch > $Self.ctx.epoch {
            // Reconfiguration has already occurred since a primary in the new epoch is accepting
            // requests and sending Prepare messages.
            return StateTransfer::start_epoch($Self, $From, $Cid, $Msg.epoch, $Msg.view, $Output);
        }
        if $Msg.view < $Self.ctx.view {
            return $Self.into();
        }
        if $Msg.view > $Self.ctx.view {
            return StateTransfer::start_view($Self, $Msg.view, $Cid, $Output);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum VrState {
    Primary(Primary),
    Backup(Backup),
    StateTransfer(StateTransfer),
    StartViewChange(StartViewChange),
    DoViewChange(DoViewChange),
    StartView(StartView),
    Recovery(Recovery),
    Reconfiguration(Reconfiguration),
    Leaving(Leaving),
    Shutdown(Shutdown)
}

impl VrState {
    fn next(self,
            msg: VrMsg,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<Envelope>) -> VrState
    {
        match self {
            VrState::Primary(s) => match_msg(s, msg, from, cid, output),
            VrState::Backup(s) => match_msg(s, msg, from, cid, output),
            VrState::StateTransfer(s) => match_msg(s, msg, from, cid, output),
            VrState::StartViewChange(s) => match_msg(s, msg, from, cid, output),
            VrState::DoViewChange(s) => match_msg(s, msg, from, cid, output),
            VrState::StartView(s) => match_msg(s, msg, from, cid, output),
            VrState::Recovery(s) => match_msg(s, msg, from, cid, output),
            VrState::Reconfiguration(s) => match_msg(s, msg, from, cid, output),
            VrState::Leaving(s) => match_msg(s, msg, from, cid, output),
            VrState::Shutdown(s) => match_msg(s, msg, from, cid, output)
        }
    }

    fn match_msg<S: State>(state: S,
                           msg: VrMsg,
                           from: Pid,
                           cid: CorrelationId,
                           output: &mut Vec<Envelope>) -> VrState {
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


