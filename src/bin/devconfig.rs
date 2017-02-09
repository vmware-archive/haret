// Copyright 2017 VMware, Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate haret;

use std::env;
use haret::config::Config;

fn main() {
    // name = dev1, dev2 .. devN
    let name = env::args().nth(1).unwrap();
    let mut n: u32 = (&name[3..]).to_string().parse().unwrap();
    n = 1000 + n * 1000;

    let cluster_port = n.to_string();
    let admin_port = (n+1).to_string();
    let api_port = (n+2).to_string();

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
        api_host: api_host,
    };
    config.write_path("config.json");
}
