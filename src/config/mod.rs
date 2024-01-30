pub mod db;
pub mod file;
pub mod structs;

use colored::Colorize;
use macros_rs::{crashln, file_exists, folder_exists, string};
use std::{collections::BTreeMap, fs};
use structs::{App, Config, Server, Settings};

pub fn read(config_path: String) -> Config {
    if !file_exists!(&config_path) {
        let mut example_pages = BTreeMap::new();

        example_pages.insert("Contact support".into(), "https://support.example.site".into());
        example_pages.insert("Status".into(), "https://status.example.site".into());

        let config = Config {
            providers: BTreeMap::new(),
            backends: BTreeMap::new(),
            settings: Settings {
                database: "users.db".into(),
                secret: "CHANGE ME".into(),
                max_age: 604800,
                server: Server {
                    prefix: "_sp".into(),
                    files: "sp_files".into(),
                    address: "127.0.0.1".into(),
                    port: 8080,
                },
                app: App {
                    name: "Secure Proxy".into(),
                    logo: "/_sp/static/logo.png".into(),
                    favicon: None,
                    accent: "indigo".into(),
                    pages: example_pages,
                },
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

    let config: Config = file::read(config_path);
    let file_path = &config.get_static();

    if !folder_exists!(file_path) {
        fs::create_dir(file_path.to_string()).unwrap();
    }

    return config;
}

impl Config {
    pub fn get_static(&self) -> String { self.settings.server.files.to_string() }
    pub fn get_address(&self) -> (String, u16) { (self.settings.server.address.to_string(), self.settings.server.port) }
}
