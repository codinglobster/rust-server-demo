//! Authentication service

use crate::auth::jwt::TokenPair;
use crate::auth::jwt::JwtService;
use crate::cache::client::RedisClient;
use crate::cache::keys::CacheKeys;
use crate::core::error::{AppError, AppResult};
use crate::models::user::{LoginRequest, RegisterRequest, UserRole};
use crate::models::session::LoginResponse;
use crate::services::UserService;
use uuid::Uuid;

#[cfg(feature = "kafka")]
use crate::messaging::producer::KafkaProducer;
#[cfg(feature = "kafka")]
use crate::messaging::topics::UserEvent;

/// Generate TTL with random jitter to prevent cache avalanche
fn generate_ttl_with_jitter(base_ttl: usize, jitter_range: usize) -> usize {
    use rand::Rng;
    let jitter = rand::thread_rng().gen_range(0..jitter_range);
    base_ttl + jitter
}

/// Authentication service
#[derive(Clone)]
pub struct AuthService {
    user_service: UserService,
    jwt_service: JwtService,
    redis: RedisClient,
    #[cfg(feature = "kafka")]
    kafka_producer: Option<KafkaProducer>,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(
        user_service: UserService,
        jwt_service: JwtService,
        redis: RedisClient,
        #[cfg(feature = "kafka")]
        kafka_producer: Option<KafkaProducer>,
    ) -> Self {
        Self {
            user_service,
            jwt_service,
            redis,
            #[cfg(feature = "kafka")]
            kafka_producer,
        }
    }

    /// Hash a password using bcrypt
    pub fn hash_password(&self, password: &str) -> AppResult<String> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))
    }

    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool> {
        bcrypt::verify(password, hash)
            .map_err(|e| AppError::Internal(format!("Failed to verify password: {}", e)))
    }

    /// Register a new user
    pub async fn register(&self, request: &RegisterRequest) -> AppResult<LoginResponse> {
        // Hash password
        let password_hash = self.hash_password(&request.password)?;

        // Create user
        let user_dto = self.user_service.register(request, &password_hash).await?;

        // Generate tokens
        let token_pair = self.jwt_service.generate_token_pair(
            user_dto.id,
            user_dto.username.clone(),
            user_dto.email.clone(),
            vec![UserRole::User.as_str().to_string()],
        )?;

        // Publish registration event
        #[cfg(feature = "kafka")]
        if let Some(producer) = &self.kafka_producer {
            let _ = producer
                .send_user_event(&UserEvent::Registered {
                    user_id: user_dto.id.to_string(),
                    username: user_dto.username.clone(),
                    email: user_dto.email.clone(),
                })
                .await;
        }

        Ok(LoginResponse {
            access_token: token_pair.access_token,
            refresh_token: token_pair.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: token_pair.access_expires_in,
            user: user_dto,
        })
    }

    /// Login a user
    pub async fn login(&self, request: &LoginRequest, _ip: Option<String>) -> AppResult<LoginResponse> {
        // Find user by username
        let user = self.user_service.get_by_username(&request.username).await?;

        // Verify password
        if !self.verify_password(&request.password, &user.password_hash)? {
            return Err(AppError::Auth(crate::core::error::AuthError::InvalidCredentials));
        }

        // Check if user is active
        if !user.is_active {
            return Err(AppError::Auth(crate::core::error::AuthError::InvalidCredentials));
        }

        // Update last login
        let _ = self
            .user_service
            .update_last_login(user.id)
            .await;

        // Generate tokens
        let roles = vec![user.role.clone()];
        let token_pair = self.jwt_service.generate_token_pair(
            user.id,
            user.username.clone(),
            user.email.clone(),
            roles,
        )?;

        // Cache access token (for fast validation) with random TTL
        let cache_key = CacheKeys::session(&token_pair.access_token);
        let ttl = generate_ttl_with_jitter(token_pair.access_expires_in as usize, 300);
        let _ = self
            .redis
            .set(&cache_key, &user.id.to_string(), Some(ttl))
            .await;

        // Publish login event
        #[cfg(feature = "kafka")]
        if let Some(producer) = &self.kafka_producer {
            let _ = producer
                .send_user_event(&UserEvent::Login {
                    user_id: user.id.to_string(),
                    ip,
                })
                .await;
        }

        Ok(LoginResponse {
            access_token: token_pair.access_token,
            refresh_token: token_pair.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: token_pair.access_expires_in,
            user: user.into(),
        })
    }

    /// Validate an access token
    pub async fn validate_token(&self, token: &str) -> AppResult<Uuid> {
        // First check cache
        let cache_key = CacheKeys::session(token);
        if let Ok(Some(user_id_str)) = self.redis.get::<String>(&cache_key).await {
            return Ok(Uuid::parse_str(&user_id_str)?);
        }

        // Fall back to JWT validation
        let user_id = self.jwt_service.get_user_id_from_token(token)?;

        // Check if user is still active
        if !self.user_service.is_user_active(user_id).await? {
            return Err(AppError::Auth(crate::core::error::AuthError::InvalidToken(
                "User is inactive".to_string(),
            )));
        }

        // Re-cache the token with random TTL
        let ttl = generate_ttl_with_jitter(3600, 300);
        let _ = self
            .redis
            .set(&cache_key, &user_id.to_string(), Some(ttl))
            .await;

        Ok(user_id)
    }

    /// Refresh tokens using a refresh token
    pub async fn refresh_token(&self, refresh_token: &str) -> AppResult<TokenPair> {
        // Validate refresh token
        let user_id = self.jwt_service.validate_refresh_token(refresh_token)?;

        // Get user
        let user_dto = self.user_service.get_user(user_id).await?;

        // Generate new token pair
        let token_pair = self.jwt_service.generate_token_pair(
            user_dto.id,
            user_dto.username.clone(),
            user_dto.email.clone(),
            vec![user_dto.role.clone()],
        )?;

        Ok(token_pair)
    }

    /// Logout a user
    pub async fn logout(&self, token: &str, _user_id: Uuid) -> AppResult<()> {
        // Remove token from cache
        let cache_key = CacheKeys::session(token);
        self.redis.del(&cache_key).await?;

        // Publish logout event
        #[cfg(feature = "kafka")]
        if let Some(producer) = &self.kafka_producer {
            let session_id = Uuid::new_v4(); // In real app, get from actual session
            let _ = producer
                .send_user_event(&UserEvent::Logout {
                    user_id: _user_id.to_string(),
                    session_id: session_id.to_string(),
                })
                .await;
        }

        Ok(())
    }

    /// Change user password
    pub async fn change_password(&self, user_id: Uuid, request: &crate::models::user::ChangePasswordRequest) -> AppResult<()> {
        // Get user
        let user = self.user_service.get_user(user_id).await?;

        // Get full user with password hash
        let user_with_hash = self.user_service.get_by_username(&user.username).await?;

        // Verify old password
        if !self.verify_password(&request.old_password, &user_with_hash.password_hash)? {
            return Err(AppError::Auth(crate::core::error::AuthError::InvalidCredentials));
        }

        // Hash new password
        let new_password_hash = self.hash_password(&request.new_password)?;

        // Update in database
        self.user_service.update_password(user_id, &new_password_hash).await?;

        tracing::info!("Password changed successfully for user: {}", user_id);

        Ok(())
    }

    /// Hash a token for storage (SHA-256)
    pub fn hash_token(&self, token: &str) -> AppResult<String> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }
}
