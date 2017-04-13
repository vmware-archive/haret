// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use rabble::{UserMsg, Error};
use namespace_msg::NamespaceMsg;
use vr::VrMsg;
use admin::{AdminReq, AdminRpy};
use api::ApiRpy;

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
    fn to_bytes(self) -> Vec<u8> {
    }

    fn from_bytes(Vec<u8>) -> Result<Msg, Error> {
    }
}
