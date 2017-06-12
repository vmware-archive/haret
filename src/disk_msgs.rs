use rabble;
use msg::Msg;
use std::convert::From;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiskReq {
   ReadNonce
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiskRpy {
    Nonce(u64),
    NotFound
}

impl From<DiskReq> for rabble::Msg<Msg> {
    fn from(msg: DiskReq) -> rabble::Msg<Msg> {
        rabble::Msg::User(Msg::DiskReq(msg))
    }
}

impl From<DiskRpy> for rabble::Msg<Msg> {
    fn from(msg: DiskRpy) -> rabble::Msg<Msg> {
        rabble::Msg::User(Msg::DiskRpy(msg))
    }
}
