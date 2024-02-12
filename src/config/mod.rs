pub mod db;
pub mod file;
pub mod structs;

use colored::Colorize;
use macros_rs::{clone, crashln, folder_exists, string, ternary};
use std::{collections::BTreeMap, fs};
use structs::{App, Backend, Config, Database, Server, Settings};
use toml_edit::Document;

type Backends = BTreeMap<String, Backend>;

impl Config {
    pub fn new() -> Self {
        let mut example_pages = BTreeMap::new();

        example_pages.insert("Contact support".into(), "https://support.example.site".into());
        example_pages.insert("Status".into(), "https://status.example.site".into());

        Self {
            providers: BTreeMap::new(),
            backends: BTreeMap::new(),
            config_path: "config.toml".into(),
            settings: Settings {
                secret: "CHANGE ME".into(),
                max_age: 604800,
                database: Database {
                    name: "".into(),
                    user: "".into(),
                    password: "".into(),
                    address: "".into(),
                    port: 5432,
                },
                server: Server {
                    prefix: "_zero".into(),
                    files: "static_files".into(),
                    address: "127.0.0.1".into(),
                    port: 8080,
                },
                app: App {
                    name: "Zerotrust".into(),
                    logo: "/_zero/static/logo.png".into(),
                    favicon: None,
                    accent: "indigo".into(),
                    pages: example_pages,
                },
            },
        }
    }

    pub fn set_path(&mut self, config_path: &String) -> &mut Self {
        self.config_path = config_path.clone();
        return self;
    }

    pub fn create_dirs(&self) -> &Self {
        let file_path = self.get_static();

        if !folder_exists!(&file_path) {
            fs::create_dir(file_path.to_string()).unwrap();
        }

        return self;
    }

    pub fn write(&self) -> &Self {
        let contents = match toml::to_string(self) {
            Ok(contents) => contents,
            Err(err) => crashln!("Cannot parse config.\n{}", string!(err).white()),
        };

        if let Err(err) = fs::write(&self.config_path, contents) {
            crashln!("Error writing config to {}.\n{}", self.config_path, string!(err).white())
        }

        tracing::info!(path = self.config_path, created = true, "config");

        return self;
    }

    pub fn backends(&self) -> Backends {
        let mut backends: Backends = BTreeMap::new();

        for (name, item) in self.backends.iter() {
            let tls = match item.tls {
                None => "http",
                Some(is_tls) => ternary!(is_tls, "https", "http"),
            };

            let url = format!("{tls}://{}:{}", item.address, item.port);

            backends.insert(
                name.clone(),
                Backend {
                    providers: clone!(item.providers),
                    url: url::Url::parse(&url).unwrap(),
                },
            );
        }

        return backends;
    }

    pub fn get_database(&self) -> String {
        if self.settings.database.user == "" || self.settings.database.name == "" || self.settings.database.address == "" {
            crashln!("Invalid postgres details, check configuration file!");
        }

        format!(
            "postgres://{username}:{password}@{server}/{db_name}",
            username = self.settings.database.user,
            password = self.settings.database.password,
            db_name = self.settings.database.name,
            server = format!("{addr}:{port}", addr = self.settings.database.address, port = self.settings.database.port)
        )
    }

    pub fn override_port(&mut self, port: u16) { self.settings.server.port = port; }
    pub fn override_address(&mut self, address: String) { self.settings.server.address = address; }

    pub fn get_state(&self) -> &Self { self }
    pub fn set(&mut self, config: Config) { *self = config }
    pub fn read(&self) -> Self { file::read(&self.config_path) }
    pub fn get_static(&self) -> String { self.settings.server.files.to_string() }
    pub fn from_str(contents: &str) -> Self { toml::from_str(contents).map_err(|err| string!(err)).unwrap() }
    pub fn get_address(&self) -> (String, u16) { (self.settings.server.address.to_string(), self.settings.server.port) }
    pub fn edit(&self) -> Document { toml::to_string(self).unwrap().parse::<Document>().expect("Invalid config") }
}
