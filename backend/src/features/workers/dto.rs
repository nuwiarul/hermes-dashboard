use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterWorkerRequest {
    pub name: String,
    pub ip: String,
    pub role: Option<String>,
    pub os: String,
    pub arch: String,
    pub ram_total: Option<i64>,
    pub disk_total: Option<i64>,
    pub capabilities: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct HeartbeatRequest {
    pub status: Option<String>,
    pub current_task: Option<String>,
    pub ram_used: Option<i64>,
    pub disk_used: Option<i64>,
    pub active_model: Option<String>,
    pub last_config_applied_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HeartbeatResponse {
    pub success: bool,
    pub message: String,
    pub config: Option<serde_json::Value>,
    pub config_updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkerDto {
    pub id: i64,
    pub name: String,
    pub ip: String,
    pub role: String,
    pub os: String,
    pub arch: String,
    pub ram_total: i64,
    pub disk_total: i64,
    pub capabilities: String,
    pub status: String,
    pub current_task: Option<String>,
    pub ram_used: i64,
    pub disk_used: i64,
    pub active_model: Option<String>,
    pub last_heartbeat: Option<String>,
    pub registered_at: String,
    pub config: String,
}

#[derive(Debug, Serialize)]
pub struct WorkerListDto {
    pub workers: Vec<WorkerDto>,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct RegisterWorkerResponse {
    pub id: i64,
    pub name: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct WorkerConfigRequest {
    pub model: Option<String>,
    pub provider: Option<String>,
    pub max_tokens: Option<i64>,
    pub temperature: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct WorkerConfigResponse {
    pub success: bool,
    pub message: String,
    pub applied_at: String,
}
