use colored::Colorize;
use macros_rs::{crashln, string};

pub fn read<T: serde::de::DeserializeOwned>(path: String) -> T {
    let contents = match std::fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(err) => crashln!("Cannot find {path}.\n{}", string!(err).white()),
    };

    match toml::from_str(&contents).map_err(|err| string!(err)) {
        Ok(parsed) => parsed,
        Err(err) => crashln!("Cannot parse {path}.\n{}", err.white()),
    }
}
