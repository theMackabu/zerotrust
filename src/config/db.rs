use std::time::Duration;

use diesel::{
    connection::SimpleConnection,
    r2d2::{self, ConnectionManager},
    SqliteConnection,
};

pub type Connection = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

#[derive(Debug)]
pub struct ConnectionOptions {
    pub enable_wal: bool,
    pub enable_foreign_keys: bool,
    pub busy_timeout: Option<Duration>,
}

impl diesel::r2d2::CustomizeConnection<Connection, diesel::r2d2::Error> for ConnectionOptions {
    fn on_acquire(&self, conn: &mut Connection) -> Result<(), diesel::r2d2::Error> {
        (|| {
            if self.enable_wal {
                conn.batch_execute("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
            }
            if self.enable_foreign_keys {
                conn.batch_execute("PRAGMA foreign_keys = ON;")?;
            }
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
            enable_foreign_keys: true,
            busy_timeout: Some(Duration::from_secs(30)),
        }))
        .build(ConnectionManager::<Connection>::new(config.settings.database.clone()))
        .expect("Failed to create pool.");

    return pool;
}
