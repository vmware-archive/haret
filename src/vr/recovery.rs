pub struct RecoveryPrimary {
    pub pid: Pid,
    pub view: u64,
    pub op: u64,
    pub commit_num: u64,
    pub log: Vec<VrMsg>
}

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
            responses: QuorumTracker::new(quorum, timeout)
        }
    }

    pub fn has_quorum(&self, current_view: u64) -> bool {
        self.responses.has_super_quorum() && self.primary.map_or(false, |p| p.view == current_view)
    }
}


