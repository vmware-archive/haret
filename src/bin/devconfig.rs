// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

extern crate haret;

use std::env;
use haret::config::Config;

fn main() {
    // name = dev1, dev2 .. devN
    let name = env::args().nth(1).unwrap();
    let mut n: u32 = (&name[3..]).to_string().parse().unwrap();
    n = 1000 + n * 1000;

    let cluster_port = n.to_string();
    let admin_port = (n + 1).to_string();
    let api_port = (n + 2).to_string();

    let mut cluster_host = "127.0.0.1:".to_string();
    cluster_host.push_str(&cluster_port);

    let mut admin_host = "127.0.0.1:".to_string();
    admin_host.push_str(&admin_port);

    let mut api_host = "127.0.0.1:".to_string();
    api_host.push_str(&api_port);

    let config = Config {
        node_name: name,
        cluster_host: cluster_host,
        admin_host: admin_host,
        api_host: api_host
    };
    config.write_path("config.json").unwrap();
}
