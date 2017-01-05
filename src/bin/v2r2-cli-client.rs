extern crate v2r2;
extern crate uuid;
extern crate rustc_serialize;
extern crate protobuf;

#[macro_use]
extern crate lazy_static;

use std::env;
use std::str;
use std::env::Args;
use std::io;
use std::process::exit;
use std::io::{Read, Result, Error, ErrorKind, Write};
use std::net::TcpStream;
use std::mem;
use uuid::Uuid;
use protobuf::{RepeatedField, parse_from_bytes, Message};
use v2r2::api::messages::*;

lazy_static! {
    static ref HELP: String = make_help();
}

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

    let mut client = V2r2Client::new(Uuid::new_v4().to_string());
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
        if &command == "help\n" {
            println!("{}", help());
        }
        match run(command, &mut client) {
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
    match run(command, &mut client) {
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

fn run(command: String, mut client: &mut V2r2Client) -> Result<String> {
    let req = try!(parse(command, &mut client));
    exec(req, client)
}

fn prompt() {
    let mut stdout = io::stdout();
    stdout.write_all(b"v2r2> ").unwrap();
    stdout.flush().unwrap();
}

struct Command {
    pattern: &'static str,
    description: &'static str,
    handler: fn(Vec<&str>, &mut V2r2Client) -> ApiRequest,
    consensus: bool,
}

fn commands() -> Vec<Command> {
    vec![
        Command {
            pattern: "list-namespaces",
            description: "List all namespaces",
            handler: list_namespaces,
            consensus: false
        },
        Command {
            pattern: "enter $namespace_id",
            description: "Enter a namespace to issue consensus requests",
            handler: enter_namespace,
            consensus: false
        },
        Command {
            pattern: "ls",
            description: "List all keys in the current namespace",
            handler: ls,
            consensus: true,
        },
        Command {
            pattern: "create *blob,set,queue $path",
            description: "Create a new node at <path> of type blob, set or queue",
            handler: create,
            consensus: true,
        },
        Command {
            pattern: "blob put $key $val",
            description: "Put a blob to the given key",
            handler: blob_put,
            consensus: true,
        },
        Command {
            pattern: "blob get $key",
            description: "Get a blob from the given key",
            handler: blob_get,
            consensus: true,
        },
        Command {
            pattern: "blob size $key",
            description: "Get the size of the blob at the given key",
            handler: blob_size,
            consensus: true,
        },
        Command {
            pattern : "queue push $path $val",
            description: "Push a blob onto the back of the queue at <path>",
            handler: queue_push,
            consensus: true,
        },
        Command {
            pattern: "queue pop $path",
            description: "Pop a blob off the front of the queue at <path>",
            handler: queue_pop,
            consensus: true,
        },
        Command {
            pattern: "queue front $path",
            description: "Get a copy of the blob at the front of the queue without removing it",
            handler: queue_front,
            consensus: true,
        },
        Command {
            pattern: "queue back $path",
            description: "Get a copy of the blob at the back of the queue without removing it",
            handler: queue_back,
            consensus: true,
        },
        Command {
            pattern: "queue len $path",
            description: "Get the lenght of the queue at <path>",
            handler: queue_len,
            consensus: true,
        },
        Command {
            pattern : "set insert $path $val",
            description: "Insert a blob into the set at <path>",
            handler: set_insert,
            consensus: true,
        },
        Command {
            pattern : "set remove $path $val",
            description: "Remove a blob from the set at <path>",
            handler: set_remove,
            consensus: true,
        },
        Command {
            pattern : "set contains $path $val",
            description: "Return true if the set at <path> contains <val>",
            handler: set_contains,
            consensus: true,
        },
        Command {
            pattern : "set union +path",
            description: "Return the union of the sets at the given paths",
            handler: set_union,
            consensus: true,
        },
        Command {
            pattern: "set intersection $path1 $path2",
            description: "Return the intersection of the sets at the given paths",
            handler: set_intersection,
            consensus: true,
        }
    ]
}

fn pattern_to_help_string(pattern: &str) -> String {
    let split = pattern.split_whitespace();
    let indent = "        ".to_string();
    split.fold(indent, |mut acc, word| {
        if word == "" {
            return acc;
        }

        // Required paramter
        if word.starts_with("$") {
            acc.push('<');
            acc.push_str(&word[1..]);
            acc.push('>');
            acc.push(' ');
            return acc;
        }

        // Required option list
        if word.starts_with("*") {
            acc.push('<');
            acc.push_str(&word[1..]);
            acc.push('>');
            acc.push(' ');
            return acc;
        }

        // Remainder of line is space seperated options of the same type
        if word.starts_with("+") {
            acc.push('<');
            acc.push_str(&word[1..]);
            acc.push_str("1");
            acc.push('>');
            acc.push(' ');
            acc.push('<');
            acc.push_str(&word[1..]);
            acc.push_str("2");
            acc.push('>');
            acc.push(' ');
            acc.push_str("...");
            return acc;
        }

        acc.push_str(word);
        acc.push(' ');
        acc
    })
}

fn make_help() -> String {
    let commands = commands();
    let mut s = "Usage: v2r2-cli-client <IpAddress> [-e <command>]\n\n".to_string();
    s.push_str("    Commands\n");
    let help_patterns: Vec<_> = commands.iter().map(|c| pattern_to_help_string(&c.pattern)).collect();
    let column2_pos = help_patterns.iter().fold(0, |acc, p| {
        if p.len() > acc {
            return p.len();
        }
        acc
    }) + 8;
    for (command, pattern) in commands.iter().zip(help_patterns) {
        s.push_str(&pattern);
        s.push_str(str::from_utf8(&vec![' ' as u8; column2_pos - pattern.len()]).unwrap());
        s.push_str(command.description);
        s.push('\n');
    }
    let rest =
"
    Flags:
        -e <Command>   Non-interactive mode

    Node Types:
        blob
        queue
        set
";
    s.push_str(rest);
    s
}

fn pattern_match(pattern: &'static str, argv: &Vec<&str>) -> bool {
    let split_pattern = pattern.split_whitespace();
    let argv = argv.iter();
    let mut iter = split_pattern.zip(argv.clone());
    let mut varargs = false;
    let matched = iter.all(|(pattern, arg)| {
        if pattern == "" && *arg == "" {
            return true;
        }
        if pattern.starts_with("$") {
            return true;
        }
        if pattern.starts_with("+") {
            varargs = true;
            return true;
        }
        if pattern.starts_with("*") {
            if pattern[1..].split(",").find(|option| arg == option).is_none() {
                println!("Argument must be one of: {}", pattern);
                return false;
            }
            return true;
        }
        pattern == *arg
    });
    if matched == false { return false; }
    if !varargs && pattern.split_whitespace().count() != argv.len() {
        return false;
    }
    true
}

fn parse(argv: String, mut client: &mut V2r2Client) -> Result<ApiRequest> {
    let args: Vec<_> = argv.split_whitespace().collect();
    for command in commands() {
        if pattern_match(&command.pattern, &args) {
            if command.consensus && client.primary.is_none() {
                let msg = "This command must be run inside a namespace. Please call `enter \
                    <namespace_id>`.";
                return Err(Error::new(ErrorKind::InvalidInput, msg));
            }
            return Ok((command.handler)(args, &mut client))
        }
    }
    Err(Error::new(ErrorKind::InvalidInput, "Invalid Input. Type 'help' for commands"))
}

fn list_namespaces(_: Vec<&str>, _: &mut V2r2Client) -> ApiRequest {
    let mut request = ApiRequest::new();
    request.set_get_namespaces(true);
    request
}

fn enter_namespace(argv: Vec<&str>, client: &mut V2r2Client) -> ApiRequest {
    client.reset_primary();
    let mut msg = RegisterClient::new();
    msg.set_client_id(client.client_id.clone());
    msg.set_namespace_id(argv[1].to_string());
    let mut request = ApiRequest::new();
    request.set_register_client(msg);
    request
}

fn ls(_: Vec<&str>, client: &mut V2r2Client) -> ApiRequest {
    let mut list_keys = ListKeys::new();
    list_keys.set_path("/".to_string());

    let mut tree_op = TreeOp::new();
    tree_op.set_list_keys(list_keys);

    let mut consensus_req = ConsensusRequest::new();
    consensus_req.set_to(client.primary.as_ref().unwrap().clone());
    consensus_req.set_client_id(client.client_id.clone());
    consensus_req.set_client_request_num(client.request_num);
    consensus_req.set_tree_op(tree_op);

    let mut api_request = ApiRequest::new();
    api_request.set_consensus_request(consensus_req);
    api_request
}

fn consensus_request(tree_op: TreeOp, client: &mut V2r2Client) -> ApiRequest {
    let mut consensus_req = ConsensusRequest::new();
    consensus_req.set_to(client.primary.as_ref().unwrap().clone());
    consensus_req.set_client_id(client.client_id.clone());
    consensus_req.set_client_request_num(client.request_num);
    consensus_req.set_tree_op(tree_op);
    let mut api_request = ApiRequest::new();
    api_request.set_consensus_request(consensus_req);
    api_request
}

fn create(mut args: Vec<&str>, client: &mut V2r2Client) -> ApiRequest {
    let path = args.pop().unwrap();
    let str_type = args.pop().unwrap();
    let node_type = match &str_type as &str {
        "blob" => NodeType::BLOB,
        "queue" => NodeType::QUEUE,
        "set" => NodeType::SET,
        _ => unreachable!()
    };
    let mut create_node = CreateNode::new();
    create_node.set_path(path.to_string());
    create_node.set_node_type(node_type);
    let mut tree_op = TreeOp::new();
    tree_op.set_create_node(create_node);
    consensus_request(tree_op, client)
}

fn blob_put(mut args: Vec<&str>, client: &mut V2r2Client) -> ApiRequest {
    let blob = args.pop().unwrap();
    let path = args.pop().unwrap();
    let mut blob_put = BlobPut::new();
    blob_put.set_path(path.to_string());
    blob_put.set_val(blob.as_bytes().to_vec());
    let mut tree_op = TreeOp::new();
    tree_op.set_blob_put(blob_put);
    consensus_request(tree_op, client)
}

fn blob_get(mut args: Vec<&str>, client: &mut V2r2Client) -> ApiRequest {
    let path = args.pop().unwrap();
    let mut blob_get = BlobGet::new();
    blob_get.set_path(path.to_string());
    let mut tree_op = TreeOp::new();
    tree_op.set_blob_get(blob_get);
    consensus_request(tree_op, client)
}

fn blob_size(mut args: Vec<&str>, client: &mut V2r2Client) -> ApiRequest {
    let path = args.pop().unwrap();
    let mut blob_size = BlobSize::new();
    blob_size.set_path(path.to_string());
    let mut tree_op = TreeOp::new();
    tree_op.set_blob_size(blob_size);
    consensus_request(tree_op, client)
}

fn queue_push(mut args: Vec<&str>, client: &mut V2r2Client) -> ApiRequest {
    let blob = args.pop().unwrap();
    let path = args.pop().unwrap();
    let mut queue_push = QueuePush::new();
    queue_push.set_path(path.to_string());
    queue_push.set_val(blob.as_bytes().to_vec());
    let mut tree_op = TreeOp::new();
    tree_op.set_queue_push(queue_push);
    consensus_request(tree_op, client)
}

fn queue_pop(mut args: Vec<&str>, client: &mut V2r2Client) -> ApiRequest {
    let path = args.pop().unwrap();
    let mut queue_pop = QueuePop::new();
    queue_pop.set_path(path.to_string());
    let mut tree_op = TreeOp::new();
    tree_op.set_queue_pop(queue_pop);
    consensus_request(tree_op, client)
}

fn queue_front(mut args: Vec<&str>, client: &mut V2r2Client) -> ApiRequest {
    let path = args.pop().unwrap();
    let mut queue_front = QueueFront::new();
    queue_front.set_path(path.to_string());
    let mut tree_op = TreeOp::new();
    tree_op.set_queue_front(queue_front);
    consensus_request(tree_op, client)
}

fn queue_back(mut args: Vec<&str>, client: &mut V2r2Client) -> ApiRequest {
    let path = args.pop().unwrap();
    let mut queue_back = QueueBack::new();
    queue_back.set_path(path.to_string());
    let mut tree_op = TreeOp::new();
    tree_op.set_queue_back(queue_back);
    consensus_request(tree_op, client)
}

fn queue_len(mut args: Vec<&str>, client: &mut V2r2Client) -> ApiRequest {
    let path = args.pop().unwrap();
    let mut queue_len = QueueLen::new();
    queue_len.set_path(path.to_string());
    let mut tree_op = TreeOp::new();
    tree_op.set_queue_len(queue_len);
    consensus_request(tree_op, client)
}

fn set_insert(mut args: Vec<&str>, client: &mut V2r2Client) -> ApiRequest {
    let blob = args.pop().unwrap();
    let path = args.pop().unwrap();
    let mut set_insert = SetInsert::new();
    set_insert.set_path(path.to_string());
    set_insert.set_val(blob.as_bytes().to_vec());
    let mut tree_op = TreeOp::new();
    tree_op.set_set_insert(set_insert);
    consensus_request(tree_op, client)
}

fn set_remove(mut args: Vec<&str>, client: &mut V2r2Client) -> ApiRequest {
    let blob = args.pop().unwrap();
    let path = args.pop().unwrap();
    let mut set_remove = SetRemove::new();
    set_remove.set_path(path.to_string());
    set_remove.set_val(blob.as_bytes().to_vec());
    let mut tree_op = TreeOp::new();
    tree_op.set_set_remove(set_remove);
    consensus_request(tree_op, client)
}

fn set_contains(mut args: Vec<&str>, client: &mut V2r2Client) -> ApiRequest {
    let blob = args.pop().unwrap();
    let path = args.pop().unwrap();
    let mut set_contains = SetContains::new();
    set_contains.set_path(path.to_string());
    set_contains.set_val(blob.as_bytes().to_vec());
    let mut tree_op = TreeOp::new();
    tree_op.set_set_contains(set_contains);
    consensus_request(tree_op, client)
}

fn set_union(args: Vec<&str>, client: &mut V2r2Client) -> ApiRequest {
    let paths: Vec<String> = args.into_iter().skip(2).map(|s| s.to_string()).collect();
    let mut set_union = SetUnion::new();
    set_union.set_paths(RepeatedField::from_vec(paths));
    let mut tree_op = TreeOp::new();
    tree_op.set_set_union(set_union);
    consensus_request(tree_op, client)
}

fn set_intersection(args: Vec<&str>, client: &mut V2r2Client) -> ApiRequest {
    let mut iter = args.into_iter().skip(2);
    let path1 = iter.next().unwrap();
    let path2 = iter.next().unwrap();
    let mut set_intersection = SetIntersection::new();
    set_intersection.set_path1(path1.to_string());
    set_intersection.set_path2(path2.to_string());
    let mut tree_op = TreeOp::new();
    tree_op.set_set_intersection(set_intersection);
    consensus_request(tree_op, client)
}

fn tree_op_result_to_string(mut result: TreeOpResult) -> String {
    let mut s = String::new();

    if result.has_ok() {
        s.push_str("Ok\n");
    } else if result.has_empty() {
        s.push_str("\n");
    } else if result.has_bool() {
        s.push_str(&format!("{}\n", result.get_bool()));
    } else if result.has_blob() {
        s.push_str(&format_blob(result.take_blob()));
    } else if result.has_int() {
        s.push_str(&format!("{:?}\n", result.get_int()));
    } else if result.has_set() {
        s.push_str(&format_set(result.take_set()));
    } else if result.has_keys() {
        for mut key in result.take_keys().take_keys().into_vec() {
            s.push_str(&format!("{}\n", key.take_name()));
        }
    }

    if result.has_optional_version() {
        s.push_str(&format!("Version = {} ", result.get_optional_version()));
    }
    s
}

fn format_blob(blob: Vec<u8>) -> String {
    match String::from_utf8(blob) {
        Ok(s) => format!("{}\n", s),
        Err(e) => format!("{:?}\n", e.into_bytes())
    }
}

fn format_set(mut set: Set) -> String {
    set.take_val().into_vec().into_iter().fold(String::new(), |mut acc, blob| {
        acc.push_str(&format_blob(blob));
        acc
    })
}

fn api_error_to_string(mut error: ApiError) -> String {
    if error.has_not_found() {
        format!("Path Not found: {}", error.take_not_found().take_path())
    } else if error.has_already_exists() {
        format!("Path Already exists: {}", error.take_already_exists().take_path())
    } else if error.has_does_not_exist() {
        format!("Path Does not exist: {}", error.take_does_not_exist().take_path())
    } else if error.has_wrong_type() {
        "Wrong type".to_string()
    } else if error.has_path_must_end_in_directory() {
        format!("Path must end in directory: {}",
                error.take_path_must_end_in_directory().take_path())
    } else if error.has_path_must_be_absolute() {
        "Paths must be absolute".to_string()
    } else if error.has_cas_failed() {
        "Cas Failed".to_string()
    } else if error.has_bad_format() {
        format!("Path is malformatted {}", error.take_bad_format().take_msg())
    } else if error.has_io() {
        format!("IO error: {}", error.take_io().take_msg())
    } else if error.has_encoding() {
        format!("Encoding error: {}", error.take_encoding().take_msg())
    } else if error.has_invalid_cas() {
        format!("Invalid CAS: {}", error.take_invalid_cas().take_msg())
    } else if error.has_msg() {
        error.take_msg()
    } else if error.has_cannot_delete_root() {
        "Cannot delete root".to_string()
    } else if error.has_invalid_msg() {
        "Invalid Message".to_string()
    } else if error.has_timeout() {
        "Timeout".to_string()
    } else if error.has_not_enough_replicas() {
        "Not enough replicas".to_string()
    } else if error.has_bad_epoch() {
        "Bad epoch".to_string()
    } else {
        "Unknown Error".to_string()
    }
}

fn exec(req: ApiRequest, client: &mut V2r2Client) -> Result<String> {
    try!(client.write_msg(req).map_err(|_| {
        Error::new(ErrorKind::NotConnected,
                   "Failed to write to socket. Please restart client and try again".to_string())
    }));
    let mut api_response = try!(client.read_msg().map_err(|_| {
        Error::new(ErrorKind::NotConnected,
                   "Failed to read from socket. Please restart client and try again".to_string())
    }));

    if api_response.has_consensus_reply() {
        let mut consensus_reply = api_response.take_consensus_reply();

        let mut s = String::new();

        if consensus_reply.has_ok() {
            s.push_str("Ok\n");
        }

        if consensus_reply.has_tree_op_result() {
           s.push_str(&tree_op_result_to_string(consensus_reply.take_tree_op_result()));
        }

        if consensus_reply.has_tree_cas_result() {
            for result in consensus_reply.take_tree_cas_result().take_results().into_iter() {
                s.push_str(&tree_op_result_to_string(result));
            }
        }

        if consensus_reply.has_path() {
            s.push_str(&format!("{}", consensus_reply.take_path()));
        }

        if consensus_reply.has_error() {
            s.push_str("Error: ");
            s.push_str(&api_error_to_string(consensus_reply.take_error()));
            s.push('\n');
        }

        s.push_str(&format!("Epoch = {}, View = {}, Client Request Num = {}",
                            consensus_reply.get_epoch(),
                            consensus_reply.get_view(),
                            consensus_reply.get_request_num()));
        return Ok(s);
    }

    if api_response.has_namespaces() {
        let namespaces = api_response.take_namespaces().take_ids().to_vec();
        return Ok(namespaces.iter().fold(String::new(), |mut acc, namespace_id | {
                acc.push_str(&namespace_id);
                acc.push_str("\n");
                acc
        }));
    }

    if api_response.has_client_registration() {
        client.primary = Some(api_response.take_client_registration().take_primary());
        return Ok(format!("Client registered. Primary = {:?}", client.primary.as_ref().unwrap()));
    }

    if api_response.has_redirect() {
        let mut redirect = api_response.take_redirect();
        let primary = redirect.take_primary();
        let api_addr = redirect.take_api_addr();
        try!(client.connect(Some(api_addr)));
        let req = try!(client.register(Some(primary.clone())));
        /// Todo: Remove this recursion to prevent potential stack overflow
        try!(exec(req, client));
        return Ok(format!("Finished Redirecting. Primary = {:?}, API Address = {}",
                   client.primary.as_ref().unwrap(),
                   client.api_addr.as_ref().unwrap()))
    }

    if api_response.has_retry() {
        let duration = api_response.take_retry().get_milliseconds();
        return Ok(format!("Primary not found. Please retry in {} seconds.", duration*1000));
    }

    if api_response.has_unknown_namespace() {
        return Ok("Unknown namespace".to_string());
    }

    if api_response.has_timeout() {
        return Ok("Timeout".to_string());
    }

    Ok(format!("unknown message {:?}", api_response))
}

fn help() -> Error {
    Error::new(ErrorKind::InvalidInput, HELP.clone())
}


// TODO: Put V2r2Client into it's own crate
/// This struct represents the V2r2 client implementation in rust. It is a low level client that is
/// useful for building higher level native clients or for building clients in other langauges via
/// FFI.
struct V2r2Client {
    pub client_id: String,
    pub api_addr: Option<String>,
    pub namespace_id: Option<String>,
    pub primary: Option<ApiPid>,
    sock: Option<TcpStream>,
    request_num: u64
}

impl V2r2Client {
    pub fn new(client_id: String) -> V2r2Client {
        V2r2Client {
            client_id: client_id,
            api_addr: None,
            namespace_id: None,
            primary: None,
            sock: None,
            request_num: 0
        }
    }

    fn reset_primary(&mut self) {
        self.primary = None;
        self.namespace_id = None;
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
    pub fn register(&mut self, primary: Option<ApiPid>) -> Result<ApiRequest> {
        if primary.is_none() && self.primary.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "Primary unknown"));
        }

        if primary.is_some() {
            self.primary = primary;
            self.namespace_id = Some(self.primary.as_ref().unwrap()
                                     .get_group().to_string());
        }
        let namespace_id = self.namespace_id.clone();
        let mut msg = RegisterClient::new();
        msg.set_client_id(self.client_id.clone());
        msg.set_namespace_id(namespace_id.as_ref().unwrap().clone());
        let mut request = ApiRequest::new();
        request.set_register_client(msg);
        Ok(request)
    }

    fn write_msg(&mut self, req: ApiRequest) -> Result<()> {
        let mut msg = ApiMsg::new();
        msg.set_request(req);
        let encoded = try!(msg.write_to_bytes().map_err(|_| {
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

    fn read_msg(&mut self) -> Result<ApiResponse> {
        let mut header = [0; 4];
        try!(self.sock.as_mut().unwrap().read_exact(&mut header));
        let len = unsafe { u32::from_be(mem::transmute(header)) };
        let mut buf = vec![0; len as usize];
        try!(self.sock.as_mut().unwrap().read_exact(&mut buf));
        let mut msg: ApiMsg = try!(parse_from_bytes(&buf[..]).map_err(|e| {
            Error::new(ErrorKind::InvalidData, e.to_string())
        }));
        Ok(msg.take_response())
    }
}
