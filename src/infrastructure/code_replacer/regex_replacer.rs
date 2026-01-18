use crate::domain::models::{ReplacementStrategy, TranslationKeyWithPosition};
use crate::domain::ports::CodeReplacer;
use async_trait::async_trait;
use std::path::Path;
use tokio::fs;

pub struct RegexReplacer;

impl RegexReplacer {
    /// Detect if a position is inside JSX context
    fn detect_jsx_context(content: &str, position: usize) -> bool {
        // Simple heuristic: check if we're inside JSX tags
        // Look backwards for < or {, forwards for > or }
        let before = &content[..position.min(content.len())];

        // Count open/close brackets to determine if in JSX
        let open_angle = before.matches('<').count();
        let close_angle = before.matches('>').count();

        // If more open than close, we're likely in JSX
        open_angle > close_angle
    }
}

#[async_trait]
impl CodeReplacer for RegexReplacer {
    async fn replace_in_file(
        &self,
        file_path: &Path,
        keys: &[TranslationKeyWithPosition],
        strategy: &ReplacementStrategy,
    ) -> anyhow::Result<String> {
        let content = fs::read_to_string(file_path).await?;

        // Sort by start_byte DESC to replace back-to-front
        // This prevents byte positions from shifting as we replace
        let mut sorted_keys = keys.to_vec();
        sorted_keys.sort_by(|a, b| b.start_byte.cmp(&a.start_byte));

        let mut result = content.clone();

        for key in sorted_keys {
            let in_jsx = Self::detect_jsx_context(&content, key.start_byte);
            let replacement = strategy.translate_call(&key.id, in_jsx);

            // Validate byte positions are within bounds
            if key.end_byte <= result.len() && key.start_byte <= key.end_byte {
                // Replace at exact byte positions
                result.replace_range(key.start_byte..key.end_byte, &replacement);
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_jsx_context() {
        // Test inside JSX tags: position 4 is between < and >
        let content = r#"<div>"Hello"</div>"#;
        assert!(RegexReplacer::detect_jsx_context(content, 4));

        // Test outside JSX tags: no tags before position
        let content2 = r#"const msg = "Hello";"#;
        assert!(!RegexReplacer::detect_jsx_context(content2, 15));
    }
}
