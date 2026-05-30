use super::dto::SessionDto;
use sqlx::sqlite::SqlitePool;

pub async fn find_all(db: &SqlitePool, limit: i64) -> Result<Vec<SessionDto>, sqlx::Error> {
    let sessions = sqlx::query_as::<_, SessionDto>(
        "SELECT 
            id,
            title,
            source,
            message_count,
            started_at,
            ended_at,
            model
         FROM sessions 
         ORDER BY started_at DESC 
         LIMIT ?",
    )
    .bind(limit)
    .fetch_all(db)
    .await?;

    Ok(sessions)
}

pub async fn count_all(db: &SqlitePool) -> Result<i64, sqlx::Error> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sessions")
        .fetch_one(db)
        .await?;
    Ok(count.0)
}
