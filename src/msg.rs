use namespaces::Namespaces;
use vr::VrMsg;

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum Msg {
    Vr(VrMsg),
    Namespace(NamespaceMsg)
    AdminReq(AdminReq),
    AdminRpy(AdminRpy),
    Error(String)
}
