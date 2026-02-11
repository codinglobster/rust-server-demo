//! Validation utilities

use crate::core::error::ValidationError;
use regex::Regex;
use validator::Validate;

lazy_static::lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();

    static ref USERNAME_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9_]{3,50}$"
    ).unwrap();

    static ref UUID_REGEX: Regex = Regex::new(
        r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
    ).unwrap();
}

/// Validate an email address
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    if email.is_empty() {
        return Err(ValidationError::MissingField("email".to_string()));
    }

    if !EMAIL_REGEX.is_match(email) {
        return Err(ValidationError::InvalidEmail);
    }

    Ok(())
}

/// Validate a username
pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.is_empty() {
        return Err(ValidationError::MissingField("username".to_string()));
    }

    if username.len() < 3 || username.len() > 50 {
        return Err(ValidationError::InvalidUsername);
    }

    if !USERNAME_REGEX.is_match(username) {
        return Err(ValidationError::InvalidUsername);
    }

    Ok(())
}

/// Validate a password
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.is_empty() {
        return Err(ValidationError::MissingField("password".to_string()));
    }

    if password.len() < 8 {
        return Err(ValidationError::InvalidPassword(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    Ok(())
}

/// Validate a UUID string
pub fn validate_uuid(s: &str) -> Result<(), ValidationError> {
    if s.is_empty() {
        return Err(ValidationError::MissingField("id".to_string()));
    }

    if !UUID_REGEX.is_match(s) {
        return Err(ValidationError::InvalidInput("Invalid UUID format".to_string()));
    }

    Ok(())
}

/// Sanitize a string (remove HTML tags, etc.)
pub fn sanitize_string(input: &str) -> String {
    ammonia::clean(input)
}

/// Validate and sanitize user input
pub fn validate_input(input: &str, max_length: usize) -> Result<String, ValidationError> {
    if input.is_empty() {
        return Err(ValidationError::MissingField("input".to_string()));
    }

    if input.len() > max_length {
        return Err(ValidationError::InvalidInput(format!(
            "Input too long (max {} characters)",
            max_length
        )));
    }

    Ok(sanitize_string(input))
}

/// Validate pagination parameters
pub fn validate_pagination(page: u64, per_page: u64) -> Result<(u64, u64), ValidationError> {
    if page == 0 {
        return Err(ValidationError::InvalidInput("Page must be >= 1".to_string()));
    }

    if per_page == 0 || per_page > 100 {
        return Err(ValidationError::InvalidInput(
            "Per page must be between 1 and 100".to_string(),
        ));
    }

    Ok((page, per_page))
}

/// Check for SQL injection patterns
pub fn is_sql_injection(input: &str) -> bool {
    let dangerous_patterns = [
        "--", ";--", "/*", "*/", "xp_", "exec(", "execute(", "cast(", "varchar",
        "nvarchar(", "char(", "nchar(", "ascii(", "substring(", "waitfor delay",
        "union select", "drop table", "delete from", "insert into", "update",
    ];

    let lower = input.to_lowercase();
    for pattern in dangerous_patterns {
        if lower.contains(pattern) {
            return true;
        }
    }

    false
}

/// Check for XSS patterns
pub fn is_xss(input: &str) -> bool {
    let dangerous_patterns = [
        "<script", "javascript:", "onerror=", "onload=", "onclick=",
        "<iframe", "<object", "<embed", "onfocus=", "onblur=", "onmouseover=",
    ];

    let lower = input.to_lowercase();
    for pattern in dangerous_patterns {
        if lower.contains(pattern) {
            return true;
        }
    }

    false
}

/// Validate request using the validator crate
pub fn validate_request<T: Validate>(request: &T) -> Result<(), ValidationError> {
    request
        .validate()
        .map_err(|e| ValidationError::InvalidInput(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("test.user+tag@example.co.uk").is_ok());
        assert!(validate_email("invalid").is_err());
        assert!(validate_email("@example.com").is_err());
    }

    #[test]
    fn test_username_validation() {
        assert!(validate_username("user123").is_ok());
        assert!(validate_username("test_user").is_ok());
        assert!(validate_username("ab").is_err()); // Too short
        assert!(validate_username("user@name").is_err()); // Invalid chars
    }

    #[test]
    fn test_password_validation() {
        assert!(validate_password("password123").is_ok());
        assert!(validate_password("short").is_err()); // Too short
    }

    #[test]
    fn test_uuid_validation() {
        assert!(validate_uuid("550e8400-e29b-41d4-a716-446655440000").is_ok());
        assert!(validate_uuid("invalid").is_err());
    }

    #[test]
    fn test_pagination_validation() {
        assert!(validate_pagination(1, 20).is_ok());
        assert!(validate_pagination(0, 20).is_err()); // Page 0
        assert!(validate_pagination(1, 0).is_err()); // Per page 0
        assert!(validate_pagination(1, 101).is_err()); // Per page > 100
    }

    #[test]
    fn test_sql_injection_detection() {
        assert!(is_sql_injection("'; DROP TABLE users; --"));
        assert!(is_sql_injection("1' OR '1'='1"));
        assert!(!is_sql_injection("normal text"));
    }
}
