extern crate uuid;
extern crate rand;
extern crate v2r2;
extern crate time;
extern crate msgpack;
extern crate rustc_serialize;

#[macro_use]
extern crate fsm;

#[macro_use]
extern crate distill;

#[macro_use]
extern crate assert_matches;

mod utils;

pub mod debugger_shared;

use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};
use uuid::Uuid;
use distill::{Test, Distill};
use utils::{vr_invariants, op_invariants, Model};
use debugger_shared::{Scheduler, SymbolicOp, DynamicOp};
use utils::generators::{oneof, paths, session_ids};
use v2r2::vr::{VrMsg, VrCtx, ElementType, VrApiReq, ClientReplyEnvelope};

struct VrTest {
    scheduler: Scheduler,
    session_ids: Vec<Uuid>,
    paths: Vec<&'static str>,
    model: Model,
    total_client_requests: u64
}

impl VrTest {
    fn new() -> VrTest {
        let mut scheduler = Scheduler::new(3);
        let view_change_op = scheduler.elect_initial_leader();
        let replicas = scheduler.new_config.replicas.clone();
        VrTest {
            scheduler: scheduler,
            session_ids: session_ids(1),
            paths: paths(),
            model: Model::new(replicas, view_change_op),
            total_client_requests: 0
        }
    }
}

impl Test for VrTest {
    type SymbolicOp = SymbolicOp;
    type DynamicOp = DynamicOp;
    type State = Vec<(&'static str, VrCtx)>;
    type Model = Model;

    /// Reset the replica group state to view 1 with r2 as a leader.
    fn reset(&mut self) {
        self.total_client_requests = 0;
        self.scheduler = Scheduler::new(3);
        let view_change_op = self.scheduler.elect_initial_leader();
        let replicas = self.scheduler.new_config.replicas.clone();
        self.model = Model::new(replicas, view_change_op);
    }

    fn gen(&mut self, n: usize) -> Vec<Self::SymbolicOp> {
        let range = Range::new(0, 100);
        (0..n).map(|_| {
            match range.ind_sample(&mut thread_rng()) {
                0...80 => SymbolicOp::ClientRequest,
                80...85 => SymbolicOp::Commit,
                85...90 => SymbolicOp::ViewChange,
                90...95 => SymbolicOp::Crash,
                _ => SymbolicOp::Restart
            }
        }).collect()
    }

    fn update(&mut self, symbolic_op: &SymbolicOp) -> Option<DynamicOp> {
        let dynamic_op = match *symbolic_op {
            SymbolicOp::ClientRequest => {
                if self.model.primary.is_none() { return None }
                self.total_client_requests += 1;
                let req = gen_client_request(&self.session_ids, &self.paths, self.total_client_requests);
                Some(DynamicOp::ClientRequest(self.model.primary.as_ref().unwrap().clone(), req))
            },
            SymbolicOp::Commit => {
                if self.model.primary.is_none() { return None }
                Some(DynamicOp::Commit(self.model.primary.as_ref().unwrap().clone()))
            },
            SymbolicOp::ViewChange => {
                // Reset the session ids as would happend with client tcp
                // disconnect/reconnect on primary change over
                self.session_ids = session_ids(self.session_ids.len());
                Some(DynamicOp::ViewChange(self.model.choose_live_backup()))
            },
            SymbolicOp::Crash => {
                match self.model.choose_live_replica() {
                    None => None,
                    Some(replica) => {
                        if let Some(ref primary) = self.model.primary {
                            if replica == *primary {
                                // Reset the session ids as would happend with client tcp
                                // disconnect/reconnect on primary change over
                                self.session_ids = session_ids(self.session_ids.len());
                            }
                        }
                        Some(DynamicOp::Crash(replica))
                    }
                }
            },
            SymbolicOp::Restart => {
                match self.model.choose_crashed_replica() {
                    None => None,
                    Some(replica) => Some(DynamicOp::Restart(replica))
                }
            },
            SymbolicOp::Reconfiguration => unreachable!()
        };
        dynamic_op.map(|op| {
            self.model.update(&op);
            op
        })
    }

    fn run(&mut self, op: &DynamicOp) -> Result<(), String> {
        match *op {
            DynamicOp::ClientRequest(ref primary, ref vrmsg) => {
                let mut replies = self.scheduler.send(primary, vrmsg.clone());
                assert_eq!(replies.len(), 1);
                assert_client_request_correctness(self, vrmsg.clone(), replies.pop().unwrap())
            },
            DynamicOp::Commit(ref primary) => {
                self.scheduler.send(primary, VrMsg::Tick);
                assert_basic_correctness(self)
            },
            DynamicOp::ViewChange(ref backup) => {
                self.scheduler.send(backup, VrMsg::Tick);
                assert_basic_correctness(self)
            },
            DynamicOp::Crash(ref replica) => {
                self.scheduler.stop(replica);
                assert_basic_correctness(self)
            },
            DynamicOp::Restart(ref replica) => {
                self.scheduler.restart(replica);
                assert_basic_correctness(self)
            },
            DynamicOp::Reconfiguration(_) => Ok(())
        }
    }

    fn get_state(&self) -> Option<Vec<(&'static str, VrCtx)>> {
        let mut states = Vec::new();
        for r in &self.model.live_replicas {
            states.push(self.scheduler.get_state(r).unwrap());
        }
        Some(states)
    }

    fn get_model(&self) -> Option<Model> {
        Some(self.model.clone())
    }
}

/// Test a single fixed membership replica group
#[test]
fn stable_group() {
    let test = VrTest::new();
    let mut distill = Distill::new("static_membership", test);
    distill.run(1000);
}

/// Assert that all correctness conditions are maintained after each client request to the group
fn assert_client_request_correctness(test: &mut VrTest,
                                     request: VrMsg,
                                     reply: ClientReplyEnvelope) -> Result<(), String> {
    try!(assert_response_matches_internal_replica_state(test, request, reply));
    try!(assert_vr_invariants(test));
    test.model.check(&test.scheduler)
}

/// Assert that we maintain correctness conditions not relating to a client request
fn assert_basic_correctness(test: &mut VrTest) -> Result<(), String> {
    try!(assert_vr_invariants(test));
    test.model.check(&test.scheduler)
}

fn assert_vr_invariants(test: &mut VrTest) -> Result<(), String> {
    let mut states = Vec::new();
    for r in &test.model.live_replicas {
        let state_tuple = test.scheduler.get_state(r).unwrap();
        states.push(state_tuple);
    }
    let quorum = test.model.replicas.len() / 2 + 1;
    try!(vr_invariants::assert_single_primary_per_epoch_view(&states));
    try!(vr_invariants::assert_minority_of_nodes_recovering(quorum, &states));
    vr_invariants::assert_quorum_of_logs_equal_up_to_smallest_commit(quorum, &states)
}


fn assert_response_matches_internal_replica_state(test: &mut VrTest,
                                                  request: VrMsg,
                                                  reply: ClientReplyEnvelope) -> Result<(), String> {
    let primary = test.model.primary.clone().unwrap();
    match request {
        VrMsg::ClientRequest {op: VrApiReq::Create{..}, ..} =>
            op_invariants::assert_create_response(&test.model.replicas,
                                                  &test.scheduler,
                                                  &primary,
                                                  request,
                                                  reply),
        VrMsg::ClientRequest {op: VrApiReq::Put{..}, ..} =>
            op_invariants::assert_put_response(&test.model.replicas,
                                               &test.scheduler,
                                               &primary,
                                               request,
                                               reply),
        VrMsg::ClientRequest {op: VrApiReq::Get{..}, ..} =>
            op_invariants::assert_get_response(&test.model.replicas,
                                               &test.scheduler,
                                               &primary,
                                               request,
                                               reply),
        _ => fail!()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//  GENERATOR FUNCTIONS
///////////////////////////////////////////////////////////////////////////////////////////////////
fn gen_client_request(session_ids: &Vec<Uuid>, paths: &Vec<&'static str>, n: u64) -> VrMsg {
    VrMsg::ClientRequest {
        session_id: oneof(session_ids),
        request_num: n,
        op: gen_op(paths)
    }
}

fn gen_op(paths: &Vec<&'static str>) -> VrApiReq {
    let rng = &mut thread_rng();
    let range = Range::new(0, 3);
    match range.ind_sample(rng) {
        0 => VrApiReq::Create {path: oneof(paths).to_string(), ty: ElementType::Binary},
        1 => VrApiReq::Put {path: oneof(paths).to_string(),
                            data: b"hello".to_vec(),
                            cas_tag: None},
        _ => VrApiReq::Get {path: oneof(paths).to_string(), cas: false}
    }
}

