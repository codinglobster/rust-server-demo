//! Redis client wrapper

use crate::config::redis::RedisConfig;
use crate::core::error::{AppError, AppResult};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

/// Redis client wrapper
#[derive(Clone, Debug)]
pub struct RedisClient {
    client: redis::Client,
}

impl RedisClient {
    /// Create a new Redis client
    pub async fn new(config: &RedisConfig) -> AppResult<Self> {
        let client = redis::Client::open(config.url.clone())
            .map_err(|e| AppError::Redis(e.to_string()))?;

        // Test the connection
        let mut conn = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::Redis(e.to_string()))?;

        // Set a test key to verify connection
        let _: String = redis::cmd("PING")
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::Redis(e.to_string()))?;

        tracing::info!("Redis connection established: {}", config.url);

        Ok(Self { client })
    }

    /// Get a connection from the pool
    async fn get_conn(&self) -> Result<redis::aio::MultiplexedConnection, redis::RedisError> {
        self.client.get_multiplexed_async_connection().await
    }

    /// Set a value with optional expiration
    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        expiration_seconds: Option<usize>,
    ) -> AppResult<()> {
        let serialized = serde_json::to_string(value)?;
        let mut conn = self.get_conn().await?;

        if let Some(expiry) = expiration_seconds {
            conn.set_ex::<_, _, ()>(key, serialized, expiry as u64).await?;
        } else {
            conn.set::<_, _, ()>(key, serialized).await?;
        }

        Ok(())
    }

    /// Get a value
    pub async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> AppResult<Option<T>> {
        let mut conn = self.get_conn().await?;
        let value: Option<String> = conn.get(key).await?;

        match value {
            Some(v) => {
                let deserialized = serde_json::from_str(&v)?;
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }

    /// Delete a value
    pub async fn del(&self, key: &str) -> AppResult<bool> {
        let mut conn = self.get_conn().await?;
        let affected: i32 = conn.del(key).await?;
        Ok(affected > 0)
    }

    /// Check if a key exists
    pub async fn exists(&self, key: &str) -> AppResult<bool> {
        let mut conn = self.get_conn().await?;
        let count: i32 = conn.exists(key).await?;
        Ok(count > 0)
    }

    /// Set expiration time for a key
    pub async fn expire(&self, key: &str, seconds: i64) -> AppResult<bool> {
        let mut conn = self.get_conn().await?;
        let result: bool = conn.expire(key, seconds).await?;
        Ok(result)
    }

    /// Increment a counter
    pub async fn incr(&self, key: &str) -> AppResult<i64> {
        let mut conn = self.get_conn().await?;
        let value: i64 = conn.incr(key, 1).await?;
        Ok(value)
    }

    /// Get a counter value
    pub async fn get_counter(&self, key: &str) -> AppResult<i64> {
        let mut conn = self.get_conn().await?;
        let value: Option<i64> = conn.get(key).await?;
        Ok(value.unwrap_or(0))
    }

    /// Add to a set
    pub async fn sadd(&self, key: &str, member: &str) -> AppResult<bool> {
        let mut conn = self.get_conn().await?;
        let count: i32 = conn.sadd(key, member).await?;
        Ok(count > 0)
    }

    /// Remove from a set
    pub async fn srem(&self, key: &str, member: &str) -> AppResult<bool> {
        let mut conn = self.get_conn().await?;
        let count: i32 = conn.srem(key, member).await?;
        Ok(count > 0)
    }

    /// Check if member exists in a set
    pub async fn sismember(&self, key: &str, member: &str) -> AppResult<bool> {
        let mut conn = self.get_conn().await?;
        let exists: bool = conn.sismember(key, member).await?;
        Ok(exists)
    }

    /// Get all members of a set
    pub async fn smembers(&self, key: &str) -> AppResult<Vec<String>> {
        let mut conn = self.get_conn().await?;
        let members: Vec<String> = conn.smembers(key).await?;
        Ok(members)
    }

    /// Push to a list (left)
    pub async fn lpush(&self, key: &str, value: &str) -> AppResult<usize> {
        let mut conn = self.get_conn().await?;
        let length: usize = conn.lpush(key, value).await?;
        Ok(length)
    }

    /// Pop from a list (right)
    pub async fn rpop(&self, key: &str) -> AppResult<Option<String>> {
        let mut conn = self.get_conn().await?;
        let value: Option<String> = conn.rpop(key, None).await?;
        Ok(value)
    }

    /// Get list length
    pub async fn llen(&self, key: &str) -> AppResult<usize> {
        let mut conn = self.get_conn().await?;
        let length: usize = conn.llen(key).await?;
        Ok(length)
    }

    /// Get list range
    pub async fn lrange(&self, key: &str, start: isize, end: isize) -> AppResult<Vec<String>> {
        let mut conn = self.get_conn().await?;
        let values: Vec<String> = conn.lrange(key, start, end).await?;
        Ok(values)
    }

    /// Trim list to specified range
    pub async fn ltrim(&self, key: &str, start: isize, end: isize) -> AppResult<()> {
        let mut conn = self.get_conn().await?;
        redis::cmd("LTRIM")
            .arg(key)
            .arg(start)
            .arg(end)
            .query_async::<()>(&mut conn)
            .await?;
        Ok(())
    }

    /// Health check
    pub async fn health_check(&self) -> AppResult<()> {
        let mut conn = self.get_conn().await?;
        let _: String = redis::cmd("PING").query_async(&mut conn).await?;
        Ok(())
    }

    /// Publish to a channel
    pub async fn publish(&self, channel: &str, message: &str) -> AppResult<i64> {
        let mut conn = self.get_conn().await?;
        let count: i64 = conn.publish(channel, message).await?;
        Ok(count)
    }

    /// Subscribe to channels
    /// Note: This creates a new connection for pubsub
    pub async fn subscribe(&self, channels: &[&str]) -> AppResult<()> {
        let mut conn = self.client.get_async_connection().await
            .map_err(|e| AppError::Redis(e.to_string()))?;

        let mut pubsub = conn.into_pubsub();

        for channel in channels {
            pubsub.subscribe(*channel).await?;
        }

        // The pubsub is moved here and would be dropped
        // In production, you'd return this and manage the subscription
        Ok(())
    }

    /// Clear all keys (use with caution!)
    pub async fn flushdb(&self) -> AppResult<()> {
        let mut conn = self.get_conn().await?;
        redis::cmd("FLUSHDB").query_async::<()>(&mut conn).await?;
        Ok(())
    }

    /// Get database statistics
    pub async fn info(&self) -> AppResult<String> {
        let mut conn = self.get_conn().await?;
        let info: String = redis::cmd("INFO").query_async(&mut conn).await?;
        Ok(info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_keys() {
        assert_eq!(CacheKeys::session("abc123"), "session:abc123");
        assert_eq!(CacheKeys::user("user-1"), "user:user-1");
        assert_eq!(CacheKeys::rate_limit("127.0.0.1"), "ratelimit:127.0.0.1");
    }
}
