# Hermes Dashboard — Implementation Plan

> **For Hermes:** Use subagent-driven-development skill to implement this plan task-by-task.

**Goal:** Web dashboard untuk monitor & kontrol Hermes Agent — session viewer, cost tracker, tool analytics, cron manager, dan remote control.

---

## ✅ Task Checklist

### Phase 1: Project Setup & Foundation
- [x] Task 1.1: Initialize Rust Backend
- [x] Task 1.2: Initialize SvelteKit Frontend (SPA Mode)
- [x] Task 1.3: Nginx Configuration (Alibaba Server)

### Phase 2: Core API Endpoints
- [x] Task 2.1: Sessions List API
- [x] Task 2.2: Stats Overview API
- [x] Task 2.3: Config Reader API

### Phase 3: Frontend Pages
- [x] Task 3.1: Dashboard Layout
- [x] Task 3.2: Dashboard Home Page
- [x] Task 3.3: Sessions Page

### Phase 4: Advanced Features
- [x] Task 4.1: WebSocket Real-time Updates
- [x] Task 4.2: Cron Jobs Manager

### Phase 5: Deployment
- [x] Task 5.1: Build & Deploy Scripts
- [x] Task 5.2: Systemd Service (Backend)

### Phase 6: Security & Authentication
- [x] Task 6.1: JWT Authentication Setup
- [x] Task 6.2: Login API (cookie-based)
- [x] Task 6.3: Middleware Authorization
- [x] Task 6.4: Rate Limiting
- [x] Task 6.5: Input Validation & Sanitization

### Phase 7: Responsive Design
- [x] Task 7.1: Mobile-First Layout
- [x] Task 7.2: Responsive Navigation
- [x] Task 7.3: Touch-Friendly Components
- [x] Task 7.4: Dark/Light Theme

### Phase 8: Testing
- [x] Task 8.1: Backend Unit Tests
- [x] Task 8.2: API Integration Tests
- [x] Task 8.3: Frontend Component Tests
- [x] Task 8.4: Accessibility Tests

### Phase 9: Performance & Optimization
- [x] Task 9.1: API Response Caching
- [x] Task 9.2: Frontend Lazy Loading
- [x] Task 9.3: Database Query Optimization
- [x] Task 9.4: Asset Optimization (minify, compress)

### Phase 10: Remote Control (Tools Menu)
- [x] Task 10.1: Switch Model — ganti model dari web UI
- [x] Task 10.2: Tool Manager — enable/disable tools
- [x] Task 10.3: Send Message — kirim pesan ke agent dari web
- [x] Task 10.4: Gateway Control — restart/status gateway

### Phase 11: Worker Management (Multi-Node)
- [x] Task 11.1: Worker Registration API — node register ke dashboard
- [x] Task 11.2: Worker Status API — heartbeat & status tracking
- [ ] Task 11.3: Worker List UI — tampilkan semua registered nodes
- [ ] Task 11.4: Worker Config API — change model/provider dari dashboard
- [ ] Task 11.5: Worker Config UI — dropdown model + apply button
- [ ] Task 11.6: Worker Task History — tampilkan task logs per node
- [ ] Task 11.7: Real-time Status — WebSocket update status nodes
- [ ] Task 11.8: Worker Health Check — auto-detect offline nodes

---

**Architecture:** 
- **Backend:** Rust (Axum 0.8) baca Hermes state.db + config + logs, expose REST API + WebSocket
- **Frontend:** SvelteKit 2 + Svelte 5 + Tailwind CSS 4 (SPA mode via adapter-static) → build jadi static files → serve via Nginx di Alibaba
- **Deploy:** Backend di Tencent (2GB RAM), Frontend di Alibaba (Nginx + 1GB RAM)
- **Runtime:** Bun (tidak perlu Node.js)

**Tech Stack:**
- Backend: Rust, Axum 0.8, SQLx 0.9, tokio, serde
- Frontend: SvelteKit 2.61, Svelte 5.55, Tailwind CSS 4.3, Chart.js
- Database: SQLite (baca langsung dari `~/.hermes/state.db`)
- Web Server: Nginx (Alibaba) untuk serve frontend static files
- Package Manager: Bun (bukan npm/yarn)

---

## Deployment Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Alibaba Server (47.84.137.49)                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Nginx (port 80/443)                                │   │
│  │  ├── /          → SvelteKit SPA (static files)      │   │
│  │  └── /api/*     → proxy_pass ke Tencent:3001        │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                         │
                         │ proxy_pass
                         ▼
┌─────────────────────────────────────────────────────────────┐
│  Tencent Server (43.156.247.129)                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Rust Backend (port 3001)                           │   │
│  │  ├── /api/sessions  → baca state.db                 │   │
│  │  ├── /api/stats     → aggregate data                │   │
│  │  ├── /api/config    → baca config.yaml              │   │
│  │  ├── /api/cron      → manage cron jobs              │   │
│  │  └── /ws            → WebSocket real-time           │   │
│  └─────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  ~/.hermes/state.db  (Hermes sessions & messages)   │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

**Kenapa pisah?**
- Backend harus di Tencent karena baca `~/.hermes/state.db` langsung
- Frontend di Alibaba karena Nginx udah ada dan serve static files lebih efisien
- Nginx handle SSL, caching, compression

**Kenapa Bun?**
- Lebih cepat dari npm/yarn (10-100x)
- Tidak perlu install Node.js di server
- `bunx` bisa gantikan `npx`

---

## Phase 1: Project Setup & Foundation

### Task 1.1: Initialize Rust Backend

**Objective:** Setup Rust project dengan Axum web framework (versi terbaru)

**Files:**
- Create: `backend/Cargo.toml`
- Create: `backend/src/main.rs`
- Create: `backend/src/config.rs`
- Create: `backend/src/db.rs`
- Create: `backend/src/routes/mod.rs`
- Create: `backend/src/routes/health.rs`

**Step 1: Create Cargo.toml**

```toml
[package]
name = "hermes-dashboard-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = { version = "0.8", features = ["ws", "macros"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.9", features = ["runtime-tokio", "sqlite"] }
tower-http = { version = "0.6", features = ["cors"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
```

**Step 2: Create main.rs**

```rust
use axum::{routing::get, Router, Extension};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod config;
mod db;
mod routes;

pub struct AppState {
    pub db: sqlx::sqlite::SqlitePool,
    pub config: config::AppConfig,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::init();

    let app_config = config::AppConfig::from_env();
    let db_pool = db::connect(&app_config.state_db_path()).await?;

    let state = Arc::new(AppState {
        db: db_pool,
        config: app_config,
    });

    let app = Router::new()
        .route("/api/health", get(routes::health::handler))
        .route("/api/sessions", get(routes::sessions::list))
        .route("/api/stats", get(routes::stats::overview))
        .route("/api/config", get(routes::config::get_config))
        .route("/api/cron", get(routes::cron::list_jobs))
        .route("/ws", get(routes::ws::ws_handler))
        .layer(Extension(state))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::info!("Server running on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
```

**Step 3: Create config.rs**

```rust
use std::path::PathBuf;

pub struct AppConfig {
    pub hermes_home: PathBuf,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let hermes_home = std::env::var("HERMES_HOME")
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_default();
                format!("{}/.hermes", home)
            })
            .into();

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .parse()
            .unwrap_or(3001);

        Self { hermes_home, port }
    }

    pub fn state_db_path(&self) -> PathBuf {
        self.hermes_home.join("state.db")
    }

    pub fn config_path(&self) -> PathBuf {
        self.hermes_home.join("config.yaml")
    }

    pub fn logs_path(&self) -> PathBuf {
        self.hermes_home.join("logs")
    }
}
```

**Step 4: Create db.rs**

```rust
use sqlx::sqlite::SqlitePool;
use std::path::Path;

pub async fn connect(db_path: &Path) -> anyhow::Result<SqlitePool> {
    let url = format!("sqlite:{}?mode=ro", db_path.display());
    let pool = SqlitePool::connect(&url).await?;
    Ok(pool)
}
```

**Step 5: Create routes/mod.rs**

```rust
pub mod health;
pub mod sessions;
pub mod stats;
pub mod config;
pub mod cron;
pub mod ws;
```

**Step 6: Create routes/health.rs**

```rust
use axum::Json;
use serde_json::{json, Value};

pub async fn handler() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "hermes-dashboard",
        "version": "0.1.0"
    }))
}
```

**Step 7: Test**

```bash
cd backend
cargo run
# In another terminal:
curl http://localhost:3001/api/health
# Expected: {"status":"ok","service":"hermes-dashboard","version":"0.1.0"}
```

**Step 8: Commit**

```bash
git add backend/
git commit -m "feat: initialize Rust backend with Axum 0.8"
```

---

### Task 1.2: Initialize SvelteKit Frontend (SPA Mode)

**Objective:** Setup SvelteKit 2 + Svelte 5 + Tailwind CSS 4 sebagai SPA

**Files:**
- Create: `frontend/` (via `bunx sv create`)

**Step 1: Create SvelteKit project dengan Bun**

```bash
cd ~/hermes-dashboard
bunx sv@latest create frontend
# Select:
#   - Skeleton project
#   - TypeScript
#   - Tailwind CSS v4
cd frontend
bun install
```

**Step 2: Install adapter-static untuk SPA**

```bash
bun add -D @sveltejs/adapter-static
```

**Step 3: Update svelte.config.js**

```javascript
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),
    kit: {
        adapter: adapter({
            pages: 'build',
            assets: 'build',
            fallback: 'index.html', // SPA fallback
            precompress: false,
            strict: true
        })
    }
};

export default config;
```

**Step 4: Create src/routes/+layout.ts**

```typescript
export const prerender = true;
export const ssr = false; // SPA mode - no server-side rendering
```

**Step 5: Add Chart.js**

```bash
bun add chart.js svelte-chartjs
```

**Step 6: Verify versions**

```bash
bun pm ls svelte
# Expected: svelte@5.x.x

bun pm ls @sveltejs/kit
# Expected: @sveltejs/kit@2.x.x

bun pm ls tailwindcss
# Expected: tailwindcss@4.x.x
```

**Step 7: Test build**

```bash
cd frontend
bun run build
# Output di folder build/ — siap di-serve Nginx
ls build/
```

**Step 8: Commit**

```bash
git add frontend/
git commit -m "feat: initialize SvelteKit 2 + Svelte 5 + Tailwind CSS 4 SPA"
```

---

### Task 1.3: Nginx Configuration (Alibaba Server)

**Objective:** Setup Nginx untuk serve SPA + proxy API ke Tencent

**Files:**
- Create: `scripts/nginx/hermes-dashboard.conf`

**Step 1: Create Nginx config**

```nginx
server {
    listen 80;
    server_name _; # Ganti dengan domain atau IP

    # Frontend static files
    root /var/www/hermes-dashboard;
    index index.html;

    # SPA fallback — semua route di-handle SvelteKit
    location / {
        try_files $uri $uri/ /index.html;
    }

    # API proxy ke backend di Tencent
    location /api/ {
        proxy_pass https://api-hermes.vinrul.my.id;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # WebSocket proxy
    location /ws {
        proxy_pass https://api-hermes.vinrul.my.id;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # Cache static assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # Gzip compression
    gzip on;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;
}
```

**Step 2: Deploy frontend ke Alibaba**

```bash
# Dari Tencent (build frontend)
cd ~/hermes-dashboard/frontend
bun run build

# Copy ke Alibaba
scp -i ~/.ssh/alibabakey.pem -r build/* ubuntu@47.84.137.49:/var/www/hermes-dashboard/

# Di Alibaba
ssh -i ~/.ssh/alibabakey.pem ubuntu@47.84.137.49
sudo chown -R www-data:www-data /var/www/hermes-dashboard
```

**Step 3: Enable Nginx config**

```bash
# Di Alibaba
sudo cp hermes-dashboard.conf /etc/nginx/sites-available/
sudo ln -s /etc/nginx/sites-available/hermes-dashboard.conf /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

**Step 4: Commit**

```bash
git add scripts/
git commit -m "feat: add Nginx config for Alibaba deployment"
```

---

## Phase 2: Core API Endpoints

### Task 2.1: Sessions List API

**Objective:** API untuk list semua session dari state.db

**Files:**
- Create: `backend/src/routes/sessions.rs`
- Create: `backend/src/models/session.rs`

**Step 1: Create models/session.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    pub session_id: String,
    pub title: Option<String>,
    pub source: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub message_count: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct SessionSummary {
    pub sessions: Vec<Session>,
    pub total: i64,
}
```

**Step 2: Create routes/sessions.rs**

```rust
use axum::{Extension, Json};
use std::sync::Arc;
use crate::AppState;
use crate::models::session::{Session, SessionSummary};

pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
) -> Json<SessionSummary> {
    let sessions = sqlx::query_as::<_, Session>(
        "SELECT 
            session_id,
            title,
            source,
            created_at,
            updated_at,
            (SELECT COUNT(*) FROM messages m WHERE m.session_id = s.session_id) as message_count
         FROM sessions s 
         ORDER BY updated_at DESC 
         LIMIT 50"
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let total = sessions.len() as i64;

    Json(SessionSummary { sessions, total })
}
```

**Step 3: Test**

```bash
cargo run
curl http://localhost:3001/api/sessions | jq
```

**Step 4: Commit**

```bash
git add backend/
git commit -m "feat: add sessions list API"
```

---

### Task 2.2: Stats Overview API

**Objective:** API untuk overview statistics

**Files:**
- Create: `backend/src/routes/stats.rs`

**Step 1: Create routes/stats.rs**

```rust
use axum::{Extension, Json};
use serde::Serialize;
use std::sync::Arc;
use crate::AppState;

#[derive(Serialize)]
pub struct StatsOverview {
    pub total_sessions: i64,
    pub total_messages: i64,
    pub sessions_today: i64,
    pub messages_today: i64,
    pub active_sources: Vec<SourceCount>,
}

#[derive(Serialize)]
pub struct SourceCount {
    pub source: String,
    pub count: i64,
}

pub async fn overview(
    Extension(state): Extension<Arc<AppState>>,
) -> Json<StatsOverview> {
    let total_sessions: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sessions"
    )
    .fetch_one(&state.db)
    .await
    .unwrap_or(0);

    let total_messages: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM messages"
    )
    .fetch_one(&state.db)
    .await
    .unwrap_or(0);

    let sessions_today: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sessions WHERE date(created_at) = date('now')"
    )
    .fetch_one(&state.db)
    .await
    .unwrap_or(0);

    let messages_today: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM messages WHERE date(created_at) = date('now')"
    )
    .fetch_one(&state.db)
    .await
    .unwrap_or(0);

    let active_sources = sqlx::query_as::<_, (String, i64)>(
        "SELECT COALESCE(source, 'unknown'), COUNT(*) 
         FROM sessions 
         GROUP BY source 
         ORDER BY COUNT(*) DESC"
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|(source, count)| SourceCount { source, count })
    .collect();

    Json(StatsOverview {
        total_sessions,
        total_messages,
        sessions_today,
        messages_today,
        active_sources,
    })
}
```

**Step 2: Test**

```bash
curl http://localhost:3001/api/stats | jq
```

**Step 3: Commit**

```bash
git add backend/
git commit -m "feat: add stats overview API"
```

---

### Task 2.3: Config Reader API

**Objective:** API untuk baca Hermes config.yaml

**Files:**
- Create: `backend/src/routes/config.rs`

**Step 1: Create routes/config.rs**

```rust
use axum::{Extension, Json};
use serde::Serialize;
use std::sync::Arc;
use crate::AppState;

#[derive(Serialize)]
pub struct ConfigInfo {
    pub model: Option<String>,
    pub provider: Option<String>,
    pub raw_yaml: String,
}

pub async fn get_config(
    Extension(state): Extension<Arc<AppState>>,
) -> Json<ConfigInfo> {
    let config_path = state.config.config_path();
    
    let raw_yaml = std::fs::read_to_string(&config_path)
        .unwrap_or_else(|_| "Config file not found".to_string());

    let model = extract_yaml_value(&raw_yaml, "default");
    let provider = extract_yaml_value(&raw_yaml, "provider");

    Json(ConfigInfo {
        model,
        provider,
        raw_yaml,
    })
}

fn extract_yaml_value(yaml: &str, key: &str) -> Option<String> {
    for line in yaml.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&format!("{}:", key)) {
            let value = trimmed.split(':').nth(1)?.trim();
            return Some(value.to_string());
        }
    }
    None
}
```

**Step 2: Test**

```bash
curl http://localhost:3001/api/config | jq
```

**Step 3: Commit**

```bash
git add backend/
git commit -m "feat: add config reader API"
```

---

## Phase 3: Frontend Pages

### Task 3.1: Dashboard Layout

**Objective:** Buat layout utama dengan sidebar navigasi (Svelte 5 syntax)

**Files:**
- Create: `frontend/src/routes/+layout.svelte`
- Create: `frontend/src/lib/components/Sidebar.svelte`
- Create: `frontend/src/lib/components/Header.svelte`

**Step 1: Create Sidebar.svelte (Svelte 5 syntax)**

```svelte
<script lang="ts">
    import { page } from '$app/stores';
    
    const navItems = [
        { href: '/', label: 'Dashboard', icon: '📊' },
        { href: '/sessions', label: 'Sessions', icon: '💬' },
        { href: '/cron', label: 'Cron Jobs', icon: '⏰' },
        { href: '/tools', label: 'Tools', icon: '🔧' },
        { href: '/settings', label: 'Settings', icon: '⚙️' },
    ];
</script>

<aside class="w-64 bg-gray-900 text-white min-h-screen p-4">
    <div class="mb-8">
        <h1 class="text-2xl font-bold">🤖 Hermes</h1>
        <p class="text-gray-400 text-sm">Dashboard</p>
    </div>
    
    <nav>
        {#each navItems as item}
            <a 
                href={item.href}
                class="flex items-center gap-3 px-4 py-3 rounded-lg mb-2 transition-colors
                       {$page.url.pathname === item.href 
                         ? 'bg-blue-600 text-white' 
                         : 'text-gray-300 hover:bg-gray-800'}"
            >
                <span>{item.icon}</span>
                <span>{item.label}</span>
            </a>
        {/each}
    </nav>
</aside>
```

**Step 2: Create Header.svelte**

```svelte
<script lang="ts">
    let { status = 'online', model = 'Unknown' }: { 
        status?: 'online' | 'offline'; 
        model?: string 
    } = $props();
</script>

<header class="bg-gray-800 text-white px-6 py-4 flex justify-between items-center">
    <div>
        <span class="text-sm text-gray-400">Model:</span>
        <span class="ml-2 font-mono">{model}</span>
    </div>
    
    <div class="flex items-center gap-2">
        <span class="w-2 h-2 rounded-full {status === 'online' ? 'bg-green-500' : 'bg-red-500'}"></span>
        <span class="text-sm capitalize">{status}</span>
    </div>
</header>
```

**Step 3: Create +layout.svelte**

```svelte
<script lang="ts">
    import Sidebar from '$lib/components/Sidebar.svelte';
    import Header from '$lib/components/Header.svelte';
</script>

<div class="flex">
    <Sidebar />
    
    <div class="flex-1">
        <Header status="online" model="mimo-v2.5" />
        
        <main class="p-6 bg-gray-100 min-h-[calc(100vh-64px)]">
            <slot />
        </main>
    </div>
</div>
```

**Step 4: Test**

```bash
cd frontend
bun dev
# Should see sidebar + header layout
```

**Step 5: Commit**

```bash
git add frontend/
git commit -m "feat: add dashboard layout with sidebar (Svelte 5)"
```

---

### Task 3.2: Dashboard Home Page

**Objective:** Halaman utama dengan stats cards (Svelte 5 runes)

**Files:**
- Create: `frontend/src/routes/+page.svelte`
- Create: `frontend/src/lib/components/StatsCard.svelte`

**Step 1: Create StatsCard.svelte (Svelte 5)**

```svelte
<script lang="ts">
    let { title, value, icon, trend = 'neutral' }: {
        title: string;
        value: string | number;
        icon: string;
        trend?: 'up' | 'down' | 'neutral';
    } = $props();
</script>

<div class="bg-white rounded-xl shadow-sm p-6">
    <div class="flex items-center justify-between mb-4">
        <span class="text-3xl">{icon}</span>
        {#if trend === 'up'}
            <span class="text-green-500 text-sm">↑ +12%</span>
        {:else if trend === 'down'}
            <span class="text-red-500 text-sm">↓ -5%</span>
        {/if}
    </div>
    <p class="text-gray-500 text-sm">{title}</p>
    <p class="text-3xl font-bold mt-1">{value}</p>
</div>
```

**Step 2: Create +page.svelte (Svelte 5 runes)**

```svelte
<script lang="ts">
    import StatsCard from '$lib/components/StatsCard.svelte';
    import { onMount } from 'svelte';
    
    let stats = $state({
        total_sessions: 0,
        total_messages: 0,
        sessions_today: 0,
        messages_today: 0,
    });
    
    let loading = $state(true);
    
    onMount(async () => {
        try {
            const res = await fetch('/api/stats');
            stats = await res.json();
        } catch (e) {
            console.error('Failed to fetch stats:', e);
        } finally {
            loading = false;
        }
    });
</script>

<div class="space-y-6">
    <h2 class="text-2xl font-bold">Dashboard</h2>
    
    {#if loading}
        <p>Loading...</p>
    {:else}
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            <StatsCard 
                title="Total Sessions" 
                value={stats.total_sessions} 
                icon="💬" 
                trend="up"
            />
            <StatsCard 
                title="Total Messages" 
                value={stats.total_messages} 
                icon="📨" 
                trend="up"
            />
            <StatsCard 
                title="Sessions Today" 
                value={stats.sessions_today} 
                icon="📅" 
            />
            <StatsCard 
                title="Messages Today" 
                value={stats.messages_today} 
                icon="⚡" 
            />
        </div>
        
        <div class="bg-white rounded-xl shadow-sm p-6">
            <h3 class="text-lg font-semibold mb-4">Recent Activity</h3>
            <!-- Chart will go here -->
        </div>
    {/if}
</div>
```

**Step 3: Test**

```bash
cd frontend
bun dev
# Should see stats cards
```

**Step 4: Commit**

```bash
git add frontend/
git commit -m "feat: add dashboard home page with Svelte 5 runes"
```

---

### Task 3.3: Sessions Page

**Objective:** Halaman untuk browse dan search sessions

**Files:**
- Create: `frontend/src/routes/sessions/+page.svelte`
- Create: `frontend/src/lib/components/SessionCard.svelte`

**Step 1: Create SessionCard.svelte (Svelte 5)**

```svelte
<script lang="ts">
    let { session }: {
        session: {
            session_id: string;
            title: string | null;
            source: string | null;
            created_at: string | null;
            message_count: number | null;
        };
    } = $props();
</script>

<div class="bg-white rounded-lg shadow-sm p-4 hover:shadow-md transition-shadow cursor-pointer">
    <div class="flex justify-between items-start">
        <div>
            <h3 class="font-semibold text-lg">
                {session.title || 'Untitled Session'}
            </h3>
            <p class="text-gray-500 text-sm mt-1">
                {session.source || 'unknown'} • {session.message_count || 0} messages
            </p>
        </div>
        <span class="text-xs text-gray-400">
            {session.created_at ? new Date(session.created_at).toLocaleDateString() : 'Unknown'}
        </span>
    </div>
    <p class="text-xs text-gray-400 mt-2 font-mono">{session.session_id}</p>
</div>
```

**Step 2: Create sessions/+page.svelte (Svelte 5)**

```svelte
<script lang="ts">
    import SessionCard from '$lib/components/SessionCard.svelte';
    import { onMount } from 'svelte';
    
    let sessions = $state<any[]>([]);
    let loading = $state(true);
    let search = $state('');
    
    let filteredSessions = $derived(
        sessions.filter(s => 
            search === '' || 
            s.title?.toLowerCase().includes(search.toLowerCase()) ||
            s.session_id.toLowerCase().includes(search.toLowerCase())
        )
    );
    
    onMount(async () => {
        try {
            const res = await fetch('/api/sessions');
            const data = await res.json();
            sessions = data.sessions;
        } catch (e) {
            console.error('Failed to fetch sessions:', e);
        } finally {
            loading = false;
        }
    });
</script>

<div class="space-y-6">
    <div class="flex justify-between items-center">
        <h2 class="text-2xl font-bold">Sessions</h2>
        <input 
            type="text" 
            placeholder="Search sessions..."
            bind:value={search}
            class="px-4 py-2 border rounded-lg w-64"
        />
    </div>
    
    {#if loading}
        <p>Loading...</p>
    {:else if filteredSessions.length === 0}
        <p class="text-gray-500">No sessions found</p>
    {:else}
        <div class="space-y-3">
            {#each filteredSessions as session}
                <SessionCard {session} />
            {/each}
        </div>
    {/if}
</div>
```

**Step 3: Test**

```bash
cd frontend
bun dev
# Navigate to /sessions
```

**Step 4: Commit**

```bash
git add frontend/
git commit -m "feat: add sessions page with Svelte 5 runes"
```

---

## Phase 4: Advanced Features

### Task 4.1: WebSocket Real-time Updates

**Objective:** Real-time status updates via WebSocket

**Files:**
- Create: `backend/src/routes/ws.rs`
- Create: `frontend/src/lib/stores/status.ts`

**Step 1: Create backend WebSocket handler**

```rust
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    Extension,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use crate::AppState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    
    // Send initial status
    let status = get_status(&state).await;
    let _ = sender.send(Message::Text(serde_json::to_string(&status).unwrap())).await;
    
    // Keep connection alive
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(_) => {}
            Message::Close(_) => break,
            _ => {}
        }
    }
}

async fn get_status(state: &AppState) -> serde_json::Value {
    serde_json::json!({
        "type": "status",
        "online": true,
        "model": "mimo-v2.5",
        "uptime": "3d 12h"
    })
}
```

**Step 2: Create frontend WebSocket store**

```typescript
// frontend/src/lib/stores/status.ts
import { writable } from 'svelte/store';
import { browser } from '$app/environment';

interface Status {
    online: boolean;
    model: string;
    uptime: string;
}

export const status = writable<Status>({
    online: false,
    model: 'Unknown',
    uptime: '0m'
});

let ws: WebSocket | null = null;

export function connectWebSocket() {
    if (!browser) return;
    
    // WebSocket URL — ganti dengan domain/IP kamu
    const wsUrl = 'ws://47.84.137.49/ws';
    ws = new WebSocket(wsUrl);
    
    ws.onmessage = (event) => {
        const data = JSON.parse(event.data);
        if (data.type === 'status') {
            status.set({
                online: data.online,
                model: data.model,
                uptime: data.uptime
            });
        }
    };
    
    ws.onclose = () => {
        setTimeout(connectWebSocket, 3000);
    };
}
```

**Step 3: Test**

```bash
cd backend && cargo run &
cd frontend && bun dev &
# Open browser, check WebSocket in DevTools
```

**Step 4: Commit**

```bash
git add backend/ frontend/
git commit -m "feat: add WebSocket real-time updates"
```

---

### Task 4.2: Cron Jobs Manager

**Objective:** API + UI untuk manage cron jobs

**Files:**
- Create: `backend/src/routes/cron.rs`
- Create: `frontend/src/routes/cron/+page.svelte`

**Step 1: Create cron API**

```rust
use axum::{Extension, Json};
use serde::Serialize;
use std::sync::Arc;
use crate::AppState;

#[derive(Serialize)]
pub struct CronJob {
    pub id: String,
    pub name: Option<String>,
    pub schedule: String,
    pub prompt: String,
    pub enabled: bool,
    pub last_run: Option<String>,
    pub next_run: Option<String>,
}

pub async fn list_jobs(
    Extension(_state): Extension<Arc<AppState>>,
) -> Json<Vec<CronJob>> {
    // TODO: Read from Hermes cron storage
    Json(vec![])
}
```

**Step 2: Create cron UI (Svelte 5)**

```svelte
<!-- frontend/src/routes/cron/+page.svelte -->
<script lang="ts">
    import { onMount } from 'svelte';
    
    let jobs = $state<any[]>([]);
    let loading = $state(true);
    
    onMount(async () => {
        try {
            const res = await fetch('/api/cron');
            jobs = await res.json();
        } catch (e) {
            console.error('Failed to fetch cron jobs:', e);
        } finally {
            loading = false;
        }
    });
</script>

<div class="space-y-6">
    <h2 class="text-2xl font-bold">Cron Jobs</h2>
    
    {#if loading}
        <p>Loading...</p>
    {:else if jobs.length === 0}
        <p class="text-gray-500">No cron jobs configured</p>
    {:else}
        <div class="space-y-3">
            {#each jobs as job}
                <div class="bg-white rounded-lg shadow-sm p-4">
                    <h3 class="font-semibold">{job.name || job.id}</h3>
                    <p class="text-sm text-gray-500">{job.schedule}</p>
                </div>
            {/each}
        </div>
    {/if}
</div>
```

**Step 3: Test**

```bash
# Navigate to /cron
```

**Step 4: Commit**

```bash
git add backend/ frontend/
git commit -m "feat: add cron jobs manager"
```

---

## Phase 5: Deployment

### Task 5.1: Build & Deploy Scripts

**Objective:** Scripts untuk build dan deploy

**Files:**
- Create: `scripts/build.sh`
- Create: `scripts/deploy-frontend.sh`
- Create: `scripts/deploy-backend.sh`

**Step 1: Create build.sh**

```bash
#!/bin/bash
set -e

echo "Building backend..."
cd backend
cargo build --release
cd ..

echo "Building frontend..."
cd frontend
bun run build
cd ..

echo "Build complete!"
echo "Backend: backend/target/release/hermes-dashboard-backend"
echo "Frontend: frontend/build/"
```

**Step 2: Create deploy-frontend.sh**

```bash
#!/bin/bash
set -e

ALIBABA_IP="47.84.137.49"
REMOTE_USER="ubuntu"
REMOTE_PATH="/var/www/hermes-dashboard"

echo "Deploying frontend to Alibaba..."

# Build frontend
cd frontend
bun run build
cd ..

# Copy to Alibaba
scp -i ~/.ssh/alibabakey.pem -r build/* ${REMOTE_USER}@${ALIBABA_IP}:${REMOTE_PATH}/

echo "Frontend deployed!"
echo "Access at: http://${ALIBABA_IP}"
```

**Step 3: Create deploy-backend.sh**

```bash
#!/bin/bash
set -e

echo "Starting backend..."

cd backend
cargo build --release

# Run backend
./target/release/hermes-dashboard-backend
```

**Step 4: Make executable**

```bash
chmod +x scripts/*.sh
```

**Step 5: Commit**

```bash
git add scripts/
git commit -m "feat: add build and deploy scripts"
```

---

### Task 5.2: Systemd Service (Backend)

**Objective:** Run backend sebagai systemd service di Tencent

**Files:**
- Create: `scripts/hermes-dashboard.service`

**Step 1: Create service file**

```ini
[Unit]
Description=Hermes Dashboard Backend
After=network.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/home/ubuntu/hermes-dashboard/backend
ExecStart=/home/ubuntu/hermes-dashboard/backend/target/release/hermes-dashboard-backend
Restart=always
RestartSec=5
Environment=RUST_LOG=info
Environment=PORT=3001

[Install]
WantedBy=multi-user.target
```

**Step 2: Install service**

```bash
sudo cp scripts/hermes-dashboard.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable hermes-dashboard
sudo systemctl start hermes-dashboard
```

**Step 3: Commit**

```bash
git add scripts/
git commit -m "feat: add systemd service for backend"
```

---

## Summary

### Versions
- **Rust Backend:** Axum 0.8.9, SQLx 0.9.0, tokio 1.52.3, serde 1.0.228
- **Frontend:** SvelteKit 2.61.1, Svelte 5.55.10, Tailwind CSS 4.3.0
- **Runtime:** Bun (tidak perlu Node.js)

### Total Tasks: 12
- Phase 1: Setup (3 tasks)
- Phase 2: Backend API (3 tasks)
- Phase 3: Frontend (3 tasks)
- Phase 4: Advanced (2 tasks)
- Phase 5: Deployment (2 tasks)

### Estimated Time: 1-2 minggu

### Deployment Notes

**Tencent (Backend):**
- Port 3001 (API + WebSocket)
- Systemd service
- Baca langsung `~/.hermes/state.db`

**Alibaba (Frontend):**
- Nginx serve static files
- Proxy `/api/*` ke Tencent:3001
- Proxy `/ws` ke Tencent:3001
- IP: 47.84.137.49

**Akses Dashboard:**
- https://hermes.vinrul.my.id (langsung via IP)
- Atau http://dashboard.example.com (kalau pakai domain)

### Next Steps
1. Review plan ini
2. Mulai Phase 1 (setup)
3. Test tiap phase sebelum lanjut
4. Deploy ke production

---

**Plan created:** 2026-05-29
**Author:** Hermes Agent (MiMo v2.5-pro)
**Status:** Ready for implementation

---

## 🌐 Deployment Architecture (Updated)

### Direct Connection (No Proxy)

```
Browser
   │
   ├─── HTTPS ──► hermes.vinrul.my.id (Alibaba)
   │              └── Nginx serve static files only
   │
   └─── HTTPS ──► api-hermes.vinrul.my.id (Tencent)
                  └── Nginx proxy ke Rust backend
```

**Key Changes:**
- Alibaba serve frontend **saja** (tidak ada /api proxy)
- Frontend langsung panggil `https://api-hermes.vinrul.my.id`
- CORS dikonfigurasi di Backend untuk allow Frontend origin
- Cookie di-set untuk domain `.vinrul.my.id` (bisa dipakai di semua subdomain)

**Environment Variables:**
```bash
FRONTEND_URL=https://hermes.vinrul.my.id
BACKEND_URL=https://api-hermes.vinrul.my.id
COOKIE_SECURE=true
```

**Frontend API Base URL:**
```typescript
export const API_BASE_URL = 'https://api-hermes.vinrul.my.id';
```


---

## 📱 Responsive Design Requirements

### Overview

Dashboard harus **responsive** dan bisa diakses dari:
- **Desktop** (1920px+)
- **Laptop** (1366px - 1919px)
- **Tablet/iPad** (768px - 1365px)
- **Mobile/iPhone/Android** (320px - 767px)

### Breakpoints

```css
/* Tailwind CSS v4 breakpoints */
sm: 640px    /* Small phones landscape */
md: 768px    /* Tablets portrait */
lg: 1024px   /* Tablets landscape, small laptops */
xl: 1280px   /* Laptops, desktops */
2xl: 1536px  /* Large desktops */
```

### Layout Rules

#### Desktop (≥1024px)
```
┌─────────────────────────────────────────────────────────┐
│  ┌──────────┐  ┌──────────────────────────────────────┐ │
│  │          │  │                                      │ │
│  │ Sidebar  │  │           Main Content               │ │
│  │ 264px    │  │                                      │ │
│  │          │  │                                      │ │
│  └──────────┘  └──────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

- Sidebar: Fixed, 264px width
- Main content: Flex-grow
- Cards: 4 columns grid

#### Tablet (768px - 1023px)
```
┌─────────────────────────────────────────┐
│  ┌───────────────────────────────────┐  │
│  │         Header + Menu Icon        │  │
│  └───────────────────────────────────┘  │
│  ┌───────────────────────────────────┐  │
│  │                                   │  │
│  │         Main Content              │  │
│  │                                   │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

- Sidebar: Hidden, toggle via hamburger menu
- Main content: Full width
- Cards: 2 columns grid

#### Mobile (<768px)
```
┌─────────────────────┐
│  ┌─────────────────┐│
│  │ Header + Menu   ││
│  └─────────────────┘│
│  ┌─────────────────┐│
│  │                 ││
│  │ Main Content    ││
│  │                 ││
│  └─────────────────┘│
└─────────────────────┘
```

- Sidebar: Hidden, toggle via hamburger menu
- Main content: Full width
- Cards: 1 column stack

### Component Responsive Rules

#### Sidebar (shared/components/Sidebar.svelte)

```svelte
<script lang="ts">
    import { page } from '$app/stores';
    
    let isOpen = $state(false);
    let isDesktop = $state(true);
    
    // Check screen size on mount
    import { onMount } from 'svelte';
    
    onMount(() => {
        const checkSize = () => {
            isDesktop = window.innerWidth >= 1024;
            if (isDesktop) isOpen = true;
        };
        
        checkSize();
        window.addEventListener('resize', checkSize);
        
        return () => window.removeEventListener('resize', checkSize);
    });
    
    const navItems = [
        { href: '/', label: 'Dashboard', icon: '📊' },
        { href: '/sessions', label: 'Sessions', icon: '💬' },
        { href: '/cron', label: 'Cron Jobs', icon: '⏰' },
        { href: '/tools', label: 'Tools', icon: '🔧' },
        { href: '/settings', label: 'Settings', icon: '⚙️' },
    ];
</script>

<!-- Mobile menu button -->
<button 
    class="lg:hidden fixed top-4 left-4 z-50 p-2 bg-gray-900 text-white rounded-lg"
    on:click={() => isOpen = !isOpen}
>
    {isOpen ? '✕' : '☰'}
</button>

<!-- Overlay for mobile -->
{#if isOpen && !isDesktop}
    <div 
        class="fixed inset-0 bg-black bg-opacity-50 z-40"
        on:click={() => isOpen = false}
    ></div>
{/if}

<!-- Sidebar -->
<aside class="fixed lg:static inset-y-0 left-0 z-40 w-64 bg-gray-900 text-white p-4
              transform transition-transform duration-200 ease-in-out
              {isOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'}">
    <div class="mb-8">
        <h1 class="text-2xl font-bold">🤖 Hermes</h1>
        <p class="text-gray-400 text-sm">Dashboard</p>
    </div>
    
    <nav>
        {#each navItems as item}
            <a 
                href={item.href}
                class="flex items-center gap-3 px-4 py-3 rounded-lg mb-2 transition-colors
                       {$page.url.pathname === item.href 
                         ? 'bg-blue-600 text-white' 
                         : 'text-gray-300 hover:bg-gray-800'}"
                on:click={() => !isDesktop && (isOpen = false)}
            >
                <span>{item.icon}</span>
                <span>{item.label}</span>
            </a>
        {/each}
    </nav>
</aside>
```

#### Stats Cards (features/dashboard/components/StatsCard.svelte)

```svelte
<script lang="ts">
    let { title, value, icon, trend = 'neutral' }: {
        title: string;
        value: string | number;
        icon: string;
        trend?: 'up' | 'down' | 'neutral';
    } = $props();
</script>

<div class="bg-white rounded-xl shadow-sm p-4 sm:p-6">
    <div class="flex items-center justify-between mb-3 sm:mb-4">
        <span class="text-2xl sm:text-3xl">{icon}</span>
        {#if trend === 'up'}
            <span class="text-green-500 text-xs sm:text-sm">↑ +12%</span>
        {:else if trend === 'down'}
            <span class="text-red-500 text-xs sm:text-sm">↓ -5%</span>
        {/if}
    </div>
    <p class="text-gray-500 text-xs sm:text-sm">{title}</p>
    <p class="text-2xl sm:text-3xl font-bold mt-1">{value}</p>
</div>
```

#### Dashboard Grid (routes/+page.svelte)

```svelte
<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6">
    <StatsCard ... />
    <StatsCard ... />
    <StatsCard ... />
    <StatsCard ... />
</div>
```

#### Session Cards (features/sessions/components/SessionCard.svelte)

```svelte
<div class="bg-white rounded-lg shadow-sm p-3 sm:p-4 hover:shadow-md transition-shadow">
    <div class="flex flex-col sm:flex-row sm:justify-between sm:items-start gap-2">
        <div>
            <h3 class="font-semibold text-base sm:text-lg">
                {session.title || 'Untitled Session'}
            </h3>
            <p class="text-gray-500 text-xs sm:text-sm mt-1">
                {session.source || 'unknown'} • {session.message_count || 0} messages
            </p>
        </div>
        <span class="text-xs text-gray-400">
            {session.created_at ? new Date(session.created_at).toLocaleDateString() : 'Unknown'}
        </span>
    </div>
    <p class="text-xs text-gray-400 mt-2 font-mono">{session.session_id}</p>
</div>
```

#### Search Input

```svelte
<input 
    type="text" 
    placeholder="Search sessions..."
    bind:value={search}
    class="w-full sm:w-64 px-3 sm:px-4 py-2 border rounded-lg text-sm"
/>
```

### Font Sizes

| Element | Mobile | Tablet | Desktop |
|---------|--------|--------|---------|
| Page title | text-xl | text-2xl | text-2xl |
| Card title | text-base | text-lg | text-lg |
| Body text | text-sm | text-sm | text-base |
| Small text | text-xs | text-xs | text-sm |

### Spacing

| Element | Mobile | Tablet | Desktop |
|---------|--------|--------|---------|
| Page padding | p-4 | p-5 | p-6 |
| Card padding | p-4 | p-5 | p-6 |
| Card gap | gap-4 | gap-5 | gap-6 |
| Section gap | space-y-4 | space-y-5 | space-y-6 |

### Testing Checklist

- [ ] Desktop (1920x1080) - Chrome, Firefox, Safari
- [ ] Laptop (1366x768) - Chrome, Firefox
- [ ] iPad (1024x768) - Safari
- [ ] iPad Mini (768x1024) - Safari
- [ ] iPhone (375x812) - Safari
- [ ] Android (360x800) - Chrome
- [ ] Small phone (320x568) - Chrome

### Tools for Testing

```bash
# Chrome DevTools
F12 → Toggle Device Toolbar (Ctrl+Shift+M)

# Responsive design mode
- iPhone SE
- iPhone 14 Pro
- iPad Air
- iPad Mini
- Samsung Galaxy S20
- Pixel 7
```



---

## Phase 6: Security & Authentication

### Task 6.1: JWT Authentication Setup
**Objective:** Setup JWT token generation & validation

**Files:**
- Create: `backend/src/features/auth/mod.rs`
- Create: `backend/src/features/auth/jwt.rs`

**Steps:**
1. Add `jsonwebtoken` crate to Cargo.toml
2. Create JWT helper (generate, validate, decode)
3. Add JWT_SECRET to .env
4. Create token refresh logic

---

### Task 6.2: Login API
**Objective:** Login endpoint untuk akses dashboard

**Files:**
- Create: `backend/src/features/auth/handler.rs`
- Create: `backend/src/features/auth/dto.rs`

**Steps:**
1. Create POST /api/auth/login endpoint
2. Validate username/password from config
3. Generate JWT token (access + refresh)
4. Return token in HttpOnly cookie

---

### Task 6.3: Middleware Authorization
**Objective:** Protected routes middleware

**Files:**
- Create: `backend/src/middleware/auth.rs`

**Steps:**
1. Create auth middleware extractor
2. Validate JWT from cookie/header
3. Reject unauthorized requests (401)
4. Apply to protected routes

---

### Task 6.4: Rate Limiting
**Objective:** Prevent abuse with rate limiting

**Files:**
- Create: `backend/src/middleware/rate_limit.rs`

**Steps:**
1. ~~Add `governor` crate for rate limiting~~ Implemented with custom sliding window
2. Configure limits per endpoint
3. Apply to login & API endpoints
4. Return 429 Too Many Requests
5. Rate limit values configurable via `.env` (`RATE_LIMIT_LOGIN_MAX`, `RATE_LIMIT_API_MAX`)

**Status:** ✅ Complete — Login: 5 req/min, API: 60 req/min (configurable)

---

### Task 6.5: Input Validation & Sanitization
**Objective:** Validate and sanitize all inputs

**Files:**
- Create: `backend/src/shared/validation.rs`

**Steps:**
1. Add `validator` crate with derive
2. Create validation rules for DTOs
3. Sanitize string inputs
4. Return 400 Bad Request for invalid input

---

## Phase 7: Responsive Design

### Task 7.1: Mobile-First Layout
**Objective:** Base layout yang responsive

**Files:**
- Modify: `frontend/src/routes/+layout.svelte`
- Create: `frontend/src/lib/shared/components/Layout.svelte`

**Steps:**
1. Create mobile-first sidebar/header
2. Implement collapsible navigation
3. Add responsive breakpoints
4. Test on mobile/tablet/desktop

---

### Task 7.2: Responsive Navigation
**Objective:** Navigation yang adaptif

**Files:**
- Create: `frontend/src/lib/shared/components/Navbar.svelte`
- Create: `frontend/src/lib/shared/components/Sidebar.svelte`

**Steps:**
1. Mobile: hamburger menu
2. Tablet: collapsible sidebar
3. Desktop: fixed sidebar
4. Add active state indicators

---

### Task 7.3: Touch-Friendly Components
**Objective:** Components yang nyaman di mobile

**Files:**
- Modify: `frontend/src/lib/shared/components/*.svelte`

**Steps:**
1. Increase touch targets (min 44px)
2. Add swipe gestures for cards
3. Implement pull-to-refresh
4. Add haptic feedback (if supported)

---

### Task 7.4: Dark/Light Theme
**Objective:** Theme switching

**Files:**
- Create: `frontend/src/lib/shared/utils/theme.ts`
- Modify: `frontend/src/app.html`

**Steps:**
1. Create theme context/store
2. Implement CSS variables for themes
3. Add toggle button in navbar
4. Persist preference in localStorage

---

## Phase 8: Testing

### Task 8.1: Backend Unit Tests
**Objective:** Unit tests untuk backend logic

**Files:**
- Create: `backend/src/features/*/tests.rs`

**Steps:**
1. Test JWT generation/validation
2. Test config parsing
3. Test database queries
4. Test error handling

**Run:**
```bash
cd backend && cargo test
```

---

### Task 8.2: API Integration Tests
**Objective:** Test API endpoints end-to-end

**Files:**
- Create: `backend/tests/api_tests.rs`

**Steps:**
1. Setup test database
2. Test health endpoint
3. Test sessions list
4. Test stats overview
5. Test auth endpoints

---

### Task 8.3: Frontend Component Tests
**Objective:** Test Svelte components

**Files:**
- Create: `frontend/src/lib/**/*.test.ts`

**Steps:**
1. Setup Vitest + Testing Library
2. Test StatsCard component
3. Test SessionCard component
4. Test Navigation component

**Run:**
```bash
cd frontend && npm test
```

---

### Task 8.4: Accessibility Tests
**Objective:** End-to-end user flows

**Files:**
- Create: `frontend/tests/e2e/`

**Steps:**
1. Setup Playwright
2. Test login flow
3. Test dashboard load
4. Test sessions page
5. Test mobile responsiveness

---

## Phase 9: Performance & Optimization

### Task 9.1: API Response Caching
**Objective:** Cache API responses

**Files:**
- Create: `backend/src/middleware/cache.rs`

**Steps:**
1. Add caching layer (in-memory)
2. Cache GET /api/stats (5 min)
3. Cache GET /api/sessions (1 min)
4. Add cache invalidation

---

### Task 9.2: Frontend Lazy Loading
**Objective:** Lazy load routes & components

**Files:**
- Modify: `frontend/src/routes/**/*.svelte`

**Steps:**
1. Implement dynamic imports
2. Lazy load chart components
3. Add loading skeletons
4. Implement prefetch on hover

---

### Task 9.3: Database Query Optimization
**Objective:** Optimize SQLite queries

**Files:**
- Modify: `backend/src/features/*/repository.rs`

**Steps:**
1. Add database indexes
2. Optimize N+1 queries
3. Use EXPLAIN QUERY PLAN
4. Add connection pooling config

---

### Task 9.4: Asset Optimization
**Objective:** Optimize frontend assets

**Files:**
- Modify: `frontend/vite.config.ts`

**Steps:**
1. Enable tree shaking
2. Minify JS/CSS
3. Optimize images
4. Enable gzip/brotli compression
5. Add cache headers

## Phase 10: Remote Control (Tools Menu)

### Task 10.1: Switch Model
**Objective:** Ganti model AI dari web UI tanpa SSH

**Files:**
- Create: `backend/src/features/tools/mod.rs`
- Create: `backend/src/features/tools/handler.rs`
- Create: `backend/src/features/tools/dto.rs`
- Create: `frontend/src/routes/tools/+page.svelte`
- Create: `frontend/src/lib/features/tools/components/ModelSwitcher.svelte`

**Steps:**
1. Create backend API `GET /api/tools/models` — list available models from config
2. Create backend API `POST /api/tools/switch-model` — update model in config.yaml
3. Read available models from `~/.hermes/config.yaml` (model section)
4. Create ModelSwitcher component — dropdown with current model + available options
5. Show confirmation before switching
6. Display success/error message after switch
7. Add model status indicator (current model in header updates)

**Notes:**
- Only edit `config.yaml` (model section), NOT credentials
- Require admin auth for this action
- Log model changes for audit trail

---

### Task 10.2: Tool Manager
**Objective:** Enable/disable tools dari web UI

**Files:**
- Create: `backend/src/features/tools/handler.rs` (add endpoints)
- Create: `frontend/src/lib/features/tools/components/ToolManager.svelte`

**Steps:**
1. Create backend API `GET /api/tools/list` — list all available tools with status
2. Create backend API `POST /api/tools/toggle` — enable/disable a tool
3. Read tools from Hermes tool registry or config
4. Create ToolManager component — list with toggle switches
5. Group tools by category (file, web, terminal, etc.)
6. Show tool description and current status
7. Require admin auth for toggle actions

---

### Task 10.3: Send Message
**Objective:** Kirim pesan ke agent langsung dari web UI

**Files:**
- Create: `backend/src/features/tools/handler.rs` (add endpoint)
- Create: `frontend/src/lib/features/tools/components/MessageSender.svelte`

**Steps:**
1. Create backend API `POST /api/tools/send-message` — send message to agent
2. Integrate with Hermes message queue or CLI
3. Create MessageSender component — text input + send button
4. Show message history (last N messages)
5. Display agent response in real-time (via WebSocket)
6. Require admin auth for sending messages

---

### Task 10.4: Gateway Control
**Objective:** Restart/status control untuk Hermes gateway

**Files:**
- Create: `backend/src/features/tools/handler.rs` (add endpoints)
- Create: `frontend/src/lib/features/tools/components/GatewayControl.svelte`

**Steps:**
1. Create backend API `GET /api/tools/gateway/status` — check gateway status
2. Create backend API `POST /api/tools/gateway/restart` — restart gateway
3. Execute `hermes` CLI commands via backend
4. Create GatewayControl component — status indicator + restart button
5. Show gateway uptime, connected platforms, active sessions
6. Confirmation dialog before restart
7. Require admin auth for restart action
8. Show real-time status updates via WebSocket

---

### Tools Page Layout

```
┌─────────────────────────────────────────────┐
│  🔧 Tools                                   │
├─────────────────────────────────────────────┤
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │  🔄 Switch Model                    │   │
│  │  Current: mimo-v2.5                 │   │
│  │  [MiMo v2.5 ▼]  [Switch]           │   │
│  └─────────────────────────────────────┘   │
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │  🛠️ Tool Manager                    │   │
│  │  ✅ Terminal      [ON]              │   │
│  │  ✅ Web Search    [ON]              │   │
│  │  ❌ File Write    [OFF]             │   │
│  └─────────────────────────────────────┘   │
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │  💬 Send Message                    │   │
│  │  [Type message here...]     [Send]  │   │
│  │  > Last: "Check crypto prices"      │   │
│  └─────────────────────────────────────┘   │
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │  ⚡ Gateway Control                 │   │
│  │  Status: Online ✅                  │   │
│  │  Uptime: 2d 5h                      │   │
│  │  Platforms: Telegram, Discord       │   │
│  │  [Restart Gateway]                  │   │
│  └─────────────────────────────────────┘   │
│                                             │
└─────────────────────────────────────────────┘
```

### Phase 11: Worker Management (Multi-Node)
- [x] Task 11.1: Worker Registration API — node register ke dashboard
- [x] Task 11.2: Worker Status API — heartbeat & status tracking
- [ ] Task 11.3: Worker List UI — tampilkan semua registered nodes
- [ ] Task 11.4: Worker Config API — change model/provider dari dashboard
- [ ] Task 11.5: Worker Config UI — dropdown model + apply button
- [ ] Task 11.6: Worker Task History — tampilkan task logs per node
- [ ] Task 11.7: Real-time Status — WebSocket update status nodes
- [ ] Task 11.8: Worker Health Check — auto-detect offline nodes

---

## Phase 11: Worker Management (Multi-Node)

> Goal: Dashboard sebagai central control untuk semua Hermes instances.
> Monitor status, change config, dan manage tasks untuk multiple nodes.

### Task 11.1: Worker Registration API

Endpoint: POST /api/workers/register

Request Body:
- name: worker name (e.g. "windows")
- ip: Tailscale IP (e.g. "100.82.105.93")
- role: "orchestrator" or "worker"
- os: "windows", "linux", "macos"
- arch: "x86_64", "aarch64"
- ram_total: total RAM in MB
- disk_total: total disk in MB
- capabilities: array of capabilities (e.g. ["rust", "bun", "codex-cli"])

Database Table: workers
- id, name, ip, role, os, arch, ram_total, disk_total
- capabilities (JSON), status, last_heartbeat, registered_at, config (JSON)

### Task 11.2: Worker Status API

Endpoints:
- POST /api/workers/:id/heartbeat — update status + metrics
- GET /api/workers — list all workers
- GET /api/workers/:id — get worker detail

Heartbeat Data:
- status: "online", "busy", "offline"
- current_task: description of current task
- ram_used: used RAM in MB
- disk_used: used disk in MB
- active_model: current model name

### Task 11.3: Worker List UI

Frontend page showing all registered nodes with:
- Name, IP, status indicator (green/yellow/red)
- RAM/Disk usage bars
- Current model and role
- Last heartbeat time
- Action buttons: Configure, View Tasks

### Task 11.4: Worker Config API

Endpoint: PUT /api/workers/:id/config

Request Body:
- model: model name (e.g. "deepseek-v4")
- provider: provider name (e.g. "deepseek")
- max_tokens: max tokens setting
- temperature: temperature setting

Response: success status + applied timestamp

### Task 11.5: Worker Config UI

Frontend form for configuring a worker:
- Model dropdown (populated from available models)
- Provider dropdown
- Max tokens input
- Temperature slider
- Apply button sends PUT to /api/workers/:id/config

### Task 11.6: Worker Task History

Endpoint: GET /api/workers/:id/tasks

Returns list of tasks executed by the worker:
- task_id, description, status (pending/running/completed/failed)
- started_at, completed_at, result

Frontend: table or timeline view of task history

### Task 11.7: Real-time Status

WebSocket endpoint: ws://api:3001/ws/workers

Events emitted:
- worker:status — status change (online/offline/busy)
- worker:heartbeat — periodic heartbeat data
- worker:task_update — task progress update

Frontend subscribes and updates UI in real-time.

### Task 11.8: Worker Health Check

Logic:
- Each worker sends heartbeat every 30 seconds
- If no heartbeat for 2 minutes → status: offline
- Dashboard auto-refreshes via WebSocket
- Notification when worker goes offline

Endpoint: GET /api/workers/health

Returns: total_workers, online count, offline count, list of offline workers

---

Dependencies:
- Phase 10 (Remote Control) — Done
- WebSocket (Phase 4) — Done
- JWT Auth (Phase 6) — Done

Estimated Tasks: 8 tasks
Estimated Effort: 2-3 hari
