use axum::{
    routing::{get, post},
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
    let rate_limit_login_max = app_config.rate_limit_login_max;
    let rate_limit_api_max = app_config.rate_limit_api_max;

    let state = Arc::new(AppState {
        db: db_pool,
        config: app_config,
        jwt: jwt_config,
    });

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
