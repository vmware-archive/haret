use uuid::Uuid;
use rabble::Pid;
use namespaces::Namespaces;

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum NamespaceMsg {
    /// Namespaces are gossiped between namespace managers,
    Namespaces(Namespaces),

    /// The following four messages are sent from a VM to indicate a change in membership state for
    /// a given namespace
    Reconfiguration {
        namespace_id: Uuid,
        old_config: VersionedReplicas,
        new_config: VersionedReplicas
    },
    Stop(Pid),
    NewPrimary(Pid),
    ClearPrimary(Uuid)
}

