//! WebSocket routes

use crate::handlers::ws::websocket_handler;
use crate::state::AppState;
use axum::{routing::get, Router};

/// Create WebSocket router (uses same AppState as API)
pub fn create_ws_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(state)
}
