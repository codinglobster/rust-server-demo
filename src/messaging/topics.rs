//! Topic definitions

/// Kafka topics
pub struct KafkaTopics;

impl KafkaTopics {
    /// User events (login, logout, registration)
    pub const USER_EVENTS: &'static str = "user-events";

    /// Message events
    pub const MESSAGES: &'static str = "messages";

    /// System notifications
    pub const NOTIFICATIONS: &'static str = "notifications";

    /// Session events
    pub const SESSION_EVENTS: &'static str = "session-events";

    /// Audit log events
    pub const AUDIT_LOG: &'static str = "audit-log";
}

/// Event types for user events topic
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "event_type")]
pub enum UserEvent {
    #[serde(rename = "user_registered")]
    Registered { user_id: String, username: String, email: String },
    #[serde(rename = "user_login")]
    Login { user_id: String, ip: Option<String> },
    #[serde(rename = "user_logout")]
    Logout { user_id: String, session_id: String },
    #[serde(rename = "user_verified")]
    Verified { user_id: String },
}

/// Event types for message events topic
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "event_type")]
pub enum MessageEvent {
    #[serde(rename = "message_created")]
    Created { message_id: String, sender_id: String, content: String },
    #[serde(rename = "message_edited")]
    Edited { message_id: String, sender_id: String, content: String },
    #[serde(rename = "message_deleted")]
    Deleted { message_id: String, sender_id: String },
}

/// Event types for session events topic
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "event_type")]
pub enum SessionEvent {
    #[serde(rename = "session_created")]
    Created { session_id: String, user_id: String, ip: Option<String> },
    #[serde(rename = "session_refreshed")]
    Refreshed { session_id: String, user_id: String },
    #[serde(rename = "session_expired")]
    Expired { session_id: String, user_id: String },
    #[serde(rename = "session_revoked")]
    Revoked { session_id: String, user_id: String },
}

impl KafkaTopics {
    /// Get all available topics
    pub fn all_topics() -> Vec<&'static str> {
        vec![
            Self::USER_EVENTS,
            Self::MESSAGES,
            Self::NOTIFICATIONS,
            Self::SESSION_EVENTS,
            Self::AUDIT_LOG,
        ]
    }
}
