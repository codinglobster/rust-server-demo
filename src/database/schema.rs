//! Database schema module
//!
//! This module contains type-safe database queries using SQLx.

use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Re-export models that map directly to database tables
pub use crate::models::user::User;
pub use crate::models::session::Session;
pub use crate::models::message::Message;

/// Raw query result for user lookup by username
#[derive(Debug, FromRow)]
pub struct UserLookupResult {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

/// Raw query result for active sessions
#[derive(Debug, FromRow)]
pub struct ActiveSessionResult {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub last_accessed_at: Option<DateTime<Utc>>,
}

/// Raw query result for message queries
#[derive(Debug, FromRow)]
pub struct MessageQueryResult {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub message_type: String,
    pub room_id: Option<Uuid>,
    pub recipient_id: Option<Uuid>,
    pub is_edited: bool,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Count result for pagination
#[derive(Debug, FromRow)]
pub struct CountResult {
    pub count: i64,
}
