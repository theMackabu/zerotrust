use chrono::Datelike;
use std::{env, process::Command, time::SystemTime};

fn main() {
    /* git attributes */
    let profile = env::var("PROFILE").unwrap();
    let output = Command::new("git").args(&["rev-parse", "--short=10", "HEAD"]).output().unwrap();
    let output_full = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();

    println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());
    println!("cargo:rustc-env=GIT_HASH={}", String::from_utf8(output.stdout).unwrap());
    println!("cargo:rustc-env=GIT_HASH_FULL={}", String::from_utf8(output_full.stdout).unwrap());

    /* build attributes */
    let date = chrono::Utc::now();
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);
    println!("cargo:rustc-env=BUILD_DATE={}-{}-{}", date.year(), date.month(), date.day());

    /* profile matching */
    match profile.as_str() {
        "debug" => println!("cargo:rustc-env=PROFILE=debug"),
        "release" => println!("cargo:rustc-env=PROFILE=release"),
        _ => println!("cargo:rustc-env=PROFILE=none"),
    }
}
