//! User handlers

use crate::auth::middleware::AuthenticatedUser;
use crate::models::user::{UserDto, UpdateUserRequest, ChangePasswordRequest};
use crate::services::UserService;
use axum::{
    extract::{Path as AxumPath, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    Json as JsonExtractor,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use validator::Validate;

/// Get current user profile
#[utoipa::path(
    get,
    path = "/api/users/me",
    responses(
        (status = 200, description = "User profile retrieved", body = UserDto),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Users",
    security(("bearer_auth" = []))
)]
pub async fn get_me(
    State(user_service): State<UserService>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match user_service.get_user(user.id).await {
        Ok(user_dto) => Ok((StatusCode::OK, Json(user_dto))),
        Err(e) => Err((
            e.status_code(),
            Json(serde_json::json!({ "error": e.to_string() })),
        )),
    }
}

/// Get user by ID
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User retrieved", body = UserDto),
        (status = 404, description = "User not found")
    ),
    tag = "Users",
    security(("bearer_auth" = []))
)]
pub async fn get_user(
    State(user_service): State<UserService>,
    AxumPath(id): AxumPath<Uuid>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match user_service.get_user(id).await {
        Ok(user_dto) => Ok((StatusCode::OK, Json(user_dto))),
        Err(e) => Err((
            e.status_code(),
            Json(serde_json::json!({ "error": e.to_string() })),
        )),
    }
}

/// List users with pagination
#[utoipa::path(
    get,
    path = "/api/users",
    params(
        ("page" = Option<u64>, Query, description = "Page number"),
        ("per_page" = Option<u64>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "Users list", body = UsersListResponse)
    ),
    tag = "Users",
    security(("bearer_auth" = []))
)]
pub async fn list_users(
    State(user_service): State<UserService>,
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let (users, total) = user_service
        .list_users(params.page.unwrap_or(1), params.per_page.unwrap_or(20))
        .await
        .unwrap_or_else(|_| (vec![], 0));

    let response = UsersListResponse {
        users,
        total,
        page: params.page.unwrap_or(1),
        per_page: params.per_page.unwrap_or(20),
    };

    Ok::<_, crate::core::error::AppError>((StatusCode::OK, Json(response)))
}

/// Update user profile
#[utoipa::path(
    put,
    path = "/api/users/me",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = UserDto),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Users",
    security(("bearer_auth" = []))
)]
pub async fn update_user(
    State(user_service): State<UserService>,
    AuthenticatedUser(user): AuthenticatedUser,
    JsonExtractor(request): JsonExtractor<UpdateUserRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match user_service.update_user(user.id, &request).await {
        Ok(user_dto) => Ok((StatusCode::OK, Json(user_dto))),
        Err(e) => Err((
            e.status_code(),
            Json(serde_json::json!({ "error": e.to_string() })),
        )),
    }
}

/// Change user password
#[utoipa::path(
    post,
    path = "/api/users/me/password",
    request_body = ChangePasswordRequest,
    responses(
        (status = 204, description = "Password changed"),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Users",
    security(("bearer_auth" = []))
)]
pub async fn change_password(
    State(_user_service): State<UserService>,
    State(auth_service): State<crate::services::AuthService>,
    AuthenticatedUser(_user): AuthenticatedUser,
    JsonExtractor(request): JsonExtractor<ChangePasswordRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    auth_service.change_password(&request).await?;

    Ok::<_, crate::core::error::AppError>(StatusCode::NO_CONTENT)
}

/// Pagination parameters
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PaginationParams {
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 100))]
    pub per_page: Option<u64>,
}

/// Users list response
#[derive(Debug, Serialize, ToSchema)]
pub struct UsersListResponse {
    pub users: Vec<UserDto>,
    pub total: i64,
    pub page: u64,
    pub per_page: u64,
}
