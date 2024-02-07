use super::structs::Config;

pub fn read(path: &String) -> Config {
    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => String::from(""),
    };

    match toml::from_str(&contents) {
        Ok(parsed) => parsed,
        Err(err) => {
            tracing::error!("file parse: {err}");
            super::structs::Config::new()
        }
    }
}
