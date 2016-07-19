//! This module contains test functions for specific API operations. It's intended to be general
//! enough to use for multiple tests.

use std::path::Path;
use super::scheduler::Scheduler;
use v2r2::vr::{VrMsg, VrCtx, VrEnvelope, VrApiReq, VrApiRsp};

pub fn assert_create_response(scheduler: &Scheduler,
                              request: VrMsg,
                              reply: VrEnvelope) -> Result<(), String>
{
    let (request_num, api_req, api_rsp) = try!(match_client_reply(request, reply));
    let path = if let VrApiReq::Create {path, ..} = api_req { path } else { fail!() };
    match api_rsp {
        VrApiRsp::Ok => {
            return assert_successful_create(scheduler, path, request_num);
        },
        VrApiRsp::ParentNotFoundError => {
            return assert_parent_not_found_primary(scheduler, path);
        },
        VrApiRsp::ElementAlreadyExistsError => {
            return assert_element_has_diff_op_num_from_latest(scheduler, path);
        },
        _ => fail!()
    }
}

/// Assertions for puts that aren't CAS
pub fn assert_put_response(scheduler: &Scheduler,
                           request: VrMsg,
                           reply: VrEnvelope) -> Result<(), String>
{
    let (request_num, api_req, api_rsp) = try!(match_client_reply(request, reply));
    let (path, data) =
        if let VrApiReq::Put {path, data, ..} = api_req { (path, data) } else { fail!() };
    match api_rsp {
        VrApiRsp::Ok => {
            return assert_successful_put_or_get(scheduler, path, request_num, data);
        },
        VrApiRsp::ElementNotFoundError(_) => {
            return assert_element_not_found_primary(scheduler, path);
        },
        _ => fail!()
    }
}

pub fn assert_get_response(scheduler: &Scheduler,
                           request: VrMsg,
                           reply: VrEnvelope) -> Result<(), String>
{
    let (request_num, api_req, api_rsp) = try!(match_client_reply(request,
                                                                              reply));
    let path = if let VrApiReq::Get {path, ..} = api_req { path } else { fail!() };
    match api_rsp {
        VrApiRsp::Element{data, ..} => {
            return assert_successful_put_or_get(scheduler, path, request_num, data);
        },
        VrApiRsp::ElementNotFoundError(_) => {
            return assert_element_not_found_primary(scheduler, path);
        },
        _ => fail!()
    }
}

/// Attempt to retrieve a client reply and extract useful data from it, along with data from the
/// request.
fn match_client_reply(request: VrMsg, reply: VrEnvelope)
  -> Result<(u64, VrApiReq, VrApiRsp), String>
{
    if let VrMsg::ClientRequest {op, request_num} = request {
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
                                request_num: u64) -> Result<(), String>
{
    try!(assert_majority_of_nodes_contain_op(scheduler, request_num));
    try!(assert_primary_has_committed_op(scheduler, request_num));
    assert_path_exists_in_primary_backend(scheduler, path.clone())
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
        match ctx.backend.tree.get(&path) {
            Some(element) => safe_assert_eq!(element.data, data),
            None => fail!()
        }
    } else {
        fail!()
    }
}

fn assert_path_exists_in_primary_backend(scheduler: &Scheduler,
                                         path: String) -> Result<(), String>
{
    if let Some(ref primary) = scheduler.primary {
        let mut ctx = scheduler.get_state(primary).unwrap().1.clone();
        // only call is public
        let get = VrApiReq::Get {path: path, cas: false};
        // op num is ignored in gets. just use 0
        match ctx.backend.call(0, get) {
            VrApiRsp::Element{..} => Ok(()),
            _ => fail!()
        }
    } else {
        fail!()
    }
}

fn assert_element_not_found_primary(scheduler: &Scheduler,
                                    path: String) -> Result<(), String>
{
    if let Some(ref primary) = scheduler.primary {
        let (_, ctx) = scheduler.get_state(primary).unwrap();
        safe_assert!(!ctx.backend.tree.contains_key(&path))
    } else {
        fail!()
    }
}

fn assert_parent_not_found_primary(scheduler: &Scheduler,
                                   path: String) -> Result<(), String>
{
    if let Some(ref primary) = scheduler.primary {
        let (_, ctx) = scheduler.get_state(primary).unwrap();

        let path = Path::new(&path);
        match path.parent() {
            Some(parent) => {
                if parent == Path::new("/") { return Ok(())}
                safe_assert!(!ctx.backend.tree.contains_key(parent.to_str().unwrap()))
            },
            // If there isn't a parent we are at the root and should not get this error
            None => fail!()
        }
    } else {
        fail!()
    }
}

fn assert_element_has_diff_op_num_from_latest(scheduler: &Scheduler,
                                              path: String) -> Result<(), String>
{

    if let Some(ref primary) = scheduler.primary {
        let (_, ctx) = scheduler.get_state(primary).unwrap();
        match ctx.backend.tree.get(&path) {
            Some(element) => safe_assert!(element.op < ctx.op),
            None => fail!()
        }
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

