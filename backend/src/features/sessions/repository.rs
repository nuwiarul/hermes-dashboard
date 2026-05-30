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

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_db() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY, title TEXT, source TEXT,
                message_count INTEGER, started_at REAL, ended_at REAL, model TEXT
            )"
        ).execute(&pool).await.unwrap();
        sqlx::query(
            "INSERT INTO sessions (id, title, source, message_count, started_at, ended_at, model) 
             VALUES ('s1', 'Session 1', 'telegram', 10, 1704067200.0, NULL, 'gpt-4')"
        ).execute(&pool).await.unwrap();
        sqlx::query(
            "INSERT INTO sessions (id, title, source, message_count, started_at, ended_at, model) 
             VALUES ('s2', 'Session 2', 'cli', 5, 1704153600.0, 1704240000.0, 'mimo-v2.5')"
        ).execute(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn test_find_all_returns_sessions() {
        let pool = setup_db().await;
        let sessions = find_all(&pool, 50).await.unwrap();
        assert_eq!(sessions.len(), 2);
    }

    #[tokio::test]
    async fn test_find_all_respects_limit() {
        let pool = setup_db().await;
        let sessions = find_all(&pool, 1).await.unwrap();
        assert_eq!(sessions.len(), 1);
    }

    #[tokio::test]
    async fn test_count_all() {
        let pool = setup_db().await;
        let count = count_all(&pool).await.unwrap();
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_find_all_order_by_started_at_desc() {
        let pool = setup_db().await;
        let sessions = find_all(&pool, 50).await.unwrap();
        // s2 has later started_at (1704153600) so it should come first
        assert_eq!(sessions[0].id, "s2");
        assert_eq!(sessions[1].id, "s1");
    }
}
