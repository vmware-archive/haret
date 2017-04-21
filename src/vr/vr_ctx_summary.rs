// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use rabble::Pid;
use super::replica::VersionedReplicas;
use super::vr_ctx::VrCtx;
use pb_msg;

/// This is all the state about a `VrCtx` that can be retrieved and sent over the wire to admin
/// clients.. There is a lot of information maintained in a `VrCtx` that is expensive to transmit,
/// such as the log. In a production system, shipping this for admin requests would be unnecessarily
/// wasteful. All important details of a replica will instead be populated inside a VrCtxSummary
/// instead.
#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct VrCtxSummary {
    pub state: String,
    pub pid: Pid,
    pub primary: Option<Pid>,
    pub epoch: u64,
    pub view: u64,
    pub op: u64,
    pub commit_num: u64,
    pub last_received_time: String,
    pub last_normal_view: u64,
    pub old_config: VersionedReplicas,
    pub new_config: VersionedReplicas
}

impl VrCtxSummary {
    pub fn new(state: &'static str, ctx: &VrCtx) -> VrCtxSummary {
        VrCtxSummary {
            state: state.to_string(),
            pid: ctx.pid.clone(),
            primary: ctx.primary.clone(),
            epoch: ctx.epoch,
            view: ctx.view,
            op: ctx.op,
            commit_num: ctx.commit_num,
            last_received_time: ctx.last_received_time.to_string(),
            last_normal_view: ctx.last_normal_view,
            old_config: ctx.old_config.clone(),
            new_config: ctx.new_config.clone()
        }
    }
}

impl From<VrCtxSummary> for pb_msg::VrCtxSummary {
    fn from(ctx: VrCtxSummary) -> pb_msg::VrCtxSummary {
        let mut msg = pb_msg::VrCtxSummary::new();
        msg.set_state(ctx.state);
        msg.set_pid(ctx.pid.into());

        let pb_primary = pb_msg::Primary::new();
        if let Some(primary) = ctx.primary {
            pb_primary.set_primary(primary.into());
        }
        msg.set_primary(pb_primary);

        msg.set_epoch(ctx.epoch);
        msg.set_view(ctx.view);
        msg.set_op(ctx.op);
        msg.set_commit_num(ctx.commit_num);
        msg.set_last_received_time(ctx.last_received_time);
        msg.set_last_normal_view(ctx.last_normal_view);
        msg.set_old_config(ctx.old_config.into());
        msg.set_new_config(ctx.new_config.into());
        msg
    }
}

impl From<pb_msg::VrCtxSummary> for VrCtxSummary {
    fn from(msg: pb_msg::VrCtxSummary) -> VrCtxSummary {
        let pb_primary = msg.take_primary();
        let primary = if pb_primary.has_primary() {
            Some(pb_primary.take_primary().into())
        } else {
            None
        };
        VrCtxSummary {
            state: msg.take_state(),
            pid: msg.take_pid().into(),
            primary: primary,
            epoch: msg.get_epoch(),
            view: msg.get_view(),
            op: msg.get_op(),
            commit_num: msg.get_commit_num(),
            last_received_time: msg.take_last_received_time(),
            last_normal_view: msg.get_last_normal_view(),
            old_config: msg.take_old_config().into(),
            new_config: msg.take_new_config().into()
        }
    }
}
