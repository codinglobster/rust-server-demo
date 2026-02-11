//! WebSocket message handler

use crate::auth::jwt::JwtService;
use crate::cache::RedisClient;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Incoming message format
#[derive(Debug, serde::Deserialize)]
pub struct IncomingMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub data: Option<serde_json::Value>,
}

/// Message handler
pub struct WsMessageHandler {
    jwt_service: JwtService,
    redis: RedisClient,
    connections: Arc<RwLock<crate::websocket::connection::ConnectionManager>>,
    #[allow(dead_code)]
    broadcaster: Arc<crate::websocket::broadcast::Broadcaster>,
}

impl WsMessageHandler {
    /// Create a new message handler
    pub fn new(
        jwt_service: JwtService,
        redis: RedisClient,
        connections: Arc<RwLock<crate::websocket::connection::ConnectionManager>>,
        broadcaster: Arc<crate::websocket::broadcast::Broadcaster>,
    ) -> Self {
        Self {
            jwt_service,
            redis,
            connections,
            broadcaster,
        }
    }

    /// Handle an incoming message
    pub async fn handle_message(
        &self,
        conn_id: Uuid,
        message: &str,
    ) -> Result<Option<String>, String> {
        // Parse message
        let msg: IncomingMessage = serde_json::from_str(message)
            .map_err(|e| format!("Invalid JSON: {}", e))?;

        match msg.message_type.as_str() {
            "auth" => self.handle_auth(conn_id, msg.data).await,
            "ping" => Ok(Some(r#"{"type":"pong"}"#.to_string())),
            "chat" => self.handle_chat(conn_id, msg.data).await,
            "join" => self.handle_join(conn_id, msg.data).await,
            "leave" => self.handle_leave(conn_id, msg.data).await,
            "presence" => self.handle_presence(conn_id).await,
            _ => Err(format!("Unknown message type: {}", msg.message_type)),
        }
    }

    /// Handle authentication
    async fn handle_auth(
        &self,
        conn_id: Uuid,
        data: Option<serde_json::Value>,
    ) -> Result<Option<String>, String> {
        let token = data
            .and_then(|d| d.get("token").and_then(|t| t.as_str()).map(|s| s.to_string()))
            .ok_or_else(|| "Missing token".to_string())?;

        match self.jwt_service.validate_token(&token) {
            Ok(claims) => {
                let user_id = claims.user_id()
                    .map_err(|e| format!("Invalid user ID: {}", e))?;

                // Update connection
                self.connections
                    .write()
                    .await
                    .authenticate(&conn_id, user_id, claims.username.clone());

                // Add to online users
                let _ = self
                    .redis
                    .sadd(crate::cache::keys::CacheKeys::ONLINE_USERS, &user_id.to_string())
                    .await;

                // Send success response
                let response = serde_json::json!({
                    "type": "auth_success",
                    "data": {
                        "user_id": user_id,
                        "username": claims.username
                    }
                });

                Ok(Some(response.to_string()))
            }
            Err(_) => {
                let response = serde_json::json!({
                    "type": "error",
                    "data": {"message": "Invalid token"}
                });
                Ok(Some(response.to_string()))
            }
        }
    }

    /// Handle chat message
    async fn handle_chat(
        &self,
        conn_id: Uuid,
        data: Option<serde_json::Value>,
    ) -> Result<Option<String>, String> {
        let data = data.ok_or_else(|| "Missing data".to_string())?;

        let content = data
            .get("content")
            .and_then(|c| c.as_str())
            .ok_or_else(|| "Missing content".to_string())?;

        let room_id = data.get("room_id").and_then(|r| r.as_str());

        // Get connection info
        let connections = self.connections.read().await;
        let conn_info = connections
            .get_connection(&conn_id)
            .ok_or_else(|| "Connection not found".to_string())?;

        let user_id = conn_info
            .user_id
            .ok_or_else(|| "Not authenticated".to_string())?;

        let username = conn_info
            .username
            .as_ref()
            .ok_or_else(|| "No username".to_string())?;

        // Broadcast message
        if let Some(room) = room_id {
            // Send to room
            let response = serde_json::json!({
                "type": "chat",
                "data": {
                    "user_id": user_id,
                    "username": username,
                    "content": content,
                    "room_id": room,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }
            });

            // Note: We'd need an async broadcast here
            // For now, return the message
            Ok(Some(response.to_string()))
        } else {
            // Broadcast to all
            let response = serde_json::json!({
                "type": "chat",
                "data": {
                    "user_id": user_id,
                    "username": username,
                    "content": content,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }
            });

            Ok(Some(response.to_string()))
        }
    }

    /// Handle join room
    async fn handle_join(
        &self,
        conn_id: Uuid,
        data: Option<serde_json::Value>,
    ) -> Result<Option<String>, String> {
        let room_id = data
            .and_then(|d| d.get("room_id").and_then(|r| r.as_str()).map(|s| s.to_string()))
            .ok_or_else(|| "Missing room_id".to_string())?;

        self.connections
            .write()
            .await
            .join_room(&conn_id, &room_id)
            .await;

        let response = serde_json::json!({
            "type": "join_success",
            "data": {"room_id": room_id}
        });

        Ok(Some(response.to_string()))
    }

    /// Handle leave room
    async fn handle_leave(
        &self,
        conn_id: Uuid,
        data: Option<serde_json::Value>,
    ) -> Result<Option<String>, String> {
        let room_id = data
            .and_then(|d| d.get("room_id").and_then(|r| r.as_str()).map(|s| s.to_string()))
            .ok_or_else(|| "Missing room_id".to_string())?;

        self.connections
            .write()
            .await
            .leave_room(&conn_id, &room_id)
            .await;

        let response = serde_json::json!({
            "type": "leave_success",
            "data": {"room_id": room_id}
        });

        Ok(Some(response.to_string()))
    }

    /// Handle presence request
    async fn handle_presence(&self, conn_id: Uuid) -> Result<Option<String>, String> {
        let connections = self.connections.read().await;
        let conn_info = connections
            .get_connection(&conn_id)
            .ok_or_else(|| "Connection not found".to_string())?;

        let online_users = connections.online_users().await;

        let response = serde_json::json!({
            "type": "presence",
            "data": {
                "online_count": online_users.len(),
                "rooms_count": connections.room_count(),
                "user_id": conn_info.user_id,
                "username": conn_info.username
            }
        });

        Ok(Some(response.to_string()))
    }
}
