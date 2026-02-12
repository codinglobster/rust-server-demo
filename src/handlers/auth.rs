//! Authentication handlers

use crate::auth::jwt::TokenPair;
use crate::auth::middleware::AuthenticatedUser;
use crate::models::user::{LoginRequest, RegisterRequest};
use crate::models::session::{LoginResponse, RefreshTokenRequest};
use crate::services::AuthService;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json},
    Json as JsonExtractor,
};
use serde::Serialize;
use utoipa::ToSchema;

/// Error response
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = LoginResponse),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 409, description = "Username or email already exists", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn register(
    State(auth_service): State<AuthService>,
    Json(request): JsonExtractor<RegisterRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match auth_service.register(&request).await {
        Ok(response) => Ok((StatusCode::CREATED, Json(response))),
        Err(e) => Err((
            match e.status_code() {
                StatusCode::CONFLICT => StatusCode::CONFLICT,
                StatusCode::BAD_REQUEST => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

/// Login a user
#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn login(
    State(auth_service): State<AuthService>,
    Json(request): JsonExtractor<LoginRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match auth_service.login(&request, None).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(e) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

/// Refresh access token
#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = TokenPair),
        (status = 401, description = "Invalid refresh token", body = ErrorResponse)
    ),
    tag = "Authentication"
)]
pub async fn refresh_token(
    State(auth_service): State<AuthService>,
    Json(request): JsonExtractor<RefreshTokenRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match auth_service.refresh_token(&request.refresh_token).await {
        Ok(token_pair) => Ok((StatusCode::OK, Json(token_pair))),
        Err(e) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

/// Logout current user
#[utoipa::path(
    post,
    path = "/api/auth/logout",
    responses(
        (status = 204, description = "Logout successful"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Authentication",
    security(("bearer_auth" = []))
)]
pub async fn logout(
    State(auth_service): State<AuthService>,
    AuthenticatedUser(user): AuthenticatedUser,
    headers: HeaderMap,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // Extract token from Authorization header
    let auth_header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Missing authorization header".to_string(),
            }),
        ))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid authorization header format".to_string(),
            }),
        ))?;

    // Logout user
    match auth_service.logout(token, user.id).await {
        Ok(_) => Ok::<_, (StatusCode, Json<ErrorResponse>)>(StatusCode::NO_CONTENT),
        Err(e) => Err((
            e.status_code(),
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}
