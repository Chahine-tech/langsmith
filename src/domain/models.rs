use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a translatable string extracted from code
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TranslationKey {
    pub id: String,        // e.g., "button_login"
    pub source: String,    // Original string: "Login"
    pub file_path: String, // Where it was found
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
#[allow(clippy::upper_case_acronyms)]
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

/// Extended TranslationKey with byte position tracking for string replacement
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TranslationKeyWithPosition {
    pub id: String,        // e.g., "button_login"
    pub source: String,    // Original string: "Login"
    pub file_path: String, // Where it was found
    pub line: usize,
    pub start_byte: usize, // Position in file (bytes)
    pub end_byte: usize,
    pub quote_type: QuoteType,
}

/// Type of quotes used in string literals
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteType {
    Double,   // "text"
    Single,   // 'text'
    Template, // `text`
    JsxText,  // <div>text</div>
}

/// Strategy for code replacement (how to generate translation calls)
#[derive(Debug, Clone)]
pub enum ReplacementStrategy {
    ReactI18n, // {t("key")} with react-i18next
    VueI18n,   // {{ $t('key') }} with vue-i18n
    Generic,   // t("key") with generic import
}

impl ReplacementStrategy {
    #[allow(dead_code)]
    pub fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.to_lowercase().as_str() {
            "react-i18n" => Ok(Self::ReactI18n),
            "vue-i18n" => Ok(Self::VueI18n),
            "generic" => Ok(Self::Generic),
            _ => Err(anyhow::anyhow!(
                "Unknown strategy: {}. Supported: react-i18n, vue-i18n, generic",
                s
            )),
        }
    }

    #[allow(dead_code)]
    pub fn import_statement(&self) -> &str {
        match self {
            Self::ReactI18n => "import { useTranslation } from 'react-i18next';",
            Self::VueI18n => "import { useI18n } from 'vue-i18n';",
            Self::Generic => "import { t } from './i18n';",
        }
    }

    #[allow(dead_code)]
    pub fn translate_call(&self, key: &str, in_jsx: bool) -> String {
        match self {
            Self::ReactI18n => {
                if in_jsx {
                    format!("{{t(\"{}\")}}", key)
                } else {
                    format!("t(\"{}\")", key)
                }
            }
            Self::VueI18n => format!("{{{{ $t('{}') }}}}", key),
            Self::Generic => format!("t(\"{}\")", key),
        }
    }
}
