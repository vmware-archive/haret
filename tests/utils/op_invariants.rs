//! This module contains test functions for specific API operations. It's intended to be general
//! enough to use for multiple tests.

use uuid::Uuid;
use std::path::Path;
use v2r2::vr::{Dispatcher, Replica, VrMsg, VrCtx, ClientReplyEnvelope, VrApiReq, VrApiRsp};

pub fn assert_create_response(replicas: &Vec<Replica>,
                              dispatcher: &Dispatcher,
                              primary: &Replica,
                              request: VrMsg) -> Result<(), String> {

    let (session_id, request_num, api_req, api_rsp) = try!(match_client_reply(dispatcher, request));
    let path = if let VrApiReq::Create {path, ..} = api_req { path } else { fail!() };
    match api_rsp {
        VrApiRsp::Ok => {
            return assert_successful_create(replicas,
                                            dispatcher,
                                            primary,
                                            path,
                                            &session_id,
                                            request_num)
        },
        VrApiRsp::ParentNotFoundError => {
            return assert_parent_not_found_primary(dispatcher, primary, path)
        },
        VrApiRsp::ElementAlreadyExistsError => {
            return assert_element_has_diff_op_num_from_latest(dispatcher,
                                                              primary,
                                                              path)
        },
        _ => fail!()
    }
}

/// Assertions for puts that aren't CAS
pub fn assert_put_response(replicas: &Vec<Replica>,
                           dispatcher: &Dispatcher,
                           primary: &Replica,
                           request: VrMsg) -> Result<(), String> {

    let (session_id, request_num, api_req, api_rsp) = try!(match_client_reply(dispatcher, request));
    let (path, data) =
        if let VrApiReq::Put {path, data, ..} = api_req { (path, data) } else { fail!() };
    match api_rsp {
        VrApiRsp::Ok => {
            return assert_successful_put_or_get(replicas,
                                                dispatcher,
                                                primary,
                                                path,
                                                &session_id,
                                                request_num,
                                                data)
        },
        VrApiRsp::ElementNotFoundError(_) => {
            return assert_element_not_found_primary(dispatcher, primary, path)
        },
        _ => fail!()
    }
}

pub fn assert_get_response(replicas: &Vec<Replica>,
                           dispatcher: &Dispatcher,
                           primary: &Replica,
                           request: VrMsg) -> Result<(), String> {

    let (session_id, request_num, api_req, api_rsp) = try!(match_client_reply(dispatcher, request));
    let path = if let VrApiReq::Get {path, ..} = api_req { path } else { fail!() };
    match api_rsp {
        VrApiRsp::Element{data, ..} => {
            return assert_successful_put_or_get(replicas,
                                                dispatcher,
                                                primary,
                                                path,
                                                &session_id,
                                                request_num,
                                                data)
        },
        VrApiRsp::ElementNotFoundError(_) => {
            return assert_element_not_found_primary(dispatcher, primary, path)
        },
        _ => fail!()
    }
}

/// Attempt to retrieve a client reply and extract useful data from it, along with data from the
/// request.
fn match_client_reply(dispatcher: &Dispatcher,
                      request: VrMsg) -> Result<(Uuid, u64, VrApiReq, VrApiRsp), String> {
    if let VrMsg::ClientRequest {op, session_id, request_num} = request {
        match dispatcher.try_recv_client_reply() {
            Ok(ClientReplyEnvelope {to, msg: VrMsg::ClientReply {request_num: reply_req_num,
                                                                 value, ..}}) => {
                let _ = safe_assert_eq!(to, session_id);
                let _ = safe_assert_eq!(reply_req_num, request_num, op);
                return Ok((session_id, request_num, op, value))
            },
            _ => fail!()
        }
    }
    fail!()
}


pub fn assert_successful_create(replicas: &Vec<Replica>,
                                dispatcher: &Dispatcher,
                                primary: &Replica,
                                path: String,
                                session_id: &Uuid,
                                request_num: u64) -> Result<(), String>
{
    try!(assert_majority_of_nodes_contain_op(replicas, dispatcher, &session_id, request_num));
    try!(assert_primary_has_committed_op(dispatcher, primary, &session_id, request_num));
    assert_path_exists_in_primary_backend(dispatcher, primary, path.clone())
}

pub fn assert_successful_put_or_get(replicas: &Vec<Replica>,
                                    dispatcher: &Dispatcher,
                                    primary: &Replica,
                                    path: String,
                                    session_id: &Uuid,
                                    request_num: u64,
                                    data: Vec<u8>) -> Result<(), String>
{
    try!(assert_majority_of_nodes_contain_op(replicas, dispatcher, &session_id, request_num));
    try!(assert_primary_has_committed_op(dispatcher, primary, &session_id, request_num));
    assert_data_matches_primary_backend(dispatcher, primary, path, data)
}

pub fn assert_majority_of_nodes_contain_op(replicas: &Vec<Replica>,
                                           dispatcher: &Dispatcher,
                                           session_id: &Uuid,
                                           request_num: u64) -> Result<(), String> {
    let quorum = replicas.len() / 2 + 1;
    let mut contained_in_log = 0;
    for r in replicas {
        match dispatcher.get_state(&r) {
            Some((_, ctx)) => {
                if is_client_request_last_in_log(&ctx, session_id, request_num) {
                    contained_in_log += 1;
                }
            },
            None => ()
        }
    }
    safe_assert!(contained_in_log >= quorum)
}

#[allow(unused_must_use)]
pub fn assert_primary_has_committed_op(dispatcher: &Dispatcher,
                                       primary: &Replica,
                                       session_id: &Uuid,
                                       request_num: u64) -> Result<(), String> {

    let (state, ctx) = dispatcher.get_state(primary).unwrap();
    safe_assert_eq!(state, "primary");
    safe_assert_eq!(ctx.op, ctx.commit_num);
    safe_assert_eq!(true, is_client_request_last_in_log(&ctx, session_id, request_num))
}

fn assert_data_matches_primary_backend(dispatcher: &Dispatcher,
                                       primary: &Replica,
                                       path: String,
                                       data: Vec<u8>) -> Result<(), String> {
    let (_, ctx) = dispatcher.get_state(primary).unwrap();
    match ctx.backend.tree.get(&path) {
        Some(element) => safe_assert_eq!(element.data, data),
        None => fail!()
    }
}

fn assert_path_exists_in_primary_backend(dispatcher: &Dispatcher,
                                         primary: &Replica,
                                         path: String) -> Result<(), String> {
    let (_, mut ctx) = dispatcher.get_state(primary).unwrap();
    // only call is public
    let get = VrApiReq::Get {path: path, cas: false};
    // op num is ignored in gets. just use 0
    match ctx.backend.call(0, get) {
        VrApiRsp::Element{..} => Ok(()),
        _ => fail!()
    }
}

fn assert_element_not_found_primary(dispatcher: &Dispatcher,
                                    primary: &Replica,
                                    path: String) -> Result<(), String> {
    let (_, ctx) = dispatcher.get_state(primary).unwrap();
    safe_assert!(!ctx.backend.tree.contains_key(&path))
}

fn assert_parent_not_found_primary(dispatcher: &Dispatcher,
                                   primary: &Replica,
                                   path: String) -> Result<(), String> {
    let (_, ctx) = dispatcher.get_state(primary).unwrap();

    let path = Path::new(&path);
    match path.parent() {
        Some(parent) => {
            if parent == Path::new("/") { return Ok(())}
            safe_assert!(!ctx.backend.tree.contains_key(parent.to_str().unwrap()))
        },
        // If there isn't a parent we are at the root and should not get this error
        None => fail!()
    }
}

fn assert_element_has_diff_op_num_from_latest(dispatcher: &Dispatcher,
                                              primary: &Replica,
                                              path: String) -> Result<(), String> {
    let (_, ctx) = dispatcher.get_state(primary).unwrap();
    match ctx.backend.tree.get(&path) {
        Some(element) => safe_assert!(element.op < ctx.op),
        None => fail!()
    }
}


fn is_client_request_last_in_log(ctx: &VrCtx, session_id: &Uuid, request_num: u64) -> bool {
    if ctx.op == 0 { return false; }
    let ref msg = ctx.log[(ctx.op - 1) as usize];
    if let &VrMsg::ClientRequest {session_id: logged_id, request_num: logged_num, ..} = msg {
        if *session_id == logged_id && request_num == logged_num {
            return true;
        }
    }
    false
}

