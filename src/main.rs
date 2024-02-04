mod app;
mod auth;
mod cli;
mod config;
mod helpers;
mod http;
mod models;
mod pages;
mod schema;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use config::db::Pool;
use macros_rs::{crashln, str};
use once_cell::sync::OnceCell;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{filter::LevelFilter, prelude::*};

#[derive(Parser)]
#[command(version = str!(cli::get_version(false)))]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
    /// Config path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    #[arg(short, long)]
    address: Option<String>,
    #[arg(short, long)]
    port: Option<u64>,
}

pub static POOL: OnceCell<Pool> = OnceCell::new();
pub static CONFIG_PATH: OnceCell<String> = OnceCell::new();

fn main() {
    let cli = Cli::parse();

    let formatting_layer = BunyanFormattingLayer::new("server".into(), std::io::stdout)
        .skip_fields(vec!["file", "line"].into_iter())
        .expect("Unable to create logger");

    let level = match cli.verbose.log_level_filter() as usize {
        0 => Some(LevelFilter::OFF),
        1 => Some(LevelFilter::ERROR),
        2 => Some(LevelFilter::WARN),
        3 => Some(LevelFilter::INFO),
        4 => Some(LevelFilter::DEBUG),
        5 => Some(LevelFilter::TRACE),
        _ => None,
    };

    tracing_subscriber::registry()
        .with(level.unwrap_or(LevelFilter::INFO))
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .init();

    if let Err(err) = CONFIG_PATH.set(cli.config.clone()) {
        crashln!("Failed to set config path!\n{:?}", err)
    };

    let pool = config::db::init_db(&cli.config);
    config::db::run_migrations(&mut pool.get().unwrap());

    if let Err(err) = POOL.set(pool.clone()) {
        crashln!("Failed to set config!\n{:?}", err)
    };

    if let Err(err) = http::start(pool, &cli.config) {
        crashln!("Failed to start server!\n{:?}", err)
    };
}
