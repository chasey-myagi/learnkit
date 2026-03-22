//! Handlers for listing and serving lessons within a program.

use axum::extract::{Path, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use super::error::AppError;
use super::state::{validate_slug, AppState};

/// Escape HTML special characters to prevent XSS when injecting into templates.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    validate_slug(&slug)?;

    let result = tokio::task::spawn_blocking(move || -> Result<serde_json::Value, AppError> {
        match state.open_db(&slug)? {
            Some(conn) => {
                let rows = crate::db::lessons::list_lessons(&conn, None)?;
                serde_json::to_value(&rows)
                    .map_err(|e| AppError::Internal(format!("serialization error: {e}")))
            }
            None => Ok(serde_json::json!([])),
        }
    })
    .await
    .map_err(|e| AppError::Internal(format!("task join error: {e}")))?;

    Ok(Json(result?))
}

/// Embedded static assets (compile-time fallback for CSS/JS).
#[allow(dead_code)]
const EMBEDDED_CSS: &str = include_str!("../../templates/lesson.css");
#[allow(dead_code)]
const EMBEDDED_JS: &str = include_str!("../../templates/lesson.js");

/// Load the shell.html template.
///
/// Priority:
/// 1. `{learnkit_root}/templates/shell.html`
/// 2. `LEARNKIT_TEMPLATE_DIR` env var
///
/// Returns an error if no template is found (no compile-time fallback for shell).
fn load_shell_template(learnkit_root: &std::path::Path) -> Result<String, AppError> {
    // 1. Check in learnkit_root/templates/
    let root_template = learnkit_root.join("templates").join("shell.html");
    if root_template.exists() {
        return std::fs::read_to_string(&root_template)
            .map_err(|e| AppError::Internal(format!("Failed to read shell.html: {e}")));
    }

    // 2. Check LEARNKIT_TEMPLATE_DIR env var
    if let Ok(dir) = std::env::var("LEARNKIT_TEMPLATE_DIR") {
        let env_template = std::path::Path::new(&dir).join("shell.html");
        if env_template.exists() {
            return std::fs::read_to_string(&env_template)
                .map_err(|e| AppError::Internal(format!("Failed to read shell.html: {e}")));
        }
    }

    Err(AppError::Internal(
        "shell.html template not found".to_string(),
    ))
}

/// Dynamically render a lesson page by combining shell.html + body content.
///
/// Route: GET /lessons/:program/lessons/:subject/:lesson
pub async fn serve_lesson(
    State(state): State<Arc<AppState>>,
    Path((program, subject, lesson)): Path<(String, String, String)>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Validate slugs for safety (prevent path traversal)
    validate_slug(&program)?;
    validate_slug(&subject)?;
    validate_slug(&lesson)?;

    let root = state.learnkit_root.clone();

    let result = tokio::task::spawn_blocking(move || -> Result<String, AppError> {
        // 2. Load shell template
        let shell_template = load_shell_template(&root)?;

        // 3. Read body content file: {root}/{program}/lessons/{subject}/{lesson}.html
        let content_path = root
            .join(&program)
            .join("lessons")
            .join(&subject)
            .join(format!("{lesson}.html"));

        if !content_path.exists() {
            return Err(AppError::NotFound(format!(
                "Lesson not found: {program}/{subject}/{lesson}"
            )));
        }

        let content = std::fs::read_to_string(&content_path)
            .map_err(|e| AppError::Internal(format!("Failed to read lesson content: {e}")))?;

        // 4. Build title from lesson slug (HTML-escaped)
        let title = html_escape(&lesson.replace('-', " ").replace('_', " "));
        let safe_program = html_escape(&program);
        let safe_subject = html_escape(&subject);
        let safe_lesson = html_escape(&lesson);

        // 5. Build breadcrumb (using escaped values)
        let breadcrumb = format!(
            "<a href=\"/\">{safe_program}</a> / <a href=\"/\">{safe_subject}</a> / {safe_lesson}"
        );

        // 6. Template substitution
        // IMPORTANT: Replace {{content}} LAST because content may contain placeholder-like strings
        let html = shell_template
            .replace("{{title}}", &title)
            .replace("{{lesson_title}}", &title)
            .replace("{{subject_title}}", &safe_subject)
            .replace("{{program}}", &safe_program)
            .replace("{{subject}}", &safe_subject)
            .replace("{{lesson}}", &safe_lesson)
            .replace("{{api_base}}", &format!("/api/programs/{}", safe_program))
            .replace("{{breadcrumb}}", &breadcrumb)
            .replace("{{prev_link}}", "")
            .replace("{{next_link}}", "")
            .replace("{{prev_title}}", "")
            .replace("{{next_title}}", "")
            .replace("{{content}}", &content);

        Ok(html)
    })
    .await
    .map_err(|e| AppError::Internal(format!("task join error: {e}")))?;

    let html = result?;
    Ok(([(header::CONTENT_TYPE, "text/html; charset=utf-8")], html))
}

/// Serve embedded lesson.css
pub async fn serve_static_css() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/css")], EMBEDDED_CSS)
}

/// Serve embedded lesson.js
pub async fn serve_static_js() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/javascript")],
        EMBEDDED_JS,
    )
}
