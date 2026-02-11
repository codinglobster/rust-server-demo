//! WebSocket connection manager

use crate::cache::RedisClient;
use crate::cache::keys::CacheKeys;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

/// Connection information
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub id: Uuid,
    pub addr: SocketAddr,
    pub user_id: Option<Uuid>,
    pub username: Option<String>,
    pub rooms: Vec<String>,
    pub connected_at: chrono::DateTime<chrono::Utc>,
}

/// Connection manager
#[derive(Clone)]
pub struct ConnectionManager {
    connections: HashMap<Uuid, ConnectionInfo>,
    senders: HashMap<Uuid, UnboundedSender<String>>,
    redis: RedisClient,
}

impl ConnectionManager {
    /// Create a new connection manager
    pub fn new(redis: RedisClient) -> Self {
        Self {
            connections: HashMap::new(),
            senders: HashMap::new(),
            redis,
        }
    }

    /// Add a new connection
    pub fn add_connection(
        &mut self,
        id: Uuid,
        addr: SocketAddr,
        sender: UnboundedSender<String>,
    ) {
        let info = ConnectionInfo {
            id,
            addr,
            user_id: None,
            username: None,
            rooms: vec![],
            connected_at: chrono::Utc::now(),
        };

        self.connections.insert(id, info);
        self.senders.insert(id, sender);

        tracing::debug!("Connection added: {} from {}", id, addr);
    }

    /// Remove a connection
    pub async fn remove_connection(&mut self, id: &Uuid) {
        if let Some(info) = self.connections.remove(id) {
            // Remove from Redis online users
            if let Some(user_id) = info.user_id {
                let _ = self.redis.srem(CacheKeys::ONLINE_USERS, &user_id.to_string()).await;
            }

            // Remove from all rooms
            for room in &info.rooms {
                let room_key = CacheKeys::room_members(room);
                let _ = self.redis.srem(&room_key, &id.to_string()).await;
            }
        }

        self.senders.remove(id);

        tracing::debug!("Connection removed: {}", id);
    }

    /// Authenticate a connection
    pub fn authenticate(&mut self, conn_id: &Uuid, user_id: Uuid, username: String) {
        if let Some(info) = self.connections.get_mut(conn_id) {
            info.user_id = Some(user_id);
            info.username = Some(username.clone());

            tracing::info!("Connection {} authenticated as user {}", conn_id, username);
        }
    }

    /// Join a room
    pub async fn join_room(&mut self, conn_id: &Uuid, room_id: &str) {
        if let Some(info) = self.connections.get_mut(conn_id) {
            if !info.rooms.contains(&room_id.to_string()) {
                info.rooms.push(room_id.to_string());

                // Add to Redis set
                let room_key = CacheKeys::room_members(room_id);
                let _ = self.redis.sadd(&room_key, &conn_id.to_string()).await;

                tracing::debug!("Connection {} joined room {}", conn_id, room_id);
            }
        }
    }

    /// Leave a room
    pub async fn leave_room(&mut self, conn_id: &Uuid, room_id: &str) {
        if let Some(info) = self.connections.get_mut(conn_id) {
            info.rooms.retain(|r| r != room_id);

            // Remove from Redis set
            let room_key = CacheKeys::room_members(room_id);
            let _ = self.redis.srem(&room_key, &conn_id.to_string()).await;

            tracing::debug!("Connection {} left room {}", conn_id, room_id);
        }
    }

    /// Send a message to a specific connection
    pub fn send_to(&self, conn_id: &Uuid, message: &str) -> bool {
        if let Some(sender) = self.senders.get(conn_id) {
            sender
                .send(message.to_string())
                .map_err(|e| {
                    tracing::error!("Failed to send message to {}: {}", conn_id, e);
                })
                .is_ok()
        } else {
            false
        }
    }

    /// Send a message to all connections
    pub fn broadcast(&self, message: &str) {
        let mut failed = Vec::new();

        for (id, sender) in &self.senders {
            if sender
                .send(message.to_string())
                .is_err()
            {
                failed.push(*id);
            }
        }

        // Clean up failed connections
        for id in failed {
            tracing::warn!("Removing failed connection: {}", id);
            // Note: We can't remove here directly because we're borrowing self
            // This will be cleaned up by the connection close handler
        }
    }

    /// Send a message to all members of a room
    pub async fn send_to_room(&self, room_id: &str, message: &str) {
        let room_key = CacheKeys::room_members(room_id);

        if let Ok(members) = self.redis.smembers(&room_key).await {
            for member_str in members {
                if let Ok(member_id) = Uuid::parse_str(&member_str) {
                    self.send_to(&member_id, message);
                }
            }
        }
    }

    /// Get connection count
    pub fn count(&self) -> usize {
        self.connections.len()
    }

    /// Get room count
    pub fn room_count(&self) -> usize {
        let mut rooms = std::collections::HashSet::new();
        for info in self.connections.values() {
            for room in &info.rooms {
                rooms.insert(room.clone());
            }
        }
        rooms.len()
    }

    /// Get connection info
    pub fn get_connection(&self, id: &Uuid) -> Option<&ConnectionInfo> {
        self.connections.get(id)
    }

    /// Get all online user IDs
    pub async fn online_users(&self) -> Vec<Uuid> {
        if let Ok(members) = self.redis.smembers(CacheKeys::ONLINE_USERS).await {
            members
                .into_iter()
                .filter_map(|s| Uuid::parse_str(&s).ok())
                .collect()
        } else {
            vec![]
        }
    }

    /// Iterate over all connections
    pub fn iter_connections(&self) -> impl Iterator<Item = (&Uuid, &ConnectionInfo)> {
        self.connections.iter()
    }
}
