// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::error::Error;
use std::fmt;
use std::io;
use std::io::Read;
use std::fs::File;
use std::fmt::Write;
use rustc_serialize::Decodable;
use msgpack::Decoder;
use fsm::Fsm;
use haret::vr::{Replica, VrCtx, Envelope, VrTypes};
use debugger_shared::{Scheduler, DynamicOp};

// Tracks the state of the replicas and dispatchers between Frames
// Note that frame_state is only set once during the call to initial run.
pub struct State {
    frame_state: Scheduler,
    step_state: Scheduler,
    step_count: usize,
    total_steps_in_frame: Option<usize>
}

impl State {
    fn new(scheduler: Scheduler) -> State {
        State {
            frame_state: scheduler.clone(),
            step_state: scheduler,
            step_count: 0,
            total_steps_in_frame: None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Status {
    pub frame_count: usize,
    pub step_count: usize,
    pub current_op: Option<DynamicOp>,
    pub next_op: Option<DynamicOp>,
    pub last_received_envelope : Option<Envelope>
}

pub struct Debugger {
    scheduler: Scheduler,
    schedule: Vec<DynamicOp>,
    history: Vec<State>,
    frame_count: usize,
    last_received_envelope: Option<Envelope>,
    diff_start: Option<(usize, usize, Scheduler)>
}

impl Debugger {
    pub fn new() -> Debugger {
        let mut scheduler = Scheduler::new(3);
        scheduler.elect_initial_leader();
        Debugger {
            scheduler: scheduler,
            schedule: Vec::new(),
            history: Vec::new(),
            frame_count: 0,
            last_received_envelope: None,
            diff_start: None
        }
    }

    pub fn load_schedule(&mut self, filename: &str) -> Result<(), DbgError> {
        let mut file = File::open(filename)?;
        let mut encoded = Vec::new();
        file.read_to_end(&mut encoded)?;
        let mut decoder = Decoder::new(&encoded[..]);
        match Decodable::decode(&mut decoder) {
            Ok(schedule) => self.schedule = schedule,
            Err(_) => return Err(DbgError::Io("Improperly encoded msgpack data".to_string()))
        }
        self.initial_run();
        Ok(())
    }

    // Just do an initial run on startup to get the complete history. It's easier to do this than to
    // track which parts are already computed.
    fn initial_run(&mut self) {
        self.history.push(State::new(self.scheduler.clone()));
        for op in &self.schedule {
            self.scheduler.run(op);
            self.history.push(State::new(self.scheduler.clone()));
            self.frame_count += 1;
        }
    }

    pub fn get_status(&self) -> Status {
        let current_op = if self.frame_count >= self.schedule.len() {
            None
        } else {
            Some(self.schedule[self.frame_count].clone())
        };

        let next_op = if self.frame_count >= self.schedule.len() - 1 {
            None
        } else {
            Some(self.schedule[self.frame_count + 1].clone())
        };

        Status {
            frame_count: self.frame_count,
            step_count: self.current_step(),
            current_op: current_op,
            next_op: next_op,
            last_received_envelope: self.last_received_envelope.clone()
        }
    }

    pub fn jump_forward(&mut self) {
        // Since we can intermix jumps and steps we need to reset the pre-jump state on each jump
        self.reset_step_state();
        if self.frame_count == self.history.len() - 1 { return; }
        self.frame_count += 1;
    }

    pub fn jump_backward(&mut self) {
        // Since we can intermix jumps and steps we need to reset the pre-jump state on each jump
        self.reset_step_state();
        if self.frame_count == 0 { return; }
        self.frame_count -= 1;
        self.maybe_clear_diff_start();
    }

    pub fn step_forward(&mut self) {
        if self.frame_count == self.history.len() - 1 { return; }
        // Load the frame state if we are just beginning to step
        if self.current_step() == 0 {
            self.step_forward_new_frame();
        } else {
            match self.scheduler.next() {
                Some(envelope) => {
                    self.scheduler.send_once(envelope.clone());
                    self.step_forward_same_frame(envelope);
                },
                None => {
                    // We have dispatched all messages within this frame
                    // Reset the last frame's state and step into the next one
                    self.reset_step_state();
                    self.frame_count += 1;
                }
            }
        }
    }

    /// We don't actually physically step backward. We backup to the last frame, load the memory from
    /// the saved history, then step forward from there. This is more computationally expensive, but
    /// prevents unbounded memory growth for long histories. One consequence of this is that steping
    /// backward over frame boundaries requires stepping forward until there are no more messages in
    /// the prior frame, then recording the count of the last message and stepping forward again. We
    /// memoize this number so that we only have to do this once per frame.
    pub fn step_backward(&mut self) {
        if self.frame_count == 0 { return; }

        if self.current_step() == 0 {
            self.step_backward_prev_frame();
        } else {
            // Go back to the beginning of the frame
            let current_step = self.current_step();
            self.reset_step_state();
            // Step forward
            for _ in 0..current_step-1 {
                self.step_forward();
            }
        }
    }

    fn step_backward_prev_frame(&mut self) {
        self.frame_count -= 1;
        self.reset_step_state();
        // We already know how many steps are in this frame
        if let Some(total_steps) = self.history[self.frame_count].total_steps_in_frame {
            for _ in 0..total_steps {
                self.step_forward();
            }
        } else {
            // Compute and save how many steps are in this frame
            self.step_backward_prev_frame_and_learn_step_count();
        }
    }

    fn step_backward_prev_frame_and_learn_step_count(&mut self) {
        let mut total_steps = 0;
        let current_frame = self.frame_count;
        loop {
            self.step_forward();
            if current_frame == self.frame_count {
                total_steps += 1;
            } else {
                self.history[current_frame].total_steps_in_frame = Some(total_steps);
                self.step_backward_prev_frame();
                break;
            }
        }
    }

    fn step_forward_new_frame(&mut self) {
        self.reset_step_state();
        let current_state = self.current_state().clone();
        self.scheduler = current_state.clone();
        let ref op = self.schedule[self.frame_count];
        self.scheduler.run(op);
        let state = &mut self.history[self.frame_count];
        state.step_count += 1;
        state.step_state = self.scheduler.clone();
    }

    fn step_forward_same_frame(&mut self, envelope: Envelope) {
        self.last_received_envelope = Some(envelope);
        let state = &mut self.history[self.frame_count];
        state.step_count +=1;
        state.step_state = self.scheduler.clone();
    }

    fn reset_step_state(&mut self) {
        self.last_received_envelope = None;
        let state = &mut self.history[self.frame_count];
        state.step_count = 0;
        state.step_state = state.frame_state.clone();
    }

    pub fn start_diff(&mut self) {
        self.diff_start = Some((self.frame_count, self.current_step(), self.current_state().clone()));
    }

    pub fn diff(&self, replica: &Replica) -> Result<String, &'static str> {
        if self.diff_start.is_none() { return Err("Error: Please start a diff"); }
        match self.diff_start.as_ref().unwrap().2.fsms.get(&replica) {
            None => {
                match self.current_state().fsms.get(&replica) {
                    None => Err("Error: Replica not found"),
                    Some(_) => Ok("Replica was added to group".to_string())
                }
            },
            Some(old) => {
                match self.current_state().fsms.get(&replica) {
                    None => Ok("Replica was removed from group".to_string()),
                    Some(new) => Ok(diff_replicas(old, new))
                }
            }
        }
    }

    pub fn replica_names(&self) -> Vec<String> {
        let ref dispatcher_state = self.current_state();
        let mut names = Vec::new();
        for (r, _)  in dispatcher_state.fsms.iter() {
            let mut s = r.name.clone();
            s.push_str("::");
            s.push_str(&r.node.name);
            names.push(s);
        }
        names
    }

    pub fn replica_state(&self, replica: &Replica) -> Option<(&'static str, &VrCtx)> {
        println!("Checking replica state for {:?}", replica);
        let ref dispatcher_state = self.current_state();
        match dispatcher_state.fsms.get(replica) {
            None => None,
            Some(fsm) => Some((fsm.state.0, &fsm.ctx))
        }
    }

    fn current_state(&self) -> &Scheduler {
        &self.history[self.frame_count].step_state
    }

    fn current_step(&self) -> usize {
        self.history[self.frame_count].step_count
    }

    fn maybe_clear_diff_start(&mut self) {
        if let Some((frame_count, step_count, _)) = self.diff_start {
            // Reset the diff_start since we don't want to do negative diffs
            if frame_count > self.frame_count {
                self.diff_start = None;
            } else if frame_count == self.frame_count {
                if step_count > self.current_step() {
                    self.diff_start = None;
                }
            }
        }
    }
}

fn diff_replicas(old: &Fsm<VrTypes>, new: &Fsm<VrTypes>) -> String {
    let mut diff = String::new();
    if old.state.0 != new.state.0 {
        let _ = writeln!(&mut diff, "State changed from {} to {}", old.state.0, new.state.0);
    }
    if old.ctx.primary != new.ctx.primary {
        let _ = write!(&mut diff, "Primary changed:\n    Old: {:?}\n    New: {:?}\n",
               old.ctx.primary, new.ctx.primary);
    }
    if old.ctx.epoch != new.ctx.epoch {
        let _ = writeln!(&mut diff, "Epoch changed from {} to {}", old.ctx.epoch, new.ctx.epoch);
    }
    if old.ctx.view != new.ctx.view {
        let _ = writeln!(&mut diff, "View changed from {} to {}", old.ctx.view, new.ctx.view);
    }
    if old.ctx.op != new.ctx.op {
        let _ = writeln!(&mut diff, "Op Number changed from {} to {}", old.ctx.op, new.ctx.op);
    }
    if old.ctx.commit_num != new.ctx.commit_num {
        let _ = writeln!(&mut diff, "Commit Number changed from {} to {}",
                 old.ctx.commit_num, new.ctx.commit_num);
    }
    if old.ctx.last_received_time != new.ctx.last_received_time {
        let _ = writeln!(&mut diff, "Last received time changed");
    }
    if old.ctx.last_normal_view != new.ctx.last_normal_view {
        let _ = writeln!(&mut diff, "Last normal view changed from {} to {}",
                 old.ctx.last_normal_view, new.ctx.last_normal_view);
    }
    if old.ctx.quorum != new.ctx.quorum {
        let _ = writeln!(&mut diff, "Quorum requirements changed from {} replicas to {} replicas",
                 old.ctx.quorum, new.ctx.quorum);
    }
    if old.ctx.log != new.ctx.log {
        // TODO: Show differing entries
        let _ = writeln!(&mut diff, "The log has changed");
    }
    if old.ctx.backend != new.ctx.backend {
        // TODO: Show added, removed, changed nodes
        let _ = writeln!(&mut diff, "The backend has changed");
    }
    if old.ctx.old_config != new.ctx.old_config {
        // TODO: show differences
        let _ = writeln!(&mut diff, "Old configuration has changed");
    }
    if old.ctx.new_config != new.ctx.new_config {
        // TODO: show differences
        let _ = writeln!(&mut diff, "New configuration has changed");
    }
    if old.ctx.session_table != new.ctx.session_table {
        let _ = writeln!(&mut diff, "Client table has changed");
    }
    if old.ctx.recovery_nonce != new.ctx.recovery_nonce {
        let _ = writeln!(&mut diff, "Recovery nonce has changed from {:?} to {:?}",
                 old.ctx.recovery_nonce, new.ctx.recovery_nonce);
    }
    if old.ctx.recovery_primary != new.ctx.recovery_primary {
        let _ = write!(&mut diff, "Recovery primary changed:\n    Old: {:?}\n    New: {:?}\n",
               old.ctx.recovery_primary, new.ctx.recovery_primary);
    }
    if diff.len() == 0 { return "No difference found".to_string(); }
    diff
}

#[derive(Debug, Clone)]
pub enum DbgError {
    Io(String)
}

impl Error for DbgError {
    fn description(&self) -> &str {
        match *self {
            DbgError::Io(ref string) => string,
        }
    }
}

impl fmt::Display for DbgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DbgError::Io(ref string) => write!(f, "IO error: {}", string),
        }
    }
}

/// Need to implement From so we can use try!
impl From<io::Error> for DbgError {
    fn from(err: io::Error) -> DbgError {
        let s = format!("{}", err);
        DbgError::Io(s)
    }
}
