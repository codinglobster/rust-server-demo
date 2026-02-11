//! Message broadcaster

use crate::cache::client::RedisClient;
use crate::cache::keys::PubSubChannels;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Broadcaster for sending messages to clients
#[derive(Clone)]
pub struct Broadcaster {
    redis: RedisClient,
    connections: Arc<RwLock<crate::websocket::connection::ConnectionManager>>,
}

impl Broadcaster {
    /// Create a new broadcaster
    pub fn new(
        redis: RedisClient,
        connections: Arc<RwLock<crate::websocket::connection::ConnectionManager>>,
    ) -> Self {
        Self { redis, connections }
    }

    /// Broadcast a message to all connected clients
    pub async fn broadcast_all<T: Serialize>(&self, message_type: &str, data: &T) {
        let msg = OutgoingMessage {
            message_type: message_type.to_string(),
            data,
        };

        if let Ok(json) = serde_json::to_string(&msg) {
            // Send to all local connections
            self.connections.read().await.broadcast(&json);

            // Publish to Redis for other instances
            let _ = self
                .redis
                .publish(PubSubChannels::MESSAGES, &json)
                .await;
        }
    }

    /// Send a message to a specific room
    pub async fn send_to_room<T: Serialize>(
        &self,
        room_id: &str,
        message_type: &str,
        data: &T,
    ) {
        let msg = OutgoingMessage {
            message_type: message_type.to_string(),
            data,
        };

        if let Ok(json) = serde_json::to_string(&msg) {
            // Send to room members
            self.connections
                .read()
                .await
                .send_to_room(room_id, &json);

            // Publish to Redis for distribution
            let _ = self
                .redis
                .publish(&format!("{}:{}", PubSubChannels::MESSAGES, room_id), &json)
                .await;
        }
    }

    /// Send a message to a specific user
    pub async fn send_to_user<T: Serialize>(
        &self,
        user_id: Uuid,
        message_type: &str,
        data: &T,
    ) {
        let msg = OutgoingMessage {
            message_type: message_type.to_string(),
            data,
        };

        if let Ok(json) = serde_json::to_string(&msg) {
            let connections = self.connections.read().await;

            // Find all connections for this user
            for (id, info) in connections.iter_connections() {
                if info.user_id == Some(user_id) {
                    connections.send_to(id, &json);
                }
            }
        }
    }
}

/// Outgoing message format
#[derive(Debug, Serialize)]
struct OutgoingMessage<T: Serialize> {
    #[serde(rename = "type")]
    message_type: String,
    data: T,
}
