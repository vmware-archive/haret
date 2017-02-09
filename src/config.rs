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

use std::fs::File;
use std::io::{Read, Write};
use rustc_serialize::json;

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
