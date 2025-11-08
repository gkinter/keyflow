use crate::{config::AppConfig, security::rate_limit::RateLimiter, session::SessionSigner};
use oauth2::basic::BasicClient;
use reqwest::Client;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub session_signer: SessionSigner,
    pub oauth_client: BasicClient,
    pub http_client: Client,
    pub config: AppConfig,
    pub auth_rate_limiter: RateLimiter,
}

pub type SharedState = Arc<AppState>;

impl AppState {
    pub fn new(
        pool: PgPool,
        session_signer: SessionSigner,
        oauth_client: BasicClient,
        http_client: Client,
        config: AppConfig,
        auth_rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            pool,
            session_signer,
            oauth_client,
            http_client,
            config,
            auth_rate_limiter,
        }
    }

    pub fn shared(self) -> SharedState {
        Arc::new(self)
    }
}

