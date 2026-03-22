use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

use super::state::{validate_slug, AppState};

pub async fn get_progress(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    validate_slug(&slug)?;

    let result = tokio::task::spawn_blocking(move || {
        match state.open_db(&slug) {
            Some(conn) => {
                let status_counts = crate::db::lessons::count_by_status(&conn)
                    .map_err(|e| anyhow::anyhow!(e))?;
                let (sections_read, sections_total) =
                    crate::db::sections::get_section_progress(&conn)
                        .map_err(|e| anyhow::anyhow!(e))?;

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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .map_err(|_: anyhow::Error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
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
) -> Result<impl IntoResponse, StatusCode> {
    validate_slug(&slug)?;

    // Validate status enum
    match body.status.as_str() {
        "in_progress" | "completed" => {}
        _ => return Err(StatusCode::BAD_REQUEST),
    }

    let result = tokio::task::spawn_blocking(move || {
        // Check if program directory exists
        let program_dir = state.learnkit_root.join(&slug);
        if !program_dir.exists() {
            return Err(StatusCode::NOT_FOUND);
        }

        let conn = match state.open_db(&slug) {
            Some(c) => c,
            None => {
                // DB doesn't exist yet — create it
                let db_path = program_dir.join("learnkit.db");
                let c = rusqlite::Connection::open(&db_path)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                c.execute_batch("PRAGMA journal_mode=WAL;")
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                crate::db::schema::ensure_tables(&c)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                c
            }
        };

        // Mark section as read
        crate::db::sections::mark_section_read(&conn, &body.lesson_path, &body.section)
            .map_err(|_| StatusCode::NOT_FOUND)?;

        // Update lesson status
        match body.status.as_str() {
            "in_progress" | "completed" => {
                crate::db::lessons::update_lesson_status(&conn, &body.lesson_path, &body.status)
                    .map_err(|_| StatusCode::NOT_FOUND)?;
            }
            _ => {} // Already validated above, but keep for safety
        }

        Ok::<_, StatusCode>(serde_json::json!({ "ok": true }))
    })
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)??;

    Ok(Json(result))
}

pub async fn prepare_status(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    validate_slug(&slug)?;

    let result = tokio::task::spawn_blocking(move || {
        match state.open_db(&slug) {
            Some(conn) => {
                let ready_count = crate::db::lessons::count_prepared_unfinished(&conn)
                    .map_err(|e| anyhow::anyhow!(e))?;

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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .map_err(|_: anyhow::Error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}
