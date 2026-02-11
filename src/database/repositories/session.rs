//! Session repository

use crate::core::error::AppResult;
use crate::models::session::{Session, SessionCreate};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

/// Session repository for database operations
#[derive(Clone)]
pub struct SessionRepository {
    pool: PgPool,
}

impl SessionRepository {
    /// Create a new session repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Find a session by ID
    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Session>> {
        let session = sqlx::query_as::<_, Session>(
            r#"
            SELECT * FROM sessions WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    /// Find an active session by token hash
    pub async fn find_by_token_hash(&self, token_hash: &str) -> AppResult<Option<Session>> {
        let session = sqlx::query_as::<_, Session>(
            r#"
            SELECT * FROM sessions
            WHERE token_hash = $1 AND is_active = true
            "#,
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    /// Find an active session by refresh token hash
    pub async fn find_by_refresh_token_hash(&self, refresh_token_hash: &str) -> AppResult<Option<Session>> {
        let session = sqlx::query_as::<_, Session>(
            r#"
            SELECT * FROM sessions
            WHERE refresh_token_hash = $1 AND is_active = true
            "#,
        )
        .bind(refresh_token_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    /// Create a new session
    pub async fn create(&self, session: &SessionCreate) -> AppResult<Session> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let new_session = sqlx::query_as::<_, Session>(
            r#"
            INSERT INTO sessions (
                id, user_id, token_hash, refresh_token_hash,
                ip_address, user_agent, is_active, expires_at,
                created_at, last_accessed_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(session.user_id)
        .bind(&session.token_hash)
        .bind(&session.refresh_token_hash)
        .bind(&session.ip_address)
        .bind(&session.user_agent)
        .bind(true)
        .bind(session.expires_at)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(new_session)
    }

    /// Update last accessed timestamp
    pub async fn update_last_accessed(&self, id: Uuid) -> AppResult<()> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE sessions
            SET last_accessed_at = $2
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Invalidate a session (set is_active = false)
    pub async fn invalidate(&self, id: Uuid) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE sessions
            SET is_active = false
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Invalidate all sessions for a user
    pub async fn invalidate_all_for_user(&self, user_id: Uuid) -> AppResult<u64> {
        let result = sqlx::query(
            r#"
            UPDATE sessions
            SET is_active = false
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Delete expired sessions
    pub async fn delete_expired(&self) -> AppResult<u64> {
        let result = sqlx::query(
            r#"
            DELETE FROM sessions
            WHERE expires_at < $1 OR (is_active = false AND created_at < $2)
            "#,
        )
        .bind(Utc::now())
        .bind(Utc::now() - chrono::Duration::days(30))
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// List active sessions for a user
    pub async fn list_active_for_user(&self, user_id: Uuid) -> AppResult<Vec<Session>> {
        let sessions = sqlx::query_as::<_, Session>(
            r#"
            SELECT * FROM sessions
            WHERE user_id = $1 AND is_active = true
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(sessions)
    }

    /// Count active sessions for a user
    pub async fn count_active_for_user(&self, user_id: Uuid) -> AppResult<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM sessions
            WHERE user_id = $1 AND is_active = true
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_session_repository_creation() {
        // This is a placeholder test
        // Real tests would require a test database
        assert!(true);
    }
}
