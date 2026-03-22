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

    // "/api/programs//scope" — empty slug triggers validate_slug rejection
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/programs//scope")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
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
