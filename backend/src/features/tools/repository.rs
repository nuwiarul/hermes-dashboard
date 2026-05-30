use super::dto::{AvailableModel, MessageTarget, ModelInfo, ToolsetInfo};
use std::path::Path;

/// Path to hermes binary — use absolute path since systemd has limited PATH
const HERMES_BIN: &str = "/home/ubuntu/.local/bin/hermes";

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
    let output = std::process::Command::new(HERMES_BIN)
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
    let mut cmd = std::process::Command::new(HERMES_BIN);
    cmd.arg("send");

    if let Some(t) = target {
        cmd.args(["--to", t]);
    }

    cmd.args(["--json"]);

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

/// Replace a YAML key's value (simple line-based replacement)
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

// === Toolset functions (Task 10.2) ===

/// All available toolsets in Hermes
pub fn get_all_toolsets() -> Vec<ToolsetInfo> {
    vec![
        ToolsetInfo {
            name: "terminal".to_string(),
            description: "Execute shell commands on the server".to_string(),
            enabled: true,
            category: "Core".to_string(),
        },
        ToolsetInfo {
            name: "file".to_string(),
            description: "Read, write, and search files".to_string(),
            enabled: true,
            category: "Core".to_string(),
        },
        ToolsetInfo {
            name: "web".to_string(),
            description: "Web search and browse websites".to_string(),
            enabled: true,
            category: "Core".to_string(),
        },
        ToolsetInfo {
            name: "browser".to_string(),
            description: "Interactive browser automation".to_string(),
            enabled: true,
            category: "Core".to_string(),
        },
        ToolsetInfo {
            name: "vision".to_string(),
            description: "Analyze images and screenshots".to_string(),
            enabled: true,
            category: "Core".to_string(),
        },
        ToolsetInfo {
            name: "delegation".to_string(),
            description: "Spawn subagents for parallel tasks".to_string(),
            enabled: true,
            category: "Advanced".to_string(),
        },
        ToolsetInfo {
            name: "cronjob".to_string(),
            description: "Schedule and manage cron jobs".to_string(),
            enabled: true,
            category: "Advanced".to_string(),
        },
        ToolsetInfo {
            name: "todo".to_string(),
            description: "Manage task lists and todos".to_string(),
            enabled: true,
            category: "Advanced".to_string(),
        },
        ToolsetInfo {
            name: "skills".to_string(),
            description: "Load and manage skills".to_string(),
            enabled: true,
            category: "Advanced".to_string(),
        },
        ToolsetInfo {
            name: "image_gen".to_string(),
            description: "Generate images with AI".to_string(),
            enabled: true,
            category: "Media".to_string(),
        },
        ToolsetInfo {
            name: "video".to_string(),
            description: "Video processing and analysis".to_string(),
            enabled: true,
            category: "Media".to_string(),
        },
        ToolsetInfo {
            name: "tts".to_string(),
            description: "Text-to-speech conversion".to_string(),
            enabled: true,
            category: "Media".to_string(),
        },
        ToolsetInfo {
            name: "spotify".to_string(),
            description: "Control Spotify playback".to_string(),
            enabled: true,
            category: "Media".to_string(),
        },
        ToolsetInfo {
            name: "discord".to_string(),
            description: "Discord integration".to_string(),
            enabled: true,
            category: "Platforms".to_string(),
        },
        ToolsetInfo {
            name: "telegram".to_string(),
            description: "Telegram integration".to_string(),
            enabled: true,
            category: "Platforms".to_string(),
        },
        ToolsetInfo {
            name: "homeassistant".to_string(),
            description: "Home Assistant control".to_string(),
            enabled: true,
            category: "Platforms".to_string(),
        },
    ]
}

/// Read disabled toolsets from config.yaml
pub fn read_disabled_toolsets(config_path: &Path) -> Result<Vec<String>, std::io::Error> {
    let raw_yaml = std::fs::read_to_string(config_path)?;

    let mut disabled = Vec::new();
    let mut in_disabled = false;

    for line in raw_yaml.lines() {
        let trimmed = line.trim();

        if trimmed == "disabled_toolsets:" {
            in_disabled = true;
            if trimmed.ends_with("[]") || trimmed.ends_with("disabled_toolsets: []") {
                in_disabled = false;
            }
            continue;
        }

        if in_disabled {
            if !line.starts_with(' ') && !line.starts_with('\t') && !trimmed.is_empty() {
                in_disabled = false;
                continue;
            }

            if let Some(name) = trimmed.strip_prefix("- ") {
                let name = name.trim().trim_matches('\'').trim_matches('"');
                if !name.is_empty() {
                    disabled.push(name.to_string());
                }
            }
        }
    }

    Ok(disabled)
}

/// Update disabled toolsets in config.yaml
pub fn update_disabled_toolsets(
    config_path: &Path,
    disabled: &[String],
) -> Result<(), std::io::Error> {
    let raw_yaml = std::fs::read_to_string(config_path)?;
    let mut result = String::new();
    let mut in_disabled = false;
    let mut skipped_old = false;

    for line in raw_yaml.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("disabled_toolsets:") {
            in_disabled = true;
            result.push_str(&format!(
                "  disabled_toolsets: {}\n",
                format_disabled_list(disabled)
            ));
            skipped_old = true;
            continue;
        }

        if in_disabled {
            if trimmed.starts_with("- ") || trimmed.is_empty() {
                continue;
            }
            in_disabled = false;
        }

        if !skipped_old || !in_disabled {
            if line.trim().starts_with("disabled_toolsets:") && !skipped_old {
                continue;
            }
            result.push_str(line);
            result.push('\n');
        }
    }

    if !result.contains("disabled_toolsets:") {
        result = result.replacen(
            "agent:\n",
            &format!(
                "agent:\n  disabled_toolsets: {}\n",
                format_disabled_list(disabled)
            ),
            1,
        );
    }

    std::fs::write(config_path, result)?;
    Ok(())
}

/// Format disabled list as YAML array
fn format_disabled_list(disabled: &[String]) -> String {
    if disabled.is_empty() {
        "[]".to_string()
    } else {
        let items: Vec<String> = disabled.iter().map(|s| format!("'{}'", s)).collect();
        format!("[{}]", items.join(", "))
    }
}

// === Gateway Control functions (Task 10.4) ===

use super::dto::{GatewayProfile, GatewayStatusResponse};

/// Get gateway status by running `hermes gateway status`
pub fn get_gateway_status() -> Result<GatewayStatusResponse, String> {
    let output = std::process::Command::new(HERMES_BIN)
        .args(["gateway", "status"])
        .output()
        .map_err(|e| format!("Failed to run hermes gateway status: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let combined = format!("{}{}", stdout, stderr);

    let running = combined.contains("User gateway service is running")
        || combined.contains("Active: active (running)");

    let pid = extract_pid(&combined);
    let uptime = extract_uptime(&combined);
    let profiles = list_gateway_profiles()?;

    Ok(GatewayStatusResponse {
        running,
        pid,
        uptime,
        service_name: "hermes-gateway".to_string(),
        profiles,
        raw_status: combined,
    })
}

/// Restart gateway by running `hermes gateway restart`
pub fn restart_gateway() -> Result<String, String> {
    let output = std::process::Command::new(HERMES_BIN)
        .args(["gateway", "restart"])
        .output()
        .map_err(|e| format!("Failed to run hermes gateway restart: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        return Err(format!(
            "Gateway restart failed: {}",
            if stderr.is_empty() { &stdout } else { &stderr }
        ));
    }

    let combined = format!("{}{}", stdout, stderr);
    Ok(combined)
}

/// List gateway profiles from `hermes gateway list`
fn list_gateway_profiles() -> Result<Vec<GatewayProfile>, String> {
    let output = std::process::Command::new(HERMES_BIN)
        .args(["gateway", "list"])
        .output()
        .map_err(|e| format!("Failed to run hermes gateway list: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let mut profiles = Vec::new();

    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('✓') || trimmed.starts_with('✗') {
            let active = trimmed.starts_with('✓');
            let name = if let Some(start) = trimmed.find(char::is_whitespace) {
                let rest = &trimmed[start..];
                if let Some(end) = rest.find('(') {
                    rest[..end].trim().to_string()
                } else if let Some(end) = rest.find('—') {
                    rest[..end].trim().to_string()
                } else {
                    rest.trim().to_string()
                }
            } else {
                continue;
            };

            let profile_pid = if let Some(pid_pos) = trimmed.find("PID ") {
                Some(trimmed[pid_pos + 4..].trim().to_string())
            } else {
                None
            };

            profiles.push(GatewayProfile {
                name,
                active,
                pid: profile_pid,
            });
        }
    }

    Ok(profiles)
}

/// Extract PID from status output
fn extract_pid(text: &str) -> Option<String> {
    for line in text.lines() {
        if let Some(pos) = line.find("PID ") {
            let rest = &line[pos + 4..];
            let pid: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            if !pid.is_empty() {
                return Some(pid);
            }
        }
        if let Some(pos) = line.find("Main PID:") {
            let rest = &line[pos + 9..];
            let pid: String = rest
                .chars()
                .skip_while(|c| c.is_whitespace())
                .take_while(|c| c.is_ascii_digit())
                .collect();
            if !pid.is_empty() {
                return Some(pid);
            }
        }
    }
    None
}

/// Extract uptime from status output
fn extract_uptime(text: &str) -> Option<String> {
    for line in text.lines() {
        if line.contains("Active:") && line.contains("since") {
            if let Some(pos) = line.find("since ") {
                let uptime = line[pos + 6..].trim().to_string();
                return Some(uptime);
            }
        }
    }
    None
}
