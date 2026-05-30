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

// === Toolset DTOs ===

/// A toolset with its status
#[derive(Debug, Serialize)]
pub struct ToolsetInfo {
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub category: String,
}

/// Response for GET /api/tools/toolsets
#[derive(Debug, Serialize)]
pub struct ToolsetsResponse {
    pub toolsets: Vec<ToolsetInfo>,
    pub disabled_count: usize,
}

/// Request for POST /api/tools/toggle-toolset
#[derive(Debug, Deserialize)]
pub struct ToggleToolsetRequest {
    pub name: String,
    pub enabled: bool,
}

/// Response for POST /api/tools/toggle-toolset
#[derive(Debug, Serialize)]
pub struct ToggleToolsetResponse {
    pub success: bool,
    pub message: String,
    pub toolset: String,
    pub enabled: bool,
}

// === Send Message DTOs (Task 10.3) ===

/// A messaging target (platform + contact)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageTarget {
    pub platform: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub target_type: String,
    pub thread_id: Option<String>,
}

/// Response for GET /api/tools/targets
#[derive(Debug, Serialize)]
pub struct TargetsResponse {
    pub targets: Vec<MessageTarget>,
}

/// Request for POST /api/tools/send-message
#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub message: String,
    pub target: Option<String>,
}

/// Response for POST /api/tools/send-message
#[derive(Debug, Serialize)]
pub struct SendMessageResponse {
    pub success: bool,
    pub message: String,
    pub platform: Option<String>,
    pub chat_id: Option<String>,
    pub message_id: Option<String>,
}
