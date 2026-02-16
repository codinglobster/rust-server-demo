//! Rust Server Demo - A real-time/long-connection server
//!
//! This server provides:
//! - RESTful API with Axum
//! - WebSocket support for real-time communication
//! - PostgreSQL for persistent storage
//! - Redis for caching and session management
//! - Kafka for message queuing
//! - JWT authentication

pub mod cache;
pub mod config;
pub mod core;
pub mod database;
pub mod state;
#[cfg(feature = "kafka")]
pub mod kafka;
pub mod models;
pub mod services;
pub mod handlers;
pub mod routes;
pub mod websocket;
pub mod auth;
pub mod utils;
