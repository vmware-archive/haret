use resp::{Format, Combinator, Parse, Parser, RespType};
use rustc_serialize::Encodable;
use msgpack::{Encoder, from_msgpack};
use std::io::{Error, ErrorKind};
use super::ElementType;

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
    // TODO: Should probably differentiate between NotFound and actual errors
    ParentNotFoundError,
    ElementAlreadyExistsError,
    ElementNotFoundError(String),
    CasFailedError {path: String, expected: u64, actual: u64},
    Error {msg: String},
}

// This just wraps a message pack encoded string inside a resp bulk string
impl Format for VrApiReq {
    fn format(&self) -> Combinator {
        let c = Combinator::new();
        let data = Encoder::to_msgpack(&self).unwrap();
        c.bulk(data)
    }
}

impl Parse for VrApiReq {
    fn parsers() -> Vec<Parser<VrApiReq>> {
        vec![Parser::<VrApiReq>::new(Box::new(|bulk| {
            if let RespType::Bulk(data) = (*bulk).pop().unwrap() {
                match from_msgpack(&data) {
                    Err(e) => Err(Error::new(ErrorKind::InvalidInput, e)),
                    Ok(val) => Ok(val)
                }
            } else {
                assert!(false);
                Err(Error::new(ErrorKind::InvalidInput, "Failed to parse VR VrApiReq"))
            }
        })).bulk(None)]
    }
}

// This just wraps a message pack encoded string inside a resp bulk string
impl Format for VrApiRsp {
    fn format(&self) -> Combinator {
        let c = Combinator::new();
        let data = Encoder::to_msgpack(&self).unwrap();
        c.bulk(data)
    }
}

impl Parse for VrApiRsp {
    fn parsers() -> Vec<Parser<VrApiRsp>> {
        vec![Parser::<VrApiRsp>::new(Box::new(|bulk| {
            if let RespType::Bulk(data) = (*bulk).pop().unwrap() {
                match from_msgpack(&data) {
                    Err(e) => Err(Error::new(ErrorKind::InvalidInput, e)),
                    Ok(val) => Ok(val)
                }
            } else {
                assert!(false);
                Err(Error::new(ErrorKind::InvalidInput, "Failed to parse VR VrApiRsp"))
            }
        })).bulk(None)]
    }
}

