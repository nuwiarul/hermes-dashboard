use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SessionDto {
    pub id: String,
    pub title: Option<String>,
    pub source: String,
    pub message_count: Option<i64>,
    pub started_at: f64,
    pub ended_at: Option<f64>,
    pub model: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SessionSummaryDto {
    pub sessions: Vec<SessionDto>,
    pub total: i64,
}
