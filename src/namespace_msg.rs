use rabble::Pid;
use namespaces::Namespaces;
use vr::VersionedReplicas;

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct ClientId(pub String);

#[derive(Debug, Clone, Eq, Hash, PartialEq, RustcEncodable, RustcDecodable)]
pub struct NamespaceId(pub String);

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum NamespaceMsg {
    /// Namespaces are gossiped between namespace managers,
    Namespaces(Namespaces),

    /// Register a client with the primary of a namespace
    RegisterClient(ClientId, NamespaceId),

    /// API Addresses are published from the node they live on to all other nodes.
    ApiAddr(String),

    /// The following four messages are sent from a VM to indicate a change in membership state for
    /// a given namespace
    Reconfiguration {
        namespace_id: NamespaceId,
        old_config: VersionedReplicas,
        new_config: VersionedReplicas
    },
    Stop(Pid),
    NewPrimary(Pid),
    ClearPrimary(NamespaceId)
}

