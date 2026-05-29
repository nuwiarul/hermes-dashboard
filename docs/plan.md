# Hermes Dashboard — Implementation Plan

> **For Hermes:** Use subagent-driven-development skill to implement this plan task-by-task.

**Goal:** Web dashboard untuk monitor & kontrol Hermes Agent — session viewer, cost tracker, tool analytics, cron manager, dan remote control.

**Architecture:** Rust backend (Axum) yang baca Hermes state.db + config + logs, expose REST API + WebSocket ke SvelteKit frontend. Deploy di Tencent server (2GB RAM) dengan Cloudflare Tunnel untuk akses dari iPad/mobile.

**Tech Stack:**
- Backend: Rust, Axum, SQLx, tokio, serde
- Frontend: SvelteKit, Tailwind CSS, Chart.js
- Database: SQLite (baca langsung dari `~/.hermes/state.db`)
- Deploy: Tencent server + Cloudflare Tunnel

---

## Phase 1: Project Setup & Foundation

### Task 1.1: Initialize Rust Backend

**Objective:** Setup Rust project dengan Axum web framework

**Files:**
- Create: `backend/Cargo.toml`
- Create: `backend/src/main.rs`
- Create: `backend/src/config.rs`
- Create: `backend/src/routes/mod.rs`

**Step 1: Create Cargo.toml**

```toml
[package]
name = "hermes-dashboard-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = { version = "0.7", features = ["ws", "macros"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
tower-http = { version = "0.5", features = ["cors", "fs"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
```

**Step 2: Create main.rs**

```rust
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod config;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::init();

    let app = Router::new()
        .route("/api/health", get(routes::health::handler))
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

**Step 4: Create routes/mod.rs**

```rust
pub mod health;
pub mod sessions;
pub mod stats;
pub mod config;
pub mod cron;
```

**Step 5: Create routes/health.rs**

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

**Step 6: Test**

```bash
cd backend
cargo run
# In another terminal:
curl http://localhost:3001/api/health
# Expected: {"status":"ok","service":"hermes-dashboard","version":"0.1.0"}
```

**Step 7: Commit**

```bash
git add backend/
git commit -m "feat: initialize Rust backend with Axum"
```

---

### Task 1.2: Initialize SvelteKit Frontend

**Objective:** Setup SvelteKit project dengan Tailwind CSS

**Files:**
- Create: `frontend/` (via `npx sv create`)

**Step 1: Create SvelteKit project**

```bash
cd ~/hermes-dashboard
npx sv@latest create frontend
# Select: Skeleton project, TypeScript, Tailwind CSS
cd frontend
bun install
```

**Step 2: Add Chart.js**

```bash
bun add chart.js svelte-chartjs
```

**Step 3: Setup API proxy in vite.config.ts**

```typescript
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [sveltekit()],
    server: {
        proxy: {
            '/api': {
                target: 'http://localhost:3001',
                changeOrigin: true
            }
        }
    }
});
```

**Step 4: Test**

```bash
cd frontend
bun dev
# Open http://localhost:5173 — should see SvelteKit welcome page
```

**Step 5: Commit**

```bash
git add frontend/
git commit -m "feat: initialize SvelteKit frontend with Tailwind"
```

---

### Task 1.3: Setup Database Connection

**Objective:** Connect ke Hermes state.db SQLite

**Files:**
- Create: `backend/src/db.rs`
- Modify: `backend/src/main.rs`

**Step 1: Create db.rs**

```rust
use sqlx::sqlite::SqlitePool;
use std::path::Path;

pub async fn connect(db_path: &Path) -> anyhow::Result<SqlitePool> {
    let url = format!("sqlite:{}?mode=ro", db_path.display());
    let pool = SqlitePool::connect(&url).await?;
    Ok(pool)
}
```

**Step 2: Update main.rs**

```rust
use axum::{routing::get, Router, Extension};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use std::sync::Arc;

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
        .layer(Extension(state))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::info!("Server running on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
```

**Step 3: Test**

```bash
cd backend
cargo run
curl http://localhost:3001/api/health
```

**Step 4: Commit**

```bash
git add backend/
git commit -m "feat: add SQLite database connection"
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
use chrono::NaiveDateTime;

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

**Objective:** API untuk overview statistics (total sessions, messages, dll)

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

    // Simple YAML parsing for model info
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

**Objective:** Buat layout utama dengan sidebar navigasi

**Files:**
- Create: `frontend/src/routes/+layout.svelte`
- Create: `frontend/src/lib/components/Sidebar.svelte`
- Create: `frontend/src/lib/components/Header.svelte`

**Step 1: Create Sidebar.svelte**

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
    export let status: 'online' | 'offline' = 'online';
    export let model: string = 'Unknown';
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
git commit -m "feat: add dashboard layout with sidebar"
```

---

### Task 3.2: Dashboard Home Page

**Objective:** Halaman utama dengan stats cards dan chart

**Files:**
- Create: `frontend/src/routes/+page.svelte`
- Create: `frontend/src/lib/components/StatsCard.svelte`
- Create: `frontend/src/lib/components/UsageChart.svelte`

**Step 1: Create StatsCard.svelte**

```svelte
<script lang="ts">
    export let title: string;
    export let value: string | number;
    export let icon: string;
    export let trend: 'up' | 'down' | 'neutral' = 'neutral';
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

**Step 2: Create +page.svelte**

```svelte
<script lang="ts">
    import StatsCard from '$lib/components/StatsCard.svelte';
    import { onMount } from 'svelte';
    
    let stats = {
        total_sessions: 0,
        total_messages: 0,
        sessions_today: 0,
        messages_today: 0,
    };
    
    let loading = true;
    
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
# Should see stats cards (values might be 0 if state.db is empty)
```

**Step 4: Commit**

```bash
git add frontend/
git commit -m "feat: add dashboard home page with stats"
```

---

### Task 3.3: Sessions Page

**Objective:** Halaman untuk browse dan search sessions

**Files:**
- Create: `frontend/src/routes/sessions/+page.svelte`
- Create: `frontend/src/lib/components/SessionCard.svelte`

**Step 1: Create SessionCard.svelte**

```svelte
<script lang="ts">
    export let session: {
        session_id: string;
        title: string | null;
        source: string | null;
        created_at: string | null;
        message_count: number | null;
    };
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

**Step 2: Create sessions/+page.svelte**

```svelte
<script lang="ts">
    import SessionCard from '$lib/components/SessionCard.svelte';
    import { onMount } from 'svelte';
    
    let sessions: any[] = [];
    let loading = true;
    let search = '';
    
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
    
    $: filteredSessions = sessions.filter(s => 
        search === '' || 
        s.title?.toLowerCase().includes(search.toLowerCase()) ||
        s.session_id.toLowerCase().includes(search.toLowerCase())
    );
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
# Navigate to /sessions — should see session list
```

**Step 4: Commit**

```bash
git add frontend/
git commit -m "feat: add sessions page with search"
```

---

## Phase 4: Advanced Features

### Task 4.1: WebSocket Real-time Updates

**Objective:** Real-time status updates via WebSocket

**Files:**
- Create: `backend/src/routes/ws.rs`
- Modify: `backend/src/main.rs`
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
use tokio::sync::broadcast;
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
            Message::Text(text) => {
                // Handle client messages if needed
            }
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

**Step 2: Add to main.rs**

```rust
.route("/ws", get(routes::ws::ws_handler))
```

**Step 3: Create frontend WebSocket store**

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
    
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    ws = new WebSocket(`${protocol}//${window.location.host}/ws`);
    
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
        setTimeout(connectWebSocket, 3000); // Reconnect
    };
}
```

**Step 4: Test**

```bash
cd backend && cargo run &
cd frontend && bun dev &
# Open browser, check WebSocket connection in DevTools
```

**Step 5: Commit**

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
    // Read from Hermes cron storage
    // This is a simplified version — actual implementation
    // would parse the cron jobs from Hermes state
    Json(vec![])
}
```

**Step 2: Create cron UI**

```svelte
<!-- frontend/src/routes/cron/+page.svelte -->
<script lang="ts">
    import { onMount } from 'svelte';
    
    let jobs: any[] = [];
    let loading = true;
    
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
    
    async function toggleJob(id: string, enable: boolean) {
        // API call to enable/disable job
    }
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
                <div class="bg-white rounded-lg shadow-sm p-4 flex justify-between items-center">
                    <div>
                        <h3 class="font-semibold">{job.name || job.id}</h3>
                        <p class="text-sm text-gray-500">{job.schedule}</p>
                    </div>
                    <button 
                        on:click={() => toggleJob(job.id, !job.enabled)}
                        class="px-4 py-2 rounded {job.enabled ? 'bg-green-500' : 'bg-gray-400'}"
                    >
                        {job.enabled ? 'Enabled' : 'Disabled'}
                    </button>
                </div>
            {/each}
        </div>
    {/if}
</div>
```

**Step 3: Test**

```bash
# Navigate to /cron in browser
```

**Step 4: Commit**

```bash
git add backend/ frontend/
git commit -m "feat: add cron jobs manager"
```

---

## Phase 5: Deployment

### Task 5.1: Build & Deploy Scripts

**Objective:** Scripts untuk build dan deploy ke Tencent server

**Files:**
- Create: `scripts/build.sh`
- Create: `scripts/deploy.sh`

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
```

**Step 2: Create deploy.sh**

```bash
#!/bin/bash
set -e

SERVER="localhost"
PORT=3001

echo "Starting Hermes Dashboard..."

# Start backend
cd backend
./target/release/hermes-dashboard-backend &
BACKEND_PID=$!

# Serve frontend static files (or use reverse proxy)
cd ../frontend
bun run preview --port 4173 &
FRONTEND_PID=$!

echo "Backend running on port $PORT (PID: $BACKEND_PID)"
echo "Frontend running on port 4173 (PID: $FRONTEND_PID)"
echo ""
echo "Dashboard available at: http://$SERVER:4173"

# Wait for Ctrl+C
trap "kill $BACKEND_PID $FRONTEND_PID; exit" SIGINT SIGTERM
wait
```

**Step 3: Make executable**

```bash
chmod +x scripts/*.sh
```

**Step 4: Commit**

```bash
git add scripts/
git commit -m "feat: add build and deploy scripts"
```

---

### Task 5.2: Systemd Service

**Objective:** Run dashboard sebagai systemd service

**Files:**
- Create: `scripts/hermes-dashboard.service`

**Step 1: Create service file**

```ini
[Unit]
Description=Hermes Dashboard
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
git commit -m "feat: add systemd service file"
```

---

## Summary

### Total Tasks: 12
- Phase 1: Setup (3 tasks)
- Phase 2: Backend API (3 tasks)
- Phase 3: Frontend (3 tasks)
- Phase 4: Advanced (2 tasks)
- Phase 5: Deployment (2 tasks)

### Estimated Time: 1-2 minggu

### Next Steps
1. Review plan ini
2. Mulai Phase 1 (setup)
3. Test tiap phase sebelum lanjut
4. Deploy ke Tencent server
5. Setup Cloudflare Tunnel untuk akses dari iPad

---

**Plan created:** $(date)
**Author:** Hermes Agent
**Status:** Ready for implementation
