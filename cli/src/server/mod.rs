//! HTTP API server module for LearnKit.
//!
//! Provides a REST API for the web UI to interact with programs, lessons, and progress.

use axum::{routing::{get, post}, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

pub mod ask;
pub mod error;
pub mod state;
pub mod health;
pub mod programs;
pub mod lessons;
pub mod progress;

pub fn create_router(state: Arc<state::AppState>) -> Router {
    // NOTE: Axum 0.7 uses `:param` syntax for path parameters.
    // Migrate to `{param}` syntax when upgrading to Axum 0.8+.
    Router::new()
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
        // Planned endpoints:
        // POST /api/programs/:slug/qa       — submit a new Q&A pair
        // POST /api/programs/:slug/prepare  — trigger lesson preparation
        // GET  /api/programs/:slug/resources — list learning resources
        //
        // Static file serving for lesson HTML files
        .nest_service("/lessons", ServeDir::new(state.learnkit_root.clone()))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
