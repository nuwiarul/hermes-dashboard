# AGENTS.md — Project Guidelines

> **Purpose:** Guidelines untuk AI agents (Hermes, Codex, dll) dan contributors yang mengerjakan project Hermes Dashboard.

---

## 📋 Project Overview

**Hermes Dashboard** adalah web dashboard untuk monitor & kontrol Hermes Agent.

**Tech Stack:**
- Backend: Rust (Axum 0.8), SQLx 0.9, tokio
- Frontend: SvelteKit 2, Svelte 5, Tailwind CSS 4
- Database: SQLite (read-only dari Hermes state.db)
- Deploy: Tencent (backend) + Alibaba (frontend/Nginx)

**Repository:** https://github.com/nuwiarul/hermes-dashboard

---

## ✅ BOLEH Dilakukan Agent

### Code Changes
- [ ] Write code sesuai feature-based architecture
- [ ] Implement fitur sesuai task di `docs/plan.md`
- [ ] Add unit tests untuk logic baru
- [ ] Fix bugs yang di-assign
- [ ] Refactor kode yang sudah ada (dengan alasan jelas)
- [ ] Add error handling
- [ ] Add logging untuk debugging

### Documentation
- [ ] Update `docs/*.md` jika ada perubahan
- [ ] Add comments untuk complex logic
- [ ] Update README jika ada fitur baru
- [ ] Mark task sebagai complete di checklist

### Git
- [ ] Buat branch baru untuk setiap task
- [ ] Gunakan conventional commits
- [ ] Push ke branch (bukan main)
- [ ] Create Pull Request

---

## ❌ TIDAK BOLEH Dilakukan Agent

### Security (JANGAN UBAH!)
- ❌ Langsung push ke main branch
- ❌ Deploy ke production tanpa approval
- ❌ Ubah security settings (JWT secret, auth config)
- ❌ Ubah environment variables di server
- ❌ Modify `.env` file
- ❌ Change CORS policy
- ❌ Disable authentication
- ❌ Expose credentials di code

### Production (JANGAN TANPA APPROVAL!)
- ❌ Deploy ke production server
- ❌ Restart services di server
- ❌ Modify Nginx config di server
- ❌ Change systemd service file
- ❌ Modify database schema
- ❌ Delete data dari database

### Destructive Actions
- ❌ Hapus file/folder tanpa konfirmasi
- ❌ Force push ke branch manapun
- ❌ Reset branch ke commit sebelumnya
- ❌ Revert commits tanpa alasan jelas
- ❌ Overwrite file yang sedang di-edit user

### Dependencies
- ❌ Tambah dependency baru tanpa diskusi
- ❌ Update major version tanpa testing
- ❌ Remove dependency yang masih dipakai
- ❌ Install package global di server

---

## ⚠️ BUTUH REVIEW DULU

Perubahan berikut **harus dapat approval** sebelum merge:

### Critical Changes
- [ ] Perubahan API endpoints (routes, handlers)
- [ ] Perubahan database queries (repository)
- [ ] Perubahan authentication flow (auth feature)
- [ ] Perubahan CORS atau security headers
- [ ] Perubahan error handling strategy

### Infrastructure
- [ ] Perubahan dependency versions (Cargo.toml, package.json)
- [ ] Perubahan build scripts
- [ ] Perubahan deployment scripts
- [ ] Perubahan systemd service
- [ ] Perubahan Nginx config

### Architecture
- [ ] Perubahan project structure
- [ ] Perubahan naming conventions
- [ ] Perubahan API response format
- [ ] Perubahan state management approach

---

## 📁 File Structure Rules

### Backend (Rust)

```
features/
├── <feature_name>/
│   ├── mod.rs          # Module exports
│   ├── dto.rs          # Data Transfer Objects
│   ├── handler.rs      # Axum route handlers
│   └── repository.rs   # Database queries
```

**Rules:**
- Setiap fitur HARUS di folder `features/<nama>/`
- DTO, handler, repository HARUS terpisah
- Shared code taruh di `shared/`
- Jangan campur kode fitur berbeda

### Frontend (SvelteKit)

```
src/lib/features/
├── <feature_name>/
│   ├── components/     # Feature-specific components
│   ├── types.ts        # TypeScript interfaces
│   ├── api.ts          # API calls
│   └── index.ts        # Re-exports
```

**Rules:**
- Components yang dipakai 2+ fitur → `shared/components/`
- Types yang dipakai 2+ fitur → `shared/types/`
- API utils → `shared/utils/api.ts`
- Routes di `src/routes/` (SvelteKit file-based routing)

### Naming Conventions

**Files:**
```
Backend (Rust):
- snake_case: session.rs, handler.rs, dto.rs
- Module files: mod.rs

Frontend (Svelte/SvelteKit):
- PascalCase components: SessionCard.svelte, StatsCard.svelte
- camelCase utils: api.ts, status.ts
- kebab-case routes: sessions/, settings/
```

**Variables & Functions:**
```
Backend (Rust):
- snake_case: fetch_sessions(), get_stats()
- SCREAMING_SNAKE_CASE: JWT_SECRET, API_BASE_URL

Frontend (TypeScript):
- camelCase: fetchSessions(), getStats()
- PascalCase types: Session, DashboardStats
- UPPER_SNAKE_CASE constants: API_BASE_URL
```

---

## 🔀 Git Workflow

### Branch Strategy

```
main ← Production (always deployable)
  │
  ├── feat/<nama>    ← Fitur baru
  ├── fix/<nama>     ← Bug fix
  ├── docs/<nama>    ← Dokumentasi
  ├── refactor/<nama> ← Refactor code
  └── chore/<nama>   ← Maintenance
```

### Branch Naming

```bash
# ✅ Benar
git checkout -b feat/add-sessions-page
git checkout -b fix/null-pointer-stats
git checkout -b docs/update-api-docs

# ❌ Salah
git checkout -b new-feature        # Terlalu vague
git checkout -b fix-bug            # Tidak spesifik
git checkout -b update             # Tidak jelas
```

### Commit Messages

**Format:** `<type>: <description>`

**Types:**
```
feat:     Fitur baru
fix:      Bug fix
refactor: Refactor code (tidak mengubah behavior)
docs:     Dokumentasi
test:     Tambah/update tests
chore:    Maintenance (deps, configs, dll)
style:    Formatting, whitespace
perf:     Performance improvement
```

**Examples:**
```bash
# ✅ Benar
git commit -m "feat: add sessions list page with search"
git commit -m "fix: resolve null pointer in stats API"
git commit -m "refactor: extract auth middleware to shared module"
git commit -m "docs: update API documentation"

# ❌ Salah
git commit -m "update code"           # Tidak jelas
git commit -m "fix bug"               # Tidak spesifik
git commit -m "WIP"                   # Tidak descriptive
git commit -m "changes"               # Tidak informative
```

### Pull Request Rules

**Sebelum Create PR:**
- [ ] Code sudah di-test locally
- [ ] Tidak ada console errors
- [ ] Commit messages sudah benar
- [ ] Branch up-to-date dengan main

**PR Description harus include:**
```markdown
## Changes
- [List perubahan]

## Testing
- [ ] Tested locally
- [ ] No console errors
- [ ] Responsive design (jika UI)

## Screenshots
[Attach screenshots jika UI changes]
```

**Merge Rules:**
- PR harus di-review dan approved oleh Dodik
- Tidak boleh merge sendiri
- Squash merge atau merge commit (sesuai preferensi reviewer)

---

## 🧪 Testing Requirements

### Backend (Rust)

**Unit Tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_yaml_value() {
        let yaml = "model:\n  default: mimo-v2.5";
        let result = extract_yaml_value(yaml, "default");
        assert_eq!(result, Some("mimo-v2.5".to_string()));
    }
}
```

**Integration Tests:**
```rust
#[tokio::test]
async fn test_health_endpoint() {
    // Setup test app
    let app = create_test_app().await;
    
    // Make request
    let response = app
        .oneshot(Request::builder().uri("/api/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    
    // Assert
    assert_eq!(response.status(), StatusCode::OK);
}
```

**Run Tests:**
```bash
cd backend
cargo test
cargo test -- --nocapture  # Show println! output
```

### Frontend (SvelteKit)

**Component Tests (optional):**
```typescript
import { render, screen } from '@testing-library/svelte';
import StatsCard from '$lib/features/dashboard/components/StatsCard.svelte';

test('renders stats card with value', () => {
    render(StatsCard, { 
        props: { title: 'Sessions', value: 42, icon: '💬' } 
    });
    
    expect(screen.getByText('42')).toBeInTheDocument();
    expect(screen.getByText('Sessions')).toBeInTheDocument();
});
```

**Run Tests:**
```bash
cd frontend
bun test
```

### Test Coverage Goals

| Component | Coverage Target |
|-----------|----------------|
| API handlers | 80%+ |
| Repository functions | 90%+ |
| Utility functions | 90%+ |
| React components | 70%+ |

---

## 🔒 Security Guidelines

### Authentication

- JWT token disimpan di HttpOnly cookie
- Token tidak boleh di-log atau di-expose
- Password tidak boleh di-hardcode
- Gunakan environment variables untuk secrets

### API Security

- Semua endpoint (kecuali `/api/auth/login`) butuh auth
- Validate semua input dari user
- Sanitize data sebelum query database
- Return generic error messages (jangan expose internal errors)

### CORS

- Allow origin harus spesifik (bukan wildcard)
- Credentials harus true (untuk cookies)
- Methods harus di-whitelist

### Environment Variables

```bash
# HARUS di .env (jangan di code)
DASHBOARD_USERNAME=admin
DASHBOARD_PASSWORD=<strong-password>
JWT_SECRET=<random-secret>
COOKIE_SECURE=false  # Set true untuk HTTPS
```

**Rules:**
- ❌ Jangan commit .env file
- ❌ Jangan hardcode secrets
- ✅ Gunakan env::var() di Rust
- ✅ Gunakan process.env di Node/Bun

---

## 📝 Code Style

### Rust (Backend)

**Formatting:**
```bash
cargo fmt
```

**Linting:**
```bash
cargo clippy
```

**Rules:**
- Gunakan `rustfmt` default config
- Fix semua clippy warnings
- Use `anyhow::Result` untuk error handling
- Use `tracing` untuk logging (bukan `println!`)

**Example:**
```rust
use anyhow::Result;
use tracing::{info, error};

pub async fn fetch_sessions(db: &SqlitePool) -> Result<Vec<Session>> {
    info!("Fetching sessions from database");
    
    let sessions = sqlx::query_as::<_, Session>("SELECT * FROM sessions")
        .fetch_all(db)
        .await
        .map_err(|e| {
            error!("Failed to fetch sessions: {}", e);
            anyhow::anyhow!("Database error: {}", e)
        })?;
    
    info!("Fetched {} sessions", sessions.len());
    Ok(sessions)
}
```

### TypeScript (Frontend)

**Formatting:**
```bash
bun run format  # Prettier
```

**Linting:**
```bash
bun run lint  # ESLint
```

**Rules:**
- Gunakan Prettier default config
- Fix semua ESLint errors
- Use TypeScript strict mode
- Use `$state()` dan `$derived()` (Svelte 5 runes)
- Avoid `any` type

**Example:**
```typescript
// ✅ Benar
interface Session {
    session_id: string;
    title: string | null;
    created_at: string | null;
}

let sessions = $state<Session[]>([]);
let loading = $state(true);
let filteredSessions = $derived(
    sessions.filter(s => s.title?.includes(search))
);

// ❌ Salah
let sessions: any = [];  // Hindari any
let filtered = sessions.filter(s => s.title.includes(search));  // Null safety
```

### CSS (Tailwind)

**Rules:**
- Gunakan Tailwind utility classes
- Extract ke component jika terlalu panjang
- Gunakan design tokens untuk colors
- Mobile-first responsive design

**Example:**
```svelte
<!-- ✅ Benar: Komponen reusable -->
<div class="bg-white rounded-xl shadow-sm p-6 hover:shadow-md transition-shadow">
    <slot />
</div>

<!-- ❌ Salah: Inline styles terlalu panjang -->
<div style="background: white; border-radius: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.1); padding: 24px;">
```

---

## 🚀 Deployment Rules

### Manual Deploy Process

1. **Code reviewed & approved** oleh Dodik
2. **Merge ke main** branch
3. **Pull di server** Tencent
4. **Build backend** (`cargo build --release`)
5. **Build frontend** (`bun run build`)
6. **Deploy frontend** ke Alibaba (SCP)
7. **Restart backend** service
8. **Verify** deployment

### Deploy Checklist

- [ ] PR sudah di-merge
- [ ] Tests passing
- [ ] Build berhasil
- [ ] Frontend deployed ke Alibaba
- [ ] Backend restarted di Tencent
- [ ] Health check berhasil
- [ ] Manual testing di browser

### Rollback Procedure

Jika deploy gagal:

```bash
# 1. Revert ke commit sebelumnya
git revert HEAD

# 2. Push revert
git push origin main

# 3. Re-deploy
./scripts/deploy.sh

# 4. Verify
curl http://43.156.247.129:3001/api/health
```

---

## 📚 Documentation Requirements

### HARUS Update Dokumentasi Jika:

- [ ] Menambah fitur baru → update `docs/plan.md`
- [ ] Mengubah API → update `docs/structure.md`
- [ ] Mengubah security → update `docs/security.md`
- [ ] Mengubah workflow → update `docs/workflow.md`
- [ ] Menambah dependency → update `Cargo.toml` / `package.json`

### Documentation Style

- Gunakan Bahasa Indonesia untuk dokumentasi user-facing
- Gunakan Bahasa Inggris untuk code comments
- Include examples untuk API endpoints
- Include screenshots untuk UI changes

---

## 🤝 Communication

### Asking Questions

Jika ada yang tidak jelas:
1. Cek dokumentasi dulu (`docs/*.md`)
2. Cek existing code untuk reference
3. Tanya Dodik via Pull Request comment
4. Jangan assume — selalu konfirmasi

### Reporting Issues

Jika menemukan bug atau masalah:
1. Buat GitHub Issue
2. Describe masalah dengan jelas
3. Include steps to reproduce
4. Include error logs/screenshots
5. Assign ke Dodik untuk review

---

## 📋 Quick Reference

### Commands Cheat Sheet

```bash
# Development
cd backend && cargo run                    # Run backend locally
cd frontend && bun dev                     # Run frontend locally
cargo test                                 # Run backend tests
bun test                                   # Run frontend tests

# Code Quality
cargo fmt && cargo clippy                  # Format & lint Rust
bun run format && bun run lint             # Format & lint TypeScript

# Git
git checkout -b feat/xxx                   # Create branch
git commit -m "feat: xxx"                  # Commit
git push origin feat/xxx                   # Push
gh pr create                               # Create PR

# Deploy
./scripts/deploy.sh                        # Full deploy
sudo systemctl restart hermes-dashboard    # Restart backend
curl http://43.156.247.129:3001/api/health # Health check
```

### File Locations

```
~/hermes-dashboard/              # Project root
├── backend/                     # Rust backend
│   ├── src/features/            # Feature modules
│   └── target/release/          # Build output
├── frontend/                    # SvelteKit frontend
│   ├── src/lib/features/        # Feature modules
│   └── build/                   # Build output
├── docs/                        # Documentation
└── scripts/                     # Deploy scripts
```

---

**Last updated:** 2026-05-29
**Version:** 1.0.0
**Maintainer:** Dodik (nuwiarul)

---

## 📱 Responsive Design Requirements

### Mandatory Rules

**SEMUA komponen HARUS responsive!**

- ✅ Desktop (1920px+)
- ✅ Laptop (1366px - 1919px)
- ✅ Tablet/iPad (768px - 1365px)
- ✅ Mobile/iPhone/Android (320px - 767px)

### Breakpoints

```css
sm: 640px    /* Small phones landscape */
md: 768px    /* Tablets portrait */
lg: 1024px   /* Tablets landscape, small laptops */
xl: 1280px   /* Laptops, desktops */
2xl: 1536px  /* Large desktops */
```

### Rules

1. **Jangan pakai fixed width** → Gunakan `w-full`, `max-w-*`, atau persen
2. **Jangan pakai fixed font size** → Gunakan responsive: `text-sm sm:text-base`
3. **Jangan pakai fixed padding** → Gunakan responsive: `p-4 sm:p-6`
4. **Sidebar harus toggle di mobile** → Hamburger menu
5. **Grid harus responsif** → `grid-cols-1 sm:grid-cols-2 lg:grid-cols-4`
6. **Touch target minimal 44x44px** → Untuk mobile usability

### Testing

**WAJIB test di semua ukuran sebelum commit:**
- [ ] Desktop (1920x1080)
- [ ] Tablet (768x1024)
- [ ] Mobile (375x812)

**Gunakan Chrome DevTools:**
```
F12 → Toggle Device Toolbar (Ctrl+Shift+M)
```

