//! Health check handler

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Health check response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<DatabaseHealth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis: Option<RedisHealth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka: Option<KafkaHealth>,
}

/// Database health status
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatabaseHealth {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_connections: Option<u32>,
}

/// Redis health status
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RedisHealth {
    pub status: String,
}

/// Kafka health status
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct KafkaHealth {
    pub status: String,
}

/// Application state for health check
pub type HealthState = crate::state::AppState;

/// Basic health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check successful", body = HealthResponse)
    ),
    tag = "Health"
)]
pub async fn health_check() -> impl IntoResponse {
    let response = HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: None,
        redis: None,
        kafka: None,
    };

    (StatusCode::OK, Json(response))
}

/// Detailed health check with all services
pub async fn detailed_health_check(State(state): State<HealthState>) -> impl IntoResponse {
    // Check database
    let status = match state.db.health_check().await {
        Ok(_) => "ok".to_string(),
        Err(_) => "error".to_string(),
    };
    let stats = state.db.pool_stats();
    let db_health = Some(DatabaseHealth {
        status,
        pool_size: Some(stats.size),
        idle_connections: Some(stats.idle),
    });

    // Check Redis
    let status = match state.redis.health_check().await {
        Ok(_) => "ok".to_string(),
        Err(_) => "error".to_string(),
    };
    let redis_health = Some(RedisHealth { status });

    // Check Kafka
    #[cfg(feature = "kafka")]
    let kafka_health = if let Some(kafka) = &state.kafka {
        let status = match kafka.health_check().await {
            Ok(_) => "ok".to_string(),
            Err(_) => "error".to_string(),
        };
        Some(KafkaHealth { status })
    } else {
        None
    };
    #[cfg(not(feature = "kafka"))]
    let kafka_health = None;

    // Determine overall status
    let overall_status = if db_health.as_ref().map(|d| d.status == "ok").unwrap_or(true)
        && redis_health.as_ref().map(|r| r.status == "ok").unwrap_or(true)
        && kafka_health.as_ref().map(|k: &KafkaHealth| k.status == "ok").unwrap_or(true)
    {
        "ok"
    } else {
        "degraded"
    };

    let response = HealthResponse {
        status: overall_status.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_health,
        redis: redis_health,
        kafka: kafka_health,
    };

    let status_code = if overall_status == "ok" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status_code, Json(response))
}

/// Liveness probe - returns OK if the process is running
pub async fn liveness() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

/// Readiness probe - returns OK if the app is ready to serve traffic
pub async fn readiness(State(state): State<HealthState>) -> impl IntoResponse {
    if state.db.health_check().await.is_err() {
        return (StatusCode::SERVICE_UNAVAILABLE, "Database not ready");
    }
    (StatusCode::OK, "Ready")
}

/// Version endpoint
pub async fn version() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "name": env!("CARGO_PKG_NAME"),
            "rust_version": env!("CARGO_PKG_RUST_VERSION"),
        })),
    )
}
