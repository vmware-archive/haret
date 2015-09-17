extern crate v2r2;

use std::env;
use std::env::Args;
use std::io;
use std::process::exit;
use std::io::{Result, Error, ErrorKind, Write};
use std::str::SplitWhitespace;
use std::net::TcpStream;
use v2r2::resp::{Writer, Reader, Format};
use v2r2::admin::{Req, Res, Msg};

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
            Err(err) => println!("{}", err)
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

fn parse(command: &str) -> Result<Req> {
    let mut iter = command.split_whitespace();
    match iter.next() {
        Some("config") => parse_config(&mut iter),
        Some("cluster") => parse_cluster(&mut iter),
        Some(_) => Err(help()),
        None => Err(help())
    }
}

fn parse_config(iter: &mut SplitWhitespace) -> Result<Req> {
    match iter.next() {
        Some("set") => {
            let args: Vec<_> = iter.collect();
            if args.len() != 2 { return Err(help()); }
            let key = args[0].to_string();
            let val = args[1].to_string();
            Ok(Req::ConfigSet(key, val))
        },
        Some("get") => {
            let args: Vec<_> = iter.collect();
            if args.len() != 1 { return Err(help()); }
            let key = args[0].to_string();
            Ok(Req::ConfigGet(key))
        },
        Some(_) => Err(help()),
        None => Err(help())
    }
}

fn parse_cluster(iter: &mut SplitWhitespace) -> Result<Req> {
    match iter.next() {
        Some("join") => {
            let args: Vec<_> = iter.collect();
            if args.len() != 1 { return Err(help()); }
            let ipstr = args[0].to_string();
            Ok(Req::ClusterJoin(ipstr))
        },
        Some("members") => {
            if iter.count() != 0 { return Err(help()); }
            Ok(Req::ClusterMembers)
        },
        Some("status") => {
            if iter.count() != 0 { return Err(help()); }
            Ok(Req::ClusterStatus)
        },
        Some(_) => Err(help()),
        None => Err(help())
    }
}

fn exec(req: Req, sock: &mut TcpStream) -> Result<String> {
    let mut writer = Writer::new();
    writer.format(Msg::Req(req));
    try!(writer.write(sock));
    let mut reader = Reader::<Msg>::new();
    match reader.read(sock) {
        Ok(Some(Msg::Res(response))) => {
            match response {
                Res::Ok => Ok("ok".to_string()),
                Res::Simple(s) => Ok(s),
                Res::Err(s) => Ok(s)
            }
        },
        Ok(_) => {
            // We shouldn't ever get here since we are using blocking sockets
            Err(Error::new(ErrorKind::InvalidData, "Failed to read response from server"))
        },
        Err(e) => Err(e)
    }
}

fn help() -> Error {
    let string  =
"Usage: v2r2-admin <IpAddress> [-e <command>]

    Commands:
        config set <Key> <Val>
        config get <Key>
        cluster join <ip:port>
        cluster members
        cluster status

    Flags:
        -e <Command>   Non-interactive mode

    Config Keys:
        node           The name of the current node
        cluster        The name of the cluster the node is a member of
        cluster-host   The ip:port of the cluster server listener
        admin-host     The ip:port of the admin server listener

    Examples:
        Get the name of the current node in non-interactive mode:
            v2r2-admin 127.0.0.1:2001 -e 'config get node'
    ";
    Error::new(ErrorKind::InvalidInput, string)
}
