//! Configuration module

pub mod base;
pub mod database;
pub mod jwt;
pub mod redis;

#[cfg(feature = "kafka")]
pub mod kafka;

pub use base::ServerConfig;
pub use database::DatabaseConfig;
pub use jwt::JwtConfig;
pub use redis::RedisConfig;

#[cfg(feature = "kafka")]
pub use kafka::KafkaConfig;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    #[cfg(feature = "kafka")]
    pub kafka: KafkaConfig,
}
