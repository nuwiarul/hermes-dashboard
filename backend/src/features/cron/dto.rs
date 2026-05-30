use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct CronJobDto {
    pub id: String,
    pub name: String,
    pub schedule: String,
    pub prompt: String,
    pub enabled: bool,
    pub state: String,
    pub next_run: Option<String>,
    pub deliver: Vec<String>,
    pub skills: Vec<String>,
    pub script: Option<String>,
    pub no_agent: bool,
    pub workdir: Option<String>,
    pub created_at: Option<String>,
    pub last_run: Option<String>,
}

#[derive(Serialize)]
pub struct CronJobsResponse {
    pub jobs: Vec<CronJobDto>,
    pub total: usize,
}
