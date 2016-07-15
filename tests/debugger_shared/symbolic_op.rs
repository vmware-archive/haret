#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub enum SymbolicOp {
    ClientRequest,
    Reconfiguration,
    Commit,
    ViewChange,
    Crash,
    Restart
}
