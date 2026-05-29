use axum::{routing::get, Extension, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod config;
mod db;
mod routes;

pub struct AppState {
    pub db: sqlx::sqlite::SqlitePool,
    pub config: config::AppConfig,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app_config = config::AppConfig::from_env();
    let db_pool = db::connect(&app_config.state_db_path()).await?;

    let state = Arc::new(AppState {
        db: db_pool,
        config: app_config,
    });

    let app = Router::new()
        .route("/api/health", get(routes::health::handler))
        .layer(Extension(state))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::info!("Server running on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
