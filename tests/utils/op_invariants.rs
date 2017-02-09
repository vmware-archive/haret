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

//! This module contains test functions for specific API operations. It's intended to be general
//! enough to use for multiple tests.

use super::scheduler::Scheduler;
use haret::vr::{VrMsg, VrCtx, VrEnvelope, VrApiReq, VrApiRsp, VrApiError, TreeOp, TreeOpResult, NodeType};
use vertree::{self, Reply, Value};

pub fn assert_create_response(scheduler: &Scheduler,
                              request: VrMsg,
                              reply: VrEnvelope) -> Result<(), String>
{
    let (request_num, api_req, api_rsp) = try!(match_client_reply(request, reply));
    let (path, ty) = if let VrApiReq::TreeOp(TreeOp::CreateNode {path, ty}) = api_req {
        (path, ty)
    } else {
        fail!()
    };

    match api_rsp {
        VrApiRsp::Ok => {
            assert_successful_create(scheduler, path, request_num, ty)
        },
        VrApiRsp::Error(VrApiError::AlreadyExists(_)) => {
            assert_successful_create(scheduler, path, request_num, ty)
        },
        VrApiRsp::Error(VrApiError::PathMustEndInDirectory(_)) => {
            Ok(())
        },
        e => {
            println!("e = {:?}", e);
            fail!()
        }
        //_ => fail!()
    }
}

/// Assertions for puts that aren't CAS
pub fn assert_put_response(scheduler: &Scheduler,
                           request: VrMsg,
                           reply: VrEnvelope) -> Result<(), String>
{
    let (request_num, api_req, api_rsp) = try!(match_client_reply(request, reply));
    let (path, data) =
        if let VrApiReq::TreeOp(TreeOp::BlobPut {path, val, ..}) = api_req {
            (path, val)
        } else {
            fail!()
        };

    match api_rsp {
        VrApiRsp::TreeOpResult(TreeOpResult::Ok(_)) => {
            assert_successful_put_or_get(scheduler, path, request_num, data)
        },
        VrApiRsp::Error(VrApiError::AlreadyExists(_)) => Ok(()),
        VrApiRsp::Error(VrApiError::PathMustEndInDirectory(_)) => Ok(()),
        VrApiRsp::Error(VrApiError::WrongType(_, ty)) => safe_assert_eq!(ty, NodeType::Directory),
        VrApiRsp::Error(VrApiError::DoesNotExist(_)) => {
            assert_element_not_found_primary(scheduler, path)
        },
        e => {
            println!("put unhandled error = {:?}", e);
            fail!()
        }
    }
}

pub fn assert_get_response(scheduler: &Scheduler,
                           request: VrMsg,
                           reply: VrEnvelope) -> Result<(), String>
{
    let (request_num, api_req, api_rsp) = try!(match_client_reply(request, reply));
    let path = if let VrApiReq::TreeOp(TreeOp::BlobGet {path, ..}) = api_req {
        path
    } else {
        fail!()
    };

    match api_rsp {
        VrApiRsp::TreeOpResult(TreeOpResult::Blob(data, _)) => {
            assert_successful_put_or_get(scheduler, path, request_num, data)
        },
        VrApiRsp::Error(VrApiError::AlreadyExists(_)) => Ok(()),
        VrApiRsp::Error(VrApiError::PathMustEndInDirectory(_)) => Ok(()),
        VrApiRsp::Error(VrApiError::WrongType(_, ty)) => safe_assert_eq!(ty, NodeType::Directory),
        VrApiRsp::Error(VrApiError::DoesNotExist(_)) => {
            assert_element_not_found_primary(scheduler, path)
        },
        e => {
            println!("get unhandled error = {:?}", e);
            fail!()
        }
    }
}

/// Attempt to retrieve a client reply and extract useful data from it, along with data from the
/// request.
fn match_client_reply(request: VrMsg, reply: VrEnvelope)
  -> Result<(u64, VrApiReq, VrApiRsp), String>
{
    if let VrMsg::ClientRequest {op, request_num, ..} = request {
        let VrEnvelope {msg, ..} = reply;
        if let VrMsg::ClientReply {request_num: reply_req_num, value, ..} = msg {
            let _ = safe_assert_eq!(reply_req_num, request_num, op);
            return Ok((request_num, op, value));
        }
    }
    fail!()
}


pub fn assert_successful_create(scheduler: &Scheduler,
                                path: String,
                                request_num: u64,
                                ty: NodeType) -> Result<(), String>
{
    try!(assert_majority_of_nodes_contain_op(scheduler, request_num));
    try!(assert_primary_has_committed_op(scheduler, request_num));
    assert_path_exists_in_primary_backend(scheduler, path.clone(), ty)
}

pub fn assert_successful_put_or_get(scheduler: &Scheduler,
                                    path: String,
                                    request_num: u64,
                                    data: Vec<u8>) -> Result<(), String>
{
    try!(assert_majority_of_nodes_contain_op(scheduler, request_num));
    try!(assert_primary_has_committed_op(scheduler, request_num));
    assert_data_matches_primary_backend(scheduler, path, data)
}

pub fn assert_majority_of_nodes_contain_op(scheduler: &Scheduler,
                                           request_num: u64) -> Result<(), String> {
    let mut contained_in_log = 0;
    for r in &scheduler.new_config.replicas {
        match scheduler.get_state(&r) {
            Some((_, ref ctx)) => {
                if is_client_request_last_in_log(ctx, request_num) {
                    contained_in_log += 1;
                }
            },
            None => ()
        }
    }
    safe_assert!(contained_in_log >= scheduler.quorum())
}

pub fn assert_primary_has_committed_op(scheduler: &Scheduler,
                                       request_num: u64) -> Result<(), String>
{
    if let Some(ref primary) = scheduler.primary {
        let (state, ctx) = scheduler.get_state(primary).unwrap();
        try!(safe_assert_eq!(state, "primary"));
        try!(safe_assert_eq!(ctx.op, ctx.commit_num));
        safe_assert!(is_client_request_last_in_log(&ctx, request_num))
    } else {
        fail!()
    }
}

fn assert_data_matches_primary_backend(scheduler: &Scheduler,
                                       path: String,
                                       data: Vec<u8>) -> Result<(), String>
{
    if let Some(ref primary) = scheduler.primary {
        let (_, ctx) = scheduler.get_state(primary).unwrap();
        match ctx.backend.tree.blob_get(path) {
            Ok(Reply {value, ..}) => {
                if let Value::Blob(blob) = value {
                    return safe_assert_eq!(blob, data);
                }
                fail!()
            },
            _ => fail!()
        }
    } else {
        fail!()
    }
}

fn assert_path_exists_in_primary_backend(scheduler: &Scheduler,
                                         path: String,
                                         ty: NodeType) -> Result<(), String>
{
    if let Some(ref primary) = scheduler.primary {
        let ctx = scheduler.get_state(primary).unwrap().1.clone();
        if ctx.backend.tree.find(&path, ty.into()).is_err() {
            // Check to see if it was already created as a directory
            if ctx.backend.tree.find(&path, vertree::NodeType::Directory).is_err() {
                fail!()
            }
        }
        Ok(())
    } else {
        fail!()
    }
}

fn assert_element_not_found_primary(scheduler: &Scheduler,
                                    path: String) -> Result<(), String>
{
    if let Some(ref primary) = scheduler.primary {
        let (_, ctx) = scheduler.get_state(primary).unwrap();
        safe_assert!(ctx.backend.tree.blob_get(path).is_err())
    } else {
        fail!()
    }
}

fn is_client_request_last_in_log(ctx: &VrCtx, request_num: u64) -> bool {
    if ctx.op == 0 { return false; }
    let ref msg = ctx.log[(ctx.op - 1) as usize];
    if let &VrMsg::ClientRequest {request_num: logged_num, ..} = msg {
        if request_num == logged_num {
            return true;
        }
    }
    false
}

