// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

extern crate haret_client;
extern crate uuid;

#[macro_use]
extern crate lazy_static;

use std::env;
use std::str;
use std::env::Args;
use std::io;
use std::process::exit;
use std::io::{Result, Error, ErrorKind, Write};
use uuid::Uuid;
use haret_client::HaretClient;

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

    let mut client = HaretClient::new(Uuid::new_v4().to_string());
    client.connect(Some(addr)).unwrap();

    if let Some(flag) = args.next() {
        run_script(&flag, args, client)
    } else {
        run_interactive(client)
    }
}

fn run_interactive(mut client: HaretClient) {
    loop {
        prompt();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        if command == "help\n" {
            println!("{}", help());
        }
        match run(&command, &mut client) {
            Ok(result) => println!("{}", result),
            Err(err) => println!("{}", err)
        }
    }
}

fn run_script(flag: &str, mut args: Args, mut client: HaretClient) {
    if flag != "-e" {
        println!("Invalid Flag");
        println!("{}", help());
        exit(-1);
    }
    let command = args.next().unwrap_or_default();
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

fn run(command: &str, mut client: &mut HaretClient) -> Result<String> {
    let args: Vec<_> = command.split_whitespace().collect();
    for command in commands() {
        if pattern_match(command.pattern, &args) {
            if command.consensus && client.primary.is_none() {
                let msg = "This command must be run inside a namespace. Please call `enter \
                    <namespace_id>`.";
                return Err(Error::new(ErrorKind::InvalidInput, msg));
            }
            return (command.handler)(args, &mut client);
        }
    }
    Err(Error::new(ErrorKind::InvalidInput, "Invalid Input. Type 'help' for commands"))
}

fn prompt() {
    let mut stdout = io::stdout();
    stdout.write_all(b"haret> ").unwrap();
    stdout.flush().unwrap();
}

struct Command {
    pattern: &'static str,
    description: &'static str,
    handler: fn(Vec<&str>, &mut HaretClient) -> Result<String>,
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
        },
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
        if word.starts_with('$') {
            acc.push('<');
            acc.push_str(&word[1..]);
            acc.push('>');
            acc.push(' ');
            return acc;
        }

        // Required option list
        if word.starts_with('*') {
            acc.push('<');
            acc.push_str(&word[1..]);
            acc.push('>');
            acc.push(' ');
            return acc;
        }

        // Remainder of line is space seperated options of the same type
        if word.starts_with('+') {
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
    let mut s = "Usage: haret-cli-client <IpAddress> [-e <command>]\n\n".to_string();
    s.push_str("    Commands\n");
    let help_patterns: Vec<_> = commands.iter().map(|c| pattern_to_help_string(c.pattern)).collect();
    let column2_pos = help_patterns.iter().fold(0, |acc, p| {
        if p.len() > acc {
            return p.len();
        }
        acc
    }) + 8;
    for (command, pattern) in commands.iter().zip(help_patterns) {
        s.push_str(&pattern);
        s.push_str(str::from_utf8(&vec![b' '; column2_pos - pattern.len()]).unwrap());
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

fn pattern_match(pattern: &'static str, argv: &[&str]) -> bool {
    let split_pattern = pattern.split_whitespace();
    let argv = argv.iter();
    let mut iter = split_pattern.zip(argv.clone());
    let mut varargs = false;
    let matched = iter.all(|(pattern, arg)| {
        if pattern.is_empty() && arg.is_empty() {
            return true;
        }
        if pattern.starts_with('$') {
            return true;
        }
        if pattern.starts_with('+') {
            varargs = true;
            return true;
        }
        if pattern.starts_with('*') {
            if pattern[1..].split(',').find(|option| arg == option).is_none() {
                println!("Argument must be one of: {}", pattern);
                return false;
            }
            return true;
        }
        pattern == *arg
    });
    if !matched { return false; }
    if !varargs && pattern.split_whitespace().count() != argv.len() {
        return false;
    }
    true
}

fn list_namespaces(_: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    client.list_namespaces()
}

fn enter_namespace(argv: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    client.enter_namespace(argv[1])
}

fn ls(_: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    client.ls()
}

fn create(mut args: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    let path = args.pop().unwrap();
    let str_type = args.pop().unwrap();
    client.create(path, str_type)
}

fn blob_put(mut args: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    let blob = args.pop().unwrap();
    let path = args.pop().unwrap();
    client.blob_put(blob, path)
}

fn blob_get(mut args: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    let path = args.pop().unwrap();
    client.blob_get(path)
}

fn blob_size(mut args: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    let path = args.pop().unwrap();
    client.blob_size(path)
}

fn queue_push(mut args: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    let blob = args.pop().unwrap();
    let path = args.pop().unwrap();
    client.queue_push(blob, path)
}

fn queue_pop(mut args: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    let path = args.pop().unwrap();
    client.queue_pop(path)
}

fn queue_front(mut args: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    let path = args.pop().unwrap();
    client.queue_front(path)
}

fn queue_back(mut args: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    let path = args.pop().unwrap();
    client.queue_back(path)
}

fn queue_len(mut args: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    let path = args.pop().unwrap();
    client.queue_len(path)
}

fn set_insert(mut args: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    let blob = args.pop().unwrap();
    let path = args.pop().unwrap();
    client.set_insert(blob, path)
}

fn set_remove(mut args: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    let blob = args.pop().unwrap();
    let path = args.pop().unwrap();
    client.set_remove(blob, path)
}

fn set_contains(mut args: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    let blob = args.pop().unwrap();
    let path = args.pop().unwrap();
    client.set_contains(blob, path)
}

fn set_union(args: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    client.set_union(args.into_iter().skip(2))
}

fn set_intersection(args: Vec<&str>, client: &mut HaretClient) -> Result<String> {
    let mut iter = args.into_iter().skip(2);
    let path1 = iter.next().unwrap();
    let path2 = iter.next().unwrap();
    client.set_intersection(path1, path2)
}

fn help() -> Error {
    Error::new(ErrorKind::InvalidInput, HELP.clone())
}
