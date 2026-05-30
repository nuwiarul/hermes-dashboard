use super::dto::{CronJobDto, CronJobsResponse};
use super::repository;
use crate::AppState;
use axum::{Extension, Json};
use std::sync::Arc;

pub async fn list_jobs(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<CronJobsResponse>, (axum::http::StatusCode, String)> {
    let jobs = repository::find_all(&state)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let total = jobs.len();

    Ok(Json(CronJobsResponse { jobs, total }))
}
