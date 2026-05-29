## 🔒 Security: JWT Authentication

### Overview

Dashboard dilindungi JWT (JSON Web Token). User harus login dulu sebelum akses halaman lain. Token disimpan di localStorage dan dikirim di setiap API request via `Authorization: Bearer <token>`.

### Flow

```
┌─────────────────────────────────────────────────────────┐
│  User buka dashboard                                    │
│         │                                               │
│         ▼                                               │
│  ┌─────────────┐    No token?     ┌──────────────┐    │
│  │  Check Token │ ──────────────► │  Login Page   │    │
│  └─────────────┘                  └──────┬───────┘    │
│         │                                │             │
│         │ Valid token?                   │ Submit      │
│         ▼                                ▼             │
│  ┌─────────────┐                  ┌──────────────┐    │
│  │  Dashboard   │ ◄────────────── │ POST /api/    │    │
│  │  (protected) │    Set token    │ auth/login    │    │
│  └─────────────┘                  └──────────────┘    │
│         │                                               │
│         │ Expired token?                                │
│         ▼                                               │
│  ┌─────────────┐                                        │
│  │  Auto       │                                        │
│  │  Refresh    │                                        │
│  └─────────────┘                                        │
└─────────────────────────────────────────────────────────┘
```

### Backend Changes

#### New Feature: `features/auth/`

```
features/auth/
├── mod.rs
├── dto.rs           # LoginRequest, LoginResponse, Claims
├── handler.rs       # login(), refresh(), me()
├── middleware.rs     # auth_middleware()
└── repository.rs    # verify_credentials()
```

#### Dependencies (add to Cargo.toml)

```toml
jsonwebtoken = "9"
argon2 = "0.5"  # For password hashing (optional, can use simple hash for v1)
uuid = { version = "1", features = ["v4"] }
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
    pub token: String,
    pub expires_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
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
use std::sync::Arc;
use crate::AppState;
use super::dto::*;

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Verify credentials
    if !verify_credentials(&state, &payload.username, &payload.password) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    // Generate JWT
    let token = generate_token(&payload.username)?;
    let expires_at = chrono::Utc::now().timestamp() + 3600; // 1 hour
    
    Ok(Json(LoginResponse { token, expires_at }))
}

pub async fn me(
    Extension(claims): Extension<Claims>,
) -> Json<UserInfo> {
    Json(UserInfo { username: claims.sub })
}

fn verify_credentials(state: &AppState, username: &str, password: &str) -> bool {
    // Compare with credentials from config
    state.config.auth.username == username && 
    state.config.auth.password == password
}

fn generate_token(username: &str) -> Result<String, StatusCode> {
    let claims = Claims {
        sub: username.to_string(),
        exp: chrono::Utc::now().timestamp() + 3600,
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
use std::sync::Arc;
use crate::AppState;
use super::dto::Claims;

pub async fn auth_middleware(
    Extension(state): Extension<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip auth for login endpoint
    if request.uri().path() == "/api/auth/login" {
        return Ok(next.run(request).await);
    }
    
    // Extract token from Authorization header
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Verify token
    let claims = verify_token(token)?;
    
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

#### Config Changes (`config.rs`)

```rust
pub struct AuthConfig {
    pub username: String,
    pub password: String,
    pub jwt_secret: String,
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
        }
    }
}

pub struct AppConfig {
    pub hermes_home: PathBuf,
    pub port: u16,
    pub auth: AuthConfig,
}
```

#### Router Update (`main.rs`)

```rust
use features::{sessions, stats, config, cron, ws, auth};

let app = Router::new()
    // Public routes (no auth)
    .route("/api/auth/login", post(auth::handler::login))
    .route("/api/health", get(shared::health::handler))
    
    // Protected routes (auth required)
    .route("/api/sessions", get(sessions::handler::list))
    .route("/api/stats", get(stats::handler::overview))
    .route("/api/config", get(config::handler::get_config))
    .route("/api/cron", get(cron::handler::list_jobs))
    .route("/ws", get(ws::handler::ws_handler))
    
    // Auth middleware (applied to all routes)
    .layer(axum::middleware::from_fn(auth::middleware::auth_middleware))
    .layer(Extension(state))
    .layer(CorsLayer::permissive());
```

---

### Frontend Changes

#### New Feature: `features/auth/`

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
    token: string;
    expires_at: number;
}

export interface UserInfo {
    username: string;
}

export interface AuthState {
    isAuthenticated: boolean;
    token: string | null;
    user: UserInfo | null;
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
        body: JSON.stringify(credentials),
    });
    
    if (!res.ok) {
        throw new Error('Invalid credentials');
    }
    
    return res.json();
}

export async function getUserInfo(token: string): Promise<UserInfo> {
    const res = await fetch(`${API_BASE_URL}/api/auth/me`, {
        headers: { 'Authorization': `Bearer ${token}` },
    });
    
    if (!res.ok) {
        throw new Error('Unauthorized');
    }
    
    return res.json();
}
```

#### Store (`features/auth/store.ts`)

```typescript
import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import type { AuthState } from './types';

const TOKEN_KEY = 'hermes_dashboard_token';

function createAuthStore() {
    const { subscribe, set, update } = writable<AuthState>({
        isAuthenticated: false,
        token: null,
        user: null,
    });
    
    // Load token from localStorage on init
    if (browser) {
        const savedToken = localStorage.getItem(TOKEN_KEY);
        if (savedToken) {
            set({
                isAuthenticated: true,
                token: savedToken,
                user: null, // Will be fetched
            });
        }
    }
    
    return {
        subscribe,
        setToken: (token: string) => {
            if (browser) {
                localStorage.setItem(TOKEN_KEY, token);
            }
            set({
                isAuthenticated: true,
                token,
                user: null,
            });
        },
        logout: () => {
            if (browser) {
                localStorage.removeItem(TOKEN_KEY);
            }
            set({
                isAuthenticated: false,
                token: null,
                user: null,
            });
        },
        getToken: (): string | null => {
            if (browser) {
                return localStorage.getItem(TOKEN_KEY);
            }
            return null;
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
    
    let username = $state('');
    let password = $state('');
    let error = $state('');
    let loading = $state(false);
    
    async function handleSubmit() {
        loading = true;
        error = '';
        
        try {
            const response = await login({ username, password });
            auth.setToken(response.token);
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

<LoginForm />
```

#### Protected Layout (`routes/+layout.svelte`)

```svelte
<script lang="ts">
    import { auth } from '$lib/features/auth/store';
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    
    // Public routes that don't require auth
    const publicRoutes = ['/login'];
    
    $effect(() => {
        const isPublicRoute = publicRoutes.includes($page.url.pathname);
        
        if (!$auth.isAuthenticated && !isPublicRoute) {
            goto('/login');
        }
    });
</script>

{#if $auth.isAuthenticated || publicRoutes.includes($page.url.pathname)}
    <slot />
{/if}
```

#### API Wrapper Update (`shared/utils/api.ts`)

```typescript
import { auth } from '$lib/features/auth/store';
import { get } from 'svelte/store';

export const API_BASE_URL = 'http://47.84.137.49:3001';

export async function apiFetch<T>(
    path: string,
    options: RequestInit = {}
): Promise<T> {
    const token = auth.getToken();
    
    const headers: Record<string, string> = {
        'Content-Type': 'application/json',
        ...((options.headers as Record<string, string>) || {}),
    };
    
    if (token) {
        headers['Authorization'] = `Bearer ${token}`;
    }
    
    const res = await fetch(`${API_BASE_URL}${path}`, {
        ...options,
        headers,
    });
    
    if (res.status === 401) {
        auth.logout();
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

### Environment Variables

```bash
# Backend (.env atau systemd service)
DASHBOARD_USERNAME=admin
DASHBOARD_PASSWORD=your-secure-password-here
JWT_SECRET=your-random-secret-key-at-least-32-chars
```

### Security Features

| Feature | Implementation |
|---------|---------------|
| Login | POST /api/auth/login → returns JWT |
| Token | JWT with 1-hour expiry |
| Storage | localStorage (frontend) |
| Transport | Bearer token in Authorization header |
| Middleware | Verify JWT on all protected routes |
| Auto-redirect | → /login if unauthorized |
| Logout | Clear localStorage + redirect |

### Password Security Notes

**V1 (Initial):**
- Password stored in env var (plain text)
- Simple comparison for authentication
- Good enough for personal use

**V2 (Future):**
- Hash password with argon2
- Store hash in config file
- Support multiple users

---

**Last updated:** 2026-05-29
**Security:** JWT Authentication
