use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::BTreeMap};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub providers: BTreeMap<Cow<'static, str>, Provider>,
    pub backends: BTreeMap<Cow<'static, str>, Location>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(alias = "max-age")]
    pub app: App,
    pub server: Server,
    pub max_age: u32,
    pub database: Cow<'static, str>,
    pub secret: Cow<'static, str>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server {
    #[serde(alias = "static")]
    pub files: Cow<'static, str>,
    pub address: Cow<'static, str>,
    pub port: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct App {
    pub name: Cow<'static, str>,
    pub logo: Cow<'static, str>,
    pub accent: Cow<'static, str>,
    pub pages: BTreeMap<Cow<'static, str>, Cow<'static, str>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Provider {
    #[serde(alias = "client-id")]
    pub client_id: Cow<'static, str>,
    #[serde(alias = "client-secret")]
    pub client_secret: Cow<'static, str>,
    #[serde(alias = "auth-url")]
    pub auth_url: Cow<'static, str>,
    #[serde(alias = "token-url")]
    pub token_url: Cow<'static, str>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    pub providers: Vec<Cow<'static, str>>,
    pub address: Cow<'static, str>,
    pub port: u16,
    pub tls: Option<bool>,
}
