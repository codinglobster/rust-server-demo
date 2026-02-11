//! Session service

use crate::cache::client::RedisClient;
use crate::cache::keys::CacheKeys;
use crate::core::error::AppResult;
use crate::database::repositories::session::SessionRepository;
use crate::models::session::{SessionCreate, SessionDto};
use chrono::{Duration, Utc};
use uuid::Uuid;

/// Session service
#[derive(Clone)]
pub struct SessionService {
    repository: SessionRepository,
    redis: RedisClient,
}

impl SessionService {
    /// Create a new session service
    pub fn new(repository: SessionRepository, redis: RedisClient) -> Self {
        Self { repository, redis }
    }

    /// Create a new session
    pub async fn create_session(
        &self,
        user_id: Uuid,
        token_hash: &str,
        refresh_token_hash: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        expires_in_seconds: i64,
    ) -> AppResult<SessionDto> {
        let expires_at = Utc::now() + Duration::seconds(expires_in_seconds);

        let session_create = SessionCreate {
            user_id,
            token_hash: token_hash.to_string(),
            refresh_token_hash,
            ip_address,
            user_agent,
            expires_at,
        };

        let session = self.repository.create(&session_create).await?;
        Ok(SessionDto::from(session))
    }

    /// Get a session by ID
    pub async fn get_session(&self, id: Uuid) -> AppResult<Option<SessionDto>> {
        let session = self.repository.find_by_id(id).await?;
        Ok(session.map(SessionDto::from))
    }

    /// Get an active session by token hash
    pub async fn get_session_by_token_hash(&self, token_hash: &str) -> AppResult<Option<SessionDto>> {
        let session = self.repository.find_by_token_hash(token_hash).await?;

        // Check if session is expired
        if let Some(s) = &session {
            if s.expires_at < Utc::now() {
                // Invalidate expired session
                let _ = self.repository.invalidate(s.id).await;
                return Ok(None);
            }
        }

        Ok(session.map(SessionDto::from))
    }

    /// Update last accessed timestamp
    pub async fn update_last_accessed(&self, id: Uuid) -> AppResult<()> {
        self.repository.update_last_accessed(id).await
    }

    /// Invalidate a session
    pub async fn invalidate_session(&self, id: Uuid) -> AppResult<()> {
        self.repository.invalidate(id).await
    }

    /// Invalidate all sessions for a user
    pub async fn invalidate_all_user_sessions(&self, user_id: Uuid) -> AppResult<u64> {
        self.repository.invalidate_all_for_user(user_id).await
    }

    /// List active sessions for a user
    pub async fn list_user_sessions(&self, user_id: Uuid) -> AppResult<Vec<SessionDto>> {
        let sessions = self.repository.list_active_for_user(user_id).await?;
        Ok(sessions.into_iter().map(SessionDto::from).collect())
    }

    /// Count active sessions for a user
    pub async fn count_user_sessions(&self, user_id: Uuid) -> AppResult<i64> {
        self.repository.count_active_for_user(user_id).await
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self) -> AppResult<u64> {
        self.repository.delete_expired().await
    }

    /// Check if user has too many active sessions
    pub async fn has_too_many_sessions(&self, user_id: Uuid, max_sessions: i64) -> AppResult<bool> {
        let count = self.count_user_sessions(user_id).await?;
        Ok(count >= max_sessions)
    }

    /// Invalidate oldest session for a user (when exceeding max sessions)
    pub async fn invalidate_oldest_session(&self, user_id: Uuid) -> AppResult<()> {
        let sessions = self.repository.list_active_for_user(user_id).await?;

        if let Some(oldest) = sessions.first() {
            self.repository.invalidate(oldest.id).await?;
        }

        Ok(())
    }
}
