// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use rabble::{Pid, CorrelationId, Envelope};
use msg::Msg;
use vr::states::*;
use super::vr_msg::VrMsg;
use super::vr_ctx::VrCtx;

/// Generate a state struct: `$struct_name` from a set of fields
///
/// Generate `impl From<$struct_name> for VrState`
macro_rules! state {
    ($struct_name:ident {
        $( $field:ident: $ty:ty),*
    }) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $struct_name {
            $( pub $field: $ty ),*
        }

        impl From<$struct_name> for VrState {
            fn from(state: $struct_name) -> VrState {
                VrState::$struct_name(state)
            }
        }

        impl State for $struct_name {
            fn ctx(self) -> VrCtx {
                self.ctx
            }

            fn borrow_ctx(&self) -> &VrCtx {
                &self.ctx
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

/// This represents a transition from one state to another via a received message
pub trait Transition {
    fn handle(self,
            msg: VrMsg,
            from: Pid,
            correlation_id: CorrelationId,
            output: &mut Vec<Envelope<Msg>>) -> VrState;
}

pub trait State: Into<VrState> {
    fn ctx(self) -> VrCtx;
    fn borrow_ctx(&self) -> &VrCtx;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn next(self,
            msg: VrMsg,
            from: Pid,
            cid: CorrelationId,
            output: &mut Vec<Envelope<Msg>>) -> VrState
    {
        match self {
            VrState::Primary(s) => s.handle(msg, from, cid, output),
            VrState::Backup(s) => s.handle(msg, from, cid, output),
            VrState::StateTransfer(s) => s.handle(msg, from, cid, output),
            VrState::StartViewChange(s) => s.handle(msg, from, cid, output),
            VrState::DoViewChange(s) => s.handle(msg, from, cid, output),
            VrState::StartView(s) => s.handle(msg, from, cid, output),
            VrState::Recovery(s) => s.handle(msg, from, cid, output),
            VrState::Reconfiguration(s) => s.handle(msg, from, cid, output),
            VrState::Leaving(s) => s.handle(msg, from, cid, output),
            VrState::Shutdown(s) => s.into()
        }
    }

    pub fn ctx(&self) -> &VrCtx {
        match *self {
            VrState::Primary(ref s) => &s.ctx,
            VrState::Backup(ref s) => &s.ctx,
            VrState::StateTransfer(ref s) => &s.ctx,
            VrState::StartViewChange(ref s) => &s.ctx,
            VrState::DoViewChange(ref s) => &s.ctx,
            VrState::StartView(ref s) => &s.ctx,
            VrState::Recovery(ref s) => &s.ctx,
            VrState::Reconfiguration(ref s) => &s.ctx,
            VrState::Leaving(ref s) => &s.ctx,
            VrState::Shutdown(ref s) => &s.ctx
        }
    }
}
