pub struct Latest {
    pub last_normal_view: u64,
    pub commit_num: u64,
    pub op: u64,
    pub log: Vec<VrMsg>
}

impl Latest {
    fn new() -> Latest {
        Latest {
            last_normal_view: 0,
            commit_num: 0,
            op: 0,
            log: Vec::new()
        }
    }
}

pub struct ViewChangeState {
    pub responses: QuorumTracker<VrMsg>,
    latest: Latest,
}

impl ViewChangeState {
    pub fn new(quorum: u64, timeout: Duration) -> ViewChangeState {
        ViewChangeState {
            responses: QuorumTracker::new(quorum, timeout),
            latest: Latest::new()
        }
    }

    pub fn has_quorum(&self) -> bool {
        self.responses.has_quorum()
    }

    pub compute_latest_state(&mut self, current: Latest) -> Latest {
        self.responses.drain()
            .map(|(_, msg)| convert_do_view_change_msg_to_latest(msg))
            .fold(current, |mut latest, other| {
                if (other.last_normal_view > latest.last_normal_view) ||
                    (other.last_normal_view = latest.last_normal_view && other.op > latest.op)
                {
                   latest.last_normal_view = other.last_normal_view;
                   latest.op = other.op;
                   latest.log = log;
                }
                if other.commit_num > latest.commit_num {
                    latest.commit_num = commit_num;
                }
            })
    }
}

fn convert_do_view_change_msg_to_latest(msg: VrMsg) -> Latest {
    if let VrMsg::DoViewChange{op, last_normal_view, commit_num, log} = msg {
        return Latest {
            last_normal_view: last_normal_view,
            commit_num: commit_num,
            op: op,
            log: log
        };
    }
    unreachable!()
}
