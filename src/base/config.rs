extern crate toml;

use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub database: Database,
}

#[derive(Deserialize, Clone)]
pub struct Database {
    pub host: String,
    pub port: Option<u16>,
    pub user: String,
    pub password: String,
    pub db: String,
}

impl Config {
    pub fn new() -> Config {
        let path = Path::new("config.toml");
        let mut file = File::open(&path).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        let cfg: Config = toml::from_str(s.as_str()).unwrap();
        cfg.clone()
    }
}
