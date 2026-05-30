use std::path::PathBuf;

pub struct AppConfig {
    pub hermes_home: PathBuf,
    pub port: u16,
    pub cors_origins: Vec<String>,
    pub rate_limit_login_max: u32,
    pub rate_limit_api_max: u32,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let hermes_home = std::env::var("HERMES_HOME")
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_default();
                format!("{}/.hermes", home)
            })
            .into();

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .parse()
            .unwrap_or(3001);

        let cors_origins = std::env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "https://hermes.vinrul.my.id".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let rate_limit_login_max = std::env::var("RATE_LIMIT_LOGIN_MAX")
            .unwrap_or_else(|_| "5".to_string())
            .parse()
            .unwrap_or(5);

        let rate_limit_api_max = std::env::var("RATE_LIMIT_API_MAX")
            .unwrap_or_else(|_| "60".to_string())
            .parse()
            .unwrap_or(60);

        Self {
            hermes_home,
            port,
            cors_origins,
            rate_limit_login_max,
            rate_limit_api_max,
        }
    }

    pub fn state_db_path(&self) -> PathBuf {
        self.hermes_home.join("state.db")
    }

    pub fn config_path(&self) -> PathBuf {
        self.hermes_home.join("config.yaml")
    }

    pub fn logs_path(&self) -> PathBuf {
        self.hermes_home.join("logs")
    }
}
