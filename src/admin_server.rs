use std::io::{Result, ErrorKind, Read, Write};
use std::io;
use std::error::Error;
use std::thread;
use std::thread::{Thread, JoinHandle};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, RwLock};
use std::collections::{HashMap};
use mio::Token;
use mio::tcp::TcpStream;
use tcpserver::{TcpServer, Event};
use resp::{Format, Combinator, Writer, Reader, Parse, Parser, RespType};
use config::Config;

#[derive(Clone, Debug, PartialEq)]
enum Req {
    ConfigSet(String, String),
    ConfigGet(String)
}

#[derive(Clone, Debug, PartialEq)]
enum Res {
    Ok,
    Simple(String),
    Err(String)
}

#[derive(Clone, Debug, PartialEq)]
enum Msg {
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
        let config_set_constructor = Box::new(|types: &Vec<RespType>| {
            if let RespType::Bulk(ref key) = types[3] {
                match String::from_utf8(key.clone()) {
                    Ok(keystr) => {
                        if let RespType::Bulk(ref val) = types[4] {
                            match String::from_utf8(val.clone()) {
                                Ok(valstr) => return Ok(Msg::Req(Req::ConfigSet(keystr, valstr))),
                                Err(e) => return Err(io::Error::new(ErrorKind::InvalidInput, e))
                            }
                        } else {
                            // We should never get here
                            assert!(false);
                            Err(io::Error::new(ErrorKind::InvalidInput,
                                           "Failed to construct Msg::Req::ClusterSet"))
                        }
                    },
                    Err(e) => return Err(io::Error::new(ErrorKind::InvalidInput, e))
                }
            } else {
                // We should never get here
                assert!(false);
                Err(io::Error::new(ErrorKind::InvalidInput,
                               "Failed to construct Msg::Req::ClusterSet"))
            }
        });

        let config_get_constructor = Box::new(|types: &Vec<RespType>| {
            if let RespType::Bulk(ref key) = types[3] {
                match String::from_utf8(key.clone()) {
                    Ok(keystr) => return Ok(Msg::Req(Req::ConfigGet(keystr))),
                    Err(e) => return Err(io::Error::new(ErrorKind::InvalidInput, e))
                }
            } else {
                // We should never get here
                assert!(false);
                Err(io::Error::new(ErrorKind::InvalidInput,
                               "Failed to construct Msg::Req::ClusterSet"))
            }
        });

        vec![Parser::new(config_set_constructor).array().bulk(Some("config"))
                .bulk(Some("set")).bulk(None).bulk(None).end(),

             Parser::new(config_get_constructor).array()
                 .bulk(Some("config")).bulk(Some("get")).bulk(None).end(),

             Parser::new(Box::new(|_| Ok(Msg::Res(Res::Ok)))).simple(Some("ok")),

             Parser::new(Box::new(|types| {
                 if let Some(&RespType::Simple(ref s)) = types.first() {
                     Ok(Msg::Res(Res::Simple(s.clone())))
                 } else {
                    // We should never get here
                    assert!(false);
                    Err(io::Error::new(ErrorKind::InvalidInput,
                                   "Failed to construct Msg::Res::Simple"))
                 }
             })).simple(None),

             Parser::new(Box::new(|types| {
                 if let Some(&RespType::Error(ref s)) = types.first() {
                     return Ok(Msg::Res(Res::Err(s.clone())))
                 } else {
                    // We should never get here
                    assert!(false);
                    Err(io::Error::new(ErrorKind::InvalidInput,
                                   "Failed to construct Msg::Res::Err"))
                 }
             })).error(None)

            ]
    }
}

/// All Events from the event loop are sent via the sender to a separate thread
/// which will perform all allocations, encoding, computation and socket handling
/// via the AdminHandler.
pub struct AdminServer {
    // Other types of servers can have thread pools and multiple senders
    thread: JoinHandle<()>,
    sender: Sender<Event>,
    config: Arc<RwLock<Config>>
}

impl TcpServer for AdminServer {
    fn run(config: Arc<RwLock<Config>>) -> AdminServer {
        let config2 = config.clone();
        let (tx, rx) = channel();
        let handle = thread::Builder::new().name("admin_server".to_string()).spawn(move || {
            let mut handler = AdminHandler::new(config);
            loop {
                match rx.recv().unwrap() {
                    Event::NewSock(token, sock) => handler.insert(token, sock),
                    Event::Readable(token) => handler.read(token),
                    Event::Writable(token) => handler.write(token)
                }
            }
        }).unwrap();

        AdminServer {
            thread: handle,
            sender: tx,
            config: config2
        }
    }

    fn host(&self) -> String {
        let config = self.config.read().unwrap();
        config.admin_host.clone()
    }

    fn new_sock(&mut self, token: Token, sock: TcpStream) {
        self.sender.send(Event::NewSock(token, sock)).unwrap();
    }

    fn readable(&mut self, token: Token) {
        self.sender.send(Event::Readable(token)).unwrap();
    }

    fn writable(&mut self, token: Token) {
        self.sender.send(Event::Writable(token)).unwrap();
    }
}

/// Handle all events from the event loop
/// Maintain the internal state of the admin server
struct AdminHandler {
    config: Arc<RwLock<Config>>,
    conns: HashMap<Token, (TcpStream, Reader<Msg>, Writer)>,
    replies: HashMap<Token, Vec<Msg>>
}

impl AdminHandler {
    fn new(config: Arc<RwLock<Config>>) -> AdminHandler {
        AdminHandler {
            config: config,
            conns: HashMap::new(),
            replies: HashMap::new()
        }
    }

    fn insert(&mut self, token: Token, sock: TcpStream) {
        self.conns.insert(token, (sock, Reader::new(), Writer::new()));
    }

    fn set_config(&mut self, token: Token, key: String, val: String) {
        let mut replies = self.replies.entry(token).or_insert(Vec::new());
        let mut config = self.config.write().unwrap();
        match config.set(key, val) {
            Ok(()) => replies.push(Msg::Res(Res::Ok)),
            Err(e) => replies.push(Msg::Res(Res::Err(e.description().to_string())))
        };
    }

    fn get_config(&mut self, token: Token, key: String) {
        let mut replies = self.replies.entry(token).or_insert(Vec::new());
        let config = self.config.read().unwrap();
        match config.get(key) {
            Ok(string) => replies.push(Msg::Res(Res::Simple(string))),
            Err(e) => replies.push(Msg::Res(Res::Err(e.description().to_string())))
        };
    }

    fn read(&mut self, token: Token) {
        match self.do_read(token) {
            Ok(None) => (),
            Ok(Some(Msg::Req(Req::ConfigSet(key, val)))) => self.set_config(token, key, val),
            Ok(Some(Msg::Req(Req::ConfigGet(key)))) => self.get_config(token, key),
            Ok(Some(msg)) => println!("Got a message {:?}", msg),
            Err(e) => {
                // TODO: deregister the socket
                println!("Got an error on read: {:?}", e)
            }
        }
        self.do_write(token);
    }

    fn do_read(&mut self, token: Token) -> Result<Option<Msg>> {
        let &mut(ref mut sock, ref mut reader, _) = self.conns.get_mut(&token).unwrap();
        reader.read(sock)
    }

    fn do_write(&mut self, token: Token) {
        let &mut(ref mut sock, _, ref mut writer) = self.conns.get_mut(&token).unwrap();
        while writer.is_ready() {
            if writer.is_empty() {
                if let Some(replies) = self.replies.get_mut(&token) {
                    if replies.len() > 0 {
                        let reply = replies.remove(0);
                        writer.format(reply);
                    } else {
                        return;
                    }
                } else {
                    return;
                }
            }
            match writer.write(sock) {
                Ok(None) => writer.ready(false), // EWOULDBLOCK
                Ok(Some(())) => (), // We completed writing the message
                Err(err) => {
                    // TODO: deregister the socket
                    println!("Got an error on write: {:?}", err)
                }
            }
        }
    }

    fn write(&mut self, token: Token) {
        self.writer_ready(token, true);
        self.do_write(token);
    }

    fn writer_ready(&mut self, token: Token, ready: bool) {
        let &mut(_, _, ref mut writer) = self.conns.get_mut(&token).unwrap();
        writer.ready(ready);
    }
}
