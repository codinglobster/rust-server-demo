//! Room handlers

use crate::auth::middleware::AuthenticatedUser;
use crate::core::error::AppResult;
use crate::models::room::{
    CreateRoomRequest, JoinRoomRequest, RoomDto, RoomMemberDto, RoomMembersResponse,
    RoomsListResponse, UpdateMemberRoleRequest, UpdateRoomRequest,
};
use crate::services::RoomService;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use uuid::Uuid;

/// Pagination parameters
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

/// Create a new room
#[utoipa::path(
    post,
    path = "/api/rooms",
    tags = ["Rooms"],
    request_body = CreateRoomRequest,
    responses(
        (status = 201, description = "Room created successfully", body = RoomDto),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_room(
    State(room_service): State<RoomService>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(request): Json<CreateRoomRequest>,
) -> AppResult<(StatusCode, Json<RoomDto>)> {
    let room = room_service.create_room(user.id, request).await?;
    Ok((StatusCode::CREATED, Json(room)))
}

/// Get room by ID
#[utoipa::path(
    get,
    path = "/api/rooms/{id}",
    tags = ["Rooms"],
    params(
        ("id" = Uuid, Path, description = "Room ID")
    ),
    responses(
        (status = 200, description = "Room found", body = RoomDto),
        (status = 404, description = "Room not found"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_room(
    State(room_service): State<RoomService>,
    AuthenticatedUser(_user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<RoomDto>> {
    let room = room_service.get_room(id).await?;
    Ok(Json(room))
}

/// Update room
#[utoipa::path(
    put,
    path = "/api/rooms/{id}",
    tags = ["Rooms"],
    params(
        ("id" = Uuid, Path, description = "Room ID")
    ),
    request_body = UpdateRoomRequest,
    responses(
        (status = 200, description = "Room updated successfully", body = RoomDto),
        (status = 400, description = "Invalid request"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Room not found"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_room(
    State(room_service): State<RoomService>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateRoomRequest>,
) -> AppResult<Json<RoomDto>> {
    let room = room_service.update_room(id, user.id, request).await?;
    Ok(Json(room))
}

/// Delete room
#[utoipa::path(
    delete,
    path = "/api/rooms/{id}",
    tags = ["Rooms"],
    params(
        ("id" = Uuid, Path, description = "Room ID")
    ),
    responses(
        (status = 204, description = "Room deleted successfully"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Room not found"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_room(
    State(room_service): State<RoomService>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    room_service.delete_room(id, user.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// List all rooms
#[utoipa::path(
    get,
    path = "/api/rooms",
    tags = ["Rooms"],
    params(
        ("page" = Option<u64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<u64>, Query, description = "Items per page (default: 20)")
    ),
    responses(
        (status = 200, description = "List of rooms", body = RoomsListResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_rooms(
    State(room_service): State<RoomService>,
    AuthenticatedUser(_user): AuthenticatedUser,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<RoomsListResponse>> {
    let (rooms, total) = room_service.list_rooms(params.page, params.per_page).await?;

    Ok(Json(RoomsListResponse {
        rooms,
        total: total as u64,
        page: params.page,
        per_page: params.per_page,
    }))
}

/// Get user's rooms
#[utoipa::path(
    get,
    path = "/api/rooms/me",
    tags = ["Rooms"],
    params(
        ("page" = Option<u64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<u64>, Query, description = "Items per page (default: 20)")
    ),
    responses(
        (status = 200, description = "List of user's rooms", body = RoomsListResponse),
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_my_rooms(
    State(room_service): State<RoomService>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<RoomsListResponse>> {
    let (rooms, total) = room_service
        .get_user_rooms(user.id, params.page, params.per_page)
        .await?;

    Ok(Json(RoomsListResponse {
        rooms,
        total: total as u64,
        page: params.page,
        per_page: params.per_page,
    }))
}

/// Join a room
#[utoipa::path(
    post,
    path = "/api/rooms/{id}/join",
    tags = ["Rooms"],
    params(
        ("id" = Uuid, Path, description = "Room ID")
    ),
    request_body = JoinRoomRequest,
    responses(
        (status = 200, description = "Joined room successfully", body = RoomMemberDto),
        (status = 400, description = "Already a member or room is full"),
        (status = 404, description = "Room not found"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn join_room(
    State(room_service): State<RoomService>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(_request): Json<JoinRoomRequest>,
) -> AppResult<Json<RoomMemberDto>> {
    let member = room_service.join_room(id, user.id).await?;
    Ok(Json(member))
}

/// Leave a room
#[utoipa::path(
    post,
    path = "/api/rooms/{id}/leave",
    tags = ["Rooms"],
    params(
        ("id" = Uuid, Path, description = "Room ID")
    ),
    responses(
        (status = 204, description = "Left room successfully"),
        (status = 400, description = "Owner cannot leave"),
        (status = 404, description = "Not a member"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn leave_room(
    State(room_service): State<RoomService>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    room_service.leave_room(id, user.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// List room members
#[utoipa::path(
    get,
    path = "/api/rooms/{id}/members",
    tags = ["Rooms"],
    params(
        ("id" = Uuid, Path, description = "Room ID")
    ),
    responses(
        (status = 200, description = "List of room members", body = RoomMembersResponse),
        (status = 403, description = "Private room - must be member"),
        (status = 404, description = "Room not found"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_members(
    State(room_service): State<RoomService>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<RoomMembersResponse>> {
    let members = room_service.list_members(id, user.id).await?;
    let total = members.len() as u64;

    Ok(Json(RoomMembersResponse { members, total }))
}

/// Update member role
#[utoipa::path(
    put,
    path = "/api/rooms/{id}/members/{user_id}/role",
    tags = ["Rooms"],
    params(
        ("id" = Uuid, Path, description = "Room ID"),
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateMemberRoleRequest,
    responses(
        (status = 200, description = "Role updated successfully", body = RoomMemberDto),
        (status = 400, description = "Invalid role or cannot change owner"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Member not found"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_member_role(
    State(room_service): State<RoomService>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((id, user_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdateMemberRoleRequest>,
) -> AppResult<Json<RoomMemberDto>> {
    let member = room_service
        .update_member_role(id, user_id, user.id, request)
        .await?;
    Ok(Json(member))
}

/// Remove member from room
#[utoipa::path(
    delete,
    path = "/api/rooms/{id}/members/{user_id}",
    tags = ["Rooms"],
    params(
        ("id" = Uuid, Path, description = "Room ID"),
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 204, description = "Member removed successfully"),
        (status = 400, description = "Cannot remove owner"),
        (status = 403, description = "Insufficient permissions"),
        (status = 404, description = "Member not found"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn remove_member(
    State(room_service): State<RoomService>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((id, user_id)): Path<(Uuid, Uuid)>,
) -> AppResult<StatusCode> {
    room_service.remove_member(id, user_id, user.id).await?;
    Ok(StatusCode::NO_CONTENT)
}
