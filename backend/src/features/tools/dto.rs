use serde::{Deserialize, Serialize};

/// Current model configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub default: String,
    pub provider: String,
    pub fallback: Option<String>,
    pub base_url: Option<String>,
}

/// Response for GET /api/tools/models
#[derive(Debug, Serialize)]
pub struct ModelsResponse {
    pub current: ModelInfo,
    pub available: Vec<AvailableModel>,
}

/// An available model option
#[derive(Debug, Serialize)]
pub struct AvailableModel {
    pub name: String,
    pub provider: String,
    pub description: Option<String>,
}

/// Request for POST /api/tools/switch-model
#[derive(Debug, Deserialize)]
pub struct SwitchModelRequest {
    pub model: String,
}

/// Response for POST /api/tools/switch-model
#[derive(Debug, Serialize)]
pub struct SwitchModelResponse {
    pub success: bool,
    pub message: String,
    pub current_model: Option<String>,
}
