# Hermes Dashboard — Implementation Plan

> **For Hermes:** Use subagent-driven-development skill to implement this plan task-by-task.

**Goal:** Web dashboard untuk monitor & kontrol Hermes Agent — session viewer, cost tracker, tool analytics, cron manager, dan remote control.

---

## ✅ Task Checklist

### Phase 1: Project Setup & Foundation
- [ ] Task 1.1: Initialize Rust Backend
- [ ] Task 1.2: Initialize SvelteKit Frontend (SPA Mode)
- [ ] Task 1.3: Nginx Configuration (Alibaba Server)

### Phase 2: Core API Endpoints
- [ ] Task 2.1: Sessions List API
- [ ] Task 2.2: Stats Overview API
- [ ] Task 2.3: Config Reader API

### Phase 3: Frontend Pages
- [ ] Task 3.1: Dashboard Layout
- [ ] Task 3.2: Dashboard Home Page
- [ ] Task 3.3: Sessions Page

### Phase 4: Advanced Features
- [ ] Task 4.1: WebSocket Real-time Updates
- [ ] Task 4.2: Cron Jobs Manager

### Phase 5: Deployment
- [ ] Task 5.1: Build & Deploy Scripts
- [ ] Task 5.2: Systemd Service (Backend)

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
        proxy_pass http://43.156.247.129:3001;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # WebSocket proxy
    location /ws {
        proxy_pass http://43.156.247.129:3001;
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
- http://47.84.137.49 (langsung via IP)
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
