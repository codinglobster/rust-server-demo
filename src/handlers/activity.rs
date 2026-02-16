//! Activity log handlers

use crate::auth::middleware::AuthenticatedUser;
use crate::core::error::AppResult;
use crate::models::activity::{ActivityLogsResponse, CreateActivityLogRequest};
use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Activity list query parameters
#[derive(Debug, Deserialize, ToSchema)]
pub struct ActivityListQuery {
    /// Page number (default: 1)
    #[serde(default = "default_page")]
    pub page: u64,

    /// Items per page (default: 20, max: 100)
    #[serde(default = "default_per_page")]
    pub per_page: u64,
}

fn default_page() -> u64 {
    1
}

fn default_per_page() -> u64 {
    20
}

/// Create activity log
#[utoipa::path(
    post,
    path = "/api/activities",
    tag = "activities",
    request_body = CreateActivityLogRequest,
    responses(
        (status = 201, description = "Activity log created successfully", body = crate::models::activity::ActivityLogDto),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_activity(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Json(request): Json<CreateActivityLogRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let activity = state.activity_service.create_activity(&request).await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Activity logged",
        "data": activity
    })))
}

/// List all activity logs
#[utoipa::path(
    get,
    path = "/api/activities",
    tag = "activities",
    params(
        ("page" = u64, Query, description = "Page number"),
        ("per_page" = u64, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "Activity logs retrieved successfully", body = ActivityLogsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_activities(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Query(params): Query<ActivityListQuery>,
) -> AppResult<Json<ActivityLogsResponse>> {
    let per_page = params.per_page.min(100);
    let response = state.activity_service.list_activities(params.page, per_page).await?;

    Ok(Json(response))
}

/// Get recent activity logs
#[utoipa::path(
    get,
    path = "/api/activities/recent",
    tag = "activities",
    responses(
        (status = 200, description = "Recent activities retrieved successfully", body = Vec<crate::models::activity::ActivityLogDto>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_recent_activities(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
) -> AppResult<Json<Vec<crate::models::activity::ActivityLogDto>>> {
    let activities = state.activity_service.get_recent_activities(50).await?;

    Ok(Json(activities))
}

/// Get activity logs for a specific user
#[utoipa::path(
    get,
    path = "/api/activities/user/{user_id}",
    tag = "activities",
    params(
        ("user_id" = Uuid, Path, description = "User ID"),
        ("page" = u64, Query, description = "Page number"),
        ("per_page" = u64, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "User activities retrieved successfully", body = ActivityLogsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - can only view own activities unless admin"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_user_activities(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(user_id): Path<Uuid>,
    Query(params): Query<ActivityListQuery>,
) -> AppResult<Json<ActivityLogsResponse>> {
    // Users can only view their own activities unless they're admins
    if user.0.id != user_id && !user.0.is_admin() {
        return Err(crate::core::error::AppError::forbidden(
            "You can only view your own activity logs",
        ));
    }

    let per_page = params.per_page.min(100);
    let response = state
        .activity_service
        .list_user_activities(user_id, params.page, per_page)
        .await?;

    Ok(Json(response))
}

/// Create activity router
pub fn create_activity_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_activities))
        .route("/recent", get(get_recent_activities))
        .route("/user/:user_id", get(list_user_activities))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_page() {
        assert_eq!(default_page(), 1);
    }

    #[test]
    fn test_default_per_page() {
        assert_eq!(default_per_page(), 20);
    }
}
