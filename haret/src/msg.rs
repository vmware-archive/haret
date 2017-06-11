// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use namespace_msg::NamespaceMsg;
use vr::VrMsg;
use admin::{AdminReq, AdminRpy};
use api::ApiRpy;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Msg {
    Vr(VrMsg),
    Namespace(NamespaceMsg),
    AdminReq(AdminReq),
    AdminRpy(AdminRpy),
    ApiRpy(ApiRpy),
    Error(String)
}
