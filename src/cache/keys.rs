//! Cache key definitions

/// Cache key prefixes
pub struct CacheKeys;

impl CacheKeys {
    /// Session cache key prefix
    pub const SESSION: &'static str = "session:";

    /// User cache key prefix
    pub const USER: &'static str = "user:";

    /// Rate limit key prefix
    pub const RATE_LIMIT: &'static str = "ratelimit:";

    /// Online users set
    pub const ONLINE_USERS: &'static str = "online:users";

    /// WebSocket connection prefix
    pub const WS_CONN: &'static str = "ws:conn:";

    /// Room members prefix
    pub const ROOM_MEMBERS: &'static str = "room:members:";

    /// Message cache prefix
    pub const MESSAGES: &'static str = "messages:";

    /// Room cache prefix
    pub const ROOM: &'static str = "room:";

    /// Null value marker for cache penetration prevention
    pub const NULL_VALUE: &'static str = "null";
}

/// Pub/Sub channels
pub struct PubSubChannels;

impl PubSubChannels {
    /// Message broadcast channel
    pub const MESSAGES: &'static str = "channel:messages";

    /// User presence channel
    pub const PRESENCE: &'static str = "channel:presence";

    /// System notifications channel
    pub const NOTIFICATIONS: &'static str = "channel:notifications";
}

impl CacheKeys {
    /// Generate session cache key
    pub fn session(token: &str) -> String {
        format!("{}{}", Self::SESSION, token)
    }

    /// Generate user cache key
    pub fn user(user_id: &str) -> String {
        format!("{}{}", Self::USER, user_id)
    }

    /// Generate rate limit key
    pub fn rate_limit(identifier: &str) -> String {
        format!("{}{}", Self::RATE_LIMIT, identifier)
    }

    /// Generate WebSocket connection key
    pub fn ws_connection(conn_id: &str) -> String {
        format!("{}{}", Self::WS_CONN, conn_id)
    }

    /// Generate room members key
    pub fn room_members(room_id: &str) -> String {
        format!("{}{}", Self::ROOM_MEMBERS, room_id)
    }

    /// Generate messages cache key for a room
    pub fn messages(room_id: &str) -> String {
        format!("{}{}", Self::MESSAGES, room_id)
    }

    /// Generate room cache key
    pub fn room(room_id: uuid::Uuid) -> String {
        format!("{}{}", Self::ROOM, room_id)
    }
}
