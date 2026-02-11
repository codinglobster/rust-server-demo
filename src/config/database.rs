//! Database configuration

use serde::{Deserialize, Serialize};

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// PostgreSQL connection URL
    pub url: String,
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    /// Minimum number of connections in the pool
    pub min_connections: u32,
    /// Connection timeout in seconds
    pub connect_timeout: u64,
    /// Idle timeout in seconds
    pub idle_timeout: u64,
    /// Maximum lifetime of a connection in seconds
    pub max_lifetime: u64,
    /// Enable SQL statement logging
    pub enable_logging: bool,
    /// Whether to automatically run migrations on startup
    pub auto_migrate: bool,
    /// Slow query threshold in milliseconds
    pub slow_query_threshold: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgresql://postgres:postgres@localhost:5432/rust_server".to_string(),
            max_connections: 10,
            min_connections: 1,
            connect_timeout: 10,
            idle_timeout: 600,
            max_lifetime: 1800,
            enable_logging: true,
            auto_migrate: true,
            slow_query_threshold: 100,
        }
    }
}

impl DatabaseConfig {
    /// Get the connection options for SQLx
    pub fn as_sqlx_options(&self) -> sqlx::postgres::PgPoolOptions {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(self.max_connections)
            .acquire_timeout(std::time::Duration::from_secs(self.connect_timeout))
            .idle_timeout(std::time::Duration::from_secs(self.idle_timeout))
            .max_lifetime(std::time::Duration::from_secs(self.max_lifetime))
            // SQLx 0.8 doesn't have sqlx_logging method
            // Logging is configured via SQLX_LOGGING env var instead
    }
}
