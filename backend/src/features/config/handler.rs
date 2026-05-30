use axum::{Extension, Json};
use std::sync::Arc;
use crate::AppState;
use super::repository;

pub async fn get_config(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<super::dto::ConfigDto>, crate::shared::error::AppError> {
    let config_path = state.config.config_path();
    let config = repository::read_config(&config_path)
        .map_err(|e| crate::shared::error::AppError::Internal(format!("Config not found: {}", e)))?;
    Ok(Json(config))
}
