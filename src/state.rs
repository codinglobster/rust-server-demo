//! Application state (shared by API and WebSocket routes)

use crate::auth::jwt::JwtService;
use crate::auth::middleware::AuthState;
use crate::cache::RedisClient;
use crate::database::Database;
use crate::services::{AuthService, UserService, ActivityService};
use axum::extract::FromRef;
use std::sync::Arc;

#[cfg(feature = "kafka")]
use crate::kafka::KafkaProducer;

/// Application state (single state type for all routes)
#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,
    pub user_service: UserService,
    pub activity_service: ActivityService,
    pub db: Database,
    pub redis: RedisClient,
    pub jwt_service: Arc<JwtService>,
    pub auth_state: AuthState,
    #[cfg(feature = "kafka")]
    pub kafka: Option<Arc<KafkaProducer>>,
}

impl FromRef<AppState> for AuthService {
    fn from_ref(state: &AppState) -> Self {
        state.auth_service.clone()
    }
}

impl FromRef<AppState> for UserService {
    fn from_ref(state: &AppState) -> Self {
        state.user_service.clone()
    }
}

impl FromRef<AppState> for ActivityService {
    fn from_ref(state: &AppState) -> Self {
        state.activity_service.clone()
    }
}

impl FromRef<AppState> for AuthState {
    fn from_ref(state: &AppState) -> Self {
        state.auth_state.clone()
    }
}
