//! WebSocket server

use crate::websocket::{Broadcaster, ConnectionManager, WsMessageHandler};
use crate::auth::jwt::JwtService;
use crate::cache::RedisClient;
use std::sync::Arc;
use tokio::sync::RwLock;

/// WebSocket server state
#[derive(Clone)]
pub struct WebSocketServer {
    pub connections: Arc<RwLock<ConnectionManager>>,
    pub broadcaster: Arc<Broadcaster>,
    pub message_handler: Arc<WsMessageHandler>,
}

impl WebSocketServer {
    /// Create a new WebSocket server
    pub fn new(jwt_service: JwtService, redis: RedisClient) -> Self {
        let connections = Arc::new(RwLock::new(ConnectionManager::new(redis.clone())));
        let broadcaster = Arc::new(Broadcaster::new(redis.clone(), connections.clone()));
        let message_handler = Arc::new(WsMessageHandler::new(
            jwt_service,
            redis,
            connections.clone(),
            broadcaster.clone(),
        ));

        Self {
            connections,
            broadcaster,
            message_handler,
        }
    }

    /// Get server statistics
    pub async fn stats(&self) -> ServerStats {
        let connections = self.connections.read().await;
        ServerStats {
            active_connections: connections.count(),
            rooms_count: connections.room_count(),
        }
    }
}

/// Server statistics
#[derive(Debug, Clone, serde::Serialize)]
pub struct ServerStats {
    pub active_connections: usize,
    pub rooms_count: usize,
}
