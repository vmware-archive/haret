use namespace_msg::NamespaceMsg;
use vr::VrMsg;
use admin::{AdminReq, AdminRpy};
use api::ApiRpy;

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum Msg {
    Vr(VrMsg),
    Namespace(NamespaceMsg),
    AdminReq(AdminReq),
    AdminRpy(AdminRpy),
    ApiRpy(ApiRpy),
    Error(String)
}
