use thiserror::Error;

/// Authentication-related errors
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Token expired")]
    TokenExpired,

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("User not found")]
    UserNotFound,
}

/// Validation errors
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid email format")]
    InvalidEmail,

    #[error("Invalid password: {0}")]
    InvalidPassword(String),

    #[error("Invalid username format")]
    InvalidUsername,

    #[error("Required field missing: {0}")]
    MissingField(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// Application-specific errors
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    Redis(String),

    #[error("Kafka error: {0}")]
    Kafka(String),

    #[error("Authentication error: {0}")]
    Auth(#[from] AuthError),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JWT error: {0}")]
    Jwt(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("WebSocket error: {0}")]
    WebSocket(String),
}

impl AppError {
    /// Convert to HTTP status code
    pub fn status_code(&self) -> axum::http::StatusCode {
        match self {
            AppError::Auth(_) => axum::http::StatusCode::UNAUTHORIZED,
            AppError::Unauthorized => axum::http::StatusCode::UNAUTHORIZED,
            AppError::Validation(_) => axum::http::StatusCode::BAD_REQUEST,
            AppError::BadRequest(_) => axum::http::StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => axum::http::StatusCode::NOT_FOUND,
            AppError::Conflict(_) => axum::http::StatusCode::CONFLICT,
            AppError::RateLimitExceeded => axum::http::StatusCode::TOO_MANY_REQUESTS,
            AppError::ServiceUnavailable(_) => axum::http::StatusCode::SERVICE_UNAVAILABLE,
            AppError::Database(_) | AppError::Redis(_) | AppError::Kafka(_) | AppError::Internal(_) => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Get error code for API responses
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::Auth(AuthError::InvalidCredentials) => "AUTH_INVALID_CREDENTIALS",
            AppError::Auth(AuthError::TokenExpired) => "AUTH_TOKEN_EXPIRED",
            AppError::Auth(AuthError::InvalidToken(_)) => "AUTH_INVALID_TOKEN",
            AppError::Auth(AuthError::InsufficientPermissions) => "AUTH_INSUFFICIENT_PERMISSIONS",
            AppError::Auth(AuthError::UserNotFound) => "AUTH_USER_NOT_FOUND",
            AppError::Validation(_) => "VALIDATION_ERROR",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Conflict(_) => "CONFLICT",
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::Redis(_) => "REDIS_ERROR",
            AppError::Kafka(_) => "KAFKA_ERROR",
            AppError::Internal(_) => "INTERNAL_ERROR",
            AppError::Unauthorized => "UNAUTHORIZED",
            AppError::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::ServiceUnavailable(_) => "SERVICE_UNAVAILABLE",
            AppError::Jwt(_) => "JWT_ERROR",
            AppError::WebSocket(_) => "WEBSOCKET_ERROR",
            AppError::Configuration(_) => "CONFIGURATION_ERROR",
            AppError::Serialization(_) => "SERIALIZATION_ERROR",
            AppError::Io(_) => "IO_ERROR",
        }
    }

    /// Check if error is client error (4xx)
    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            AppError::Auth(_) |
            AppError::Validation(_) |
            AppError::NotFound(_) |
            AppError::Conflict(_) |
            AppError::Unauthorized |
            AppError::RateLimitExceeded |
            AppError::BadRequest(_)
        )
    }
}

/// Error response structure
#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

impl From<&AppError> for ErrorResponse {
    fn from(err: &AppError) -> Self {
        ErrorResponse {
            code: err.error_code().to_string(),
            message: err.to_string(),
            details: if err.is_client_error() {
                None
            } else {
                Some("Internal server error".to_string())
            },
            trace_id: None,
        }
    }
}

// Implement From trait for redis errors
impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        AppError::Redis(err.to_string())
    }
}

// Implement From trait for Kafka errors
#[cfg(feature = "kafka")]
impl From<rdkafka::error::KafkaError> for AppError {
    fn from(err: rdkafka::error::KafkaError) -> Self {
        AppError::Kafka(err.to_string())
    }
}

// Implement From trait for JWT errors
impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::Auth(AuthError::TokenExpired)
            }
            _ => AppError::Jwt(err.to_string()),
        }
    }
}

// Implement From trait for UUID parse errors
impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> Self {
        AppError::BadRequest(err.to_string())
    }
}

// Implement From trait for String (e.g. config validation)
impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Configuration(s)
    }
}

// Implement From trait for config errors
impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::Configuration(err.to_string())
    }
}

/// Convert AppError to axum response
impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let error_response: ErrorResponse = (&self).into();

        (status, axum::Json(error_response)).into_response()
    }
}

/// Type alias for Result
pub type AppResult<T> = Result<T, AppError>;
