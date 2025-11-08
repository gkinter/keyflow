use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Config(String),
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("http client error: {0}")]
    HttpClient(#[from] reqwest::Error),
    #[error("oauth error: {0}")]
    OAuth(#[from] oauth2::RequestTokenError<oauth2::reqwest::Error<reqwest::Error>>),
    #[error("serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("token error: {0}")]
    Token(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("too many requests")]
    TooManyRequests,
    #[error("internal server error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Config(_) | AppError::Internal => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
            }
            AppError::Database(_) | AppError::Migration(_) | AppError::HttpClient(_) | AppError::OAuth(_) | AppError::Json(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
            }
            AppError::Token(_) | AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::TooManyRequests => (StatusCode::TOO_MANY_REQUESTS, "too many requests".to_string()),
        };

        let body = Json(ErrorBody { error: message });

        (status, body).into_response()
    }
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::Token(err.to_string())
    }
}

