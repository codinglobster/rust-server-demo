//! Kafka configuration

use serde::{Deserialize, Serialize};

/// Kafka configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    /// Kafka broker addresses (comma-separated)
    pub brokers: String,
    /// Consumer group ID
    pub group_id: String,
    /// Session timeout in milliseconds
    pub session_timeout: u64,
    /// Auto offset reset (earliest, latest, none)
    pub auto_offset_reset: String,
    /// Enable auto commit
    pub enable_auto_commit: bool,
    /// Message timeout in seconds
    pub message_timeout: u64,
    /// Maximum number of messages in a single poll
    pub max_poll_records: usize,
    /// Topics to subscribe to (comma-separated)
    pub topics: Vec<String>,
    /// Producer topic prefixes
    pub producer_prefix: String,
}

impl Default for KafkaConfig {
    fn default() -> Self {
        Self {
            brokers: "localhost:9092".to_string(),
            group_id: "rust-server-group".to_string(),
            session_timeout: 10000,
            auto_offset_reset: "latest".to_string(),
            enable_auto_commit: true,
            message_timeout: 30,
            max_poll_records: 100,
            topics: vec![],
            producer_prefix: "rust-server".to_string(),
        }
    }
}

impl KafkaConfig {
    /// Get the broker list as a vector
    pub fn broker_list(&self) -> Vec<String> {
        self.brokers.split(',').map(|s| s.trim().to_string()).collect()
    }

    /// Create a producer topic name
    pub fn producer_topic(&self, topic: &str) -> String {
        format!("{}-{}", self.producer_prefix, topic)
    }
}
