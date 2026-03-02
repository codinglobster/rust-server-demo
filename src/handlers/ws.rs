//! WebSocket handler with room broadcast support

use crate::auth::jwt::JwtService;
use crate::cache::client::RedisClient;
use crate::services::{MessageService, RoomService};
use crate::state::AppState;
use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

/// WebSocket connection state
#[derive(Clone)]
pub struct WsConnection {
    pub user_id: Uuid,
    pub username: String,
    pub tx: mpsc::UnboundedSender<String>,
}

/// WebSocket server state managing all connections
pub struct WsServer {
    pub connections: Arc<RwLock<HashMap<Uuid, WsConnection>>>,
    pub room_members: Arc<RwLock<HashMap<Uuid, Vec<Uuid>>>>, // room_id -> [user_ids]
}

impl WsServer {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            room_members: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add connection
    pub async fn add_connection(&self, user_id: Uuid, username: String, tx: mpsc::UnboundedSender<String>) {
        let conn = WsConnection { user_id, username, tx };
        self.connections.write().await.insert(user_id, conn);
    }

    /// Remove connection
    pub async fn remove_connection(&self, user_id: Uuid) {
        self.connections.write().await.remove(&user_id);

        // Remove from all rooms
        let mut room_members = self.room_members.write().await;
        for members in room_members.values_mut() {
            members.retain(|id| *id != user_id);
        }
    }

    /// Join room
    pub async fn join_room(&self, room_id: Uuid, user_id: Uuid) {
        let mut room_members = self.room_members.write().await;
        room_members.entry(room_id).or_insert_with(Vec::new).push(user_id);
    }

    /// Leave room
    pub async fn leave_room(&self, room_id: Uuid, user_id: Uuid) {
        let mut room_members = self.room_members.write().await;
        if let Some(members) = room_members.get_mut(&room_id) {
            members.retain(|id| *id != user_id);
        }
    }

    /// Broadcast to room
    pub async fn broadcast_to_room(&self, room_id: Uuid, message: String, exclude_user: Option<Uuid>) {
        let room_members = self.room_members.read().await;
        let connections = self.connections.read().await;

        if let Some(members) = room_members.get(&room_id) {
            for user_id in members {
                if let Some(exclude) = exclude_user {
                    if *user_id == exclude {
                        continue;
                    }
                }

                if let Some(conn) = connections.get(user_id) {
                    let _ = conn.tx.send(message.clone());
                }
            }
        }
    }

    /// Send to specific user
    pub async fn send_to_user(&self, user_id: Uuid, message: String) {
        let connections = self.connections.read().await;
        if let Some(conn) = connections.get(&user_id) {
            let _ = conn.tx.send(message);
        }
    }
}

/// WebSocket handler state
#[derive(Clone)]
pub struct WebSocketState {
    pub jwt_service: JwtService,
    pub redis: RedisClient,
    pub message_service: MessageService,
    pub room_service: RoomService,
    pub ws_server: Arc<WsServer>,
}

/// WebSocket connection handler (uses AppState)
pub async fn websocket_handler(
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    // Create or get shared WsServer
    static WS_SERVER: once_cell::sync::Lazy<Arc<WsServer>> =
        once_cell::sync::Lazy::new(|| Arc::new(WsServer::new()));

    let ws_state = WebSocketState {
        jwt_service: state.jwt_service.as_ref().clone(),
        redis: state.redis.clone(),
        message_service: state.message_service.clone(),
        room_service: state.room_service.clone(),
        ws_server: WS_SERVER.clone(),
    };

    ws.on_upgrade(|socket| handle_socket(socket, ws_state))
}

/// Handle individual WebSocket connection
async fn handle_socket(socket: WebSocket, state: WebSocketState) {
    let (mut sender, mut receiver) = socket.split();

    // Create channel for this connection
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // Spawn task to handle outgoing messages
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Send welcome message
    let welcome = serde_json::json!({
        "type": "system",
        "data": {"message": "Connected to server. Please authenticate."}
    });

    if let Ok(msg) = serde_json::to_string(&welcome) {
        let _ = tx.send(msg);
    }

    let mut authenticated_user: Option<(Uuid, String)> = None;

    // Handle incoming messages
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    // Parse incoming message
                    if let Ok(ws_msg) = serde_json::from_str::<serde_json::Value>(&text) {
                        tracing::debug!("Received WebSocket message: {}", ws_msg);

                        // Handle different message types
                        if let Some(msg_type) = ws_msg.get("type").and_then(|v| v.as_str()) {
                            match msg_type {
                                "ping" => {
                                    // Respond with pong
                                    let pong = serde_json::json!({"type": "pong"});
                                    if let Ok(msg) = serde_json::to_string(&pong) {
                                        let _ = tx.send(msg);
                                    }
                                }
                                "auth" => {
                                    // Handle authentication
                                    if let Some(token) = ws_msg
                                        .get("data")
                                        .and_then(|d| d.get("token"))
                                        .and_then(|t| t.as_str())
                                    {
                                        match state.jwt_service.validate_token(token) {
                                            Ok(claims) => {
                                                let user_id = match Uuid::parse_str(&claims.sub) {
                                                    Ok(id) => id,
                                                    Err(_) => continue,
                                                };

                                                authenticated_user = Some((user_id, claims.username.clone()));

                                                // Add to connection pool
                                                state.ws_server.add_connection(user_id, claims.username.clone(), tx.clone()).await;

                                                let success = serde_json::json!({
                                                    "type": "auth_success",
                                                    "data": {
                                                        "user_id": claims.sub,
                                                        "username": claims.username
                                                    }
                                                });
                                                if let Ok(msg) = serde_json::to_string(&success) {
                                                    let _ = tx.send(msg);
                                                }
                                            }
                                            Err(_) => {
                                                let error = serde_json::json!({
                                                    "type": "error",
                                                    "data": {"message": "Invalid token"}
                                                });
                                                if let Ok(msg) = serde_json::to_string(&error) {
                                                    let _ = tx.send(msg);
                                                }
                                            }
                                        }
                                    }
                                }
                                "join_room" => {
                                    // Join a room
                                    if let Some((user_id, _)) = authenticated_user {
                                        if let Some(room_id_str) = ws_msg
                                            .get("data")
                                            .and_then(|d| d.get("room_id"))
                                            .and_then(|r| r.as_str())
                                        {
                                            if let Ok(room_id) = Uuid::parse_str(room_id_str) {
                                                // Check if user is member of the room
                                                if let Ok(is_member) = state.room_service.is_member(room_id, user_id).await {
                                                    if is_member {
                                                        state.ws_server.join_room(room_id, user_id).await;

                                                        let success = serde_json::json!({
                                                            "type": "room_joined",
                                                            "data": {"room_id": room_id_str}
                                                        });
                                                        if let Ok(msg) = serde_json::to_string(&success) {
                                                            let _ = tx.send(msg);
                                                        }

                                                        // Notify room
                                                        let notification = serde_json::json!({
                                                            "type": "user_joined",
                                                            "data": {
                                                                "room_id": room_id_str,
                                                                "user_id": user_id.to_string()
                                                            }
                                                        });
                                                        if let Ok(msg) = serde_json::to_string(&notification) {
                                                            state.ws_server.broadcast_to_room(room_id, msg, Some(user_id)).await;
                                                        }
                                                    } else {
                                                        let error = serde_json::json!({
                                                            "type": "error",
                                                            "data": {"message": "Not a member of this room"}
                                                        });
                                                        if let Ok(msg) = serde_json::to_string(&error) {
                                                            let _ = tx.send(msg);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        let error = serde_json::json!({
                                            "type": "error",
                                            "data": {"message": "Not authenticated"}
                                        });
                                        if let Ok(msg) = serde_json::to_string(&error) {
                                            let _ = tx.send(msg);
                                        }
                                    }
                                }
                                "leave_room" => {
                                    // Leave a room
                                    if let Some((user_id, _)) = authenticated_user {
                                        if let Some(room_id_str) = ws_msg
                                            .get("data")
                                            .and_then(|d| d.get("room_id"))
                                            .and_then(|r| r.as_str())
                                        {
                                            if let Ok(room_id) = Uuid::parse_str(room_id_str) {
                                                state.ws_server.leave_room(room_id, user_id).await;

                                                let success = serde_json::json!({
                                                    "type": "room_left",
                                                    "data": {"room_id": room_id_str}
                                                });
                                                if let Ok(msg) = serde_json::to_string(&success) {
                                                    let _ = tx.send(msg);
                                                }

                                                // Notify room
                                                let notification = serde_json::json!({
                                                    "type": "user_left",
                                                    "data": {
                                                        "room_id": room_id_str,
                                                        "user_id": user_id.to_string()
                                                    }
                                                });
                                                if let Ok(msg) = serde_json::to_string(&notification) {
                                                    state.ws_server.broadcast_to_room(room_id, msg, None).await;
                                                }
                                            }
                                        }
                                    }
                                }
                                "room_message" => {
                                    // Send message to room
                                    if let Some((user_id, ref username)) = authenticated_user {
                                        if let Some(data) = ws_msg.get("data") {
                                            if let (Some(room_id_str), Some(content)) = (
                                                data.get("room_id").and_then(|r| r.as_str()),
                                                data.get("content").and_then(|c| c.as_str()),
                                            ) {
                                                if let Ok(room_id) = Uuid::parse_str(room_id_str) {
                                                    // Persist message
                                                    let create_req = crate::models::message::CreateMessageRequest {
                                                        content: content.to_string(),
                                                        message_type: Some("room".to_string()),
                                                        room_id: Some(room_id),
                                                        recipient_id: None,
                                                        metadata: None,
                                                    };

                                                    if let Ok(message) = state.message_service.create_message(user_id, &create_req).await {
                                                        // Broadcast to room
                                                        let broadcast = serde_json::json!({
                                                            "type": "room_message",
                                                            "data": {
                                                                "id": message.id.to_string(),
                                                                "room_id": room_id_str,
                                                                "sender_id": user_id.to_string(),
                                                                "sender_username": username,
                                                                "content": content,
                                                                "created_at": message.created_at
                                                            }
                                                        });
                                                        if let Ok(msg) = serde_json::to_string(&broadcast) {
                                                            state.ws_server.broadcast_to_room(room_id, msg, None).await;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        let error = serde_json::json!({
                                            "type": "error",
                                            "data": {"message": "Not authenticated"}
                                        });
                                        if let Ok(msg) = serde_json::to_string(&error) {
                                            let _ = tx.send(msg);
                                        }
                                    }
                                }
                                "private_message" => {
                                    // Send private message
                                    if let Some((user_id, ref username)) = authenticated_user {
                                        if let Some(data) = ws_msg.get("data") {
                                            if let (Some(recipient_id_str), Some(content)) = (
                                                data.get("recipient_id").and_then(|r| r.as_str()),
                                                data.get("content").and_then(|c| c.as_str()),
                                            ) {
                                                if let Ok(recipient_id) = Uuid::parse_str(recipient_id_str) {
                                                    // Persist message
                                                    let create_req = crate::models::message::CreateMessageRequest {
                                                        content: content.to_string(),
                                                        message_type: Some("private".to_string()),
                                                        room_id: None,
                                                        recipient_id: Some(recipient_id),
                                                        metadata: None,
                                                    };

                                                    if let Ok(message) = state.message_service.create_message(user_id, &create_req).await {
                                                        // Send to recipient
                                                        let private_msg = serde_json::json!({
                                                            "type": "private_message",
                                                            "data": {
                                                                "id": message.id.to_string(),
                                                                "sender_id": user_id.to_string(),
                                                                "sender_username": username,
                                                                "content": content,
                                                                "created_at": message.created_at
                                                            }
                                                        });
                                                        if let Ok(msg) = serde_json::to_string(&private_msg) {
                                                            state.ws_server.send_to_user(recipient_id, msg).await;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        let error = serde_json::json!({
                                            "type": "error",
                                            "data": {"message": "Not authenticated"}
                                        });
                                        if let Ok(msg) = serde_json::to_string(&error) {
                                            let _ = tx.send(msg);
                                        }
                                    }
                                }
                                _ => {
                                    tracing::warn!("Unknown message type: {}", msg_type);
                                }
                            }
                        }
                    }
                }
                Message::Close(_) => {
                    tracing::debug!("WebSocket connection closed");
                    break;
                }
                Message::Ping(bytes) => {
                    // Pong is handled by the framework
                    tracing::trace!("Received ping: {:?}", bytes);
                }
                _ => {}
            }
        }

        // Cleanup on disconnect
        if let Some((user_id, _)) = authenticated_user {
            state.ws_server.remove_connection(user_id).await;
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }

    tracing::debug!("WebSocket handler ended");
}
