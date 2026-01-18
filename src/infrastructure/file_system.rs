use crate::domain::models::{FileType, LanguageFile};
use crate::domain::ports::{FileScanner, FileWriter};
use async_trait::async_trait;
use std::path::Path;
use tokio::fs;
use walkdir::WalkDir;

#[allow(dead_code)]
pub struct FileSystemWriter;

#[async_trait]
impl FileWriter for FileSystemWriter {
    async fn write_language_file(
        &self,
        path: &Path,
        language: &LanguageFile,
    ) -> anyhow::Result<()> {
        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let json = serde_json::to_string_pretty(&language.translations)?;
        fs::write(path, json).await?;

        tracing::info!("Written {}", path.display());
        Ok(())
    }

    async fn read_language_file(&self, path: &Path) -> anyhow::Result<LanguageFile> {
        let content = fs::read_to_string(path).await?;
        let translations = serde_json::from_str(&content)?;
        Ok(LanguageFile { translations })
    }
}

#[allow(dead_code)]
pub struct FileSystemScanner;

#[async_trait]
impl FileScanner for FileSystemScanner {
    async fn scan(&self, root: &Path) -> anyhow::Result<Vec<(std::path::PathBuf, FileType)>> {
        let mut files = Vec::new();

        for entry in WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            if let Some(ext) = entry.path().extension()
                && let Some(ext_str) = ext.to_str()
            {
                let file_type = FileType::from_extension(ext_str);
                if file_type.is_supported() {
                    files.push((entry.path().to_path_buf(), file_type));
                }
            }
        }

        Ok(files)
    }
}
