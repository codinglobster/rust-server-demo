//! User repository

use crate::core::error::AppResult;
use crate::models::user::{User, RegisterRequest, UpdateUserRequest, UserRole};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

/// User repository for database operations
#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    /// Create a new user repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Find a user by ID
    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Find a user by username
    pub async fn find_by_username(&self, username: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Find a user by email
    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Create a new user
    pub async fn create(&self, request: &RegisterRequest, password_hash: &str) -> AppResult<User> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                id, username, email, password_hash, full_name,
                is_active, is_verified, role, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&request.username)
        .bind(&request.email)
        .bind(password_hash)
        .bind(&request.full_name)
        .bind(true)
        .bind(false)
        .bind(UserRole::User.as_str())
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Update a user
    pub async fn update(&self, id: Uuid, request: &UpdateUserRequest) -> AppResult<User> {
        let now = Utc::now();

        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET full_name = COALESCE($2, full_name),
                email = COALESCE($3, email),
                updated_at = $4
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&request.full_name)
        .bind(&request.email)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Update user password
    pub async fn update_password(&self, id: Uuid, password_hash: &str) -> AppResult<()> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE users
            SET password_hash = $2, updated_at = $3
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(password_hash)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Update last login timestamp
    pub async fn update_last_login(&self, id: Uuid) -> AppResult<()> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE users
            SET last_login_at = $2
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Delete a user (soft delete by setting is_active = false)
    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE users
            SET is_active = false, updated_at = $2
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// List users with pagination
    pub async fn list(&self, page: u64, per_page: u64) -> AppResult<(Vec<User>, i64)> {
        let offset = (page - 1) * per_page;

        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users
            WHERE is_active = true
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let total = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM users WHERE is_active = true
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((users, total))
    }

    /// Update user role
    pub async fn update_role(&self, id: Uuid, role: &str) -> AppResult<User> {
        let now = Utc::now();

        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET role = $2, updated_at = $3
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(role)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Verify user email
    pub async fn verify_email(&self, id: Uuid) -> AppResult<User> {
        let now = Utc::now();

        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET is_verified = true, updated_at = $2
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_user_repository_creation() {
        // This is a placeholder test
        // Real tests would require a test database
        assert!(true);
    }
}
