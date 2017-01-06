use std::fs::File;
use std::io::{Result, Read, Write, Error, ErrorKind};
use rustc_serialize::json;

#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Config {
    pub node_name: String,
    pub cluster_host: String, // ip:port or dns name used on cluster network
    pub admin_host: String, // ip:port or dns name used for admin interface
    pub vr_api_host: String // ip:port used for serving vr clients
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
        let path = "/tmp/v2r2.json";
        let config = Config {
            node_name: "node1".to_string(),
            cluster_host: "192.168.1.1:5000".to_string(),
            admin_host: "127.0.0.1:5001".to_string(),
            vr_api_host: "127.0.0.1:5002".to_string()
        };

        config.write_path(path);
        let config2 = Config::read_path(path);
        assert_eq!(config, config2);
    }
}
