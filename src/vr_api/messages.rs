use resp::{Format, Combinator, Parse, Parser, RespType};
use rustc_serialize::Encodable;
use msgpack::{Encoder, from_msgpack};
use std::io::{Error, ErrorKind};
use element::{ElementType, Version};

#[derive(Debug, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum Req {
    Create {path: String, ty: ElementType},
    Put {path: String, data: Vec<u8>, cas_tag: Option<Version>},
    Get {path: String, cas: bool},
    Delete {path: String, cas_tag: Option<Version>},
    List {path: String}
}

#[derive(Debug, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum Rsp {
    Element {data: Vec<u8>, cas_tag: Option<Version>},
    KeyList {keys: Vec<String>},
    Ok,
    // TODO: Should probably differentiate between NotFound and actual errors
    Error {msg: String},
}

// This just wraps a message pack encoded string inside a resp bulk string
impl Format for Req {
    fn format(&self) -> Combinator {
        let c = Combinator::new();
        let data = Encoder::to_msgpack(&self).unwrap();
        c.bulk(data)
    }
}

impl Parse for Req {
    fn parsers() -> Vec<Parser<Req>> {
        vec![Parser::<Req>::new(Box::new(|bulk| {
            if let RespType::Bulk(data) = (*bulk).pop().unwrap() {
                match from_msgpack(&data) {
                    Err(e) => Err(Error::new(ErrorKind::InvalidInput, e)),
                    Ok(val) => Ok(val)
                }
            } else {
                assert!(false);
                Err(Error::new(ErrorKind::InvalidInput, "Failed to parse VR Req"))
            }
        })).bulk(None)]
    }
}

// This just wraps a message pack encoded string inside a resp bulk string
impl Format for Rsp {
    fn format(&self) -> Combinator {
        let c = Combinator::new();
        let data = Encoder::to_msgpack(&self).unwrap();
        c.bulk(data)
    }
}

impl Parse for Rsp {
    fn parsers() -> Vec<Parser<Rsp>> {
        vec![Parser::<Rsp>::new(Box::new(|bulk| {
            if let RespType::Bulk(data) = (*bulk).pop().unwrap() {
                match from_msgpack(&data) {
                    Err(e) => Err(Error::new(ErrorKind::InvalidInput, e)),
                    Ok(val) => Ok(val)
                }
            } else {
                assert!(false);
                Err(Error::new(ErrorKind::InvalidInput, "Failed to parse VR Rsp"))
            }
        })).bulk(None)]
    }
}

