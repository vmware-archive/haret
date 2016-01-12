use std::fs::File;
use std::io::{Result, Read, Write, Error, ErrorKind};
use serde_json;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub node_name: String,
    pub cluster_name: String,
    pub cluster_host: String, // ip:port or dns name used on cluster network
    pub admin_host: String, // ip:port or dns name used for admin interface
    pub vr_api_host: String, // ip:port used for serving vr clients
    pub vr_host: String // ip:port used by vr protocol

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
        let config: Config = serde_json::from_str(&string).unwrap();
        config
    }

    // TODO: return errors instead of crashing
    pub fn write_path(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        let string = serde_json::to_string(&self).unwrap().into_bytes();
        file.write_all(&string).unwrap();
    }

    pub fn get(&self, key: String) -> Result<String> {
        match &key[..] {
            "node" => Ok(self.node_name.clone()),
            "cluster" => Ok(self.cluster_name.clone()),
            "cluster-host" => Ok(self.cluster_host.clone()),
            "admin-host" => Ok(self.admin_host.clone()),
            "vr-api-host" => Ok(self.vr_api_host.clone()),
            "vr-host" => Ok(self.vr_host.clone()),
            _ => Err(self.err(key))
        }
    }

    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        match &key[..] {
            "node" => {
                self.node_name = val;
                Ok(())
            },
            "cluster" => {
                self.cluster_name = val;
                Ok(())
            },
            "cluster-host" => {
                // TODO: add this node to cluster membership and start cluster server
                self.cluster_host = val;
                Ok(())
            },
            "admin-host" => Err(Error::new(ErrorKind::InvalidInput,
                                          "admin-host is not configurable at runtime")),
            "vr-api-host" => Err(Error::new(ErrorKind::InvalidInput,
                                          "vr-api-host is not configurable at runtime")),
            "vr-host" => Err(Error::new(ErrorKind::InvalidInput,
                                          "vr-host is not configurable at runtime")),
            _ => Err(self.err(key))
        }
    }

    fn err(&self, key: String) -> Error {
        Error::new(ErrorKind::InvalidInput,
                   format!("Config Key: '{}' does not exist.", key))

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
            cluster_name: "cluster1".to_string(),
            cluster_host: "192.168.1.1:5000".to_string(),
            admin_host: "127.0.0.1:5001".to_string(),
            vr_api_host: "127.0.0.1:5002".to_string(),
            vr_host: "127.0.0.1:5003".to_string()
        };

        config.write_path(path);
        let config2 = Config::read_path(path);
        assert_eq!(config, config2);
    }
}
