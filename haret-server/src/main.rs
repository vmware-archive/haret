// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

extern crate haret;
extern crate rabble;

#[macro_use]
extern crate slog;
extern crate slog_stdlog;
extern crate slog_term;
extern crate slog_envlogger;

use slog::DrainExt;
use std::thread;
use rabble::{Pid, NodeId, Service, TcpServerHandler};
use rabble::serialize::{MsgpackSerializer, ProtobufSerializer};
use haret::config::Config;
use haret::{Msg, NamespaceMgr, DiskMgr};
use haret::admin::{AdminConnectionHandler, AdminMsg};
use haret::api::ApiConnectionHandler;
use haret::api::messages::ApiMsg;

fn main() {

    let config = Config::read();
    let node_id = NodeId {
        name: config.node_name.clone(),
        addr: config.cluster_host.clone()
    };
    /// Set up logging to go to the terminal and be configured via `RUST_LOG`
    let drain = slog_term::streamer().async().full().build();
    let logger = slog::Logger::root(slog_envlogger::EnvLogger::new(drain.fuse()), o!());

    info!(logger, "Starting Rabble Node"; "node_id" => node_id.to_string());
    let (node, mut handles) = rabble::rouse::<Msg>(node_id.clone(), Some(logger.clone()));

    // Create and start the namespace manager
    let namespace_mgr = NamespaceMgr::new(node.clone(), config.clone(), logger.clone());
    info!(logger, "Starting Namespace Manager"; "pid" => namespace_mgr.pid.to_string());
    let mut namespace_mgr_service = Service::new(namespace_mgr.pid.clone(),
                                                 node.clone(),
                                                 namespace_mgr).unwrap();
    handles.push(thread::spawn(move || {
        namespace_mgr_service.wait();
    }));

    // Create and start the Disk Manager
    let disk_mgr = DiskMgr::new(node.clone(), config.data_dir, logger.clone());
    info!(logger, "Starting Disk Manager"; "pid" => disk_mgr.pid.to_string());
    let mut disk_mgr_service = Service::new(disk_mgr.pid.clone(), node.clone(), disk_mgr).unwrap();
    handles.push(thread::spawn(move || {
        disk_mgr_service.wait();
    }));

    // Create and start the admin server
    let admin_pid = Pid {
        name: "admin-server".to_string(),
        group: Some("haret".to_string()),
        node: node_id.clone()
    };
    info!(logger, "Starting Admin Server"; "pid" => admin_pid.to_string(),
                                           "listening" => config.admin_host.clone());
    let handler: TcpServerHandler<AdminConnectionHandler, MsgpackSerializer<AdminMsg>> =
        TcpServerHandler::new(admin_pid.clone(), &config.admin_host, 5000, None);
    let mut admin_service = Service::new(admin_pid, node.clone(), handler).unwrap();
    handles.push(thread::spawn(move || {
        admin_service.wait();
    }));

    // Create and start the API server
    let api_pid = Pid {
        name: "api-server".to_string(),
        group: Some("haret".to_string()),
        node: node_id.clone()
    };
    info!(logger, "Starting API Server"; "pid" => api_pid.to_string(),
                                         "listening" => config.api_host.clone());
    let handler: TcpServerHandler<ApiConnectionHandler, ProtobufSerializer<ApiMsg>> =
        TcpServerHandler::new(api_pid.clone(), &config.api_host, 5000, None);
    let mut api_service = Service::new(api_pid, node.clone(), handler).unwrap();
    handles.push(thread::spawn(move || {
        api_service.wait();
    }));

    for h in handles {
        h.join().unwrap();
    }
}
