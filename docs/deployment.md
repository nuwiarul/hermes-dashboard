# Hermes Dashboard — Deployment Configuration

## 🌐 Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  Browser                                                        │
│     │                                                           │
│     ├─── HTTPS ─────────────────────────────────────────────┐   │
│     │                                                        │   │
│     │    ┌──────────────────────────────────────────────┐   │   │
│     │    │  Cloudflare (SSL Termination)                │   │   │
│     │    └──────────────────────────────────────────────┘   │   │
│     │                                                        │   │
│     ▼                                                        ▼   │
│  ┌─────────────────────┐                    ┌─────────────────┐ │
│  │  hermes.vinrul.my.id│                    │api-hermes.      │ │
│  │  (Alibaba)          │                    │  vinrul.my.id   │ │
│  │                     │                    │  (Tencent)      │ │
│  │  ┌───────────────┐  │                    │                 │ │
│  │  │  Nginx        │  │                    │  ┌───────────┐  │ │
│  │  │  Port 443     │  │                    │  │  Nginx    │  │ │
│  │  │  SSL ✅       │  │                    │  │  Port 443 │  │ │
│  │  └───────────────┘  │                    │  │  SSL ✅   │  │ │
│  │                     │                    │  └─────┬─────┘  │ │
│  │  ┌───────────────┐  │                    │        │        │ │
│  │  │  Static Files │  │                    │  ┌─────▼─────┐  │ │
│  │  │  SvelteKit SPA│  │                    │  │  Rust API │  │ │
│  │  └───────────────┘  │                    │  │  Port 3001│  │ │
│  │                     │                    │  └───────────┘  │ │
│  └─────────────────────┘                    └─────────────────┘ │
│          │                                            │        │
│          │                                            │        │
│          └──────────────┐    ┌────────────────────────┘        │
│                         │    │                                  │
│                         ▼    ▼                                  │
│                    ┌─────────────────┐                          │
│                    │  Direct API     │                          │
│                    │  Calls (HTTPS)  │                          │
│                    └─────────────────┘                          │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Key Points:**
- Frontend dan Backend adalah **terpisah** (different origins)
- Frontend langsung panggil Backend API via HTTPS
- Tidak ada proxy di Alibaba — Alibaba serve frontend saja
- CORS dikonfigurasi di Backend untuk allow Frontend origin

---

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

### Alibaba Server (Frontend Only)

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

**Note:** Tidak ada `/api` atau `/ws` proxy — Frontend langsung panggil `https://api-hermes.vinrul.my.id` dari browser.

**Enable & Test:**
```bash
sudo ln -s /etc/nginx/sites-available/hermes.vinrul.my.id /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

### Tencent Server (Backend Only)

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

## 🔐 CORS Configuration (Backend)

Karena Frontend dan Backend adalah **different origins**, CORS harus dikonfigurasi dengan benar.

### Backend (main.rs)

```rust
use tower_http::cors::{CorsLayer, AllowOrigin, AllowMethods, AllowHeaders};
use http::Method;

let cors = CorsLayer::new()
    // Allow Frontend origin
    .allow_origin(AllowOrigin::exact("https://hermes.vinrul.my.id".parse().unwrap()))
    // Allow credentials (cookies)
    .allow_credentials(true)
    // Allow methods
    .allow_methods(AllowMethods::from(vec![
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
        Method::OPTIONS,
    ]))
    // Allow headers
    .allow_headers(AllowHeaders::from(vec![
        "content-type".parse().unwrap(),
        "authorization".parse().unwrap(),
    ]));
```

### CORS Headers (Auto-generated by tower-http)

```
Access-Control-Allow-Origin: https://hermes.vinrul.my.id
Access-Control-Allow-Credentials: true
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS
Access-Control-Allow-Headers: content-type, authorization
```

---

## 🍪 Cookie Configuration (Cross-Origin)

Karena Frontend dan Backend adalah **different origins**, cookie harus dikonfigurasi untuk cross-origin.

### Backend (auth handler)

```rust
use tower_cookies::Cookie;

let cookie = Cookie::build(("hermes_token", token))
    .domain(".vinrul.my.id")  // Allow cookie untuk semua subdomain
    .path("/")
    .http_only(true)
    .secure(true)             // HTTPS only
    .same_site(tower_cookies::SameSite::None)  // Cross-origin
    .max_age(time::Duration::hours(24))
    .build();
```

### Cookie Attributes (Cross-Origin)

| Attribute | Value | Reason |
|-----------|-------|--------|
| `domain` | `.vinrul.my.id` | Works for all subdomains |
| `path` | `/` | Available for all paths |
| `httpOnly` | `true` | Not accessible via JavaScript |
| `secure` | `true` | HTTPS only |
| `sameSite` | `None` | Cross-origin (required for different domains) |
| `maxAge` | `86400` | 24 hours |

**Important:** `SameSite=None` requires `Secure=true` (HTTPS).

---

## 🖥️ Frontend Configuration

### API Base URL

```typescript
// src/lib/shared/utils/api.ts
export const API_BASE_URL = 'https://api-hermes.vinrul.my.id';
```

### Fetch with Credentials

```typescript
// All API calls must include credentials for cookies
const response = await fetch(`${API_BASE_URL}/api/sessions`, {
    credentials: 'include',  // CRITICAL for cross-origin cookies
    headers: {
        'Content-Type': 'application/json',
    },
});
```

### Vite Config (Development)

```typescript
// vite.config.ts
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [sveltekit()],
    server: {
        proxy: {
            '/api': {
                target: 'https://api-hermes.vinrul.my.id',
                changeOrigin: true,
                secure: true,
            },
            '/ws': {
                target: 'wss://api-hermes.vinrul.my.id',
                ws: true,
            },
        },
    },
});
```

---

## 🧪 Verification

### Test SSL

```bash
# Frontend
curl -I https://hermes.vinrul.my.id

# Backend
curl -I https://api-hermes.vinrul.my.id
```

### Test CORS

```bash
# Preflight request
curl -X OPTIONS https://api-hermes.vinrul.my.id/api/health \
  -H "Origin: https://hermes.vinrul.my.id" \
  -H "Access-Control-Request-Method: GET" \
  -v

# Should return:
# Access-Control-Allow-Origin: https://hermes.vinrul.my.id
# Access-Control-Allow-Credentials: true
```

### Test API

```bash
# Health check
curl https://api-hermes.vinrul.my.id/api/health

# Expected response:
# {"status":"ok","service":"hermes-dashboard","version":"0.1.0"}
```

### Test Cookie

```bash
# Login
curl -X POST https://api-hermes.vinrul.my.id/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"your-password"}' \
  -c cookies.txt -v

# Should see Set-Cookie header with:
# - Domain=.vinrul.my.id
# - Secure
# - HttpOnly
# - SameSite=None
```

### Test Browser

1. Buka https://hermes.vinrul.my.id
2. Open DevTools → Network tab
3. Login
4. Check:
   - API calls ke `api-hermes.vinrul.my.id` ✅
   - Cookie `hermes_token` di-set untuk `.vinrul.my.id` ✅
   - CORS headers ada di response ✅

---

## 📋 Environment Variables

### Backend (.env atau systemd service)

```bash
# Server URLs
FRONTEND_URL=https://hermes.vinrul.my.id
BACKEND_URL=https://api-hermes.vinrul.my.id

# Auth
DASHBOARD_USERNAME=admin
DASHBOARD_PASSWORD=your-s...rue
```

---

## 📋 Deployment Checklist

### Pre-deployment

- [ ] DNS records configured in Cloudflare
- [ ] SSL certificates installed on both servers
- [ ] Nginx configured on both servers (frontend only on Alibaba, backend only on Tencent)
- [ ] Firewall ports opened (443, 80)
- [ ] UFW configured

### Backend Configuration

- [ ] CORS configured to allow `https://hermes.vinrul.my.id`
- [ ] Cookie configured with `domain=.vinrul.my.id`, `same_site=None`, `secure=true`
- [ ] Environment variables set

### Frontend Configuration

- [ ] API base URL set to `https://api-hermes.vinrul.my.id`
- [ ] All fetch calls include `credentials: 'include'`

### Deployment

- [ ] Backend built and deployed to Tencent
- [ ] Frontend built and deployed to Alibaba
- [ ] Backend service running
- [ ] Nginx reloaded on both servers

### Post-deployment

- [ ] SSL test passed (both domains)
- [ ] CORS test passed (preflight request)
- [ ] API health check passed
- [ ] Frontend accessible via browser
- [ ] Login functionality working
- [ ] Cookie set correctly (check DevTools)
- [ ] API calls working (check Network tab)

---

## 🔄 Architecture Comparison

### Before (Proxy via Alibaba)

```
Browser ──HTTPS──► Alibaba ──HTTP──► Tencent
                   (Nginx proxy)
```

- ❌ Extra hop (slower)
- ❌ HTTP between servers (insecure)
- ❌ Complex proxy config
- ✅ Same origin (no CORS)

### After (Direct Connection)

```
Browser ──HTTPS──► Alibaba (Frontend)
    │
    └──HTTPS──► Tencent (Backend)
```

- ✅ Faster (direct)
- ✅ All HTTPS (secure)
- ✅ Simple config (no proxy)
- ⚠️ Cross-origin (need CORS + cookie config)

---

## 🚨 Important Notes

### Cloudflare SSL Mode

**Setting di Cloudflare Dashboard:**
- SSL/TLS Mode: **Full (Strict)**
- This ensures end-to-end encryption

### Cross-Origin Cookie Limitations

- `SameSite=None` required for cross-origin
- `Secure=true` required when `SameSite=None`
- Cookie di-set untuk domain `.vinrul.my.id` (bisa dipakai di semua subdomain)
- Browser harus support `SameSite=None` (Chrome 80+, Firefox 69+, Safari 13+)

### CORS Preflight

Browser akan mengirim `OPTIONS` request sebelum `POST`/`PUT`/`DELETE`:
```
OPTIONS /api/auth/login HTTP/1.1
Origin: https://hermes.vinrul.my.id
Access-Control-Request-Method: POST
Access-Control-Request-Headers: content-type
```

Backend harus respond dengan:
```
HTTP/1.1 204 No Content
Access-Control-Allow-Origin: https://hermes.vinrul.my.id
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS
Access-Control-Allow-Headers: content-type
Access-Control-Allow-Credentials: true
```

---

**Last updated:** 2026-05-29
**Architecture:** Direct Connection (No Proxy)
**Status:** Production-ready with SSL
