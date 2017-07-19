// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! The tests in this module test VR operation by running a bunch of VR replicas in a single tenant on
//! one node. By seperating transport from Fsm operation we are able to run a VR group locally or
//! remotely. While in production, we never want to run more than one replica on a given node, for
//! testing running them locally allows us complete control over how they interact and how we send
//! messages. We can fully stop the peers and inspect the global state to ensure that the protocol
//! invariants are maintained. This is in addition to the FSM level constraints that operate on a
//! single FSM.

#![cfg_attr(feature="cargo-clippy", allow(large_enum_variant))]
#![cfg_attr(feature="cargo-clippy", allow(collapsible_if))]
#![cfg_attr(feature="cargo-clippy", allow(while_let_on_iterator))]

extern crate uuid;
extern crate haret;
extern crate quickcheck;
extern crate rand;
extern crate rabble;

#[macro_use]
extern crate assert_matches;

#[macro_use]
extern crate slog;
extern crate slog_stdlog;
extern crate slog_term;
extern crate slog_envlogger;

extern crate time;

extern crate vertree;

mod utils;

use rabble::{Pid, Envelope};
use vertree::NodeType;
use haret::Msg;
use haret::vr::{VrMsg, VrState};
use haret::vr::{ClientReply, ClientRequest};
use haret::vr::vr_msg::Reconfiguration;
use haret::api::{ApiReq, ApiRsp, TreeOp, TreeOpResult};
use utils::scheduler::Scheduler;

#[test]
fn basic() {
    basic_ops();
    state_transfer();
    recovery();
    reconfiguration();
}

fn basic_ops() {
    let mut scheduler = Scheduler::new("unit-basic", 3);
    let replicas = scheduler.new_config.replicas.clone();

    let op = ApiReq::TreeOp(TreeOp::CreateNode {path: "/test_root".to_string(),
                                                  ty: NodeType::Blob});
    let msg = ClientRequest {client_id: "test_client".to_string(), op: op, request_num: 1};
    let primary = &replicas[0];

    // See if we get a response to our first operation.
    scheduler.send_msg(primary, msg.into());
    let replies = scheduler.send_until_empty();

    assert_create_response(replies);

    // Resend the message. We should get the same response
    /* TODO: Re-enable this check when we add back in client table support
    scheduler.send_msg(primary, msg.clone());
    let replies = scheduler.send_until_empty();
    assert_create_response(replies);
    */

    // The log should still only have one operation in it, since the response was returned from the
    // client_table
    {
        let state = scheduler.get_state(primary).unwrap();
        assert_eq!(state.ctx().log.len(), 1);
        assert_eq!(state.ctx().op, 1);
    }

    assert_put_and_get(primary, &mut scheduler);

    // A tick timeout occurs resulting in a commit message being sent. Note that the current
    // timeouts are all 0, therefore no sleeps are necessary and this is deterministic.
    assert_commit_gets_sent(primary, &mut scheduler, 3);

    // Force a tick timeout at a backup and result in a view change
    assert_second_view_change(&replicas, &mut scheduler);
}

/// We guarantee a state transfer by dropping all messages to r1 and proceeding with the basic ops
/// test. Then we "turn r1 back on" and watch the state change occur.
fn state_transfer() {
    let mut scheduler = Scheduler::new("unit-state_transfer", 3);
    let replicas = scheduler.new_config.replicas.clone();

    let op = ApiReq::TreeOp(TreeOp::CreateNode {path: "/test_root".to_string(), ty: NodeType::Blob});
    let msg = ClientRequest {client_id: "test_client".to_string(), op: op, request_num: 1};
    let primary = &replicas[0];
    scheduler.send_msg(primary, msg.into());
    // dispatch all commands, dropping messages to r2
    let replies = scheduler.send_until_empty_with_drop(&replicas[1]);

    // Ensure we still commit the create op
    assert_create_response(replies);

    // Ensure we have no ops at r2
    {
        let state = scheduler.get_state(&replicas[1]).unwrap();
        assert_eq!(state.ctx().log.len(), 0);
        assert_eq!(state.ctx().op, 0);
        assert_eq!(state.ctx().commit_num, 0);
    }

    // Send a commit to all nodes and ensure the state transfer
    assert_commit_gets_sent(primary, &mut scheduler, 1);

    // Ensure r2 has received and committed the op
    let state = scheduler.get_state(&replicas[1]).unwrap();
    let ctx = state.ctx();
    assert_eq!(ctx.log.len(), 1);
    assert_eq!(ctx.op, 1);
    assert_eq!(ctx.commit_num, 1);
}

fn recovery() {
    let mut scheduler = Scheduler::new("unit-recovery", 3);
    let replicas = scheduler.new_config.replicas.clone();

    let op = ApiReq::TreeOp(TreeOp::CreateNode {path: "/test_root".to_string(), ty: NodeType::Blob});
    let msg = ClientRequest {client_id: "test_client".to_string(), op: op, request_num: 1};
    let view0_primary = &replicas[0];

    // See if we get a response to our first operation.
    scheduler.send_msg(view0_primary, msg.into());
    let replies = scheduler.send_until_empty();
    assert_create_response(replies);
    assert_commit_gets_sent(view0_primary, &mut scheduler, 1);

    // Stop the view0_primary and trigger an election
    scheduler.stop(view0_primary);
    scheduler.send_msg(&replicas[1], VrMsg::Tick);
    scheduler.send_until_empty();

    // Restart the view0_primary and check the status of the replicas
    scheduler.restart(view0_primary);

    {
        let state = scheduler.get_state(view0_primary).unwrap();
        assert_matches!(state, VrState::Backup(_));
        assert_eq!(state.ctx().view, 1);
        assert_eq!(state.ctx().op, 1);

        let state  = scheduler.get_state(&replicas[1]).unwrap();
        assert_matches!(state, VrState::Primary(_));
        assert_eq!(state.ctx().view, 1);
        assert_eq!(state.ctx().op, 1);

        let state = scheduler.get_state(&replicas[2]).unwrap();
        assert_matches!(state, VrState::Backup(_));
        assert_eq!(state.ctx().view, 1);
        assert_eq!(state.ctx().op, 1);
    }

    // Send a tick to the old view0_primary so that it starts recovery
    scheduler.send_msg(view0_primary, VrMsg::Tick);
    scheduler.send_until_empty();

    // Ensure that the old view0_primary is now a backup with the proper state
    let state = scheduler.get_state(view0_primary).unwrap();
    assert_matches!(state, VrState::Backup(_));
    assert_eq!(state.ctx().view, 2);
    assert_eq!(state.ctx().op, 1);
}

fn reconfiguration() {
    let mut scheduler = Scheduler::new("unit-reconfiguration", 3);
    let mut replicas = scheduler.new_config.replicas.clone();

    assert_eq!(scheduler.fsms.len(), 3);

    let new_replica = Pid {group: replicas[0].group.clone(),
                           name: "r4".to_string(),
                           node: replicas[0].node.clone() };
    replicas.push(new_replica);
    let msg = Reconfiguration {client_req_num: 1, epoch: 1, replicas: replicas.clone()};
    let primary = &replicas[0];
    // Send reconfiguration to the primary and commit it
    scheduler.send_msg(primary, msg.into());
    let mut replies = scheduler.send_until_empty();

    assert_eq!(replies.len(), 1);

    // Assure that we get a response to the committed reconfiguration
    let msg = replies.pop().unwrap().msg;
    if let rabble::Msg::User(Msg::Vr(VrMsg::ClientReply(reply)))= msg {
        assert_eq!(reply.request_num, 1);
        assert_eq!(reply.value, ApiRsp::Ok);
    } else {
        assert!(false);
    }

    // Ensure that the new replica was started
    assert_eq!(scheduler.fsms.len(), 4);

    // Send a Tick to start reconfiguration state transfer
    scheduler.send_msg(&replicas[3], VrMsg::Tick);
    scheduler.send_until_empty();
    let state = scheduler.get_state(&replicas[3]).unwrap();
    assert_matches!(state, VrState::Backup(_));

    assert_new_epoch(&replicas, &mut scheduler);

    // Send a tick to the new replica and dispatch all resulting messages so that it can cause a
    // view change in the new epoch.
    scheduler.send_msg(&replicas[3], VrMsg::Tick);
    scheduler.send_until_empty();
    let state = scheduler.get_state(&replicas[3]).unwrap();
    assert_matches!(state, VrState::Backup(_));
    assert_eq!(state.ctx().epoch, 2);
    assert_eq!(state.ctx().view, 1);
}

fn assert_new_epoch(replicas: &[Pid], scheduler: &mut Scheduler) {
    if let Some(state) = scheduler.get_state(&replicas[0]) {
        assert_eq!(state.ctx().epoch, 2);
        assert_matches!(state, VrState::Primary(_));
        assert_eq!(state.ctx().view, 0);
    } else {
        assert!(false)
    }
    if let Some(state) = scheduler.get_state(&replicas[1]) {
        assert_eq!(state.ctx().epoch, 2);
        assert_matches!(state, VrState::Backup(_));
        assert_eq!(state.ctx().view, 0);
    } else {
        assert!(false)
    }
    if let Some(state) = scheduler.get_state(&replicas[2]) {
        assert_eq!(state.ctx().epoch, 2);
        assert_matches!(state, VrState::Backup(_));
        assert_eq!(state.ctx().view, 0);
    } else {
        assert!(false)
    }
}

fn assert_second_view_change(replicas: &[Pid], scheduler: &mut Scheduler) {
    scheduler.send_msg(&replicas[1], VrMsg::Tick);
    scheduler.send_until_empty();
    if let Some(state) = scheduler.get_state(&replicas[0]) {
        assert_matches!(state, VrState::Backup(_));
        assert_eq!(state.ctx().view, 1);
        assert_eq!(state.ctx().op, 3);
    } else {
        assert!(false)
    }
    if let Some(state) = scheduler.get_state(&replicas[1]) {
        assert_matches!(state, VrState::Primary(_));
        assert_eq!(state.ctx().view, 1);
        assert_eq!(state.ctx().op, 3);
    } else {
        assert!(false)
    }
    if let Some(state) = scheduler.get_state(&replicas[2]) {
        assert_matches!(state, VrState::Backup(_));
        assert_eq!(state.ctx().view, 1);
        assert_eq!(state.ctx().op, 3);
    } else {
        assert!(false)
    }
}

// Ensure the primary sends a commit to the 2 backups
fn assert_commit_gets_sent(primary: &Pid,
                           scheduler: &mut Scheduler,
                           expected_commit: u64)
{
    scheduler.send_msg(primary, VrMsg::Tick);
    for _ in 0..2 {
        if let Some(Envelope {to, msg, ..}) = scheduler.next() {
            if let rabble::Msg::User(Msg::Vr(VrMsg::Commit(commit))) = msg {
                assert_eq!(commit.view, 0);
                assert_eq!(commit.commit_num, expected_commit);
                scheduler.send_msg(&to, commit.into());
            } else {
                println!("failing msg = {:#?}", msg);
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }
    scheduler.send_until_empty();
}

fn assert_put_and_get(primary: &Pid, scheduler: &mut Scheduler) {
    let put_op =
        ApiReq::TreeOp(TreeOp::BlobPut { path: "/test_root".to_string(), val: b"hello".to_vec()});
    let put_msg =
        ClientRequest {client_id: "test_client".to_string(), op: put_op, request_num: 2};
    scheduler.send_msg(primary, put_msg.into());
    scheduler.send_until_empty();
    let get_op = ApiReq::TreeOp(TreeOp::BlobGet { path: "/test_root".to_string()});
    let get_msg = ClientRequest {client_id: "test_client".to_string(), op: get_op, request_num: 3};
    scheduler.send_msg(primary, get_msg.into());
    let mut replies = scheduler.send_until_empty();

    // Ensure we get back the value stored
    assert_eq!(replies.len(), 1);

    let msg = replies.pop().unwrap().msg;
    if let rabble::Msg::User(Msg::Vr(VrMsg::ClientReply(reply)))= msg {
        let ClientReply {epoch, view, request_num, value} = reply;
        assert_eq!(epoch, 1);
        assert_eq!(view, 0);
        assert_eq!(request_num, 3);
        if let ApiRsp::TreeOpResult(TreeOpResult::Blob(data, _)) = value {
            assert_eq!(data, b"hello".to_vec());
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }
}

fn assert_create_response(mut replies: Vec<Envelope<Msg>>) {
    assert_eq!(replies.len(), 1);
    let msg = replies.pop().unwrap().msg;
    if let rabble::Msg::User(Msg::Vr(VrMsg::ClientReply(reply))) = msg {
        assert_eq!(reply.request_num, 1);
        assert_eq!(reply.value, ApiRsp::Ok);
    } else {
        assert!(false);
    }
}
