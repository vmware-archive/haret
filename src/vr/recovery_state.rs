use rabble::Pid;
use uuid::Uuid;
use time::Duration;
use super::quorum_tracker::QuorumTracker;
use super::vrmsg::VrMsg;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPrimary {
    pub pid: Pid,
    pub view: u64,
    pub op: u64,
    pub commit_num: u64,
    pub log: Vec<VrMsg>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryState {
    pub nonce: Uuid,
    // Primary from the latest view we've heard from
    pub primary: Option<RecoveryPrimary>,
    pub responses: QuorumTracker<()>,
}

impl RecoveryState {
    pub fn new(quorum: u64, timeout: Duration) -> RecoveryState {
        RecoveryState {
            nonce: Uuid::new_v4(),
            primary: None,
            responses: QuorumTracker::new(quorum as usize, &timeout)
        }
    }

    pub fn has_quorum(&self, current_view: u64) -> bool {
        self.responses.has_super_quorum() && self.primary.as_ref().map_or(false, |p| p.view == current_view)
    }
}


