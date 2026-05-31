use super::dto::{HeartbeatRequest, RegisterWorkerRequest, RegisterWorkerResponse, WorkerDto, WorkerListDto};
use super::repository;
use crate::shared::error::AppError;
use axum::{Extension, Json};
use std::sync::Arc;
use crate::AppState;

pub async fn register(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RegisterWorkerRequest>,
) -> Result<Json<RegisterWorkerResponse>, AppError> {
    let id = repository::upsert(&state.dashboard_db, &payload).await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(RegisterWorkerResponse {
        id,
        name: payload.name,
        message: "Worker registered successfully".to_string(),
    }))
}

pub async fn heartbeat(
    Extension(state): Extension<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    Json(payload): Json<HeartbeatRequest>,
) -> Result<Json<super::dto::HeartbeatResponse>, AppError> {
    repository::heartbeat(
        &state.dashboard_db,
        id,
        payload.status.as_deref(),
        payload.current_task.as_deref(),
        payload.ram_used,
        payload.disk_used,
        payload.active_model.as_deref(),
    )
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    // Check if config needs to be sent to worker
    let config_updated_at = repository::get_config_updated_at(&state.dashboard_db, id)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let mut config_to_send = None;
    let mut config_timestamp = None;

    if let Some(updated_at) = &config_updated_at {
        // Compare with worker's last applied config
        if payload.last_config_applied_at.as_ref() != Some(updated_at) {
            // Config is newer, send it to worker
            let worker = repository::find_by_id(&state.dashboard_db, id)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

            if let Some(w) = worker {
                if let Ok(config_value) = serde_json::from_str::<serde_json::Value>(&w.config) {
                    config_to_send = Some(config_value);
                    config_timestamp = Some(updated_at.clone());
                }
            }
        }
    }

    Ok(Json(super::dto::HeartbeatResponse {
        success: true,
        message: "Heartbeat updated".to_string(),
        config: config_to_send,
        config_updated_at: config_timestamp,
    }))
}

pub async fn list_workers(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<WorkerListDto>, AppError> {
    let workers = repository::find_all(&state.dashboard_db).await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let total = repository::count_all(&state.dashboard_db).await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(WorkerListDto { workers, total }))
}

pub async fn get_worker(
    Extension(state): Extension<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> Result<Json<WorkerDto>, AppError> {
    let worker = repository::find_by_id(&state.dashboard_db, id).await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or(AppError::NotFound)?;

    Ok(Json(worker))
}

pub async fn update_config(
    Extension(state): Extension<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
    Json(payload): Json<super::dto::WorkerConfigRequest>,
) -> Result<Json<super::dto::WorkerConfigResponse>, AppError> {
    // Check if worker exists
    let worker = repository::find_by_id(&state.dashboard_db, id).await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or(AppError::NotFound)?;

    // Update config
    repository::update_config(
        &state.dashboard_db,
        id,
        payload.model.as_deref(),
        payload.provider.as_deref(),
        payload.max_tokens,
        payload.temperature,
    )
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    Ok(Json(super::dto::WorkerConfigResponse {
        success: true,
        message: format!("Configuration updated for worker '{}'", worker.name),
        applied_at: now,
    }))
}
