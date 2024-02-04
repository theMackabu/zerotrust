use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub struct Backend {
    pub url: url::Url,
    pub providers: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)]
    pub config_path: String,
    pub settings: Settings,
    pub providers: BTreeMap<String, Provider>,
    pub backends: BTreeMap<String, Location>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(alias = "max-age")]
    pub app: App,
    pub server: Server,
    pub max_age: i64,
    pub database: String,
    pub secret: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server {
    #[serde(alias = "static")]
    pub files: String,
    pub prefix: String,
    pub address: String,
    pub port: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub logo: String,
    pub favicon: Option<String>,
    pub accent: String,
    pub pages: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Provider {
    #[serde(alias = "client-id")]
    pub client_id: String,
    #[serde(alias = "client-secret")]
    pub client_secret: String,
    #[serde(alias = "auth-url")]
    pub auth_url: String,
    #[serde(alias = "token-url")]
    pub token_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(alias = "display-name")]
    pub display_name: String,
    pub providers: Vec<String>,
    pub address: String,
    pub port: u16,
    pub tls: Option<bool>,
}
