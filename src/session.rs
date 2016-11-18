use std::collections::HashSet;
use rabble::Pid;
use uuid::Uuid;

/// A Session represents a single client connection
///
/// A session is uniquely identified by it's connection id. Sessions are stored in a map referenced
/// by their ids in the API Server.
///
/// All replicas that are contacted by the client are kept track of because each replica maintains a
/// session table. When the session goes away, the session table needs to be cleaned up on each of
/// these replicas.
pub struct Session {
    pub namespace_id: Uuid,
    /// A list of Replicas that this client has talked to
    pub replicas: HashSet<Pid>
}

impl Session {
    fn new(namespace_id: Uuid) -> Session {
        Session {
            namespace_id: namespace_id,
            replicas: HashSet::new()
        }
    }
}
