//! User service

use crate::cache::client::RedisClient;
use crate::cache::keys::CacheKeys;
use crate::core::error::{AppError, AppResult};
use crate::database::repositories::user::UserRepository;
use crate::models::user::{RegisterRequest, UpdateUserRequest, User, UserDto, UserRole};
use sqlx::PgPool;
use uuid::Uuid;

/// Generate TTL with random jitter to prevent cache avalanche
/// base_ttl: base expiration time in seconds
/// jitter_range: random jitter range in seconds (0 to jitter_range)
fn generate_ttl_with_jitter(base_ttl: usize, jitter_range: usize) -> usize {
    use rand::Rng;
    let jitter = rand::thread_rng().gen_range(0..jitter_range);
    base_ttl + jitter
}

/// User service
#[derive(Clone)]
pub struct UserService {
    #[allow(dead_code)]
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
        if let Ok(Some(cached)) = self.redis.get::<String>(&cache_key).await {
            // Check if it's a null value marker (cache penetration prevention)
            if cached == CacheKeys::NULL_VALUE {
                return Err(AppError::NotFound("User not found".to_string()));
            }
            // Try to deserialize as UserDto
            if let Ok(user_dto) = serde_json::from_str::<UserDto>(&cached) {
                return Ok(user_dto);
            }
        }

        // Cache miss - use distributed lock to prevent cache breakdown
        let lock_key = format!("lock:{}", cache_key);
        let lock_value = uuid::Uuid::new_v4().to_string();

        // Try to acquire lock with 3 second timeout
        let lock_acquired = self
            .redis
            .try_lock_with_timeout(&lock_key, &lock_value, 10, 3000)
            .await?;

        if lock_acquired {
            // Double-check cache after acquiring lock
            if let Ok(Some(cached)) = self.redis.get::<String>(&cache_key).await {
                // Release lock before returning
                let _ = self.redis.release_lock(&lock_key, &lock_value).await;

                if cached == CacheKeys::NULL_VALUE {
                    return Err(AppError::NotFound("User not found".to_string()));
                }
                if let Ok(user_dto) = serde_json::from_str::<UserDto>(&cached) {
                    return Ok(user_dto);
                }
            }

            // Get from database
            let user_opt = self.repository.find_by_id(id).await?;

            // If user doesn't exist, cache null value to prevent cache penetration
            if user_opt.is_none() {
                // Cache null value with shorter TTL (5 minutes)
                let _ = self
                    .redis
                    .set(&cache_key, &CacheKeys::NULL_VALUE, Some(300))
                    .await;

                // Release lock
                let _ = self.redis.release_lock(&lock_key, &lock_value).await;

                return Err(AppError::NotFound("User not found".to_string()));
            }

            let user = user_opt.unwrap();
            let user_dto = UserDto::from(user.clone());

            // Cache with random TTL to prevent cache avalanche (1 hour + 0-5 minutes jitter)
            let ttl = generate_ttl_with_jitter(3600, 300);
            let serialized = serde_json::to_string(&user_dto)?;
            let _ = self
                .redis
                .set(&cache_key, &serialized, Some(ttl))
                .await;

            // Release lock
            let _ = self.redis.release_lock(&lock_key, &lock_value).await;

            Ok(user_dto)
        } else {
            // Failed to acquire lock - wait and retry reading from cache
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            // Retry reading from cache (recursive call with max depth protection)
            if let Ok(Some(cached)) = self.redis.get::<String>(&cache_key).await {
                if cached == CacheKeys::NULL_VALUE {
                    return Err(AppError::NotFound("User not found".to_string()));
                }
                if let Ok(user_dto) = serde_json::from_str::<UserDto>(&cached) {
                    return Ok(user_dto);
                }
            }

            // If still no cache, fall back to direct DB query
            let user = self
                .repository
                .find_by_id(id)
                .await?
                .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

            Ok(UserDto::from(user))
        }
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
        if self.repository.find_by_username(&request.username).await?.is_some() {
            return Err(AppError::Conflict("Username already exists".to_string()));
        }

        // Check if email already exists
        if self.repository.find_by_email(&request.email).await?.is_some() {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }

        // Create user
        let user = self.repository.create(request, password_hash).await?;
        let user_dto = UserDto::from(user);

        Ok(user_dto)
    }

    /// Update a user
    pub async fn update_user(&self, id: Uuid, request: &UpdateUserRequest) -> AppResult<UserDto> {
        let cache_key = CacheKeys::user(&id.to_string());

        // First cache deletion (before DB update)
        let _ = self.redis.del(&cache_key).await;

        // Update database
        let user = self.repository.update(id, request).await?;
        let user_dto = UserDto::from(user.clone());

        // Delayed second deletion (prevent dirty reads in high concurrency)
        let redis_clone = self.redis.clone();
        let cache_key_clone = cache_key.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let _ = redis_clone.del(&cache_key_clone).await;
        });

        Ok(user_dto)
    }

    /// Update user password
    pub async fn update_password(&self, id: Uuid, password_hash: &str) -> AppResult<()> {
        let cache_key = CacheKeys::user(&id.to_string());

        // First cache deletion
        let _ = self.redis.del(&cache_key).await;

        // Update database
        self.repository.update_password(id, password_hash).await?;

        // Delayed second deletion
        let redis_clone = self.redis.clone();
        let cache_key_clone = cache_key.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let _ = redis_clone.del(&cache_key_clone).await;
        });

        Ok(())
    }

    /// Update user role
    pub async fn update_role(&self, id: Uuid, role: &str) -> AppResult<UserDto> {
        // Validate role
        role.parse::<UserRole>().map_err(|_| {
            AppError::Validation(crate::core::error::ValidationError::InvalidInput(
                "Invalid role".to_string(),
            ))
        })?;

        let cache_key = CacheKeys::user(&id.to_string());

        // First cache deletion
        let _ = self.redis.del(&cache_key).await;

        // Update database
        let user = self.repository.update_role(id, role).await?;
        let user_dto = UserDto::from(user.clone());

        // Delayed second deletion
        let redis_clone = self.redis.clone();
        let cache_key_clone = cache_key.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let _ = redis_clone.del(&cache_key_clone).await;
        });

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
        let cache_key = CacheKeys::user(&id.to_string());

        // First cache deletion
        let _ = self.redis.del(&cache_key).await;

        // Delete from database
        self.repository.delete(id).await?;

        // Delayed second deletion
        let redis_clone = self.redis.clone();
        let cache_key_clone = cache_key.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let _ = redis_clone.del(&cache_key_clone).await;
        });

        Ok(())
    }

    /// Verify user email
    pub async fn verify_email(&self, id: Uuid) -> AppResult<UserDto> {
        let cache_key = CacheKeys::user(&id.to_string());

        // First cache deletion
        let _ = self.redis.del(&cache_key).await;

        // Update database
        let user = self.repository.verify_email(id).await?;
        let user_dto = UserDto::from(user.clone());

        // Delayed second deletion
        let redis_clone = self.redis.clone();
        let cache_key_clone = cache_key.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let _ = redis_clone.del(&cache_key_clone).await;
        });

        Ok(user_dto)
    }

    /// Check if user is active
    pub async fn is_user_active(&self, id: Uuid) -> AppResult<bool> {
        let user = self.repository.find_by_id(id).await?;
        Ok(user.map(|u| u.is_active).unwrap_or(false))
    }

    /// Update last login timestamp
    pub async fn update_last_login(&self, id: Uuid) -> AppResult<()> {
        let cache_key = CacheKeys::user(&id.to_string());

        // First cache deletion
        let _ = self.redis.del(&cache_key).await;

        // Update database
        self.repository.update_last_login(id).await?;

        // Delayed second deletion
        let redis_clone = self.redis.clone();
        let cache_key_clone = cache_key.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let _ = redis_clone.del(&cache_key_clone).await;
        });

        Ok(())
    }

    /// Warm up cache with hot user data
    /// Loads multiple users into cache, useful for service startup
    pub async fn warm_up_cache(&self, user_ids: Vec<Uuid>) -> AppResult<usize> {
        let mut success_count = 0;

        for id in user_ids {
            // Use get_user which will populate cache on miss
            if self.get_user(id).await.is_ok() {
                success_count += 1;
            }
        }

        tracing::info!(
            "Cache warm-up completed: {}/{} users cached",
            success_count,
            success_count
        );

        Ok(success_count)
    }

    /// Warm up cache for the most recently active users
    /// Useful for preloading hot data on service startup
    pub async fn warm_up_recent_active_users(&self, limit: u64) -> AppResult<usize> {
        // Get recently active users from database
        let (users, _) = self.repository.list(1, limit).await?;

        let user_ids: Vec<Uuid> = users.iter().map(|u| u.id).collect();
        self.warm_up_cache(user_ids).await
    }
}
