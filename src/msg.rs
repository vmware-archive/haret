use namespace_msg::NamespaceMsg;
use vr::VrMsg;
use admin::{AdminReq, AdminRpy};
use vr::{VrApiRsp};

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum Msg {
    Vr(VrMsg),
    Namespace(NamespaceMsg),
    AdminReq(AdminReq),
    AdminRpy(AdminRpy),
    VrApiRsp(VrApiRsp),
    Error(String)
}
