extern crate v2r2;

use std::env;
use std::env::Args;
use std::io;
use std::process::exit;
use std::io::{Result, Error, ErrorKind, Write};
use std::str::{SplitWhitespace, FromStr};
use std::net::TcpStream;
use v2r2::resp::{Writer, Reader, Format};
use v2r2::vr_api::{Req, Rsp};
use v2r2::element::{Version, ElementType};

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
    stdout.write_all(b"v2r2> ").unwrap();
    stdout.flush().unwrap();
}

fn parse(command: &str) -> Result<Req> {
    let mut iter = command.split_whitespace();
    match iter.next() {
        Some("create") => parse_create(&mut iter),
        Some("put") => parse_put(&mut iter),
        Some("delete") => parse_delete(&mut iter),
        Some("get") => parse_get(&mut iter),
        Some("list") => parse_list(&mut iter),
        Some(_) => Err(help()),
        None => Err(help())
    }
}

fn parse_create(iter: &mut SplitWhitespace) -> Result<Req> {
    match iter.next() {
        Some(str_type) => match ElementType::from_str(str_type) {
            Ok(ty) => {
                let args: Vec<_> = iter.collect();
                if args.len() != 1 { return Err(help()); }
                let path = args[0].to_string();
                Ok(Req::Create {path: path, ty: ty})
            },
            Err(_) => Err(help())
        },
        None => Err(help())
    }
}

fn parse_put(iter: &mut SplitWhitespace) -> Result<Req> {
    let path = try!(iter.next().ok_or(help()));
    let path = path.to_string();
    let data = try!(iter.next().ok_or(help()));
    let data = data.bytes().collect();
    match iter.next() {
        Some(str_tag) => {
            match Version::from_str(str_tag) {
                Ok(tag) => Ok(Req::Put {path: path, data: data, cas_tag: Some(tag)}),
                Err(_) => {
                    println!("Invalid Version for CAS. Must be of format Epoch:VSN:LSN");
                    Err(help())
                }
            }
        },
        None => Ok(Req::Put {path: path, data: data, cas_tag: None})
    }
}

fn parse_delete(iter: &mut SplitWhitespace) -> Result<Req> {
    let path = try!(iter.next().ok_or(help()));
    let path = path.to_string();
    match iter.next() {
        Some(str_tag) => {
            match Version::from_str(str_tag) {
                Ok(tag) => Ok(Req::Delete {path: path, cas_tag: Some(tag)}),
                Err(_) => {
                    println!("Invalid Version for CAS. Must be of format Epoch:VSN:LSN");
                    Err(help())
                }
            }
        },
        None => Ok(Req::Delete {path: path, cas_tag: None})
    }
}

fn parse_get(iter: &mut SplitWhitespace) -> Result<Req> {
    let path = try!(iter.next().ok_or(help()));
    let path = path.to_string();
    let rv = match iter.next() {
        Some("cas") => Ok(Req::Get {path: path, cas: true}),
        Some(_) => Err(help()),
        None => Ok(Req::Get {path: path, cas: false})
    };
    if iter.count() != 0 { return Err(help()); }
    rv
}

fn parse_list(iter: &mut SplitWhitespace) -> Result<Req> {
    let path = try!(iter.next().ok_or(help()));
    let path = path.to_string();
    if iter.count() != 0 { return Err(help()); }
    Ok(Req::List {path: path})
}

fn exec(msg: Req, sock: &mut TcpStream) -> Result<String> {
    let mut writer = Writer::new();
    writer.format(msg);
    try!(writer.write(sock));
    let mut reader = Reader::<Rsp>::new();
    loop {
        match reader.read(sock) {
            Ok(Some(response)) => {
                return match response {
                    Rsp::Ok => Ok("ok".to_string()),
                    Rsp::Element {data, cas_tag} => {
                        // TODO: The data may not always be utf8
                        let string = String::from_utf8(data).unwrap();
                        match cas_tag {
                            Some(tag) => {
                                Ok(format!("CAS: {}\n{}", tag.to_string(), string))
                            },
                            None => Ok(string)
                        }
                    },
                    Rsp::KeyList {keys} => {
                        Ok(keys.iter().fold(String::new(), |mut acc, k| {
                            acc.push_str(k);
                            acc.push_str("\n");
                            acc
                        }))
                    },
                    Rsp::Error {msg: s} => Ok(s)
                }
            },
            Ok(None) => (),
            Err(e) => return Err(e)
        }
    }
}

fn help() -> Error {
    let string  =
"Usage: v2r2-cli-client <IpAddress> [-e <command>]

    Commands:
        create <Element Type> <Path>
        put <Path> <Data> [CAS Version]
        delete <Path> [CAS Version]
        get <Path> [\"CAS\"]
        list <Path>

    Flags:
        -e <Command>   Non-interactive mode

    Element Types:
        binary
        list
        queue
        set

    Examples:
        Create a node /foo
            v2r2> create binary /foo
        Put data to /foo
            v2r2> put /foo newdata
        Put data only if the version matches existing data
            v2r2> put /foo somedata 0:0:2

    ";
    Error::new(ErrorKind::InvalidInput, string)
}
