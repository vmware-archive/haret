// Copyright © 2016-2017 VMware, Inc. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use toml;
use error::VrError;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub node_name: String,
    pub cluster_host: String, // ip:port or dns name used on cluster network
    pub admin_host: String, // ip:port or dns name used for admin interface
    pub api_host: String, // ip:port used for serving vr clients
    pub data_dir: PathBuf
}

impl Config {
    pub fn read() -> Config {
        Config::read_path("config.toml")
    }

    pub fn write(&self) -> Result<(), VrError> {
        self.write_path("config.toml")
    }

    fn read_path(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut string = String::new();
        file.read_to_string(&mut string).unwrap();
        toml::from_str(&string).unwrap()
    }

    pub fn write_path(&self, path: &str) -> Result<(), VrError> {
        let mut file = File::create(path).unwrap();
        let string = toml::to_string(&self).unwrap();
        file.write_all(string.as_bytes())?;
        file.sync_all().map_err(|e| e.into())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_config() {
        let path = "/tmp/haret.toml";
        let config = Config {
            node_name: "node1".to_string(),
            cluster_host: "192.168.1.1:5000".to_string(),
            admin_host: "127.0.0.1:5001".to_string(),
            api_host: "127.0.0.1:5002".to_string(),
            data_dir: PathBuf::from("")
        };

        config.write_path(path).unwrap();
        let config2 = Config::read_path(path);
        assert_eq!(config, config2);
    }
}
