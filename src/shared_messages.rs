//! This module includes messages shared by all TCP Servers(admin, cluster, vr, vr-api) in V2R2.

use uuid::Uuid;
use vr::Replica;

/// Every time VR api client connects to any v2r2 port it sends a 'NewSessionRequest(tenant_id)' message asking for
/// a session id and the current primary for the given tenant.
#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct NewSessionRequest(pub Uuid);

/// If the current primary is on the connected node, respond with a new `SessionId` and the current
/// primary. If the current primary is on a different node then return a `Redirect` to the the new
/// replica and host. If there is no current primary for that tenant then return a `Retry`.
#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum NewSessionReply {
    SessionId {session_id: Uuid, primary: Replica},
    Redirect {host: String},
    Retry(u64), // milliseconds
    NoSuchTenant
}
