use axum::{Extension, Json};
use std::sync::Arc;
use crate::AppState;
use super::dto::*;
use super::repository;

/// GET /api/tools/models — List current model and available models
pub async fn get_models(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<ModelsResponse>, crate::shared::error::AppError> {
    let config_path = state.config.config_path();

    let current = repository::read_model_config(&config_path)
        .map_err(|e| crate::shared::error::AppError::Internal(format!("Failed to read config: {}", e)))?;

    let available = repository::get_available_models();

    Ok(Json(ModelsResponse {
        current,
        available,
    }))
}

/// POST /api/tools/switch-model — Switch to a different model
pub async fn switch_model(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<SwitchModelRequest>,
) -> Result<Json<SwitchModelResponse>, crate::shared::error::AppError> {
    let config_path = state.config.config_path();

    // Validate model name
    if payload.model.is_empty() {
        return Ok(Json(SwitchModelResponse {
            success: false,
            message: "Model name is required".to_string(),
            current_model: None,
        }));
    }

    // Check if model is in available list
    let available = repository::get_available_models();
    let model_info = available.iter().find(|m| m.name == payload.model);

    let provider = match model_info {
        Some(info) => info.provider.clone(),
        None => {
            return Ok(Json(SwitchModelResponse {
                success: false,
                message: format!("Unknown model: {}", payload.model),
                current_model: None,
            }));
        }
    };

    // Update config.yaml
    repository::update_model_in_config(&config_path, &payload.model, &provider)
        .map_err(|e| crate::shared::error::AppError::Internal(format!("Failed to update config: {}", e)))?;

    tracing::info!("Model switched to {} (provider: {})", payload.model, provider);

    Ok(Json(SwitchModelResponse {
        success: true,
        message: format!("Model switched to {}", payload.model),
        current_model: Some(payload.model),
    }))
}
