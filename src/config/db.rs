use crate::config::structs::Config;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use diesel::{
    pg::{Pg, PgConnection},
    r2d2::{self, ConnectionManager},
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type Connection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn init_db(path: &String) -> Pool {
    let config = Config::new().set_path(path).read();

    let pool = r2d2::Pool::builder()
        .max_size(16)
        .build(ConnectionManager::<Connection>::new(config.get_database()))
        .expect("Failed to create pool.");

    return pool;
}

pub fn run_migrations(conn: &mut impl MigrationHarness<Pg>) {
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => tracing::info!("migrated records"),
        Err(err) => tracing::error!(err = err.to_string(), "error migrating records"),
    }
}
