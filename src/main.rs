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
use config::{db::Pool, structs::Config};
use macros_rs::{crashln, file_exists, str};
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode, DebounceEventResult};
use once_cell::sync::OnceCell;
use std::{path::Path, time::Duration};
use tokio::sync::mpsc;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{filter::LevelFilter, prelude::*};

#[derive(Clone, Parser)]
#[command(version = str!(cli::get_version(false)))]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,
    /// Config path
    #[arg(short, long, default_value = "config.toml")]
    pub config: String,
    #[arg(short, long)]
    /// Override config address
    pub address: Option<String>,
    /// Override config port
    #[arg(short, long)]
    pub port: Option<u16>,
}

#[derive(Debug)]
struct ConfigUpdated;

pub static POOL: OnceCell<Pool> = OnceCell::new();
pub static CONFIG_PATH: OnceCell<String> = OnceCell::new();

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let (reload_tx, mut reload_rx) = mpsc::channel(1);

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

    let mut notify = new_debouncer(Duration::from_millis(250), move |res: DebounceEventResult| match res {
        Ok(_) => {
            tracing::info!("config updated");
            reload_tx.blocking_send(ConfigUpdated).unwrap();
        }
        Err(err) => tracing::error!("file watch error: {err}"),
    })
    .unwrap();

    if let Err(err) = CONFIG_PATH.set(cli.config.clone()) {
        crashln!("Failed to set config path!\n{:?}", err)
    } else {
        if !file_exists!(&cli.config) {
            Config::new().set_path(&cli.config).write();
            tracing::warn!("written initial config, please add postgres details");
            std::process::exit(1);
        }
    }

    let pool = config::db::init_db(&cli.config.clone());
    config::db::run_migrations(&mut pool.get().unwrap());

    if let Err(err) = POOL.set(pool.clone()) {
        crashln!("Failed to set pool!\n{:?}", err)
    };

    notify.watcher().watch(Path::new(&cli.config), RecursiveMode::NonRecursive).unwrap();

    loop {
        let mut server = http::start(pool.clone(), cli.clone());
        let handle = server.handle();

        tokio::select! {
            res = &mut server => {
                res?;
                break;
            },
            Some(_) = reload_rx.recv() => {
                drop(handle.stop(true));
                server.await?;
                continue;
            }
        }
    }

    Ok(())
}
