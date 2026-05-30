use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ConfigDto {
    pub config_version: Option<i64>,
    pub model: Option<String>,
    pub provider: Option<String>,
    pub gateway_enabled: bool,
    pub max_turns: Option<i64>,
    pub raw_yaml: String,
}
