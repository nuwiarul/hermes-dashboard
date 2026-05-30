use super::dto::CronJobDto;
use crate::AppState;
use anyhow::Result;
use std::path::PathBuf;

pub fn get_jobs_path(state: &AppState) -> PathBuf {
    state.config.hermes_home.join("cron").join("jobs.json")
}

pub async fn find_all(state: &AppState) -> Result<Vec<CronJobDto>> {
    let path = get_jobs_path(state);

    if !path.exists() {
        return Ok(vec![]);
    }

    let content = tokio::fs::read_to_string(&path).await?;
    let parsed: serde_json::Value = serde_json::from_str(&content)?;

    // jobs.json can be either:
    // 1. A plain array: [...]
    // 2. An object with "jobs" key: { "jobs": [...], "updated_at": "..." }
    let raw_jobs: Vec<serde_json::Value> = match &parsed {
        serde_json::Value::Array(arr) => arr.clone(),
        serde_json::Value::Object(obj) => obj
            .get("jobs")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default(),
        _ => vec![],
    };

    let jobs = raw_jobs.into_iter().map(|j| parse_job(j)).collect();

    Ok(jobs)
}

fn parse_job(j: serde_json::Value) -> CronJobDto {
    let id = j
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    let name = j
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let schedule = extract_schedule(j.get("schedule"));

    let prompt = j
        .get("prompt")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let enabled = j.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);

    let state_str = j
        .get("state")
        .and_then(|v| v.as_str())
        .unwrap_or(if enabled { "scheduled" } else { "paused" })
        .to_string();

    let next_run = j
        .get("next_run_at")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let deliver: Vec<String> = match j.get("deliver") {
        Some(serde_json::Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
        Some(serde_json::Value::String(s)) => vec![s.clone()],
        _ => vec![],
    };

    let skills: Vec<String> = j
        .get("skills")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let script = j
        .get("script")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let no_agent = j.get("no_agent").and_then(|v| v.as_bool()).unwrap_or(false);

    let workdir = j
        .get("workdir")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let created_at = j
        .get("created_at")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let last_run = j
        .get("last_run_at")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    CronJobDto {
        id,
        name,
        schedule,
        prompt,
        enabled,
        state: state_str,
        next_run,
        deliver,
        skills,
        script,
        no_agent,
        workdir,
        created_at,
        last_run,
    }
}

fn extract_schedule(schedule: Option<&serde_json::Value>) -> String {
    match schedule {
        None => "?".to_string(),
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(serde_json::Value::Object(obj)) => obj
            .get("display")
            .or_else(|| obj.get("value"))
            .or_else(|| obj.get("expr"))
            .or_else(|| obj.get("run_at"))
            .and_then(|v| v.as_str())
            .unwrap_or("?")
            .to_string(),
        Some(other) => other.to_string(),
    }
}
