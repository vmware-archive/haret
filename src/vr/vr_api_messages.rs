use rabble::Pid;
use super::ElementType;

type Milliseconds = u64;

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct ClientId(pub String);

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct NamespaceId(pub String);

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum VrClientMsg {
    Req {pid: Option<Pid>, op: VrApiReq, request_num: u64},
    Rsp {epoch: u64, view: u64, request_num: u64, value: VrApiRsp}
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum VrApiReq {
    GetNamespaces,
    RegisterClient(ClientId, NamespaceId),
    Create {path: String, ty: ElementType},
    Put {path: String, data: Vec<u8>, cas_tag: Option<u64>},
    Get {path: String, cas: bool},
    Delete {path: String, cas_tag: Option<u64>},
    List {path: String},
    Null // used during reconfiguration
}

impl VrApiReq {
    pub fn get_path(&self) -> String {
        match *self {
            VrApiReq::Create {ref path, ..} => path.clone(),
            VrApiReq::Put {ref path, ..} => path.clone(),
            VrApiReq::Get {ref path, ..} => path.clone(),
            VrApiReq::Delete {ref path, ..} => path.clone(),
            VrApiReq::List {ref path, ..} => path.clone(),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum VrApiRsp {
    Namespaces(Vec<NamespaceId>),
    ClientRegistration {primary: Pid, new_registration: bool},
    Redirect {primary: Pid, api_addr: String},
    Retry(Milliseconds),
    Element {data: Vec<u8>, cas_tag: Option<u64>},
    KeyList {keys: Vec<String>},
    Ok,
    Error {msg: String},
    Timeout,
    ParentNotFoundError,
    ElementAlreadyExistsError,
    ElementNotFoundError(String),
    CasFailedError {path: String, expected: u64, actual: u64},
}

impl VrApiRsp {
    pub fn is_err(&self) -> bool {
        match *self {
            VrApiRsp::Error{..} => true,
            VrApiRsp::Timeout => true,
            VrApiRsp::ParentNotFoundError => true,
            VrApiRsp::ElementAlreadyExistsError => true,
            VrApiRsp::ElementNotFoundError(_) => true,
            VrApiRsp::CasFailedError{..} => true,
            _ => false
        }
    }
}
