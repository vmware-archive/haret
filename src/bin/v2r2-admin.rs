extern crate v2r2;
extern crate uuid;
extern crate rabble;
extern crate rustc_serialize;
extern crate rmp_serialize as msgpack;

use std::env;
use std::env::Args;
use std::io;
use std::process::exit;
use std::io::{Read, Result, Error, ErrorKind, Write};
use std::str::{SplitWhitespace, FromStr};
use std::net::TcpStream;
use std::mem;
use msgpack::{Encoder, Decoder};
use rustc_serialize::{Encodable, Decodable};
use uuid::Uuid;
use rabble::{Pid, NodeId};
use v2r2::admin::{AdminReq, AdminRpy, AdminMsg};

fn main() {
    let mut args = env::args();
    let addr = args.nth(1).unwrap();
    let sock = TcpStream::connect(&addr[..]).unwrap();
    if let Some(flag) = args.next() {
        run_script(&flag, args, sock);
    } else {
        run_interactive(sock);
    }
}

fn run_interactive(mut sock: TcpStream) {
    loop {
        prompt();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        match run(&command, &mut sock) {
            Ok(result) => println!("{}", result),
            Err(err) => {
                println!("{}", err);
                let kind = err.kind();
                if kind == ErrorKind::ConnectionReset || kind == ErrorKind::ConnectionAborted {
                    exit(-1);
                }
            }
        }
    }
}

fn run_script(flag: &str, mut args: Args, mut sock: TcpStream) {
    if flag != "-e" {
        println!("Invalid Flag");
        println!("{}", help());
        exit(-1);
    }
    let command = args.next().unwrap_or(String::new());
    match run(&command, &mut sock) {
        Ok(result) => {
            println!("{}", result);
            exit(0);
        }
        Err(err) => {
            println!("{}", err);
            exit(-1)
        }
    }
}

fn run(command: &str, sock: &mut TcpStream) -> Result<String> {
    let req = try!(parse(&command));
    exec(req, sock)
}

fn prompt() {
    let mut stdout = io::stdout();
    stdout.write_all(b"v2r2-admin> ").unwrap();
    stdout.flush().unwrap();
}

fn parse(command: &str) -> Result<AdminReq> {
    let mut iter = command.split_whitespace();
    match iter.next() {
        Some("config") => parse_no_args("config", &mut iter).map(|_| AdminReq::GetConfig),
        Some("cluster") => parse_cluster(&mut iter),
        Some("vr") => parse_vr(&mut iter),
        Some(_) => Err(help()),
        None => Err(help())
    }
}

fn parse_no_args(header: &'static str, iter: &mut SplitWhitespace) -> Result<()> {
    if iter.count() != 0 {
        println!("'{}' takes no parameters", header);
        return Err(help());
    }
    Ok(())
}

fn parse_cluster(mut iter: &mut SplitWhitespace) -> Result<AdminReq> {
    match iter.next() {
        Some("join") => {
            let args: Vec<_> = iter.collect();
            if args.len() != 1 {
                println!("'join' takes a single argument");
                return Err(help());
            }
            NodeId::from_str(args[0]).map(|node_id| AdminReq::Join(node_id))
                                     .map_err(|s| Error::new(ErrorKind::InvalidInput, s))
        },
        Some("status") => {
            parse_no_args("cluster status", &mut iter).map(|_| AdminReq::GetClusterStatus)
        },
        Some(_) => Err(help()),
        None => Err(help())
    }
}

fn parse_vr(mut iter: &mut SplitWhitespace) -> Result<AdminReq> {
    match iter.next() {
        Some("create") => parse_vr_create(iter),
        Some("namespaces") => parse_no_args("vr namespaces", &mut iter).map(|_| AdminReq::GetNamespaces),
        Some("replica") => parse_vr_replica(iter),
        Some("primary") => parse_vr_primary(iter),
        _ => Err(help())
    }
}

fn parse_vr_replica(iter: &mut SplitWhitespace) -> Result<AdminReq> {
    match iter.next() {
        Some(string) => {
            match Pid::from_str(&string) {
                Ok(replica) => Ok(AdminReq::GetReplicaState(replica)),
                Err(_) => {
                    println!("Error: Couldn't parse replica");
                    Err(help())
                }
            }
        },
        None => Err(help())
    }
}

fn parse_vr_primary(iter: &mut SplitWhitespace) -> Result<AdminReq> {
    match iter.next() {
        Some(string) => {
            match Uuid::parse_str(string) {
                Ok(uuid) => Ok(AdminReq::GetPrimary(uuid)),
                Err(_) => {
                    println!("Error: Couldn't parse namespace id as UUID");
                    Err(help())
                }
            }
        },
        None => Err(help())
    }
}


fn parse_vr_create(iter: &mut SplitWhitespace) -> Result<AdminReq> {
    match iter.next() {
        Some("namespace") => {
            let args: Vec<&str> = iter.collect();
            if args.len() != 1 {
                println!("No spaces allowed in UngroupedPid list");
                return Err(help());
            }
            let pidopts: Vec<_> = args[0].split(",").map(|s| Pid::from_str(s)).collect();
            if pidopts.iter().any(|p| p.is_err()) {
                return Err(Error::new(ErrorKind::InvalidInput, "Failed to parse pids"));
            }
            let pids: Vec<_> = pidopts.into_iter().map(|p| p.unwrap()).collect();
            Ok(AdminReq::CreateNamespace(pids))
        },
        _ => Err(help())
    }
}

fn exec(req: AdminReq, sock: &mut TcpStream) -> Result<String> {
    try!(write_msg(req, sock));
    match try!(read_msg(sock)) {
        AdminMsg::Rpy(rpy) => {
            match rpy {
                AdminRpy::Ok => Ok("ok".to_string()),
                AdminRpy::Timeout => Ok("timeout".to_string()),
                AdminRpy::Error(string) => Err(Error::new(ErrorKind::Other, &string[..])),
                AdminRpy::Config(config) => Ok(format!("{:#?}", config)),
                AdminRpy::NamespaceId(uuid) => Ok(uuid.to_string()),
                AdminRpy::Namespaces(namespaces) => Ok(format!("{:#?}", namespaces)),
                AdminRpy::ReplicaState(state) => Ok(format!("{:#?}", state)),
                AdminRpy::ReplicaNotFound(pid) => Err(Error::new(ErrorKind::NotFound,
                                                                 pid.to_string())),
                AdminRpy::Primary(pid) => Ok(pid.map_or("None".to_string(), |p| p.to_string())),
                AdminRpy::ClusterStatus(status) => Ok(format!("{:#?}", status))
            }
        },
        msg => Err(Error::new(ErrorKind::InvalidData,
                              format!("Invalid reply from server: {:?}", msg)))
    }
}

fn read_msg(sock: &mut TcpStream) -> Result<AdminMsg> {
    let mut header = [0; 4];
    try!(sock.read_exact(&mut header));
    let len = unsafe { u32::from_be(mem::transmute(header)) };
    let mut buf = vec![0; len as usize];
    try!(sock.read_exact(&mut buf));
    let mut decoder = Decoder::new(&buf[..]);
    Decodable::decode(&mut decoder).map_err(|e| {
        Error::new(ErrorKind::InvalidData, e.to_string())
    })
}

fn write_msg(req: AdminReq, sock: &mut TcpStream) -> Result<()> {
    let mut encoded = Vec::new();
    try!(AdminMsg::Req(req).encode(&mut Encoder::new(&mut encoded)).map_err(|_| {
        Error::new(ErrorKind::InvalidInput, "Failed to encode msgpack data")
    }));
    let len: u32 = encoded.len() as u32;
    // 4 byte len header
    let header: [u8; 4] = unsafe { mem::transmute(len.to_be()) };
    try!(sock.write_all(&header));
    sock.write_all(&encoded)
}


fn help() -> Error {
    let string  =
"Usage: v2r2-admin <IpAddress> [-e <command>]

    Commands:
        cluster join <NodeId>
        cluster status
        vr create namespace <UngroupedPid1,UngroupedPid2,..,UngroupedPidN>
        vr namespaces
        vr replica <Pid>
        vr primary <NamespaceId>
        config

    Flags:
        -e <Command>   Non-interactive mode

    Config Keys:
        node           The name of the current node
        cluster        The name of the cluster the node is a member of
        cluster-host   The ip:port of the cluster server listener
        admin-host     The ip:port of the admin server listener
        vr-host        The ip:port of the viewstamped replication protocol server

    Argument formats:
        Uuid           see: https://doc.rust-lang.org/uuid/uuid/index.html
        UngroupedPid   replica_name::node_name
        Pid            namespace_uuid::replica_name::node_name

    Examples:
        Get the cluster status in non-interactive mode:
            v2r2-admin 127.0.0.1:2001 -e 'cluster status'
    ";
    Error::new(ErrorKind::InvalidInput, string)
}
