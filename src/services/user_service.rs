//! User service

use crate::cache::client::RedisClient;
use crate::cache::keys::CacheKeys;
use crate::core::error::{AppError, AppResult};
use crate::database::repositories::user::UserRepository;
use crate::models::user::{RegisterRequest, UpdateUserRequest, User, UserDto, UserRole};
use sqlx::PgPool;
use uuid::Uuid;

/// User service
#[derive(Clone)]
pub struct UserService {
    pool: PgPool,
    repository: UserRepository,
    redis: RedisClient,
}

impl UserService {
    /// Create a new user service
    pub fn new(pool: PgPool, redis: RedisClient) -> Self {
        let repository = UserRepository::new(pool.clone());
        Self {
            pool,
            repository,
            redis,
        }
    }

    /// Get a user by ID (with cache)
    pub async fn get_user(&self, id: Uuid) -> AppResult<UserDto> {
        // Try cache first
        let cache_key = CacheKeys::user(&id.to_string());
        if let Ok(Some(cached)) = self.redis.get::<UserDto>(&cache_key).await {
            return Ok(cached);
        }

        // Get from database
        let user = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let user_dto = UserDto::from(user.clone());

        // Cache for 1 hour
        let _ = self
            .redis
            .set(&cache_key, &user_dto, Some(3600))
            .await;

        Ok(user_dto)
    }

    /// Get a user by username
    pub async fn get_by_username(&self, username: &str) -> AppResult<User> {
        self.repository
            .find_by_username(username)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))
    }

    /// Get a user by email
    pub async fn get_by_email(&self, email: &str) -> AppResult<Option<User>> {
        self.repository.find_by_email(email).await
    }

    /// Register a new user
    pub async fn register(&self, request: &RegisterRequest, password_hash: &str) -> AppResult<UserDto> {
        // Check if username already exists
        if let Some(_) = self.repository.find_by_username(&request.username).await? {
            return Err(AppError::Conflict("Username already exists".to_string()));
        }

        // Check if email already exists
        if let Some(_) = self.repository.find_by_email(&request.email).await? {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }

        // Create user
        let user = self.repository.create(request, password_hash).await?;
        let user_dto = UserDto::from(user);

        Ok(user_dto)
    }

    /// Update a user
    pub async fn update_user(&self, id: Uuid, request: &UpdateUserRequest) -> AppResult<UserDto> {
        let user = self.repository.update(id, request).await?;
        let user_dto = UserDto::from(user.clone());

        // Invalidate cache
        let cache_key = CacheKeys::user(&id.to_string());
        let _ = self.redis.del(&cache_key).await;

        Ok(user_dto)
    }

    /// Update user password
    pub async fn update_password(&self, id: Uuid, password_hash: &str) -> AppResult<()> {
        self.repository.update_password(id, password_hash).await?;

        // Invalidate cache
        let cache_key = CacheKeys::user(&id.to_string());
        let _ = self.redis.del(&cache_key).await;

        Ok(())
    }

    /// Update user role
    pub async fn update_role(&self, id: Uuid, role: &str) -> AppResult<UserDto> {
        // Validate role
        UserRole::from_str(role)
            .ok_or_else(|| AppError::Validation(crate::core::error::ValidationError::InvalidInput(
                "Invalid role".to_string(),
            )))?;

        let user = self.repository.update_role(id, role).await?;
        let user_dto = UserDto::from(user.clone());

        // Invalidate cache
        let cache_key = CacheKeys::user(&id.to_string());
        let _ = self.redis.del(&cache_key).await;

        Ok(user_dto)
    }

    /// List users with pagination
    pub async fn list_users(&self, page: u64, per_page: u64) -> AppResult<(Vec<UserDto>, i64)> {
        let (users, total) = self.repository.list(page, per_page).await?;

        let user_dtos: Vec<UserDto> = users.into_iter().map(UserDto::from).collect();

        Ok((user_dtos, total))
    }

    /// Delete a user (soft delete)
    pub async fn delete_user(&self, id: Uuid) -> AppResult<()> {
        self.repository.delete(id).await?;

        // Invalidate cache
        let cache_key = CacheKeys::user(&id.to_string());
        let _ = self.redis.del(&cache_key).await;

        Ok(())
    }

    /// Verify user email
    pub async fn verify_email(&self, id: Uuid) -> AppResult<UserDto> {
        let user = self.repository.verify_email(id).await?;
        let user_dto = UserDto::from(user.clone());

        // Invalidate cache
        let cache_key = CacheKeys::user(&id.to_string());
        let _ = self.redis.del(&cache_key).await;

        Ok(user_dto)
    }

    /// Check if user is active
    pub async fn is_user_active(&self, id: Uuid) -> AppResult<bool> {
        let user = self.repository.find_by_id(id).await?;
        Ok(user.map(|u| u.is_active).unwrap_or(false))
    }

    /// Update last login timestamp
    pub async fn update_last_login(&self, id: Uuid) -> AppResult<()> {
        self.repository.update_last_login(id).await?;

        // Invalidate cache
        let cache_key = CacheKeys::user(&id.to_string());
        let _ = self.redis.del(&cache_key).await;

        Ok(())
    }
}
