## 📁 Project Structure

```
hermes-dashboard/
├── README.md
├── docs/
│   └── plan.md
├── backend/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs              # Entry point, router setup, AppState
│       ├── config.rs            # AppConfig (hermes_home, port, paths)
│       ├── db.rs                # SQLite connection pool
│       ├── routes/
│       │   ├── mod.rs           # Module declarations
│       │   ├── health.rs        # GET /api/health — status check
│       │   ├── sessions.rs      # GET /api/sessions — list sessions
│       │   ├── stats.rs         # GET /api/stats — overview statistics
│       │   ├── config.rs        # GET /api/config — read config.yaml
│       │   ├── cron.rs          # GET /api/cron — list cron jobs
│       │   └── ws.rs            # GET /ws — WebSocket real-time
│       └── models/
│           ├── mod.rs           # Module declarations
│           └── session.rs       # Session, SessionSummary structs
├── frontend/
│   ├── package.json
│   ├── svelte.config.js         # adapter-static config
│   ├── vite.config.ts           # Dev proxy ke backend
│   └── src/
│       ├── app.html             # HTML template
│       ├── app.css              # Tailwind CSS v4 import
│       ├── routes/
│       │   ├── +layout.svelte   # Main layout (Sidebar + Header)
│       │   ├── +layout.ts       # SPA config (ssr=false, prerender=true)
│       │   └── +page.svelte     # Dashboard home (stats cards)
│       │   ├── sessions/
│       │   │   └── +page.svelte # Sessions list + search
│       │   ├── cron/
│       │   │   └── +page.svelte # Cron jobs manager
│       │   ├── tools/
│       │   │   └── +page.svelte # Tools browser
│       │   └── settings/
│       │       └── +page.svelte # Settings page
│       └── lib/
│           ├── components/
│           │   ├── Sidebar.svelte    # Navigation sidebar
│           │   ├── Header.svelte     # Top bar (model, status)
│           │   ├── StatsCard.svelte  # Stats card component
│           │   └── SessionCard.svelte # Session list item
│           ├── stores/
│           │   └── status.ts         # WebSocket status store
│           └── utils/
│               └── api.ts            # Fetch helpers
└── scripts/
    ├── build.sh                 # Build backend + frontend
    ├── deploy-frontend.sh       # SCP frontend ke Alibaba
    ├── deploy-backend.sh        # Build & run backend
    ├── hermes-dashboard.service # Systemd service file
    └── nginx/
        └── hermes-dashboard.conf # Nginx config untuk Alibaba
```

---

## 📄 File Descriptions

### Backend (Rust + Axum)

| File | Purpose | Lines Est. |
|------|---------|------------|
| `Cargo.toml` | Dependencies: axum 0.8, sqlx 0.9, tokio, serde, tower-http | ~20 |
| `main.rs` | Entry point. Init tracing, config, db pool. Setup routes + CORS. Start server on :3001 | ~50 |
| `config.rs` | AppConfig struct. Reads HERMES_HOME, PORT from env. Helper methods: state_db_path(), config_path(), logs_path() | ~40 |
| `db.rs` | connect() function. Opens SQLite in read-only mode. Returns SqlitePool | ~15 |
| `routes/health.rs` | Simple health check. Returns {"status":"ok","service":"hermes-dashboard"} | ~15 |
| `routes/sessions.rs` | Query sessions table. JOIN with message count. Returns SessionSummary (sessions + total) | ~40 |
| `routes/stats.rs` | Aggregate queries: total_sessions, total_messages, sessions_today, messages_today, active_sources | ~60 |
| `routes/config.rs` | Read config.yaml. Parse model/provider with simple string extraction. Return ConfigInfo + raw_yaml | ~40 |
| `routes/cron.rs` | Placeholder for cron jobs. Will parse Hermes cron storage later | ~30 |
| `routes/ws.rs` | WebSocket handler. Send initial status. Keep connection alive. Broadcast updates | ~50 |
| `models/session.rs` | Session struct (sqlx::FromRow). SessionSummary struct. Fields: session_id, title, source, created_at, updated_at, message_count | ~25 |

### Frontend (SvelteKit 2 + Svelte 5 + Tailwind CSS 4)

| File | Purpose | Lines Est. |
|------|---------|------------|
| `svelte.config.js` | adapter-static with SPA fallback (index.html) | ~20 |
| `vite.config.ts` | Dev proxy: /api → localhost:3001 | ~15 |
| `app.css` | Tailwind CSS v4 import (@import "tailwindcss") | ~5 |
| `+layout.ts` | SPA mode: ssr=false, prerender=true | ~5 |
| `+layout.svelte` | Flex layout: Sidebar (264px) + Header + main content slot | ~25 |
| `+page.svelte` | Dashboard home. Fetch /api/stats. Render 4x StatsCard grid | ~60 |
| `sessions/+page.svelte` | Sessions list. Fetch /api/sessions. Search filter. Render SessionCard list | ~70 |
| `cron/+page.svelte` | Cron jobs list. Fetch /api/cron. Enable/disable toggle | ~50 |
| `tools/+page.svelte` | Tools browser. Show installed tools + status | ~50 |
| `settings/+page.svelte` | Settings. Show current config. Edit model/provider | ~60 |
| `Sidebar.svelte` | Navigation links: Dashboard, Sessions, Cron, Tools, Settings. Active state highlight | ~35 |
| `Header.svelte` | Show current model + online/offline status dot | ~25 |
| `StatsCard.svelte` | Card with icon, title, value, trend indicator | ~25 |
| `SessionCard.svelte` | Session item: title, source, message count, date | ~30 |
| `stores/status.ts` | WebSocket store. Auto-reconnect on close. Writable store for status | ~40 |
| `utils/api.ts` | Fetch wrapper with error handling. Base URL config | ~20 |

### Scripts

| File | Purpose |
|------|---------|
| `build.sh` | cargo build --release + bun run build |
| `deploy-frontend.sh` | Build frontend + SCP to Alibaba /var/www/hermes-dashboard |
| `deploy-backend.sh` | Build backend + run as systemd service |
| `hermes-dashboard.service` | Systemd unit file for backend |
| `nginx/hermes-dashboard.conf` | Nginx config: serve SPA + proxy /api to Tencent:3001 + WebSocket proxy |

---

## 🔗 API Endpoints

| Method | Path | Description | Response |
|--------|------|-------------|----------|
| GET | `/api/health` | Health check | `{"status":"ok","service":"hermes-dashboard","version":"0.1.0"}` |
| GET | `/api/sessions` | List sessions | `{"sessions":[...],"total":47}` |
| GET | `/api/stats` | Overview stats | `{"total_sessions":47,"total_messages":1234,"sessions_today":5,"messages_today":89,"active_sources":[...]}` |
| GET | `/api/config` | Read config | `{"model":"mimo-v2.5","provider":"xiaomi","raw_yaml":"..."}` |
| GET | `/api/cron` | List cron jobs | `[{"id":"...","name":"...","schedule":"...","enabled":true}]` |
| WS | `/ws` | WebSocket | `{"type":"status","online":true,"model":"mimo-v2.5","uptime":"3d 12h"}` |

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
│  ┌─────────────────────────────────────────────────┐   │
│  │ 💬 Rust Backend Debug             3 hours ago   │   │
│  │   cli • 25 messages                             │   │
│  │   20260529_090000_g7h8i9                        │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```
