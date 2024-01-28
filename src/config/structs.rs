use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub providers: Option<BTreeMap<String, Provider>>,
    pub backends: Option<BTreeMap<String, Location>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub database: String,
    pub address: String,
    pub port: u16,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Provider {
    pub id: String,
    pub secret: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Location {
    pub providers: Vec<String>,
    pub address: String,
    pub port: u16,
}
