// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use uuid::Uuid;
use rabble::{Pid, Error};
use super::vr_api_messages::{VrApiReq, VrApiRsp};
use super::replica::VersionedReplicas;
use pb_msg;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VrMsg {
    /// A message that drives the state of the fsm during periods of inactivity
    Tick,
    ClientRequest {
        op: VrApiReq,
        client_id: String,
        request_num: u64
    },
    Reconfiguration {
        client_req_num: u64,
        epoch: u64,
        replicas: Vec<Pid>
    },
    ClientReply {
        epoch: u64,
        view: u64,
        request_num: u64,
        value: VrApiRsp
    },
    StartViewChange {
        epoch: u64,
        view: u64,
        op: u64,
        from: Pid
    },
    DoViewChange {
        epoch: u64,
        view: u64,
        op: u64,
        from: Pid,
        last_normal_view: u64,
        log: Vec<VrMsg>,
        commit_num: u64
    },
    StartView {
        epoch: u64,
        view: u64,
        op: u64,
        log: Vec<VrMsg>,
        commit_num: u64
    },
    Prepare {
        epoch: u64,
        view: u64,
        op: u64,
        commit_num: u64,
        msg: Box<VrMsg>
    },
    PrepareOk {
        epoch: u64,
        view: u64,
        op: u64,
        from: Pid
    },
    Commit {
        epoch: u64,
        view: u64,
        commit_num: u64
    },
    GetState {
        epoch: u64,
        view: u64,
        op: u64,
        from: Pid
    },
    NewState {
        epoch: u64,
        view: u64,
        op: u64,
        primary: Option<Pid>,
        commit_num: u64,
        log_tail: Vec<VrMsg>,
    },
    Recovery {
        from: Pid,
        nonce: Uuid
    },
    RecoveryResponse {
        epoch: u64,
        view: u64,
        nonce: Uuid,
        from: Pid,
        // The following fields are only valid when sent by the Primary
        op: Option<u64>,
        commit_num: Option<u64>,
        log: Option<Vec<VrMsg>>
    },
    StartEpoch {
        epoch: u64,
        op: u64,
        old_config: VersionedReplicas,
        new_config: VersionedReplicas
    },
    EpochStarted {
        epoch: u64,
        from: Pid
    }
}

impl VrMsg {
    // This is only used for filtering messages in vr_fsm. However we don't ever want to filter
    // client requests from the primary, so we exclude `VrMsg::Reconfiguration`
    pub fn get_epoch(&self) -> Option<u64> {
        match *self {
            VrMsg::ClientReply {epoch, ..} => Some(epoch),
            VrMsg::StartViewChange {epoch, ..} => Some(epoch),
            VrMsg::DoViewChange {epoch, ..} => Some(epoch),
            VrMsg::StartView {epoch, ..} => Some(epoch),
            VrMsg::Prepare {epoch, ..} => Some(epoch),
            VrMsg::PrepareOk {epoch, ..} => Some(epoch),
            VrMsg::Commit {epoch, ..} => Some(epoch),
            VrMsg::GetState {epoch, ..} => Some(epoch),
            VrMsg::NewState {epoch, ..} => Some(epoch),
            VrMsg::RecoveryResponse {epoch, ..} => Some(epoch),
            VrMsg::StartEpoch {epoch, ..} => Some(epoch),
            VrMsg::EpochStarted {epoch, ..} => Some(epoch),
            _ => None
        }
    }

    pub fn get_view(&self) -> Option<u64> {
        match *self {
            VrMsg::ClientReply {view, ..} => Some(view),
            VrMsg::StartViewChange {view, ..} => Some(view),
            VrMsg::DoViewChange {view, ..} => Some(view),
            VrMsg::StartView {view, ..} => Some(view),
            VrMsg::Prepare {view, ..} => Some(view),
            VrMsg::PrepareOk {view, ..} => Some(view),
            VrMsg::Commit {view, ..} => Some(view),
            VrMsg::GetState {view, ..} => Some(view),
            VrMsg::NewState {view, ..} => Some(view),
            VrMsg::RecoveryResponse {view, ..} => Some(view),
            _ => None
        }
    }
}

impl From<VrMsg> for pb_msg::VrMsg {
    fn from(vrmsg: VrMsg) -> pb_msg::VrMsg {
        let mut pb_vrmsg = pb_msg::VrMsg::new();
        match vrmsg {
            VrMsg::Tick => pb_vrmsg.set_tick(true),
            VrMsg::ClientRequest {op, client_id, request_num} => {
                let mut client_req = pb_msg::ClientRequest::new();
                client_req.set_op(op.into());
                client_req.set_client_id(client_id);
                client_req.set_request_num(request_num);
                pb_vrmsg.set_client_request(client_req)
            },
            VrMsg::Reconfiguration {client_req_num, epoch, replicas} => {
                let mut reconfig = pb_msg::Reconfiguration::new();
                reconfig.set_client_request_num(client_req_num);
                reconfig.set_epoch(epoch);
                reconfig.set_replicas(replicas.into_iter().map(|p| p.into()).collect());
                pb_vrmsg.set_reconfiguration(reconfig)
            },
            VrMsg::ClientReply {epoch, view, request_num, value} => {
                let mut reply = pb_msg::ClientReply::new();
                reply.set_epoch(epoch);
                reply.set_view(view);
                reply.set_request_num(request_num);
                reply.set_value(value.into());
                pb_vrmsg.set_client_reply(reply);
            },
            VrMsg::StartViewChange {epoch, view, op, from} => {
                let mut msg = pb_msg::StartViewChange::new();
                msg.set_epoch(epoch);
                msg.set_view(view);
                msg.set_op(op);
                msg.set_from(from.into());
                pb_vrmsg.set_start_view_change(msg);
            },
            VrMsg::DoViewChange {epoch, view, op, from, last_normal_view, log, commit_num} => {
                let mut msg = pb_msg::DoViewChange::new();
                msg.set_epoch(epoch);
                msg.set_view(view);
                msg.set_op(op);
                msg.set_from(from.into());
                msg.set_last_normal_view(last_normal_view);
                msg.set_log(log.into_iter().map(|vrmsg| vrmsg.into()).collect());
                msg.set_commit_num(commit_num);
                pb_vrmsg.set_do_view_change(msg);
            },
            VrMsg::StartView {epoch, view, op, log, commit_num} => {
                let mut msg = pb_msg::StartView::new();
                msg.set_epoch(epoch);
                msg.set_view(view);
                msg.set_op(op);
                msg.set_log(log.into_iter().map(|vrmsg| vrmsg.into()).collect());
                msg.set_commit_num(commit_num);
                pb_vrmsg.set_start_view(msg);
            },
            VrMsg::Prepare {epoch, view, op, commit_num, msg} => {
                let mut msg = pb_msg::Prepare::new();
                msg.set_epoch(epoch);
                msg.set_view(view);
                msg.set_op(op);
                msg.set_commit_num(commit_num);
                msg.set_msg((*msg).into());
                pb_vrmsg.set_start_view(msg);
            },
            VrMsg::PrepareOk {epoch, view, op, from} => {
                let mut msg = pb_msg::PrepareOk::new();
                msg.set_epoch(epoch);
                msg.set_view(view);
                msg.set_op(op);
                msg.set_from(from.into());
                pb_vrmsg.set_prepare_ok(msg);
            },
            VrMsg::Commit {epoch, view, commit_num} => {
                let mut msg = pb_msg::Commit::new();
                msg.set_epoch(epoch);
                msg.set_view(view);
                msg.set_commit_num(commit_num);
                pb_vrmsg.set_commit(msg);
            },
            VrMsg::GetState {epoch, view, op, from} => {
                let mut msg = pb_msg::GetState::new();
                msg.set_epoch(epoch);
                msg.set_view(view);
                msg.set_op(op);
                msg.set_from(from.into());
                pb_vrmsg.set_get_state(msg);
            },
            VrMsg::NewState {epoch, view, op, primary, commit_num, log_tail} => {
                let mut msg = pb_msg::NewState::new();
                msg.set_epoch(epoch);
                msg.set_view(view);
                msg.set_op(op);
                if let Some(primary) = primary {
                    msg.set_primary(primary);
                }
                msg.set_commit_num(commit_num);
                msg.set_log_tail(log_tail.into_iter().map(|vrmsg| vrmsg.into()).collect());
                pb_vrmsg.set_new_state(msg);
            },
            VrMsg::Recovery {from, nonce} => {
                let mut msg = pb_msg::Recovery::new();
                msg.set_from(from.into());
                msg.set_nonce(nonce.to_string());
                pb_vrmsg.set_recovery(msg);
            },
            VrMsg::RecoveryResponse {epoch, view, nonce, from, op, commit_num, log} => {
                let mut msg = pb_msg::RecoveryResponse::new();
                msg.set_epoch(epoch);
                msg.set_view(view);
                msg.set_nonce(nonce.to_string());
                msg.set_from(from.into());
                if let Some(op) = op {
                    msg.set_op(op);
                    msg.set_commit_num(commit_num.unwrap());
                    msg.set_log(log.unwrap().into_iter().map(|vrmsg| vrmsg.nto()).collect());
                }
                pb_vrmsg.set_recovery_response(msg);
            },
            VrMsg::StartEpoch {epoch, op, old_config, new_config} => {
                let mut msg = pb_msg::StartEpoch::new();
                msg.set_epoch(epoch);
                msg.set_op(op);
                msg.set_old_config(old_config.into());
                msg.set_new_config(new_config.into());
                pb_vrmsg.set_start_epoch(msg);
            },
            VrMsg::EpochStarted {epoch, from} => {
                let mut msg = pb_msg::EpochStarted::new();
                msg.set_epoch(epoch);
                msg.set_from(from.into());
                pb_vrmsg.set_epoch_started(msg);
            }
        }
        pb_vrmsg
    }
}

impl TryFrom<pb_msg::VrMsg> for VrMsg {
    type Error = Error;
    fn try_from(pbmsg: pbmsg::VrMsg) -> Result<VrMsg, Error> {
        if pbmsg.has_tick() {
            return Ok(VrMsg::Tick);
        }
        if pbmsg.has_client_request() {
            let client_req = pbmsg.take_client_request();
            return Ok(VrMsg::ClientRequest {
                op: client_req.take_op().try_into()?,
                client_id: client_req.take_client_id(),
                request_num: client_req.get_request_num()
            });
        },
        if pbmsg.has_reconfiguration() {
            let reconfig = pbmsg.take_reconfiguration();
            return Ok(VrMsg::Reconfiguration {
                client_req_num: reconfig.get_client_req_num(),
                epoch: reconfig.get_epoch(),
                replicas: reconfig.take_replicas().into_iter().map(|p| p.into()).collect()
            });
        },
        if pbmsg.has_client_reply() {
            let client_rpy = pbmsg.take_client_reply();
            return Ok(VrMsg::ClientReply {
                epoch: client_rpy.get_epoch(),
                view: client_rpy.get_view(),
                request_num: client_rpy.get_request_num(),
                value: client_rpy.take_value().try_into()?
            });
        }
        if pbmsg.has_start_view_change() {
            let msg = pbmsg.take_start_view_change();
            return Ok(VrMsg::StartViewChange {
                epoch: msg.get_epoch(),
                view: msg.get_view(),
                op: msg.get_op(),
                from: msg.take_from().into()
            });
        }
        if pbmsg.has_do_view_change() {
            let msg = pbmsg.take_do_view_change();
            return Ok(VrMsg::DoViewChange {
                epoch: msg.get_epoch(),
                view: msg.get_view(),
                op: msg.get_op(),
                from: msg.take_pid().into(),
                last_normal_view: msg.get_last_normal_view(),
                log: msg.take_log().into_iter().map(|vrmsg| vrmsg.into()).collect(),
                commit_num: msg.get_commit_num()
            });
        }
        if pbmsg.has_start_view() {
            let msg = pbmsg.take_start_view();
            return Ok(VrMsg::StartView {
                epoch: msg.get_epoch(),
                view: msg.get_view(),
                op: msg.get_op(),
                log: msg.take_log().into_iter().map(|vrmsg| vrmsg.into()).collect(),
                commit_num: msg.get_commit_num()
            });
        }
        if pbmsg.has_prepare() {
            let msg = pbmsg.take_prepare();
            return Ok(VrMsg::Prepare {
                epoch: msg.get_epoch(),
                view: msg.get_view(),
                op: msg.get_op(),
                commit_num: msg.get_commit_num(),
                msg: Box::new(msg.take_msg().try_into()?)
            });
        }
        if pbmsg.has_prepare_ok() {
            let msg = pbmsg.take_prepare_ok();
            return Ok(VrMsg::PrepareOk {
                epoch: msg.get_epoch(),
                view: msg.get_view(),
                op: msg.get_op(),
                from: msg.take_from().into()
            });
        }
        if pbmsg.has_commit() {
            let msg = pbmsg.take_commit();
            return Ok(VrMsg::Commit {
                epoch: msg.get_epoch(),
                view: msg.get_view(),
                commit_num: msg.get_commit_num()
            });
        }
        if pbmsg.has_get_state() {
            let msg = pbmsg.take_get_state();
            return Ok(VrMsg::GetState {
                epoch: msg.get_epoch(),
                view: msg.get_view(),
                op: msg.get_op(),
                from: msg.take_from().into()
            });
        }
        if pbmsg.has_new_state() {
            let msg = pbmsg.take_new_state();
            let primary = if msg.has_primary() {
                Some(msg.take_primary().into());
            } else {
                None
            };
            return Ok(VrMsg::NewState {
                epoch: msg.get_epoch(),
                view: msg.get_view(),
                op: msg.get_op(),
                primary: primary,
                commit_num: msg.get_commit_num(),
                log_tail: msg.take_log_tail().into_iter().map(|vrmsg| vrmsg.into()).collect()
            });
        }
        if pbmsg.has_recovery() {
            let msg = pbmsg.take_recovery();
            return Ok(VrMsg::Recovery {
                from: msg.take_from().into(),
                nonce: Uuid::parse_str(&msg.take_nonce())?
            });
        }
        if pbmsg.has_recovery_response() {
            let msg = pbmsg.take_recovery_response();
            // Assume that if we have one of the fields we have all of them
            let (op, commit_num, log) = if msg.has_op() {
                (Some(msg.get_op()),
                 Some(msg.get_commit_num()),
                 Some(msg.take_log().into_iter().map(|vrmsg| vrmsg.into()).collect()))
            } else {
                (None, None, None)
            };
            return Ok(VrMsg::RecoveryResponse {
                epoch: msg.get_epoch(),
                view: msg.get_view(),
                nonce: Uuid::parse_str(&msg.take_nonce())?,
                from: msg.take_pid().into(),
                op: op,
                commit_num: commit_num,
                log: log
            });
        }
        if pbmsg.has_start_epoch() {
            let msg = pbmsg.take_start_epoch();
            Ok(VrMsg::StartEpoch {
                epoch: msg.get_epoch(),
                op: msg.get_op(),
                old_config: msg.take_old_config().into(),
                new_config: msg.take_new_config().into()
            });
        }
        if pbmsg.has_epoch_started() {
            let msg = pbmsg.take_epoch_started();
            Ok(VrMsg::EpochStarted {
                epoch: msg.get_epoch(),
                from: msg.take_pid().into()
            });
        }

        Err(ErrorKind::ProtobufDecodeError("Unknown Message received").into())
    }
}
