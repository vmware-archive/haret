use std::io::{Error, ErrorKind, Read, Write};
use std::thread;
use std::thread::{Thread, JoinHandle};
use std::sync::mpsc::{channel, Sender};
use std::collections::{HashMap};
use std::hash::Hash;
use mio::Token;
use mio::tcp::TcpStream;
use tcpserver::{TcpServer, Event};
use resp::{Format, Combinator, Writer, Reader, Parse, Parser, RespType};

#[derive(Clone, Debug, PartialEq)]
enum Req {
    ClusterSetName(String),
    ClusterGetName
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
            Msg::Req(Req::ClusterSetName(ref name)) =>
                c.array().bulk_s("cluster").bulk_s("setname").bulk_s(&name).end(),
            Msg::Req(Req::ClusterGetName) =>
                c.array().bulk_s("cluster").bulk_s("getname").end(),
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
        // The constructor takes the chain of matched types from parsing
        // and constructs a Msg::ClusterSetName(...)
        let cluster_set_name_constructor = Box::new(|types: &Vec<RespType>| {
            if let RespType::Bulk(ref val) = types[3] {
                match String::from_utf8(val.clone()) {
                    Ok(string) => return Ok(Msg::Req(Req::ClusterSetName(string))),
                    Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e))
                }
            } else {
                // We should never get here
                assert!(false);
                Err(Error::new(ErrorKind::InvalidInput,
                               "Failed to construct Msg::Req::ClusterSetName"))
            }
        });

        vec![Parser::new(cluster_set_name_constructor).array()
            .bulk(Some("cluster")).bulk(Some("setname")).bulk(None).end(),

             Parser::new(Box::new(|_| Ok(Msg::Req(Req::ClusterGetName))))
                 .array().bulk(Some("cluster")).bulk(Some("getname")).end(),

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

/// All Events from the event loop are sent via the sender to a separate thread
/// which will perform all allocations, encoding, computation and socket handling
/// via the AdminHandler.
pub struct AdminServer {
    // Other types of servers can have thread pools and multiple senders
    thread: JoinHandle<()>,
    sender: Sender<Event>,
}

impl TcpServer for AdminServer {
    fn run() -> AdminServer {
        let (tx, rx) = channel();
        let handle = thread::Builder::new().name("admin_server".to_string()).spawn(move || {
            let mut handler = AdminHandler::new();
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
            sender: tx
        }
    }

    fn new_sock(&mut self, token: Token, sock: TcpStream) {
        self.sender.send(Event::NewSock(token, sock)).unwrap();
    }

    fn readable(&mut self, token: Token) {
        self.sender.send(Event::Readable(token));
    }

    fn writable(&mut self, token: Token) {
        self.sender.send(Event::Writable(token));
    }
}

/// Handle all events from the event loop
struct AdminHandler {
    conns: HashMap<Token, (TcpStream, Reader<Msg>, Writer)>,
//    replies: HashMap<TcpStream
}

impl AdminHandler {
    fn new() -> AdminHandler {
        AdminHandler {
            conns: HashMap::new(),
            //replies: HashMap::new()
        }
    }

    fn insert(&mut self, token: Token, sock: TcpStream) {
        self.conns.insert(token, (sock, Reader::new(), Writer::new()));
    }

    fn read(&mut self, token: Token) {
        let &mut(ref mut sock, ref mut reader, _) = self.conns.get_mut(&token).unwrap();
        match reader.read(sock) {
            Ok(None) => (),
            Ok(Some(msg)) => println!("Got a message {:?}", msg),
            Err(e) => println!("Got an error: {:?}", e)
        }
    }

    fn write(&mut self, token: Token) {
        let &mut(ref mut sock, _, ref mut writer) = self.conns.get_mut(&token).unwrap();
        if (writer.is_empty()) {
            /*let msg = Msg::Res(Res::Ok);
            writer.format(msg);
            writer.write(sock); */
        }
    }
}
