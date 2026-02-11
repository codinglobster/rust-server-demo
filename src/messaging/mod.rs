// Messaging module exports
#[cfg(feature = "kafka")]
pub mod producer;
#[cfg(feature = "kafka")]
pub mod consumer;
pub mod topics;

#[cfg(feature = "kafka")]
pub use producer::KafkaProducer;
#[cfg(feature = "kafka")]
pub use consumer::KafkaConsumer;
