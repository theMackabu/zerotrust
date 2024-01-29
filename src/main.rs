mod auth;
mod cli;
mod config;
mod helpers;
mod http;
mod pages;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use config::structs::Config;
use macros_rs::{crashln, str};
use once_cell::sync::OnceCell;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{filter::LevelFilter, prelude::*};

#[derive(Parser)]
#[command(version = str!(cli::get_version(false)))]
struct Cli {
    /// test
    #[command(subcommand)]
    command: Commands,
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
}

// add pmc restore command
#[derive(Subcommand)]
enum Commands {
    /// Start the proxy
    #[command(visible_alias = "serve")]
    Start {
        /// Config path
        #[arg(short, long, default_value = "config.toml")]
        config: String,
    },
}

pub static CONFIG: OnceCell<Config> = OnceCell::new();

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

    match cli.command {
        Commands::Start { config } => {
            if let Err(err) = CONFIG.set(config::read(config)) {
                crashln!("Failed to set config!\n{:?}", err)
            };

            if let Err(err) = http::start() {
                crashln!("Failed to start server!\n{:?}", err)
            };
        }
    };
}
