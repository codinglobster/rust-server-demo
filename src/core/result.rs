//! Result type wrapper
//!
//! This module provides type aliases for consistent Result types across the application.

use crate::core::error::AppError;

/// Standard Result type for the application
pub type AppResult<T> = Result<T, AppError>;

/// Result type for database operations
pub type DbResult<T> = Result<T, sqlx::Error>;

/// Result type for Redis operations
pub type RedisResult<T> = Result<T, redis::RedisError>;

/// Result type for Kafka operations
#[cfg(feature = "kafka")]
pub type KafkaResult<T> = Result<T, rdkafka::error::KafkaError>;
