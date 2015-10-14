//! The tests in this module test VR operation by running a bunch of VR replicas in a single tenant on
//! one node. By seperating transport from Fsm operation we are able to run a VR group locally or
//! remotely. While in production, we never want to run more than one replica on a given node, for
//! testing running them locally allows us complete control over how they interact and how we send
//! messages. We can fully stop the peers and inspect the global state to ensure that the protocol
//! invariants are maintained. This is in addition to the FSM level constraints that operate on a
//! single FSM.

extern crate uuid;
extern crate v2r2;
extern crate fsm;

use uuid::Uuid;
use v2r2::vr::{Dispatcher, Replica, RawReplica, VrMsg, Envelope, ClientReplyEnvelope};
use v2r2::vr_api::{ElementType, VrApiReq, VrApiRsp};
use v2r2::Member;
use fsm::FsmType;

#[test]
fn initial_view_change() {
    let (mut dispatcher, replicas) = init_tenant();
    elect_initial_leader(&mut dispatcher, &replicas);
}

#[test]
fn basic_ops() {
    let (mut dispatcher, replicas) = init_tenant();
    elect_initial_leader(&mut dispatcher, &replicas);

    let client_id = Uuid::new_v4();
    let op = VrApiReq::Create {path: "/test_root".to_string(), ty: ElementType::Binary};
    let msg = VrMsg::ClientRequest {op: op, client_id: client_id, request_num: 1};
    let primary = &replicas[1];
    dispatcher.send(primary, msg.clone());
    dispatcher.dispatch_all_received_messages();

    // See if we get a response to our first operation.
    assert_create_response(&client_id, &dispatcher);

    // Resend the message. We should get the same response
    dispatcher.send(primary, msg.clone());
    dispatcher.dispatch_all_received_messages();
    assert_create_response(&client_id, &dispatcher);
    // The log should still only have one operation in it, since the response was returned from the
    // client_table
    let (_, primary_ctx) = dispatcher.get_state(primary).unwrap();
    assert_eq!(primary_ctx.log.len(), 1);
    assert_eq!(primary_ctx.op, 1);

    assert_put_and_get(primary, &client_id, &mut dispatcher);

    // A tick timeout occurs resulting in a commit message being sent. Note that the current
    // timeouts are all 0, therefore no sleeps are necessary and this is deterministic.
    assert_commit_gets_sent(primary, &mut dispatcher, 3);

    // Force a tick timeout at a backup and result in a view change
    assert_second_view_change(&replicas, &mut dispatcher);

    // Ensure we can get the last committed value from the new primary
    // We don't even need to dispatch received messages since this should be served directly from
    // the primary, since it's the last committed request.
    assert_new_primary_returns_last_committed_result(&replicas[2], &client_id, &mut dispatcher);
}

#[test]
/// We guarantee a state change by dropping all messages to r1 and proceeding with the basic ops
/// test. Then we "turn r1 back on" and watch the state change occur.
fn state_change() {
    let (mut dispatcher, replicas) = init_tenant();
    elect_initial_leader(&mut dispatcher, &replicas);

    let client_id = Uuid::new_v4();
    let op = VrApiReq::Create {path: "/test_root".to_string(), ty: ElementType::Binary};
    let msg = VrMsg::ClientRequest {op: op, client_id: client_id, request_num: 1};
    let primary = &replicas[1];
    dispatcher.send(primary, msg.clone());
    // dispatch all commands, dropping messages to r1
    dispatch_with_drop(&mut dispatcher, &replicas[0]);

    // Ensure we still commit the create op
    assert_create_response(&client_id, &dispatcher);

    // Ensure we have no ops at r1 still
    let (_, r1_ctx) = dispatcher.get_state(&replicas[0]).unwrap();
    assert_eq!(r1_ctx.log.len(), 0);
    assert_eq!(r1_ctx.op, 0);
    assert_eq!(r1_ctx.commit_num, 0);

    // Send a commit to all nodes and ensure the state transfer
    assert_commit_gets_sent(primary, &mut dispatcher, 1);

    // Ensure r1 has received and committed the op
    let (_, r1_ctx) = dispatcher.get_state(&replicas[0]).unwrap();
    assert_eq!(r1_ctx.log.len(), 1);
    assert_eq!(r1_ctx.op, 1);
    assert_eq!(r1_ctx.commit_num, 1);

}

#[test]
fn recovery() {
    let (mut dispatcher, replicas) = init_tenant();
    elect_initial_leader(&mut dispatcher, &replicas);

    let client_id = Uuid::new_v4();
    let op = VrApiReq::Create {path: "/test_root".to_string(), ty: ElementType::Binary};
    let msg = VrMsg::ClientRequest {op: op, client_id: client_id, request_num: 1};
    let primary = &replicas[1];
    dispatcher.send(primary, msg.clone());
    dispatcher.dispatch_all_received_messages();

    // See if we get a response to our first operation.
    assert_create_response(&client_id, &dispatcher);
    assert_commit_gets_sent(primary, &mut dispatcher, 1);

    // Stop the primary and trigger an election
    dispatcher.stop(primary);
    dispatcher.send(&replicas[0], VrMsg::Tick);
    dispatcher.dispatch_all_received_messages();

    // Restart the old primary and check the status of the replicas
    dispatcher.restart(primary.clone(), FsmType::Local);
    let (state, ctx) = dispatcher.get_state(primary).unwrap();
    assert_eq!(state, "recovery");
    assert_eq!(ctx.view, 0);
    assert_eq!(ctx.op, 0);

    let (state, ctx) = dispatcher.get_state(&replicas[0]).unwrap();
    assert_eq!(state, "backup");
    assert_eq!(ctx.view, 2);
    assert_eq!(ctx.op, 1);

    let (state, ctx) = dispatcher.get_state(&replicas[2]).unwrap();
    assert_eq!(state, "primary");
    assert_eq!(ctx.view, 2);
    assert_eq!(ctx.op, 1);

    // Send a tick to the old primary so that it starts recovery
    dispatcher.send(primary, VrMsg::Tick);
    dispatcher.dispatch_all_received_messages();

    // Ensure that the old primary is now a backup with the proper state
    let (state, ctx) = dispatcher.get_state(primary).unwrap();
    assert_eq!(state, "backup");
    assert_eq!(ctx.view, 2);
    assert_eq!(ctx.op, 1);
}

#[test]
fn reconfiguration() {
    let (mut dispatcher, mut replicas) = init_tenant();
    elect_initial_leader(&mut dispatcher, &replicas);

    assert_eq!(dispatcher.local_replicas.len(), 3);

    let node = Member {
        name: "node1".to_string(),
        cluster: "test".to_string(),
        ip: "127.0.0.1:5000".to_string()
    };
    let new_replica = Replica {tenant: replicas[0].tenant.clone(),
                               name: "r4".to_string(),
                               node: node.clone() };
    replicas.push(new_replica);
    let client_id = Uuid::new_v4();
    let msg = VrMsg::Reconfiguration{client_id: client_id,
                                     client_req_num: 1,
                                     epoch: 1,
                                     replicas: replicas.clone()};
    let primary = &replicas[1];
    // Send reconfiguration to the primary and commit it
    dispatcher.send(primary, msg.clone());
    dispatcher.dispatch_all_received_messages();

    // Assure that we get a response to the committed reconfiguration
    let ClientReplyEnvelope{to, msg} = dispatcher.try_recv_client_reply().unwrap();
    assert_eq!(to, client_id);
    if let VrMsg::ClientReply{request_num, value, ..} = msg {
        assert_eq!(request_num, 1);
        assert_eq!(value, VrApiRsp::Ok);
    } else {
        assert!(false);
    }

    // Start the new replica
    let dispatch_msg = dispatcher.try_recv_dispatch_msg().unwrap();
    dispatcher.handle_dispatch_msg(dispatch_msg);
    assert_eq!(dispatcher.local_replicas.len(), 4);

    let (state, _) = dispatcher.get_state(&replicas[3]).unwrap();
    assert_eq!(state, "startup");

    assert_new_epoch(&replicas, &mut dispatcher);

    // Send a tick to the new replica and dispatch all resulting messages so that it can transition
    // to backup state in the new epoch.
    dispatcher.send(&replicas[3], VrMsg::Tick);
    dispatcher.dispatch_all_received_messages();
    let (state, ctx) = dispatcher.get_state(&replicas[3]).unwrap();
    assert_eq!(ctx.epoch, 2);
    assert_eq!(state, "backup");
    assert_eq!(ctx.view, 0);
}

fn assert_new_epoch(replicas: &Vec<Replica>, dispatcher: &mut Dispatcher) {
    if let Some((state, ctx)) = dispatcher.get_state(&replicas[0]) {
        assert_eq!(ctx.epoch, 2);
        assert_eq!(state, "primary");
        assert_eq!(ctx.view, 0);
    } else {
        assert!(false)
    }
    if let Some((state, ctx)) = dispatcher.get_state(&replicas[1]) {
        assert_eq!(ctx.epoch, 2);
        assert_eq!(state, "backup");
        assert_eq!(ctx.view, 0);
    } else {
        assert!(false)
    }
    if let Some((state, ctx)) = dispatcher.get_state(&replicas[2]) {
        assert_eq!(ctx.epoch, 2);
        assert_eq!(state, "backup");
        assert_eq!(ctx.view, 0);
    } else {
        assert!(false)
    }
}

fn dispatch_with_drop(dispatcher: &mut Dispatcher, drop_replica: &Replica) {
    while let Ok(Envelope {to, msg}) = dispatcher.try_recv() {
        if to != *drop_replica {
            dispatcher.send(&to, msg);
        }
    }
}

fn assert_new_primary_returns_last_committed_result(primary: &Replica,
                                                    client_id: &Uuid,
                                                    dispatcher: &mut Dispatcher) {
    let get_op = VrApiReq::Get { path: "/test_root".to_string(), cas: false };
    let get_msg = VrMsg::ClientRequest {op: get_op, client_id: client_id.clone(), request_num: 3};
    dispatcher.send(primary, get_msg);

    // Ensure we get back the correct response. Note that since we didn't call
    // dispatcher.dispatch_all_received_messsages() we ensure that this response was stored in the
    // client table and sent directly from the primary without contacting other replicas.
    let ClientReplyEnvelope{to, msg} = dispatcher.try_recv_client_reply().unwrap();
    assert_eq!(to, *client_id);
    if let VrMsg::ClientReply{epoch, view, request_num, value} = msg {
        assert_eq!(epoch, 1);
        assert_eq!(view, 2);
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

fn assert_second_view_change(replicas: &Vec<Replica>, dispatcher: &mut Dispatcher) {
    dispatcher.send(&replicas[0], VrMsg::Tick);
    dispatcher.dispatch_all_received_messages();
    if let Some((state, ctx)) = dispatcher.get_state(&replicas[0]) {
        assert_eq!(state, "backup");
        assert_eq!(ctx.view, 2);
        assert_eq!(ctx.op, 3);
    } else {
        assert!(false)
    }
    if let Some((state, ctx)) = dispatcher.get_state(&replicas[1]) {
        assert_eq!(state, "backup");
        assert_eq!(ctx.view, 2);
        assert_eq!(ctx.op, 3);
    } else {
        assert!(false)
    }
    if let Some((state, ctx)) = dispatcher.get_state(&replicas[2]) {
        assert_eq!(state, "primary");
        assert_eq!(ctx.view, 2);
        assert_eq!(ctx.op, 3);
    } else {
        assert!(false)
    }
}

fn assert_commit_gets_sent(primary: &Replica, dispatcher: &mut Dispatcher, expected_commit: u64) {
    dispatcher.send(primary, VrMsg::Tick);

    // Ensure the primary sends the commit to the 2 backups
    for _ in 0..2 {
        let Envelope {to, msg} = dispatcher.try_recv().unwrap();
        let msg2 = msg.clone();
        if let VrMsg::Commit {view, commit_num, ..} = msg {
            assert_eq!(view, 1);
            assert_eq!(commit_num, expected_commit);
            // We re-dispatch the commit so that all replicas know the latest commit val. This is
            // necessary only for testing, as then we don't have to worry about receiving old commits
            // when trying to call dispatcher.try_recv_client_reply() after a new primary is
            // elected.
            dispatcher.send(&to, msg2);
        } else {
            assert!(false);
        }
    }

    dispatcher.dispatch_all_received_messages();
}

fn assert_put_and_get(primary: &Replica, client_id: &Uuid, dispatcher: &mut Dispatcher) {
    let put_op = VrApiReq::Put { path: "/test_root".to_string(), data: b"hello".to_vec(), cas_tag: None};
    let put_msg = VrMsg::ClientRequest {op: put_op, client_id: client_id.clone(), request_num: 2};
    dispatcher.send(primary, put_msg);
    let get_op = VrApiReq::Get { path: "/test_root".to_string(), cas: false };
    let get_msg = VrMsg::ClientRequest {op: get_op, client_id: client_id.clone(), request_num: 3};
    dispatcher.send(primary, get_msg);
    dispatcher.dispatch_all_received_messages();

    // Skip over the put response
    dispatcher.try_recv_client_reply().unwrap();
    // Ensure we get back the value stored
    let ClientReplyEnvelope{to, msg} = dispatcher.try_recv_client_reply().unwrap();
    assert_eq!(to, *client_id);
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

fn assert_create_response(client_id: &Uuid, dispatcher: &Dispatcher) {
    if let Ok(ClientReplyEnvelope{to, msg}) = dispatcher.try_recv_client_reply() {
        assert_eq!(to, *client_id);
        if let VrMsg::ClientReply{request_num, value, ..} = msg {
            assert_eq!(request_num, 1);
            assert_eq!(value, VrApiRsp::Ok);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }
}

fn elect_initial_leader(dispatcher: &mut Dispatcher, replicas: &Vec<Replica>) {
    // Force an inactivity timeout on a single replica.
    dispatcher.send(&replicas[0], VrMsg::Tick);

    // Let all state transition messages get sent. Note that this won't infinitely block because we
    // control ticks from the test process and don't automatically generate messages to send to
    // replicas.
    dispatcher.dispatch_all_received_messages();

    // Show that there is a single primary and two backups. Also show that for view 1, the primary
    // is the second replica (r2). This is because we round robin the replicas using view numbers as
    // described in the papper, and view 0 is the first replica, although there is never an actual
    //  primary in view 0.
    if let Some((state, ctx)) = dispatcher.get_state(&replicas[0]) {
        assert_eq!(state, "backup");
        assert_eq!(ctx.view, 1);
    } else {
        assert!(false)
    }
    if let Some((state, ctx)) = dispatcher.get_state(&replicas[1]) {
        assert_eq!(state, "primary");
        assert_eq!(ctx.view, 1);
    } else {
        assert!(false)
    }
    if let Some((state, ctx)) = dispatcher.get_state(&replicas[2]) {
        assert_eq!(state, "backup");
        assert_eq!(ctx.view, 1);
    } else {
        assert!(false)
    }
}


fn init_tenant() -> (Dispatcher, Vec<Replica>) {
    let node = Member {
        name: "node1".to_string(),
        cluster: "test".to_string(),
        ip: "127.0.0.1:5000".to_string()
    };
    let mut dispatcher = Dispatcher::new(node.clone());
    // Set the timeouts to zero to preclude the need for sleeps
    dispatcher.set_idle_timeout_ms(0);

    let raw_replicas = vec![RawReplica {name: "r1".to_string(), node: node.clone()},
                            RawReplica {name: "r2".to_string(), node: node.clone()},
                            RawReplica {name: "r3".to_string(), node: node.clone()}];

    let tenant = dispatcher.create_tenant(raw_replicas.clone(), FsmType::Local);
    (dispatcher, raw_replicas.iter().cloned().map(|r| Replica::new(tenant, r)).collect())
}
