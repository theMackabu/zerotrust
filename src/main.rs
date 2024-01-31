mod app;
mod auth;
mod cli;
mod config;
mod helpers;
mod http;
mod models;
mod pages;
mod schema;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use config::{db::Pool, structs::Config};
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
    /// Config path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

#[derive(Subcommand)]
enum User {
    /// Add a user
    #[command()]
    Add { name: String },
    /// Remove a user
    #[command()]
    Remove { name: String },
    /// Reset user info
    #[command()]
    Reset { name: String },
    /// Link user to provider
    #[command()]
    Link { name: String },
}

// add pmc restore command
#[derive(Subcommand)]
enum Commands {
    /// Start the proxy
    #[command(visible_alias = "serve")]
    Start,
    /// User management
    User {
        #[command(subcommand)]
        command: User,
    },
}

pub static POOL: OnceCell<Pool> = OnceCell::new();
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

    if let Err(err) = CONFIG.set(config::read(cli.config)) {
        crashln!("Failed to set config!\n{:?}", err)
    };

    let pool = config::db::init_db();
    config::db::run_migrations(&mut pool.get().unwrap());

    if let Err(err) = POOL.set(pool.clone()) {
        crashln!("Failed to set config!\n{:?}", err)
    };

    match cli.command {
        Commands::Start => {
            if let Err(err) = http::start(pool) {
                crashln!("Failed to start server!\n{:?}", err)
            };
        }
        Commands::User { command } => match command {
            User::Add { name } => {
                // move to cli.rs
                use inquire::{min_length, Confirm, Password, PasswordDisplayMode, Text};

                let email = Text::new("email:").prompt().unwrap();
                let admin = Confirm::new("admin?").with_default(false).with_help_message("This user has admin powers").prompt().unwrap();

                let password = Password::new("password:")
                    .with_display_toggle_enabled()
                    .with_display_mode(PasswordDisplayMode::Masked)
                    .with_validator(min_length!(10))
                    .with_formatter(&|_| String::from("Matched"))
                    .with_help_message("It is recommended to generate a new one only for this purpose")
                    .with_custom_confirmation_error_message("The passwords don't match.")
                    .prompt()
                    .unwrap();

                let user_dto = models::user::UserDTO {
                    admin,
                    password,
                    username: name.to_string(),
                    email: email.to_lowercase(),
                    providers: serde_json::json!(["basic"]).to_string(),
                    services: serde_json::json!([]).to_string(),
                };

                match models::user::User::signup(user_dto, &mut pool.get().unwrap()) {
                    Ok(message) => println!("{message}"),
                    Err(err) => crashln!("Failed to add user, {err}"),
                }
            }
            User::Remove { name } => {}
            User::Reset { name } => {}
            User::Link { name } => {}
        },
    };
}
