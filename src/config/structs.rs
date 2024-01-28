use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub providers: BTreeMap<String, Provider>,
    pub backends: BTreeMap<String, Location>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub database: String,
    pub address: String,
    pub port: u16,
}

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Location {
    pub providers: Vec<String>,
    pub address: String,
    pub port: u16,
}
