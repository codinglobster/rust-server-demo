//! API routes

use crate::handlers::{auth, health, user, activity};
use crate::state::AppState;
use axum::middleware;
#[allow(unused_imports)]
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
        user::update_user_role,
        activity::list_activities,
        activity::get_recent_activities,
        activity::list_user_activities,
    ),
    components(
        schemas(
            crate::models::user::UserDto,
            crate::models::user::RegisterRequest,
            crate::models::user::LoginRequest,
            crate::models::user::UpdateUserRequest,
            crate::models::user::ChangePasswordRequest,
            crate::models::user::UpdateUserRoleRequest,
            crate::models::session::LoginResponse,
            crate::models::session::RefreshTokenRequest,
            crate::auth::jwt::TokenPair,
            health::HealthResponse,
            user::UsersListResponse,
            crate::models::activity::ActivityLogDto,
            crate::models::activity::ActivityLogsResponse,
            crate::models::activity::CreateActivityLogRequest,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Authentication", description = "User authentication"),
        (name = "Users", description = "User management"),
        (name = "Activities", description = "Activity logs and event tracking"),
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

    // Auth routes without authentication
    let auth_public_routes = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/refresh", post(auth::refresh_token));

    // Auth routes with authentication (logout)
    let auth_protected_routes = Router::new()
        .route("/logout", post(auth::logout))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::auth_middleware,
        ));

    // Combine auth routes
    let auth_routes = Router::new()
        .merge(auth_public_routes)
        .merge(auth_protected_routes);

    let user_routes = Router::new()
        .route("/me", get(user::get_me).put(user::update_user))
        .route("/me/password", post(user::change_password))
        .route("/{id}", get(user::get_user))
        .route("/", get(user::list_users))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::auth_middleware,
        ));

    // Admin routes (require admin role)
    let admin_routes = Router::new()
        .route("/{id}/role", axum::routing::put(user::update_user_role))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::auth_middleware,
        ))
        .route_layer(middleware::from_fn(
            crate::auth::middleware::require_admin_middleware,
        ));

    // Merge user and admin routes
    let all_user_routes = user_routes.merge(admin_routes);

    // Activity routes (require authentication)
    let activity_routes = activity::create_activity_router()
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::auth_middleware,
        ));

    Router::new()
        .nest("/health", health_routes)
        .nest("/auth", auth_routes)
        .nest("/users", all_user_routes)
        .nest("/activities", activity_routes)
        .with_state(state)
}

/// Create the complete API router
pub fn create_app_router(state: AppState) -> Router<AppState> {
    create_api_routes(state)
}
