use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        Config {
            host: "127.0.0.1".to_string(),
            port: 8080,
        }
    }

    pub fn load(file_path: &str) -> anyhow::Result<Self> {
        let path = Path::new(file_path);
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        // test new method
        let config = Config::new();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);

        // test load method
        // write a file to disk
        let file_path = "test_config.json";
        let config = Config::new();
        let serialized = serde_json::to_string(&config).unwrap();  
        let mut file = File::create(file_path).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
        // load the file
        let loaded_config = Config::load(file_path).unwrap();
        assert_eq!(loaded_config.host, config.host);
        assert_eq!(loaded_config.port, config.port);
        // remove the test file
        std::fs::remove_file(file_path).unwrap();
    }
}
