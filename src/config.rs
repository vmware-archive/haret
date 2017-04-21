// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::fs::File;
use std::io::{Read, Write};
use rustc_serialize::json;
use pb_msg;

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Config {
    pub node_name: String,
    pub cluster_host: String, // ip:port or dns name used on cluster network
    pub admin_host: String, // ip:port or dns name used for admin interface
    pub api_host: String // ip:port used for serving vr clients
}

impl Config {
    pub fn read() -> Config {
        Config::read_path("config.json")
    }

    pub fn write(&self) {
        self.write_path("config.json");
    }

    fn read_path(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut string = String::new();
        file.read_to_string(&mut string).unwrap();
        let config: Config = json::decode(&string).unwrap();
        config
    }

    // TODO: return errors instead of crashing
    pub fn write_path(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        let string = json::encode(&self).unwrap().into_bytes();
        file.write_all(&string).unwrap();
    }

}

impl From<Config> for pb_msg::Config {
    fn from(config: Config) -> pb_msg::Config {
        let msg = pb_msg::Config::new();
        msg.set_node_name(config.node_name);
        msg.set_cluster_host(config.cluster_host);
        msg.set_admin_host(config.admin_host);
        msg.set_api_host(config.api_host);
        msg
    }
}

impl From<pb_msg::Config> for Config {
    fn from(msg: pb_msg::Config) -> Config {
        Config {
            node_name: msg.take_node_name(),
            cluster_host: msg.take_cluster_host(),
            admin_host: msg.take_admin_host(),
            api_host: msg.take_api_host()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_config() {
        let path = "/tmp/haret.json";
        let config = Config {
            node_name: "node1".to_string(),
            cluster_host: "192.168.1.1:5000".to_string(),
            admin_host: "127.0.0.1:5001".to_string(),
            api_host: "127.0.0.1:5002".to_string()
        };

        config.write_path(path);
        let config2 = Config::read_path(path);
        assert_eq!(config, config2);
    }
}
