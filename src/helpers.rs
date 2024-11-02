use actix_web_static_files::deps::static_files::Resource;
use include_dir::Dir;
use macros_rs::fmtstr;
use mime_guess::MimeGuess;
use std::{collections::HashMap, time::SystemTime};

pub fn build_hashmap(dir: &'static Dir) -> HashMap<&'static str, Resource> {
    let mut map = HashMap::new();

    fn flatten_into(map: &mut HashMap<&'static str, Resource>, dir: &'static Dir) {
        for file in dir.files() {
            let unix_epoch = SystemTime::UNIX_EPOCH;
            let build_timestamp = env!("BUILD_TIMESTAMP").parse().unwrap_or(0);

            let modified = file
                .metadata()
                .map(|m| m.modified())
                .map(|t| t.duration_since(unix_epoch))
                .and_then(|d| d.ok())
                .map(|d| d.as_secs())
                .unwrap_or(build_timestamp);

            map.insert(
                file.path().to_str().expect("Failed to create path"),
                Resource {
                    modified,
                    data: file.contents(),
                    mime_type: {
                        let mime = MimeGuess::from_path(file.path()).first_or_octet_stream();
                        fmtstr!("{}/{}", mime.type_(), mime.subtype())
                    },
                },
            );
        }
        for subdir in dir.dirs() {
            flatten_into(map, subdir);
        }
    }

    flatten_into(&mut map, dir);

    return map;
}
