//! JWT configuration

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// JWT configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// Secret key for signing tokens
    pub secret: String,
    /// Access token expiration in seconds
    pub access_expiration: u64,
    /// Refresh token expiration in seconds
    pub refresh_expiration: u64,
    /// Token issuer
    pub issuer: String,
    /// Token audience
    pub audience: String,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: "change-this-secret-in-production".to_string(),
            access_expiration: 3600, // 1 hour
            refresh_expiration: 604800, // 7 days
            issuer: "rust-server".to_string(),
            audience: "rust-server-users".to_string(),
        }
    }
}

impl JwtConfig {
    /// Get the access token expiration duration
    pub fn access_token_duration(&self) -> Duration {
        Duration::from_secs(self.access_expiration)
    }

    /// Get the refresh token expiration duration
    pub fn refresh_token_duration(&self) -> Duration {
        Duration::from_secs(self.refresh_expiration)
    }

    /// Validate the secret key
    pub fn validate_secret(&self) -> Result<(), String> {
        if self.secret.len() < 32 {
            return Err("JWT secret must be at least 32 characters long".to_string());
        }
        if self.secret == "change-this-secret-in-production" {
            return Err("JWT secret must be changed from default".to_string());
        }
        Ok(())
    }
}
