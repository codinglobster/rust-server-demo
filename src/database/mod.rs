// Database module exports
pub mod connection;
pub mod repositories;
pub mod schema;

// Re-export for convenience
pub use connection::{create_pool, Database};
