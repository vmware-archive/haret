extern crate uuid;
extern crate rand;
extern crate v2r2;
extern crate fsm;
extern crate time;
extern crate msgpack;
extern crate rustc_serialize;

#[macro_use]
#[path = "../../tests/debugger/mod.rs"]
mod debugger;

#[path = "../../tests/debugger_shared/mod.rs"]
mod debugger_shared;

use std::env;
use std::process::exit;
use std::io;
use std::io::Write;
use std::fmt::Write as FmtWrite;
use uuid::Uuid;
use debugger::{Debugger, Scheduler};
use debugger_shared::{test_setup, Action};
use v2r2::Member;
use v2r2::vr::{Replica};

fn main() {
    let mut args = env::args();
    let filename = match args.nth(1) {
        Some(filename) => filename,
        None => {
            println!("Error: Must supply a file for the debugger to load as the first argument");
            exit(-1);
        }
    };
    let (mut dispatcher, replicas) = test_setup::init_tenant();
    test_setup::elect_initial_leader(&mut dispatcher, &replicas);
    let mut debugger = Debugger::new(Scheduler::new(dispatcher));

    if let Err(e) = debugger.load_schedule(&filename) {
        println!("{}", e);
        exit(-1);
    }

    loop {
        prompt();
        let command = read_line();
        run(&mut debugger, &command);
    }
}

fn run(debugger: &mut Debugger, command: &str) {
    let words: Vec<&str> = command.split_whitespace().collect();
    if words.len() == 0 { return; }
    match words[0] {
        "print" | "p" => print(debugger, &words),
        "jump-fwd" | "jf" => jump_forward(debugger, &words),
        "jump-bwd" | "jb" => jump_backward(debugger, &words),
        "step-fwd" | "sf" =>  step_forward(debugger, &words),
        "step-bwd" | "sb" => step_backward(debugger, &words),
        "reset-diff" | "rd" => debugger.start_diff(),
        "diff" | "d" => show_diff(debugger, &words),
        "status" | "s" => show_status(debugger),
        _ => help()
    }

}

fn jump_forward(debugger: &mut Debugger, words: &Vec<&str>) {
    if words.len() > 2 { return help() }
    if words.len() == 1 {
        debugger.jump_forward();
    } else {
        match words[1].parse::<usize>() {
            Ok(n) => {
                for _ in 0..n {
                    debugger.jump_forward();
                }
            },
            Err(_) => println!("jf requires an integer argument or no argument")
        }
    }
}

fn jump_backward(debugger: &mut Debugger, words: &Vec<&str>) {
    if words.len() > 2 { return help() }
    if words.len() == 1 {
        debugger.jump_backward();
    } else {
        match words[1].parse::<usize>() {
            Ok(n) => {
                for _ in 0..n {
                    debugger.jump_backward();
                }
            },
            Err(_) => println!("jb requires an integer argument or no argument")
        }
    }
}

fn step_forward(debugger: &mut Debugger, words: &Vec<&str>) {
    if words.len() > 2 { return help() }
    if words.len() == 1 {
        debugger.step_forward();
    } else {
        match words[1].parse::<usize>() {
            Ok(n) => {
                for _ in 0..n {
                    debugger.step_forward();
                }
            },
            Err(_) => println!("sf requires an integer argument or no argument")
        }
    }
}

fn step_backward(debugger: &mut Debugger, words: &Vec<&str>) {
    if words.len() > 2 { return help() }
    if words.len() == 1 {
        debugger.step_backward();
    } else {
        match words[1].parse::<usize>() {
            Ok(n) => {
                for _ in 0..n {
                    debugger.step_backward();
                }
            },
            Err(_) => println!("sb requires an integer argument or no argument")
        }
    }
}


fn print(debugger: &Debugger, words: &Vec<&str>) {
    match words.len() {
        2 => print2(debugger, words[1]),
        _ => help()
    }
}

fn print2(debugger: &Debugger, arg: &str) {
    match arg {
        "replicas" => println!("{:?}", debugger.replica_names()),
        _ => {
            if let Some(replica) = parse_replica(arg) {
                match debugger.replica_state(&replica) {
                    None => {
                        println!("Error: Replica does not exist");
                    }
                    Some((state, ctx)) => {
                        println!("State: {}", state);
                        println!("{:#?}", ctx);
                    }
                }
            } else {
                println!("Invalid replica format. Must be of type name::node");
                help();
            }
        }
    }
}

fn parse_replica(replica_str: &str) -> Option<Replica> {
    let v: Vec<&str> = replica_str.split("::").collect();
    if v.len() != 2 { return None }
    let replica_name = v[0].to_string();
    let node_name = v[1].to_string();
    // This is the same data used in test_setup.rs
    let member = Member {
        name: node_name,
        cluster_host: "".to_string(),
        vr_host: "".to_string(),
        vr_api_host: "".to_string()
    };

    let tenant = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
    Some(Replica {
        tenant: tenant,
        name: replica_name,
        node: member
    })
}

fn show_diff(debugger: &Debugger, words: &Vec<&str>) {
    if words.len() != 2 {
        return println!("diff requires a replica as an argument");
    }
    if let Some(replica) = parse_replica(words[1]) {
        match debugger.diff(&replica) {
            Err(err) => println!("{}", err),
            Ok(diff) => println!("{}", diff)
        }
    } else {
        println!("Invalid replica format. Must be of type node:name");
    }
}

fn show_status(debugger: &Debugger) {
    let val = debugger.get_status();
    let mut status = String::new();
    writeln!(&mut status, "Frame count: {}", val.frame_count).unwrap();
    writeln!(&mut status, "Step count: {}", val.step_count).unwrap();

    if let Some(frame) = val.current_test_msg {
        write!(&mut status, "\nCurrent actions: ").unwrap();
        for action in &frame.actions {
            write_action(&mut status, action);
        }
    }

    if let Some(frame) = val.next_test_msg {
        write!(&mut status, "\nNext actions: ").unwrap();
        for action in &frame.actions {
            write_action(&mut status, action);
        }
    }

    if let Some(envelope) = val.last_received_vrmsg {
        writeln!(&mut status, "\nLast received internal message from {}:{} to {}:{}",
                 envelope.from.node.name, envelope.from.name,
                 envelope.to.node.name, envelope.to.name).unwrap();
        writeln!(&mut status, "{:#?}", envelope.msg).unwrap();
    }
    println!("{}", status);
}

fn write_action(status: &mut String, action: &Action) {
    match *action {
        Action::Send(ref replica, ref msg) => {
            writeln!(status, "Send to {}:{}", replica.node.name, replica.name).unwrap();
            writeln!(status, "{:?}", msg).unwrap();
        },
        Action::Stop(ref replica) => {
            writeln!(status, "Stop {}:{}", replica.node.name, replica.name).unwrap();
        },
        Action::Restart(ref replica) => {
            writeln!(status, "Restart {}:{}", replica.node.name, replica.name).unwrap();
        }
    }
}

fn prompt() {
    let mut stdout = io::stdout();
    stdout.write_all(b"dbg> ").unwrap();
    stdout.flush().unwrap();
}

fn read_line() -> String {
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    return command
}

fn help() {
    let string  =
"Usage: debugger-cli <filename>

    Commands:
        (p)rint <arg>        Print the given argument
        jump-fwd | jf [n]    Jump forward n test messages or once if no argument is given
        jump-bwd | jb [n]    Jump backward n test messages or once if no argument is given
        step-fwd | sf [n]    Step forward by n inter-replica messages or one if no argument given
        step-bwd | sb [n]    Step backward by n inter-replica messages or one if no argument given
        reset-diff | rd      Take a snapshot of the state where we want to baseline our diff
        diff | d <replica>   Show the difference between the current state and the baseline
                             Note that the diff baseline is deleted if a user jumps/steps backwards
                             to a frame/step before the diff was taken. Only forward diffs are
                             allowed.
        (s)tatus             Show status about the current location being debugged

    Print Argumentes:
        replicas             Print the names of all the replicas
        <replica>            Print the state and context of the given replica
";
    println!("{}", string);
}
