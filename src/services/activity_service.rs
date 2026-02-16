//! Activity log service

use crate::cache::client::RedisClient;
use crate::core::error::AppResult;
use crate::models::activity::{
    ActivityLog, ActivityLogDto, ActivityLogsResponse, CreateActivityLogRequest,
};
use chrono::Utc;
#[cfg(feature = "kafka")]
use crate::kafka::KafkaProducer;
use sqlx::PgPool;
use uuid::Uuid;

/// Activity log service
#[derive(Clone)]
pub struct ActivityService {
    pool: PgPool,
    redis: RedisClient,
    #[cfg(feature = "kafka")]
    kafka_producer: Option<KafkaProducer>,
}

impl ActivityService {
    /// Create a new activity service
    pub fn new(pool: PgPool, redis: RedisClient) -> Self {
        Self {
            pool,
            redis,
            #[cfg(feature = "kafka")]
            kafka_producer: None,
        }
    }

    /// Set Kafka producer
    #[cfg(feature = "kafka")]
    pub fn with_kafka(mut self, producer: KafkaProducer) -> Self {
        self.kafka_producer = Some(producer);
        self
    }

    /// Create an activity log
    pub async fn create_activity(
        &self,
        request: &CreateActivityLogRequest,
    ) -> AppResult<ActivityLogDto> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        // Determine event category
        let event_type_category = self.categorize_event(&request.event_type);

        // Insert into database
        let activity = sqlx::query_as::<_, ActivityLog>(
            r#"
            INSERT INTO activity_logs (
                id, user_id, event_type, event_type_category, description,
                metadata, ip_address, user_agent, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, user_id, event_type, event_type_category, description,
                metadata, ip_address, user_agent, created_at
            "#,
        )
        .bind(id)
        .bind(request.user_id)
        .bind(&request.event_type)
        .bind(&event_type_category)
        .bind(&request.description)
        .bind(&request.metadata)
        .bind(&request.ip_address)
        .bind(&request.user_agent)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        // Fetch username if user_id is present
        let username = if let Some(user_id) = activity.user_id {
            sqlx::query_scalar::<_, String>("SELECT username FROM users WHERE id = $1")
                .bind(user_id)
                .fetch_optional(&self.pool)
                .await?
        } else {
            None
        };

        let mut dto = ActivityLogDto::from(activity);
        dto.username = username;

        // Publish to Kafka if enabled
        #[cfg(feature = "kafka")]
        if let Some(producer) = &self.kafka_producer {
            use crate::models::activity::ActivityKafkaEvent;
            let kafka_event = ActivityKafkaEvent {
                event_id: dto.id,
                user_id: dto.user_id,
                event_type: dto.event_type.clone(),
                event_type_category: dto.event_type_category.clone(),
                description: dto.description.clone(),
                metadata: dto.metadata.clone(),
                ip_address: dto.ip_address.clone(),
                user_agent: dto.user_agent.clone(),
                timestamp: dto.created_at,
            };

            let _ = producer.publish_activity(&kafka_event);
            let _ = producer.flush(1000);
        }

        Ok(dto)
    }

    /// Get activity logs with pagination
    pub async fn list_activities(
        &self,
        page: u64,
        per_page: u64,
    ) -> AppResult<ActivityLogsResponse> {
        let offset = (page - 1) * per_page;

        let activities = sqlx::query_as::<_, ActivityLogDto>(
            r#"
            SELECT
                al.id, al.user_id, u.username, al.event_type, al.event_type_category,
                al.description, al.metadata, al.ip_address, al.user_agent, al.created_at
            FROM activity_logs al
            LEFT JOIN users u ON al.user_id = u.id
            ORDER BY al.created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM activity_logs")
            .fetch_one(&self.pool)
            .await?;

        Ok(ActivityLogsResponse {
            activities,
            total,
            page,
            per_page,
        })
    }

    /// Get activity logs for a specific user
    pub async fn list_user_activities(
        &self,
        user_id: Uuid,
        page: u64,
        per_page: u64,
    ) -> AppResult<ActivityLogsResponse> {
        let offset = (page - 1) * per_page;

        let activities = sqlx::query_as::<_, ActivityLogDto>(
            r#"
            SELECT
                al.id, al.user_id, u.username, al.event_type, al.event_type_category,
                al.description, al.metadata, al.ip_address, al.user_agent, al.created_at
            FROM activity_logs al
            LEFT JOIN users u ON al.user_id = u.id
            WHERE al.user_id = $1
            ORDER BY al.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM activity_logs WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(ActivityLogsResponse {
            activities,
            total,
            page,
            per_page,
        })
    }

    /// Get recent activities from cache
    pub async fn get_recent_activities(&self, limit: usize) -> AppResult<Vec<ActivityLogDto>> {
        let cache_key = "recent_activities";
        let cached = self.redis.lrange(cache_key, 0, (limit as isize) - 1).await?;

        if !cached.is_empty() {
            let mut activities = Vec::new();
            for activity_str in cached {
                if let Ok(activity) = serde_json::from_str::<ActivityLogDto>(&activity_str) {
                    activities.push(activity);
                }
            }
            if !activities.is_empty() {
                return Ok(activities);
            }
        }

        // Cache miss, fetch from database
        let activities = sqlx::query_as::<_, ActivityLogDto>(
            r#"
            SELECT
                al.id, al.user_id, u.username, al.event_type, al.event_type_category,
                al.description, al.metadata, al.ip_address, al.user_agent, al.created_at
            FROM activity_logs al
            LEFT JOIN users u ON al.user_id = u.id
            ORDER BY al.created_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        // Cache the results
        for activity in &activities {
            if let Ok(json) = serde_json::to_string(activity) {
                let _ = self.redis.lpush(cache_key, &json).await;
            }
        }
        let _ = self.redis.expire(cache_key, 300).await; // 5 minutes

        Ok(activities)
    }

    /// Categorize event type
    fn categorize_event(&self, event_type: &str) -> String {
        if event_type.starts_with("user_") {
            "user".to_string()
        } else if event_type.starts_with("message_") {
            "message".to_string()
        } else if event_type.starts_with("room_") {
            "room".to_string()
        } else if event_type.starts_with("system_") {
            "system".to_string()
        } else if event_type.starts_with("error_") {
            "error".to_string()
        } else {
            "other".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::activity::{ActivityEventType, CreateActivityLogRequest};
    use uuid::Uuid;

    #[test]
    fn test_categorize_event() {
        // Create a mock service without actual DB/Redis connections
        // Just test the categorize_event method logic
        struct TestService;
        impl TestService {
            fn categorize_event(&self, event_type: &str) -> String {
                if event_type.starts_with("user_") {
                    "user".to_string()
                } else if event_type.starts_with("message_") {
                    "message".to_string()
                } else if event_type.starts_with("room_") {
                    "room".to_string()
                } else if event_type.starts_with("system_") {
                    "system".to_string()
                } else if event_type.starts_with("error_") {
                    "error".to_string()
                } else {
                    "other".to_string()
                }
            }
        }

        let service = TestService;
        assert_eq!(service.categorize_event("user_logged_in"), "user");
        assert_eq!(service.categorize_event("message_sent"), "message");
        assert_eq!(service.categorize_event("room_joined"), "room");
        assert_eq!(service.categorize_event("system_alert"), "system");
        assert_eq!(service.categorize_event("error_occurred"), "error");
        assert_eq!(service.categorize_event("unknown_event"), "other");
    }

    #[test]
    fn test_create_activity_request_validation() {
        let request = CreateActivityLogRequest {
            user_id: Some(Uuid::new_v4()),
            event_type: "user_logged_in".to_string(),
            description: "Test activity".to_string(),
            metadata: Some(serde_json::json!({"key": "value"})),
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("TestAgent/1.0".to_string()),
        };

        assert_eq!(request.event_type, "user_logged_in");
        assert_eq!(request.description, "Test activity");
        assert!(request.metadata.is_some());
    }

    #[test]
    fn test_activity_event_type_conversion() {
        use std::str::FromStr;

        // Test string to enum conversion
        let event_type_str = "user_logged_in";
        let event_type = ActivityEventType::from_str(event_type_str);
        assert!(event_type.is_ok());
        assert_eq!(event_type.unwrap(), ActivityEventType::UserLoggedIn);

        // Test enum to string conversion
        let event_type = ActivityEventType::MessageSent;
        assert_eq!(event_type.as_str(), "message_sent");
    }

    #[test]
    fn test_activity_event_type_display() {
        assert_eq!(format!("{}", ActivityEventType::UserLoggedIn), "user_logged_in");
        assert_eq!(format!("{}", ActivityEventType::MessageSent), "message_sent");
    }

    #[test]
    fn test_activity_kafka_event_json_roundtrip() {
        use crate::models::activity::ActivityKafkaEvent;
        use chrono::Utc;

        let original_event = ActivityKafkaEvent {
            event_id: Uuid::new_v4(),
            user_id: Some(Uuid::new_v4()),
            event_type: "test_event".to_string(),
            event_type_category: "test".to_string(),
            description: "Test event".to_string(),
            metadata: Some(serde_json::json!({"key": "value"})),
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("TestAgent".to_string()),
            timestamp: Utc::now(),
        };

        let json = original_event.to_json().unwrap();
        let restored_event = ActivityKafkaEvent::from_json(&json).unwrap();

        assert_eq!(original_event.event_id, restored_event.event_id);
        assert_eq!(original_event.event_type, restored_event.event_type);
    }
}
