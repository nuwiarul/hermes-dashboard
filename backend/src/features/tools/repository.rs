use super::dto::{AvailableModel, MessageTarget, ModelInfo};
use std::path::Path;

// ── Model functions ──

/// Read current model config from config.yaml
pub fn read_model_config(config_path: &Path) -> Result<ModelInfo, std::io::Error> {
    let raw_yaml = std::fs::read_to_string(config_path)?;

    let default =
        extract_yaml_string(&raw_yaml, "default").unwrap_or_else(|| "unknown".to_string());
    let provider =
        extract_yaml_string(&raw_yaml, "provider").unwrap_or_else(|| "unknown".to_string());
    let fallback = extract_yaml_string(&raw_yaml, "fallback");
    let base_url = extract_yaml_string(&raw_yaml, "base_url");

    Ok(ModelInfo {
        default,
        provider,
        fallback,
        base_url,
    })
}

/// Get list of available models
pub fn get_available_models() -> Vec<AvailableModel> {
    vec![
        AvailableModel {
            name: "mimo-v2.5".to_string(),
            provider: "xiaomi".to_string(),
            description: Some("Xiaomi MiMo v2.5 - Fast, good for general tasks".to_string()),
        },
        AvailableModel {
            name: "mimo-v2.5-pro".to_string(),
            provider: "xiaomi".to_string(),
            description: Some("Xiaomi MiMo v2.5 Pro - Better reasoning".to_string()),
        },
        AvailableModel {
            name: "deepseek-v4-flash".to_string(),
            provider: "deepseek".to_string(),
            description: Some("DeepSeek V4 Flash - Fast coding model".to_string()),
        },
        AvailableModel {
            name: "deepseek-v3".to_string(),
            provider: "deepseek".to_string(),
            description: Some("DeepSeek V3 - Strong reasoning".to_string()),
        },
        AvailableModel {
            name: "claude-sonnet-4".to_string(),
            provider: "anthropic".to_string(),
            description: Some("Claude Sonnet 4 - Best coding model".to_string()),
        },
        AvailableModel {
            name: "gpt-4.1".to_string(),
            provider: "openai".to_string(),
            description: Some("GPT-4.1 - General purpose".to_string()),
        },
    ]
}

/// Update model in config.yaml
pub fn update_model_in_config(
    config_path: &Path,
    new_model: &str,
    new_provider: &str,
) -> Result<(), std::io::Error> {
    let raw_yaml = std::fs::read_to_string(config_path)?;

    let new_yaml = replace_yaml_value(&raw_yaml, "default", new_model);
    let new_yaml = replace_yaml_value(&new_yaml, "provider", new_provider);

    std::fs::write(config_path, new_yaml)?;

    Ok(())
}

// ── Send Message functions (Task 10.3) ──

/// List available messaging targets via `hermes send --list --json`
pub fn list_send_targets() -> Result<Vec<MessageTarget>, String> {
    let output = std::process::Command::new("hermes")
        .args(["send", "--list", "--json"])
        .output()
        .map_err(|e| format!("Failed to run hermes send: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("hermes send --list failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse hermes output: {}", e))?;

    let mut targets = Vec::new();

    if let Some(platforms) = json.get("platforms").and_then(|v| v.as_object()) {
        for (platform, contacts) in platforms {
            if let Some(arr) = contacts.as_array() {
                for contact in arr {
                    let id = contact
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let name = contact
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let target_type = contact
                        .get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string();
                    let thread_id = contact
                        .get("thread_id")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());

                    targets.push(MessageTarget {
                        platform: platform.clone(),
                        id: id.clone(),
                        name,
                        target_type,
                        thread_id,
                    });
                }
            }
        }
    }

    Ok(targets)
}

/// Send a message via `hermes send`
pub fn send_message(message: &str, target: Option<&str>) -> Result<serde_json::Value, String> {
    let mut cmd = std::process::Command::new("hermes");
    cmd.arg("send");

    if let Some(t) = target {
        cmd.args(["--to", t]);
    }

    cmd.args(["--json"]);

    // Write message to stdin
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let mut child = cmd
        .spawn()
        .map_err(|e| format!("Failed to spawn hermes send: {}", e))?;

    if let Some(ref mut stdin) = child.stdin {
        use std::io::Write;
        stdin
            .write_all(message.as_bytes())
            .map_err(|e| format!("Failed to write to hermes stdin: {}", e))?;
    }
    // Drop stdin to close it
    drop(child.stdin.take());

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for hermes send: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("hermes send failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse hermes output: {}", e))?;

    Ok(json)
}

// ── YAML helpers ──

fn replace_yaml_value(yaml: &str, key: &str, new_value: &str) -> String {
    let mut result = String::new();
    let mut in_model_section = false;

    for line in yaml.lines() {
        let trimmed = line.trim();

        if trimmed == "model:" {
            in_model_section = true;
            result.push_str(line);
            result.push('\n');
            continue;
        }

        if in_model_section
            && !line.starts_with(' ')
            && !line.starts_with('\t')
            && !trimmed.is_empty()
        {
            in_model_section = false;
        }

        if in_model_section && trimmed.starts_with(&format!("{}:", key)) {
            let indent = line.len() - line.trim_start().len();
            let indent_str: String = " ".repeat(indent);
            result.push_str(&format!("{}{}: {}\n", indent_str, key, new_value));
            continue;
        }

        result.push_str(line);
        result.push('\n');
    }

    result
}

fn extract_yaml_string(yaml: &str, key: &str) -> Option<String> {
    for line in yaml.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&format!("{}:", key)) {
            let value = trimmed.split(':').nth(1)?.trim();
            let value = value.trim_matches('\'').trim_matches('"');
            if value.is_empty() {
                return None;
            }
            return Some(value.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_yaml_value() {
        let yaml = "model:\n  default: mimo-v2.5\n  provider: xiaomi\n";
        let result = replace_yaml_value(yaml, "default", "deepseek-v3");
        assert!(result.contains("default: deepseek-v3"));
        assert!(!result.contains("default: mimo-v2.5"));
    }
}
