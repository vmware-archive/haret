use uuid::Uuid;
use time::SteadyTime;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone)]
pub struct Timeout {
    client_id: Uuid,
    epoch: u64,
    view: u64,
    request_num: u64,
    time: SteadyTime
}

impl Display for Timeout {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{:#?}", self)
    }
}

