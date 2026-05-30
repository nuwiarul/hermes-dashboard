use super::repository;
use crate::AppState;
use crate::shared::cache::{get_cached, insert_cached};
use crate::shared::error::AppError;
use axum::{Extension, Json};
use std::sync::Arc;

pub async fn get_config(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    if let Some(cached) = get_cached(&state.cache.config, "config:main") {
        return Ok(Json(cached));
    }

    let config_path = state.config.config_path();
    let config = repository::read_config(&config_path).map_err(|e| {
        AppError::Internal(format!("Config not found: {}", e))
    })?;
    let value =
        serde_json::to_value(config).map_err(|e| AppError::Internal(e.to_string()))?;
    insert_cached(&state.cache.config, "config:main", value.clone());

    Ok(Json(value))
}
