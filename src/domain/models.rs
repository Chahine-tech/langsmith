use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Represents a translatable string extracted from code
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TranslationKey {
    pub id: String,           // e.g., "button_login"
    pub source: String,       // Original string: "Login"
    pub file_path: String,    // Where it was found
    pub line: usize,
}

/// Represents a language and its translations
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageFile {
    #[serde(flatten)]
    pub translations: HashMap<String, String>,
}

impl LanguageFile {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            translations: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, key: String, value: String) {
        self.translations.insert(key, value);
    }

    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&String> {
        self.translations.get(key)
    }
}

/// Supported file types for extraction
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    JavaScript,
    TypeScript,
    JSX,
    TSX,
    Vue,
    HTML,
    Other,
}

impl FileType {
    #[allow(dead_code)]
    pub fn from_extension(ext: &str) -> Self {
        match ext {
            "js" => FileType::JavaScript,
            "ts" => FileType::TypeScript,
            "jsx" => FileType::JSX,
            "tsx" => FileType::TSX,
            "vue" => FileType::Vue,
            "html" | "htm" => FileType::HTML,
            _ => FileType::Other,
        }
    }

    #[allow(dead_code)]
    pub fn is_supported(&self) -> bool {
        !matches!(self, FileType::Other)
    }
}

/// Configuration for extraction
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExtractionConfig {
    pub source_dir: String,
    pub output_dir: String,
    pub base_language: String,
}
