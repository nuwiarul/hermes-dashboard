use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StatsOverviewDto {
    pub total_sessions: i64,
    pub total_messages: i64,
    pub sessions_today: i64,
    pub messages_today: i64,
    pub active_sources: Vec<SourceCountDto>,
    pub total_input_tokens: i64,
    pub total_output_tokens: i64,
    pub total_cache_read_tokens: i64,
    pub total_reasoning_tokens: i64,
    pub total_tool_calls: i64,
    pub estimated_cost_usd: f64,
    pub actual_cost_usd: f64,
}

#[derive(Debug, Serialize)]
pub struct SourceCountDto {
    pub source: String,
    pub count: i64,
}
