//! Kafka producer

use crate::config::kafka::KafkaConfig;
use crate::core::error::{AppError, AppResult};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use serde::Serialize;
use std::time::Duration;

/// Kafka producer wrapper
#[derive(Clone)]
pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    /// Create a new Kafka producer
    pub fn new(config: &KafkaConfig) -> AppResult<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("message.timeout.ms", &config.message_timeout.to_string())
            .set("request.required.acks", "all")
            .set("enable.idempotence", "true")
            .set("compression.type", "snappy")
            .create()
            .map_err(|e| AppError::Kafka(e.to_string()))?;

        tracing::info!("Kafka producer created: {}", config.brokers);

        Ok(Self { producer })
    }

    /// Send a message to a topic
    pub async fn send<T: Serialize>(
        &self,
        topic: &str,
        key: Option<&str>,
        payload: &T,
    ) -> AppResult<()> {
        let payload_json = serde_json::to_string(payload)?;

        let record = FutureRecord::to(topic)
            .payload(&payload_json)
            .key(key.unwrap_or(""));

        let delivery_timeout = Timeout::After(Duration::from_secs(5));

        self.producer
            .send(record, delivery_timeout)
            .await
            .map_err(|(e, _)| AppError::Kafka(e.to_string()))?;

        Ok(())
    }

    /// Send a user event
    pub async fn send_user_event(&self, event: &crate::messaging::topics::UserEvent) -> AppResult<()> {
        self.send(
            crate::messaging::topics::KafkaTopics::USER_EVENTS,
            Some("user-event"),
            event,
        )
        .await
    }

    /// Send a message event
    pub async fn send_message_event(
        &self,
        event: &crate::messaging::topics::MessageEvent,
    ) -> AppResult<()> {
        self.send(
            crate::messaging::topics::KafkaTopics::MESSAGES,
            Some("message-event"),
            event,
        )
        .await
    }

    /// Send a session event
    pub async fn send_session_event(
        &self,
        event: &crate::messaging::topics::SessionEvent,
    ) -> AppResult<()> {
        self.send(
            crate::messaging::topics::KafkaTopics::SESSION_EVENTS,
            Some("session-event"),
            event,
        )
        .await
    }

    /// Health check
    pub async fn health_check(&self) -> AppResult<()> {
        // Try to send a ping message to audit log topic
        let ping = serde_json::json!({
            "type": "ping",
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        let record = FutureRecord::to(crate::messaging::topics::KafkaTopics::AUDIT_LOG)
            .payload(&serde_json::to_string(&ping).unwrap())
            .key("ping");

        let delivery_timeout = Timeout::After(Duration::from_secs(2));

        self.producer
            .send(record, delivery_timeout)
            .await
            .map_err(|(e, _)| AppError::Kafka(e.to_string()))?;

        Ok(())
    }
}
