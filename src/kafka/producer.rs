//! Kafka producer wrapper for publishing events

use crate::config::kafka::KafkaConfig;
use crate::core::error::{AppError, AppResult};
use rdkafka::config::ClientConfig;
use rdkafka::message::ToBytes;
use rdkafka::producer::{BaseRecord, Producer, ThreadedProducer};
use rdkafka::util::Timeout;
use std::time::Duration;
use uuid::Uuid;

/// Kafka producer
#[derive(Clone)]
pub struct KafkaProducer {
    producer: ThreadedProducer<BaseRecord<'static, Vec<u8>, Vec<u8>>>,
    config: KafkaConfig,
}

impl KafkaProducer {
    /// Create a new Kafka producer
    pub fn new(config: KafkaConfig) -> AppResult<Self> {
        let brokers = config.broker_list().join(",");

        let producer: ThreadedProducer<BaseRecord<'static, Vec<u8>, Vec<u8>>> = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .set("message.timeout.ms", &config.message_timeout.to_string())
            .set("queue.buffering.max.messages", "100000")
            .set("batch.size", "16384")
            .set("linger.ms", "10")
            .set("compression.type", "snappy")
            .create()
            .map_err(|e| AppError::internal_error(format!("Failed to create Kafka producer: {}", e)))?;

        Ok(Self { producer, config })
    }

    /// Publish a message to Kafka
    pub fn publish<K, P>(&self, topic: &str, key: Option<K>, payload: Option<P>) -> AppResult<()>
    where
        K: ToBytes,
        P: ToBytes,
    {
        let full_topic = self.config.producer_topic(topic);

        let mut record = BaseRecord::to(&full_topic);

        if let Some(k) = key {
            record = record.key(k);
        }

        if let Some(p) = payload {
            record = record.payload(p);
        }

        self.producer
            .send(record, Timeout::After(Duration::from_secs(5)))
            .map_err(|e| AppError::internal_error(format!("Failed to send Kafka message: {}", e)))?;

        Ok(())
    }

    /// Publish activity event
    pub fn publish_activity(&self, event: &crate::models::activity::ActivityKafkaEvent) -> AppResult<()> {
        let payload = event.to_json()?;
        let key = event.user_id.map(|id| id.to_string());

        self.publish("activity-events", key.as_deref(), Some(payload.as_bytes()))
    }

    /// Flush the producer
    pub fn flush(&self, timeout_ms: u64) -> AppResult<()> {
        self.producer
            .flush(Timeout::After(Duration::from_millis(timeout_ms)))
            .map_err(|e| AppError::internal_error(format!("Failed to flush Kafka producer: {}", e)))?;

        Ok(())
    }
}

/// Kafka consumer wrapper
#[cfg(feature = "kafka")]
pub struct KafkaConsumer {
    consumer: rdkafka::consumer::StreamConsumer<rdkafka::consumer::DefaultConsumerContext>,
    config: KafkaConfig,
}

#[cfg(feature = "kafka")]
impl KafkaConsumer {
    /// Create a new Kafka consumer
    pub fn new(config: &KafkaConfig) -> AppResult<Self> {
        use rdkafka::consumer::{CommitMode, Consumer};
        use rdkafka::MessageStream;

        let brokers = config.broker_list().join(",");
        let topics: Vec<&str> = config.topics.iter().map(|s| s.as_str()).collect();

        let consumer: rdkafka::consumer::StreamConsumer<_> = ClientConfig::new()
            .set("group.id", &config.group_id)
            .set("bootstrap.servers", &brokers)
            .set("session.timeout.ms", &config.session_timeout.to_string())
            .set("enable.auto.commit", &config.enable_auto_commit.to_string())
            .set("auto.offset.reset", &config.auto_offset_reset)
            .create()
            .map_err(|e| AppError::internal_error(format!("Failed to create Kafka consumer: {}", e)))?;

        if !topics.is_empty() {
            consumer
                .subscribe(&topics)
                .map_err(|e| AppError::internal_error(format!("Failed to subscribe to topics: {}", e)))?;
        }

        Ok(Self { consumer, config: config.clone() })
    }

    /// Get the consumer
    pub fn consumer(&self) -> &rdkafka::consumer::StreamConsumer<rdkafka::consumer::DefaultConsumerContext> {
        &self.consumer
    }

    /// Subscribe to topics
    pub fn subscribe(&self, topics: &[&str]) -> AppResult<()> {
        self.consumer
            .subscribe(topics)
            .map_err(|e| AppError::internal_error(format!("Failed to subscribe to topics: {}", e)))?;

        Ok(())
    }
}
