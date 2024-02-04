pub mod db;
pub mod file;
pub mod structs;

use colored::Colorize;
use macros_rs::{clone, crashln, folder_exists, string, ternary};
use std::{collections::BTreeMap, fs};
use structs::{App, Backend, Config, Server, Settings};
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
                database: "users.db".into(),
                secret: "CHANGE ME".into(),
                max_age: 604800,
                server: Server {
                    prefix: "_zero".into(),
                    files: "staticFiles".into(),
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
            crashln!("Error writing config.\n{}", string!(err).white())
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

    pub fn get_state(&self) -> &Self { self }
    pub fn get_mut(&mut self) -> &mut Self { self }
    pub fn set(&mut self, config: Config) { *self = config }
    pub fn read(&self) -> Self { file::read(&self.config_path) }
    pub fn get_static(&self) -> String { self.settings.server.files.to_string() }
    pub fn from_str(contents: &str) -> Self { toml::from_str(contents).map_err(|err| string!(err)).unwrap() }
    pub fn get_address(&self) -> (String, u16) { (self.settings.server.address.to_string(), self.settings.server.port) }
    pub fn edit(&self) -> Document { toml::to_string(self).unwrap().parse::<Document>().expect("Invalid config") }
}
