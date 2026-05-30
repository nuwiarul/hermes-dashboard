use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::{
    extract::{ConnectInfo, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tokio::sync::Mutex;

/// Simple per-IP rate limiter
struct RateLimitEntry {
    count: u32,
    window_start: Instant,
}

pub struct RateLimitState {
    login_limiter: Mutex<HashMap<String, RateLimitEntry>>,
    api_limiter: Mutex<HashMap<String, RateLimitEntry>>,
    login_max: u32,
    login_window: Duration,
    api_max: u32,
    api_window: Duration,
}

impl RateLimitState {
    pub fn new(login_max: u32, api_max: u32) -> Self {
        Self {
            login_limiter: Mutex::new(HashMap::new()),
            api_limiter: Mutex::new(HashMap::new()),
            login_max,
            login_window: Duration::from_secs(60),
            api_max,
            api_window: Duration::from_secs(60),
        }
    }

    async fn check_rate_limit(
        &self,
        ip: &str,
        limiter: &Mutex<HashMap<String, RateLimitEntry>>,
        max: u32,
        window: Duration,
    ) -> bool {
        let mut map = limiter.lock().await;
        let now = Instant::now();

        let entry = map.entry(ip.to_string()).or_insert(RateLimitEntry {
            count: 0,
            window_start: now,
        });

        // Reset window if expired
        if now.duration_since(entry.window_start) >= window {
            entry.count = 0;
            entry.window_start = now;
        }

        entry.count += 1;
        entry.count <= max
    }
}

/// Rate limiting middleware for login endpoints (strict: 5/min)
pub async fn rate_limit_login(
    axum::extract::State(state): axum::extract::State<Arc<RateLimitState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let ip = addr.ip().to_string();

    if !state.check_rate_limit(&ip, &state.login_limiter, state.login_max, state.login_window).await {
        tracing::warn!("Rate limit exceeded for IP {} on login endpoint", ip);
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(request).await)
}

/// Rate limiting middleware for API endpoints (moderate: 60/min)
pub async fn rate_limit_api(
    axum::extract::State(state): axum::extract::State<Arc<RateLimitState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let ip = addr.ip().to_string();

    if !state.check_rate_limit(&ip, &state.api_limiter, state.api_max, state.api_window).await {
        tracing::warn!("Rate limit exceeded for IP {} on API endpoint", ip);
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_state_creation() {
        let state = RateLimitState::new(5, 60);
        assert_eq!(state.login_max, 5);
        assert_eq!(state.api_max, 60);
    }

    #[test]
    fn test_rate_limit_custom_values() {
        let state = RateLimitState::new(10, 120);
        assert_eq!(state.login_max, 10);
        assert_eq!(state.api_max, 120);
    }
}
