use super::dto::SessionSummaryDto;
use super::repository;
use crate::AppState;
use crate::shared::cache::{get_cached, insert_cached};
use crate::shared::error::AppError;
use axum::{Extension, Json};
use std::sync::Arc;

pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    if let Some(cached) = get_cached(&state.cache.sessions, "sessions:list") {
        return Ok(Json(cached));
    }

    let sessions = repository::find_all(&state.db, 50).await?;
    let total = repository::count_all(&state.db).await?;
    let dto = SessionSummaryDto { sessions, total };
    let value =
        serde_json::to_value(dto).map_err(|e| AppError::Internal(e.to_string()))?;
    insert_cached(&state.cache.sessions, "sessions:list", value.clone());

    Ok(Json(value))
}
