//! Session management handlers

use crate::auth::middleware::AuthenticatedUser;
use crate::core::error::{AppError, AppResult};
use crate::services::session_service::SessionService;
use crate::models::session::SessionDto;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// List user's active sessions response
#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SessionsListResponse {
    pub sessions: Vec<SessionDto>,
    pub total: usize,
}

/// Session invalidation response
#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SessionInvalidateResponse {
    pub message: String,
    pub invalidated_count: u64,
}

/// List all active sessions for current user
#[utoipa::path(
    get,
    path = "/api/sessions",
    tag = "Sessions",
    security(("bearer" = [])),
    responses(
        (status = 200, description = "Sessions retrieved successfully", body = SessionsListResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_sessions(
    State(session_service): State<SessionService>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> AppResult<Json<SessionsListResponse>> {
    let sessions = session_service.list_user_sessions(user.id).await?;
    let total = sessions.len();

    Ok(Json(SessionsListResponse { sessions, total }))
}

/// Get specific session details
#[utoipa::path(
    get,
    path = "/api/sessions/{id}",
    tag = "Sessions",
    security(("bearer" = [])),
    params(
        ("id" = Uuid, Path, description = "Session ID")
    ),
    responses(
        (status = 200, description = "Session found", body = SessionDto),
        (status = 404, description = "Session not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - session belongs to another user")
    )
)]
pub async fn get_session(
    State(session_service): State<SessionService>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<SessionDto>> {
    let session = session_service
        .get_session(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Session not found".to_string()))?;

    // Verify session belongs to current user
    if session.user_id != user.id {
        return Err(AppError::Auth(crate::core::error::AuthError::InsufficientPermissions));
    }

    Ok(Json(session))
}

/// Invalidate (logout) a specific session
#[utoipa::path(
    delete,
    path = "/api/sessions/{id}",
    tag = "Sessions",
    security(("bearer" = [])),
    params(
        ("id" = Uuid, Path, description = "Session ID to invalidate")
    ),
    responses(
        (status = 200, description = "Session invalidated successfully"),
        (status = 404, description = "Session not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - session belongs to another user")
    )
)]
pub async fn invalidate_session(
    State(session_service): State<SessionService>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    // Verify session exists and belongs to current user
    let session = session_service
        .get_session(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Session not found".to_string()))?;

    if session.user_id != user.id {
        return Err(AppError::Auth(crate::core::error::AuthError::InsufficientPermissions));
    }

    // Invalidate session
    session_service.invalidate_session(id).await?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Session invalidated successfully"
        })),
    ))
}

/// Invalidate all other sessions (keep current)
#[utoipa::path(
    delete,
    path = "/api/sessions/other",
    tag = "Sessions",
    security(("bearer" = [])),
    responses(
        (status = 200, description = "Other sessions invalidated", body = SessionInvalidateResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn invalidate_other_sessions(
    State(session_service): State<SessionService>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> AppResult<Json<SessionInvalidateResponse>> {
    // Invalidate all user sessions
    let invalidated_count = session_service
        .invalidate_all_user_sessions(user.id)
        .await?;

    Ok(Json(SessionInvalidateResponse {
        message: format!("Invalidated {} session(s)", invalidated_count),
        invalidated_count,
    }))
}

/// Get session statistics for current user
#[utoipa::path(
    get,
    path = "/api/sessions/stats",
    tag = "Sessions",
    security(("bearer" = [])),
    responses(
        (status = 200, description = "Session statistics"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_session_stats(
    State(session_service): State<SessionService>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> AppResult<Json<serde_json::Value>> {
    let count = session_service.count_user_sessions(user.id).await?;

    Ok(Json(serde_json::json!({
        "user_id": user.id,
        "active_sessions": count
    })))
}
