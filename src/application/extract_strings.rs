use crate::domain::models::LanguageFile;
use crate::domain::ports::{FileScanner, FileWriter, StringExtractor};
use std::collections::HashMap;
use std::path::Path;

/// Use case: Extract all translatable strings from a codebase
#[allow(dead_code)]
pub struct ExtractStringsUseCase;

impl ExtractStringsUseCase {
    #[allow(dead_code)]
    pub async fn execute(
        source_path: &Path,
        output_path: &Path,
        base_language: &str,
        scanner: &dyn FileScanner,
        extractor: &dyn StringExtractor,
        writer: &dyn FileWriter,
    ) -> anyhow::Result<()> {
        // 1. Scan all supported files
        let files = scanner.scan(source_path).await?;
        tracing::info!("Scanned {} files", files.len());

        // 2. Extract strings from each file
        let mut all_keys = HashMap::new();
        for (file_path, file_type) in files {
            let keys = extractor.extract(&file_path, file_type).await?;
            for key in keys {
                all_keys.insert(key.id.clone(), key.source.clone());
            }
        }

        tracing::info!("Found {} unique keys", all_keys.len());

        // 3. Create language file and write
        let mut language_file = LanguageFile::new();
        for (key, value) in all_keys {
            language_file.insert(key, value);
        }

        let output_file = output_path.join(format!("{}.json", base_language));
        writer
            .write_language_file(&output_file, &language_file)
            .await?;

        Ok(())
    }
}
