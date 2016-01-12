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
use uuid::Uuid;
use v2r2::vr::{VrMsg, Replica, ElementType, VrApiReq, VrApiRsp, ClientEnvelope};
use v2r2::{
               NewSessionRequest, NewSessionReply};
use v2r2::frame::{ReadState, WriteState};
use msgpack::{Encoder, from_msgpack};

static mut req_num: u64 = 0;

fn main() {
    let mut args = env::args();
    let tenant_id = Uuid::parse_str(&args.nth(1).unwrap()).unwrap();
    let addr = args.next().unwrap();
    let (primary, sock, session_id) = start_session(tenant_id, addr);
    if let Some(flag) = args.next() {
        run_script(&flag, args, sock, &primary, session_id);
    } else {
        run_interactive(sock, &primary, session_id);
    }
}

fn start_session(tenant_id: Uuid, addr: String) -> (Replica, TcpStream, Uuid) {
    let mut sock = TcpStream::connect(&addr[..]).unwrap();
    send_new_session_request(tenant_id, &mut sock, addr);
    let reply = read_session_reply(&mut sock);
    match reply {
        NewSessionReply::SessionId {session_id, primary} => {
            println!("Session {} created for {}", session_id, primary);
            (primary, sock, session_id)
        },
        NewSessionReply::Redirect {primary} => {
            println!("Redirect to {}", primary);
            (primary, sock, Uuid::nil())
        },
        NewSessionReply::Retry(timeout) => panic!("Retry in {} milliseconds", timeout)
    }
}

fn read_session_reply(sock: &mut TcpStream) -> NewSessionReply {
    let mut reader = ReadState::new();
    loop {
        match reader.read(sock) {
            (_, Ok(Some(data))) => {
                match from_msgpack(&data) {
                    Ok(reply) => return reply,
                    Err(_) => panic!("Failed to decode msgpack data for NewSessionReply")
                }
            },
            (new_reader, Ok(None)) => {
                reader = new_reader;
            },
            (_, Err(e)) => {
                panic!("Failed to read response from server: {}", e);
            }
        }
    }
}

fn send_new_session_request(tenant_id: Uuid, sock: &mut TcpStream, addr: String) {
    let msg = Encoder::to_msgpack(&NewSessionRequest(tenant_id)).unwrap();
    let mut writer = WriteState::new();
    writer = writer.push(msg);
    loop {
        if let Ok((more_to_write, new_writer)) = writer.write(sock) {
            if !more_to_write { break; }
            writer = new_writer;
        } else {
            panic!("Failed to send session request to {}", addr);
        }
    }
}

fn run_interactive(mut sock: TcpStream, replica: &Replica, session_id: Uuid) {
    loop {
        prompt();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        match run(&command, &mut sock, replica, session_id) {
            Ok(result) => println!("{}", result),
            Err(err) => println!("{}", err)
        }
    }
}

fn run_script(flag: &str, mut args: Args, mut sock: TcpStream, replica: &Replica,
              session_id: Uuid) {
    if flag != "-e" {
        println!("Invalid Flag");
        println!("{}", help());
        exit(-1);
    }
    let command = args.next().unwrap_or(String::new());
    match run(&command, &mut sock, replica, session_id) {
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

fn run(command: &str, sock: &mut TcpStream, replica: &Replica, session_id: Uuid) -> Result<String> {
    let req = try!(parse(&command));
    exec(req, sock, replica, session_id)
}

fn prompt() {
    let mut stdout = io::stdout();
    stdout.write_all(b"v2r2> ").unwrap();
    stdout.flush().unwrap();
}

fn parse(command: &str) -> Result<VrApiReq> {
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

fn parse_create(iter: &mut SplitWhitespace) -> Result<VrApiReq> {
    match iter.next() {
        Some(str_type) => match ElementType::from_str(str_type) {
            Ok(ty) => {
                let args: Vec<_> = iter.collect();
                if args.len() != 1 { return Err(help()); }
                let path = args[0].to_string();
                Ok(VrApiReq::Create {path: path, ty: ty})
            },
            Err(_) => Err(help())
        },
        None => Err(help())
    }
}

fn parse_put(iter: &mut SplitWhitespace) -> Result<VrApiReq> {
    let path = try!(iter.next().ok_or(help()));
    let path = path.to_string();
    let data = try!(iter.next().ok_or(help()));
    let data = data.bytes().collect();
    match iter.next() {
        Some(str_tag) => {
            match u64::from_str(str_tag) {
                Ok(tag) => Ok(VrApiReq::Put {path: path, data: data, cas_tag: Some(tag)}),
                Err(_) => {
                    println!("Invalid Version for CAS. Must be an integer");
                    Err(help())
                }
            }
        },
        None => Ok(VrApiReq::Put {path: path, data: data, cas_tag: None})
    }
}

fn parse_delete(iter: &mut SplitWhitespace) -> Result<VrApiReq> {
    let path = try!(iter.next().ok_or(help()));
    let path = path.to_string();
    match iter.next() {
        Some(str_tag) => {
            match u64::from_str(str_tag) {
                Ok(tag) => Ok(VrApiReq::Delete {path: path, cas_tag: Some(tag)}),
                Err(_) => {
                    println!("Invalid Version for CAS. Must be an integer");
                    Err(help())
                }
            }
        },
        None => Ok(VrApiReq::Delete {path: path, cas_tag: None})
    }
}

fn parse_get(iter: &mut SplitWhitespace) -> Result<VrApiReq> {
    let path = try!(iter.next().ok_or(help()));
    let path = path.to_string();
    let rv = match iter.next() {
        Some("cas") => Ok(VrApiReq::Get {path: path, cas: true}),
        Some(_) => Err(help()),
        None => Ok(VrApiReq::Get {path: path, cas: false})
    };
    if iter.count() != 0 { return Err(help()); }
    rv
}

fn parse_list(iter: &mut SplitWhitespace) -> Result<VrApiReq> {
    let path = try!(iter.next().ok_or(help()));
    let path = path.to_string();
    if iter.count() != 0 { return Err(help()); }
    Ok(VrApiReq::List {path: path})
}

fn exec(msg: VrApiReq, sock: &mut TcpStream, replica: &Replica, session_id: Uuid) -> Result<String> {
    unsafe {
        req_num += 1;
    }

    let req = VrMsg::ClientRequest {
        op: msg,
        session_id: session_id,
        request_num: unsafe { req_num }
    };

    let envelope = ClientEnvelope {
        to: replica.clone(),
        msg: req
    };

    let mut writer = WriteState::new();
    writer = writer.push(Encoder::to_msgpack(&envelope).unwrap());
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
                if let VrMsg::ClientReply {value, ..} = from_msgpack(&data).unwrap() {
                    return match value{
                        VrApiRsp::Ok => Ok("ok".to_string()),
                        VrApiRsp::Timeout => Ok("Timeout".to_string()),
                        VrApiRsp::Element {data, cas_tag} => {
                            // TODO: The data may not always be utf8
                            let string = String::from_utf8(data).unwrap();
                            match cas_tag {
                                Some(tag) => {
                                    Ok(format!("CAS: {}\n{}", tag.to_string(), string))
                                },
                                None => Ok(string)
                            }
                        },
                        VrApiRsp::KeyList {keys} => {
                            Ok(keys.iter().fold(String::new(), |mut acc, k| {
                                acc.push_str(k);
                                acc.push_str("\n");
                                acc
                            }))
                        },
                        VrApiRsp::ParentNotFoundError => Ok("Parent path not found".to_string()),
                        VrApiRsp::ElementAlreadyExistsError => Ok("Element already exists".to_string()),
                        VrApiRsp::ElementNotFoundError(path) =>
                            Ok(format!("Element {} Not found", path)),
                        VrApiRsp::CasFailedError {path, expected, actual} =>
                            Ok(format!("CAS on {} failed. Expected: {}, Actual: {}",
                                       path, expected, actual)),
                        VrApiRsp::Error {msg: s} => Ok(s)
                    }
                } else {
                    unreachable!()
                }
            },
            (new_reader, Ok(None)) =>  {
                reader = new_reader;
            },
            (_, Err(_)) => panic!("Failed to read response from server")
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
