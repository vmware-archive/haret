// Copyright 2017 VMware, Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rabble::Pid;
use super::replica::VersionedReplicas;
use super::vr_ctx::VrCtx;

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
