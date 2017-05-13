// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
extern crate quickcheck;
extern crate uuid;
extern crate rand;
extern crate haret;
extern crate rabble;
extern crate time;

#[macro_use]
extern crate funfsm;

extern crate assert_matches;

#[macro_use]
extern crate slog;
extern crate slog_stdlog;
extern crate slog_term;
extern crate slog_envlogger;

extern crate vertree;

#[macro_use]
mod utils;

use utils::{vr_invariants, op_invariants};
use utils::scheduler::Scheduler;
use utils::arbitrary::{Op, ClientRequest};
use haret::vr::{VrMsg, VrApiReq, VrEnvelope, TreeOp};

/// Test that a fixed replica set (no reconfigurations) properly runs VR
/// operations
quickcheck! {
    fn prop_static_membership(ops: Vec<Op>) -> bool {
        let mut scheduler = Scheduler::new(3);
        scheduler.elect_initial_leader();
        let mut client_req_num = 0;
        for op in ops {
            if let Err(s) = assert_op(op.clone(), &mut scheduler, &mut client_req_num) {
                let errmsg = format!("{:?} Error: {}", op, s);
                error!(scheduler.logger, errmsg);
                println!("Crashed nodes: {:#?}", scheduler.crashed_nodes);
                println!("States: {:#?}", scheduler.get_states());
                return false;
            }
        }
        true
    }
}

fn assert_op(op: Op, scheduler: &mut Scheduler, client_req_num: &mut u64) -> Result<(), String> {
    match op {
        Op::ClientRequest(ClientRequest(VrMsg::ClientRequest { op, client_id, .. })) => {
            // Client requests are generated with invalid client request nums
            *client_req_num += 1;
            let req = VrMsg::ClientRequest {
                request_num: *client_req_num,
                op: op,
                client_id: client_id
            };
            let mut replies = scheduler.send_to_primary(req.clone())?;
            if replies.len() == 1 {
                return assert_client_request_correctness(&scheduler, req, replies.pop().unwrap());
            }
            safe_assert_eq!(replies.len(), 0)
        }
        Op::ClientRequest(r) => {
            return Err(format!("Invalid client request: {:#?}", r));
        }
        Op::Commit => {
            scheduler.send_to_primary(VrMsg::Tick)?;
            assert_basic_correctness(scheduler)
        }
        Op::ViewChange => {
            scheduler.send_to_backup(VrMsg::Tick)?;
            assert_basic_correctness(scheduler)
        }
        Op::CrashBackup => {
            scheduler.stop_backup();
            assert_basic_correctness(scheduler)
        }
        Op::CrashPrimary => {
            scheduler.stop_primary();
            assert_basic_correctness(scheduler)
        }
        Op::Restart => {
            scheduler.restart_crashed_node()?;
            assert_basic_correctness(scheduler)
        }
    }
}

/// Assert that all correctness conditions are maintained after each client
/// request to the group
fn assert_client_request_correctness(scheduler: &Scheduler,
                                     request: VrMsg,
                                     reply: VrEnvelope)
                                     -> Result<(), String> {
    assert_response_matches_internal_replica_state(scheduler, request, reply)?;
    assert_vr_invariants(scheduler)
}

/// Assert that we maintain correctness conditions not relating to a client
/// request
fn assert_basic_correctness(scheduler: &Scheduler) -> Result<(), String> {
    assert_vr_invariants(scheduler)
}

fn assert_vr_invariants(scheduler: &Scheduler) -> Result<(), String> {
    let quorum = scheduler.quorum();
    let states = scheduler.get_states();
    vr_invariants::assert_single_primary_per_epoch_view(&states)?;
    vr_invariants::assert_minority_of_nodes_recovering(quorum, &states)?;
    vr_invariants::assert_quorum_of_logs_equal_up_to_smallest_commit(quorum, &states)
}


fn assert_response_matches_internal_replica_state(scheduler: &Scheduler,
                                                  request: VrMsg,
                                                  reply: VrEnvelope)
                                                  -> Result<(), String> {
    match request {
        VrMsg::ClientRequest { op: VrApiReq::TreeOp(TreeOp::CreateNode { .. }), .. } => {
            op_invariants::assert_create_response(scheduler, request, reply)
        }
        VrMsg::ClientRequest { op: VrApiReq::TreeOp(TreeOp::BlobPut { .. }), .. } => {
            op_invariants::assert_put_response(scheduler, request, reply)
        }
        VrMsg::ClientRequest { op: VrApiReq::TreeOp(TreeOp::BlobGet { .. }), .. } => {
            op_invariants::assert_get_response(scheduler, request, reply)
        }
        _ => fail!(),
    }
}
