//! Handlers for lesson progress tracking — reading sections and updating status.

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

use super::error::{AppError, DbError};
use super::state::{validate_lesson_path, validate_slug, AppState};

pub async fn get_progress(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    validate_slug(&slug)?;

    let result = tokio::task::spawn_blocking(move || -> Result<serde_json::Value, AppError> {
        match state.open_db(&slug)? {
            Some(conn) => {
                let status_counts = crate::db::lessons::count_by_status(&conn)?;
                let (sections_read, sections_total) =
                    crate::db::sections::get_section_progress(&conn)?;

                Ok(serde_json::json!({
                    "lessons": status_counts,
                    "sections": {
                        "read": sections_read,
                        "total": sections_total,
                    }
                }))
            }
            None => Ok(serde_json::json!({
                "lessons": {},
                "sections": {
                    "read": 0,
                    "total": 0,
                }
            })),
        }
    })
    .await
    .map_err(|e| AppError::Internal(format!("task join error: {e}")))?;

    Ok(Json(result?))
}

#[derive(Deserialize)]
pub struct UpdateProgressBody {
    #[serde(rename = "lessonPath")]
    pub lesson_path: String,
    pub section: String,
    pub status: String,
}

pub async fn update_progress(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
    Json(body): Json<UpdateProgressBody>,
) -> Result<impl IntoResponse, AppError> {
    validate_slug(&slug)?;
    validate_lesson_path(&body.lesson_path)?;

    // Validate status enum once, before spawning the blocking task
    match body.status.as_str() {
        "in_progress" | "completed" => {}
        other => {
            return Err(AppError::BadRequest(format!(
                "invalid status: '{other}', expected 'in_progress' or 'completed'"
            )))
        }
    }

    let result = tokio::task::spawn_blocking(move || {
        // Check if program directory exists
        let program_dir = state.learnkit_root.join(&slug);
        if !program_dir.exists() {
            return Err(AppError::NotFound(format!("program '{slug}' not found")));
        }

        let conn = match state.open_db(&slug)? {
            Some(c) => c,
            None => state.open_or_create_db(&slug)?,
        };

        // Mark section as read — distinguish not-found from internal errors
        mark_section_read_checked(&conn, &body.lesson_path, &body.section)?;

        // Update lesson status (already validated above, no redundant check needed)
        update_lesson_status_checked(&conn, &body.lesson_path, &body.status)?;

        Ok::<_, AppError>(serde_json::json!({ "ok": true }))
    })
    .await
    .map_err(|e| AppError::Internal(format!("task join error: {e}")))?;

    Ok(Json(result?))
}

/// Wrapper around `db::sections::mark_section_read` that maps errors to AppError
/// with proper not-found vs internal distinction.
fn mark_section_read_checked(
    conn: &rusqlite::Connection,
    lesson_id: &str,
    title: &str,
) -> Result<(), DbError> {
    let affected = conn.execute(
        "UPDATE sections SET read = 1, read_at = datetime('now') WHERE lesson_id = ?1 AND title = ?2",
        [lesson_id, title],
    )?;

    if affected == 0 {
        return Err(DbError::NotFound(format!(
            "section '{title}' not found in lesson '{lesson_id}'"
        )));
    }
    Ok(())
}

/// Wrapper around `db::lessons::update_lesson_status` that maps errors to AppError
/// with proper not-found vs internal distinction.
fn update_lesson_status_checked(
    conn: &rusqlite::Connection,
    id: &str,
    status: &str,
) -> Result<(), DbError> {
    let affected = match status {
        "in_progress" => conn.execute(
            "UPDATE lessons SET status = ?1, started_at = datetime('now') WHERE id = ?2",
            [status, id],
        )?,
        "completed" => conn.execute(
            "UPDATE lessons SET status = ?1, completed_at = datetime('now') WHERE id = ?2",
            [status, id],
        )?,
        _ => conn.execute(
            "UPDATE lessons SET status = ?1 WHERE id = ?2",
            [status, id],
        )?,
    };

    if affected == 0 {
        return Err(DbError::NotFound(format!("lesson '{id}' not found")));
    }
    Ok(())
}

pub async fn prepare_status(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    validate_slug(&slug)?;

    let result = tokio::task::spawn_blocking(move || -> Result<serde_json::Value, AppError> {
        match state.open_db(&slug)? {
            Some(conn) => {
                let ready_count = crate::db::lessons::count_prepared_unfinished(&conn)?;

                let need_prepare = ready_count <= 1;

                let preparing = state.preparing_lessons.lock().unwrap_or_else(|e| e.into_inner());
                let currently_preparing: Vec<String> = preparing.iter().cloned().collect();

                Ok(serde_json::json!({
                    "needPrepare": need_prepare,
                    "preparedUnfinished": ready_count,
                    "currentlyPreparing": currently_preparing,
                }))
            }
            None => Ok(serde_json::json!({
                "needPrepare": false,
                "preparedUnfinished": 0,
                "currentlyPreparing": [],
            })),
        }
    })
    .await
    .map_err(|e| AppError::Internal(format!("task join error: {e}")))?;

    Ok(Json(result?))
}
