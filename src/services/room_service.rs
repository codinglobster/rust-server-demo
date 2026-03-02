//! Room service for business logic

use crate::cache::{keys::CacheKeys, RedisClient};
use crate::core::error::{AppError, AppResult, AuthError};
use crate::database::repositories::room::RoomRepository;
use crate::models::room::{
    CreateRoomRequest, Room, RoomDto, RoomMemberDto, RoomRole, UpdateMemberRoleRequest,
    UpdateRoomRequest,
};
use uuid::Uuid;

/// Room service
#[derive(Clone)]
pub struct RoomService {
    repository: RoomRepository,
    redis: RedisClient,
}

impl RoomService {
    /// Create new service
    pub fn new(repository: RoomRepository, redis: RedisClient) -> Self {
        Self { repository, redis }
    }

    /// Create a new room
    pub async fn create_room(&self, owner_id: Uuid, request: CreateRoomRequest) -> AppResult<RoomDto> {
        let room = self.repository.create(owner_id, &request).await?;

        // Cache the room
        let cache_key = CacheKeys::room(room.id);
        let _ = self.redis.set(&cache_key, &room, Some(3600)).await;

        Ok(room.into())
    }

    /// Get room by ID
    pub async fn get_room(&self, id: Uuid) -> AppResult<RoomDto> {
        // Try cache first
        let cache_key = CacheKeys::room(id);
        if let Ok(Some(room)) = self.redis.get::<Room>(&cache_key).await {
            let mut dto: RoomDto = room.into();
            // Get member count
            if let Ok(count) = self.repository.count_members(id).await {
                dto.member_count = Some(count);
            }
            return Ok(dto);
        }

        // Fallback to database
        let room = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(AppError::NotFound("Room not found".to_string()))?;

        // Cache for next time
        let _ = self.redis.set(&cache_key, &room, Some(3600)).await;

        let mut dto: RoomDto = room.into();
        // Get member count
        if let Ok(count) = self.repository.count_members(id).await {
            dto.member_count = Some(count);
        }

        Ok(dto)
    }

    /// Update room
    pub async fn update_room(
        &self,
        id: Uuid,
        user_id: Uuid,
        request: UpdateRoomRequest,
    ) -> AppResult<RoomDto> {
        // Check permission - must be owner or admin
        self.check_room_permission(id, user_id, vec![RoomRole::Owner, RoomRole::Admin])
            .await?;

        let room = self.repository.update(id, &request).await?;

        // Invalidate cache
        let cache_key = CacheKeys::room(id);
        let _ = self.redis.del(&cache_key).await;

        // Delayed double delete
        let redis_clone = self.redis.clone();
        let cache_key_clone = cache_key.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let _ = redis_clone.del(&cache_key_clone).await;
        });

        Ok(room.into())
    }

    /// Delete room
    pub async fn delete_room(&self, id: Uuid, user_id: Uuid) -> AppResult<()> {
        // Check permission - must be owner
        self.check_room_permission(id, user_id, vec![RoomRole::Owner])
            .await?;

        self.repository.delete(id).await?;

        // Invalidate cache
        let cache_key = CacheKeys::room(id);
        let _ = self.redis.del(&cache_key).await;

        Ok(())
    }

    /// List all rooms with pagination
    pub async fn list_rooms(&self, page: u64, per_page: u64) -> AppResult<(Vec<RoomDto>, i64)> {
        let rooms = self.repository.list(page, per_page).await?;
        let total = self.repository.count().await?;

        let mut room_dtos = Vec::new();
        for room in rooms {
            let mut dto: RoomDto = room.clone().into();
            // Get member count
            if let Ok(count) = self.repository.count_members(room.id).await {
                dto.member_count = Some(count);
            }
            room_dtos.push(dto);
        }

        Ok((room_dtos, total))
    }

    /// Get user's rooms
    pub async fn get_user_rooms(&self, user_id: Uuid, page: u64, per_page: u64) -> AppResult<(Vec<RoomDto>, i64)> {
        let rooms = self.repository.find_by_user(user_id, page, per_page).await?;
        let total = self.repository.count_by_user(user_id).await?;

        let mut room_dtos = Vec::new();
        for room in rooms {
            let mut dto: RoomDto = room.clone().into();
            // Get member count
            if let Ok(count) = self.repository.count_members(room.id).await {
                dto.member_count = Some(count);
            }
            room_dtos.push(dto);
        }

        Ok((room_dtos, total))
    }

    /// Join a room
    pub async fn join_room(&self, room_id: Uuid, user_id: Uuid) -> AppResult<RoomMemberDto> {
        // Check if room exists
        let room = self
            .repository
            .find_by_id(room_id)
            .await?
            .ok_or(AppError::NotFound("Room not found".to_string()))?;

        // Check if already a member
        if let Some(_) = self.repository.find_member(room_id, user_id).await? {
            return Err(AppError::BadRequest("Already a member of this room".to_string()));
        }

        // Check max members
        if let Some(max) = room.max_members {
            let count = self.repository.count_members(room_id).await?;
            if count >= max as i64 {
                return Err(AppError::BadRequest("Room is full".to_string()));
            }
        }

        // Add as member
        let member = self
            .repository
            .add_member(room_id, user_id, RoomRole::Member.as_str())
            .await?;

        Ok(member.into())
    }

    /// Leave a room
    pub async fn leave_room(&self, room_id: Uuid, user_id: Uuid) -> AppResult<()> {
        // Check if member exists
        let member = self
            .repository
            .find_member(room_id, user_id)
            .await?
            .ok_or(AppError::NotFound("Not a member of this room".to_string()))?;

        // Owner cannot leave - must delete room instead
        if member.role == RoomRole::Owner.as_str() {
            return Err(AppError::BadRequest(
                "Room owner cannot leave. Delete the room instead.".to_string(),
            ));
        }

        self.repository.remove_member(room_id, user_id).await?;

        Ok(())
    }

    /// List room members
    pub async fn list_members(&self, room_id: Uuid, user_id: Uuid) -> AppResult<Vec<RoomMemberDto>> {
        // Check if user is a member or room is public
        let room = self
            .repository
            .find_by_id(room_id)
            .await?
            .ok_or(AppError::NotFound("Room not found".to_string()))?;

        if room.is_private {
            // Must be a member to see members list
            self.repository
                .find_member(room_id, user_id)
                .await?
                .ok_or(AppError::Auth(AuthError::InsufficientPermissions))?;
        }

        let members = self.repository.list_members(room_id).await?;
        Ok(members.into_iter().map(|m| m.into()).collect())
    }

    /// Update member role
    pub async fn update_member_role(
        &self,
        room_id: Uuid,
        target_user_id: Uuid,
        current_user_id: Uuid,
        request: UpdateMemberRoleRequest,
    ) -> AppResult<RoomMemberDto> {
        // Check permission - must be owner or admin
        self.check_room_permission(room_id, current_user_id, vec![RoomRole::Owner, RoomRole::Admin])
            .await?;

        // Validate role
        let role = request
            .role
            .parse::<RoomRole>()
            .map_err(|_| AppError::BadRequest("Invalid role".to_string()))?;

        // Cannot change owner role
        let target_member = self
            .repository
            .find_member(room_id, target_user_id)
            .await?
            .ok_or(AppError::NotFound("Member not found".to_string()))?;

        if target_member.role == RoomRole::Owner.as_str() {
            return Err(AppError::BadRequest("Cannot change owner role".to_string()));
        }

        let member = self
            .repository
            .update_member_role(room_id, target_user_id, role.as_str())
            .await?;

        Ok(member.into())
    }

    /// Remove member from room
    pub async fn remove_member(
        &self,
        room_id: Uuid,
        target_user_id: Uuid,
        current_user_id: Uuid,
    ) -> AppResult<()> {
        // Check permission - must be owner or admin
        self.check_room_permission(room_id, current_user_id, vec![RoomRole::Owner, RoomRole::Admin])
            .await?;

        // Cannot remove owner
        let target_member = self
            .repository
            .find_member(room_id, target_user_id)
            .await?
            .ok_or(AppError::NotFound("Member not found".to_string()))?;

        if target_member.role == RoomRole::Owner.as_str() {
            return Err(AppError::BadRequest("Cannot remove owner".to_string()));
        }

        self.repository.remove_member(room_id, target_user_id).await?;

        Ok(())
    }

    /// Check if user has permission in room
    async fn check_room_permission(
        &self,
        room_id: Uuid,
        user_id: Uuid,
        allowed_roles: Vec<RoomRole>,
    ) -> AppResult<()> {
        let member = self
            .repository
            .find_member(room_id, user_id)
            .await?
            .ok_or(AppError::Auth(AuthError::InsufficientPermissions))?;

        let member_role = member
            .role
            .parse::<RoomRole>()
            .map_err(|_| AppError::Auth(AuthError::InsufficientPermissions))?;

        if !allowed_roles.contains(&member_role) {
            return Err(AppError::Auth(AuthError::InsufficientPermissions));
        }

        Ok(())
    }

    /// Check if user is member of room
    pub async fn is_member(&self, room_id: Uuid, user_id: Uuid) -> AppResult<bool> {
        let member = self.repository.find_member(room_id, user_id).await?;
        Ok(member.is_some())
    }
}
