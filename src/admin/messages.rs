use std::io::{Error, ErrorKind};
use resp::{Format, Combinator, Parse, Parser, RespType};

#[derive(Clone, Debug, PartialEq)]
pub enum Req {
    ConfigSet(String, String),
    ConfigGet(String),
    ClusterJoin(String),
    ClusterMembers,
    ClusterStatus
}

#[derive(Clone, Debug, PartialEq)]
pub enum Res {
    Ok,
    Simple(String),
    Err(String)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    Req(Req),
    Res(Res)
}

impl Format for Msg {
    fn format(&self) -> Combinator {
        let c = Combinator::new();
        match *self {
            Msg::Req(Req::ConfigSet(ref key, ref val)) =>
                c.array().bulk_s("config").bulk_s("set").bulk_s(&key).bulk_s(&val).end(),
            Msg::Req(Req::ConfigGet(ref key)) =>
                c.array().bulk_s("config").bulk_s("get").bulk_s(&key).end(),
            Msg::Req(Req::ClusterJoin(ref ipstr)) =>
                c.array().bulk_s("cluster").bulk_s("join").bulk_s(&ipstr).end(),
            Msg::Req(Req::ClusterMembers) =>
                c.array().bulk_s("cluster").bulk_s("members").end(),
            Msg::Req(Req::ClusterStatus) =>
                c.array().bulk_s("cluster").bulk_s("status").end(),
            Msg::Res(Res::Ok) => c.simple("ok"),
            Msg::Res(Res::Simple(ref string)) => c.simple(string),
            Msg::Res(Res::Err(ref string)) => c.error(string)
        }
    }
}

/// You must list more specific matches first or you may match any type
/// instead of exact matches in the current implementation. For example,
/// the match for "ok" string must come before the simple match that
/// matches any string. Exact matches are performed by passing `Some(val)`
/// to the parser function, while matches for any value of the correct type
/// pass `None`.
impl Parse for Msg {
    fn parsers() -> Vec<Parser<Msg>> {
        let config_set_constructor = Box::new(|types: &mut Vec<RespType>| {
            if let RespType::Bulk(ref key) = types[3] {
                match String::from_utf8(key.clone()) {
                    Ok(keystr) => {
                        if let RespType::Bulk(ref val) = types[4] {
                            match String::from_utf8(val.clone()) {
                                Ok(valstr) => return Ok(Msg::Req(Req::ConfigSet(keystr, valstr))),
                                Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e))
                            }
                        } else {
                            // We should never get here
                            assert!(false);
                            Err(Error::new(ErrorKind::InvalidInput,
                                           "Failed to construct Msg::Req::ClusterSet"))
                        }
                    },
                    Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e))
                }
            } else {
                // We should never get here
                assert!(false);
                Err(Error::new(ErrorKind::InvalidInput,
                               "Failed to construct Msg::Req::ClusterSet"))
            }
        });

        let config_get_constructor = Box::new(|types: &mut Vec<RespType>| {
            if let RespType::Bulk(ref key) = types[3] {
                match String::from_utf8(key.clone()) {
                    Ok(keystr) => return Ok(Msg::Req(Req::ConfigGet(keystr))),
                    Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e))
                }
            } else {
                // We should never get here
                assert!(false);
                Err(Error::new(ErrorKind::InvalidInput,
                               "Failed to construct Msg::Req::ClusterSet"))
            }
        });

        let cluster_join_constructor = Box::new(|types: &mut Vec<RespType>| {
            if let RespType::Bulk(ref key) = types[3] {
                match String::from_utf8(key.clone()) {
                    Ok(ipstr) => return Ok(Msg::Req(Req::ClusterJoin(ipstr))),
                    Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e))
                }
            } else {
                // We should never get here
                assert!(false);
                Err(Error::new(ErrorKind::InvalidInput,
                               "Failed to construct Msg::Req::Join"))
            }
        });

        vec![Parser::new(config_set_constructor).array().bulk(Some("config"))
                .bulk(Some("set")).bulk(None).bulk(None).end(),

             Parser::new(config_get_constructor).array()
                 .bulk(Some("config")).bulk(Some("get")).bulk(None).end(),

             Parser::new(cluster_join_constructor).array()
                 .bulk(Some("cluster")).bulk(Some("join")).bulk(None).end(),

             Parser::new(Box::new(|_| Ok(Msg::Req(Req::ClusterMembers)))).array()
                 .bulk(Some("cluster")).bulk(Some("members")).end(),

             Parser::new(Box::new(|_| Ok(Msg::Req(Req::ClusterStatus)))).array()
                 .bulk(Some("cluster")).bulk(Some("status")).end(),

             Parser::new(Box::new(|_| Ok(Msg::Res(Res::Ok)))).simple(Some("ok")),

             Parser::new(Box::new(|types| {
                 if let Some(&RespType::Simple(ref s)) = types.first() {
                     Ok(Msg::Res(Res::Simple(s.clone())))
                 } else {
                    // We should never get here
                    assert!(false);
                    Err(Error::new(ErrorKind::InvalidInput,
                                   "Failed to construct Msg::Res::Simple"))
                 }
             })).simple(None),

             Parser::new(Box::new(|types| {
                 if let Some(&RespType::Error(ref s)) = types.first() {
                     return Ok(Msg::Res(Res::Err(s.clone())))
                 } else {
                    // We should never get here
                    assert!(false);
                    Err(Error::new(ErrorKind::InvalidInput,
                                   "Failed to construct Msg::Res::Err"))
                 }
             })).error(None)

            ]
    }
}

