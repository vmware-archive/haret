use rustc_serialize::Encodable;
use v2r2::vr::{Replica, VrMsg};

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub enum Action {
    Send(Replica, VrMsg),
    Stop(Replica),
    Restart(Replica)
}

