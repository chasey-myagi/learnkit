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
        AppError::Internal(format!("database error: {e}"))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError::Internal(format!("{e}"))
    }
}

/// Database-layer error enum that distinguishes "not found" from internal failures.
pub enum DbError {
    NotFound(String),
    Internal(rusqlite::Error),
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
            DbError::Internal(e) => AppError::Internal(format!("database error: {e}")),
        }
    }
}
