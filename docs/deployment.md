# Hermes Dashboard — Deployment Configuration

## 🌐 Domain & DNS

| Domain | Server | IP | Purpose |
|--------|--------|-----|---------|
| `hermes.vinrul.my.id` | Alibaba | 47.84.137.49 | Frontend (SvelteKit SPA) |
| `api-hermes.vinrul.my.id` | Tencent | 43.156.247.129 | Backend (Rust API) |

**DNS Records (Cloudflare):**
```
hermes.vinrul.my.id      → A → 47.84.137.49 (Proxied ✅)
api-hermes.vinrul.my.id  → A → 43.156.247.129 (Proxied ✅)
```

---

## 🔒 SSL Certificates

**Provider:** Cloudflare (Origin Certificate)

**Location:** `/etc/ssl/` (both servers)

**Files:**
```
/etc/ssl/
├── cloudflare.crt              # Root CA certificate
├── vinrul_my_id_cert.pem       # Domain certificate
└── vinrul_my_id_key.pem        # Private key
```

**Certificate Details:**
- **Type:** Origin Certificate (Cloudflare → Server)
- **Validity:** 15 years
- **Coverage:** `*.vinrul.my.id` (wildcard)

---

## 🛡️ Firewall Configuration

### Alibaba Server (47.84.137.49)

```bash
# UFW Status
sudo ufw status

# Allowed ports
443/tcp    ALLOW    Anywhere    # Nginx Full
80/tcp     ALLOW    Anywhere    # HTTP (redirect to HTTPS)
```

### Tencent Server (43.156.247.129)

```bash
# UFW Status
sudo ufw status

# Allowed ports
443/tcp    ALLOW    Anywhere    # Nginx Full
80/tcp     ALLOW    Anywhere    # HTTP (redirect to HTTPS)
3001/tcp   ALLOW    127.0.0.1   # Backend (localhost only, proxied by Nginx)
```

---

## 🔧 Nginx Configuration

### Alibaba Server (Frontend)

**File:** `/etc/nginx/sites-available/hermes.vinrul.my.id`

```nginx
server {
    listen 80;
    server_name hermes.vinrul.my.id;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name hermes.vinrul.my.id;

    # SSL Configuration
    ssl_certificate /etc/ssl/vinrul_my_id_cert.pem;
    ssl_certificate_key /etc/ssl/vinrul_my_id_key.pem;
    ssl_trusted_certificate /etc/ssl/cloudflare.crt;

    # SSL Settings
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384;
    ssl_prefer_server_ciphers off;

    # Security Headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Referrer-Policy "no-referrer-when-downgrade" always;
    add_header Content-Security-Policy "default-src 'self' http: https: ws: wss: data: blob: 'unsafe-inline'; frame-ancestors 'self';" always;

    # Frontend static files
    root /var/www/hermes-dashboard;
    index index.html;

    # SPA fallback
    location / {
        try_files $uri $uri/ /index.html;
    }

    # API proxy ke backend di Tencent
    location /api/ {
        proxy_pass http://api-hermes.vinrul.my.id;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # WebSocket proxy
    location /ws {
        proxy_pass http://api-hermes.vinrul.my.id;
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

**Enable & Test:**
```bash
sudo ln -s /etc/nginx/sites-available/hermes.vinrul.my.id /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

### Tencent Server (Backend)

**File:** `/etc/nginx/sites-available/api-hermes.vinrul.my.id`

```nginx
server {
    listen 80;
    server_name api-hermes.vinrul.my.id;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name api-hermes.vinrul.my.id;

    # SSL Configuration
    ssl_certificate /etc/ssl/vinrul_my_id_cert.pem;
    ssl_certificate_key /etc/ssl/vinrul_my_id_key.pem;
    ssl_trusted_certificate /etc/ssl/cloudflare.crt;

    # SSL Settings
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384;
    ssl_prefer_server_ciphers off;

    # Security Headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;

    # API proxy ke Rust backend
    location / {
        proxy_pass http://127.0.0.1:3001;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # WebSocket support
    location /ws {
        proxy_pass http://127.0.0.1:3001;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

**Enable & Test:**
```bash
sudo ln -s /etc/nginx/sites-available/api-hermes.vinrul.my.id /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

---

## 🧪 Verification

### Test SSL

```bash
# Frontend
curl -I https://hermes.vinrul.my.id

# Backend
curl https://api-hermes.vinrul.my.id/api/health
```

### Test API

```bash
# Health check
curl https://api-hermes.vinrul.my.id/api/health

# Expected response:
# {"status":"ok","service":"hermes-dashboard","version":"0.1.0"}
```

### Test Browser

1. Buka https://hermes.vinrul.my.id
2. Harus redirect ke login page
3. Login dengan credentials
4. Dashboard harus terbuka

---

## 🔐 Environment Variables Update

### Backend (.env atau systemd service)

```bash
# Server URLs
FRONTEND_URL=https://hermes.vinrul.my.id
BACKEND_URL=https://api-hermes.vinrul.my.id

# Auth
DASHBOARD_USERNAME=admin
DASHBOARD_PASSWORD=your-strong-password
JWT_SECRET=your-random-secret-key

# Cookie (HTTPS mode)
COOKIE_SECURE=true
```

### Frontend (vite.config.ts)

```typescript
export default defineConfig({
    plugins: [sveltekit()],
    server: {
        proxy: {
            '/api': {
                target: 'https://api-hermes.vinrul.my.id',
                changeOrigin: true,
                secure: true
            }
        }
    }
});
```

### Frontend API Base URL

```typescript
// src/lib/shared/utils/api.ts
export const API_BASE_URL = 'https://api-hermes.vinrul.my.id';
```

---

## 📋 Deployment Checklist

### Pre-deployment

- [ ] DNS records configured in Cloudflare
- [ ] SSL certificates installed on both servers
- [ ] Nginx configured on both servers
- [ ] Firewall ports opened (443, 80)
- [ ] UFW configured

### Deployment

- [ ] Backend built and deployed to Tencent
- [ ] Frontend built and deployed to Alibaba
- [ ] Backend service running
- [ ] Nginx reloaded

### Post-deployment

- [ ] SSL test passed
- [ ] API health check passed
- [ ] Frontend accessible via browser
- [ ] Login functionality working
- [ ] API calls working (check Network tab)

---

## 🔄 Updated URLs

| Service | Old URL | New URL |
|---------|---------|---------|
| Frontend | http://47.84.137.49 | https://hermes.vinrul.my.id |
| Backend API | http://43.156.247.129:3001 | https://api-hermes.vinrul.my.id |
| Health Check | http://43.156.247.129:3001/api/health | https://api-hermes.vinrul.my.id/api/health |

---

## 🚨 Important Notes

### Cloudflare SSL Mode

**Setting di Cloudflare Dashboard:**
- SSL/TLS Mode: **Full (Strict)**
- This ensures end-to-end encryption

### Cookie Settings

With HTTPS enabled:
```rust
// In auth handler
.cookie_secure(true)  // Only send over HTTPS
.cookie_same_site(SameSite::Lax)  // CSRF protection
```

### CORS Update

```rust
// In main.rs
CorsLayer::new()
    .allow_origin("https://hermes.vinrul.my.id".parse().unwrap())
    .allow_credentials(true)
```

---

**Last updated:** 2026-05-29
**Status:** Production-ready with SSL
