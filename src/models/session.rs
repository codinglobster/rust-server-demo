//! Session model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use utoipa::ToSchema;

/// Session model from database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub refresh_token_hash: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub is_active: bool,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub last_accessed_at: Option<DateTime<Utc>>,
}

/// Session create request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SessionCreate {
    pub user_id: Uuid,
    pub token_hash: String,
    pub refresh_token_hash: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub expires_at: DateTime<Utc>,
}

/// Session DTO for API responses
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SessionDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub ip_address: Option<String>,
    pub is_active: bool,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub last_accessed_at: Option<DateTime<Utc>>,
}

impl From<Session> for SessionDto {
    fn from(session: Session) -> Self {
        Self {
            id: session.id,
            user_id: session.user_id,
            ip_address: session.ip_address,
            is_active: session.is_active,
            expires_at: session.expires_at,
            created_at: session.created_at,
            last_accessed_at: session.last_accessed_at,
        }
    }
}

/// Session refresh request
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

/// Token response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

/// Login response with tokens
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub user: crate::models::user::UserDto,
}

/// Logout request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: Option<String>,
}
