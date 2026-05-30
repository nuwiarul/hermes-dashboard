use super::repository;
use crate::AppState;
use crate::shared::cache::{get_cached, insert_cached};
use crate::shared::error::AppError;
use axum::{Extension, Json};
use std::sync::Arc;

pub async fn overview(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    if let Some(cached) = get_cached(&state.cache.stats, "stats:overview") {
        return Ok(Json(cached));
    }

    let stats = repository::get_stats(&state.db).await?;
    let value =
        serde_json::to_value(stats).map_err(|e| AppError::Internal(e.to_string()))?;
    insert_cached(&state.cache.stats, "stats:overview", value.clone());

    Ok(Json(value))
}
