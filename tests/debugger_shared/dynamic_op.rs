use v2r2::vr::{VrMsg, Replica};

/// All the dynamic operations that can be generated for Distill based tests.
#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub enum DynamicOp {
    ClientRequest(Replica, VrMsg),
    Reconfiguration(VrMsg),
    Commit(Replica),
    ViewChange(Replica),
    Crash(Replica),
    Restart(Replica)
}
