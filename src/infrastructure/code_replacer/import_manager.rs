use crate::domain::models::{FileType, ReplacementStrategy};
use crate::domain::ports::ImportManager;
use async_trait::async_trait;
use regex::Regex;

pub struct SimpleImportManager;

impl SimpleImportManager {
    /// Check if an import statement already exists in content
    fn has_import(content: &str, import_statement: &str) -> bool {
        // Extract the package name from import statement
        // e.g., "import { useTranslation } from 'react-i18next';" -> react-i18next
        // Regex compile-time unwrap is safe (compile-time constant)
        let re = Regex::new(r#"from\s+['"]([^'"]+)['"]"#).expect("invalid regex");
        if let Some(cap) = re.captures(import_statement) {
            if let Some(package) = cap.get(1) {
                content.contains(package.as_str())
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Find the position to insert new imports (after last existing import)
    fn find_import_insertion_point(content: &str) -> usize {
        // Find last import statement
        // Regex compile-time unwrap is safe (compile-time constant)
        let re_pattern = r#"(?m)^import\s+.*from\s+['"][^'"]+['"];?\s*$"#;
        let re = Regex::new(re_pattern).expect("invalid regex");

        if let Some(last_match) = re.find_iter(content).last() {
            // Insert after last import
            last_match.end()
        } else {
            // No imports found, insert at beginning
            0
        }
    }
}

#[async_trait]
impl ImportManager for SimpleImportManager {
    async fn ensure_import(
        &self,
        content: &str,
        _file_type: FileType,
        strategy: &ReplacementStrategy,
    ) -> anyhow::Result<String> {
        let import_stmt = strategy.import_statement();

        // Check if import already exists
        if Self::has_import(content, import_stmt) {
            return Ok(content.to_string());
        }

        let insertion_point = Self::find_import_insertion_point(content);

        let mut result = String::new();
        result.push_str(&content[..insertion_point]);
        result.push('\n');
        result.push_str(import_stmt);
        result.push('\n');
        result.push_str(&content[insertion_point..]);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_import() {
        let content = "import { useTranslation } from \"react-i18next\";\nconst x = 1;";
        let import_stmt = "import { useTranslation } from \"react-i18next\";";
        assert!(SimpleImportManager::has_import(content, import_stmt));

        let content2 = "const x = 1;";
        assert!(!SimpleImportManager::has_import(content2, import_stmt));
    }

    #[test]
    fn test_find_import_insertion_point() {
        let content = "import x from \"a\";\nimport y from \"b\";\nconst z = 1;";
        let point = SimpleImportManager::find_import_insertion_point(content);
        // Should be after the second import
        assert!(point > 0);
    }
}
