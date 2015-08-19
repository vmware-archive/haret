use resp::{Format, Combinator, Parse, Parser, RespType};
use rustc_serialize::Encodable;
use msgpack::{Encoder, from_msgpack};
use membership::Member;
use orset::ORSet;
use std::io::{Error, ErrorKind};

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub enum Msg {
    Members(String, ORSet<Member>),
    Ping
}

// This just wraps a message pack encoded string inside a resp bulk string
impl Format for Msg {
    fn format(&self) -> Combinator {
        let c = Combinator::new();
        let data = Encoder::to_msgpack(&self).unwrap();
        c.bulk(data)
    }
}

impl Parse for Msg {
    fn parsers() -> Vec<Parser<Msg>> {
        vec![Parser::<Msg>::new(Box::new(|bulk| {
            if let RespType::Bulk(data) = (*bulk).pop().unwrap() {
                match from_msgpack(&data) {
                    Err(e) => Err(Error::new(ErrorKind::InvalidInput, e)),
                    Ok(val) => Ok(val)
                }
            } else {
                assert!(false);
                Err(Error::new(ErrorKind::InvalidInput, "Failed to parse Cluster Msg"))
            }
        })).bulk(None)]
    }
}

