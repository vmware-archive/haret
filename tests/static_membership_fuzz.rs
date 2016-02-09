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

mod utils;

pub mod debugger_shared;

use rand::{thread_rng, ThreadRng};
use rand::distributions::{IndependentSample, Range};
use uuid::Uuid;
use distill::{Test, Distill};
use utils::{vr_invariants, op_invariants, Model};
use debugger_shared::{Scheduler, TestMsg};
use utils::generators::{oneof, paths, session_ids};
use v2r2::vr::{VrMsg, VrCtx, ElementType, VrApiReq, ClientReplyEnvelope};

// These constants should all add up to 100%
const CLIENT_REQUEST_PCT: u8 = 80;
const COMMIT_PCT: u8 = 5;
const VIEW_CHANGE_PCT: u8 = 5;
const CRASH_PCT: u8 = 5;
const RESTART_PCT: u8 = 5;

const COMMIT_TOP: u8 = CLIENT_REQUEST_PCT + COMMIT_PCT;
const VIEW_CHANGE_TOP: u8 = COMMIT_TOP + VIEW_CHANGE_PCT;
const CRASH_TOP: u8 = VIEW_CHANGE_TOP + 5;

struct VrTest {
    scheduler: Scheduler,
    session_ids: Vec<Uuid>,
    paths: Vec<&'static str>,
    rng: ThreadRng,
    model: Model
}

impl VrTest {
    fn new() -> VrTest {
        assert_eq!(100, CLIENT_REQUEST_PCT +
                        COMMIT_PCT +
                        VIEW_CHANGE_PCT +
                        CRASH_PCT +
                        RESTART_PCT);
        let mut scheduler = Scheduler::new(3);
        scheduler.elect_initial_leader();
        let replicas = scheduler.new_config.replicas.clone();
        VrTest {
            scheduler: scheduler,
            session_ids: session_ids(1),
            paths: paths(),
            rng: thread_rng(),
            model: Model::new(replicas, 1)
        }
    }

    fn choose_request(&mut self, range: &mut Range<u8>, n: u64) -> Option<TestMsg> {
        match range.ind_sample(&mut self.rng) {
            0...CLIENT_REQUEST_PCT => {
                let req = gen_client_request(&mut self.rng, &self.session_ids, &self.paths, n);
                Some(TestMsg::ClientRequest(req))
            },
            CLIENT_REQUEST_PCT...COMMIT_TOP => Some(TestMsg::Commit),
            COMMIT_TOP...VIEW_CHANGE_TOP => {
                // Reset the session_ids (really session IDs when the view changes). In production
                // this would happen with tcp disconnect/reconnect.
                self.session_ids = session_ids(self.session_ids.len());
                Some(TestMsg::ViewChange(self.model.choose_live_backup()))
            },
            VIEW_CHANGE_TOP...CRASH_TOP => {
                match self.model.choose_live_replica() {
                    // We only crash at maximim a minority of replicas because otherwise we end up in
                    // an unsupported configuration with dataloss. In this case just try to generate
                    // a different message.
                    None => None,
                    Some(replica) => {
                        if let Some(ref primary) = self.model.primary {
                            if replica == *primary {
                                // Reset the session_ids (really session IDs when the view changes). In production
                                // this would happen with tcp disconnect/reconnect.
                                self.session_ids = session_ids(self.session_ids.len());
                            }
                        }
                        Some(TestMsg::Crash(replica, Uuid::new_v4()))
                    }
                }
            },
            _ => {
                match self.model.choose_crashed_replica() {
                    None => None,
                    Some((replica, uuid)) => Some(TestMsg::Restart(replica, uuid))
                }
            }
        }
    }

    /// We can only send ticks to a backup or restart a crashed replica when there is no primary
    fn choose_request_no_primary(&mut self) -> TestMsg{
        let range = Range::new(0, 2);
        match range.ind_sample(&mut self.rng) {
            0 => TestMsg::ViewChange(self.model.choose_live_backup()),
            1 => match self.model.choose_crashed_replica() {
                None => TestMsg::ViewChange(self.model.choose_live_backup()),
                Some((replica, uuid)) => TestMsg::Restart(replica, uuid)
            },
            _ => unreachable!()
        }
    }
}

impl Test for VrTest {
    type Msg = TestMsg;
    type State = Vec<(&'static str, VrCtx)>;
    type Model = Model;

    /// Reset the replica group state to view 1 with r2 as a leader.
    fn reset(&mut self, record: bool) {
        self.scheduler = Scheduler::new(3);
        self.scheduler.elect_initial_leader();
        let replicas = self.scheduler.new_config.replicas.clone();
        let view = 1;
        self.model = Model::new(replicas, view);
        if record {
            self.scheduler.record()
        }
    }

    fn generate(&mut self, n: u64) -> TestMsg {
        match self.model.primary {
            None => self.choose_request_no_primary(),
            Some(_) => {
                let mut range = Range::new(0, 100);
                loop {
                    match self.choose_request(&mut range, n) {
                        None => (),
                        Some(msg) => return msg
                    }
                }
            }
        }
    }

    fn update(&mut self, request: TestMsg) {
        self.model.update(request);
    }

    fn drop_msg(&self, request: &TestMsg) -> bool {
        self.model.should_drop(request)
    }

    fn run(&mut self, request: TestMsg) -> Result<(), String> {
        let test_msg = request.clone();
        match request {
            TestMsg::ClientRequest(vrmsg) => {
                let mut replies = self.scheduler.send(test_msg,
                                                      &self.model.primary.clone().unwrap(),
                                                      vrmsg.clone());
                assert_eq!(replies.len(), 1);
                assert_client_request_correctness(self, vrmsg, replies.pop().unwrap())
            },
            TestMsg::Commit => {
                self.scheduler.send(test_msg, &self.model.primary.clone().unwrap(), VrMsg::Tick);
                assert_basic_correctness(self)
            },
            TestMsg::ViewChange(backup) => {
                self.scheduler.send(test_msg, &backup, VrMsg::Tick);
                assert_basic_correctness(self)
            },
            TestMsg::Crash(replica, _) => {
                self.scheduler.stop(test_msg, &replica);
                assert_basic_correctness(self)
            },
            TestMsg::Restart(replica, _) => {
                self.scheduler.restart(test_msg, &replica);
                assert_basic_correctness(self)
            },
            TestMsg::Reconfiguration(_vrmsg) => Ok(()),
            TestMsg::Null => Ok(())
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

    fn get_schedule(&self) -> Option<Vec<u8>> {
        Some(self.scheduler.serialize_history())
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
fn gen_client_request(rng: &mut ThreadRng, session_ids: &Vec<Uuid>, paths: &Vec<&'static str>, n: u64) -> VrMsg {
    VrMsg::ClientRequest {
        session_id: oneof(rng, session_ids),
        request_num: n,
        op: gen_op(rng, paths)
    }
}

fn gen_op(rng: &mut ThreadRng, paths: &Vec<&'static str>) -> VrApiReq {
    let range = Range::new(0, 3);
    match range.ind_sample(rng) {
        0 => VrApiReq::Create {path: oneof(rng, paths).to_string(), ty: ElementType::Binary},
        1 => VrApiReq::Put {path: oneof(rng, paths).to_string(),
                            data: b"hello".to_vec(),
                            cas_tag: None},
        2 => VrApiReq::Get {path: oneof(rng, paths).to_string(), cas: false},
        _ => unreachable!()
    }
}

