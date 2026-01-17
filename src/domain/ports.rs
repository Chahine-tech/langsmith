use crate::domain::models::{TranslationKey, LanguageFile, FileType};
use async_trait::async_trait;
use std::path::Path;

/// Port: Responsible for extracting strings from files
#[async_trait]
#[allow(unused)]
pub trait StringExtractor: Send + Sync {
    /// Extract all translatable strings from a file
    async fn extract(&self, path: &Path, file_type: FileType) -> anyhow::Result<Vec<TranslationKey>>;
}

/// Port: Responsible for writing translation files
#[async_trait]
#[allow(unused)]
pub trait FileWriter: Send + Sync {
    /// Write a language file to disk
    async fn write_language_file(&self, path: &Path, language: &LanguageFile) -> anyhow::Result<()>;

    /// Read a language file from disk
    async fn read_language_file(&self, path: &Path) -> anyhow::Result<LanguageFile>;
}

/// Port: Responsible for finding files to process
#[async_trait]
#[allow(unused)]
pub trait FileScanner: Send + Sync {
    /// Scan a directory and return all supported files
    async fn scan(&self, root: &Path) -> anyhow::Result<Vec<(std::path::PathBuf, FileType)>>;
}

/// Port: Responsible for translating strings via API
#[async_trait]
#[allow(unused)]
pub trait Translator: Send + Sync {
    /// Translate text to target language
    async fn translate(&self, text: &str, target_lang: &str) -> anyhow::Result<String>;
}
