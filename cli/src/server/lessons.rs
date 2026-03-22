//! Handler for listing lessons within a program.

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use super::error::AppError;
use super::state::{validate_slug, AppState};

pub async fn list(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    validate_slug(&slug)?;

    let result = tokio::task::spawn_blocking(move || {
        match state.open_db(&slug) {
            Some(conn) => {
                let rows = crate::db::lessons::list_lessons(&conn, None)?;
                serde_json::to_value(&rows).map_err(|e| anyhow::anyhow!(e))
            }
            None => Ok(serde_json::json!([])),
        }
    })
    .await
    .map_err(|e| AppError::Internal(format!("task join error: {e}")))?
    .map_err(|e: anyhow::Error| AppError::Internal(format!("{e}")))?;

    Ok(Json(result))
}
