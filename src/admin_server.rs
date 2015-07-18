use std::io::{Result, ErrorKind, Read, Write};
use std::io;
use std::error::Error;
use std::thread;
use std::thread::{Thread, JoinHandle};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, RwLock};
use std::collections::{HashMap};
use mio;
use mio::Token;
use mio::tcp::TcpStream;
use tcpserver::{TcpServer, Event};
use resp::{Format, Combinator, Writer, Reader, Parse, Parser, RespType};
use config::Config;
use event_loop::Notification;

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
    // We need to declare these types before initializing, hence the Option type.
    thread: Option<JoinHandle<()>>,
    sender: Option<Sender<Event>>,
    config: Arc<RwLock<Config>>
}

impl TcpServer for AdminServer {
    fn new(config: Arc<RwLock<Config>>) -> AdminServer {
        AdminServer {
            thread: None,
            sender: None,
            config: config
        }
    }

    fn run(&mut self, event_loop_sender: mio::Sender<Notification>) {
        let config = self.config.clone();
        let (tx, rx) = channel();
        let handle = thread::Builder::new().name("admin_server".to_string()).spawn(move || {
            let mut handler = AdminHandler::new(config, event_loop_sender);
            loop {
                match rx.recv().unwrap() {
                    Event::NewSock(token, sock) => handler.insert(token, sock),
                    Event::Readable(token) => handler.read(token),
                    Event::Writable(token) => handler.write(token)
                }
            }
        }).unwrap();

        self.sender = Some(tx);
        self.thread = Some(handle);
    }

    fn host(&self) -> String {
        let config = self.config.read().unwrap();
        config.admin_host.clone()
    }

    fn new_sock(&mut self, token: Token, sock: TcpStream) {
        self.sender.as_ref().unwrap().send(Event::NewSock(token, sock)).unwrap();
    }

    fn readable(&mut self, token: Token) {
        self.sender.as_ref().unwrap().send(Event::Readable(token)).unwrap();
    }

    fn writable(&mut self, token: Token) {
        self.sender.as_ref().unwrap().send(Event::Writable(token)).unwrap();
    }
}

/// Handle all events from the event loop
/// Maintain the internal state of the admin server
struct AdminHandler {
    config: Arc<RwLock<Config>>,
    conns: HashMap<Token, (TcpStream, Reader<Msg>, Writer)>,
    replies: HashMap<Token, Vec<Msg>>,
    sender: mio::Sender<Notification>
}

impl AdminHandler {
    fn new(config: Arc<RwLock<Config>>, tx: mio::Sender<Notification>) -> AdminHandler {
        AdminHandler {
            config: config,
            conns: HashMap::new(),
            replies: HashMap::new(),
            sender: tx
        }
    }

    fn insert(&mut self, token: Token, sock: TcpStream) {
        self.conns.insert(token, (sock, Reader::new(), Writer::new()));
    }

    fn deregister(&mut self, token: Token) {
        if let Some((sock, _, _)) = self.conns.remove(&token) {
            self.replies.remove(&token);
            self.sender.send(Notification::Deregister(sock)).unwrap();
            println!("Deregistered socket for {:?}", token);
        } else {
            println!("Error: Tried to deregister a token with no corresponding socket");
        }
    }

    fn read(&mut self, token: Token) {
        match self.do_read(token) {
            None => (),
            Some(token) => self.deregister(token)
        }
    }

    fn do_read(&mut self, token: Token) -> Option<Token> {
        if let Some(&mut(ref mut sock, ref mut reader, ref mut writer)) =
          self.conns.get_mut(&token) {
            let mut replies = self.replies.entry(token).or_insert(Vec::new());
            let mut config = &mut self.config;
            match reader.read(sock) {
                Ok(None) => (),
                Ok(Some(Msg::Req(Req::ConfigSet(key, val)))) => set_config(token, key, val,
                                                                           replies, config),
                Ok(Some(Msg::Req(Req::ConfigGet(key)))) => get_config(token, key, replies, config),
                Ok(Some(msg)) => println!("Got a message {:?}", msg),
                Err(e) => {
                    println!("Error reading from socket with token {:?}: {:?}", token, e);
                    return Some(token);
                }
            }
            do_write(token, sock, writer, replies)
        } else {
            None
        }
    }

    fn write(&mut self, token: Token) {
        match self.writer_ready(token) {
            None => (),
            Some(token) => self.deregister(token)
        }
    }

    fn writer_ready(&mut self, token: Token) -> Option<Token> {
        // It's possible we already deregistered and one last write gets triggered
        if let Some(&mut (ref mut sock, _, ref mut writer)) = self.conns.get_mut(&token) {
            let mut replies = self.replies.entry(token).or_insert(Vec::new());
            writer.ready(true);
            do_write(token, sock, writer, replies)
        } else {
            None
        }
    }
}

fn do_write(token: Token, sock: &mut TcpStream, writer: &mut Writer, replies: &mut Vec<Msg>) ->
Option<Token> {
    while writer.is_ready() {
        if writer.is_empty() {
            if replies.len() > 0 {
                let reply = replies.remove(0);
                writer.format(reply);
            } else {
                return None;
            }
        }
        match writer.write(sock) {
            Ok(None) => writer.ready(false), // EWOULDBLOCK
            Ok(Some(())) => (), // We completed writing the message
            Err(err) => {
                println!("Got an error on write: {:?}", err);
                return Some(token);
            }
        }
    }
    None
}

fn set_config(token: Token, key: String, val: String, replies: &mut Vec<Msg>,
              config: &mut Arc<RwLock<Config>>) {
    let mut config = config.write().unwrap();
    match config.set(key, val) {
        Ok(()) => replies.push(Msg::Res(Res::Ok)),
        Err(e) => replies.push(Msg::Res(Res::Err(e.description().to_string())))
    };
}

fn get_config(token: Token, key: String, replies: &mut Vec<Msg>, config: &mut Arc<RwLock<Config>>) {
    let config = config.read().unwrap();
    match config.get(key) {
        Ok(string) => replies.push(Msg::Res(Res::Simple(string))),
        Err(e) => replies.push(Msg::Res(Res::Err(e.description().to_string())))
    };
}

