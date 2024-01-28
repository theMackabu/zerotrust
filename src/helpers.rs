use actix_web_static_files::deps::static_files::Resource;
use include_dir::Dir;
use macros_rs::fmtstr;
use mime_guess::MimeGuess;
use std::{collections::HashMap, time::SystemTime};

pub fn build_hashmap(dir: &'static Dir) -> HashMap<&'static str, Resource> {
    let mut map = HashMap::new();

    fn flatten_into(map: &mut HashMap<&'static str, Resource>, dir: &'static Dir) {
        for file in dir.files() {
            let time = file.metadata().unwrap().modified().duration_since(SystemTime::UNIX_EPOCH);
            let modified = if let Ok(modified) = time { modified.as_secs() } else { 0 };

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
