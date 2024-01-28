use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub backends: Option<BTreeMap<String, Location>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub address: String,
    pub port: u16,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
}
