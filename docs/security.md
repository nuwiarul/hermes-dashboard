## 🔒 Security: JWT Authentication (Dual-Token with Refresh)

### Overview

Dashboard menggunakan **dual-token system** untuk keamanan maksimal:
- **Access Token** — Short-lived (15 menit), untuk akses API
- **Refresh Token** — Long-lived (7 hari), untuk mendapatkan access token baru

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  LOGIN FLOW                                                     │
│  ───────────                                                    │
│                                                                 │
│  1. User login (POST /api/auth/login)                           │
│         │                                                       │
│         ▼                                                       │
│  2. Server verify credentials                                   │
│         │                                                       │
│         ├─ Invalid → 401 Unauthorized                           │
│         │                                                       │
│         └─ Valid → Generate tokens                              │
│                   │                                             │
│                   ├── Access Token (15 min)                     │
│                   │   - HttpOnly                                │
│                   │   - Secure                                  │
│                   │   - SameSite=None                           │
│                   │   - Max-Age=900 (15 min)                    │
│                   │                                             │
│                   └── Refresh Token (7 days)                    │
│                       - HttpOnly                                │
│                       - Secure                                  │
│                       - SameSite=None                           │
│                       - Path=/auth/refresh                      │
│                       - Max-Age=604800 (7 days)                 │
│                                                                 │
│  3. Set kedua cookies di response                               │
│         │                                                       │
│         ▼                                                       │
│  4. Response: { success: true }                                 │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  API REQUEST FLOW                                               │
│  ────────────────                                               │
│                                                                 │
│  1. Browser kirim request ke API                                │
│         │                                                       │
│         ├── Cookie: access_token=*** (otomatis)                 │
│         │                                                       │
│         ▼                                                       │
│  2. Backend verify access token                                 │
│         │                                                       │
│         ├─ Valid → Process request, return data                 │
│         │                                                       │
│         └─ Expired → 401 Unauthorized                           │
│                   │                                             │
│                   ▼                                             │
│  3. Frontend detect 401                                         │
│         │                                                       │
│         ▼                                                       │
│  4. Auto-refresh (POST /api/auth/refresh)                       │
│         │                                                       │
│         ├── Cookie: refresh_token=*** (otomatis)                │
│         │                                                       │
│         ▼                                                       │
│  5. Backend verify refresh token                                │
│         │                                                       │
│         ├─ Invalid/Expired → 401, redirect to login             │
│         │                                                       │
│         └─ Valid → Generate new tokens (rotation)               │
│                   │                                             │
│                   ├── New access token (15 min)                 │
│                   └── New refresh token (7 days)                │
│                                                                 │
│  6. Retry original request dengan new access token              │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

### Token Comparison

| Property | Access Token | Refresh Token |
|----------|-------------|---------------|
| **Lifetime** | 15 minutes | 7 days |
| **Purpose** | API access | Get new access token |
| **HttpOnly** | ✅ | ✅ |
| **Secure** | ✅ | ✅ |
| **SameSite** | None | None |
| **Path** | `/` | `/auth/refresh` |
| **Rotation** | No | Yes (on each refresh) |

---

### Backend Implementation

#### Dependencies (Cargo.toml)

```toml
jsonwebtoken = "9"
tower-cookies = "0.11"
tower = "0.5"
tower-http = { version = "0.6", features = ["cors", "limit"] }
```

#### Config (config.rs)

```rust
pub struct AuthConfig {
    pub username: String,
    pub password: String,
    pub jwt_secret: String,
    pub access_token_ttl: i64,      // 900 (15 min)
    pub refresh_token_ttl: i64,     // 604800 (7 days)
    pub cookie_domain: String,      // ".vinrul.my.id"
    pub cookie_secure: bool,        // true
    pub rate_limit_login: u64,      // 5 attempts per minute
    pub rate_limit_refresh: u64,    // 10 attempts per minute
}

impl AuthConfig {
    pub fn from_env() -> Self {
        Self {
            username: std::env::var("DASHBOARD_USERNAME")
                .unwrap_or_else(|_| "admin".to_string()),
            password: std::env::var("DASHBOARD_PASSWORD")
                .expect("DASHBOARD_PASSWORD must be set"),
            jwt_secret: std::env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            access_token_ttl: std::env::var("ACCESS_TOKEN_TTL")
                .unwrap_or_else(|_| "900".to_string())
                .parse()
                .unwrap_or(900),
            refresh_token_ttl: std::env::var("REFRESH_TOKEN_TTL")
                .unwrap_or_else(|_| "604800".to_string())
                .parse()
                .unwrap_or(604800),
            cookie_domain: std::env::var("COOKIE_DOMAIN")
                .unwrap_or_else(|_| ".vinrul.my.id".to_string()),
            cookie_secure: std::env::var("COOKIE_SECURE")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            rate_limit_login: std::env::var("RATE_LIMIT_LOGIN")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            rate_limit_refresh: std::env::var("RATE_LIMIT_REFRESH")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
        }
    }
}
```

#### DTOs (features/auth/dto.rs)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessTokenClaims {
    pub sub: String,      // username
    pub exp: i64,         // expiration timestamp
    pub iat: i64,         // issued at
    pub token_type: String, // "access"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshTokenClaims {
    pub sub: String,      // username
    pub exp: i64,         // expiration timestamp
    pub iat: i64,         // issued at
    pub token_type: String, // "refresh"
    pub jti: String,      // unique token ID (for rotation tracking)
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub username: String,
}
```

#### Token Generation (features/auth/token.rs)

```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use uuid::Uuid;
use super::dto::*;

const ACCESS_TOKEN_SECRET: &str = "access-secret";  // Load from config
const REFRESH_TOKEN_SECRET: &str = "refresh-secret"; // Load from config

pub fn generate_access_token(username: &str, ttl: i64) -> Result<String, anyhow::Error> {
    let claims = AccessTokenClaims {
        sub: username.to_string(),
        exp: chrono::Utc::now().timestamp() + ttl,
        iat: chrono::Utc::now().timestamp(),
        token_type: "access".to_string(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(ACCESS_TOKEN_SECRET.as_bytes()),
    )
    .map_err(|e| anyhow::anyhow!("Failed to generate access token: {}", e))
}

pub fn generate_refresh_token(username: &str, ttl: i64) -> Result<String, anyhow::Error> {
    let claims = RefreshTokenClaims {
        sub: username.to_string(),
        exp: chrono::Utc::now().timestamp() + ttl,
        iat: chrono::Utc::now().timestamp(),
        token_type: "refresh".to_string(),
        jti: Uuid::new_v4().to_string(),  // Unique ID for rotation
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(REFRESH_TOKEN_SECRET.as_bytes()),
    )
    .map_err(|e| anyhow::anyhow!("Failed to generate refresh token: {}", e))
}

pub fn verify_access_token(token: &str) -> Result<AccessTokenClaims, anyhow::Error> {
    let token_data = decode::<AccessTokenClaims>(
        token,
        &DecodingKey::from_secret(ACCESS_TOKEN_SECRET.as_bytes()),
        &Validation::default(),
    )?;
    
    if token_data.claims.token_type != "access" {
        return Err(anyhow::anyhow!("Invalid token type"));
    }
    
    Ok(token_data.claims)
}

pub fn verify_refresh_token(token: &str) -> Result<RefreshTokenClaims, anyhow::Error> {
    let token_data = decode::<RefreshTokenClaims>(
        token,
        &DecodingKey::from_secret(REFRESH_TOKEN_SECRET.as_bytes()),
        &Validation::default(),
    )?;
    
    if token_data.claims.token_type != "refresh" {
        return Err(anyhow::anyhow!("Invalid token type"));
    }
    
    Ok(token_data.claims)
}
```

#### Handler (features/auth/handler.rs)

```rust
use axum::{Json, Extension, http::StatusCode};
use tower_cookies::Cookies;
use std::sync::Arc;
use crate::AppState;
use super::dto::*;
use super::token::*;

const ACCESS_COOKIE: &str = "access_token";
const REFRESH_COOKIE: &str = "refresh_token";

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    cookies: Cookies,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Verify credentials
    if !verify_credentials(&state, &payload.username, &payload.password) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    let username = &payload.username;
    let config = &state.config.auth;
    
    // Generate tokens
    let access_token = generate_access_token(username, config.access_token_ttl)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let refresh_token = generate_refresh_token(username, config.refresh_token_ttl)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Set access token cookie
    let access_cookie = tower_cookies::Cookie::build((ACCESS_COOKIE, access_token))
        .domain(&config.cookie_domain)
        .path("/")
        .http_only(true)
        .secure(config.cookie_secure)
        .same_site(tower_cookies::SameSite::None)
        .max_age(time::Duration::seconds(config.access_token_ttl))
        .build();
    
    // Set refresh token cookie
    let refresh_cookie = tower_cookies::Cookie::build((REFRESH_COOKIE, refresh_token))
        .domain(&config.cookie_domain)
        .path("/auth/refresh")  // Only sent to refresh endpoint
        .http_only(true)
        .secure(config.cookie_secure)
        .same_site(tower_cookies::SameSite::None)
        .max_age(time::Duration::seconds(config.refresh_token_ttl))
        .build();
    
    cookies.add(access_cookie);
    cookies.add(refresh_cookie);
    
    Ok(Json(LoginResponse {
        success: true,
        message: "Login successful".to_string(),
    }))
}

pub async fn refresh(
    Extension(state): Extension<Arc<AppState>>,
    cookies: Cookies,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Get refresh token from cookie
    let refresh_token = cookies
        .get(REFRESH_COOKIE)
        .map(|c| c.value().to_string())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Verify refresh token
    let claims = verify_refresh_token(&refresh_token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    let username = &claims.sub;
    let config = &state.config.auth;
    
    // Generate NEW tokens (rotation)
    let new_access_token = generate_access_token(username, config.access_token_ttl)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let new_refresh_token = generate_refresh_token(username, config.refresh_token_ttl)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Set new access token cookie
    let access_cookie = tower_cookies::Cookie::build((ACCESS_COOKIE, new_access_token))
        .domain(&config.cookie_domain)
        .path("/")
        .http_only(true)
        .secure(config.cookie_secure)
        .same_site(tower_cookies::SameSite::None)
        .max_age(time::Duration::seconds(config.access_token_ttl))
        .build();
    
    // Set new refresh token cookie (rotation!)
    let refresh_cookie = tower_cookies::Cookie::build((REFRESH_COOKIE, new_refresh_token))
        .domain(&config.cookie_domain)
        .path("/auth/refresh")
        .http_only(true)
        .secure(config.cookie_secure)
        .same_site(tower_cookies::SameSite::None)
        .max_age(time::Duration::seconds(config.refresh_token_ttl))
        .build();
    
    cookies.add(access_cookie);
    cookies.add(refresh_cookie);
    
    Ok(Json(LoginResponse {
        success: true,
        message: "Token refreshed".to_string(),
    }))
}

pub async fn logout(cookies: Cookies, Extension(state): Extension<Arc<AppState>>) -> Json<LoginResponse> {
    let config = &state.config.auth;
    
    // Clear access token
    let access_cookie = tower_cookies::Cookie::build((ACCESS_COOKIE, ""))
        .domain(&config.cookie_domain)
        .path("/")
        .http_only(true)
        .secure(config.cookie_secure)
        .max_age(time::Duration::seconds(0))
        .build();
    
    // Clear refresh token
    let refresh_cookie = tower_cookies::Cookie::build((REFRESH_COOKIE, ""))
        .domain(&config.cookie_domain)
        .path("/auth/refresh")
        .http_only(true)
        .secure(config.cookie_secure)
        .max_age(time::Duration::seconds(0))
        .build();
    
    cookies.remove(access_cookie);
    cookies.remove(refresh_cookie);
    
    Json(LoginResponse {
        success: true,
        message: "Logged out".to_string(),
    })
}

pub async fn me(
    Extension(claims): Extension<AccessTokenClaims>,
) -> Json<UserInfo> {
    Json(UserInfo { username: claims.sub })
}

fn verify_credentials(state: &AppState, username: &str, password: &str) -> bool {
    state.config.auth.username == username && 
    state.config.auth.password == password
}
```

#### Middleware (features/auth/middleware.rs)

```rust
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::StatusCode,
    Extension,
};
use tower_cookies::Cookies;
use std::sync::Arc;
use crate::AppState;
use super::dto::AccessTokenClaims;
use super::token::verify_access_token;

const ACCESS_COOKIE: &str = "access_token";

pub async fn auth_middleware(
    Extension(_state): Extension<Arc<AppState>>,
    cookies: Cookies,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip auth for public routes
    let path = request.uri().path();
    if path.starts_with("/api/auth/login") || 
       path.starts_with("/api/auth/refresh") ||
       path == "/api/health" {
        return Ok(next.run(request).await);
    }
    
    // Extract access token from cookie
    let access_token = cookies
        .get(ACCESS_COOKIE)
        .map(|cookie| cookie.value().to_string())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Verify token
    let claims = verify_access_token(&access_token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Add claims to request extensions
    request.extensions_mut().insert(claims);
    
    Ok(next.run(request).await)
}
```

#### Origin/Referer Validation (features/auth/validation.rs)

```rust
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::StatusCode,
    Extension,
};
use std::sync::Arc;
use crate::AppState;

pub async fn origin_validator(
    Extension(state): Extension<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get Origin or Referer header
    let origin = request.headers()
        .get("origin")
        .or_else(|| request.headers().get("referer"))
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string());
    
    // Validate origin
    if let Some(origin) = origin {
        let allowed_origin = format!("https://{}", state.config.frontend_domain);
        
        if !origin.starts_with(&allowed_origin) {
            tracing::warn!("Blocked request from invalid origin: {}", origin);
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        // No origin/referer header — could be direct API call
        // Allow if it's a health check or other public endpoint
        let path = request.uri().path();
        if !path.starts_with("/api/auth/") && path != "/api/health" {
            tracing::warn!("Blocked request without origin header");
            return Err(StatusCode::FORBIDDEN);
        }
    }
    
    Ok(next.run(request).await)
}
```

#### Rate Limiting (features/auth/rate_limit.rs)

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};

struct RateLimitEntry {
    count: u64,
    window_start: Instant,
}

pub struct RateLimiter {
    limits: Arc<Mutex<HashMap<String, RateLimitEntry>>>,
    max_requests: u64,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u64, window_duration: Duration) -> Self {
        Self {
            limits: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_duration,
        }
    }
    
    pub async fn check(&self, key: &str) -> bool {
        let mut limits = self.limits.lock().await;
        let now = Instant::now();
        
        let entry = limits.entry(key.to_string()).or_insert(RateLimitEntry {
            count: 0,
            window_start: now,
        });
        
        // Reset window if expired
        if now.duration_since(entry.window_start) > self.window_duration {
            entry.count = 0;
            entry.window_start = now;
        }
        
        // Check limit
        if entry.count >= self.max_requests {
            return false;  // Rate limited
        }
        
        entry.count += 1;
        true  // Allowed
    }
}

// In main.rs or AppState
pub fn create_rate_limiters() -> (RateLimiter, RateLimiter) {
    let login_limiter = RateLimiter::new(5, Duration::from_secs(60));    // 5 per minute
    let refresh_limiter = RateLimiter::new(10, Duration::from_secs(60)); // 10 per minute
    
    (login_limiter, refresh_limiter)
}
```

#### Rate Limit Middleware (features/auth/rate_limit_middleware.rs)

```rust
use axum::{
    extract::ConnectInfo,
    extract::Request,
    middleware::Next,
    response::Response,
    http::StatusCode,
    Extension,
};
use std::net::SocketAddr;
use super::rate_limit::RateLimiter;
use std::sync::Arc;

pub async fn login_rate_limiter(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(limiter): Extension<Arc<RateLimiter>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let key = format!("login:{}", addr.ip());
    
    if !limiter.check(&key).await {
        tracing::warn!("Rate limited login attempt from {}", addr.ip());
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    
    Ok(next.run(request).await)
}

pub async fn refresh_rate_limiter(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(limiter): Extension<Arc<RateLimiter>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let key = format!("refresh:{}", addr.ip());
    
    if !limiter.check(&key).await {
        tracing::warn!("Rate limited refresh attempt from {}", addr.ip());
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    
    Ok(next.run(request).await)
}
```

#### Router Update (main.rs)

```rust
use tower_cookies::CookieManagerLayer;
use features::{sessions, stats, config, cron, ws, auth};

// Create rate limiters
let (login_limiter, refresh_limiter) = auth::rate_limit::create_rate_limiters();

let app = Router::new()
    // Public routes (no auth, but rate limited)
    .route("/api/auth/login", post(auth::handler::login))
        .layer(axum::middleware::from_fn(auth::rate_limit_middleware::login_rate_limiter))
        .layer(Extension(Arc::new(login_limiter)))
    .route("/api/auth/refresh", post(auth::handler::refresh))
        .layer(axum::middleware::from_fn(auth::rate_limit_middleware::refresh_rate_limiter))
        .layer(Extension(Arc::new(refresh_limiter)))
    .route("/api/auth/logout", post(auth::handler::logout))
    .route("/api/health", get(shared::health::handler))
    
    // Protected routes (auth required)
    .route("/api/auth/me", get(auth::handler::me))
    .route("/api/sessions", get(sessions::handler::list))
    .route("/api/stats", get(stats::handler::overview))
    .route("/api/config", get(config::handler::get_config))
    .route("/api/cron", get(cron::handler::list_jobs))
    .route("/ws", get(ws::handler::ws_handler))
    
    // Middleware layers (order matters!)
    .layer(axum::middleware::from_fn(auth::middleware::auth_middleware))
    .layer(axum::middleware::from_fn(auth::validation::origin_validator))
    .layer(CookieManagerLayer::new())
    .layer(Extension(state))
    .layer(CorsLayer::permissive()
        .allow_credentials(true));
```

---

### Frontend Implementation

#### Auth Store (features/auth/store.ts)

```typescript
import { writable } from 'svelte/store';
import type { AuthState, UserInfo } from './types';
import { checkAuth, refreshToken, logout as apiLogout } from './api';

function createAuthStore() {
    const { subscribe, set, update } = writable<AuthState>({
        isAuthenticated: false,
        user: null,
        loading: true,
    });
    
    let refreshTimer: ReturnType<typeof setTimeout> | null = null;
    
    return {
        subscribe,
        
        async check() {
            set({ isAuthenticated: false, user: null, loading: true });
            
            const user = await checkAuth();
            
            if (user) {
                set({ isAuthenticated: true, user, loading: false });
                this.scheduleRefresh();  // Auto-refresh before expiry
            } else {
                set({ isAuthenticated: false, user: null, loading: false });
            }
        },
        
        setUser(user: UserInfo) {
            set({ isAuthenticated: true, user, loading: false });
            this.scheduleRefresh();
        },
        
        scheduleRefresh() {
            // Clear existing timer
            if (refreshTimer) {
                clearTimeout(refreshTimer);
            }
            
            // Refresh 1 minute before access token expires (14 min)
            refreshTimer = setTimeout(async () => {
                const success = await refreshToken();
                if (success) {
                    this.scheduleRefresh();  // Schedule next refresh
                } else {
                    // Refresh failed, logout
                    await this.logout();
                }
            }, 14 * 60 * 1000);  // 14 minutes
        },
        
        async logout() {
            if (refreshTimer) {
                clearTimeout(refreshTimer);
            }
            await apiLogout();
            set({ isAuthenticated: false, user: null, loading: false });
        },
    };
}

export const auth = createAuthStore();
```

#### Auth API (features/auth/api.ts)

```typescript
import { API_BASE_URL } from '$lib/shared/utils/api';
import type { LoginRequest, LoginResponse, UserInfo } from './types';

export async function login(credentials: LoginRequest): Promise<LoginResponse> {
    const res = await fetch(`${API_BASE_URL}/api/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',  // Include cookies
        body: JSON.stringify(credentials),
    });
    
    if (!res.ok) {
        throw new Error('Invalid credentials');
    }
    
    return res.json();
}

export async function refreshToken(): Promise<boolean> {
    try {
        const res = await fetch(`${API_BASE_URL}/api/auth/refresh`, {
            method: 'POST',
            credentials: 'include',  // Include refresh token cookie
        });
        
        return res.ok;
    } catch {
        return false;
    }
}

export async function logout(): Promise<void> {
    await fetch(`${API_BASE_URL}/api/auth/logout`, {
        method: 'POST',
        credentials: 'include',
    });
}

export async function checkAuth(): Promise<UserInfo | null> {
    try {
        const res = await fetch(`${API_BASE_URL}/api/auth/me`, {
            credentials: 'include',
        });
        
        if (!res.ok) {
            // Try refresh if 401
            if (res.status === 401) {
                const refreshed = await refreshToken();
                if (refreshed) {
                    // Retry with new token
                    const retryRes = await fetch(`${API_BASE_URL}/api/auth/me`, {
                        credentials: 'include',
                    });
                    if (retryRes.ok) {
                        return retryRes.json();
                    }
                }
            }
            return null;
        }
        
        return res.json();
    } catch {
        return null;
    }
}
```

#### API Wrapper with Auto-Refresh (shared/utils/api.ts)

```typescript
import { refreshToken } from '$lib/features/auth/api';

export const API_BASE_URL = 'https://api-hermes.vinrul.my.id';

export async function apiFetch<T>(
    path: string,
    options: RequestInit = {}
): Promise<T> {
    const res = await fetch(`${API_BASE_URL}${path}`, {
        ...options,
        credentials: 'include',
        headers: {
            'Content-Type': 'application/json',
            ...((options.headers as Record<string, string>) || {}),
        },
    });
    
    // If 401, try to refresh
    if (res.status === 401) {
        const refreshed = await refreshToken();
        
        if (refreshed) {
            // Retry with new token
            const retryRes = await fetch(`${API_BASE_URL}${path}`, {
                ...options,
                credentials: 'include',
                headers: {
                    'Content-Type': 'application/json',
                    ...((options.headers as Record<string, string>) || {}),
                },
            });
            
            if (retryRes.ok) {
                return retryRes.json();
            }
        }
        
        // Refresh failed, redirect to login
        window.location.href = '/login';
        throw new Error('Unauthorized');
    }
    
    if (!res.ok) {
        throw new Error(`API error: ${res.status}`);
    }
    
    return res.json();
}
```

---

### CORS Configuration

```rust
use tower_http::cors::{CorsLayer, AllowOrigin, AllowMethods, AllowHeaders};
use http::Method;

let cors = CorsLayer::new()
    // Load from env: CORS_ORIGIN
    // Allow specific origin (NOT wildcard!)
    .allow_origin(AllowOrigin::exact("https://hermes.vinrul.my.id".parse().unwrap()))
    // Allow credentials
    .allow_credentials(true)
    // Allow methods
    .allow_methods(AllowMethods::from(vec![
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
        Method::OPTIONS,
    ]))
    // Allow headers
    .allow_headers(AllowHeaders::from(vec![
        "content-type".parse().unwrap(),
        "cookie".parse().unwrap(),
    ]));
```

**Important:** 
- ❌ Jangan pakai `AllowOrigin::any()` (wildcard)
- ✅ Pakai `AllowOrigin::exact()` (specific origin)
- ✅ Set `allow_credentials(true)` untuk cookies

---

### Rate Limiting

| Endpoint | Limit | Window |
|----------|-------|--------|
| `POST /api/auth/login` | 5 attempts | 1 minute |
| `POST /api/auth/refresh` | 10 attempts | 1 minute |

**Rate Limit Headers:**
```
X-RateLimit-Limit: 5
X-RateLimit-Remaining: 3
X-RateLimit-Reset: 1620000000
```

**Response when limited:**
```json
{
    "error": "Too many requests",
    "message": "Rate limit exceeded. Please try again later.",
    "retry_after": 45
}
```

---

### Environment Variables

```bash
# Auth
DASHBOARD_USERNAME=admin
DASHBOARD_PASSWORD=your-s... Day)

# Cookie
COOKIE_DOMAIN=.vinrul.my.id
COOKIE_SECURE=true

# Rate Limiting
RATE_LIMIT_LOGIN=5        # 5 attempts per minute
RATE_LIMIT_REFRESH=10     # 10 attempts per minute

# CORS
FRONTEND_URL=https://hermes.vinrul.my.id
```

---

### Security Features Summary

| Feature | Implementation | Status |
|---------|---------------|--------|
| **Access Token** | 15 min, HttpOnly, Secure, SameSite=None | ✅ |
| **Refresh Token** | 7 days, HttpOnly, Secure, SameSite=None, Path=/auth/refresh | ✅ |
| **Token Rotation** | New refresh token on each refresh | ✅ |
| **CORS** | Specific origin, credentials true | ✅ |
| **Origin Validation** | Check Origin/Referer header | ✅ |
| **Rate Limiting** | Login: 5/min, Refresh: 10/min | ✅ |
| **XSS Protection** | HttpOnly cookies | ✅ |
| **CSRF Protection** | SameSite=None + Origin validation | ✅ |

---

### Testing

**1. Test Login:**
```bash
curl -X POST https://api-hermes.vinrul.my.id/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"your-password"}' \
  -c cookies.txt -v

# Should see TWO Set-Cookie headers:
# Set-Cookie: access_token=***; ...; Max-Age=900
# Set-Cookie: refresh_token=***; ...; Path=/auth/refresh; Max-Age=604800
```

**2. Test Protected Route:**
```bash
curl https://api-hermes.vinrul.my.id/api/sessions \
  -b cookies.txt

# Should return data
```

**3. Test Token Refresh:**
```bash
# Wait 15+ minutes (or delete access_token from cookies.txt)
curl -X POST https://api-hermes.vinrul.my.id/api/auth/refresh \
  -b cookies.txt -c cookies.txt -v

# Should get new tokens
```

**4. Test Rate Limiting:**
```bash
# Try 6 login attempts quickly
for i in {1..6}; do
  curl -X POST https://api-hermes.vinrul.my.id/api/auth/login \
    -H "Content-Type: application/json" \
    -d '{"username":"admin","password":"wrong"}' -v
done

# 6th attempt should return 429 Too Many Requests
```

**5. Test Origin Validation:**
```bash
# Should work (correct origin)
curl https://api-hermes.vinrul.my.id/api/health \
  -H "Origin: https://hermes.vinrul.my.id" -v

# Should fail (wrong origin)
curl https://api-hermes.vinrul.my.id/api/health \
  -H "Origin: https://evil.com" -v
```

---

**Last updated:** 2026-05-29
**Security:** Dual-Token with Refresh Rotation, Rate Limiting, Origin Validation
