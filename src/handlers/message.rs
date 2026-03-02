//! Message handlers

use crate::auth::middleware::AuthenticatedUser;
use crate::core::error::{AppError, AppResult};
use crate::models::message::{CreateMessageRequest, MessageDto, MessagesResponse, UpdateMessageRequest};
use crate::services::message_service::MessageService;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Pagination query parameters
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_per_page")]
    pub per_page: u64,
}

fn default_page() -> u64 {
    1
}

fn default_per_page() -> u64 {
    20
}

/// Room query parameter
#[derive(Debug, Deserialize)]
pub struct RoomQueryParams {
    pub room_id: Option<Uuid>,
}

/// Create a new message
#[utoipa::path(
    post,
    path = "/api/messages",
    tag = "Messages",
    security(("bearer" = [])),
    request_body = CreateMessageRequest,
    responses(
        (status = 201, description = "Message created successfully", body = MessageDto),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn create_message(
    State(message_service): State<MessageService>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(request): Json<CreateMessageRequest>,
) -> AppResult<(StatusCode, Json<MessageDto>)> {
    // Validate that message has either room_id or recipient_id
    if request.room_id.is_none() && request.recipient_id.is_none() {
        return Err(AppError::BadRequest(
            "Message must have either room_id or recipient_id".to_string(),
        ));
    }

    let message = message_service.create_message(user.id, &request).await?;

    Ok((StatusCode::CREATED, Json(message)))
}

/// Get a specific message
#[utoipa::path(
    get,
    path = "/api/messages/{id}",
    tag = "Messages",
    security(("bearer" = [])),
    params(
        ("id" = Uuid, Path, description = "Message ID")
    ),
    responses(
        (status = 200, description = "Message found", body = MessageDto),
        (status = 404, description = "Message not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_message(
    State(message_service): State<MessageService>,
    AuthenticatedUser(_user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<MessageDto>> {
    let message = message_service
        .get_message(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Message not found".to_string()))?;

    Ok(Json(message))
}

/// Update a message (only sender can update)
#[utoipa::path(
    put,
    path = "/api/messages/{id}",
    tag = "Messages",
    security(("bearer" = [])),
    params(
        ("id" = Uuid, Path, description = "Message ID")
    ),
    request_body = UpdateMessageRequest,
    responses(
        (status = 200, description = "Message updated successfully", body = MessageDto),
        (status = 404, description = "Message not found or not authorized"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn update_message(
    State(message_service): State<MessageService>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateMessageRequest>,
) -> AppResult<Json<MessageDto>> {
    // Verify message exists and belongs to user
    let existing = message_service
        .get_message(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Message not found".to_string()))?;

    if existing.sender_id != user.id {
        return Err(AppError::Auth(
            crate::core::error::AuthError::InsufficientPermissions,
        ));
    }

    let message = message_service
        .update_message(id, user.id, &request.content)
        .await?;

    Ok(Json(message))
}

/// Delete a message (only sender can delete)
#[utoipa::path(
    delete,
    path = "/api/messages/{id}",
    tag = "Messages",
    security(("bearer" = [])),
    params(
        ("id" = Uuid, Path, description = "Message ID")
    ),
    responses(
        (status = 200, description = "Message deleted successfully"),
        (status = 404, description = "Message not found or not authorized"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn delete_message(
    State(message_service): State<MessageService>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    // Verify message exists and belongs to user
    let existing = message_service
        .get_message(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Message not found".to_string()))?;

    if existing.sender_id != user.id {
        return Err(AppError::Auth(
            crate::core::error::AuthError::InsufficientPermissions,
        ));
    }

    message_service.delete_message(id, user.id).await?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Message deleted successfully"
        })),
    ))
}

/// List messages with pagination (room or user messages)
#[utoipa::path(
    get,
    path = "/api/messages",
    tag = "Messages",
    security(("bearer" = [])),
    params(
        ("room_id" = Option<Uuid>, Query, description = "Filter by room ID"),
        ("page" = Option<u64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<u64>, Query, description = "Items per page (default: 20)")
    ),
    responses(
        (status = 200, description = "Messages retrieved successfully", body = MessagesResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_messages(
    State(message_service): State<MessageService>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(room_params): Query<RoomQueryParams>,
    Query(pagination): Query<PaginationParams>,
) -> AppResult<Json<MessagesResponse>> {
    let (messages, total) = if let Some(room_id) = room_params.room_id {
        // List room messages
        message_service
            .list_room_messages(room_id, pagination.page, pagination.per_page)
            .await?
    } else {
        // List user's own messages
        message_service
            .list_user_messages(user.id, pagination.page, pagination.per_page)
            .await?
    };

    Ok(Json(MessagesResponse {
        messages,
        total: total as u64,
        page: pagination.page,
        per_page: pagination.per_page,
    }))
}

/// Get recent messages for a room
#[utoipa::path(
    get,
    path = "/api/messages/room/{room_id}/recent",
    tag = "Messages",
    security(("bearer" = [])),
    params(
        ("room_id" = Uuid, Path, description = "Room ID"),
        ("limit" = Option<u32>, Query, description = "Number of messages to retrieve (default: 50)")
    ),
    responses(
        (status = 200, description = "Recent messages retrieved", body = Vec<MessageDto>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_recent_room_messages(
    State(message_service): State<MessageService>,
    AuthenticatedUser(_user): AuthenticatedUser,
    Path(room_id): Path<Uuid>,
    Query(limit_param): Query<LimitParam>,
) -> AppResult<Json<Vec<MessageDto>>> {
    let limit = limit_param.limit.unwrap_or(50).min(100) as usize;

    // Try cache first
    let cached_messages = message_service
        .get_recent_messages_from_cache(room_id, limit)
        .await?;

    if !cached_messages.is_empty() {
        return Ok(Json(cached_messages));
    }

    // Fall back to database
    let (messages, _) = message_service.list_room_messages(room_id, 1, limit as u64).await?;

    Ok(Json(messages))
}

/// Limit query parameter
#[derive(Debug, Deserialize)]
pub struct LimitParam {
    pub limit: Option<u32>,
}

/// Get user's sent messages
#[utoipa::path(
    get,
    path = "/api/messages/me",
    tag = "Messages",
    security(("bearer" = [])),
    params(
        ("page" = Option<u64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<u64>, Query, description = "Items per page (default: 20)")
    ),
    responses(
        (status = 200, description = "User messages retrieved", body = MessagesResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_my_messages(
    State(message_service): State<MessageService>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(pagination): Query<PaginationParams>,
) -> AppResult<Json<MessagesResponse>> {
    let (messages, total) = message_service
        .list_user_messages(user.id, pagination.page, pagination.per_page)
        .await?;

    Ok(Json(MessagesResponse {
        messages,
        total: total as u64,
        page: pagination.page,
        per_page: pagination.per_page,
    }))
}
