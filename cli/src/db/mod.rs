pub mod schema;
pub mod lessons;
pub mod sections;
pub mod qa;
pub mod resources;

use rusqlite::Connection;
use anyhow::Result;
use crate::config;

pub fn open(program: &str) -> Result<Connection> {
    let db_path = config::program_root(program).join("learnkit.db");
    let conn = Connection::open(&db_path)?;
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;
    schema::ensure_tables(&conn)?;
    Ok(conn)
}
