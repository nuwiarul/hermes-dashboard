use axum::{
    routing::{get, post, put},
    Extension, Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod config;
mod db;
mod features;
mod middleware;
mod routes;
pub mod shared;

use features::auth::jwt::JwtConfig;
use middleware::rate_limit::RateLimitState;

pub struct AppState {
    pub db: sqlx::sqlite::SqlitePool,
    pub dashboard_db: sqlx::sqlite::SqlitePool,
    pub config: config::AppConfig,
    pub jwt: JwtConfig,
    pub cache: shared::cache::ApiCache,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let app_config = config::AppConfig::from_env();
    let jwt_config = JwtConfig::from_env();
    let db_pool = db::connect(&app_config.state_db_path()).await?;
    let dashboard_db = db::connect_rw(&app_config.dashboard_db_path()).await?;
    let port = app_config.port;
    let rate_limit_login_max = app_config.rate_limit_login_max;
    let rate_limit_api_max = app_config.rate_limit_api_max;

    let state = Arc::new(AppState {
        db: db_pool,
        dashboard_db: dashboard_db.clone(),
        config: app_config,
        jwt: jwt_config,
        cache: shared::cache::ApiCache::new(),
    });

    // Run dashboard DB migrations
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS workers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            ip TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'worker',
            os TEXT NOT NULL,
            arch TEXT NOT NULL,
            ram_total INTEGER NOT NULL DEFAULT 0,
            disk_total INTEGER NOT NULL DEFAULT 0,
            capabilities TEXT NOT NULL DEFAULT '[]',
            status TEXT NOT NULL DEFAULT 'offline',
            current_task TEXT,
            ram_used INTEGER NOT NULL DEFAULT 0,
            disk_used INTEGER NOT NULL DEFAULT 0,
            active_model TEXT,
            last_heartbeat TEXT,
            registered_at TEXT NOT NULL DEFAULT (datetime('now')),
            config TEXT NOT NULL DEFAULT '{}'
        )"
    )
    .execute(&dashboard_db)
    .await?;

    let rate_limit_state = Arc::new(RateLimitState::new(
        rate_limit_login_max,
        rate_limit_api_max,
    ));

    let cors_origins: Vec<http::HeaderValue> = state
        .config
        .cors_origins
        .iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(cors_origins)
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
            http::Method::OPTIONS,
        ])
        .allow_headers([
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
            http::header::COOKIE,
        ])
        .allow_credentials(true);

    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/api/health", get(routes::health::handler))
        .route("/api/auth/login", post(features::auth::handler::login))
        .route("/api/auth/logout", post(features::auth::handler::logout))
        .route("/api/auth/refresh", post(features::auth::handler::refresh))
        .route("/api/auth/me", get(features::auth::handler::me))
        // Worker registration & heartbeat (no auth - workers register themselves)
        .route("/api/workers/register", post(features::workers::handler::register))
        .route("/api/workers/{id}/heartbeat", post(features::workers::handler::heartbeat))
        .layer(axum::middleware::from_fn_with_state(
            rate_limit_state.clone(),
            middleware::rate_limit::rate_limit_login,
        ));

    // Protected routes (auth required)
    let protected_routes = Router::new()
        .route("/api/sessions", get(features::sessions::handler::list))
        .route("/api/stats", get(features::stats::handler::overview))
        .route("/api/config", get(features::config::handler::get_config))
        .route("/api/cron", get(features::cron::handler::list_jobs))
        .route(
            "/api/tools/models",
            get(features::tools::handler::get_models),
        )
        .route(
            "/api/tools/switch-model",
            post(features::tools::handler::switch_model),
        )
        .route(
            "/api/tools/toolsets",
            get(features::tools::handler::get_toolsets),
        )
        .route(
            "/api/tools/toggle-toolset",
            post(features::tools::handler::toggle_toolset),
        )
        .route(
            "/api/tools/targets",
            get(features::tools::handler::get_targets),
        )
        .route(
            "/api/tools/send-message",
            post(features::tools::handler::send_message),
        )
        .route(
            "/api/tools/gateway/status",
            get(features::tools::handler::get_gateway_status),
        )
        .route(
            "/api/tools/gateway/restart",
            post(features::tools::handler::restart_gateway),
        )
        // Worker management (protected - dashboard view)
        .route("/api/workers", get(features::workers::handler::list_workers))
        .route("/api/workers/{id}", get(features::workers::handler::get_worker))
        .route("/api/workers/{id}/config", put(features::workers::handler::update_config))
        .route("/ws", get(routes::ws::ws_handler))
        .layer(axum::middleware::from_fn(middleware::auth::require_auth))
        .layer(axum::middleware::from_fn_with_state(
            rate_limit_state.clone(),
            middleware::rate_limit::rate_limit_api,
        ));

    let app = public_routes
        .merge(protected_routes)
        .layer(Extension(state))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Server running on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
