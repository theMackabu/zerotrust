use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use macros_rs::then;
use std::time::Duration;

use diesel::{
    connection::SimpleConnection,
    r2d2::{self, ConnectionManager},
    sqlite::Sqlite,
    SqliteConnection,
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type Connection = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

#[derive(Debug)]
pub struct ConnectionOptions {
    pub enable_wal: bool,
    pub wal_truncate: bool,
    pub enable_foreign_keys: bool,
    pub busy_timeout: Option<Duration>,
}

impl diesel::r2d2::CustomizeConnection<Connection, diesel::r2d2::Error> for ConnectionOptions {
    fn on_acquire(&self, conn: &mut Connection) -> Result<(), diesel::r2d2::Error> {
        (|| {
            then!(self.enable_wal, conn.batch_execute("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?);
            then!(self.wal_truncate, conn.batch_execute("PRAGMA wal_autocheckpoint = 1000; PRAGMA wal_checkpoint(TRUNCATE);")?);
            then!(self.enable_foreign_keys, conn.batch_execute("PRAGMA foreign_keys = ON;")?);

            if let Some(d) = self.busy_timeout {
                conn.batch_execute(&format!("PRAGMA busy_timeout = {};", d.as_millis()))?;
            }

            Ok(())
        })()
        .map_err(diesel::r2d2::Error::QueryError)
    }
}

pub fn init_db() -> Pool {
    let config = crate::CONFIG.get().unwrap();

    let pool = r2d2::Pool::builder()
        .max_size(16)
        .connection_customizer(Box::new(ConnectionOptions {
            enable_wal: true,
            wal_truncate: true,
            enable_foreign_keys: true,
            busy_timeout: Some(Duration::from_secs(30)),
        }))
        .build(ConnectionManager::<Connection>::new(config.settings.database.clone()))
        .expect("Failed to create pool.");

    return pool;
}

pub fn run_migrations(conn: &mut impl MigrationHarness<Sqlite>) {
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => tracing::info!("migrated records"),
        Err(err) => tracing::error!(err = err.to_string(), "error migrating records"),
    }
}
