//! HTTP API server module for LearnKit.
//!
//! Provides a REST API for the web UI to interact with programs, lessons, and progress.

use axum::{routing::{get, post}, Router};
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

pub mod ask;
pub mod error;
pub mod state;
pub mod health;
pub mod programs;
pub mod lessons;
pub mod progress;

/// Create the router with optional frontend serving.
///
/// When `frontend_dist` is `Some`, static files from that directory are served
/// as a fallback (after API routes), with SPA support: unknown paths fall back
/// to `index.html`.
pub fn create_router_with_frontend(
    state: Arc<state::AppState>,
    frontend_dist: Option<PathBuf>,
) -> Router {
    // NOTE: Axum 0.7 uses `:param` syntax for path parameters.
    // Migrate to `{param}` syntax when upgrading to Axum 0.8+.
    let api_router = Router::new()
        .route("/api/health", get(health::health))
        .route("/api/programs", get(programs::list))
        .route("/api/programs/:slug/scope", get(programs::scope_handler))
        .route("/api/programs/:slug/lessons", get(lessons::list))
        .route("/api/programs/:slug/progress", get(progress::get_progress))
        .route("/api/programs/:slug/progress", post(progress::update_progress))
        .route("/api/programs/:slug/qa-history", get(programs::qa_history))
        .route("/api/programs/:slug/prepare-status", get(progress::prepare_status))
        .route("/api/programs/:slug/ask", post(ask::submit_ask))
        .route("/api/programs/:slug/answer/:request_id", get(ask::poll_answer))
        // Static file serving for lesson HTML files
        .nest_service("/lessons", ServeDir::new(state.learnkit_root.clone()))
        .layer(CorsLayer::permissive())
        .with_state(state);

    match frontend_dist {
        Some(dist) => {
            let index_file = dist.join("index.html");
            api_router.fallback_service(
                ServeDir::new(dist).fallback(ServeFile::new(index_file)),
            )
        }
        None => api_router,
    }
}

/// Create the router without frontend serving (backward-compatible wrapper).
pub fn create_router(state: Arc<state::AppState>) -> Router {
    create_router_with_frontend(state, None)
}
