//! Shared application state and request validation utilities.

use crate::server::error::AppError;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub learnkit_root: PathBuf,
    pub preparing_lessons: Arc<Mutex<HashSet<String>>>,
    /// Tracks which program DBs have already been schema-initialized in this process.
    initialized_dbs: Mutex<HashSet<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            learnkit_root: crate::config::learnkit_root(),
            preparing_lessons: Arc::new(Mutex::new(HashSet::new())),
            initialized_dbs: Mutex::new(HashSet::new()),
        }
    }

    /// Create state with a custom root path. Used by integration tests and
    /// scenarios where the default config root is not appropriate.
    #[allow(dead_code)]
    pub fn with_root(root: PathBuf) -> Self {
        Self {
            learnkit_root: root,
            preparing_lessons: Arc::new(Mutex::new(HashSet::new())),
            initialized_dbs: Mutex::new(HashSet::new()),
        }
    }

    /// Open a DB connection for a program.
    ///
    /// Returns `Ok(None)` if the DB file doesn't exist yet. Returns `Err` if the
    /// file exists but cannot be opened. On first open per slug, runs
    /// `ensure_tables`; subsequent opens skip schema setup for efficiency.
    pub fn open_db(&self, slug: &str) -> Result<Option<rusqlite::Connection>, AppError> {
        let db_path = self.learnkit_root.join(slug).join("learnkit.db");
        if !db_path.exists() {
            return Ok(None);
        }

        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| AppError::Internal(format!("Failed to open DB: {}", e)))?;

        conn.execute_batch("PRAGMA journal_mode=WAL;")
            .map_err(|e| AppError::Internal(format!("Failed to set WAL: {}", e)))?;

        // Only ensure_tables on first open per program slug in this process
        let mut initialized = self.initialized_dbs.lock().unwrap_or_else(|e| e.into_inner());
        if !initialized.contains(slug) {
            crate::db::schema::ensure_tables(&conn)
                .map_err(|e| AppError::Internal(format!("Failed to init schema: {}", e)))?;
            initialized.insert(slug.to_string());
        }

        Ok(Some(conn))
    }

    /// Open or create a DB connection for a program. Used when a write operation
    /// needs to create the DB file if it doesn't exist yet.
    pub fn open_or_create_db(&self, slug: &str) -> Result<rusqlite::Connection, AppError> {
        let db_path = self.learnkit_root.join(slug).join("learnkit.db");

        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| AppError::Internal(format!("failed to open DB: {e}")))?;

        conn.execute_batch("PRAGMA journal_mode=WAL;")
            .map_err(|e| AppError::Internal(format!("failed to set WAL: {e}")))?;

        let needs_init = {
            let initialized = self.initialized_dbs.lock().unwrap_or_else(|e| e.into_inner());
            !initialized.contains(slug)
        };

        if needs_init {
            crate::db::schema::ensure_tables(&conn)
                .map_err(|e| AppError::Internal(format!("failed to ensure tables: {e}")))?;
            let mut initialized = self.initialized_dbs.lock().unwrap_or_else(|e| e.into_inner());
            initialized.insert(slug.to_string());
        }

        Ok(conn)
    }
}

/// Validate that a slug is safe (no path traversal, no special characters).
pub fn validate_slug(slug: &str) -> Result<(), AppError> {
    if slug.len() > 128 {
        return Err(AppError::BadRequest("slug too long".to_string()));
    }
    if slug.is_empty()
        || slug.contains("..")
        || slug.contains('/')
        || slug.contains('\\')
        || slug.contains('\0')
    {
        return Err(AppError::BadRequest(format!("invalid slug: '{slug}'")));
    }
    Ok(())
}

/// Validate that a lesson path has the expected `subject/lesson` format.
/// Rejects empty segments, path traversal (`..`), and paths without exactly one `/`.
pub fn validate_lesson_path(path: &str) -> Result<(), AppError> {
    if path.contains("..") || path.contains('\\') || path.contains('\0') {
        return Err(AppError::BadRequest(format!(
            "invalid lesson path: '{path}'"
        )));
    }

    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err(AppError::BadRequest(format!(
            "lesson path must be 'subject/lesson', got: '{path}'"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_lesson_path_valid() {
        assert!(validate_lesson_path("subject-one/lesson-one").is_ok());
        assert!(validate_lesson_path("math/algebra").is_ok());
    }

    #[test]
    fn test_validate_lesson_path_invalid() {
        assert!(validate_lesson_path("").is_err());
        assert!(validate_lesson_path("no-slash").is_err());
        assert!(validate_lesson_path("/leading-slash").is_err());
        assert!(validate_lesson_path("trailing-slash/").is_err());
        assert!(validate_lesson_path("a/b/c").is_err());
        assert!(validate_lesson_path("../etc").is_err());
        assert!(validate_lesson_path("a\\b").is_err());
    }
}
