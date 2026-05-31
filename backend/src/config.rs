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

    pub fn dashboard_db_path(&self) -> PathBuf {
        self.hermes_home.join("dashboard.db")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_paths() {
        let config = AppConfig {
            hermes_home: PathBuf::from("/home/test/.hermes"),
            port: 3001,
            cors_origins: vec![],
            rate_limit_login_max: 5,
            rate_limit_api_max: 60,
        };

        assert_eq!(config.state_db_path(), PathBuf::from("/home/test/.hermes/state.db"));
        assert_eq!(config.config_path(), PathBuf::from("/home/test/.hermes/config.yaml"));
        assert_eq!(config.logs_path(), PathBuf::from("/home/test/.hermes/logs"));
    }

    #[test]
    fn test_config_from_env_defaults() {
        // Clear env vars to test defaults
        std::env::remove_var("HERMES_HOME");
        std::env::remove_var("PORT");
        std::env::remove_var("CORS_ORIGINS");
        std::env::remove_var("RATE_LIMIT_LOGIN_MAX");
        std::env::remove_var("RATE_LIMIT_API_MAX");

        let config = AppConfig::from_env();
        assert_eq!(config.port, 3001);
        assert_eq!(config.rate_limit_login_max, 5);
        assert_eq!(config.rate_limit_api_max, 60);
        assert!(!config.cors_origins.is_empty());
    }

    #[test]
    fn test_config_from_env_custom() {
        std::env::set_var("PORT", "8080");
        std::env::set_var("RATE_LIMIT_LOGIN_MAX", "10");
        std::env::set_var("RATE_LIMIT_API_MAX", "120");
        std::env::set_var("CORS_ORIGINS", "http://localhost:5173,http://localhost:3000");

        let config = AppConfig::from_env();
        assert_eq!(config.port, 8080);
        assert_eq!(config.rate_limit_login_max, 10);
        assert_eq!(config.rate_limit_api_max, 120);
        assert_eq!(config.cors_origins.len(), 2);
        assert!(config.cors_origins.contains(&"http://localhost:5173".to_string()));

        std::env::remove_var("PORT");
        std::env::remove_var("RATE_LIMIT_LOGIN_MAX");
        std::env::remove_var("RATE_LIMIT_API_MAX");
        std::env::remove_var("CORS_ORIGINS");
    }

    #[test]
    fn test_config_invalid_port_falls_back() {
        std::env::set_var("PORT", "not-a-number");
        let config = AppConfig::from_env();
        assert_eq!(config.port, 3001); // fallback
        std::env::remove_var("PORT");
    }
}
