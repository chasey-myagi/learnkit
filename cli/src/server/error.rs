//! Unified error type for all API handlers.
//!
//! Converts domain errors into structured JSON responses with appropriate HTTP status codes,
//! replacing the previous pattern of returning bare `StatusCode` values.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Application-level error that maps to an HTTP response with a JSON body.
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal Error: {}", msg),
        }
    }
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, axum::Json(serde_json::json!({ "error": message }))).into_response()
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        eprintln!("[learnkit] Database error: {}", e);
        AppError::Internal("An internal error occurred".to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        eprintln!("[learnkit] Internal error: {:#}", e);
        AppError::Internal("An internal error occurred".to_string())
    }
}

/// Database-layer error enum that distinguishes "not found" from internal failures.
pub enum DbError {
    NotFound(String),
    Internal(rusqlite::Error),
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::NotFound(msg) => write!(f, "DB Not Found: {}", msg),
            DbError::Internal(e) => write!(f, "DB Error: {}", e),
        }
    }
}

impl From<rusqlite::Error> for DbError {
    fn from(e: rusqlite::Error) -> Self {
        DbError::Internal(e)
    }
}

impl From<DbError> for AppError {
    fn from(e: DbError) -> Self {
        match e {
            DbError::NotFound(msg) => AppError::NotFound(msg),
            DbError::Internal(e) => {
                eprintln!("[learnkit] Database error: {}", e);
                AppError::Internal("An internal error occurred".to_string())
            }
        }
    }
}
