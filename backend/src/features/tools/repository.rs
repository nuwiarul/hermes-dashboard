use super::dto::{AvailableModel, ModelInfo, ToolsetInfo};
use std::path::Path;

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

/// Get list of available models (hardcoded for now, could be dynamic later)
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

    // Replace model.default value
    let new_yaml = replace_yaml_value(&raw_yaml, "default", new_model);

    // Replace model.provider value
    let new_yaml = replace_yaml_value(&new_yaml, "provider", new_provider);

    std::fs::write(config_path, new_yaml)?;

    Ok(())
}

/// Replace a YAML key's value (simple line-based replacement)
fn replace_yaml_value(yaml: &str, key: &str, new_value: &str) -> String {
    let mut result = String::new();
    let mut in_model_section = false;

    for line in yaml.lines() {
        let trimmed = line.trim();

        // Track if we're in the model section
        if trimmed == "model:" {
            in_model_section = true;
            result.push_str(line);
            result.push('\n');
            continue;
        }

        // Exit model section when we hit another top-level key
        if in_model_section
            && !line.starts_with(' ')
            && !line.starts_with('\t')
            && !trimmed.is_empty()
        {
            in_model_section = false;
        }

        // Replace the key if we're in the model section
        if in_model_section && trimmed.starts_with(&format!("{}:", key)) {
            // Determine indentation
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

// === Toolset functions ===

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

    // Find disabled_toolsets section
    let mut disabled = Vec::new();
    let mut in_disabled = false;

    for line in raw_yaml.lines() {
        let trimmed = line.trim();

        if trimmed == "disabled_toolsets:" {
            in_disabled = true;
            // Check if it's an empty list
            if trimmed.ends_with("[]") || trimmed.ends_with("disabled_toolsets: []") {
                in_disabled = false;
            }
            continue;
        }

        if in_disabled {
            // Exit if we hit another key (not indented)
            if !line.starts_with(' ') && !line.starts_with('\t') && !trimmed.is_empty() {
                in_disabled = false;
                continue;
            }

            // Extract toolset name from list item (e.g., "  - terminal")
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

        // Find disabled_toolsets section
        if trimmed.starts_with("disabled_toolsets:") {
            in_disabled = true;
            result.push_str(&format!(
                "  disabled_toolsets: {}\n",
                format_disabled_list(disabled)
            ));
            skipped_old = true;
            continue;
        }

        // Skip old list items
        if in_disabled {
            if trimmed.starts_with("- ") || trimmed.is_empty() {
                continue;
            }
            // Hit next key
            in_disabled = false;
        }

        // Keep lines that are not in disabled_toolsets
        if !skipped_old || !in_disabled {
            // Skip the old disabled_toolsets line if we haven't processed it yet
            if line.trim().starts_with("disabled_toolsets:") && !skipped_old {
                continue;
            }
            result.push_str(line);
            result.push('\n');
        }
    }

    // If disabled_toolsets wasn't found, add it after agent section
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
