use axum::{Extension, Json};
use std::sync::Arc;
use crate::AppState;
use super::repository;

pub async fn overview(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<super::dto::StatsOverviewDto>, crate::shared::error::AppError> {
    let stats = repository::get_stats(&state.db).await?;
    Ok(Json(stats))
}
