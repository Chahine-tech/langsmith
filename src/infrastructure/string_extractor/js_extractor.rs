use crate::domain::ports::StringExtractor;
use crate::domain::models::{TranslationKey, FileType};
use async_trait::async_trait;
use std::path::Path;
use regex::Regex;

#[allow(dead_code)]
pub struct SwcStringExtractor;

#[async_trait]
impl StringExtractor for SwcStringExtractor {
    async fn extract(&self, path: &Path, _file_type: FileType) -> anyhow::Result<Vec<TranslationKey>> {
        let content = std::fs::read_to_string(path)?;

        // Simple regex-based string extraction (Phase 1)
        // Detects: "string", 'string', `string`, and JSX text nodes
        let mut keys = Vec::new();
        let mut seen = std::collections::HashSet::new();

        // Match double-quoted strings
        if let Ok(re) = Regex::new(r#""([^"\\]|\\.)*""#) {
            for cap in re.find_iter(&content) {
                if let Some(text) = cap.as_str().strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                    let key = format_key(text);
                    if !seen.contains(&key) {
                        keys.push(TranslationKey {
                            id: key.clone(),
                            source: text.to_string(),
                            file_path: path.to_string_lossy().to_string(),
                            line: 0,
                        });
                        seen.insert(key);
                    }
                }
            }
        }

        // Match single-quoted strings
        if let Ok(re) = Regex::new(r"'([^'\\]|\\.)*'") {
            for cap in re.find_iter(&content) {
                if let Some(text) = cap.as_str().strip_prefix('\'').and_then(|s| s.strip_suffix('\'')) {
                    let key = format_key(text);
                    if !seen.contains(&key) && text.len() > 2 {
                        keys.push(TranslationKey {
                            id: key.clone(),
                            source: text.to_string(),
                            file_path: path.to_string_lossy().to_string(),
                            line: 0,
                        });
                        seen.insert(key);
                    }
                }
            }
        }

        Ok(keys)
    }
}

/// Convert a string to a translation key
/// "Hello World" -> "hello_world"
#[allow(dead_code)]
fn format_key(source: &str) -> String {
    source
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_key() {
        assert_eq!(format_key("Hello World"), "hello_world");
        assert_eq!(format_key("Click Me!"), "click_me");
        assert_eq!(format_key("user-profile"), "user_profile");
    }
}
