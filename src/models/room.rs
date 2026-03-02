//! Room model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Room model from database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Uuid,
    pub is_private: bool,
    pub max_members: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Room member model from database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RoomMember {
    pub id: Uuid,
    pub room_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}

/// Room member role enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RoomRole {
    Owner,
    Admin,
    Member,
}

impl RoomRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            RoomRole::Owner => "owner",
            RoomRole::Admin => "admin",
            RoomRole::Member => "member",
        }
    }
}

impl std::str::FromStr for RoomRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "owner" => Ok(RoomRole::Owner),
            "admin" => Ok(RoomRole::Admin),
            "member" => Ok(RoomRole::Member),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for RoomRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Create room request
#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct CreateRoomRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 5000))]
    pub description: Option<String>,
    pub is_private: Option<bool>,
    pub max_members: Option<i32>,
}

/// Update room request
#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct UpdateRoomRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 5000))]
    pub description: Option<String>,
    pub is_private: Option<bool>,
    pub max_members: Option<i32>,
}

/// Room DTO for API responses
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct RoomDto {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Uuid,
    pub is_private: bool,
    pub max_members: Option<i32>,
    pub member_count: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Room> for RoomDto {
    fn from(room: Room) -> Self {
        Self {
            id: room.id,
            name: room.name,
            description: room.description,
            owner_id: room.owner_id,
            is_private: room.is_private,
            max_members: room.max_members,
            member_count: None,
            created_at: room.created_at,
            updated_at: room.updated_at,
        }
    }
}

/// Room member DTO for API responses
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct RoomMemberDto {
    pub id: Uuid,
    pub room_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}

impl From<RoomMember> for RoomMemberDto {
    fn from(member: RoomMember) -> Self {
        Self {
            id: member.id,
            room_id: member.room_id,
            user_id: member.user_id,
            role: member.role,
            joined_at: member.joined_at,
        }
    }
}

/// Join room request
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct JoinRoomRequest {
    pub password: Option<String>,
}

/// Update member role request
#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct UpdateMemberRoleRequest {
    pub role: String,
}

/// Rooms list response with pagination
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct RoomsListResponse {
    pub rooms: Vec<RoomDto>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}

/// Room members list response
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct RoomMembersResponse {
    pub members: Vec<RoomMemberDto>,
    pub total: u64,
}
