use axum::http::StatusCode;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub learnkit_root: PathBuf,
    pub preparing_lessons: Arc<Mutex<HashSet<String>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            learnkit_root: crate::config::learnkit_root(),
            preparing_lessons: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn with_root(root: PathBuf) -> Self {
        Self {
            learnkit_root: root,
            preparing_lessons: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Open DB connection for a program. Returns None if DB doesn't exist yet.
    pub fn open_db(&self, slug: &str) -> Option<rusqlite::Connection> {
        let db_path = self.learnkit_root.join(slug).join("learnkit.db");
        if !db_path.exists() {
            return None;
        }
        let conn = rusqlite::Connection::open(&db_path).ok()?;
        conn.execute_batch("PRAGMA journal_mode=WAL;").ok()?;
        crate::db::schema::ensure_tables(&conn).ok()?;
        Some(conn)
    }
}

/// Validate that a slug is safe (no path traversal, no special characters).
pub fn validate_slug(slug: &str) -> Result<(), StatusCode> {
    if slug.is_empty()
        || slug.contains("..")
        || slug.contains('/')
        || slug.contains('\\')
        || slug.contains('\0')
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(())
}
