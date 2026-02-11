//! Message model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Message model from database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
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

/// Message types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Text,
    System,
    Private,
    Room,
    Notification,
}

impl MessageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageType::Text => "text",
            MessageType::System => "system",
            MessageType::Private => "private",
            MessageType::Room => "room",
            MessageType::Notification => "notification",
        }
    }
}

impl std::str::FromStr for MessageType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(MessageType::Text),
            "system" => Ok(MessageType::System),
            "private" => Ok(MessageType::Private),
            "room" => Ok(MessageType::Room),
            "notification" => Ok(MessageType::Notification),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Message create request
#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate)]
pub struct CreateMessageRequest {
    pub content: String,
    pub message_type: Option<String>,
    pub room_id: Option<Uuid>,
    pub recipient_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
}

/// Message update request
#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate)]
pub struct UpdateMessageRequest {
    #[validate(length(max = 5000))]
    pub content: String,
}

/// Message DTO for API responses
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MessageDto {
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

impl From<Message> for MessageDto {
    fn from(message: Message) -> Self {
        Self {
            id: message.id,
            sender_id: message.sender_id,
            content: message.content,
            message_type: message.message_type,
            room_id: message.room_id,
            recipient_id: message.recipient_id,
            is_edited: message.is_edited,
            metadata: message.metadata,
            created_at: message.created_at,
            updated_at: message.updated_at,
        }
    }
}

/// WebSocket message format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WsMessage {
    /// Authentication message
    #[serde(rename = "auth")]
    Auth { token: String },
    /// Chat message
    #[serde(rename = "chat")]
    Chat { content: String, room_id: Option<String> },
    /// System message
    #[serde(rename = "system")]
    System { content: String },
    /// Private message
    #[serde(rename = "private")]
    Private { recipient_id: String, content: String },
    /// Presence update
    #[serde(rename = "presence")]
    Presence { status: String },
    /// Error message
    #[serde(rename = "error")]
    Error { code: String, message: String },
    /// Pong message
    #[serde(rename = "pong")]
    Pong,
    /// Ping message
    #[serde(rename = "ping")]
    Ping,
}

impl WsMessage {
    /// Serialize message to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserialize message from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Message list response with pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesResponse {
    pub messages: Vec<MessageDto>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}
