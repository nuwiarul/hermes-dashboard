/// Input validation and sanitization utilities

/// Validation error details
#[derive(Debug)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: &str, message: &str) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

/// Sanitize string input — trim whitespace, remove null bytes
pub fn sanitize_string(input: &str) -> String {
    input
        .trim()
        .replace('\0', "") // Remove null bytes
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t') // Allow newlines/tabs
        .collect()
}

/// Validate username
/// Rules: non-empty, 3-50 chars, alphanumeric + underscore only
pub fn validate_username(username: &str) -> Result<String, ValidationError> {
    let sanitized = sanitize_string(username);

    if sanitized.is_empty() {
        return Err(ValidationError::new("username", "Username is required"));
    }

    if sanitized.len() < 3 {
        return Err(ValidationError::new(
            "username",
            "Username must be at least 3 characters",
        ));
    }

    if sanitized.len() > 50 {
        return Err(ValidationError::new(
            "username",
            "Username must be at most 50 characters",
        ));
    }

    // Only allow alphanumeric and underscore
    if !sanitized.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(ValidationError::new(
            "username",
            "Username can only contain letters, numbers, and underscores",
        ));
    }

    Ok(sanitized)
}

/// Validate password
/// Rules: non-empty, 8-100 chars
pub fn validate_password(password: &str) -> Result<String, ValidationError> {
    let sanitized = sanitize_string(password);

    if sanitized.is_empty() {
        return Err(ValidationError::new("password", "Password is required"));
    }

    if sanitized.len() < 8 {
        return Err(ValidationError::new(
            "password",
            "Password must be at least 8 characters",
        ));
    }

    if sanitized.len() > 100 {
        return Err(ValidationError::new(
            "password",
            "Password must be at most 100 characters",
        ));
    }

    Ok(sanitized)
}

/// Validate generic text input (for future use)
pub fn validate_text(input: &str, field: &str, max_len: usize) -> Result<String, ValidationError> {
    let sanitized = sanitize_string(input);

    if sanitized.is_empty() {
        return Err(ValidationError::new(
            field,
            &format!("{} is required", field),
        ));
    }

    if sanitized.len() > max_len {
        return Err(ValidationError::new(
            field,
            &format!("{} must be at most {} characters", field, max_len),
        ));
    }

    Ok(sanitized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_string() {
        assert_eq!(sanitize_string("  hello  "), "hello");
        assert_eq!(sanitize_string("hel\x00lo"), "hello");
        assert_eq!(sanitize_string("test\ninput"), "test\ninput");
    }

    #[test]
    fn test_validate_username_valid() {
        assert!(validate_username("admin").is_ok());
        assert!(validate_username("user_123").is_ok());
        assert!(validate_username("  admin  ").is_ok()); // trimmed
    }

    #[test]
    fn test_validate_username_invalid() {
        assert!(validate_username("").is_err());
        assert!(validate_username("ab").is_err()); // too short
        assert!(validate_username("user name").is_err()); // space
        assert!(validate_username("user@name").is_err()); // special char
    }

    #[test]
    fn test_validate_password_valid() {
        assert!(validate_password("password123").is_ok());
        assert!(validate_password("  mypassword  ").is_ok()); // trimmed to 10 chars
    }

    #[test]
    fn test_validate_password_invalid() {
        assert!(validate_password("").is_err());
        assert!(validate_password("short").is_err()); // too short
    }

    #[test]
    fn test_validate_text() {
        assert!(validate_text("hello", "name", 10).is_ok());
        assert!(validate_text("", "name", 10).is_err()); // empty
        assert!(validate_text("a".repeat(11).as_str(), "name", 10).is_err()); // too long
    }

    #[test]
    fn test_validate_username_max_length() {
        let long_name = "a".repeat(51);
        assert!(validate_username(&long_name).is_err());
        
        let max_name = "a".repeat(50);
        assert!(validate_username(&max_name).is_ok());
    }

    #[test]
    fn test_validate_password_max_length() {
        let long_pass = "a".repeat(101);
        assert!(validate_password(&long_pass).is_err());
        
        let max_pass = "a".repeat(100);
        assert!(validate_password(&max_pass).is_ok());
    }

    #[test]
    fn test_sanitize_removes_control_chars() {
        let input = "hello\x01\x02\x03world";
        let result = sanitize_string(input);
        assert_eq!(result, "helloworld");
    }

    #[test]
    fn test_sanitize_allows_newlines_and_tabs() {
        let input = "hello\nworld\t!";
        let result = sanitize_string(input);
        assert_eq!(result, "hello\nworld\t!");
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::new("field", "message");
        assert_eq!(format!("{}", err), "field: message");
    }
}
