use super::dto::{RegisterWorkerRequest, WorkerDto};
use sqlx::sqlite::SqlitePool;

pub async fn register(
    db: &SqlitePool,
    req: &RegisterWorkerRequest,
) -> Result<i64, sqlx::Error> {
    let capabilities = serde_json::to_string(&req.capabilities.clone().unwrap_or_default())
        .unwrap_or_else(|_| "[]".to_string());
    let role = req.role.as_deref().unwrap_or("worker");

    let result = sqlx::query(
        "INSERT INTO workers (name, ip, role, os, arch, ram_total, disk_total, capabilities, status, registered_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'online', datetime('now'))"
    )
    .bind(&req.name)
    .bind(&req.ip)
    .bind(role)
    .bind(&req.os)
    .bind(&req.arch)
    .bind(req.ram_total.unwrap_or(0))
    .bind(req.disk_total.unwrap_or(0))
    .bind(&capabilities)
    .execute(db)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn upsert(
    db: &SqlitePool,
    req: &RegisterWorkerRequest,
) -> Result<i64, sqlx::Error> {
    // Try to find existing worker by name + ip
    let existing = sqlx::query_as::<_, WorkerDto>(
        "SELECT * FROM workers WHERE name = ? AND ip = ?"
    )
    .bind(&req.name)
    .bind(&req.ip)
    .fetch_optional(db)
    .await?;

    if let Some(worker) = existing {
        // Update existing worker
        let capabilities = serde_json::to_string(&req.capabilities.clone().unwrap_or_default())
            .unwrap_or_else(|_| "[]".to_string());
        let role = req.role.as_deref().unwrap_or("worker");

        sqlx::query(
            "UPDATE workers SET role = ?, os = ?, arch = ?, ram_total = ?, disk_total = ?, capabilities = ?, status = 'online', last_heartbeat = datetime('now')
             WHERE id = ?"
        )
        .bind(role)
        .bind(&req.os)
        .bind(&req.arch)
        .bind(req.ram_total.unwrap_or(0))
        .bind(req.disk_total.unwrap_or(0))
        .bind(&capabilities)
        .bind(worker.id)
        .execute(db)
        .await?;

        Ok(worker.id)
    } else {
        // Insert new worker
        register(db, req).await
    }
}

pub async fn heartbeat(
    db: &SqlitePool,
    worker_id: i64,
    status: Option<&str>,
    current_task: Option<&str>,
    ram_used: Option<i64>,
    disk_used: Option<i64>,
    active_model: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE workers SET
            status = COALESCE(?, status),
            current_task = ?,
            ram_used = COALESCE(?, ram_used),
            disk_used = COALESCE(?, disk_used),
            active_model = COALESCE(?, active_model),
            last_heartbeat = datetime('now')
         WHERE id = ?"
    )
    .bind(status)
    .bind(current_task)
    .bind(ram_used)
    .bind(disk_used)
    .bind(active_model)
    .bind(worker_id)
    .execute(db)
    .await?;

    Ok(())
}

pub async fn find_all(db: &SqlitePool) -> Result<Vec<WorkerDto>, sqlx::Error> {
    let workers = sqlx::query_as::<_, WorkerDto>(
        "SELECT * FROM workers ORDER BY last_heartbeat DESC"
    )
    .fetch_all(db)
    .await?;

    Ok(workers)
}

pub async fn find_by_id(db: &SqlitePool, id: i64) -> Result<Option<WorkerDto>, sqlx::Error> {
    let worker = sqlx::query_as::<_, WorkerDto>(
        "SELECT * FROM workers WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(worker)
}

pub async fn count_all(db: &SqlitePool) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM workers")
        .fetch_one(db)
        .await?;

    Ok(row.0)
}
