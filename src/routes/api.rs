//! API routes

use crate::handlers::{auth, health, user};
use crate::state::AppState;
use axum::middleware;
use axum::{
    routing::{get, post, put},
    Router,
};
use utoipa::OpenApi;

/// API documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        auth::register,
        auth::login,
        auth::refresh_token,
        auth::logout,
        user::get_me,
        user::get_user,
        user::list_users,
        user::update_user,
        user::change_password,
    ),
    components(
        schemas(
            crate::models::user::UserDto,
            crate::models::user::RegisterRequest,
            crate::models::user::LoginRequest,
            crate::models::user::UpdateUserRequest,
            crate::models::user::ChangePasswordRequest,
            crate::models::session::LoginResponse,
            crate::models::session::RefreshTokenRequest,
            crate::auth::jwt::TokenPair,
            health::HealthResponse,
            user::UsersListResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Authentication", description = "User authentication"),
        (name = "Users", description = "User management"),
    ),
    info(
        title = "Rust Server Demo API",
        version = "0.1.0",
        description = "A real-time/long-connection server API with PostgreSQL, Redis, and Kafka",
    )
)]
pub struct ApiDoc;

/// Create API router (single state type AppState)
pub fn create_api_routes(state: AppState) -> Router<AppState> {
    let health_routes = Router::new()
        .route("/health", get(health::health_check))
        .route("/liveness", get(health::liveness))
        .route("/readiness", get(health::readiness))
        .route("/version", get(health::version));

    let auth_routes = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/refresh", post(auth::refresh_token))
        .route("/logout", post(auth::logout));

    let user_routes = Router::new()
        .route("/me", get(user::get_me).put(user::update_user))
        .route("/me/password", post(user::change_password))
        .route("/{id}", get(user::get_user))
        .route("/", get(user::list_users))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::auth_middleware,
        ));

    Router::new()
        .nest("/health", health_routes)
        .nest("/auth", auth_routes)
        .nest("/users", user_routes)
        .with_state(state)
}

/// Create the complete API router
pub fn create_app_router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/api", create_api_routes(state.clone()))
        .with_state(state)
}
