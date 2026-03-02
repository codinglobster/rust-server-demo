//! Rate limiting middleware

use crate::cache::client::RedisClient;
use crate::cache::keys::CacheKeys;
use crate::core::error::AppError;
use axum::{
    extract::{ConnectInfo, Request},
    middleware::Next,
    response::Response,
};
use std::net::SocketAddr;

/// Rate limit configuration
#[derive(Clone, Debug)]
pub struct RateLimitConfig {
    /// Maximum requests per window
    pub max_requests: u32,
    /// Time window in seconds
    pub window_seconds: i64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window_seconds: 60,
        }
    }
}

/// Extract client identifier from request
/// Priority: User ID > IP address
fn extract_identifier(req: &Request) -> String {
    // Try to get user ID from extensions (set by auth middleware)
    if let Some(user) = req.extensions().get::<crate::auth::claims::TokenUser>() {
        return format!("user:{}", user.id);
    }

    // Try to get IP from ConnectInfo
    if let Some(connect_info) = req.extensions().get::<ConnectInfo<SocketAddr>>() {
        return format!("ip:{}", connect_info.0.ip());
    }

    // Fallback to X-Forwarded-For header
    if let Some(forwarded) = req.headers().get("x-forwarded-for") {
        if let Ok(ip) = forwarded.to_str() {
            return format!("ip:{}", ip.split(',').next().unwrap_or("unknown"));
        }
    }

    // Fallback to X-Real-IP header
    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(ip) = real_ip.to_str() {
            return format!("ip:{}", ip);
        }
    }

    "ip:unknown".to_string()
}

/// Rate limiting middleware factory
/// Creates a middleware that can be used with Axum's layer system
pub fn create_rate_limiter(
    redis: RedisClient,
    config: RateLimitConfig,
) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, AppError>> + Send>> + Clone {
    move |req: Request, next: Next| {
        let redis = redis.clone();
        let config = config.clone();
        Box::pin(async move {
            rate_limit_check(redis, config, req, next).await
        })
    }
}

/// Internal rate limit check function
async fn rate_limit_check(
    redis: RedisClient,
    config: RateLimitConfig,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let identifier = extract_identifier(&req);
    let cache_key = CacheKeys::rate_limit(&identifier);

    // Increment request count
    let count = redis.incr(&cache_key).await?;

    // Set expiration on first request in window
    if count == 1 {
        redis.expire(&cache_key, config.window_seconds).await?;
    }

    // Check if limit exceeded
    if count > config.max_requests as i64 {
        tracing::warn!(
            "Rate limit exceeded for {}: {} requests in {} seconds",
            identifier,
            count,
            config.window_seconds
        );

        return Err(AppError::RateLimitExceeded);
    }

    // Add rate limit headers to response
    let mut response = next.run(req).await;

    // Add rate limit info headers
    response.headers_mut().insert(
        "X-RateLimit-Limit",
        config.max_requests.to_string().parse().unwrap(),
    );
    response.headers_mut().insert(
        "X-RateLimit-Remaining",
        (config.max_requests as i64 - count)
            .max(0)
            .to_string()
            .parse()
            .unwrap(),
    );

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_rate_limit_config() {
        let config = RateLimitConfig::default();
        assert_eq!(config.max_requests, 100);
        assert_eq!(config.window_seconds, 60);
    }

    #[test]
    fn test_extract_identifier_without_auth() {
        let req = Request::new(axum::body::Body::empty());
        let identifier = extract_identifier(&req);
        assert!(identifier.starts_with("ip:"));
    }
}
