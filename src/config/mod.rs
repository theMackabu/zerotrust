pub mod structs;

use colored::Colorize;
use macros_rs::{crashln, file_exists, string};
use std::{collections::BTreeMap, fs};
use structs::{Config, Settings};

pub fn read() -> Config {
    let config_path = format!("config.toml");

    if !file_exists!(&config_path) {
        let config = Config {
            providers: BTreeMap::new(),
            backends: BTreeMap::new(),
            settings: Settings {
                database: string!("users.db"),
                address: string!("127.0.0.1"),
                port: 8080,
            },
        };

        let contents = match toml::to_string(&config) {
            Ok(contents) => contents,
            Err(err) => crashln!("Cannot parse config.\n{}", string!(err).white()),
        };

        if let Err(err) = fs::write(&config_path, contents) {
            crashln!("Error writing config.\n{}", string!(err).white())
        }

        tracing::info!(path = config_path, created = true, "config");
    }

    crate::file::read(config_path)
}

impl Config {
    pub fn get_address(&self) -> (String, u16) { (self.settings.address.clone(), self.settings.port.clone()) }
}
