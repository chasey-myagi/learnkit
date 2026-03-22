use axum::{routing::{get, post}, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub mod state;
pub mod health;
pub mod programs;
pub mod lessons;
pub mod progress;

pub fn create_router(state: Arc<state::AppState>) -> Router {
    Router::new()
        .route("/api/health", get(health::health))
        .route("/api/programs", get(programs::list))
        .route("/api/programs/:slug/scope", get(programs::scope_handler))
        .route("/api/programs/:slug/lessons", get(lessons::list))
        .route("/api/programs/:slug/progress", get(progress::get_progress))
        .route("/api/programs/:slug/progress", post(progress::update_progress))
        .route("/api/programs/:slug/qa-history", get(programs::qa_history))
        .route("/api/programs/:slug/prepare-status", get(progress::prepare_status))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
