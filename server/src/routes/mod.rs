use crate::state::SharedState;
use axum::{
    routing::{get, post},
    Router,
};

mod auth;
mod health;

pub use auth::{CurrentUser, SESSION_COOKIE};

pub fn build_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/auth/github/login", get(auth::github_login))
        .route("/auth/github/callback", get(auth::github_callback))
        .route("/auth/logout", post(auth::logout))
        .route("/session", get(auth::current_session))
        .route("/me", get(auth::me))
        .with_state(state)
}

