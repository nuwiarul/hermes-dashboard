use super::dto::SessionSummaryDto;
use super::repository;
use crate::AppState;
use axum::{Extension, Json};
use std::sync::Arc;

pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<SessionSummaryDto>, crate::shared::AppError> {
    let sessions = repository::find_all(&state.db, 50).await?;
    let total = repository::count_all(&state.db).await?;

    Ok(Json(SessionSummaryDto { sessions, total }))
}
