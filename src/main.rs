//! Rust Server Demo - Main Application Entry Point

use axum::{
    extract::State,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, StatusCode,
    },
    response::Json,
    routing::get,
    Router,
};
use rust_server_demo::{
    auth::jwt::JwtService,
    auth::middleware::AuthState,
    cache::RedisClient,
    config::{
        self as app_config,
        base::ServerConfig,
        database::DatabaseConfig,
        jwt::JwtConfig,
    },
    core::telemetry::{init_telemetry, shutdown_telemetry},
    database::Database,
    routes::{create_api_routes, create_ws_routes, AppState},
    services::{AuthService, UserService},
};
use std::{
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use tokio::signal;
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::CorsLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    trace::TraceLayer,
};
use tracing::{error, info, info_span};

#[cfg(feature = "kafka")]
use rust_server_demo::messaging::KafkaProducer;
#[cfg(feature = "kafka")]
use rust_server_demo::config::kafka::KafkaConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize configuration from environment
    let server_config = load_server_config();
    let db_config = load_db_config();
    let redis_config = load_redis_config();
    let jwt_config = load_jwt_config();

    #[cfg(feature = "kafka")]
    let kafka_config = load_kafka_config();

    // Initialize telemetry
    let env_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    init_telemetry(
        "rust-server-demo",
        &server_config.environment,
        &env_filter,
    )?;

    info!("Starting Rust Server Demo v{}", env!("CARGO_PKG_VERSION"));

    // Initialize database
    info!("Connecting to database...");
    let db = Database::new(&db_config).await?;

    // Run migrations if enabled
    if db_config.auto_migrate {
        info!("Running database migrations...");
        match db.run_migrations().await {
            Ok(_) => info!("Migrations completed successfully"),
            Err(e) => {
                error!("Migration failed: {}", e);
                return Err(e.into());
            }
        }
    }

    // Initialize Redis
    info!("Connecting to Redis...");
    let redis = RedisClient::new(&redis_config).await?;

    // Initialize JWT service
    info!("Initializing JWT service...");
    let jwt_service = Arc::new(JwtService::new(&jwt_config)?);

    // Initialize Kafka (optional, only if feature is enabled)
    #[cfg(feature = "kafka")]
    let kafka_producer = if !kafka_config.brokers.is_empty() {
        info!("Connecting to Kafka...");
        match KafkaProducer::new(&kafka_config) {
            Ok(producer) => Some(Arc::new(producer)),
            Err(e) => {
                warn!("Failed to connect to Kafka: {}", e);
                info!("Continuing without Kafka support");
                None
            }
        }
    } else {
        info!("Kafka not configured, skipping");
        None
    };

    // Initialize services
    let user_service = UserService::new(db.pool().clone(), redis.clone());

    #[cfg(feature = "kafka")]
    let auth_service = AuthService::new(
        user_service.clone(),
        jwt_service.as_ref().clone(),
        redis.clone(),
        kafka_producer.clone(),
    );

    #[cfg(not(feature = "kafka"))]
    let auth_service = AuthService::new(
        user_service.clone(),
        jwt_service.as_ref().clone(),
        redis.clone(),
    );

    // Create application state (single state for API and WebSocket)
    let app_state = AppState {
        auth_service: auth_service.clone(),
        user_service: user_service.clone(),
        db: db.clone(),
        redis: redis.clone(),
        jwt_service: Arc::clone(&jwt_service),
        auth_state: AuthState::new(Arc::clone(&jwt_service)),
        #[cfg(feature = "kafka")]
        kafka: kafka_producer.clone(),
    };

    // Build router (API 统一在 /api 下: /api/auth/*, /api/users/*, /api/health/*)
    let api_routes = create_api_routes(app_state.clone());
    let ws_routes = create_ws_routes(app_state.clone());

    let app = Router::new()
        .route("/", get(root_handler))
        .nest("/api", api_routes)
        .merge(ws_routes)
        .route("/metrics", get(metrics_handler))
        .route("/health", get(health_check_handler))
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                })
                .on_response(
                    |response: &axum::http::Response<_>, _latency: Duration, _span: &tracing::Span| {
                        tracing::info!(
                            status = response.status().as_u16(),
                        );
                    },
                ),
        )
        .layer(CatchPanicLayer::new())
        .layer(CompressionLayer::new())
        .layer(SetSensitiveHeadersLayer::new(std::iter::once(AUTHORIZATION)))
        .layer(PropagateHeaderLayer::new(AUTHORIZATION))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::PATCH,
                    Method::OPTIONS,
                ])
                .allow_headers([CONTENT_TYPE, AUTHORIZATION])
                .allow_credentials(false),
        );

    // Start server
    let addr = SocketAddr::from((
        server_config.host.parse::<std::net::IpAddr>().unwrap_or_else(|_| {
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0))
        }),
        server_config.port,
    ));

    info!("Server listening on http://{}", addr);
    info!("API documentation: http://{}/swagger-ui", addr);
    info!("Health check: http://{}/health", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    // Spawn graceful shutdown handler
    let shutdown_signal = async {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
            info!("Received Ctrl+C, shutting down...");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
            info!("Received SIGTERM, shutting down...");
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
    };

    // Start server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
        .expect("Server error");

    // Cleanup
    info!("Shutting down telemetry...");
    shutdown_telemetry();

    info!("Shutdown complete");
    Ok(())
}

/// Load server configuration from environment
fn load_server_config() -> ServerConfig {
    ServerConfig {
        host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        port: std::env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080),
        workers: std::env::var("SERVER_WORKERS")
            .unwrap_or_else(|_| "0".to_string())
            .parse()
            .unwrap_or(0),
        max_body_size: std::env::var("SERVER_MAX_BODY_SIZE")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10),
        request_timeout: std::env::var("SERVER_REQUEST_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .unwrap_or(30),
        cors_enabled: std::env::var("SERVER_CORS_ENABLED")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true),
        compression_enabled: std::env::var("SERVER_COMPRESSION_ENABLED")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true),
        log_level: std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        environment: std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
        jaeger_endpoint: std::env::var("JAEGER_ENDPOINT").ok(),
    }
}

/// Load database configuration from environment
fn load_db_config() -> DatabaseConfig {
    DatabaseConfig {
        url: std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/rust_server".to_string()),
        max_connections: std::env::var("DB_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10),
        min_connections: std::env::var("DB_MIN_CONNECTIONS")
            .unwrap_or_else(|_| "1".to_string())
            .parse()
            .unwrap_or(1),
        connect_timeout: std::env::var("DB_CONNECT_TIMEOUT")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10),
        idle_timeout: std::env::var("DB_IDLE_TIMEOUT")
            .unwrap_or_else(|_| "600".to_string())
            .parse()
            .unwrap_or(600),
        max_lifetime: std::env::var("DB_MAX_LIFETIME")
            .unwrap_or_else(|_| "1800".to_string())
            .parse()
            .unwrap_or(1800),
        enable_logging: std::env::var("DB_ENABLE_LOGGING")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true),
        auto_migrate: std::env::var("DB_AUTO_MIGRATE")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true),
        slow_query_threshold: std::env::var("DB_SLOW_QUERY_THRESHOLD")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .unwrap_or(100),
    }
}

/// Load Redis configuration from environment
fn load_redis_config() -> app_config::redis::RedisConfig {
    app_config::redis::RedisConfig {
        url: std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
        max_reconnects: std::env::var("REDIS_MAX_RECONNECTS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10),
        reconnect_delay: std::env::var("REDIS_RECONNECT_DELAY")
            .unwrap_or_else(|_| "1000".to_string())
            .parse()
            .unwrap_or(1000),
        connect_timeout: std::env::var("REDIS_CONNECT_TIMEOUT")
            .unwrap_or_else(|_| "5".to_string())
            .parse()
            .unwrap_or(5),
        default_expiration: std::env::var("REDIS_DEFAULT_EXPIRATION")
            .unwrap_or_else(|_| "3600".to_string())
            .parse()
            .unwrap_or(3600),
        pool_size: std::env::var("REDIS_POOL_SIZE")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10),
    }
}

/// Load JWT configuration from environment
fn load_jwt_config() -> JwtConfig {
    JwtConfig {
        secret: std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "change-this-secret-in-production-use-at-least-32-characters".to_string()),
        access_expiration: std::env::var("JWT_ACCESS_EXPIRATION")
            .unwrap_or_else(|_| "3600".to_string())
            .parse()
            .unwrap_or(3600),
        refresh_expiration: std::env::var("JWT_REFRESH_EXPIRATION")
            .unwrap_or_else(|_| "604800".to_string())
            .parse()
            .unwrap_or(604800),
        issuer: std::env::var("JWT_ISSUER")
            .unwrap_or_else(|_| "rust-server".to_string()),
        audience: std::env::var("JWT_AUDIENCE")
            .unwrap_or_else(|_| "rust-server-users".to_string()),
    }
}

#[cfg(feature = "kafka")]
/// Load Kafka configuration from environment
fn load_kafka_config() -> KafkaConfig {
    KafkaConfig {
        brokers: std::env::var("KAFKA_BROKERS")
            .unwrap_or_else(|_| "localhost:9092".to_string()),
        group_id: std::env::var("KAFKA_GROUP_ID")
            .unwrap_or_else(|_| "rust-server-group".to_string()),
        session_timeout: std::env::var("KAFKA_SESSION_TIMEOUT")
            .unwrap_or_else(|_| "10000".to_string())
            .parse()
            .unwrap_or(10000),
        auto_offset_reset: std::env::var("KAFKA_AUTO_OFFSET_RESET")
            .unwrap_or_else(|_| "latest".to_string()),
        enable_auto_commit: std::env::var("KAFKA_ENABLE_AUTO_COMMIT")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true),
        message_timeout: std::env::var("KAFKA_MESSAGE_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .unwrap_or(30),
        max_poll_records: std::env::var("KAFKA_MAX_POLL_RECORDS")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .unwrap_or(100),
        topics: std::env::var("KAFKA_TOPICS")
            .unwrap_or_else(|_| String::new())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        producer_prefix: std::env::var("KAFKA_PRODUCER_PREFIX")
            .unwrap_or_else(|_| "rust-server".to_string()),
    }
}

/// Root path handler
async fn root_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
        "description": "Real-time server with PostgreSQL, Redis, WebSocket",
        "links": {
            "health": "/health",
            "api_docs": "/swagger-ui",
            "api_health": "/api/health/health",
            "auth_register": "/api/auth/register",
            "auth_login": "/api/auth/login",
            "ws": "/ws"
        }
    }))
}

/// Health check handler
async fn health_check_handler(State(state): State<AppState>) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut health = serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    });

    // Check database
    let db_status = if state.db.health_check().await.is_ok() {
        "ok"
    } else {
        "error"
    };
    health["database"] = serde_json::json!(db_status);

    // Check Redis
    let redis_status = if state.redis.health_check().await.is_ok() {
        "ok"
    } else {
        "error"
    };
    health["redis"] = serde_json::json!(redis_status);

    Ok(Json(health))
}

/// Prometheus metrics handler
async fn metrics_handler() -> Result<String, StatusCode> {
    Ok(rust_server_demo::core::telemetry::export_metrics())
}

#[allow(dead_code)]
fn warn(s: &str) {
    tracing::warn!("{}", s);
}
