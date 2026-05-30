use super::dto::{SourceCountDto, StatsOverviewDto};
use sqlx::sqlite::SqlitePool;

pub async fn get_stats(db: &SqlitePool) -> Result<StatsOverviewDto, sqlx::Error> {
    // Total sessions
    let total_sessions: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sessions")
        .fetch_one(db)
        .await?;

    // Total messages
    let total_messages: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM messages")
        .fetch_one(db)
        .await?;

    // Sessions today (started_at is Unix timestamp)
    let sessions_today: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sessions WHERE date(started_at, 'unixepoch') = date('now')",
    )
    .fetch_one(db)
    .await?;

    // Messages today (timestamp is Unix timestamp)
    let messages_today: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM messages WHERE date(timestamp, 'unixepoch') = date('now')",
    )
    .fetch_one(db)
    .await?;

    // Active sources
    let source_rows: Vec<(String, i64)> = sqlx::query_as(
        "SELECT COALESCE(source, 'unknown'), COUNT(*) 
         FROM sessions 
         GROUP BY source 
         ORDER BY COUNT(*) DESC",
    )
    .fetch_all(db)
    .await?;

    let active_sources: Vec<SourceCountDto> = source_rows
        .into_iter()
        .map(|(source, count)| SourceCountDto { source, count })
        .collect();

    // Token totals
    let tokens: (i64, i64, i64, i64) = sqlx::query_as(
        "SELECT 
            COALESCE(SUM(input_tokens), 0),
            COALESCE(SUM(output_tokens), 0),
            COALESCE(SUM(cache_read_tokens), 0),
            COALESCE(SUM(reasoning_tokens), 0)
         FROM sessions",
    )
    .fetch_one(db)
    .await?;

    // Tool calls total
    let tool_calls: (i64,) =
        sqlx::query_as("SELECT COALESCE(SUM(tool_call_count), 0) FROM sessions")
            .fetch_one(db)
            .await?;

    // Cost totals
    let costs: (f64, f64) = sqlx::query_as(
        "SELECT 
            COALESCE(SUM(estimated_cost_usd), 0.0),
            COALESCE(SUM(actual_cost_usd), 0.0)
         FROM sessions",
    )
    .fetch_one(db)
    .await?;

    Ok(StatsOverviewDto {
        total_sessions: total_sessions.0,
        total_messages: total_messages.0,
        sessions_today: sessions_today.0,
        messages_today: messages_today.0,
        active_sources,
        total_input_tokens: tokens.0,
        total_output_tokens: tokens.1,
        total_cache_read_tokens: tokens.2,
        total_reasoning_tokens: tokens.3,
        total_tool_calls: tool_calls.0,
        estimated_cost_usd: costs.0,
        actual_cost_usd: costs.1,
    })
}
