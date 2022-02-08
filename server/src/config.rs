use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "Config::default_ip")]
    pub ip: String,
    
    #[serde(default = "Config::default_port")]
    pub port: u16,
}

impl Config {
    fn default_ip() -> String {
        String::from("127.0.0.1")
    }
    
    fn default_port() -> u16 {
        8080
    }
    
    pub fn load(path: &str) -> Result<Self, toml::de::Error> {
        let file = fs::read(path).unwrap();
        toml::from_slice(&file)
    }
}