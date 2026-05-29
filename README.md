# 🤖 Hermes Dashboard

Web dashboard untuk monitor & kontrol [Hermes Agent](https://github.com/NousResearch/hermes-agent).

## Features

- 📊 **Dashboard** — Real-time stats & overview
- 💬 **Session Viewer** — Browse & search chat history
- 💰 **Cost Tracker** — Token usage & cost estimation
- ⏰ **Cron Manager** — View & manage scheduled jobs
- 🔧 **Tools Browser** — Enable/disable toolsets
- ⚙️ **Remote Control** — Switch model, restart gateway

## Tech Stack

- **Backend:** Rust + Axum + SQLx
- **Frontend:** SvelteKit + Tailwind CSS + Chart.js
- **Database:** SQLite (Hermes state.db)
- **Deploy:** Systemd + Cloudflare Tunnel

## Quick Start

```bash
# Clone
git clone https://github.com/nuwiarul/hermes-dashboard.git
cd hermes-dashboard

# Backend
cd backend
cargo run

# Frontend (another terminal)
cd frontend
bun install
bun dev
```

Open http://localhost:5173

## Documentation

- [Implementation Plan](docs/plan.md)

## License

MIT
