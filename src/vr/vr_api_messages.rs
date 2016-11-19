use rabble::Pid;
use super::ElementType;

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum VrClientMsg {
    Req {pid: Pid, op: VrApiReq, request_num: u64},
    Rpy {epoch: u64, view: u64, request_num: u64, value: VrApiRsp}
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum VrApiReq {
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
            VrApiReq::Null => unreachable!()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum VrApiRsp {
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
