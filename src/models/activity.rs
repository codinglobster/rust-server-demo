//! Activity log model for tracking user events

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Activity event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActivityEventType {
    // User events
    UserRegistered,
    UserLoggedIn,
    UserLoggedOut,
    UserUpdated,
    UserPasswordChanged,

    // Message events
    MessageSent,
    MessageEdited,
    MessageDeleted,

    // Room events
    RoomJoined,
    RoomLeft,
    RoomCreated,

    // System events
    SystemAlert,
    SystemNotification,

    // Error events
    ErrorOccurred,
}

impl ActivityEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActivityEventType::UserRegistered => "user_registered",
            ActivityEventType::UserLoggedIn => "user_logged_in",
            ActivityEventType::UserLoggedOut => "user_logged_out",
            ActivityEventType::UserUpdated => "user_updated",
            ActivityEventType::UserPasswordChanged => "user_password_changed",
            ActivityEventType::MessageSent => "message_sent",
            ActivityEventType::MessageEdited => "message_edited",
            ActivityEventType::MessageDeleted => "message_deleted",
            ActivityEventType::RoomJoined => "room_joined",
            ActivityEventType::RoomLeft => "room_left",
            ActivityEventType::RoomCreated => "room_created",
            ActivityEventType::SystemAlert => "system_alert",
            ActivityEventType::SystemNotification => "system_notification",
            ActivityEventType::ErrorOccurred => "error_occurred",
        }
    }
}

impl std::str::FromStr for ActivityEventType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user_registered" => Ok(ActivityEventType::UserRegistered),
            "user_logged_in" => Ok(ActivityEventType::UserLoggedIn),
            "user_logged_out" => Ok(ActivityEventType::UserLoggedOut),
            "user_updated" => Ok(ActivityEventType::UserUpdated),
            "user_password_changed" => Ok(ActivityEventType::UserPasswordChanged),
            "message_sent" => Ok(ActivityEventType::MessageSent),
            "message_edited" => Ok(ActivityEventType::MessageEdited),
            "message_deleted" => Ok(ActivityEventType::MessageDeleted),
            "room_joined" => Ok(ActivityEventType::RoomJoined),
            "room_left" => Ok(ActivityEventType::RoomLeft),
            "room_created" => Ok(ActivityEventType::RoomCreated),
            "system_alert" => Ok(ActivityEventType::SystemAlert),
            "system_notification" => Ok(ActivityEventType::SystemNotification),
            "error_occurred" => Ok(ActivityEventType::ErrorOccurred),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for ActivityEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Activity log model from database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ActivityLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub event_type: String,
    pub event_type_category: String,
    pub description: String,
    pub metadata: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Activity log DTO for API responses
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, utoipa::ToSchema)]
pub struct ActivityLogDto {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub username: Option<String>,
    pub event_type: String,
    pub event_type_category: String,
    pub description: String,
    pub metadata: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<ActivityLog> for ActivityLogDto {
    fn from(log: ActivityLog) -> Self {
        Self {
            id: log.id,
            user_id: log.user_id,
            username: None, // Will be filled by service
            event_type: log.event_type,
            event_type_category: log.event_type_category,
            description: log.description,
            metadata: log.metadata,
            ip_address: log.ip_address,
            user_agent: log.user_agent,
            created_at: log.created_at,
        }
    }
}

/// Activity log create request
#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct CreateActivityLogRequest {
    pub user_id: Option<Uuid>,
    pub event_type: String,
    pub description: String,
    pub metadata: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

/// Activity log list response with pagination
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ActivityLogsResponse {
    pub activities: Vec<ActivityLogDto>,
    pub total: i64,
    pub page: u64,
    pub per_page: u64,
}

/// Kafka event format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityKafkaEvent {
    pub event_id: Uuid,
    pub user_id: Option<Uuid>,
    pub event_type: String,
    pub event_type_category: String,
    pub description: String,
    pub metadata: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl ActivityKafkaEvent {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_activity_event_type_to_string() {
        assert_eq!(ActivityEventType::UserRegistered.as_str(), "user_registered");
        assert_eq!(ActivityEventType::UserLoggedIn.as_str(), "user_logged_in");
        assert_eq!(ActivityEventType::MessageSent.as_str(), "message_sent");
    }

    #[test]
    fn test_activity_event_type_from_string() {
        assert_eq!(
            ActivityEventType::from_str("user_registered").unwrap(),
            ActivityEventType::UserRegistered
        );
        assert!(ActivityEventType::from_str("invalid_type").is_err());
    }

    #[test]
    fn test_activity_kafka_event_serialization() {
        let event = ActivityKafkaEvent {
            event_id: Uuid::new_v4(),
            user_id: Some(Uuid::new_v4()),
            event_type: "user_logged_in".to_string(),
            event_type_category: "user".to_string(),
            description: "Test event".to_string(),
            metadata: Some(serde_json::json!({"key": "value"})),
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("TestAgent".to_string()),
            timestamp: Utc::now(),
        };

        let json = event.to_json();
        assert!(json.is_ok());

        let deserialized = ActivityKafkaEvent::from_json(&json.unwrap());
        assert!(deserialized.is_ok());
    }

    #[test]
    fn test_activity_log_conversion() {
        let log = ActivityLog {
            id: Uuid::new_v4(),
            user_id: Some(Uuid::new_v4()),
            event_type: "user_logged_in".to_string(),
            event_type_category: "user".to_string(),
            description: "Test".to_string(),
            metadata: None,
            ip_address: None,
            user_agent: None,
            created_at: Utc::now(),
        };

        let dto: ActivityLogDto = log.into();
        assert_eq!(dto.event_type, "user_logged_in");
        assert!(dto.username.is_none()); // Not filled by conversion
    }
}

