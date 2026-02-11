//! Message service

use crate::cache::client::RedisClient;
use crate::cache::keys::CacheKeys;
use crate::core::error::AppResult;
use crate::models::message::{CreateMessageRequest, MessageDto, MessageType};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

/// Message service
#[derive(Clone)]
pub struct MessageService {
    pool: PgPool,
    redis: RedisClient,
}

impl MessageService {
    /// Create a new message service
    pub fn new(pool: PgPool, redis: RedisClient) -> Self {
        Self { pool, redis }
    }

    /// Create a new message
    pub async fn create_message(
        &self,
        sender_id: Uuid,
        request: &CreateMessageRequest,
    ) -> AppResult<MessageDto> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let message_type = request
            .message_type
            .as_ref()
            .and_then(|t| MessageType::from_str(t))
            .unwrap_or(MessageType::Text);

        let message = sqlx::query_as::<_, MessageDto>(
            r#"
            INSERT INTO messages (
                id, sender_id, content, message_type,
                room_id, recipient_id, is_edited, metadata,
                created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, sender_id, content, message_type,
                room_id, recipient_id, is_edited, metadata,
                created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(sender_id)
        .bind(&request.content)
        .bind(message_type.as_str())
        .bind(request.room_id)
        .bind(request.recipient_id)
        .bind(false)
        .bind(&request.metadata)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        // Cache recent messages for the room
        if let Some(room_id) = request.room_id {
            let cache_key = CacheKeys::messages(&room_id.to_string());
            let _ = self.redis.lpush(&cache_key, &serde_json::to_string(&message)?).await;
            // Keep only last 100 messages in cache
            let _ = self.redis.ltrim(&cache_key, 0, 99).await;
            // Set expiry to 1 hour
            let _ = self.redis.expire(&cache_key, 3600).await;
        }

        Ok(message)
    }

    /// Get a message by ID
    pub async fn get_message(&self, id: Uuid) -> AppResult<Option<MessageDto>> {
        let message = sqlx::query_as::<_, MessageDto>(
            r#"
            SELECT id, sender_id, content, message_type,
                room_id, recipient_id, is_edited, metadata,
                created_at, updated_at
            FROM messages WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(message)
    }

    /// Update a message
    pub async fn update_message(
        &self,
        id: Uuid,
        sender_id: Uuid,
        content: &str,
    ) -> AppResult<MessageDto> {
        let now = Utc::now();

        let message = sqlx::query_as::<_, MessageDto>(
            r#"
            UPDATE messages
            SET content = $3, is_edited = true, updated_at = $4
            WHERE id = $1 AND sender_id = $2
            RETURNING id, sender_id, content, message_type,
                room_id, recipient_id, is_edited, metadata,
                created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(sender_id)
        .bind(content)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(message)
    }

    /// Delete a message
    pub async fn delete_message(&self, id: Uuid, sender_id: Uuid) -> AppResult<()> {
        sqlx::query(
            r#"
            DELETE FROM messages
            WHERE id = $1 AND sender_id = $2
            "#,
        )
        .bind(id)
        .bind(sender_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// List messages for a room with pagination
    pub async fn list_room_messages(
        &self,
        room_id: Uuid,
        page: u64,
        per_page: u64,
    ) -> AppResult<(Vec<MessageDto>, i64)> {
        let offset = (page - 1) * per_page;

        let messages = sqlx::query_as::<_, MessageDto>(
            r#"
            SELECT id, sender_id, content, message_type,
                room_id, recipient_id, is_edited, metadata,
                created_at, updated_at
            FROM messages
            WHERE room_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(room_id)
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let total = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM messages WHERE room_id = $1
            "#,
        )
        .bind(room_id)
        .fetch_one(&self.pool)
        .await?;

        Ok((messages, total))
    }

    /// List messages sent by a user with pagination
    pub async fn list_user_messages(
        &self,
        sender_id: Uuid,
        page: u64,
        per_page: u64,
    ) -> AppResult<(Vec<MessageDto>, i64)> {
        let offset = (page - 1) * per_page;

        let messages = sqlx::query_as::<_, MessageDto>(
            r#"
            SELECT id, sender_id, content, message_type,
                room_id, recipient_id, is_edited, metadata,
                created_at, updated_at
            FROM messages
            WHERE sender_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(sender_id)
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let total = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM messages WHERE sender_id = $1
            "#,
        )
        .bind(sender_id)
        .fetch_one(&self.pool)
        .await?;

        Ok((messages, total))
    }

    /// Get recent messages from cache
    pub async fn get_recent_messages_from_cache(
        &self,
        room_id: Uuid,
        limit: usize,
    ) -> AppResult<Vec<MessageDto>> {
        let cache_key = CacheKeys::messages(&room_id.to_string());
        let cached = self.redis.lrange(&cache_key, 0, (limit as isize) - 1).await?;

        let mut messages = Vec::new();
        for msg_str in cached {
            if let Ok(msg) = serde_json::from_str::<MessageDto>(&msg_str) {
                messages.push(msg);
            }
        }

        Ok(messages)
    }

    /// Delete all messages in a room
    pub async fn delete_room_messages(&self, room_id: Uuid) -> AppResult<u64> {
        let result = sqlx::query(
            r#"
            DELETE FROM messages WHERE room_id = $1
            "#,
        )
        .bind(room_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
