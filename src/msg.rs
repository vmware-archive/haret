// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use rabble::{UserMsg, Result};
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

impl trait UserMsg for Msg {
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

    fn from_bytes(Vec<u8>) -> Result<Msg> {
    }
}
