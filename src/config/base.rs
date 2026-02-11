//! Base server configuration

use serde::{Deserialize, Serialize};

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server host address
    pub host: String,
    /// Server port
    pub port: u16,
    /// Number of worker threads (0 = auto)
    pub workers: usize,
    /// Request body size limit in MB
    pub max_body_size: usize,
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// Enable CORS
    pub cors_enabled: bool,
    /// Enable compression
    pub compression_enabled: bool,
    /// Log level
    pub log_level: String,
    /// Environment (dev, staging, prod)
    pub environment: String,
    /// Jaeger endpoint for distributed tracing (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jaeger_endpoint: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            workers: 0, // Auto-detect
            max_body_size: 10, // 10 MB
            request_timeout: 30,
            cors_enabled: true,
            compression_enabled: true,
            log_level: "info".to_string(),
            environment: "development".to_string(),
            jaeger_endpoint: None,
        }
    }
}

impl ServerConfig {
    /// Get the full bind address
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Check if running in development mode
    pub fn is_dev(&self) -> bool {
        self.environment == "development" || self.environment == "dev"
    }

    /// Check if running in production mode
    pub fn is_prod(&self) -> bool {
        self.environment == "production" || self.environment == "prod"
    }
}
