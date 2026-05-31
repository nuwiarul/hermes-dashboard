use sqlx::sqlite::SqlitePool;
use std::path::Path;

pub async fn connect(db_path: &Path) -> anyhow::Result<SqlitePool> {
    let url = format!("sqlite:{}?mode=ro", db_path.display());
    let pool = SqlitePool::connect(&url).await?;
    Ok(pool)
}

/// Connect to a read-write database for dashboard-specific data.
pub async fn connect_rw(db_path: &Path) -> anyhow::Result<SqlitePool> {
    let url = format!("sqlite:{}?mode=rwc", db_path.display());
    let pool = SqlitePool::connect(&url).await?;
    Ok(pool)
}
