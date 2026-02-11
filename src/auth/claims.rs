//! JWT claims and token user

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// JWT Claims structure
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Username
    pub username: String,
    /// User's email
    pub email: String,
    /// Expiration time (Unix timestamp)
    pub exp: usize,
    /// Issued at time (Unix timestamp)
    pub iat: usize,
    /// Token issuer
    pub iss: String,
    /// Token audience
    pub aud: String,
    /// User roles
    #[serde(default)]
    pub roles: Vec<String>,
    /// JWT ID (unique token identifier)
    pub jti: String,
}

impl Claims {
    /// Create new claims
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_id: Uuid,
        username: String,
        email: String,
        roles: Vec<String>,
        exp: usize,
        iat: usize,
        iss: String,
        aud: String,
    ) -> Self {
        Self {
            sub: user_id.to_string(),
            username,
            email,
            exp,
            iat,
            iss,
            aud,
            roles,
            jti: Uuid::new_v4().to_string(),
        }
    }

    /// Get user ID from claims
    pub fn user_id(&self) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(&self.sub)
    }

    /// Check if user has a specific role
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    /// Check if user is admin
    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }

    /// Check if user is moderator or higher
    pub fn is_moderator(&self) -> bool {
        self.is_admin() || self.has_role("moderator")
    }

    /// Get token identifier
    pub fn token_id(&self) -> Uuid {
        Uuid::parse_str(&self.jti).unwrap_or_else(|_| Uuid::nil())
    }
}

/// Simplified user information extracted from token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
}

impl TokenUser {
    /// Create new token user
    pub fn new(id: Uuid, username: String, email: String, roles: Vec<String>) -> Self {
        Self {
            id,
            username,
            email,
            roles,
        }
    }

    /// Check if user has a specific role
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    /// Check if user is admin
    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }

    /// Check if user is moderator or higher
    pub fn is_moderator(&self) -> bool {
        self.is_admin() || self.has_role("moderator")
    }

    /// Create from claims
    pub fn from_claims(claims: &Claims) -> Result<Self, uuid::Error> {
        Ok(Self {
            id: Uuid::parse_str(&claims.sub)?,
            username: claims.username.clone(),
            email: claims.email.clone(),
            roles: claims.roles.clone(),
        })
    }
}
