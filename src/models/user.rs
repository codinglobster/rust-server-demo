//! User model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

/// User model from database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

/// User DTO for API responses (excludes sensitive data)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserDto {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            full_name: user.full_name,
            is_active: user.is_active,
            is_verified: user.is_verified,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
            last_login_at: user.last_login_at,
        }
    }
}

/// User registration request
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub full_name: Option<String>,
}

/// User login request
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// User update request
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    #[validate(length(max = 100))]
    pub full_name: Option<String>,
    pub email: Option<String>,
}

/// Password change request
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    #[validate(length(min = 8))]
    pub new_password: String,
}

/// User roles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    User,
    Moderator,
    Admin,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::User => "user",
            UserRole::Moderator => "moderator",
            UserRole::Admin => "admin",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "user" => Some(UserRole::User),
            "moderator" => Some(UserRole::Moderator),
            "admin" => Some(UserRole::Admin),
            _ => None,
        }
    }

    pub fn has_higher_or_equal_privilege(&self, other: &UserRole) -> bool {
        match (self, other) {
            (UserRole::Admin, _) => true,
            (UserRole::Moderator, UserRole::User) => true,
            (UserRole::Moderator, UserRole::Moderator) => true,
            (UserRole::User, UserRole::User) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_role_from_str() {
        assert_eq!(UserRole::from_str("user"), Some(UserRole::User));
        assert_eq!(UserRole::from_str("USER"), Some(UserRole::User));
        assert_eq!(UserRole::from_str("admin"), Some(UserRole::Admin));
        assert_eq!(UserRole::from_str("invalid"), None);
    }

    #[test]
    fn test_user_role_privileges() {
        assert!(UserRole::Admin.has_higher_or_equal_privilege(&UserRole::Moderator));
        assert!(UserRole::Moderator.has_higher_or_equal_privilege(&UserRole::User));
        assert!(!UserRole::User.has_higher_or_equal_privilege(&UserRole::Admin));
    }
}
