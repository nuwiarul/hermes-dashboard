## 🔒 Security: JWT Authentication (HttpOnly Cookie)

### Overview

Dashboard dilindungi JWT yang disimpan di **HttpOnly Cookie**. Token tidak bisa diakses JavaScript (XSS-proof) dan otomatis terkirim di setiap request.

### Token Storage Comparison

| Method | XSS Safe | CSRF Safe | Auto-send | Implementation |
|--------|----------|-----------|-----------|----------------|
| localStorage | ❌ | ✅ | ❌ | Simpel |
| sessionStorage | ❌ | ✅ | ❌ | Simpel |
| **HttpOnly Cookie** | ✅ | ⚠️ (need SameSite) | ✅ | Sedang |
| Memory (JS variable) | ✅ | ✅ | ❌ | Ribet |

**Pilihan: HttpOnly Cookie** — balance antara security dan simplicity.

---

### Auth Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  1. User buka /login                                            │
│         │                                                       │
│         ▼                                                       │
│  2. Input username + password                                   │
│         │                                                       │
│         ▼                                                       │
│  3. POST /api/auth/login                                        │
│         │                                                       │
│         ▼                                                       │
│  4. Server verify credentials                                   │
│         │                                                       │
│         ├─ Invalid → 401 Unauthorized                           │
│         │                                                       │
│         └─ Valid → Generate JWT                                 │
│                   │                                             │
│                   ▼                                             │
│  5. Set-Cookie: hermes_token=***; HttpOnly; Secure; SameSite=None│
│                   │                                             │
│                   ▼                                             │
│  6. Response: { success: true }                                 │
│                   │                                             │
│                   ▼                                             │
│  7. Browser auto-save cookie                                    │
│                   │                                             │
│                   ▼                                             │
│  8. Redirect ke / (dashboard)                                   │
│                   │                                             │
│                   ▼                                             │
│  9. Subsequent requests auto-include Cookie header              │
│     Cookie: hermes_token=***                                    │
│                   │                                             │
│                   ▼                                             │
│  10. Server verify token di cookie                              │
│         │                                                       │
│         └─ Valid → Return data                                  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

### Backend Changes

#### New Dependencies (Cargo.toml)

```toml
jsonwebtoken = "9"
tower-cookies = "0.11"
```

#### DTOs (`features/auth/dto.rs`)

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
pub struct Claims {
    pub sub: String,      // username
    pub exp: i64,         // expiration timestamp
    pub iat: i64,         // issued at
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub username: String,
}
```

#### Handler (`features/auth/handler.rs`)

```rust
use axum::{Json, Extension, http::StatusCode};
use tower_cookies::Cookies;
use std::sync::Arc;
use crate::AppState;
use super::dto::*;

const COOKIE_NAME: &str = "hermes_token";
const JWT_SECRET: &str = "change-me-in-production"; // TODO: load from env

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    cookies: Cookies,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Verify credentials
    if !verify_credentials(&state, &payload.username, &payload.password) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    // Generate JWT
    let token = generate_token(&payload.username)?;
    
    // Set HttpOnly cookie
    let cookie = tower_cookies::Cookie::build((COOKIE_NAME, token))
    .domain(".vinrul.my.id")  // Allow cookie for all subdomains
        .path("/")
        .http_only(true)      // Not accessible via JavaScript
        .secure(true)         // HTTPS only (set false for local dev)
        .same_site(tower_cookies::SameSite::None)
        .max_age(time::Duration::hours(24))
        .build();
    
    cookies.add(cookie);
    
    Ok(Json(LoginResponse {
        success: true,
        message: "Login successful".to_string(),
    }))
}

pub async fn logout(cookies: Cookies) -> Json<LoginResponse> {
    // Remove cookie
    let cookie = tower_cookies::Cookie::build((COOKIE_NAME, ""))
    .domain(".vinrul.my.id")  // Allow cookie for all subdomains
        .path("/")
        .http_only(true)
        .secure(true)
        .max_age(time::Duration::seconds(0))
        .build();
    
    cookies.remove(cookie);
    
    Json(LoginResponse {
        success: true,
        message: "Logged out".to_string(),
    })
}

pub async fn me(
    Extension(claims): Extension<Claims>,
) -> Json<UserInfo> {
    Json(UserInfo { username: claims.sub })
}

fn verify_credentials(state: &AppState, username: &str, password: &str) -> bool {
    state.config.auth.username == username && 
    state.config.auth.password == password
}

fn generate_token(username: &str) -> Result<String, StatusCode> {
    let claims = Claims {
        sub: username.to_string(),
        exp: chrono::Utc::now().timestamp() + 86400, // 24 hours
        iat: chrono::Utc::now().timestamp(),
    };
    
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
```

#### Middleware (`features/auth/middleware.rs`)

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
use super::dto::Claims;

const COOKIE_NAME: &str = "hermes_token";
const JWT_SECRET: &str = "change-me-in-production";

pub async fn auth_middleware(
    Extension(state): Extension<Arc<AppState>>,
    cookies: Cookies,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip auth for public routes
    let path = request.uri().path();
    if path.starts_with("/api/auth/") || path == "/api/health" {
        return Ok(next.run(request).await);
    }
    
    // Extract token from cookie
    let token = cookies
        .get(COOKIE_NAME)
        .map(|cookie| cookie.value().to_string())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Verify token
    let claims = verify_token(&token)?;
    
    // Add claims to request extensions
    request.extensions_mut().insert(claims);
    
    Ok(next.run(request).await)
}

fn verify_token(token: &str) -> Result<Claims, StatusCode> {
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    Ok(token_data.claims)
}
```

#### Router Update (`main.rs`)

```rust
use tower_cookies::CookieManagerLayer;
use features::{sessions, stats, config, cron, ws, auth};

let app = Router::new()
    // Public routes (no auth)
    .route("/api/auth/login", post(auth::handler::login))
    .route("/api/auth/logout", post(auth::handler::logout))
    .route("/api/health", get(shared::health::handler))
    
    // Protected routes (auth required)
    .route("/api/auth/me", get(auth::handler::me))
    .route("/api/sessions", get(sessions::handler::list))
    .route("/api/stats", get(stats::handler::overview))
    .route("/api/config", get(config::handler::get_config))
    .route("/api/cron", get(cron::handler::list_jobs))
    .route("/ws", get(ws::handler::ws_handler))
    
    // Middleware layers
    .layer(axum::middleware::from_fn(auth::middleware::auth_middleware))
    .layer(CookieManagerLayer::new())  // Cookie parsing
    .layer(Extension(state))
    .layer(CorsLayer::permissive()
        .allow_credentials(true));  // Required for cookies
```

#### Config Changes (`config.rs`)

```rust
pub struct AuthConfig {
    pub username: String,
    pub password: String,
    pub jwt_secret: String,
    pub cookie_secure: bool,    // true for HTTPS, false for local dev
}

impl AuthConfig {
    pub fn from_env() -> Self {
        Self {
            username: std::env::var("DASHBOARD_USERNAME")
                .unwrap_or_else(|_| "admin".to_string()),
            password: std::env::var("DASHBOARD_PASSWORD")
                .expect("DASHBOARD_PASSWORD must be set"),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "change-me-in-production".to_string()),
            cookie_secure: std::env::var("COOKIE_SECURE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        }
    }
}
```

---

### Frontend Changes

#### Updated Feature: `features/auth/`

```
features/auth/
├── components/
│   └── LoginForm.svelte
├── types.ts
├── api.ts
├── store.ts
└── index.ts
```

#### Types (`features/auth/types.ts`)

```typescript
export interface LoginRequest {
    username: string;
    password: string;
}

export interface LoginResponse {
    success: boolean;
    message: string;
}

export interface UserInfo {
    username: string;
}

export interface AuthState {
    isAuthenticated: boolean;
    user: UserInfo | null;
    loading: boolean;
}
```

#### API (`features/auth/api.ts`)

```typescript
import { API_BASE_URL } from '$lib/shared/utils/api';
import type { LoginRequest, LoginResponse, UserInfo } from './types';

export async function login(credentials: LoginRequest): Promise<LoginResponse> {
    const res = await fetch(`${API_BASE_URL}/api/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',  // Important: include cookies
        body: JSON.stringify(credentials),
    });
    
    if (!res.ok) {
        throw new Error('Invalid credentials');
    }
    
    return res.json();
}

export async function logout(): Promise<void> {
    await fetch(`${API_BASE_URL}/api/auth/logout`, {
        method: 'POST',
        credentials: 'include',
    });
}

export async function getUserInfo(): Promise<UserInfo> {
    const res = await fetch(`${API_BASE_URL}/api/auth/me`, {
        credentials: 'include',  // Send cookie
    });
    
    if (!res.ok) {
        throw new Error('Unauthorized');
    }
    
    return res.json();
}

// Check if user is authenticated (try to fetch user info)
export async function checkAuth(): Promise<UserInfo | null> {
    try {
        return await getUserInfo();
    } catch {
        return null;
    }
}
```

#### Store (`features/auth/store.ts`)

```typescript
import { writable } from 'svelte/store';
import type { AuthState, UserInfo } from './types';
import { checkAuth, logout as apiLogout } from './api';

function createAuthStore() {
    const { subscribe, set, update } = writable<AuthState>({
        isAuthenticated: false,
        user: null,
        loading: true,  // Start with loading=true
    });
    
    return {
        subscribe,
        
        // Check authentication status (call on app init)
        async check() {
            set({ isAuthenticated: false, user: null, loading: true });
            
            const user = await checkAuth();
            
            if (user) {
                set({ isAuthenticated: true, user, loading: false });
            } else {
                set({ isAuthenticated: false, user: null, loading: false });
            }
        },
        
        // Set authenticated after login
        setUser(user: UserInfo) {
            set({ isAuthenticated: true, user, loading: false });
        },
        
        // Logout
        async logout() {
            await apiLogout();
            set({ isAuthenticated: false, user: null, loading: false });
        },
    };
}

export const auth = createAuthStore();
```

#### Login Form (`features/auth/components/LoginForm.svelte`)

```svelte
<script lang="ts">
    import { auth } from '../store';
    import { login } from '../api';
    import { goto } from '$app/navigation';
    
    let username = $state('');
    let password = $state('');
    let error = $state('');
    let loading = $state(false);
    
    async function handleSubmit() {
        loading = true;
        error = '';
        
        try {
            await login({ username, password });
            
            // Fetch user info and update store
            const user = await getUserInfo();
            auth.setUser(user);
            
            // Redirect to dashboard
            goto('/');
        } catch (e) {
            error = 'Invalid username or password';
        } finally {
            loading = false;
        }
    }
</script>

<div class="min-h-screen flex items-center justify-center bg-gray-100">
    <div class="bg-white p-8 rounded-xl shadow-sm w-96">
        <h1 class="text-2xl font-bold mb-6 text-center">🤖 Hermes Dashboard</h1>
        
        <form on:submit|preventDefault={handleSubmit} class="space-y-4">
            <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">
                    Username
                </label>
                <input
                    type="text"
                    bind:value={username}
                    class="w-full px-4 py-2 border rounded-lg"
                    required
                />
            </div>
            
            <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">
                    Password
                </label>
                <input
                    type="password"
                    bind:value={password}
                    class="w-full px-4 py-2 border rounded-lg"
                    required
                />
            </div>
            
            {#if error}
                <p class="text-red-500 text-sm">{error}</p>
            {/if}
            
            <button
                type="submit"
                disabled={loading}
                class="w-full bg-blue-600 text-white py-2 rounded-lg hover:bg-blue-700 disabled:opacity-50"
            >
                {loading ? 'Logging in...' : 'Login'}
            </button>
        </form>
    </div>
</div>
```

#### Login Page (`routes/login/+page.svelte`)

```svelte
<script lang="ts">
    import LoginForm from '$lib/features/auth/components/LoginForm.svelte';
    import { auth } from '$lib/features/auth/store';
    import { goto } from '$app/navigation';
    
    // Redirect to dashboard if already authenticated
    $effect(() => {
        if ($auth.isAuthenticated) {
            goto('/');
        }
    });
</script>

{#if $auth.loading}
    <div class="min-h-screen flex items-center justify-center">
        <p>Loading...</p>
    </div>
{:else if !$auth.isAuthenticated}
    <LoginForm />
{/if}
```

#### Protected Layout (`routes/+layout.svelte`)

```svelte
<script lang="ts">
    import { auth } from '$lib/features/auth/store';
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    import { onMount } from 'svelte';
    
    // Public routes that don't require auth
    const publicRoutes = ['/login'];
    
    // Check auth on mount
    onMount(async () => {
        await auth.check();
    });
    
    // Redirect if not authenticated
    $effect(() => {
        const isPublicRoute = publicRoutes.includes($page.url.pathname);
        
        if (!$auth.loading && !$auth.isAuthenticated && !isPublicRoute) {
            goto('/login');
        }
    });
</script>

{#if $auth.loading}
    <div class="min-h-screen flex items-center justify-center">
        <p>Loading...</p>
    </div>
{:else if $auth.isAuthenticated || publicRoutes.includes($page.url.pathname)}
    <slot />
{/if}
```

#### API Wrapper Update (`shared/utils/api.ts`)

```typescript
export const API_BASE_URL = 'https://hermes.vinrul.my.id:3001';

export async function apiFetch<T>(
    path: string,
    options: RequestInit = {}
): Promise<T> {
    const res = await fetch(`${API_BASE_URL}${path}`, {
        ...options,
        credentials: 'include',  // Always include cookies
        headers: {
            'Content-Type': 'application/json',
            ...((options.headers as Record<string, string>) || {}),
        },
    });
    
    if (res.status === 401) {
        // Redirect to login
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

### Cookie Configuration

| Attribute | Value | Purpose |
|-----------|-------|---------|
| `name` | `hermes_token` | Cookie name |
| `value` | JWT token | Authentication data |
| `path` | `/` | Available for all paths |
| `httpOnly` | `true` | **Not accessible via JavaScript** (XSS-safe) |
| `secure` | `true` (prod) / `false` (dev) | HTTPS only |
| `sameSite` | `Lax` | CSRF protection |
| `maxAge` | `86400` (24 hours) | Auto-expire |

---

### Security Comparison

| Attack Vector | localStorage | HttpOnly Cookie |
|---------------|--------------|-----------------|
| **XSS** | ❌ Token stolen | ✅ Safe (can't read) |
| **CSRF** | ✅ Safe (no auto-send) | ⚠️ Need SameSite |
| **Network (HTTP)** | ❌ Exposed | ❌ Exposed |
| **Network (HTTPS)** | ✅ Encrypted | ✅ Encrypted |

**Mitigations:**
- XSS → HttpOnly cookie (can't access via JS)
- CSRF → SameSite=None (blocks cross-site POST)
- Network → HTTPS (encrypt in transit)

---

### Environment Variables

```bash
# Backend
DASHBOARD_USERNAME=admin
DASHBOARD_PASSWORD=your-secure-password
JWT_SECRET=your-random-secret-key-at-least-32-chars
COOKIE_SECURE=false  # Set true for HTTPS production
```

---

### CORS Configuration (Important for Cookies)

When using cookies, CORS must be configured correctly:

```rust
use tower_http::cors::{CorsLayer, AllowOrigin};

let cors = CorsLayer::new()
    .allow_origin(AllowOrigin::exact("https://hermes.vinrul.my.id".parse().unwrap()))
    .allow_credentials(true)
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([HeaderName::from_static("content-type")]);
```

**Frontend `fetch` must include:**
```typescript
fetch(url, {
    credentials: 'include',  // CRITICAL for cookies
    // ...
});
```

---

### Testing

**1. Test Login:**
```bash
curl -X POST http://localhost:3001/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"your-password"}' \
  -c cookies.txt -v

# Should see Set-Cookie header in response
```

**2. Test Protected Route:**
```bash
curl http://localhost:3001/api/sessions \
  -b cookies.txt

# Should return data if cookie is valid
```

**3. Test Unauthorized:**
```bash
curl http://localhost:3001/api/sessions

# Should return 401
```

---

**Last updated:** 2026-05-29
**Security:** JWT + HttpOnly Cookie

---

## 🌐 Cross-Origin Configuration

Karena Frontend (`hermes.vinrul.my.id`) dan Backend (`api-hermes.vinrul.my.id`) adalah **different origins**, perlu konfigurasi khusus.

### CORS (Backend)

```rust
use tower_http::cors::{CorsLayer, AllowOrigin};

let cors = CorsLayer::new()
    .allow_origin(AllowOrigin::exact("https://hermes.vinrul.my.id".parse().unwrap()))
    .allow_credentials(true)
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
    .allow_headers(["content-type".parse().unwrap()]);
```

### Cookie (Cross-Origin)

```rust
let cookie = Cookie::build(("hermes_token", token))
    .domain(".vinrul.my.id")  // Works for all subdomains
    .path("/")
    .http_only(true)
    .secure(true)
    .same_site(tower_cookies::SameSite::None)  // Required for cross-origin
    .max_age(time::Duration::hours(24))
    .build();
```

### Frontend (credentials: include)

```typescript
const response = await fetch('https://api-hermes.vinrul.my.id/api/...', {
    credentials: 'include',  // CRITICAL for cross-origin cookies
});
```

