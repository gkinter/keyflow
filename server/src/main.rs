mod config;
mod db;
mod error;
mod models;
mod routes;
mod session;
mod state;
mod security;

use crate::{
    config::AppConfig,
    db::init_pool,
    error::AppError,
    routes::build_router,
    security::rate_limit::RateLimiter,
    session::SessionSigner,
    state::AppState,
};
use axum::Router;
use http::{
    header::{
        HeaderName, HeaderValue, CONTENT_SECURITY_POLICY, PERMISSIONS_POLICY, REFERRER_POLICY,
        STRICT_TRANSPORT_SECURITY, X_CONTENT_TYPE_OPTIONS, X_FRAME_OPTIONS,
    },
    Method,
};
use std::{net::SocketAddr, time::Duration};
use tower::ServiceBuilder;
use tower_http::{
    cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    init_tracing();

    let config = AppConfig::from_env()?;
    let pool = init_pool(&config.database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let session_signer = SessionSigner::new(&config.session_signing_keys)?;
    let oauth_client = config.oauth_client()?;
    let http_client = reqwest::Client::builder()
        .user_agent("keyflow-server")
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| AppError::Config(format!("http client init error: {}", e)))?;

    let auth_rate_limiter = RateLimiter::new(
        config.auth_rate_limit_per_minute,
        config.auth_rate_limit_burst,
    );

    let shared_state =
        AppState::new(pool, session_signer, oauth_client, http_client, config.clone(), auth_rate_limiter).shared();

    let cors = build_cors(&config)?;

    let security_headers = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::if_not_present(
            STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=63072000; includeSubDomains; preload"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            REFERRER_POLICY,
            HeaderValue::from_static("no-referrer"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            PERMISSIONS_POLICY,
            HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("cross-origin-opener-policy"),
            HeaderValue::from_static("same-origin"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("cross-origin-resource-policy"),
            HeaderValue::from_static("same-origin"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            CONTENT_SECURITY_POLICY,
            HeaderValue::from_static(
                "default-src 'self'; img-src 'self' https://avatars.githubusercontent.com data:; \
                 connect-src 'self' https://api.github.com; script-src 'self'; style-src 'self' 'unsafe-inline'",
            ),
        ));

    let app = build_router(shared_state).layer(
        ServiceBuilder::new()
            .layer(cors)
            .layer(security_headers)
            .layer(TraceLayer::new_for_http()),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .map_err(|e| AppError::Config(format!("server error: {}", e)))?;

    Ok(())
}

fn init_tracing() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        tracing_subscriber::EnvFilter::new("info,tower_http=info,sqlx=warn")
    });

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn build_cors(config: &AppConfig) -> Result<CorsLayer, AppError> {
    let origin = HeaderValue::from_str(&config.frontend_origin)
        .map_err(|e| AppError::Config(format!("invalid FRONTEND_ORIGIN: {}", e)))?;

    Ok(CorsLayer::new()
        .allow_methods(AllowMethods::list([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ]))
        .allow_origin(AllowOrigin::exact(origin))
        .allow_headers(AllowHeaders::any())
        .allow_credentials(true))
}
