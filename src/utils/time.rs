//! Time utilities

use chrono::{DateTime, Duration, Utc};
use std::time::SystemTime;

/// Get current timestamp as Unix seconds
pub fn now_seconds() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Get current timestamp as Unix milliseconds
pub fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// Convert Unix seconds to DateTime
pub fn from_seconds(seconds: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(seconds, 0).unwrap_or_else(|| Utc::now())
}

/// Convert Unix milliseconds to DateTime
pub fn from_millis(millis: i64) -> DateTime<Utc> {
    let secs = millis / 1000;
    let nsecs = ((millis % 1000) * 1_000_000) as u32;
    DateTime::from_timestamp(secs, nsecs).unwrap_or_else(|| Utc::now())
}

/// Format DateTime as ISO 8601 string
pub fn format_iso(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

/// Parse ISO 8601 string to DateTime
pub fn parse_iso(s: &str) -> Result<DateTime<Utc>, String> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| format!("Invalid datetime format: {}", e))
}

/// Calculate expiration time from now
pub fn expires_in_seconds(seconds: i64) -> DateTime<Utc> {
    Utc::now() + Duration::seconds(seconds)
}

/// Calculate expiration time from now
pub fn expires_in_minutes(minutes: i64) -> DateTime<Utc> {
    Utc::now() + Duration::minutes(minutes)
}

/// Calculate expiration time from now
pub fn expires_in_hours(hours: i64) -> DateTime<Utc> {
    Utc::now() + Duration::hours(hours)
}

/// Calculate expiration time from now
pub fn expires_in_days(days: i64) -> DateTime<Utc> {
    Utc::now() + Duration::days(days)
}

/// Check if a datetime is in the past
pub fn is_past(dt: DateTime<Utc>) -> bool {
    dt < Utc::now()
}

/// Check if a datetime is in the future
pub fn is_future(dt: DateTime<Utc>) -> bool {
    dt > Utc::now()
}

/// Get human-readable duration
pub fn human_duration(seconds: i64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else if seconds < 86400 {
        format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
    } else {
        format!("{}d {}h", seconds / 86400, (seconds % 86400) / 3600)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_conversions() {
        let now = now_seconds();
        let dt = from_seconds(now);
        assert!(dt <= Utc::now());

        let millis = now_millis();
        let dt2 = from_millis(millis);
        assert!(dt2 <= Utc::now());
    }

    #[test]
    fn test_expiration() {
        let exp = expires_in_seconds(60);
        assert!(is_future(exp));
        assert!(!is_past(exp));
    }

    #[test]
    fn test_human_duration() {
        assert_eq!(human_duration(30), "30s");
        assert_eq!(human_duration(90), "1m 30s");
        assert_eq!(human_duration(3661), "1h 1m");
        assert_eq!(human_duration(86401), "1d 0h");
    }

    #[test]
    fn test_iso_formatting() {
        let now = Utc::now();
        let iso = format_iso(now);
        let parsed = parse_iso(&iso).unwrap();
        // Within a second due to formatting
        assert!(parsed - now < Duration::seconds(1));
    }
}
