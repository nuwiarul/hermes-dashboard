## 🔄 Development Workflow (Manual Deploy)

### Overview

Workflow development dengan manual deploy. Developer push code ke GitHub, Dodik review & merge, lalu manual build & deploy ke server.

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│  DEVELOPMENT PHASE                                          │
│  ─────────────────                                          │
│                                                             │
│  1. Buat branch baru                                        │
│     git checkout -b feat/xxx                                │
│                                                             │
│  2. Coding & commit                                         │
│     git add .                                               │
│     git commit -m "feat: add xxx"                           │
│                                                             │
│  3. Push ke GitHub                                          │
│     git push origin feat/xxx                                │
│                                                             │
│  4. Create Pull Request                                     │
│     gh pr create --title "feat: xxx" --body "..."           │
│                                                             │
│  5. Dodik review PR di GitHub                               │
│     ├── Request changes → back to step 2                    │
│     └── Approved → Merge                                    │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  DEPLOY PHASE (Manual)                                      │
│  ─────────────────────                                      │
│                                                             │
│  6. Pull latest code di Tencent                             │
│     git pull origin main                                    │
│                                                             │
│  7. Build Backend                                           │
│     cd backend && cargo build --release                     │
│                                                             │
│  8. Build Frontend                                          │
│     cd frontend && bun run build                            │
│                                                             │
│  9. Deploy Frontend ke Alibaba                              │
│     scp -r frontend/build/* alibaba:/var/www/hermes-...     │
│                                                             │
│  10. Restart Backend Service                                │
│     sudo systemctl restart hermes-dashboard                 │
│                                                             │
│  11. Verify                                                 │
│     curl https://hermes.vinrul.my.id                                │
│     curl https://api-hermes.vinrul.my.id/api/health              │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  DONE ✅                                                    │
│  Dashboard updated di https://hermes.vinrul.my.id                   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Step-by-Step Guide

### Phase 1: Development

#### Step 1: Buat Branch Baru

```bash
# Pastikan di main branch dan up-to-date
git checkout main
git pull origin main

# Buat branch baru
git checkout -b feat/add-sessions-page
```

**Branch naming convention:**
```
feat/xxx    → fitur baru
fix/xxx     → bug fix
refactor/xxx → refactor code
docs/xxx    → dokumentasi
chore/xxx   → maintenance
```

#### Step 2: Coding & Commit

```bash
# Setelah selesai coding
git add .
git status  # Cek perubahan

# Commit dengan pesan yang jelas
git commit -m "feat: add sessions page with search"
```

**Commit message convention:**
```
feat: add sessions page
fix: resolve null pointer in stats
refactor: extract auth middleware
docs: update API documentation
chore: update dependencies
```

#### Step 3: Push ke GitHub

```bash
git push origin feat/add-sessions-page
```

#### Step 4: Create Pull Request

```bash
gh pr create \
  --title "feat: add sessions page with search" \
  --body "## Changes
- Add sessions list page
- Add search functionality
- Add SessionCard component

## Screenshots
[Add screenshots if UI changes]

## Testing
- [x] Tested locally
- [x] No console errors
- [x] Responsive design"
```

#### Step 5: Dodik Review PR

**Di GitHub:**
1. Buka PR
2. Review code changes
3. Beri komentar jika ada yang perlu diubah
4. Approve & Merge jika sudah oke

**Atau via CLI:**
```bash
# List PRs
gh pr list

# Review PR
gh pr review 42 --approve

# Merge PR
gh pr merge 42 --merge
```

---

### Phase 2: Deploy (Manual)

#### Step 6: Pull Latest Code

```bash
# SSH ke Tencent server
ssh tencent

# Pull latest code
cd ~/hermes-dashboard
git checkout main
git pull origin main
```

#### Step 7: Build Backend

```bash
cd ~/hermes-dashboard/backend

# Build release
cargo build --release

# Verify binary exists
ls -la target/release/hermes-dashboard-backend
```

**Expected output:**
```
-rwxr-xr-x 1 ubuntu ubuntu 12M May 29 15:30 target/release/hermes-dashboard-backend
```

#### Step 8: Build Frontend

```bash
cd ~/hermes-dashboard/frontend

# Install dependencies (jika ada perubahan)
bun install

# Build untuk production
bun run build

# Verify build output
ls -la build/
```

**Expected output:**
```
build/
├── _app/
├── index.html
└── ...
```

#### Step 9: Deploy Frontend ke Alibaba

```bash
# Dari Tencent, copy frontend build ke Alibaba
scp -i ~/.ssh/alibabakey.pem -r ~/hermes-dashboard/frontend/build/* \
  ubuntu@47.84.137.49:/var/www/hermes-dashboard/
```

**Verify di Alibaba:**
```bash
ssh -i ~/.ssh/alibabakey.pem ubuntu@47.84.137.49

# Check files
ls -la /var/www/hermes-dashboard/

# Restart Nginx (jika perlu)
sudo systemctl restart nginx
```

#### Step 10: Restart Backend Service

```bash
# Di Tencent
sudo systemctl restart hermes-dashboard

# Check status
sudo systemctl status hermes-dashboard
```

**Expected output:**
```
● hermes-dashboard.service - Hermes Dashboard Backend
     Loaded: loaded (/etc/systemd/system/hermes-dashboard.service; enabled)
     Active: active (running) since ...
```

#### Step 11: Verify Deployment

```bash
# Test backend health
curl https://api-hermes.vinrul.my.id/api/health

# Expected:
# {"status":"ok","service":"hermes-dashboard","version":"0.1.0"}

# Test frontend (dari browser)
# Buka https://hermes.vinrul.my.id

# Test API melalui Nginx
curl https://hermes.vinrul.my.id/api/health

# Expected:
# {"status":"ok","service":"hermes-dashboard","version":"0.1.0"}
```

---

## Deploy Script (Optional)

Untuk mempermudah, bisa buat script deploy:

### `scripts/deploy.sh`

```bash
#!/bin/bash
set -e

echo "🚀 Starting deployment..."

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Step 1: Pull latest code
echo -e "${YELLOW}📥 Pulling latest code...${NC}"
git checkout main
git pull origin main

# Step 2: Build backend
echo -e "${YELLOW}🔨 Building backend...${NC}"
cd backend
cargo build --release
cd ..
echo -e "${GREEN}✅ Backend built successfully${NC}"

# Step 3: Build frontend
echo -e "${YELLOW}🔨 Building frontend...${NC}"
cd frontend
bun install
bun run build
cd ..
echo -e "${GREEN}✅ Frontend built successfully${NC}"

# Step 4: Deploy frontend to Alibaba
echo -e "${YELLOW}📤 Deploying frontend to Alibaba...${NC}"
scp -i ~/.ssh/alibabakey.pem -r frontend/build/* \
  ubuntu@47.84.137.49:/var/www/hermes-dashboard/
echo -e "${GREEN}✅ Frontend deployed${NC}"

# Step 5: Restart backend
echo -e "${YELLOW}🔄 Restarting backend service...${NC}"
sudo systemctl restart hermes-dashboard
echo -e "${GREEN}✅ Backend restarted${NC}"

# Step 6: Verify
echo -e "${YELLOW}🔍 Verifying deployment...${NC}"
sleep 2

BACKEND_STATUS=$(curl -s http://localhost:3001/api/health | jq -r '.status')
if [ "$BACKEND_STATUS" = "ok" ]; then
    echo -e "${GREEN}✅ Backend is healthy${NC}"
else
    echo -e "${RED}❌ Backend health check failed${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}🎉 Deployment complete!${NC}"
echo "   Frontend: https://hermes.vinrul.my.id"
echo "   Backend:  https://api-hermes.vinrul.my.id"
```

**Make executable:**
```bash
chmod +x scripts/deploy.sh
```

**Usage:**
```bash
# Dari root project
./scripts/deploy.sh
```

---

## Quick Reference Commands

### Git Workflow

```bash
# Start new feature
git checkout main && git pull
git checkout -b feat/xxx

# Save progress
git add . && git commit -m "feat: xxx"

# Push & create PR
git push origin feat/xxx
gh pr create

# After merge, cleanup
git checkout main && git pull
git branch -d feat/xxx
```

### Build Commands

```bash
# Backend
cd backend && cargo build --release

# Frontend
cd frontend && bun install && bun run build

# Both
cd backend && cargo build --release && cd ../frontend && bun run build
```

### Deploy Commands

```bash
# Full deploy
./scripts/deploy.sh

# Manual deploy frontend
scp -i ~/.ssh/alibabakey.pem -r frontend/build/* ubuntu@47.84.137.49:/var/www/hermes-dashboard/

# Manual restart backend
sudo systemctl restart hermes-dashboard
```

### Verify Commands

```bash
# Backend health
curl https://api-hermes.vinrul.my.id/api/health

# Frontend (browser)
https://hermes.vinrul.my.id

# Backend logs
sudo journalctl -u hermes-dashboard -f

# Nginx logs (di Alibaba)
sudo tail -f /var/log/nginx/access.log
sudo tail -f /var/log/nginx/error.log
```

---

## Troubleshooting

### Backend won't start

```bash
# Check logs
sudo journalctl -u hermes-dashboard -n 50

# Check port
sudo lsof -i :3001

# Restart
sudo systemctl restart hermes-dashboard
```

### Frontend shows 404

```bash
# Check Nginx config
sudo nginx -t

# Check files exist
ls -la /var/www/hermes-dashboard/

# Restart Nginx
sudo systemctl restart nginx
```

### API returns 401

```bash
# Check if backend is running
curl http://localhost:3001/api/health

# Check Nginx proxy
curl https://hermes.vinrul.my.id/api/health
```

### Build fails

```bash
# Backend: Rust compile error
cd backend && cargo build 2>&1 | head -50

# Frontend: Bun error
cd frontend && bun install && bun run build 2>&1 | head -50
```

---

## File Locations Reference

| What | Where |
|------|-------|
| Project root | `~/hermes-dashboard/` |
| Backend source | `~/hermes-dashboard/backend/` |
| Frontend source | `~/hermes-dashboard/frontend/` |
| Backend binary | `~/hermes-dashboard/backend/target/release/hermes-dashboard-backend` |
| Frontend build | `~/hermes-dashboard/frontend/build/` |
| Deploy script | `~/hermes-dashboard/scripts/deploy.sh` |
| Systemd service | `/etc/systemd/system/hermes-dashboard.service` |
| Nginx config (Alibaba) | `/etc/nginx/sites-available/hermes-dashboard.conf` |
| Frontend files (Alibaba) | `/var/www/hermes-dashboard/` |

---

## Git Branch Strategy

```
main ← Production (always deployable)
  │
  ├── feat/xxx ← New features
  ├── fix/xxx  ← Bug fixes
  ├── docs/xxx ← Documentation
  └── refactor/xxx ← Code improvements
```

**Rules:**
1. Never push directly to main
2. Always create PR for review
3. Merge only after approval
4. Delete branch after merge

---

**Last updated:** 2026-05-29
**Workflow:** Manual Deploy

---

## 🌐 Cross-Origin Setup

Karena Frontend dan Backend adalah **different origins**, pastikan:

### Backend (Rust)

```rust
// CORS configuration
let cors = CorsLayer::new()
    .allow_origin("https://hermes.vinrul.my.id".parse().unwrap())
    .allow_credentials(true);

// Cookie configuration
let cookie = Cookie::build(("hermes_token", token))
    .domain(".vinrul.my.id")
    .secure(true)
    .same_site(SameSite::None);
```

### Frontend (SvelteKit)

```typescript
// API calls must include credentials
const response = await fetch('https://api-hermes.vinrul.my.id/api/...', {
    credentials: 'include',
});
```

### Nginx (Alibaba)

```nginx
# Tidak perlu proxy — Frontend langsung panggil Backend
# Hanya serve static files
location / {
    try_files $uri $uri/ /index.html;
}
```

