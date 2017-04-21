// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use protobuf::{Message, MessageStatic, parse_from_bytes};
use rabble::{UserMsg, Result, ErrorKind};
use namespace_msg::NamespaceMsg;
use vr::VrMsg;
use admin::{AdminReq, AdminRpy};
use api::ApiRpy;
use pb_msg;

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    Vr(VrMsg),
    Namespace(NamespaceMsg),
    AdminReq(AdminReq),
    AdminRpy(AdminRpy),
    ApiRpy(ApiRpy),
    Error(String)
}

impl UserMsg for Msg {
    fn to_bytes(self) -> Result<Vec<u8>> {
        let mut msg = pb_msg::Msg::new();
        match self {
            Msg::Vr(vrmsg) => msg.set_vr(vrmsg.into()),
            Msg::NamespaceMsg(namespace_msg) => msg.set_namespace(namespace_msg.into()),
            Msg::AdminReq(req) => msg.set_admin_req(req.into()),
            Msg::AdminRpy(rpy) => msg.set_admin_rpy(rpy.into()),
            Msg::ApiRpy(rpy) => msg.set_api_rpy(rpy.into()),
            Msg::Error(s) => msg.set_error(s)
        }
        Ok(msg.write_to_bytes()?)
    }

    fn from_bytes(buf: Vec<u8>) -> Result<Msg> {
        let msg: Msg = parse_from_bytes(&buf[..])?;
        if msg.has_vr() {
            return Ok(Msg::Vr(msg.take_vr().try_into()?));
        }
        if msg.has_namespace() {
            return Ok(Msg::NamespaceMsg(msg.take_namespace().try_into()?));
        }
        if msg.has_admin_req() {
            return Ok(Msg::AdminReq(msg.take_admin_req().try_into()?));
        }
        if msg.has_admin_rpy() {
            return Ok(Msg::AdminRpy(msg.take_admin_rpy().try_into()?));
        }
        if msg.has_api_rpy() {
            return Ok(Msg::ApiRpy(msg.take_api_rpy().try_into()?));
        }
        if msg.has_error() {
            return Ok(Msg::Error(msg.take_error()));
        }

        Err(ErrorKind::ProtobufDecodeError("Unknown Msg").into())
    }
}
