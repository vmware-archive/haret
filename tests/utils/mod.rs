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

#[macro_use]
mod macros;

pub mod arbitrary;

#[allow(dead_code)]
pub mod vr_invariants;

#[allow(dead_code)]
pub mod op_invariants;

#[allow(dead_code)]
pub mod scheduler;

#[allow(dead_code)]
#[allow(unused_must_use)] // Unnecessary warnings when using check!
pub mod vr_fsm_constraints;
