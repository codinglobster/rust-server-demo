//! Database connection and pool management

use crate::config::DatabaseConfig;
use crate::core::error::{AppError, AppResult};
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgPool;
use std::time::Duration;

/// Database wrapper with connection pool
#[derive(Debug, Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create a new database instance with the given configuration
    pub async fn new(config: &DatabaseConfig) -> AppResult<Self> {
        let pool = create_pool(config).await?;
        Ok(Self { pool })
    }

    /// Get the underlying connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Check if the database connection is healthy
    pub async fn health_check(&self) -> AppResult<()> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;
        Ok(())
    }

    /// Run database migrations
    pub async fn run_migrations(&self) -> AppResult<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| AppError::Internal(format!("Migration error: {}", e)))?;
        Ok(())
    }

    /// Get connection pool statistics
    pub fn pool_stats(&self) -> PoolStats {
        PoolStats {
            size: self.pool.size() as u32,
            idle: self.pool.num_idle() as u32,
        }
    }
}

/// Connection pool statistics
#[derive(Debug, Clone, serde::Serialize)]
pub struct PoolStats {
    pub size: u32,
    pub idle: u32,
}

/// Create a PostgreSQL connection pool
pub async fn create_pool(config: &DatabaseConfig) -> AppResult<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.connect_timeout))
        .idle_timeout(Duration::from_secs(config.idle_timeout))
        .max_lifetime(Duration::from_secs(config.max_lifetime))
        .test_before_acquire(true)
        .connect(&config.url)
        .await
        .map_err(|e| AppError::Database(e))?;

    tracing::info!("Database connection pool created: max_connections={}", config.max_connections);

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_stats() {
        let stats = PoolStats {
            size: 10,
            idle: 5,
        };
        assert_eq!(stats.size, 10);
        assert_eq!(stats.idle, 5);
    }
}
