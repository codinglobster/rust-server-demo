//! Kafka consumer

use crate::config::kafka::KafkaConfig;
use crate::core::error::{AppError, AppResult};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use std::time::Duration;

/// Kafka consumer wrapper
pub struct KafkaConsumer {
    consumer: StreamConsumer,
}

impl KafkaConsumer {
    /// Create a new Kafka consumer
    pub fn new(config: &KafkaConfig) -> AppResult<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("group.id", &config.group_id)
            .set("session.timeout.ms", &config.session_timeout.to_string())
            .set("auto.offset.reset", &config.auto_offset_reset)
            .set("enable.auto.commit", &config.enable_auto_commit.to_string())
            .create()
            .map_err(|e| AppError::Kafka(e.to_string()))?;

        tracing::info!("Kafka consumer created: {}", config.brokers);

        Ok(Self { consumer })
    }

    /// Subscribe to topics
    pub fn subscribe(&self, topics: &[&str]) -> AppResult<()> {
        self.consumer
            .subscribe(topics)
            .map_err(|e| AppError::Kafka(e.to_string()))?;

        tracing::info!("Subscribed to topics: {:?}", topics);
        Ok(())
    }

    /// Get the underlying consumer
    pub fn consumer(&self) -> &StreamConsumer {
        &self.consumer
    }

    /// Split the consumer into a stream
    pub fn split(self) -> StreamConsumer {
        self.consumer
    }

    /// Receive a message with timeout
    pub async fn recv(&self) -> AppResult<Option<rdkafka::message::BorrowedMessage<'_>>> {
        match self.consumer.recv(Duration::from_secs(1)) {
            Ok(msg) => Ok(Some(msg)),
            Err(e) if e == rdkafka::error::KafkaError::PartitionEOF(_) => Ok(None),
            Err(e) if e.to_string().contains("No message") => Ok(None),
            Err(e) => Err(AppError::Kafka(e.to_string())),
        }
    }

    /// Get consumer position
    pub fn position(&self) -> AppResult<Vec<rdkafka::TopicPartitionList>> {
        self.consumer
            .position()
            .map_err(|e| AppError::Kafka(e.to_string()))
    }

    /// Commit offsets
    pub fn commit(&self) -> AppResult<()> {
        self.consumer
            .commit()
            .map_err(|e| AppError::Kafka(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kafka_topics() {
        let topics = crate::messaging::topics::KafkaTopics::all_topics();
        assert!(topics.contains(&crate::messaging::topics::KafkaTopics::USER_EVENTS));
        assert!(topics.contains(&crate::messaging::topics::KafkaTopics::MESSAGES));
    }
}
