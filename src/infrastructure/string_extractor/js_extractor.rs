use crate::domain::models::{FileType, QuoteType, TranslationKey, TranslationKeyWithPosition};
use crate::domain::ports::StringExtractor;
use async_trait::async_trait;
use regex::Regex;
use std::path::Path;

#[allow(dead_code)]
pub struct SwcStringExtractor;

#[async_trait]
impl StringExtractor for SwcStringExtractor {
    async fn extract(
        &self,
        path: &Path,
        file_type: FileType,
    ) -> anyhow::Result<Vec<TranslationKey>> {
        let keys_with_pos = self.extract_with_positions(path, file_type).await?;

        // Convert from TranslationKeyWithPosition to TranslationKey
        Ok(keys_with_pos
            .into_iter()
            .map(|k| TranslationKey {
                id: k.id,
                source: k.source,
                file_path: k.file_path,
                line: k.line,
            })
            .collect())
    }
}

/// Extended extraction with position tracking for string replacement
#[allow(dead_code)]
impl SwcStringExtractor {
    /// Extract strings with byte position tracking
    pub async fn extract_with_positions(
        &self,
        path: &Path,
        _file_type: FileType,
    ) -> anyhow::Result<Vec<TranslationKeyWithPosition>> {
        let content = std::fs::read_to_string(path)?;
        let mut keys = Vec::new();
        let mut seen = std::collections::HashSet::new();

        let excluded = self.get_excluded_strings();

        // 1. Match double-quoted strings: "text"
        if let Ok(re) = Regex::new(r#""([^"\\]|\\.)*""#) {
            for cap in re.find_iter(&content) {
                let match_range = cap.range();
                if let Some(text) = cap
                    .as_str()
                    .strip_prefix('"')
                    .and_then(|s| s.strip_suffix('"'))
                {
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

        // 2. Match single-quoted strings: 'text'
        if let Ok(re) = Regex::new(r"'([^'\\]|\\.)*'") {
            for cap in re.find_iter(&content) {
                let match_range = cap.range();
                if let Some(text) = cap
                    .as_str()
                    .strip_prefix('\'')
                    .and_then(|s| s.strip_suffix('\''))
                {
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

        // 3. Match template literals: `text` (with proper escaping)
        if let Ok(re) = Regex::new(r"`([^\`\\]|\\.)*`") {
            for cap in re.find_iter(&content) {
                let match_range = cap.range();
                if let Some(text) = cap
                    .as_str()
                    .strip_prefix('`')
                    .and_then(|s| s.strip_suffix('`'))
                {
                    // Skip template literals with expressions: ${...}
                    if !text.contains("${") {
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
                                    quote_type: QuoteType::Template,
                                });
                                seen.insert(key);
                            }
                        }
                    }
                }
            }
        }

        // 4. Match JSX text nodes: <div>Text here</div>
        if let Ok(re) = Regex::new(r">[^<]*</") {
            for cap in re.find_iter(&content) {
                let match_str = cap.as_str();
                // Extract text between > and </
                if let Some(text) = match_str
                    .strip_prefix('>')
                    .and_then(|s| s.strip_suffix("</"))
                {
                    let text = text.trim();

                    // Skip empty strings
                    if !text.is_empty() {
                        // Skip if contains JSX component expressions ${...} or {variables}
                        if !text.contains("${") && !text.contains("{") {
                            if self.should_extract(text, &excluded) {
                                let key = format_key(text);
                                if !seen.contains(&key) {
                                    let match_range = cap.range();
                                    let line = content[..match_range.start].lines().count();

                                    keys.push(TranslationKeyWithPosition {
                                        id: key.clone(),
                                        source: text.to_string(),
                                        file_path: path.to_string_lossy().to_string(),
                                        line,
                                        start_byte: match_range.start,
                                        end_byte: match_range.end,
                                        quote_type: QuoteType::JsxText,
                                    });
                                    seen.insert(key);
                                }
                            }
                        }
                    }
                }
            }
        }

        // 5. Match HTML attributes: placeholder="text", alt="text", title="text", aria-label="text"
        if let Ok(re) = Regex::new(r#"(placeholder|alt|title|aria-label)="([^"]*)""#) {
            for cap in re.captures_iter(&content) {
                if let Some(attr_value) = cap.get(2) {
                    let text = attr_value.as_str();

                    // Skip empty attributes and expressions
                    if !text.is_empty() && !text.contains("{") && !text.contains("${") {
                        if self.should_extract(text, &excluded) {
                            let key = format_key(text);
                            if !seen.contains(&key) {
                                let match_range = attr_value.range();
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

        // Skip JSX component names (PascalCase like React, Button, MyComponent)
        if self.is_pascal_case(text)
            && text
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false)
        {
            return false;
        }

        // Skip URLs (http, https, ftp, etc.)
        if text.starts_with("http://")
            || text.starts_with("https://")
            || text.starts_with("ftp://")
            || text.starts_with("data:")
            || text.starts_with("file://")
        {
            return false;
        }

        // Skip email patterns (contains @ and .)
        if text.contains('@') && text.contains('.') && text.len() > 5 {
            let parts: Vec<&str> = text.split('@').collect();
            if parts.len() == 2 && parts[1].contains('.') {
                return false;
            }
        }

        // Skip paths with multiple slashes (e.g., "/path/to/file")
        if text.matches('/').count() > 2 {
            return false;
        }

        // Skip pure package names or imports (lowercase with dashes/underscores)
        if text
            .chars()
            .all(|c| c.is_lowercase() || c == '-' || c == '_' || c == '/')
            && text.len() < 20
        {
            return false;
        }

        // Skip if starts with @ (scoped packages)
        if text.starts_with('@') {
            return false;
        }

        // Skip if looks like a relative path
        if text.contains("./") || text.contains("../") {
            return false;
        }

        true
    }

    /// Check if text is PascalCase
    fn is_pascal_case(&self, text: &str) -> bool {
        // PascalCase: starts with uppercase, contains no spaces, and has at least one lowercase letter
        if text.is_empty() {
            return false;
        }

        let first_char = text
            .chars()
            .next()
            .ok_or(())
            .map_err(|_| "empty")
            .map(|c| c.is_uppercase())
            .unwrap_or(false);

        if !first_char {
            return false;
        }

        // Should not contain spaces or special chars (except maybe underscores)
        if text.contains(' ') || text.contains('-') {
            return false;
        }

        // Should have at least one lowercase letter to be considered PascalCase
        text.chars().any(|c| c.is_lowercase())
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

    #[test]
    fn test_is_pascal_case() {
        let extractor = SwcStringExtractor;
        assert!(extractor.is_pascal_case("Button"));
        assert!(extractor.is_pascal_case("MyComponent"));
        assert!(extractor.is_pascal_case("React"));
        assert!(!extractor.is_pascal_case("button"));
        assert!(!extractor.is_pascal_case("hello world"));
        assert!(!extractor.is_pascal_case("user-profile"));
    }

    #[test]
    fn test_should_extract_urls() {
        let extractor = SwcStringExtractor;
        let excluded = vec![];
        assert!(!extractor.should_extract("https://example.com", &excluded));
        assert!(!extractor.should_extract("http://example.com", &excluded));
        assert!(!extractor.should_extract("ftp://example.com", &excluded));
        assert!(!extractor.should_extract("data:image/png;base64", &excluded));
    }

    #[test]
    fn test_should_extract_emails() {
        let extractor = SwcStringExtractor;
        let excluded = vec![];
        assert!(!extractor.should_extract("user@example.com", &excluded));
        assert!(!extractor.should_extract("test@domain.org", &excluded));
    }

    #[test]
    fn test_should_extract_paths() {
        let extractor = SwcStringExtractor;
        let excluded = vec![];
        assert!(!extractor.should_extract("/path/to/file/name", &excluded));
        assert!(!extractor.should_extract("./relative/path/file", &excluded));
        assert!(!extractor.should_extract("../parent/path", &excluded));
    }

    #[test]
    fn test_should_extract_component_names() {
        let extractor = SwcStringExtractor;
        let excluded = vec![];
        assert!(!extractor.should_extract("MyComponent", &excluded));
        assert!(!extractor.should_extract("Button", &excluded));
        assert!(!extractor.should_extract("UserProfile", &excluded));
    }

    #[test]
    fn test_should_extract_valid_strings() {
        let extractor = SwcStringExtractor;
        let excluded = vec![];
        assert!(extractor.should_extract("Hello World", &excluded));
        assert!(extractor.should_extract("Click Me", &excluded));
        assert!(extractor.should_extract("Welcome to App", &excluded));
        assert!(extractor.should_extract("Please enter your name", &excluded));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_pascal_case_detection() {
        let extractor = SwcStringExtractor;
        // Valid PascalCase component names that should be skipped
        assert!(extractor.is_pascal_case("Button"));
        assert!(extractor.is_pascal_case("MyComponent"));
        assert!(extractor.is_pascal_case("React"));
        assert!(extractor.is_pascal_case("UserProfile"));
        assert!(extractor.is_pascal_case("MyLongComponentName"));

        // Invalid cases that should not be filtered
        assert!(!extractor.is_pascal_case("button"));
        assert!(!extractor.is_pascal_case("hello world"));
        assert!(!extractor.is_pascal_case("user-profile"));
        assert!(!extractor.is_pascal_case(""));
        assert!(!extractor.is_pascal_case("ALLCAPS"));
    }

    #[test]
    fn test_filtering_urls() {
        let extractor = SwcStringExtractor;
        let excluded = vec![];

        // URLs should be filtered
        assert!(!extractor.should_extract("https://example.com", &excluded));
        assert!(!extractor.should_extract("http://example.com/page", &excluded));
        assert!(!extractor.should_extract("ftp://files.example.com", &excluded));
        assert!(!extractor.should_extract("data:image/png;base64,ABC123", &excluded));
        assert!(!extractor.should_extract("file:///path/to/file", &excluded));
    }

    #[test]
    fn test_filtering_emails() {
        let extractor = SwcStringExtractor;
        let excluded = vec![];

        // Email addresses should be filtered
        assert!(!extractor.should_extract("user@example.com", &excluded));
        assert!(!extractor.should_extract("admin@domain.org", &excluded));
        assert!(!extractor.should_extract("test.user@company.net", &excluded));
    }

    #[test]
    fn test_filtering_file_paths() {
        let extractor = SwcStringExtractor;
        let excluded = vec![];

        // Paths with multiple slashes should be filtered
        assert!(!extractor.should_extract("/path/to/file/name", &excluded));
        assert!(!extractor.should_extract("./path/to/component", &excluded));
        assert!(!extractor.should_extract("../relative/path/file", &excluded));
        assert!(!extractor.should_extract("src/components/Button/index", &excluded));

        // Single slash paths or simple paths should pass initial filter
        // (may be filtered by other rules)
        assert!(!extractor.should_extract("./file", &excluded)); // Has ./
        assert!(!extractor.should_extract("../file", &excluded)); // Has ../
    }

    #[test]
    fn test_should_extract_valid_strings() {
        let extractor = SwcStringExtractor;
        let excluded = vec![];

        // Valid translatable strings
        assert!(extractor.should_extract("Hello World", &excluded));
        assert!(extractor.should_extract("Click Me", &excluded));
        assert!(extractor.should_extract("Welcome to App", &excluded));
        assert!(extractor.should_extract("Please enter your name", &excluded));
        assert!(extractor.should_extract("Save Changes", &excluded));
        assert!(extractor.should_extract("Confirm Action", &excluded));
    }

    #[test]
    fn test_should_extract_short_strings() {
        let extractor = SwcStringExtractor;
        let excluded = vec![];

        // Strings shorter than 3 chars should be filtered
        assert!(!extractor.should_extract("Hi", &excluded));
        assert!(!extractor.should_extract("OK", &excluded));
        assert!(!extractor.should_extract("a", &excluded));
    }

    #[test]
    fn test_format_key_with_special_chars() {
        assert_eq!(format_key("Hello World!"), "hello_world");
        assert_eq!(format_key("Click-Me!"), "click_me");
        // Note: @ is filtered out, so user@profile becomes userprofile
        assert_eq!(format_key("user@profile"), "userprofile");
        assert_eq!(format_key("save_changes"), "save_changes");
        assert_eq!(format_key("SAVE_CHANGES"), "save_changes");
    }

    #[test]
    fn test_excluded_package_names() {
        let extractor = SwcStringExtractor;
        let excluded = extractor.get_excluded_strings();

        // Common package/import names should be in excluded list
        assert!(excluded.contains(&"react"));
        assert!(excluded.contains(&"jsx"));
        assert!(excluded.contains(&"typescript"));
    }

    #[test]
    fn test_quote_type_combinations() {
        // Test that format_key works consistently for all quote types
        let key1 = format_key("Hello World");
        let key2 = format_key("Hello World");
        assert_eq!(key1, key2);

        // Different strings should have different keys
        let key3 = format_key("Welcome");
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_mixed_case_filtering() {
        let extractor = SwcStringExtractor;
        let excluded = vec![];

        // PascalCase names (likely component names)
        assert!(!extractor.should_extract("MyComponent", &excluded));
        assert!(!extractor.should_extract("FormField", &excluded));

        // Regular sentences with capitals should be extracted
        assert!(extractor.should_extract("Hello There", &excluded));
        assert!(extractor.should_extract("Welcome Back", &excluded));
    }
}
