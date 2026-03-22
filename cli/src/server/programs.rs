//! Handlers for program listing, scope retrieval, and Q&A history.

use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::error::AppError;
use super::state::{validate_slug, AppState};
use crate::scope;

#[derive(Serialize)]
struct ProgramEntry {
    slug: String,
    title: String,
}

pub async fn list(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let result = tokio::task::spawn_blocking(move || {
        let root = &state.learnkit_root;
        if !root.exists() {
            return Ok(serde_json::json!([]));
        }

        let mut programs = Vec::new();

        let entries = std::fs::read_dir(root)?;
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let scope_path = path.join("scope.md");
            if !scope_path.exists() {
                continue;
            }
            let slug = entry.file_name().to_string_lossy().to_string();
            let title = match std::fs::read_to_string(&scope_path) {
                Ok(content) => match scope::parse_scope(&content) {
                    Ok(s) => s.title,
                    Err(_) => String::new(),
                },
                Err(_) => String::new(),
            };
            programs.push(ProgramEntry { slug, title });
        }

        programs.sort_by(|a, b| a.slug.cmp(&b.slug));
        serde_json::to_value(&programs).map_err(|e| anyhow::anyhow!(e))
    })
    .await
    .map_err(|e| AppError::Internal(format!("task join error: {e}")))?
    .map_err(|e: anyhow::Error| AppError::Internal(format!("{e}")))?;

    Ok(Json(result))
}

pub async fn scope_handler(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    validate_slug(&slug)?;

    let result = tokio::task::spawn_blocking(move || {
        let scope_path = state.learnkit_root.join(&slug).join("scope.md");

        // Check existence explicitly instead of relying on error message string matching
        if !scope_path.exists() {
            return Err(AppError::NotFound(format!(
                "program '{slug}' not found"
            )));
        }

        let content = std::fs::read_to_string(&scope_path)
            .map_err(|e| AppError::Internal(format!("failed to read scope: {e}")))?;
        let parsed = scope::parse_scope(&content)
            .map_err(|e| AppError::Internal(format!("failed to parse scope: {e}")))?;
        serde_json::to_value(&parsed)
            .map_err(|e| AppError::Internal(format!("serialization error: {e}")))
    })
    .await
    .map_err(|e| AppError::Internal(format!("task join error: {e}")))?;

    Ok(Json(result?))
}

#[derive(Deserialize)]
pub struct QaHistoryQuery {
    pub lesson: Option<String>,
}

pub async fn qa_history(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
    Query(query): Query<QaHistoryQuery>,
) -> Result<impl IntoResponse, AppError> {
    validate_slug(&slug)?;

    let result = tokio::task::spawn_blocking(move || {
        match state.open_db(&slug) {
            Some(conn) => {
                let rows = crate::db::qa::list_qa(&conn, query.lesson.as_deref())?;
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
