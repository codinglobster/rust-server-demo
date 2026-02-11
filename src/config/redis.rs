//! Redis configuration

use serde::{Deserialize, Serialize};

/// Redis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    /// Redis connection URL
    pub url: String,
    /// Maximum number of reconnections
    pub max_reconnects: u32,
    /// Reconnect delay in milliseconds
    pub reconnect_delay: u64,
    /// Connection timeout in seconds
    pub connect_timeout: u64,
    /// Default key expiration time in seconds
    pub default_expiration: u64,
    /// Connection pool size
    pub pool_size: u32,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://127.0.0.1:6379".to_string(),
            max_reconnects: 10,
            reconnect_delay: 1000,
            connect_timeout: 5,
            default_expiration: 3600, // 1 hour
            pool_size: 10,
        }
    }
}

impl RedisConfig {
    /// Parse the Redis URL to get the host and port
    pub fn parse_url(&self) -> Option<(String, u16)> {
        self.url.strip_prefix("redis://")
            .and_then(|s| s.split_once(':'))
            .map(|(host, port)| {
                let port = port.strip_prefix('/').unwrap_or(port);
                (host.to_string(), port.parse().unwrap_or(6379))
            })
    }
}
