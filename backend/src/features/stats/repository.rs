use super::dto::{SourceCountDto, StatsOverviewDto};
use sqlx::sqlite::SqlitePool;

pub async fn get_stats(db: &SqlitePool) -> Result<StatsOverviewDto, sqlx::Error> {
    // Combined query 1: sessions aggregates (6 stats in 1 query)
    let sessions_agg: (i64, i64, i64, i64, i64, i64, i64, f64, f64) = sqlx::query_as(
        "SELECT 
            COUNT(*),
            COALESCE(SUM(CASE WHEN date(started_at, 'unixepoch') = date('now') THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(input_tokens), 0),
            COALESCE(SUM(output_tokens), 0),
            COALESCE(SUM(cache_read_tokens), 0),
            COALESCE(SUM(reasoning_tokens), 0),
            COALESCE(SUM(tool_call_count), 0),
            COALESCE(SUM(estimated_cost_usd), 0.0),
            COALESCE(SUM(actual_cost_usd), 0.0)
         FROM sessions",
    )
    .fetch_one(db)
    .await?;

    // Combined query 2: messages aggregates (2 stats in 1 query)
    let messages_agg: (i64, i64) = sqlx::query_as(
        "SELECT 
            COUNT(*),
            COALESCE(SUM(CASE WHEN date(timestamp, 'unixepoch') = date('now') THEN 1 ELSE 0 END), 0)
         FROM messages",
    )
    .fetch_one(db)
    .await?;

    // Source breakdown (separate because it returns rows)
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

    Ok(StatsOverviewDto {
        total_sessions: sessions_agg.0,
        sessions_today: sessions_agg.1,
        total_input_tokens: sessions_agg.2,
        total_output_tokens: sessions_agg.3,
        total_cache_read_tokens: sessions_agg.4,
        total_reasoning_tokens: sessions_agg.5,
        total_tool_calls: sessions_agg.6,
        estimated_cost_usd: sessions_agg.7,
        actual_cost_usd: sessions_agg.8,
        total_messages: messages_agg.0,
        messages_today: messages_agg.1,
        active_sources,
    })
}
