use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use super::state::{validate_slug, AppState};

pub async fn list(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    validate_slug(&slug)?;

    let result = tokio::task::spawn_blocking(move || {
        match state.open_db(&slug) {
            Some(conn) => {
                let rows = crate::db::lessons::list_lessons(&conn, None)
                    .map_err(|e| anyhow::anyhow!(e))?;
                serde_json::to_value(&rows).map_err(|e| anyhow::anyhow!(e))
            }
            None => Ok(serde_json::json!([])),
        }
    })
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .map_err(|_: anyhow::Error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}
