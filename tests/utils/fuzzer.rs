//! Fuzz testing driver used specifically to drive the testing of VR.

use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use time;
use uuid::Uuid;
use debugger_shared::CausalMsg;

/// An assert that doesn't panic on failure, but instead returns a result<(), String> with an
/// appropriate error message.
#[macro_export]
macro_rules! safe_assert {
    () => {
        return Err(format!("Assertion at File: {}, Line: {}", file!(), line!()));
    };
    ($predicate:expr) => {
        if $predicate {
            let res: Result<(), String> = Ok(());
            res
        } else {
             return Err(format!("Assert failure: {}. File: {}, Line: {}",
                     stringify!($predicate), file!(), line!()))
        }
    }
}

#[macro_export]
macro_rules! safe_assert_eq {
    ($left:expr, $right:expr) => {
        if $left == $right {
            let res: Result<(), String> = Ok(());
            res
        } else {
             return Err(format!("Assert failure: left = {:#?}, right = {:#?} File: {}, Line: {}",
                     $left, $right, file!(), line!()))
        }
    };
    ($left:expr, $right:expr, $extra:expr) => {
        if $left == $right {
            let res: Result<(), String> = Ok(());
            res
        } else {
             return Err(format!("Assert failure: left = {:#?}, right = {:#?} File: {}, Line: {}\nExtra Context: {:#?}",
                     $left, $right, file!(), line!(), $extra))
        }

    }
}

#[macro_export]
macro_rules! fail {
    () => {
        safe_assert!()
    }
}

pub trait Test {
    type Msg: Clone+Debug+CausalMsg;

    // Mandatory functions
    fn reset(&mut self, record: bool);
    fn gen_request(&mut self, n: u64) -> Self::Msg;
    fn run(&mut self, Self::Msg) -> Result<(), String>;

    // Optional functions
    fn update_model(&mut self, _request: Self::Msg) {}
    fn drop_msg(&self, _request: &Self::Msg) -> bool { false }
    fn get_states(&self) -> Option<Vec<String>> { None }
    fn get_model(&self) -> Option<String> { None }
    // The schedule is most useful in a machine readable format
    fn get_schedule(&self) -> Option<Vec<u8>> { None }
}

pub struct Fuzzer<T: Test> {
    test_name: &'static str,
    history: Vec<T::Msg>,
    // Only used during shrinking
    smallest_history: Vec<T::Msg>,
    smallest_history_msg: String,
    smallest_history_states: String,
    smallest_history_model: String,
    test: T
}

impl<T: Test> Fuzzer<T> {
    pub fn new(name: &'static str, test: T) -> Fuzzer<T> {
        Fuzzer {
            test_name: name,
            history: Vec::new(),
            smallest_history: Vec::new(),
            smallest_history_msg: "".to_string(),
            smallest_history_states: "".to_string(),
            smallest_history_model: "".to_string(),
            test: test
        }
    }

    /// Send `n` messages to the VR group and run the assertions.
    pub fn run(&mut self, n: u64) {
        let dir_root = Path::new("tests/output").join(self.test_name);
        let _ = fs::create_dir_all(&dir_root);
        for i in 1..n+1 {
            let req = self.test.gen_request(i);
            self.history.push(req.clone());
            self.test.update_model(req.clone());
            match self.test.run(req) {
                Ok(()) => (),
                Err(msg) => {
                    let dir_name = make_output_dir(&dir_root);
                    let summary = make_summary(i, msg);
                    log("summary.txt", &dir_name, summary.as_bytes());
                    self.log_replica_states(&dir_name);
                    self.log_history(&dir_name);
                    if let Some(model) = self.test.get_model() {
                        log("model.txt", &dir_name, model.as_bytes());
                    }
                    println!("{}", summary);
                    let shrink_msg = self.shrink();
                    self.record_schedule();
                    if let Some(schedule) = self.test.get_schedule() {
                        log("schedule.txt", &dir_name, &schedule);
                    }
                    log("shrink.txt", &dir_name, shrink_msg.as_bytes());
                    log("shrink_replica_states.txt", &dir_name,
                        self.smallest_history_states.as_bytes());
                    log("shrink_model.txt", &dir_name, self.smallest_history_model.as_bytes());
                    println!("{}", shrink_msg);
                    assert!(false);
                }
            }
        }
    }

    /// This is an extremely naive shrinking strategy, but it's workable on smaller histories
    fn shrink(&mut self) -> String {
        self.smallest_history = self.history.clone();
        self.shrink_by_chunk();
        let mut msg = "Shrinking...\n".to_string();
        if self.smallest_history.len() == self.history.len() {
            msg.push_str("\nShrinking failed. No smaller history found that fails.")
        } else {
            msg.push_str(&format!("\nShrinking succeeded. History size reduced from {} to {}\n",
                                   self.history.len(), self.smallest_history.len()));

            msg.push_str(&format!("History: \n{:#?}\n", self.smallest_history));
            msg.push_str(&format!("{}\n", self.smallest_history_msg));
        }
        msg
    }

    fn shrink_by_chunk(&mut self) {
        let mut chunk_size = self.smallest_history.len() / 2;
        loop {
            if self.shrink_by_chunk_extraction(chunk_size) {
                if chunk_size == 1 {
                    while self.shrink_by_chunk_extraction(chunk_size) {}
                    return;
                }
                chunk_size = self.smallest_history.len() / 2;
                print!("!{}", chunk_size);
            } else {
                if chunk_size == 1 { return; }
                chunk_size = chunk_size / 2;
                print!("+{}", chunk_size);
            }
        }
    }

    fn shrink_by_chunk_extraction(&mut self, size: usize) -> bool {
        let mut start = 0;
        while start + size < self.smallest_history.len() {
            let mut candidate = (&self.smallest_history[0..start]).to_vec();
            let causal_ids = {
                let removed = &self.smallest_history[start..start+size];
                self.causal_ids(removed)
            };
            candidate.extend_from_slice(&self.smallest_history[start+size..]);
            if self.try_candidate(candidate, causal_ids) { return true; }
            start += size;
        }
        // We don't want to try the last chunk twice
        if start + size != (self.smallest_history.len() - 1) {
            // Never discard the last operation
            let causal_ids = {
                let removed = &self.smallest_history[start..self.smallest_history.len()-1];
                self.causal_ids(removed)
            };
            let mut candidate = (&self.smallest_history[0..start]).to_vec();
            candidate.push(self.smallest_history[self.smallest_history.len() - 1].clone());
            return self.try_candidate(candidate, causal_ids)
        }
        false
    }

    fn try_candidate(&mut self, candidate: Vec<T::Msg>, causal_ids: Vec<Uuid>) -> bool {
        let mut actually_run = Vec::new();
        print!(".{}", candidate.len());
        self.test.reset(false);
        for i in 0..candidate.len() {
            let msg = candidate[i].clone();
            let causal_id = msg.causal_id();
            // Don't send messages matching the causal id of a removed message
            if causal_id.is_some() && causal_ids.contains(&causal_id.unwrap()) { continue; }
            // Some times messages are sent to the wrong peer in a shrunk sequence. In this case
            // remove them. For instance, sending a ViewChange message to the primary is a mistake.
            if self.test.drop_msg(&msg) { continue; }
            actually_run.push(msg.clone());
            self.test.update_model(msg.clone());
            match self.test.run(msg) {
                Ok(()) => (),
                Err(msg) => {
                    if self.smallest_history.len() == actually_run.len() { return false }
                    self.smallest_history = actually_run;
                    self.smallest_history_msg = msg;
                    self.smallest_history_states = "".to_string();
                    if let Some(states) = self.test.get_states() {
                        for s in states {
                            self.smallest_history_states.push_str("\n");
                            self.smallest_history_states.push_str(&s);
                        }
                    }
                    if let Some(model) = self.test.get_model() {
                        self.smallest_history_model = model;
                    }
                    return true;
                }
            }
        }
        false
    }

    /// After shrinking we re-run the failing history and tell the test to record every interesting
    /// thing it deems necessary. This can later be retrieved by calling self.test.get_schedule();
    fn record_schedule(&mut self) {
        self.test.reset(true);
        for i in 0..self.smallest_history.len() {
            let msg = self.smallest_history[i].clone();
            self.test.update_model(msg.clone());
            let _ = self.test.run(msg);
        }
    }

    fn causal_ids(&self, msgs: &[T::Msg]) -> Vec<Uuid> {
        let mut v = Vec::new();
        for m in msgs {
            match m.causal_id() {
                Some(uuid) => v.push(uuid),
                None => ()
            }
        }
        v
    }

    fn log_replica_states(&self, dir_name: &Path) {
        if let Some(states) = self.test.get_states() {
            let mut file = File::create(dir_name.join("replica_states.txt")).unwrap();
            let _ = file.write_all("Replica States: \n".as_bytes());
            for s in states {
                let _ = file.write_all(s.as_bytes());
                let _ = file.write_all("\n".as_bytes());
            }
        }
    }

    fn log_history(&self, dir_name: &Path) {
        let mut file = File::create(dir_name.join("history.txt")).unwrap();
        let _ = file.write_all((format!("History: \n{:#?}\n", self.history).as_bytes()));
    }

}

fn make_summary(request_num: u64, msg: String) -> String {
    format!("Test failed on request_num {}\n{}\n", request_num, msg)
}

fn log(filename: &str, dir_name: &Path, data: &[u8]) {
    if data.len() == 0 { return; }
    let mut file = File::create(dir_name.join(filename)).unwrap();
    let _ = file.write_all(data);
}

fn make_output_dir(dir_root: &Path) -> PathBuf {
    let date = format!("{}", time::now().rfc3339());
    let dir_name = dir_root.join(&date);
    fs::create_dir(&dir_name).unwrap();
    let current = dir_root.join("current");
    let _ = fs::remove_file(&current);
    symlink(&date, current).unwrap();
    dir_name
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, ThreadRng};
    use rand::distributions::{IndependentSample, Range};
    use uuid::Uuid;
    use debugger_shared::CausalMsg;

#[test]
#[should_panic]
fn shrink_test() {
    let max_messages = 100000;
    let test = ShrinkTester::new(max_messages, 2);
    let mut fuzzer = Fuzzer::new("shrink_test", test);
    fuzzer.run(max_messages)
}

struct ShrinkTester {
    max_count: u64,
    count: u64,
    max_size: u64,
    magic_num: u64,
    rng: ThreadRng,
    range: Range<u64>
}

/// This tester fails when it finds `count` instances of a given random number in a potential max
/// amount of `size` numbers. Shrinking is then performed on the failed history. It always fails
/// because if 3 of the same number aren't randomly generated we add them at the end.
impl ShrinkTester {
    fn new(max_size: u64, max_count: u64) -> ShrinkTester {
        ShrinkTester {
            max_size: max_size,
            max_count: max_count,
            count: 0,
            magic_num: max_size/2,
            rng: thread_rng(),
            range: Range::new(0, max_size)
        }
    }
}

impl CausalMsg for u64 {
    fn causal_id(&self) -> Option<Uuid> {
        None
    }
}

impl Test for ShrinkTester {
    type Msg = u64;

    fn reset(&mut self, _: bool) {
        self.count = 0;
    }

    fn gen_request(&mut self, n: u64) -> u64 {
        let remaining = self.max_count - self.count;
        if n == self.max_size - remaining + 1 {
            // Ensure we fail, so we can test shrinking
            self.magic_num
        } else {
            self.range.ind_sample(&mut self.rng)
        }
    }

    fn update_model(&mut self, val: u64) {
        if val == self.magic_num {
            self.count += 1;
        }
    }

    fn run(&mut self, val: u64) -> Result<(), String> {
        if self.count == self.max_count {
            Err(format!("Found Magic Num: {}, {} times", val, self.max_count))
        } else {
            Ok(())
        }
    }
}

}
