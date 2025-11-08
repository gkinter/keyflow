use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthStatus {
    status: &'static str,
}

pub async fn health() -> Json<HealthStatus> {
    Json(HealthStatus { status: "ok" })
}

