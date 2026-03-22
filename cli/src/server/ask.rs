//! Handlers for the Ask API — submit questions and poll for answers.

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

use super::error::AppError;
use super::state::{validate_slug, validate_lesson_path, AppState};

#[derive(Deserialize)]
pub struct AskRequest {
    pub selection: String,
    pub question: String,
    #[serde(rename = "lessonPath")]
    pub lesson_path: String,
}

/// POST /api/programs/:slug/ask
///
/// Accepts a question about selected text in a lesson, spawns a background
/// task to get the answer, and immediately returns a `requestId` for polling.
pub async fn submit_ask(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
    Json(payload): Json<AskRequest>,
) -> Result<impl IntoResponse, AppError> {
    validate_slug(&slug)?;
    validate_lesson_path(&payload.lesson_path)?;

    let request_id = format!("q-{}", uuid::Uuid::new_v4().simple());

    // Clone values for the background task
    let bg_root = state.learnkit_root.clone();
    let bg_slug = slug.clone();
    let bg_rid = request_id.clone();
    let bg_selection = payload.selection;
    let bg_question = payload.question;
    let bg_lesson_path = payload.lesson_path;

    tokio::spawn(async move {
        let answer_dir = bg_root.join(&bg_slug).join("answers");
        if let Err(e) = std::fs::create_dir_all(&answer_dir) {
            eprintln!("[learnkit] Failed to create answers dir: {e}");
            return;
        }

        let answer_path = answer_dir.join(format!("{}.json", bg_rid));

        let output = tokio::process::Command::new("claude")
            .arg("-p")
            .arg("--permission-mode")
            .arg("bypassPermissions")
            .arg(format!(
                "/learn-answer --request-id {} --program {} --lesson {} --selection '{}' --question '{}'",
                bg_rid, bg_slug, bg_lesson_path,
                bg_selection.replace('\'', "\\'"),
                bg_question.replace('\'', "\\'")
            ))
            .output()
            .await;

        match output {
            Ok(out) if out.status.success() => {
                // Claude should have written the answer file via learnkit CLI.
                // If it didn't, write a fallback.
                if !answer_path.exists() {
                    let answer_text = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    let answer_json = serde_json::json!({
                        "request_id": bg_rid,
                        "lesson": bg_lesson_path,
                        "selection": bg_selection,
                        "question": bg_question,
                        "answer": if answer_text.is_empty() { "回答生成完成，但内容为空。".to_string() } else { answer_text },
                    });
                    if let Err(e) = std::fs::write(&answer_path, serde_json::to_string_pretty(&answer_json).unwrap_or_default()) {
                        eprintln!("[learnkit] Failed to write fallback answer: {e}");
                    }
                }
            }
            Ok(out) => {
                // Command failed — write error answer so client stops polling
                eprintln!("[learnkit] claude command failed with status {}", out.status);
                let stderr = String::from_utf8_lossy(&out.stderr);
                eprintln!("[learnkit] stderr: {}", stderr);
                let error_json = serde_json::json!({
                    "request_id": bg_rid,
                    "lesson": bg_lesson_path,
                    "selection": bg_selection,
                    "question": bg_question,
                    "answer": "抱歉，回答生成失败。请稍后重试。",
                    "error": true,
                });
                let _ = std::fs::write(&answer_path, serde_json::to_string_pretty(&error_json).unwrap_or_default());
            }
            Err(e) => {
                // Command not found or spawn failed — write error answer
                eprintln!("[learnkit] Failed to spawn claude: {e}");
                let error_json = serde_json::json!({
                    "request_id": bg_rid,
                    "answer": format!("无法启动 claude 命令: {e}"),
                    "error": true,
                });
                let _ = std::fs::write(&answer_path, serde_json::to_string_pretty(&error_json).unwrap_or_default());
            }
        }
    });

    Ok(Json(serde_json::json!({ "requestId": request_id })))
}

/// Validate that a request_id is safe (no path traversal).
fn validate_request_id(request_id: &str) -> Result<(), AppError> {
    if request_id.is_empty()
        || request_id.contains("..")
        || request_id.contains('/')
        || request_id.contains('\\')
        || request_id.contains('\0')
    {
        return Err(AppError::BadRequest(format!(
            "invalid request_id: '{request_id}'"
        )));
    }
    Ok(())
}

/// GET /api/programs/:slug/answer/:request_id
///
/// Polls for the answer to a previously submitted question.
/// Returns `{"status": "pending"}` if not ready,
/// `{"status": "done", "answer": "..."}` when ready,
/// or `{"status": "error", "answer": "..."}` on failure.
pub async fn poll_answer(
    State(state): State<Arc<AppState>>,
    Path((slug, request_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    validate_slug(&slug)?;
    validate_request_id(&request_id)?;

    let answer_path = state
        .learnkit_root
        .join(&slug)
        .join("answers")
        .join(format!("{}.json", request_id));

    if !answer_path.exists() {
        return Ok(Json(serde_json::json!({ "status": "pending" })));
    }

    let content = std::fs::read_to_string(&answer_path)
        .map_err(|e| AppError::Internal(format!("failed to read answer file: {e}")))?;

    let parsed: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| AppError::Internal(format!("failed to parse answer JSON: {e}")))?;

    let answer = parsed["answer"]
        .as_str()
        .unwrap_or("（回答内容缺失）")
        .to_string();

    let has_error = parsed["error"].as_bool().unwrap_or(false);

    Ok(Json(serde_json::json!({
        "status": if has_error { "error" } else { "done" },
        "answer": answer,
    })))
}
