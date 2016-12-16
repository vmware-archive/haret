extern crate v2r2;
extern crate uuid;
extern crate rabble;
extern crate rustc_serialize;
extern crate rmp_serialize as msgpack;

use std::env;
use std::env::Args;
use std::io;
use std::process::exit;
use std::thread;
use std::io::{Read, Result, Error, ErrorKind, Write};
use std::str::{SplitWhitespace, FromStr};
use std::net::TcpStream;
use std::time;
use std::mem;
use msgpack::{Encoder, Decoder};
use rustc_serialize::{Encodable, Decodable};
use uuid::Uuid;
use rabble::Pid;
use v2r2::vr::{VrClientMsg, VrApiReq, VrApiRsp, ElementType, ClientId, NamespaceId};

fn main() {
    let mut args = env::args();
    // Skip arg0
    let _ = args.next();

    let addr: String = match args.next() {
        Some(api_addr) => api_addr,
        None => {
            println!("Missing IP Address\n{}", help());
            exit(-1);
        }
    };

    let mut client = V2r2Client::new(ClientId(Uuid::new_v4().to_string()));
    client.connect(Some(addr)).unwrap();

    if let Some(flag) = args.next() {
        run_script(&flag, args, client)
    } else {
        run_interactive(client)
    }
}

fn run_interactive(mut client: V2r2Client) {
    loop {
        prompt();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        match run(&command, &mut client) {
            Ok(result) => println!("{}", result),
            Err(err) => println!("{}", err)
        }
    }
}

fn run_script(flag: &str, mut args: Args, mut client: V2r2Client) {
    if flag != "-e" {
        println!("Invalid Flag");
        println!("{}", help());
        exit(-1);
    }
    let command = args.next().unwrap_or(String::new());
    match run(&command, &mut client) {
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

fn run(command: &str, client: &mut V2r2Client) -> Result<String> {
    let req = try!(parse(&command, client));
    exec(req, client)
}

fn prompt() {
    let mut stdout = io::stdout();
    stdout.write_all(b"v2r2> ").unwrap();
    stdout.flush().unwrap();
}

fn parse(command: &str, client: &V2r2Client) -> Result<VrApiReq> {
    let mut iter = command.split_whitespace();
    match iter.next() {
        Some("list-namespaces") =>
            parse_no_args("list-namespaces", &mut iter).map(|_| VrApiReq::GetNamespaces),
        Some("enter") => parse_enter(&mut iter, client),
        Some("create") => parse_create(&mut iter),
        Some("put") => parse_put(&mut iter),
        Some("delete") => parse_delete(&mut iter),
        Some("get") => parse_get(&mut iter),
        Some("list") => parse_list(&mut iter),
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

fn parse_enter(iter: &mut SplitWhitespace, client: &V2r2Client) -> Result<VrApiReq> {
    match iter.next() {
        Some(namespace_id) => {
            let id = NamespaceId(namespace_id.to_string());
            Ok(VrApiReq::RegisterClient(client.client_id.clone(), id))
        },
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


fn exec(req: VrApiReq, client: &mut V2r2Client) -> Result<String> {
    try!(client.write_msg(req));
    match try!(client.read_msg()) {
        VrApiRsp::Namespaces(namespaces) => {
            Ok(namespaces.iter().fold(String::new(), |mut acc, namespace_id | {
                acc.push_str(&namespace_id.0);
                acc.push_str("\n");
                acc
            }))
        },
        VrApiRsp::ClientRegistration {primary, ..} => {
            client.primary = Some(primary);
            Ok(format!("Client registered. Primary = {}", client.primary.as_ref().unwrap()))
        }
        VrApiRsp::Redirect {primary, api_addr} => {
            try!(client.connect(Some(api_addr)));
            let req = try!(client.register(Some(primary.clone())));
            try!(exec(req, client));
            Ok(format!("Finished Redirecting. Primary = {}, API Address = {}",
                       client.primary.as_ref().unwrap(),
                       client.api_addr.as_ref().unwrap()))

        },
        VrApiRsp::Retry(duration) => {
            thread::sleep(time::Duration::from_millis(duration));
            /// Todo: Remove this recursion to prevent potential stack overflow
            let req = try!(client.register(None));
            try!(exec(req, client));
            Ok(format!("Retry complete. Primary = {}, API Address = {}",
                       client.primary.as_ref().unwrap(),
                       client.api_addr.as_ref().unwrap()))
        },
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
        VrApiRsp::ElementNotFoundError(path) => Ok(format!("Element {} Not found", path)),
        VrApiRsp::CasFailedError {path, expected, actual} =>
            Ok(format!("CAS on {} failed. Expected: {}, Actual: {}", path, expected, actual)),
        VrApiRsp::Error {msg: s} => Ok(s)
    }
}

fn help() -> Error {
    let string  =
"Usage: v2r2-cli-client <IpAddress> [-e <command>]

    Commands:
        list-namespaces
        enter <Namespace ID>
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


// TODO: Put V2r2Client into it's own crate
/// This struct represents the V2R2 client implementation in rust. It is a low level client that is
/// useful for building higher level native clients or for building clients in other langauges via
/// FFI.
struct V2r2Client {
    pub client_id: ClientId,
    pub api_addr: Option<String>,
    pub namespace_id: Option<NamespaceId>,
    pub primary: Option<Pid>,
    sock: Option<TcpStream>,
    request_num: u64
}

impl V2r2Client {
    pub fn new(client_id: ClientId) -> V2r2Client {
        V2r2Client {
            client_id: client_id,
            api_addr: None,
            namespace_id: None,
            primary: None,
            sock: None,
            request_num: 0
        }
    }

    /// Connect to `self.api_addr`
    pub fn connect(&mut self, api_addr: Option<String>) -> Result<()> {
        if api_addr.is_none() && self.api_addr.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput,
                              "API Address unknown. Please call connect with an api_addr."));
        }
        if api_addr.is_some() {
            self.api_addr = api_addr;
        }
        self.sock = Some(try!(TcpStream::connect(&self.api_addr.as_ref().unwrap()[..])));
        Ok(())
    }

    /// Register the client id on this node for the given namespace.
    ///
    /// This function returns the registration message to be written or an error if the primary is
    /// unknown.
    pub fn register(&mut self, primary: Option<Pid>) -> Result<VrApiReq> {
        if primary.is_none() && self.primary.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "Primary unknown"));
        }

        if primary.is_some() {
            self.primary = primary;
            self.namespace_id = Some(NamespaceId(self.primary.as_ref().unwrap().group.clone().unwrap()));
        }
        let client_id = self.client_id.clone();
        let namespace_id = self.namespace_id.clone();
        Ok(VrApiReq::RegisterClient(client_id, namespace_id.unwrap()))
    }

    fn write_msg(&mut self, req: VrApiReq) -> Result<()> {
        let mut encoded = Vec::new();
        let req = VrClientMsg::Req {
            pid: self.primary.clone(),
            op: req,
            request_num: self.request_num
        };
        try!(req.encode(&mut Encoder::new(&mut encoded)).map_err(|_| {
            Error::new(ErrorKind::InvalidInput, "Failed to encode msgpack data")
        }));
        let len: u32 = encoded.len() as u32;
        // 4 byte len header
        let header: [u8; 4] = unsafe { mem::transmute(len.to_be()) };
        try!(self.sock.as_ref().unwrap().write_all(&header));
        try!(self.sock.as_ref().unwrap().write_all(&encoded));
        self.request_num += 1;
        Ok(())
    }

    fn read_msg(&mut self) -> Result<VrApiRsp> {
        let mut header = [0; 4];
        try!(self.sock.as_mut().unwrap().read_exact(&mut header));
        let len = unsafe { u32::from_be(mem::transmute(header)) };
        let mut buf = vec![0; len as usize];
        try!(self.sock.as_mut().unwrap().read_exact(&mut buf));
        let mut decoder = Decoder::new(&buf[..]);
        let msg = try!(Decodable::decode(&mut decoder).map_err(|e| {
            Error::new(ErrorKind::InvalidData, e.to_string())
        }));
        match msg {
            VrClientMsg::Rsp {value, ..} => Ok(value),
            req @ VrClientMsg::Req {..} => {
                let msg = format!("Server sent a request instead of a response: {:?}", req);
                Err(Error::new(ErrorKind::InvalidData, msg))
            }
        }
    }
}
