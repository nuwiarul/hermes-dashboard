use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use anyhow::{Result, bail};

/// Claims in JWT token
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (username)
    pub sub: String,
    /// Issued at (Unix timestamp)
    pub iat: i64,
    /// Expiration (Unix timestamp)
    pub exp: i64,
    /// Token type: "access" or "refresh"
    pub token_type: String,
}

/// Configuration for JWT
pub struct JwtConfig {
    pub secret: String,
    pub access_duration: Duration,
    pub refresh_duration: Duration,
}

impl JwtConfig {
    pub fn from_env() -> Self {
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| {
                // Generate a random secret if not set (for development only)
                use std::time::{SystemTime, UNIX_EPOCH};
                let seed = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos();
                format!("dev-secret-{}", seed)
            });

        Self {
            secret,
            access_duration: Duration::minutes(15),
            refresh_duration: Duration::days(7),
        }
    }
}

/// Generate access token (short-lived, 15 minutes)
pub fn generate_access_token(username: &str, config: &JwtConfig) -> Result<String> {
    let now = Utc::now();
    let claims = Claims {
        sub: username.to_string(),
        iat: now.timestamp(),
        exp: (now + config.access_duration).timestamp(),
        token_type: "access".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )?;

    Ok(token)
}

/// Generate refresh token (long-lived, 7 days)
pub fn generate_refresh_token(username: &str, config: &JwtConfig) -> Result<String> {
    let now = Utc::now();
    let claims = Claims {
        sub: username.to_string(),
        iat: now.timestamp(),
        exp: (now + config.refresh_duration).timestamp(),
        token_type: "refresh".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )?;

    Ok(token)
}

/// Validate and decode a JWT token
pub fn validate_token(token: &str, config: &JwtConfig, expected_type: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_bytes()),
        &Validation::default(),
    )?;

    if token_data.claims.token_type != expected_type {
        bail!("Invalid token type: expected '{}', got '{}'", expected_type, token_data.claims.token_type);
    }

    Ok(token_data.claims)
}

/// Validate access token (convenience function)
pub fn validate_access_token(token: &str, config: &JwtConfig) -> Result<Claims> {
    validate_token(token, config, "access")
}

/// Validate refresh token (convenience function)
pub fn validate_refresh_token(token: &str, config: &JwtConfig) -> Result<Claims> {
    validate_token(token, config, "refresh")
}

/// Generate both access and refresh tokens
pub fn generate_token_pair(username: &str, config: &JwtConfig) -> Result<(String, String)> {
    let access = generate_access_token(username, config)?;
    let refresh = generate_refresh_token(username, config)?;
    Ok((access, refresh))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_access_token() {
        let config = JwtConfig {
            secret: "test-secret-key-for-jwt".to_string(),
            access_duration: Duration::minutes(15),
            refresh_duration: Duration::days(7),
        };

        let token = generate_access_token("testuser", &config).unwrap();
        let claims = validate_access_token(&token, &config).unwrap();

        assert_eq!(claims.sub, "testuser");
        assert_eq!(claims.token_type, "access");
    }

    #[test]
    fn test_generate_and_validate_refresh_token() {
        let config = JwtConfig {
            secret: "test-secret-key-for-jwt".to_string(),
            access_duration: Duration::minutes(15),
            refresh_duration: Duration::days(7),
        };

        let token = generate_refresh_token("testuser", &config).unwrap();
        let claims = validate_refresh_token(&token, &config).unwrap();

        assert_eq!(claims.sub, "testuser");
        assert_eq!(claims.token_type, "refresh");
    }

    #[test]
    fn test_wrong_token_type_rejected() {
        let config = JwtConfig {
            secret: "test-secret-key-for-jwt".to_string(),
            access_duration: Duration::minutes(15),
            refresh_duration: Duration::days(7),
        };

        let token = generate_access_token("testuser", &config).unwrap();
        let result = validate_refresh_token(&token, &config);

        assert!(result.is_err());
    }
}
