use std::fmt::{Display, Formatter, Error};

// Frequencies parameterized by `()` are only used for counts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VrStats {
    pub client_tick_count: usize,
    pub client_req_count: usize,
    pub client_reply_count: usize,
    pub client_new_sock_count: usize,
    pub client_deregister_count: usize
}

impl VrStats {
    pub fn new() -> VrStats {
        VrStats {
            client_tick_count: 0,
            client_req_count: 0,
            client_reply_count: 0,
            client_new_sock_count: 0,
            client_deregister_count: 0
        }
    }
}

impl Display for VrStats {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "Total Client Ticks: {}", self.client_tick_count).unwrap();
        writeln!(f, "Total Client Requests: {}", self.client_req_count).unwrap();
        writeln!(f, "Total Client Replies: {}", self.client_reply_count).unwrap();
        writeln!(f, "Total Client New Socket Messages: {}", self.client_new_sock_count).unwrap();
        writeln!(f, "Total Client Deregistrations: {}", self.client_deregister_count).unwrap();
        Ok(())
    }
}
