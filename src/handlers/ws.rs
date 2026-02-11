//! WebSocket handler

use crate::auth::jwt::JwtService;
use crate::cache::client::RedisClient;
use crate::state::AppState;
use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};

/// WebSocket handler state (used when running WS with dedicated state)
#[derive(Clone)]
pub struct WebSocketState {
    pub jwt_service: JwtService,
    pub redis: RedisClient,
}

/// WebSocket connection handler (uses AppState)
pub async fn websocket_handler(
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let ws_state = WebSocketState {
        jwt_service: state.jwt_service.as_ref().clone(),
        redis: state.redis.clone(),
    };
    ws.on_upgrade(|socket| handle_socket(socket, ws_state))
}

/// Handle individual WebSocket connection
async fn handle_socket(socket: WebSocket, state: WebSocketState) {
    let (mut sender, mut receiver) = socket.split();

    // Send welcome message
    let welcome = serde_json::json!({
        "type": "system",
        "data": {"message": "Connected to server"}
    });

    if let Ok(msg) = serde_json::to_string(&welcome) {
        let _ = sender.send(Message::Text(msg.into())).await;
    }

    // Handle incoming messages
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
                                    let _ = sender.send(Message::Text(msg.into())).await;
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
                                            let success = serde_json::json!({
                                                "type": "auth_success",
                                                "data": {
                                                    "user_id": claims.sub,
                                                    "username": claims.username
                                                }
                                            });
                                            if let Ok(msg) = serde_json::to_string(&success) {
                                                let _ = sender.send(Message::Text(msg.into())).await;
                                            }
                                        }
                                        Err(_) => {
                                            let error = serde_json::json!({
                                                "type": "error",
                                                "data": {"message": "Invalid token"}
                                            });
                                            if let Ok(msg) = serde_json::to_string(&error) {
                                                let _ = sender.send(Message::Text(msg.into())).await;
                                            }
                                        }
                                    }
                                }
                            }
                            "chat" => {
                                // Handle chat message
                                if let Some(data) = ws_msg.get("data") {
                                    // Echo back the message
                                    let response = serde_json::json!({
                                        "type": "chat",
                                        "data": data
                                    });
                                    if let Ok(msg) = serde_json::to_string(&response) {
                                        let _ = sender.send(Message::Text(msg.into())).await;
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
                let _ = sender.send(Message::Pong(bytes)).await;
            }
            _ => {}
        }
    }

    tracing::debug!("WebSocket handler ended");
}
