use axum::{routing::{get, post}, Extension, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod config;
mod db;
mod features;
mod routes;
pub mod shared;

use features::auth::jwt::JwtConfig;

pub struct AppState {
    pub db: sqlx::sqlite::SqlitePool,
    pub config: config::AppConfig,
    pub jwt: JwtConfig,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let app_config = config::AppConfig::from_env();
    let jwt_config = JwtConfig::from_env();
    let db_pool = db::connect(&app_config.state_db_path()).await?;
    let port = app_config.port;

    let state = Arc::new(AppState {
        db: db_pool,
        config: app_config,
        jwt: jwt_config,
    });

    let cors_origins: Vec<http::HeaderValue> = state
        .config
        .cors_origins
        .iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(cors_origins)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .route("/api/health", get(routes::health::handler))
        .route("/api/auth/login", post(features::auth::handler::login))
        .route("/api/auth/logout", post(features::auth::handler::logout))
        .route("/api/sessions", get(features::sessions::handler::list))
        .route("/api/stats", get(features::stats::handler::overview))
        .route("/api/config", get(features::config::handler::get_config))
        .route("/api/cron", get(features::cron::handler::list_jobs))
        .route("/ws", get(routes::ws::ws_handler))
        .layer(Extension(state))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Server running on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
