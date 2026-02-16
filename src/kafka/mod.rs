//! Kafka module for event streaming

pub mod producer;

#[cfg(feature = "kafka")]
pub use producer::{KafkaConsumer, KafkaProducer};
