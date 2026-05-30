# Deployment Scripts

## Frontend (Alibaba Server)

### deploy-frontend.sh
Deploy SvelteKit SPA to Alibaba server (Nginx).

```bash
# From project root
./scripts/deploy-frontend.sh
```

**What it does:**
1. Build frontend (`npm run build`)
2. Upload `build/*` to Alibaba via SCP
3. Fix permissions (`www-data:www-data`)

**Prerequisites:**
- SSH key: `~/.ssh/alibabakey.pem`
- Alibaba: `47.84.137.49` (user: `ubuntu`)
- Remote dir: `/var/www/hermes-dashboard`

## Nginx Configuration

### hermes.vinrul.my.id
Nginx config for Alibaba server (frontend only).

**Install on Alibaba:**
```bash
# Copy config
sudo cp scripts/nginx/hermes.vinrul.my.id /etc/nginx/sites-available/

# Enable
sudo ln -s /etc/nginx/sites-available/hermes.vinrul.my.id /etc/nginx/sites-enabled/

# Test & reload
sudo nginx -t
sudo systemctl reload nginx
```

**Note:** Frontend calls Backend API directly (no proxy). See `docs/deployment.md`.
