use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;
use std::sync::Arc;
use std::path::PathBuf;
use tempfile::TempDir;

use learnkit::server;

/// Helper: create test app with a temp root path
fn create_test_app(root: PathBuf) -> axum::Router {
    let state = Arc::new(server::state::AppState::with_root(root));
    server::create_router(state)
}

/// Helper: create a test workspace with a program inside a temp dir
fn setup_test_program(dir: &std::path::Path, slug: &str) {
    let program_dir = dir.join(slug);
    std::fs::create_dir_all(&program_dir).unwrap();

    let scope_content = format!(
        r#"---
program: {slug}
title: Test Program
created: 2026-01-01
subjects:
  - slug: subject-one
    title: Subject One
    lessons:
      - slug: lesson-one
        title: Lesson One
        sections:
          - Section A
          - Section B
      - slug: lesson-two
        title: Lesson Two
        sections:
          - Section C
---

# Test Program
"#
    );
    std::fs::write(program_dir.join("scope.md"), &scope_content).unwrap();
}

/// Helper: parse JSON response body
async fn parse_json(response: axum::http::Response<Body>) -> serde_json::Value {
    let body = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body).unwrap()
}

/// Helper: read response body as raw bytes (for non-JSON or error responses)
async fn read_body(response: axum::http::Response<Body>) -> Vec<u8> {
    response.into_body().collect().await.unwrap().to_bytes().to_vec()
}

/// Helper: POST JSON to a URI and return the response
async fn post_json(
    app: axum::Router,
    uri: &str,
    body: serde_json::Value,
) -> axum::http::Response<Body> {
    app.oneshot(
        Request::builder()
            .method("POST")
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap(),
    )
    .await
    .unwrap()
}

/// Helper: assert status code with diagnostic body output on failure
async fn assert_status(response: axum::http::Response<Body>, expected: StatusCode) {
    let status = response.status();
    if status != expected {
        let body_bytes = read_body(response).await;
        let body_str = String::from_utf8_lossy(&body_bytes);
        panic!(
            "Expected status {}, got {}. Response body: {}",
            expected, status, body_str
        );
    }
}

/// Helper: assert that response Content-Type is application/json
fn assert_json_content_type(response: &axum::http::Response<Body>) {
    let ct = response
        .headers()
        .get("content-type")
        .expect("Missing content-type header")
        .to_str()
        .unwrap();
    assert!(
        ct.contains("application/json"),
        "Expected application/json content-type, got: {}",
        ct
    );
}

// ============================================================
// 4.1 Health Check
// ============================================================

#[tokio::test]
async fn test_health() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_json(response).await;
    assert_eq!(body["status"], "ok");
}

// ============================================================
// 4.2 GET /api/programs
// ============================================================

#[tokio::test]
async fn test_list_programs_empty() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    assert!(body.as_array().unwrap().is_empty());
}

#[tokio::test]
async fn test_list_programs_with_program() {
    let tmp = TempDir::new().unwrap();
    setup_test_program(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    let programs = body.as_array().unwrap();
    assert_eq!(programs.len(), 1);
    assert_eq!(programs[0]["slug"], "test-prog");
    assert_eq!(programs[0]["title"], "Test Program");
}

// ============================================================
// 4.3 GET /api/programs/:slug/scope
// ============================================================

#[tokio::test]
async fn test_scope_found() {
    let tmp = TempDir::new().unwrap();
    setup_test_program(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/scope")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    assert_eq!(body["program"], "test-prog");
    assert_eq!(body["title"], "Test Program");
    let subjects = body["subjects"].as_array().unwrap();
    assert_eq!(subjects.len(), 1);
    assert_eq!(subjects[0]["slug"], "subject-one");
}

#[tokio::test]
async fn test_scope_not_found() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/nonexistent/scope")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ============================================================
// 4.4 GET /api/programs/:slug/lessons
// ============================================================

/// Helper: set up a program with DB and registered lessons
fn setup_test_program_with_db(dir: &std::path::Path, slug: &str) {
    setup_test_program(dir, slug);
    // Open DB and insert lessons + sections
    let db_path = dir.join(slug).join("learnkit.db");
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    conn.execute_batch("PRAGMA journal_mode=WAL;").unwrap();
    learnkit::db::schema::ensure_tables(&conn).unwrap();

    learnkit::db::lessons::insert_lesson(
        &conn,
        "subject-one/lesson-one",
        "subject-one",
        "lesson-one",
        "Lesson One",
    )
    .unwrap();
    learnkit::db::sections::insert_sections(
        &conn,
        "subject-one/lesson-one",
        &["Section A".to_string(), "Section B".to_string()],
    )
    .unwrap();

    learnkit::db::lessons::insert_lesson(
        &conn,
        "subject-one/lesson-two",
        "subject-one",
        "lesson-two",
        "Lesson Two",
    )
    .unwrap();
    learnkit::db::sections::insert_sections(
        &conn,
        "subject-one/lesson-two",
        &["Section C".to_string()],
    )
    .unwrap();
}

#[tokio::test]
async fn test_list_lessons() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/lessons")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    let lessons = body.as_array().unwrap();
    assert_eq!(lessons.len(), 2);
    assert_eq!(lessons[0]["id"], "subject-one/lesson-one");
    assert_eq!(lessons[0]["title"], "Lesson One");
    assert_eq!(lessons[0]["status"], "pending");
}

#[tokio::test]
async fn test_list_lessons_no_db() {
    let tmp = TempDir::new().unwrap();
    setup_test_program(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/lessons")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    // No DB → empty list
    assert!(body.as_array().unwrap().is_empty());
}

// ============================================================
// 4.5 GET /api/programs/:slug/progress
// ============================================================

#[tokio::test]
async fn test_get_progress() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/progress")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    // Should have lessons and sections progress
    assert!(body["lessons"].is_object());
    assert!(body["sections"].is_object());
    assert_eq!(body["sections"]["total"], 3); // Section A, B, C
    assert_eq!(body["sections"]["read"], 0);
}

#[tokio::test]
async fn test_get_progress_no_db() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/nonexistent/progress")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    // No DB → empty progress
    assert_eq!(body["sections"]["total"], 0);
    assert_eq!(body["sections"]["read"], 0);
}

// ============================================================
// 4.6 POST /api/programs/:slug/progress
// ============================================================

#[tokio::test]
async fn test_update_progress() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");

    // First, POST to mark a section as read
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/programs/test-prog/progress")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "lessonPath": "subject-one/lesson-one",
                        "section": "Section A",
                        "status": "in_progress"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    assert_eq!(body["ok"], true);

    // Verify progress updated by GET
    let app2 = create_test_app(tmp.path().to_path_buf());
    let response2 = app2
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/progress")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body2 = parse_json(response2).await;
    assert_eq!(body2["sections"]["read"], 1);
    assert_eq!(body2["sections"]["total"], 3);
}

#[tokio::test]
async fn test_update_progress_invalid_section() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/programs/test-prog/progress")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "lessonPath": "subject-one/lesson-one",
                        "section": "Nonexistent Section",
                        "status": "in_progress"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ============================================================
// 4.7 GET /api/programs/:slug/qa-history
// ============================================================

#[tokio::test]
async fn test_qa_history_empty() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/qa-history")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    assert!(body.as_array().unwrap().is_empty());
}

#[tokio::test]
async fn test_qa_history_with_data() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");

    // Insert a QA record
    let db_path = tmp.path().join("test-prog").join("learnkit.db");
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    learnkit::db::qa::insert_qa(
        &conn,
        "q-abc123",
        "subject-one/lesson-one",
        "selected text",
        "what does this mean?",
        "it means this",
    )
    .unwrap();
    drop(conn);

    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/qa-history")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    let history = body.as_array().unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0]["id"], "q-abc123");
    assert_eq!(history[0]["question"], "what does this mean?");
    assert_eq!(history[0]["answer"], "it means this");
}

#[tokio::test]
async fn test_qa_history_no_db() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/nonexistent/qa-history")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    assert!(body.as_array().unwrap().is_empty());
}

// ============================================================
// 4.8 GET /api/programs/:slug/prepare-status
// ============================================================

#[tokio::test]
async fn test_prepare_status_need_prepare() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");
    // All lessons are "pending" — prepared_unfinished == 0, so needPrepare == true
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/prepare-status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    assert_eq!(body["needPrepare"], true);
    assert_eq!(body["preparedUnfinished"], 0);
}

#[tokio::test]
async fn test_prepare_status_enough_prepared() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");

    // Mark both lessons as "prepared" so preparedUnfinished > 1
    let db_path = tmp.path().join("test-prog").join("learnkit.db");
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    learnkit::db::lessons::update_lesson_status(&conn, "subject-one/lesson-one", "prepared")
        .unwrap();
    learnkit::db::lessons::update_lesson_status(&conn, "subject-one/lesson-two", "prepared")
        .unwrap();
    drop(conn);

    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/prepare-status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    assert_eq!(body["needPrepare"], false);
    assert_eq!(body["preparedUnfinished"], 2);
}

#[tokio::test]
async fn test_prepare_status_no_db() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/nonexistent/prepare-status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    assert_eq!(body["needPrepare"], false);
}

// ============================================================
// 5. Additional test scenarios
// ============================================================

// --- 5.1 Boundary: slug with path traversal characters ---

#[tokio::test]
async fn test_slug_path_traversal_rejected() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/..%2F..%2Fetc/scope")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// --- 5.2 Boundary: slug is empty string ---

#[tokio::test]
async fn test_slug_empty_string() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    // "/api/programs//scope" — with {slug} routing, empty segment
    // may not match the route at all → 404 (no matching route)
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs//scope")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Either 400 (validate_slug catches empty) or 404 (route doesn't match)
    let status = response.status();
    assert!(
        status == StatusCode::BAD_REQUEST || status == StatusCode::NOT_FOUND,
        "Expected 400 or 404, got {}",
        status
    );
}

// --- 5.3 Boundary: slug with special characters ---

#[tokio::test]
async fn test_slug_with_backslash_rejected() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/foo%5Cbar/scope")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // validate_slug rejects backslash
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// --- 5.4 Error: POST progress empty body ---

#[tokio::test]
async fn test_update_progress_empty_body() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/programs/test-prog/progress")
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Axum cannot parse empty body as JSON → 400
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// --- 5.5 Error: POST progress malformed JSON ---

#[tokio::test]
async fn test_update_progress_malformed_json() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/programs/test-prog/progress")
                .header("content-type", "application/json")
                .body(Body::from("not json at all"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// --- 5.6 Error: POST progress missing required fields ---

#[tokio::test]
async fn test_update_progress_missing_fields() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/programs/test-prog/progress")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "lessonPath": "subject-one/lesson-one"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Missing "section" and "status" → deserialization fails → 422
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

// --- 5.7 Error: POST progress invalid status value ---

#[tokio::test]
async fn test_update_progress_invalid_status() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/programs/test-prog/progress")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "lessonPath": "subject-one/lesson-one",
                        "section": "Section A",
                        "status": "invalid_value"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// --- 5.8 Error: POST progress nonexistent program ---

#[tokio::test]
async fn test_update_progress_nonexistent_program() {
    let tmp = TempDir::new().unwrap();
    // No program directory created
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/programs/nonexistent/progress")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "lessonPath": "subject-one/lesson-one",
                        "section": "Section A",
                        "status": "in_progress"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// --- 5.10 State: progress lifecycle ---

#[tokio::test]
async fn test_progress_lifecycle() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");

    // 1. Initial state: all sections unread
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/progress")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = parse_json(response).await;
    assert_eq!(body["sections"]["read"], 0);
    assert_eq!(body["sections"]["total"], 3);

    // 2. POST progress → in_progress
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/programs/test-prog/progress")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "lessonPath": "subject-one/lesson-one",
                        "section": "Section A",
                        "status": "in_progress"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 3. GET progress → verify in_progress count
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/progress")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = parse_json(response).await;
    assert_eq!(body["sections"]["read"], 1);
    assert_eq!(body["lessons"]["in_progress"], 1);

    // 4. POST progress → mark another section and complete lesson
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/programs/test-prog/progress")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "lessonPath": "subject-one/lesson-one",
                        "section": "Section B",
                        "status": "completed"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 5. GET progress → verify completed count
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/progress")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = parse_json(response).await;
    assert_eq!(body["sections"]["read"], 2);
    assert_eq!(body["lessons"]["completed"], 1);
}

// --- 5.11 State: idempotent progress update ---

#[tokio::test]
async fn test_update_progress_idempotent() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");

    // POST same section twice
    for _ in 0..2 {
        let app = create_test_app(tmp.path().to_path_buf());
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/programs/test-prog/progress")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "lessonPath": "subject-one/lesson-one",
                            "section": "Section A",
                            "status": "in_progress"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    // Verify count is still 1 (not doubled)
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/progress")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = parse_json(response).await;
    assert_eq!(body["sections"]["read"], 1);
    assert_eq!(body["sections"]["total"], 3);
}

// --- 5.12 State: prepare-status boundary (preparedUnfinished == 1) ---

#[tokio::test]
async fn test_prepare_status_boundary_one() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");

    // Mark exactly 1 lesson as prepared
    let db_path = tmp.path().join("test-prog").join("learnkit.db");
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    learnkit::db::lessons::update_lesson_status(&conn, "subject-one/lesson-one", "prepared")
        .unwrap();
    drop(conn);

    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/prepare-status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    // preparedUnfinished == 1 → need_prepare = (1 <= 1) = true
    assert_eq!(body["preparedUnfinished"], 1);
    assert_eq!(body["needPrepare"], true);
}

// --- 5.13 General: unknown route returns 404 ---

#[tokio::test]
async fn test_unknown_route_404() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// --- 5.14 General: wrong HTTP method ---

#[tokio::test]
async fn test_wrong_method() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/health")
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

// --- 5.15 qa-history with lesson query parameter filter ---

#[tokio::test]
async fn test_qa_history_with_lesson_filter() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");

    // Insert QA records for different lessons
    let db_path = tmp.path().join("test-prog").join("learnkit.db");
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    learnkit::db::qa::insert_qa(
        &conn,
        "q-1",
        "subject-one/lesson-one",
        "text1",
        "question for lesson one",
        "answer one",
    )
    .unwrap();
    learnkit::db::qa::insert_qa(
        &conn,
        "q-2",
        "subject-one/lesson-two",
        "text2",
        "question for lesson two",
        "answer two",
    )
    .unwrap();
    drop(conn);

    // Without filter — should return all 2
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/qa-history")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    assert_eq!(body.as_array().unwrap().len(), 2);

    // With lesson filter — should return only 1
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/qa-history?lesson=subject-one/lesson-one")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    let history = body.as_array().unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0]["id"], "q-1");
    assert_eq!(history[0]["question"], "question for lesson one");
}

// --- 5.16 Health: verify response body content ---

#[tokio::test]
async fn test_health_response_body() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    assert_eq!(body, serde_json::json!({"status": "ok"}));
    // Ensure no extra fields
    assert_eq!(body.as_object().unwrap().len(), 1);
}

// ============================================================
// 6. Extended test scenarios — boundary, coverage, state
// ============================================================

// --- 6.1 Boundary: slug with null byte (%00) ---

#[tokio::test]
async fn test_slug_with_null_byte() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    // %00 is null byte — validate_slug rejects \0
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test%00prog/scope")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// --- 6.2 Boundary: super-long slug (300 chars) ---

#[tokio::test]
async fn test_slug_too_long() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let long_slug = "a".repeat(300);
    let uri = format!("/api/programs/{}/scope", long_slug);

    let response = app
        .oneshot(
            Request::builder()
                .uri(uri.as_str())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Current validate_slug does not limit length, so slug passes validation
    // but the program dir won't exist → 404
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// --- 6.3 Boundary: Unicode slug ---

#[tokio::test]
async fn test_slug_unicode_chinese() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    // Chinese characters — validate_slug doesn't reject unicode (no .., /, \, \0)
    // so it passes validation, but program won't exist → 404
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/%E6%B8%B8%E6%88%8F%E5%BC%80%E5%8F%91/scope")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_slug_unicode_emoji() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    // Emoji slug — similarly passes validate_slug, program not found → 404
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/%F0%9F%8E%AE/scope")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// --- 6.4 Boundary: lessonPath format anomalies ---

#[tokio::test]
async fn test_progress_update_empty_lesson_path() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    // lessonPath = "" — validate_lesson_path rejects empty path → 400
    let response = post_json(
        app,
        "/api/programs/test-prog/progress",
        serde_json::json!({
            "lessonPath": "",
            "section": "Section A",
            "status": "in_progress"
        }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    // Verify error body contains meaningful message
    let body = parse_json(response).await;
    assert!(body["error"].as_str().unwrap().contains("lesson path"));
}

#[tokio::test]
async fn test_progress_update_lesson_path_no_slash() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    // lessonPath = "noslash" — validate_lesson_path rejects (no slash) → 400
    let response = post_json(
        app,
        "/api/programs/test-prog/progress",
        serde_json::json!({
            "lessonPath": "noslash",
            "section": "Section A",
            "status": "in_progress"
        }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    // Verify error body contains meaningful message
    let body = parse_json(response).await;
    assert!(body["error"].as_str().unwrap().contains("lesson path"));
}

// --- 6.5 Boundary: section name with special characters ---

#[tokio::test]
async fn test_progress_update_section_special_chars() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");

    // Insert a section with special characters
    let db_path = tmp.path().join("test-prog").join("learnkit.db");
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    learnkit::db::sections::insert_sections(
        &conn,
        "subject-one/lesson-one",
        &[
            "Section A".to_string(),
            "Section B".to_string(),
            "概述 & 总结".to_string(),
        ],
    )
    .unwrap();
    drop(conn);

    let app = create_test_app(tmp.path().to_path_buf());
    let response = post_json(
        app,
        "/api/programs/test-prog/progress",
        serde_json::json!({
            "lessonPath": "subject-one/lesson-one",
            "section": "概述 & 总结",
            "status": "in_progress"
        }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    assert_eq!(body["ok"], true);
}

// --- 6.6 Scenario: list multiple programs ---

#[tokio::test]
async fn test_list_multiple_programs() {
    let tmp = TempDir::new().unwrap();
    setup_test_program(tmp.path(), "alpha-prog");
    setup_test_program(tmp.path(), "beta-prog");
    setup_test_program(tmp.path(), "gamma-prog");

    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_json_content_type(&response);
    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    let programs = body.as_array().unwrap();
    assert_eq!(programs.len(), 3);
    // Sorted alphabetically
    assert_eq!(programs[0]["slug"], "alpha-prog");
    assert_eq!(programs[1]["slug"], "beta-prog");
    assert_eq!(programs[2]["slug"], "gamma-prog");
    // Each has a title
    for p in programs {
        assert_eq!(p["title"], "Test Program");
    }
}

// --- 6.7 Scenario: scope malformed yaml / empty file ---

#[tokio::test]
async fn test_scope_malformed_yaml() {
    let tmp = TempDir::new().unwrap();
    let program_dir = tmp.path().join("bad-scope");
    std::fs::create_dir_all(&program_dir).unwrap();
    // scope.md with no --- delimiters
    std::fs::write(
        program_dir.join("scope.md"),
        "this is not valid frontmatter at all",
    )
    .unwrap();

    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/bad-scope/scope")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // parse_scope fails → handler returns 500
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_scope_empty_file() {
    let tmp = TempDir::new().unwrap();
    let program_dir = tmp.path().join("empty-scope");
    std::fs::create_dir_all(&program_dir).unwrap();
    // Empty scope.md
    std::fs::write(program_dir.join("scope.md"), "").unwrap();

    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/empty-scope/scope")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Empty file → parse_scope fails ("missing YAML frontmatter") → 500
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// --- 6.8 Scenario: POST progress without content-type header ---

#[tokio::test]
async fn test_progress_update_no_content_type() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    // POST without content-type header — Axum Json extractor rejects → 415
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/programs/test-prog/progress")
                .body(Body::from(
                    serde_json::json!({
                        "lessonPath": "subject-one/lesson-one",
                        "section": "Section A",
                        "status": "in_progress"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Axum returns 415 Unsupported Media Type when content-type is missing for Json
    assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
}

// --- 6.9 Scenario: qa-history filter with nonexistent lesson ---

#[tokio::test]
async fn test_qa_history_filter_no_match() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");

    // Insert a QA record for lesson-one
    let db_path = tmp.path().join("test-prog").join("learnkit.db");
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    learnkit::db::qa::insert_qa(
        &conn,
        "q-xyz",
        "subject-one/lesson-one",
        "some text",
        "question",
        "answer",
    )
    .unwrap();
    drop(conn);

    // Filter by nonexistent lesson → empty array
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/qa-history?lesson=nonexistent/lesson")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    assert!(body.as_array().unwrap().is_empty());
}

// --- 6.10 State: progress status rollback (completed → in_progress) ---

#[tokio::test]
async fn test_progress_status_rollback() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");

    // Step 1: Mark section as completed
    let app = create_test_app(tmp.path().to_path_buf());
    let response = post_json(
        app,
        "/api/programs/test-prog/progress",
        serde_json::json!({
            "lessonPath": "subject-one/lesson-one",
            "section": "Section A",
            "status": "completed"
        }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    // Verify lesson is now "completed"
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/lessons")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = parse_json(response).await;
    let lessons = body.as_array().unwrap();
    let lesson_one = lessons.iter().find(|l| l["id"] == "subject-one/lesson-one").unwrap();
    assert_eq!(lesson_one["status"], "completed");

    // Step 2: POST in_progress for a different section — should rollback lesson status
    let app = create_test_app(tmp.path().to_path_buf());
    let response = post_json(
        app,
        "/api/programs/test-prog/progress",
        serde_json::json!({
            "lessonPath": "subject-one/lesson-one",
            "section": "Section B",
            "status": "in_progress"
        }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    // Verify lesson status is now "in_progress" (rolled back)
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/lessons")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = parse_json(response).await;
    let lessons = body.as_array().unwrap();
    let lesson_one = lessons.iter().find(|l| l["id"] == "subject-one/lesson-one").unwrap();
    assert_eq!(lesson_one["status"], "in_progress");
}

// --- 6.11 State: all sections read → verify lesson status ---

#[tokio::test]
async fn test_progress_all_sections_read() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");

    // Mark Section A as in_progress
    let app = create_test_app(tmp.path().to_path_buf());
    let response = post_json(
        app,
        "/api/programs/test-prog/progress",
        serde_json::json!({
            "lessonPath": "subject-one/lesson-one",
            "section": "Section A",
            "status": "in_progress"
        }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    // Mark Section B as completed — both sections now read
    let app = create_test_app(tmp.path().to_path_buf());
    let response = post_json(
        app,
        "/api/programs/test-prog/progress",
        serde_json::json!({
            "lessonPath": "subject-one/lesson-one",
            "section": "Section B",
            "status": "completed"
        }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    // Verify progress: lesson-one should have 2 sections read
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/progress")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = parse_json(response).await;
    // lesson-one has 2 sections (A, B), lesson-two has 1 section (C), total=3
    assert_eq!(body["sections"]["read"], 2);
    assert_eq!(body["sections"]["total"], 3);

    // lesson status should be "completed" (last POST set it to completed)
    assert_eq!(body["lessons"]["completed"], 1);
}

// --- 6.12 State: progress across subjects (independent) ---

#[tokio::test]
async fn test_progress_across_subjects() {
    let tmp = TempDir::new().unwrap();

    // Create a program with two subjects
    let program_dir = tmp.path().join("multi-subj");
    std::fs::create_dir_all(&program_dir).unwrap();
    let scope_content = r#"---
program: multi-subj
title: Multi Subject Program
created: 2026-01-01
subjects:
  - slug: subj-a
    title: Subject A
    lessons:
      - slug: lesson-a1
        title: Lesson A1
        sections:
          - SA1
          - SA2
  - slug: subj-b
    title: Subject B
    lessons:
      - slug: lesson-b1
        title: Lesson B1
        sections:
          - SB1
---

# Multi Subject
"#;
    std::fs::write(program_dir.join("scope.md"), scope_content).unwrap();

    // Set up DB
    let db_path = program_dir.join("learnkit.db");
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    conn.execute_batch("PRAGMA journal_mode=WAL;").unwrap();
    learnkit::db::schema::ensure_tables(&conn).unwrap();

    learnkit::db::lessons::insert_lesson(&conn, "subj-a/lesson-a1", "subj-a", "lesson-a1", "Lesson A1").unwrap();
    learnkit::db::sections::insert_sections(&conn, "subj-a/lesson-a1", &["SA1".to_string(), "SA2".to_string()]).unwrap();

    learnkit::db::lessons::insert_lesson(&conn, "subj-b/lesson-b1", "subj-b", "lesson-b1", "Lesson B1").unwrap();
    learnkit::db::sections::insert_sections(&conn, "subj-b/lesson-b1", &["SB1".to_string()]).unwrap();
    drop(conn);

    // Mark section in subject A
    let app = create_test_app(tmp.path().to_path_buf());
    let response = post_json(
        app,
        "/api/programs/multi-subj/progress",
        serde_json::json!({
            "lessonPath": "subj-a/lesson-a1",
            "section": "SA1",
            "status": "in_progress"
        }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    // Mark section in subject B
    let app = create_test_app(tmp.path().to_path_buf());
    let response = post_json(
        app,
        "/api/programs/multi-subj/progress",
        serde_json::json!({
            "lessonPath": "subj-b/lesson-b1",
            "section": "SB1",
            "status": "completed"
        }),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    // Verify lessons have independent statuses
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/multi-subj/lessons")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = parse_json(response).await;
    let lessons = body.as_array().unwrap();
    assert_eq!(lessons.len(), 2);

    let lesson_a = lessons.iter().find(|l| l["id"] == "subj-a/lesson-a1").unwrap();
    let lesson_b = lessons.iter().find(|l| l["id"] == "subj-b/lesson-b1").unwrap();
    assert_eq!(lesson_a["status"], "in_progress");
    assert_eq!(lesson_b["status"], "completed");

    // Verify overall progress
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/multi-subj/progress")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = parse_json(response).await;
    assert_eq!(body["sections"]["read"], 2); // SA1 + SB1
    assert_eq!(body["sections"]["total"], 3); // SA1, SA2, SB1
    assert_eq!(body["lessons"]["in_progress"], 1);
    assert_eq!(body["lessons"]["completed"], 1);
}

// --- 6.13 Quality: Response Content-Type verification ---

#[tokio::test]
async fn test_health_content_type() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_json_content_type(&response);
}

#[tokio::test]
async fn test_programs_content_type() {
    let tmp = TempDir::new().unwrap();
    setup_test_program(tmp.path(), "ct-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_json_content_type(&response);
}

#[tokio::test]
async fn test_scope_content_type() {
    let tmp = TempDir::new().unwrap();
    setup_test_program(tmp.path(), "ct-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/ct-prog/scope")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_json_content_type(&response);
}

#[tokio::test]
async fn test_progress_content_type() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "ct-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/ct-prog/progress")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_json_content_type(&response);
}

#[tokio::test]
async fn test_lessons_content_type() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "ct-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/ct-prog/lessons")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_json_content_type(&response);
}

#[tokio::test]
async fn test_qa_history_content_type() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "ct-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/ct-prog/qa-history")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_json_content_type(&response);
}

// --- 6.14 Quality: error response body validation ---

#[tokio::test]
async fn test_update_progress_invalid_status_error_body() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = post_json(
        app,
        "/api/programs/test-prog/progress",
        serde_json::json!({
            "lessonPath": "subject-one/lesson-one",
            "section": "Section A",
            "status": "invalid_status"
        }),
    )
    .await;

    // Should be 400 Bad Request with JSON error body
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_json_content_type(&response);
    let body = parse_json(response).await;
    // AppError now returns {"error": "message"}
    let error_msg = body["error"].as_str().unwrap();
    assert!(
        error_msg.contains("invalid status"),
        "Error should mention invalid status, got: {}",
        error_msg
    );
}

#[tokio::test]
async fn test_update_progress_missing_fields_error_body() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = post_json(
        app,
        "/api/programs/test-prog/progress",
        serde_json::json!({
            "lessonPath": "subject-one/lesson-one"
        }),
    )
    .await;

    // Missing fields → Axum JSON deserialization fails → 422
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    // Verify response body contains error info
    let body_bytes = read_body(response).await;
    let body_str = String::from_utf8_lossy(&body_bytes);
    // Axum's rejection should mention the missing field
    assert!(
        body_str.len() > 0,
        "Error response body should not be empty"
    );
}

#[tokio::test]
async fn test_scope_not_found_error_body() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/no-such-program/scope")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    // AppError returns JSON body with {"error": "..."} for 404 responses
    assert_json_content_type(&response);
    let body = parse_json(response).await;
    let error_msg = body["error"].as_str().unwrap();
    assert!(
        error_msg.contains("not found"),
        "404 error should contain 'not found', got: {}",
        error_msg
    );
}

// --- 6.15 Boundary: slug validation on multiple endpoints ---

#[tokio::test]
async fn test_slug_validation_on_lessons_endpoint() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/..%2F..%2Fetc/lessons")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_slug_validation_on_progress_endpoint() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/..%2F..%2Fetc/progress")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_slug_validation_on_qa_history_endpoint() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/..%2F..%2Fetc/qa-history")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_slug_validation_on_prepare_status_endpoint() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/..%2F..%2Fetc/prepare-status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// --- 6.16 Boundary: POST progress with invalid slug ---

#[tokio::test]
async fn test_update_progress_with_traversal_slug() {
    let tmp = TempDir::new().unwrap();
    let app = create_test_app(tmp.path().to_path_buf());

    let response = post_json(
        app,
        "/api/programs/..%2F..%2Fetc/progress",
        serde_json::json!({
            "lessonPath": "subject-one/lesson-one",
            "section": "Section A",
            "status": "in_progress"
        }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// --- 6.17 Scenario: scope with partially valid yaml ---

#[tokio::test]
async fn test_scope_partial_yaml() {
    let tmp = TempDir::new().unwrap();
    let program_dir = tmp.path().join("partial-yaml");
    std::fs::create_dir_all(&program_dir).unwrap();
    // Valid frontmatter delimiters but invalid YAML content
    std::fs::write(
        program_dir.join("scope.md"),
        "---\nthis: [invalid yaml\n---\ncontent\n",
    )
    .unwrap();

    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/partial-yaml/scope")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // YAML parse error → 500
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// --- 6.18 Scenario: multiple QA records filtering ---

#[tokio::test]
async fn test_qa_history_multiple_lessons_filter() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");

    let db_path = tmp.path().join("test-prog").join("learnkit.db");
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    // Insert QA for multiple lessons
    for i in 0..5 {
        learnkit::db::qa::insert_qa(
            &conn,
            &format!("q-{}", i),
            "subject-one/lesson-one",
            "text",
            &format!("question {}", i),
            &format!("answer {}", i),
        )
        .unwrap();
    }
    for i in 5..8 {
        learnkit::db::qa::insert_qa(
            &conn,
            &format!("q-{}", i),
            "subject-one/lesson-two",
            "text",
            &format!("question {}", i),
            &format!("answer {}", i),
        )
        .unwrap();
    }
    drop(conn);

    // No filter → all 8
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/qa-history")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = parse_json(response).await;
    assert_eq!(body.as_array().unwrap().len(), 8);

    // Filter lesson-one → 5
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/qa-history?lesson=subject-one/lesson-one")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = parse_json(response).await;
    assert_eq!(body.as_array().unwrap().len(), 5);

    // Filter lesson-two → 3
    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/qa-history?lesson=subject-one/lesson-two")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = parse_json(response).await;
    assert_eq!(body.as_array().unwrap().len(), 3);
}

// --- 6.19 Edge: using post_json helper for conciseness + assert_status ---

#[tokio::test]
async fn test_post_json_helper_verify() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");

    // Using post_json and assert_status helpers together
    let app = create_test_app(tmp.path().to_path_buf());
    let response = post_json(
        app,
        "/api/programs/test-prog/progress",
        serde_json::json!({
            "lessonPath": "subject-one/lesson-one",
            "section": "Section A",
            "status": "in_progress"
        }),
    )
    .await;
    assert_status(response, StatusCode::OK).await;
}

// --- 6.20 Boundary: POST wrong content-type (text/plain) ---

#[tokio::test]
async fn test_progress_update_wrong_content_type() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/programs/test-prog/progress")
                .header("content-type", "text/plain")
                .body(Body::from(
                    serde_json::json!({
                        "lessonPath": "subject-one/lesson-one",
                        "section": "Section A",
                        "status": "in_progress"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Wrong content-type → Axum Json extractor rejects → 415
    assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
}

// --- 6.21 Programs list: directory without scope.md is excluded ---

#[tokio::test]
async fn test_list_programs_excludes_non_program_dirs() {
    let tmp = TempDir::new().unwrap();
    setup_test_program(tmp.path(), "real-prog");
    // Create a directory without scope.md
    std::fs::create_dir_all(tmp.path().join("not-a-program")).unwrap();
    // Create a file (not a directory)
    std::fs::write(tmp.path().join("random-file.txt"), "hello").unwrap();

    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    let programs = body.as_array().unwrap();
    assert_eq!(programs.len(), 1);
    assert_eq!(programs[0]["slug"], "real-prog");
}

// --- 6.22 Scope: with valid but minimal frontmatter ---

#[tokio::test]
async fn test_scope_minimal_valid() {
    let tmp = TempDir::new().unwrap();
    let program_dir = tmp.path().join("minimal");
    std::fs::create_dir_all(&program_dir).unwrap();
    let scope_content = r#"---
program: minimal
title: Minimal
created: 2026-01-01
subjects: []
---
"#;
    std::fs::write(program_dir.join("scope.md"), scope_content).unwrap();

    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/minimal/scope")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    assert_eq!(body["program"], "minimal");
    assert_eq!(body["title"], "Minimal");
    assert!(body["subjects"].as_array().unwrap().is_empty());
}

// --- 6.23 prepare-status: verify currentlyPreparing field exists ---

#[tokio::test]
async fn test_prepare_status_currently_preparing_field() {
    let tmp = TempDir::new().unwrap();
    setup_test_program_with_db(tmp.path(), "test-prog");
    let app = create_test_app(tmp.path().to_path_buf());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs/test-prog/prepare-status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    // currentlyPreparing should exist and be an array
    assert!(body["currentlyPreparing"].is_array());
    assert!(body["currentlyPreparing"].as_array().unwrap().is_empty());
}

// --- 6.24 Malformed scope in program list doesn't crash ---

#[tokio::test]
async fn test_list_programs_with_malformed_scope() {
    let tmp = TempDir::new().unwrap();
    setup_test_program(tmp.path(), "good-prog");

    // Create a program with malformed scope.md
    let bad_dir = tmp.path().join("bad-prog");
    std::fs::create_dir_all(&bad_dir).unwrap();
    std::fs::write(bad_dir.join("scope.md"), "not valid frontmatter").unwrap();

    let app = create_test_app(tmp.path().to_path_buf());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = parse_json(response).await;
    let programs = body.as_array().unwrap();
    assert_eq!(programs.len(), 2);
    // bad-prog should have empty title since parse failed
    let bad = programs.iter().find(|p| p["slug"] == "bad-prog").unwrap();
    assert_eq!(bad["title"], "");
    // good-prog should have proper title
    let good = programs.iter().find(|p| p["slug"] == "good-prog").unwrap();
    assert_eq!(good["title"], "Test Program");
}
