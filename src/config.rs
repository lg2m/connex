use serde::Deserialize;
use serde_yaml;
use std::{fs::File, io::Read};
use dirs;

#[derive(Debug, Deserialize)]
pub struct Port {
    pub local: u16,
    pub remote: u16,
}

#[derive(Debug, Deserialize)]
pub struct Application {
    pub name: String,
    pub description: String,
    pub ports: Vec<Port>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub name: String,
    pub host: String,
    pub user: String,
    pub key_path: String,
    pub applications: Vec<Application>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub servers: Vec<Server>,
}

impl Config {
    /// Loads configuration from home directory
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let home_dir = dirs::home_dir().ok_or("Unable to find the home directory.")?;
        let config_path = home_dir.join("connex.yaml");
        let mut file = File::open(config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_yaml::from_str(&contents).map_err(Into::into)
    }
}
