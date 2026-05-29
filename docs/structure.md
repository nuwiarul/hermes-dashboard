## 📁 Project Structure

### Overview

```
hermes-dashboard/
├── README.md
├── docs/
│   ├── plan.md
│   └── structure.md
├── backend/
│   └── src/
│       ├── main.rs
│       ├── config.rs
│       ├── db.rs
│       ├── features/              # Feature-based modules
│       │   ├── mod.rs
│       │   ├── sessions/
│       │   ├── stats/
│       │   ├── config/
│       │   ├── cron/
│       │   └── ws/
│       └── shared/                # Shared utilities
│           ├── mod.rs
│           └── error.rs
├── frontend/
│   └── src/
│       ├── app.html
│       ├── app.css
│       ├── routes/                # SvelteKit file-based routing
│       │   ├── +layout.svelte
│       │   ├── +layout.ts
│       │   ├── +page.svelte       # Dashboard home
│       │   ├── sessions/
│       │   ├── cron/
│       │   ├── tools/
│       │   └── settings/
│       └── lib/
│           ├── features/          # Feature-based modules
│           │   ├── dashboard/
│           │   ├── sessions/
│           │   ├── cron/
│           │   ├── tools/
│           │   └── settings/
│           └── shared/            # Shared components & utilities
│               ├── components/
│               ├── stores/
│               ├── types/
│               └── utils/
└── scripts/
    ├── build.sh
    ├── deploy-frontend.sh
    ├── deploy-backend.sh
    ├── hermes-dashboard.service
    └── nginx/
        └── hermes-dashboard.conf
```

---

## 🔙 Backend Structure (Rust + Axum)

### Full Tree

```
backend/
├── Cargo.toml
└── src/
    ├── main.rs                    # Entry point, router, AppState
    ├── config.rs                  # AppConfig (env vars, paths)
    ├── db.rs                      # SQLite connection pool
    │
    ├── features/
    │   ├── mod.rs                 # Re-exports all features
    │   │
    │   ├── sessions/              # Feature: Sessions
    │   │   ├── mod.rs             # Module exports
    │   │   ├── dto.rs             # SessionDto, SessionSummaryDto
    │   │   ├── handler.rs         # list_sessions() handler
    │   │   └── repository.rs      # query_sessions(), count_messages()
    │   │
    │   ├── stats/                 # Feature: Statistics
    │   │   ├── mod.rs
    │   │   ├── dto.rs             # StatsOverviewDto, SourceCountDto
    │   │   ├── handler.rs         # get_stats() handler
    │   │   └── repository.rs      # aggregate_sessions(), aggregate_messages()
    │   │
    │   ├── config/                # Feature: Config Reader
    │   │   ├── mod.rs
    │   │   ├── dto.rs             # ConfigInfoDto
    │   │   ├── handler.rs         # get_config() handler
    │   │   └── repository.rs      # read_config_yaml()
    │   │
    │   ├── cron/                  # Feature: Cron Jobs
    │   │   ├── mod.rs
    │   │   ├── dto.rs             # CronJobDto
    │   │   ├── handler.rs         # list_jobs(), toggle_job()
    │   │   └── repository.rs      # read_cron_jobs()
    │   │
    │   └── ws/                    # Feature: WebSocket
    │       ├── mod.rs
    │       ├── dto.rs             # WsMessage, StatusUpdate
    │       ├── handler.rs         # ws_handler(), handle_socket()
    │       └── repository.rs      # get_current_status()
    │
    └── shared/
        ├── mod.rs                 # Re-exports
        └── error.rs               # AppError, error responses
```

### File Details

#### Entry Point

| File | Purpose | Est. Lines |
|------|---------|------------|
| `main.rs` | Init tracing, config, db. Build router with all feature routes. Start Axum server on :3001 | ~60 |
| `config.rs` | AppConfig struct with `from_env()`. Methods: `state_db_path()`, `config_path()`, `logs_path()` | ~40 |
| `db.rs` | `connect(path) -> SqlitePool`. Opens SQLite in read-only mode | ~15 |

#### Features

**sessions/**

| File | Purpose | Exports |
|------|---------|---------|
| `dto.rs` | `SessionDto { session_id, title, source, created_at, updated_at, message_count }`, `SessionSummaryDto { sessions: Vec<SessionDto>, total: i64 }` | DTOs with `#[derive(Serialize, Deserialize)]` |
| `handler.rs` | `async fn list(Extension(state)) -> Json<SessionSummaryDto>` | Axum handler |
| `repository.rs` | `async fn find_all(db, limit) -> Vec<Session>`, `async fn count_messages(db, session_id) -> i64` | DB queries |

**stats/**

| File | Purpose | Exports |
|------|---------|---------|
| `dto.rs` | `StatsOverviewDto { total_sessions, total_messages, sessions_today, messages_today, active_sources }`, `SourceCountDto { source, count }` | DTOs |
| `handler.rs` | `async fn overview(Extension(state)) -> Json<StatsOverviewDto>` | Handler |
| `repository.rs` | `async fn count_sessions(db) -> i64`, `async fn count_messages(db) -> i64`, `async fn count_sessions_today(db) -> i64`, `async fn group_by_source(db) -> Vec<(String, i64)>` | DB queries |

**config/**

| File | Purpose | Exports |
|------|---------|---------|
| `dto.rs` | `ConfigInfoDto { model, provider, raw_yaml }` | DTO |
| `handler.rs` | `async fn get_config(Extension(state)) -> Json<ConfigInfoDto>` | Handler |
| `repository.rs` | `async fn read_yaml(path) -> String`, `fn extract_value(yaml, key) -> Option<String>` | File helpers |

**cron/**

| File | Purpose | Exports |
|------|---------|---------|
| `dto.rs` | `CronJobDto { id, name, schedule, prompt, enabled, last_run, next_run }` | DTO |
| `handler.rs` | `async fn list_jobs() -> Json<Vec<CronJobDto>>`, `async fn toggle_job(id, enable)` | Handlers |
| `repository.rs` | `async fn read_jobs(path) -> Vec<CronJobDto>` | File reader |

**ws/**

| File | Purpose | Exports |
|------|---------|---------|
| `dto.rs` | `WsMessage { msg_type, data }`, `StatusUpdate { online, model, uptime }` | DTOs |
| `handler.rs` | `async fn ws_handler(WebSocketUpgrade) -> impl IntoResponse`, `async fn handle_socket(socket, state)` | Handlers |
| `repository.rs` | `async fn get_status(state) -> StatusUpdate` | Status getter |

#### Shared

| File | Purpose |
|------|---------|
| `error.rs` | `AppError` enum (NotFound, Database, Config). Implement `IntoResponse`. Helper: `impl From<sqlx::Error> for AppError` |

### features/mod.rs

```rust
pub mod sessions;
pub mod stats;
pub mod config;
pub mod cron;
pub mod ws;
```

### Router Registration (main.rs)

```rust
use features::{sessions, stats, config, cron, ws};

let app = Router::new()
    // Health check (shared)
    .route("/api/health", get(shared::health::handler))
    // Sessions feature
    .route("/api/sessions", get(sessions::handler::list))
    // Stats feature
    .route("/api/stats", get(stats::handler::overview))
    // Config feature
    .route("/api/config", get(config::handler::get_config))
    // Cron feature
    .route("/api/cron", get(cron::handler::list_jobs))
    // WebSocket feature
    .route("/ws", get(ws::handler::ws_handler))
    .layer(Extension(state))
    .layer(CorsLayer::permissive());
```

---

## 🎨 Frontend Structure (SvelteKit 2 + Svelte 5)

### Full Tree

```
frontend/
├── package.json
├── svelte.config.js
├── vite.config.ts
└── src/
    ├── app.html
    ├── app.css                        # Tailwind CSS v4
    │
    ├── routes/                        # SvelteKit file-based routing
    │   ├── +layout.svelte             # Main layout (Sidebar + Header)
    │   ├── +layout.ts                 # SPA config (ssr=false)
    │   │
    │   ├── +page.svelte               # Dashboard home (/)
    │   │
    │   ├── sessions/
    │   │   └── +page.svelte           # Sessions list (/sessions)
    │   │
    │   ├── cron/
    │   │   └── +page.svelte           # Cron jobs (/cron)
    │   │
    │   ├── tools/
    │   │   └── +page.svelte           # Tools browser (/tools)
    │   │
    │   └── settings/
    │       └── +page.svelte           # Settings (/settings)
    │
    └── lib/
        ├── features/
        │   │
        │   ├── dashboard/             # Feature: Dashboard
        │   │   ├── components/
        │   │   │   └── StatsCard.svelte
        │   │   ├── types.ts           # DashboardStats, SourceCount
        │   │   ├── api.ts             # fetchStats()
        │   │   └── index.ts           # Re-exports
        │   │
        │   ├── sessions/              # Feature: Sessions
        │   │   ├── components/
        │   │   │   └── SessionCard.svelte
        │   │   ├── types.ts           # Session, SessionSummary
        │   │   ├── api.ts             # fetchSessions()
        │   │   └── index.ts
        │   │
        │   ├── cron/                  # Feature: Cron Jobs
        │   │   ├── components/
        │   │   │   └── CronJobCard.svelte
        │   │   ├── types.ts           # CronJob
        │   │   ├── api.ts             # fetchCronJobs(), toggleCronJob()
        │   │   └── index.ts
        │   │
        │   ├── tools/                 # Feature: Tools
        │   │   ├── components/
        │   │   │   └── ToolCard.svelte
        │   │   ├── types.ts           # Tool, ToolStatus
        │   │   ├── api.ts             # fetchTools(), toggleTool()
        │   │   └── index.ts
        │   │
        │   └── settings/              # Feature: Settings
        │       ├── components/
        │       │   └── ConfigEditor.svelte
        │       ├── types.ts           # AppConfig
        │       ├── api.ts             # fetchConfig(), updateConfig()
        │       └── index.ts
        │
        └── shared/                    # Shared across features
            ├── components/
            │   ├── Sidebar.svelte     # Navigation sidebar
            │   ├── Header.svelte      # Top bar (model, status)
            │   └── Loading.svelte     # Loading spinner
            │
            ├── stores/
            │   └── status.ts          # WebSocket status store
            │
            ├── types/
            │   └── common.ts          # ApiResponse<T>, PaginatedResponse<T>
            │
            └── utils/
                └── api.ts             # Base fetch wrapper, error handling
```

### File Details

#### Routes (Pages)

| Route | File | Imports From |
|-------|------|--------------|
| `/` | `+page.svelte` | `features/dashboard/`, `shared/components/` |
| `/sessions` | `sessions/+page.svelte` | `features/sessions/`, `shared/components/` |
| `/cron` | `cron/+page.svelte` | `features/cron/`, `shared/components/` |
| `/tools` | `tools/+page.svelte` | `features/tools/`, `shared/components/` |
| `/settings` | `settings/+page.svelte` | `features/settings/`, `shared/components/` |

#### Features

**dashboard/**

| File | Purpose | Exports |
|------|---------|---------|
| `types.ts` | `DashboardStats { total_sessions, total_messages, sessions_today, messages_today }`, `SourceCount { source, count }` | TypeScript interfaces |
| `api.ts` | `async fetchStats(): Promise<DashboardStats>` | API function |
| `components/StatsCard.svelte` | Card with icon, title, value, trend | Component (props: `title`, `value`, `icon`, `trend`) |
| `index.ts` | Re-exports: `export { fetchStats } from './api'` | Barrel exports |

**sessions/**

| File | Purpose | Exports |
|------|---------|---------|
| `types.ts` | `Session { session_id, title, source, created_at, updated_at, message_count }`, `SessionSummary { sessions, total }` | Interfaces |
| `api.ts` | `async fetchSessions(): Promise<SessionSummary>` | API function |
| `components/SessionCard.svelte` | Session list item: title, source, message count, date | Component (props: `session`) |
| `index.ts` | Barrel exports | Re-exports |

**cron/**

| File | Purpose | Exports |
|------|---------|---------|
| `types.ts` | `CronJob { id, name, schedule, prompt, enabled, last_run, next_run }` | Interface |
| `api.ts` | `async fetchCronJobs(): Promise<CronJob[]>`, `async toggleCronJob(id, enable): Promise<void>` | API functions |
| `components/CronJobCard.svelte` | Cron job item: name, schedule, enable/disable toggle | Component (props: `job`) |
| `index.ts` | Barrel exports | Re-exports |

**tools/**

| File | Purpose | Exports |
|------|---------|---------|
| `types.ts` | `Tool { name, enabled, description }`, `ToolStatus { tools: Tool[] }` | Interfaces |
| `api.ts` | `async fetchTools(): Promise<ToolStatus>`, `async toggleTool(name, enable): Promise<void>` | API functions |
| `components/ToolCard.svelte` | Tool item: name, description, enable/disable toggle | Component (props: `tool`) |
| `index.ts` | Barrel exports | Re-exports |

**settings/**

| File | Purpose | Exports |
|------|---------|---------|
| `types.ts` | `AppConfig { model, provider, raw_yaml }` | Interface |
| `api.ts` | `async fetchConfig(): Promise<AppConfig>`, `async updateConfig(config): Promise<void>` | API functions |
| `components/ConfigEditor.svelte` | Config editor: show current model/provider, edit form | Component (props: `config`) |
| `index.ts` | Barrel exports | Re-exports |

#### Shared

**components/**

| File | Purpose | Props |
|------|---------|-------|
| `Sidebar.svelte` | Navigation sidebar with links to all pages | None |
| `Header.svelte` | Top bar showing current model + online/offline status | `status`, `model` |
| `Loading.svelte` | Reusable loading spinner | `size` (sm/md/lg) |

**stores/**

| File | Purpose | Exports |
|------|---------|---------|
| `status.ts` | WebSocket connection store. Auto-reconnect. Updates `online`, `model`, `uptime` | `status` writable store, `connectWebSocket()` |

**types/**

| File | Purpose | Exports |
|------|---------|---------|
| `common.ts` | Generic response types | `ApiResponse<T>`, `PaginatedResponse<T>`, `ApiError` |

**utils/**

| File | Purpose | Exports |
|------|---------|---------|
| `api.ts` | Base fetch wrapper with error handling, JSON parsing, base URL config | `apiFetch<T>(url, options?)`, `API_BASE_URL` |

### Import Examples

```typescript
// In routes/sessions/+page.svelte
import { fetchSessions } from '$lib/features/sessions/api';
import { SessionCard } from '$lib/features/sessions/components';
import { Loading } from '$lib/shared/components';
import type { Session } from '$lib/features/sessions/types';

// In features/dashboard/api.ts
import { apiFetch } from '$lib/shared/utils/api';
import type { DashboardStats } from './types';

export async function fetchStats(): Promise<DashboardStats> {
    return apiFetch<DashboardStats>('/api/stats');
}
```

---

## 📄 File Descriptions Summary

### Backend (Rust)

| Category | Files | Purpose |
|----------|-------|---------|
| Entry | `main.rs`, `config.rs`, `db.rs` | App bootstrap, config, DB connection |
| Feature: Sessions | `dto.rs`, `handler.rs`, `repository.rs` | List sessions with message counts |
| Feature: Stats | `dto.rs`, `handler.rs`, `repository.rs` | Aggregate statistics |
| Feature: Config | `dto.rs`, `handler.rs`, `repository.rs` | Read Hermes config.yaml |
| Feature: Cron | `dto.rs`, `handler.rs`, `repository.rs` | Manage cron jobs |
| Feature: WebSocket | `dto.rs`, `handler.rs`, `repository.rs` | Real-time status updates |
| Shared | `error.rs` | Error types and responses |

### Frontend (SvelteKit)

| Category | Files | Purpose |
|----------|-------|---------|
| Routes | `+layout.svelte`, `+layout.ts`, `+page.svelte`, 4x `+page.svelte` | Page routing |
| Feature: Dashboard | `StatsCard.svelte`, `types.ts`, `api.ts`, `index.ts` | Dashboard stats display |
| Feature: Sessions | `SessionCard.svelte`, `types.ts`, `api.ts`, `index.ts` | Sessions list & search |
| Feature: Cron | `CronJobCard.svelte`, `types.ts`, `api.ts`, `index.ts` | Cron jobs management |
| Feature: Tools | `ToolCard.svelte`, `types.ts`, `api.ts`, `index.ts` | Tools browser |
| Feature: Settings | `ConfigEditor.svelte`, `types.ts`, `api.ts`, `index.ts` | Config editor |
| Shared | `Sidebar.svelte`, `Header.svelte`, `Loading.svelte`, `status.ts`, `common.ts`, `api.ts` | Reusable components & utilities |

---

## 🔗 API Endpoints

| Method | Path | Feature | Handler | Description |
|--------|------|---------|---------|-------------|
| GET | `/api/health` | shared | `health::handler` | Health check |
| GET | `/api/sessions` | sessions | `handler::list` | List sessions |
| GET | `/api/stats` | stats | `handler::overview` | Overview statistics |
| GET | `/api/config` | config | `handler::get_config` | Read config |
| GET | `/api/cron` | cron | `handler::list_jobs` | List cron jobs |
| WS | `/ws` | ws | `handler::ws_handler` | WebSocket real-time |

---

## 📊 Database Schema (Read-only from state.db)

```sql
-- Hermes state.db tables (read-only access)
CREATE TABLE sessions (
    session_id TEXT PRIMARY KEY,
    title TEXT,
    source TEXT,
    created_at TEXT,
    updated_at TEXT
);

CREATE TABLE messages (
    id INTEGER PRIMARY KEY,
    session_id TEXT,
    role TEXT,
    content TEXT,
    created_at TEXT,
    FOREIGN KEY (session_id) REFERENCES sessions(session_id)
);
```

---

## 🎨 UI Wireframes

### Dashboard Home (`/`)
```
┌─────────────────────────────────────────────────────────┐
│  🤖 Hermes Dashboard     Model: mimo-v2.5    🟢 Online │
├──────────┬──────────────────────────────────────────────┤
│          │                                              │
│ 📊 Home  │  ┌──────────┐ ┌──────────┐ ┌──────────┐    │
│ 💬 Sess  │  │ 💬 47    │ │ 📨 1234  │ │ 📅 5     │    │
│ ⏰ Cron  │  │ Sessions │ │ Messages │ │ Today    │    │
│ 🔧 Tools │  └──────────┘ └──────────┘ └──────────┘    │
│ ⚙️ Sett  │  ┌──────────┐                              │
│          │  │ ⚡ 89    │                              │
│          │  │ Msg Today│                              │
│          │  └──────────┘                              │
│          │                                              │
│          │  ┌──────────────────────────────────────┐   │
│          │  │  📈 Activity Chart (7 days)          │   │
│          │  │  ████░░░░░░░░░░░░                    │   │
│          │  └──────────────────────────────────────┘   │
└──────────┴──────────────────────────────────────────────┘
```

### Sessions Page (`/sessions`)
```
┌─────────────────────────────────────────────────────────┐
│  Sessions                                    🔍 Search  │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────┐   │
│  │ 💬 Setup Codex                    2 min ago     │   │
│  │   telegram • 12 messages                        │   │
│  │   20260529_143052_a1b2c3                        │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 💬 Crypto Price Monitor           1 hour ago    │   │
│  │   telegram • 8 messages                         │   │
│  │   20260529_120000_d4e5f6                        │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

---

## 📦 Component Ownership

### Shared Components (used in 2+ features)

| Component | Used By |
|-----------|---------|
| `Sidebar.svelte` | All pages (via layout) |
| `Header.svelte` | All pages (via layout) |
| `Loading.svelte` | Dashboard, Sessions, Cron, Tools, Settings |

### Feature-Specific Components

| Component | Feature | Used Only In |
|-----------|---------|--------------|
| `StatsCard.svelte` | Dashboard | `/` page only |
| `SessionCard.svelte` | Sessions | `/sessions` page only |
| `CronJobCard.svelte` | Cron | `/cron` page only |
| `ToolCard.svelte` | Tools | `/tools` page only |
| `ConfigEditor.svelte` | Settings | `/settings` page only |

---

**Last updated:** 2026-05-29
**Structure:** Feature-based modular architecture

---

## 🔒 Auth Feature (NEW)

### Backend Addition

```
features/auth/
├── mod.rs
├── dto.rs           # LoginRequest, LoginResponse, Claims, UserInfo
├── handler.rs       # login(), me()
├── middleware.rs     # auth_middleware()
└── repository.rs    # verify_credentials()
```

**New Dependencies:**
```toml
jsonwebtoken = "9"
uuid = { version = "1", features = ["v4"] }
```

**New Environment Variables:**
```bash
DASHBOARD_USERNAME=admin
DASHBOARD_PASSWORD=your-secure-password
JWT_SECRET=change-me-in-production
```

**Protected Routes:**
```
POST /api/auth/login    → Public
GET  /api/health        → Public
GET  /api/sessions      → Protected (JWT required)
GET  /api/stats         → Protected
GET  /api/config        → Protected
GET  /api/cron          → Protected
WS   /ws                → Protected
```

### Frontend Addition

```
features/auth/
├── components/
│   └── LoginForm.svelte     # Login form component
├── types.ts                 # LoginRequest, LoginResponse, AuthState
├── api.ts                   # login(), getUserInfo()
├── store.ts                 # auth store (token management)
└── index.ts                 # Re-exports
```

**New Route:**
```
/login                       # Login page (public)
```

**Updated Files:**
```
routes/+layout.svelte        # Add auth check + redirect
shared/utils/api.ts          # Add Authorization header
```

