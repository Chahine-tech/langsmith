use crate::domain::ports::StringExtractor;
use crate::domain::models::{TranslationKey, FileType, TranslationKeyWithPosition, QuoteType};
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

        // List of strings to exclude (imports, package names, etc)
        let excluded = self.get_excluded_strings();

        // Match double-quoted strings
        if let Ok(re) = Regex::new(r#""([^"\\]|\\.)*""#) {
            for cap in re.find_iter(&content) {
                if let Some(text) = cap.as_str().strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                    if self.should_extract(text, &excluded) {
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
        }

        // Match single-quoted strings
        if let Ok(re) = Regex::new(r"'([^'\\]|\\.)*'") {
            for cap in re.find_iter(&content) {
                if let Some(text) = cap.as_str().strip_prefix('\'').and_then(|s| s.strip_suffix('\'')) {
                    if self.should_extract(text, &excluded) {
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
        }

        Ok(keys)
    }
}

/// Extended extraction with position tracking for string replacement
#[allow(dead_code)]
impl SwcStringExtractor {
    /// Extract strings with byte position tracking
    pub async fn extract_with_positions(&self, path: &Path, _file_type: FileType) 
        -> anyhow::Result<Vec<TranslationKeyWithPosition>>
    {
        let content = std::fs::read_to_string(path)?;
        let mut keys = Vec::new();
        let mut seen = std::collections::HashSet::new();

        let excluded = self.get_excluded_strings();

        // Match double-quoted strings
        if let Ok(re) = Regex::new(r#""([^"\\]|\\.)*""#) {
            for cap in re.find_iter(&content) {
                let match_range = cap.range();
                if let Some(text) = cap.as_str().strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                    if self.should_extract(text, &excluded) {
                        let key = format_key(text);
                        if !seen.contains(&key) {
                            let line = content[..match_range.start].lines().count();
                            
                            keys.push(TranslationKeyWithPosition {
                                id: key.clone(),
                                source: text.to_string(),
                                file_path: path.to_string_lossy().to_string(),
                                line,
                                start_byte: match_range.start,
                                end_byte: match_range.end,
                                quote_type: QuoteType::Double,
                            });
                            seen.insert(key);
                        }
                    }
                }
            }
        }

        // Match single-quoted strings
        if let Ok(re) = Regex::new(r"'([^'\\]|\\.)*'") {
            for cap in re.find_iter(&content) {
                let match_range = cap.range();
                if let Some(text) = cap.as_str().strip_prefix('\'').and_then(|s| s.strip_suffix('\'')) {
                    if self.should_extract(text, &excluded) {
                        let key = format_key(text);
                        if !seen.contains(&key) {
                            let line = content[..match_range.start].lines().count();
                            
                            keys.push(TranslationKeyWithPosition {
                                id: key.clone(),
                                source: text.to_string(),
                                file_path: path.to_string_lossy().to_string(),
                                line,
                                start_byte: match_range.start,
                                end_byte: match_range.end,
                                quote_type: QuoteType::Single,
                            });
                            seen.insert(key);
                        }
                    }
                }
            }
        }

        Ok(keys)
    }

    /// Check if a string should be extracted
    fn should_extract(&self, text: &str, excluded: &[&str]) -> bool {
        // Skip if too short
        if text.len() < 3 {
            return false;
        }

        // Skip if in excluded list
        if excluded.contains(&text) {
            return false;
        }

        // Skip pure package names or imports
        if text.chars().all(|c| c.is_lowercase() || c == '-' || c == '_' || c == '/') && text.len() < 20 {
            return false;
        }

        // Skip if starts with @ (scoped packages)
        if text.starts_with('@') {
            return false;
        }

        // Skip if looks like a path
        if text.contains("./") || text.contains("../") {
            return false;
        }

        true
    }

    /// List of strings that should not be extracted
    fn get_excluded_strings(&self) -> Vec<&'static str> {
        vec![
            "react",
            "tsx",
            "jsx",
            "javascript",
            "typescript",
            "vue",
            "angular",
            "svelte",
            "next",
            "remix",
            "gatsby",
            "node_modules",
            "dist",
            "build",
        ]
    }
}

/// Convert a string to a translation key
/// "Hello World" -> "hello_world"
/// "user-profile" -> "user_profile"
#[allow(dead_code)]
fn format_key(source: &str) -> String {
    source
        .to_lowercase()
        .replace("-", "_")
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
        assert_eq!(format_key("User Profile"), "user_profile");
    }
}
