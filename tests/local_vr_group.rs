//! The tests in this module test VR operation by running a bunch of VR replicas in a single tenant on
//! one node. By seperating transport from Fsm operation we are able to run a VR group locally or
//! remotely. While in production, we never want to run more than one replica on a given node, for
//! testing running them locally allows us complete control over how they interact and how we send
//! messages. We can fully stop the peers and inspect the global state to ensure that the protocol
//! invariants are maintained. This is in addition to the FSM level constraints that operate on a
//! single FSM.

extern crate msgpack;
extern crate rustc_serialize;
extern crate uuid;
extern crate v2r2;
extern crate distill;
#[macro_use]
extern crate fsm;

pub mod debugger_shared;

use uuid::Uuid;
use v2r2::vr::{Replica, VrMsg, Envelope, ClientReplyEnvelope, PeerEnvelope};
use v2r2::vr::{ElementType, VrApiReq, VrApiRsp};
use debugger_shared::Scheduler;

#[test]
fn basic_ops() {
    let mut scheduler = Scheduler::new(3);
    scheduler.elect_initial_leader();
    let replicas = scheduler.new_config.replicas.clone();

    let session_id = Uuid::new_v4();
    let op = VrApiReq::Create {path: "/test_root".to_string(), ty: ElementType::Binary};
    let msg = VrMsg::ClientRequest {op: op, session_id: session_id, request_num: 1};
    let primary = &replicas[1];

    // See if we get a response to our first operation.
    scheduler.send_direct(primary, msg.clone());
    let replies = scheduler.send_until_empty();
    assert_create_response(&session_id, replies);

    println!("AFTER FIRST ASSERT_CREATE_RESPONSE");
    // Resend the message. We should get the same response
    scheduler.send_direct(primary, msg.clone());
    let replies = scheduler.send_until_empty();
    assert_create_response(&session_id, replies);

    // The log should still only have one operation in it, since the response was returned from the
    // client_table
    let (_, primary_ctx) = scheduler.get_state(primary).unwrap();
    assert_eq!(primary_ctx.log.len(), 1);
    assert_eq!(primary_ctx.op, 1);

    assert_put_and_get(primary, &session_id, &mut scheduler);

    // A tick timeout occurs resulting in a commit message being sent. Note that the current
    // timeouts are all 0, therefore no sleeps are necessary and this is deterministic.
    assert_commit_gets_sent(primary, &mut scheduler, 3);

    // Force a tick timeout at a backup and result in a view change
    assert_second_view_change(&replicas, &mut scheduler);

}

#[test]
/// We guarantee a state transfer by dropping all messages to r1 and proceeding with the basic ops
/// test. Then we "turn r1 back on" and watch the state change occur.
fn state_transfer() {
    let mut scheduler = Scheduler::new(3);
    scheduler.elect_initial_leader();
    let replicas = scheduler.new_config.replicas.clone();

    let session_id = Uuid::new_v4();
    let op = VrApiReq::Create {path: "/test_root".to_string(), ty: ElementType::Binary};
    let msg = VrMsg::ClientRequest {op: op, session_id: session_id, request_num: 1};
    let primary = &replicas[1];
    scheduler.send_direct(primary, msg.clone());
    // dispatch all commands, dropping messages to r1
    let replies = scheduler.send_until_empty_with_drop(&replicas[0]);

    // Ensure we still commit the create op
    assert_create_response(&session_id, replies);

    // Ensure we have no ops at r1
    let (_, r1_ctx) = scheduler.get_state(&replicas[0]).unwrap();
    assert_eq!(r1_ctx.log.len(), 0);
    assert_eq!(r1_ctx.op, 0);
    assert_eq!(r1_ctx.commit_num, 0);

    // Send a commit to all nodes and ensure the state transfer
    assert_commit_gets_sent(primary, &mut scheduler, 1);

    // Ensure r1 has received and committed the op
    let (_, r1_ctx) = scheduler.get_state(&replicas[0]).unwrap();
    assert_eq!(r1_ctx.log.len(), 1);
    assert_eq!(r1_ctx.op, 1);
    assert_eq!(r1_ctx.commit_num, 1);

}

#[test]
fn recovery() {
    let mut scheduler = Scheduler::new(3);
    scheduler.elect_initial_leader();
    let replicas = scheduler.new_config.replicas.clone();

    let session_id = Uuid::new_v4();
    let op = VrApiReq::Create {path: "/test_root".to_string(), ty: ElementType::Binary};
    let msg = VrMsg::ClientRequest {op: op, session_id: session_id, request_num: 1};
    let view1_primary = &replicas[1];

    // See if we get a response to our first operation.
    scheduler.send_direct(view1_primary, msg.clone());
    let replies = scheduler.send_until_empty();
    assert_create_response(&session_id, replies);
    assert_commit_gets_sent(view1_primary, &mut scheduler, 1);

    // Stop the view1_primary and trigger an election
    scheduler.stop_(view1_primary);
    scheduler.send_direct(&replicas[0], VrMsg::Tick);
    scheduler.send_until_empty();

    // Restart the view1_primary and check the status of the replicas
    scheduler.restart_(view1_primary);

    let (state, ctx) = scheduler.get_state(view1_primary).unwrap();
    assert_eq!(state, "backup");
    assert_eq!(ctx.view, 2);
    assert_eq!(ctx.op, 1);

    let (state, ctx) = scheduler.get_state(&replicas[0]).unwrap();
    assert_eq!(state, "backup");
    assert_eq!(ctx.view, 2);
    assert_eq!(ctx.op, 1);

    let (state, ctx) = scheduler.get_state(&replicas[2]).unwrap();
    assert_eq!(state, "primary");
    assert_eq!(ctx.view, 2);
    assert_eq!(ctx.op, 1);

    // Send a tick to the old view1_primary so that it starts recovery
    scheduler.send_direct(view1_primary, VrMsg::Tick);
    scheduler.send_until_empty();

    // Ensure that the old view1_primary is now a backup with the proper state
    let (state, ctx) = scheduler.get_state(view1_primary).unwrap();
    assert_eq!(state, "backup");
    assert_eq!(ctx.view, 3);
    assert_eq!(ctx.op, 1);
}

#[test]
fn reconfiguration() {
    let mut scheduler = Scheduler::new(3);
    scheduler.elect_initial_leader();
    let mut replicas = scheduler.new_config.replicas.clone();

    assert_eq!(scheduler.fsms.len(), 3);

    let new_replica = Replica {tenant: replicas[0].tenant.clone(),
                               name: "r4".to_string(),
                               node: replicas[0].node.clone() };
    replicas.push(new_replica);
    let session_id = Uuid::new_v4();
    let msg = VrMsg::Reconfiguration{session_id: session_id,
                                     client_req_num: 1,
                                     epoch: 1,
                                     replicas: replicas.clone()};
    let primary = &replicas[1];
    // Send reconfiguration to the primary and commit it
    scheduler.send_direct(primary, msg.clone());
    let mut replies = scheduler.send_until_empty();

    assert_eq!(replies.len(), 1);

    // Assure that we get a response to the committed reconfiguration
    let ClientReplyEnvelope{to, msg} = replies.pop().unwrap();
    assert_eq!(to, session_id);
    if let VrMsg::ClientReply {request_num, value, ..} = msg {
        assert_eq!(request_num, 1);
        assert_eq!(value, VrApiRsp::Ok);
    } else {
        assert!(false);
    }

    // Ensure that the new replica was started
    assert_eq!(scheduler.fsms.len(), 4);

    let (state, _) = scheduler.get_state(&replicas[3]).unwrap();
    assert_eq!(state, "backup");

    assert_new_epoch(&replicas, &mut scheduler);

    // Send a tick to the new replica and dispatch all resulting messages so that it can cause a
    // view change in the new epoch.
    scheduler.send_direct(&replicas[3], VrMsg::Tick);
    scheduler.send_until_empty();
    let (state, ctx) = scheduler.get_state(&replicas[3]).unwrap();
    assert_eq!(ctx.epoch, 2);
    assert_eq!(state, "backup");
    assert_eq!(ctx.view, 1);
}

fn assert_new_epoch(replicas: &Vec<Replica>, scheduler: &mut Scheduler) {
    if let Some((state, ctx)) = scheduler.get_state(&replicas[0]) {
        assert_eq!(ctx.epoch, 2);
        assert_eq!(state, "primary");
        assert_eq!(ctx.view, 0);
    } else {
        assert!(false)
    }
    if let Some((state, ctx)) = scheduler.get_state(&replicas[1]) {
        assert_eq!(ctx.epoch, 2);
        assert_eq!(state, "backup");
        assert_eq!(ctx.view, 0);
    } else {
        assert!(false)
    }
    if let Some((state, ctx)) = scheduler.get_state(&replicas[2]) {
        assert_eq!(ctx.epoch, 2);
        assert_eq!(state, "backup");
        assert_eq!(ctx.view, 0);
    } else {
        assert!(false)
    }
}

fn assert_second_view_change(replicas: &Vec<Replica>, scheduler: &mut Scheduler) {
    scheduler.send_direct(&replicas[0], VrMsg::Tick);
    scheduler.send_until_empty();
    if let Some((state, ctx)) = scheduler.get_state(&replicas[0]) {
        assert_eq!(state, "backup");
        assert_eq!(ctx.view, 2);
        assert_eq!(ctx.op, 3);
    } else {
        assert!(false)
    }
    if let Some((state, ctx)) = scheduler.get_state(&replicas[1]) {
        assert_eq!(state, "backup");
        assert_eq!(ctx.view, 2);
        assert_eq!(ctx.op, 3);
    } else {
        assert!(false)
    }
    if let Some((state, ctx)) = scheduler.get_state(&replicas[2]) {
        assert_eq!(state, "primary");
        assert_eq!(ctx.view, 2);
        assert_eq!(ctx.op, 3);
    } else {
        assert!(false)
    }
}

// Ensure the primary sends a commit to the 2 backups
fn assert_commit_gets_sent(primary: &Replica, scheduler: &mut Scheduler, expected_commit: u64) {
    scheduler.send_direct(primary, VrMsg::Tick);
    for _ in 0..2 {
        if let Some(Envelope::Peer(PeerEnvelope {to, msg, ..})) = scheduler.next() {
            let msg2 = msg.clone();
            if let VrMsg::Commit {view, commit_num, ..} = msg {
                assert_eq!(view, 1);
                assert_eq!(commit_num, expected_commit);
                scheduler.send_direct(&to, msg2);
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

fn assert_put_and_get(primary: &Replica, session_id: &Uuid, scheduler: &mut Scheduler) {
    let put_op = VrApiReq::Put { path: "/test_root".to_string(), data: b"hello".to_vec(), cas_tag: None};
    let put_msg = VrMsg::ClientRequest {op: put_op, session_id: session_id.clone(), request_num: 2};
    scheduler.send_direct(primary, put_msg);
    scheduler.send_until_empty();
    let get_op = VrApiReq::Get { path: "/test_root".to_string(), cas: false };
    let get_msg = VrMsg::ClientRequest {op: get_op, session_id: session_id.clone(), request_num: 3};
    scheduler.send_direct(primary, get_msg);
    let mut replies = scheduler.send_until_empty();

    // Ensure we get back the value stored
    assert_eq!(replies.len(), 1);
    let ClientReplyEnvelope{to, msg} = replies.pop().unwrap();
    assert_eq!(to, *session_id);
    if let VrMsg::ClientReply{epoch, view, request_num, value} = msg {
        assert_eq!(epoch, 1);
        assert_eq!(view, 1);
        assert_eq!(request_num, 3);
        if let VrApiRsp::Element {data, ..} = value {
            assert_eq!(data, b"hello".to_vec());
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }

}

fn assert_create_response(session_id: &Uuid, mut replies: Vec<ClientReplyEnvelope>) {
    assert_eq!(replies.len(), 1);
    let ClientReplyEnvelope{to, msg} = replies.pop().unwrap();
    assert_eq!(to, *session_id);
    if let VrMsg::ClientReply{request_num, value, ..} = msg {
        assert_eq!(request_num, 1);
        assert_eq!(value, VrApiRsp::Ok);
    } else {
        assert!(false);
    }
}
