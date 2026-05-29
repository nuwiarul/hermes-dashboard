use std::path::PathBuf;

pub struct AppConfig {
    pub hermes_home: PathBuf,
    pub port: u16,
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

        Self { hermes_home, port }
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
