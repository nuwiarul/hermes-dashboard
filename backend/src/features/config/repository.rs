use super::dto::ConfigDto;
use std::path::Path;

pub fn read_config(config_path: &Path) -> Result<ConfigDto, std::io::Error> {
    let raw_yaml = std::fs::read_to_string(config_path)?;

    let config_version = extract_yaml_i64(&raw_yaml, "_config_version");
    let model = extract_yaml_string(&raw_yaml, "default");
    let provider = extract_yaml_string(&raw_yaml, "provider");
    let max_turns = extract_yaml_i64(&raw_yaml, "max_turns");

    // Check if gateway is enabled (look for gateway section)
    let gateway_enabled =
        raw_yaml.contains("gateway:") && !raw_yaml.contains("gateway:\n  enabled: false");

    Ok(ConfigDto {
        config_version,
        model,
        provider,
        gateway_enabled,
        max_turns,
        raw_yaml,
    })
}

fn extract_yaml_string(yaml: &str, key: &str) -> Option<String> {
    for line in yaml.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&format!("{}:", key)) {
            let value = trimmed.split(':').nth(1)?.trim();
            // Remove quotes if present
            let value = value.trim_matches('\'').trim_matches('"');
            if value.is_empty() {
                return None;
            }
            return Some(value.to_string());
        }
    }
    None
}

fn extract_yaml_i64(yaml: &str, key: &str) -> Option<i64> {
    extract_yaml_string(yaml, key)?.parse().ok()
}
