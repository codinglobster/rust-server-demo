//! Room repository for database operations

use crate::core::error::AppResult;
use crate::models::room::{CreateRoomRequest, Room, RoomMember, UpdateRoomRequest};
use sqlx::PgPool;
use uuid::Uuid;

/// Room repository
#[derive(Clone)]
pub struct RoomRepository {
    pool: PgPool,
}

impl RoomRepository {
    /// Create new repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new room
    pub async fn create(&self, owner_id: Uuid, request: &CreateRoomRequest) -> AppResult<Room> {
        let room = sqlx::query_as::<_, Room>(
            r#"
            INSERT INTO rooms (name, description, owner_id, is_private, max_members)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(&request.name)
        .bind(&request.description)
        .bind(owner_id)
        .bind(request.is_private.unwrap_or(false))
        .bind(request.max_members)
        .fetch_one(&self.pool)
        .await?;

        // Add owner as member with owner role
        sqlx::query(
            r#"
            INSERT INTO room_members (room_id, user_id, role)
            VALUES ($1, $2, 'owner')
            "#,
        )
        .bind(room.id)
        .bind(owner_id)
        .execute(&self.pool)
        .await?;

        Ok(room)
    }

    /// Get room by ID
    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Room>> {
        let room = sqlx::query_as::<_, Room>(
            r#"
            SELECT * FROM rooms WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(room)
    }

    /// Update room
    pub async fn update(&self, id: Uuid, request: &UpdateRoomRequest) -> AppResult<Room> {
        let room = sqlx::query_as::<_, Room>(
            r#"
            UPDATE rooms
            SET name = COALESCE($2, name),
                description = COALESCE($3, description),
                is_private = COALESCE($4, is_private),
                max_members = COALESCE($5, max_members)
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&request.name)
        .bind(&request.description)
        .bind(request.is_private)
        .bind(request.max_members)
        .fetch_one(&self.pool)
        .await?;

        Ok(room)
    }

    /// Delete room
    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query(
            r#"
            DELETE FROM rooms WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// List all rooms with pagination
    pub async fn list(&self, page: u64, per_page: u64) -> AppResult<Vec<Room>> {
        let offset = (page - 1) * per_page;

        let rooms = sqlx::query_as::<_, Room>(
            r#"
            SELECT * FROM rooms
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rooms)
    }

    /// Count total rooms
    pub async fn count(&self) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM rooms
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    /// Get user's rooms
    pub async fn find_by_user(&self, user_id: Uuid, page: u64, per_page: u64) -> AppResult<Vec<Room>> {
        let offset = (page - 1) * per_page;

        let rooms = sqlx::query_as::<_, Room>(
            r#"
            SELECT r.* FROM rooms r
            INNER JOIN room_members rm ON r.id = rm.room_id
            WHERE rm.user_id = $1
            ORDER BY rm.joined_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rooms)
    }

    /// Count user's rooms
    pub async fn count_by_user(&self, user_id: Uuid) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM room_members
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    /// Add user to room
    pub async fn add_member(&self, room_id: Uuid, user_id: Uuid, role: &str) -> AppResult<RoomMember> {
        let member = sqlx::query_as::<_, RoomMember>(
            r#"
            INSERT INTO room_members (room_id, user_id, role)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(room_id)
        .bind(user_id)
        .bind(role)
        .fetch_one(&self.pool)
        .await?;

        Ok(member)
    }

    /// Remove user from room
    pub async fn remove_member(&self, room_id: Uuid, user_id: Uuid) -> AppResult<()> {
        sqlx::query(
            r#"
            DELETE FROM room_members
            WHERE room_id = $1 AND user_id = $2
            "#,
        )
        .bind(room_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get room member
    pub async fn find_member(&self, room_id: Uuid, user_id: Uuid) -> AppResult<Option<RoomMember>> {
        let member = sqlx::query_as::<_, RoomMember>(
            r#"
            SELECT * FROM room_members
            WHERE room_id = $1 AND user_id = $2
            "#,
        )
        .bind(room_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(member)
    }

    /// List room members
    pub async fn list_members(&self, room_id: Uuid) -> AppResult<Vec<RoomMember>> {
        let members = sqlx::query_as::<_, RoomMember>(
            r#"
            SELECT * FROM room_members
            WHERE room_id = $1
            ORDER BY joined_at ASC
            "#,
        )
        .bind(room_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(members)
    }

    /// Count room members
    pub async fn count_members(&self, room_id: Uuid) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM room_members
            WHERE room_id = $1
            "#,
        )
        .bind(room_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    /// Update member role
    pub async fn update_member_role(&self, room_id: Uuid, user_id: Uuid, role: &str) -> AppResult<RoomMember> {
        let member = sqlx::query_as::<_, RoomMember>(
            r#"
            UPDATE room_members
            SET role = $3
            WHERE room_id = $1 AND user_id = $2
            RETURNING *
            "#,
        )
        .bind(room_id)
        .bind(user_id)
        .bind(role)
        .fetch_one(&self.pool)
        .await?;

        Ok(member)
    }
}
