// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use namespace_msg::NamespaceMsg;
use vr::VrMsg;
use admin::{AdminReq, AdminRpy};
use api::ApiRpy;
use disk_msgs::{DiskReq, DiskRpy};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Msg {
    Vr(VrMsg),
    Namespace(NamespaceMsg),
    AdminReq(AdminReq),
    AdminRpy(AdminRpy),
    ApiRpy(ApiRpy),
    DiskReq(DiskReq),
    DiskRpy(DiskRpy),
    Error(String)
}
