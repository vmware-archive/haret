extern crate v2r2;
extern crate uuid;
extern crate msgpack;

use std::env;
use std::env::Args;
use std::io;
use std::process::exit;
use std::io::{Result, Error, ErrorKind, Write};
use std::str::{SplitWhitespace, FromStr};
use std::net::TcpStream;
use v2r2::admin::{AdminClientReq, AdminClientRpy};
use v2r2::frame::{ReadState, WriteState};
use v2r2::vr::{Replica};
use msgpack::{Encoder, from_msgpack};

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

fn parse(command: &str) -> Result<AdminClientReq> {
    let mut iter = command.split_whitespace();
    match iter.next() {
        Some("config") => parse_config(&mut iter),
        Some("cluster") => parse_cluster(&mut iter),
        Some("vr") => parse_vr(&mut iter),
        Some(_) => Err(help()),
        None => Err(help())
    }
}

fn parse_config(iter: &mut SplitWhitespace) -> Result<AdminClientReq> {
    match iter.next() {
        Some("set") => {
            let args: Vec<_> = iter.collect();
            if args.len() != 2 { return Err(help()); }
            let key = args[0].to_string();
            let val = args[1].to_string();
            Ok(AdminClientReq::ConfigSet(key, val))
        },
        Some("get") => {
            let args: Vec<_> = iter.collect();
            if args.len() != 1 { return Err(help()); }
            let key = args[0].to_string();
            Ok(AdminClientReq::ConfigGet(key))
        },
        Some(_) => Err(help()),
        None => Err(help())
    }
}

fn parse_cluster(iter: &mut SplitWhitespace) -> Result<AdminClientReq> {
    match iter.next() {
        Some("join") => {
            let args: Vec<_> = iter.collect();
            if args.len() != 1 { return Err(help()); }
            let ipstr = args[0].to_string();
            Ok(AdminClientReq::ClusterJoin(ipstr))
        },
        Some("members") => {
            if iter.count() != 0 { return Err(help()); }
            Ok(AdminClientReq::ClusterMembers)
        },
        Some("status") => {
            if iter.count() != 0 { return Err(help()); }
            Ok(AdminClientReq::ClusterStatus)
        },
        Some(_) => Err(help()),
        None => Err(help())
    }
}

fn parse_vr(iter: &mut SplitWhitespace) -> Result<AdminClientReq> {
    match iter.next() {
        Some("create") => parse_vr_create(iter),
        Some("tenants") => Ok(AdminClientReq::VrTenants),
        Some("replica") => parse_vr_replica(iter),
        Some("stats") => Ok(AdminClientReq::VrStats),
        _ => Err(help())
    }
}

fn parse_vr_replica(iter: &mut SplitWhitespace) -> Result<AdminClientReq> {
    match iter.next() {
        Some(string) => {
            match Replica::from_str(&string) {
                Ok(replica) => Ok(AdminClientReq::VrReplica(replica)),
                Err(_) => {
                    println!("Error: Couldn't parse replica");
                    Err(help())
                }
            }
        },
        None => Err(help())
    }
}

fn parse_vr_create(iter: &mut SplitWhitespace) -> Result<AdminClientReq> {
    match iter.next() {
        Some("tenant") => {
            let args: Vec<&str> = iter.collect();
            if args.len() != 1 {
                println!("No spaces allowed in RawReplica list");
                return Err(help());
            }
            Ok(AdminClientReq::VrCreateTenant(args[0].to_string()))
        },
        _ => Err(help())
    }
}

fn exec(req: AdminClientReq, sock: &mut TcpStream) -> Result<String> {
    let mut writer = WriteState::new();
    writer = writer.push(Encoder::to_msgpack(&req).unwrap());
    loop {
        if let Ok((more_to_write, new_writer)) = writer.write(sock) {
            if !more_to_write { break; }
            writer = new_writer;
        } else {
            return Err(Error::new(ErrorKind::Other, "Failed to send request to server"))
        }
    }

    let mut reader = ReadState::new();
    loop {
        match reader.read(sock) {
            (_, Ok(Some(data))) => {
                let reply = match from_msgpack(&data).unwrap() {
                    AdminClientRpy::Ok => Ok("ok".to_string()),
                    AdminClientRpy::Error(string) => Err(Error::new(ErrorKind::Other, &string[..])),
                    AdminClientRpy::Timeout => Ok("timeout".to_string()),
                    AdminClientRpy::Config(_, val) => Ok(val),
                    AdminClientRpy::Members(string) => Ok(string),
                    AdminClientRpy::MemberStatus(status) => Ok(status),
                    AdminClientRpy::VrTenantId(uuid) => Ok(uuid.to_string()),
                    AdminClientRpy::VrTenants(tenants) => Ok(format!("{:#?}", tenants)),
                    AdminClientRpy::VrReplica(state, ctx) => Ok(format_replica_state(state, ctx)),
                    AdminClientRpy::VrStats(stats) => Ok(stats)
                };
                return reply;
            },
            (new_reader, Ok(None)) =>  {
                reader = new_reader;
            },
            (_, Err(_)) =>
                return Err(Error::new(ErrorKind::Other, "Failed to read response from server"))
        }
    }
}

fn format_replica_state(state: String, ctx: String) -> String {
    let mut s = "state: ".to_string();
    s.push_str(&state);
    s.push_str("\n");
    s.push_str(&ctx);
    s
}

fn help() -> Error {
    let string  =
"Usage: v2r2-admin <IpAddress> [-e <command>]

    Commands:
        config set <Key> <Val>
        config get <Key>
        cluster join <Ip:Port>
        cluster members
        cluster status
        vr create tenant <RawReplica1,RawReplica2,..,RawReplicaN>
        vr tenants
        vr replica <Replica>
        vr stats

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
        RawReplica     replica_name::node_name
        Replica        tenant_uuid::replica_name::node_name

    Examples:
        Get the name of the current node in non-interactive mode:
            v2r2-admin 127.0.0.1:2001 -e 'config get node'
    ";
    Error::new(ErrorKind::InvalidInput, string)
}
