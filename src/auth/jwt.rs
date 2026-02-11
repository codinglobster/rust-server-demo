//! JWT service for token generation and validation

use crate::auth::claims::Claims;
use crate::config::JwtConfig;
use crate::core::error::{AppError, AppResult};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

/// Token pair containing access and refresh tokens
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub access_expires_in: u64,
    pub refresh_expires_in: u64,
}

/// JWT service
#[derive(Clone)]
pub struct JwtService {
    config: JwtConfig,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    /// Create a new JWT service
    pub fn new(config: &JwtConfig) -> Result<Self, AppError> {
        config.validate_secret()?;

        let encoding_key = EncodingKey::from_secret(config.secret.as_ref());
        let decoding_key = DecodingKey::from_secret(config.secret.as_ref());

        Ok(Self {
            config: config.clone(),
            encoding_key,
            decoding_key,
        })
    }

    /// Generate an access token for a user
    pub fn generate_access_token(
        &self,
        user_id: Uuid,
        username: String,
        email: String,
        roles: Vec<String>,
    ) -> AppResult<String> {
        let now = Utc::now();
        let expiration = now + Duration::seconds(self.config.access_expiration as i64);

        let claims = Claims::new(
            user_id,
            username,
            email,
            roles,
            expiration.timestamp() as usize,
            now.timestamp() as usize,
            self.config.issuer.clone(),
            self.config.audience.clone(),
        );

        encode(
            &Header::default(),
            &claims,
            &self.encoding_key,
        )
        .map_err(|e| AppError::Jwt(e.to_string()))
    }

    /// Generate a refresh token for a user
    pub fn generate_refresh_token(&self, user_id: Uuid) -> AppResult<String> {
        let now = Utc::now();
        let expiration = now + Duration::seconds(self.config.refresh_expiration as i64);

        let claims = Claims::new(
            user_id,
            "refresh".to_string(),
            String::new(),
            vec!["refresh".to_string()],
            expiration.timestamp() as usize,
            now.timestamp() as usize,
            self.config.issuer.clone(),
            self.config.audience.clone(),
        );

        encode(
            &Header::default(),
            &claims,
            &self.encoding_key,
        )
        .map_err(|e| AppError::Jwt(e.to_string()))
    }

    /// Generate a token pair for a user
    pub fn generate_token_pair(
        &self,
        user_id: Uuid,
        username: String,
        email: String,
        roles: Vec<String>,
    ) -> AppResult<TokenPair> {
        let access_token = self.generate_access_token(user_id, username, email, roles.clone())?;
        let refresh_token = self.generate_refresh_token(user_id)?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            access_expires_in: self.config.access_expiration,
            refresh_expires_in: self.config.refresh_expiration,
        })
    }

    /// Validate and decode a token
    pub fn validate_token(&self, token: &str) -> AppResult<Claims> {
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_audience(&[&self.config.audience]);
        validation.set_issuer(&[&self.config.issuer]);

        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)
            .map_err(|e| AppError::Auth(crate::core::error::AuthError::InvalidToken(e.to_string())))?;

        // Check if token is expired
        let now = Utc::now().timestamp() as usize;
        if token_data.claims.exp < now {
            return Err(AppError::Auth(crate::core::error::AuthError::TokenExpired));
        }

        Ok(token_data.claims)
    }

    /// Validate a refresh token
    pub fn validate_refresh_token(&self, token: &str) -> AppResult<Uuid> {
        let claims = self.validate_token(token)?;

        // Check if this is a refresh token
        if !claims.roles.contains(&"refresh".to_string()) {
            return Err(AppError::Auth(crate::core::error::AuthError::InvalidToken(
                "Not a refresh token".to_string(),
            )));
        }

        claims.user_id().map_err(|e| AppError::Jwt(e.to_string()))
    }

    /// Get user ID from token without full validation (for quick checks)
    pub fn get_user_id_from_token(&self, token: &str) -> AppResult<Uuid> {
        // Quick decode without signature verification (use carefully!)
        let claims = self.validate_token(token)?;
        claims.user_id().map_err(|e| AppError::Jwt(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_service_creation() {
        let config = JwtConfig {
            secret: "this-is-a-very-long-secret-key-for-testing-purposes-only".to_string(),
            ..Default::default()
        };

        let service = JwtService::new(&config);
        assert!(service.is_ok());
    }

    #[test]
    fn test_invalid_secret() {
        let config = JwtConfig::default();
        let service = JwtService::new(&config);
        assert!(service.is_err());
    }
}
