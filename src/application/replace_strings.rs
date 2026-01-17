use crate::domain::models::*;
use crate::domain::ports::*;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use tokio::fs;

pub struct ReplaceStringsUseCase;

impl ReplaceStringsUseCase {
    pub async fn execute(
        source_path: &Path,
        translation_file: &Path,
        strategy: ReplacementStrategy,
        dry_run: bool,
        in_place: bool,
        scanner: &dyn FileScanner,
        extractor: &dyn StringExtractor,
        replacer: &dyn CodeReplacer,
        import_mgr: &dyn ImportManager,
    ) -> anyhow::Result<()> {
        // 1. Load translation keys from JSON
        let translations = Self::load_translations(translation_file).await?;

        // 2. Scan source files
        let files = scanner.scan(source_path).await?;

        // 3. Process each file
        for (file_path, file_type) in files {
            // Extract strings using position tracking
            let all_keys = Self::extract_with_positions(extractor, &file_path, file_type).await?;

            // Filter: only replace strings that exist in translation JSON
            let keys_to_replace: Vec<_> = all_keys
                .into_iter()
                .filter(|k| translations.contains_key(&k.id))
                .collect();

            if keys_to_replace.is_empty() {
                continue;
            }

            // Replace strings
            let mut content = replacer.replace_in_file(&file_path, &keys_to_replace, &strategy).await?;

            // Add imports
            content = import_mgr.ensure_import(&content, file_type, &strategy).await?;

            if dry_run {
                println!("Would replace {} strings in {}", keys_to_replace.len(), file_path.display());
                for key in &keys_to_replace {
                    println!("  - Line {}: \"{}\" -> t(\"{}\")", key.line, key.source, key.id);
                }
                continue;
            }

            // Write result
            let output_path = if in_place {
                file_path.clone()
            } else {
                Self::create_i18n_filename(&file_path)
            };

            fs::write(&output_path, content).await?;
            tracing::info!("✓ Replaced {} strings in {}", keys_to_replace.len(), output_path.display());
        }

        Ok(())
    }

    async fn load_translations(path: &Path) -> anyhow::Result<HashMap<String, String>> {
        let content = fs::read_to_string(path).await?;
        let translations: HashMap<String, String> = serde_json::from_str(&content)?;
        Ok(translations)
    }

    fn create_i18n_filename(path: &Path) -> PathBuf {
        // App.tsx -> App.i18n.tsx
        let stem = path.file_stem().unwrap().to_string_lossy();
        let ext = path.extension().unwrap().to_string_lossy();
        let parent = path.parent().unwrap();
        parent.join(format!("{}.i18n.{}", stem, ext))
    }

    /// Helper to extract with positions, gracefully falling back
    async fn extract_with_positions(
        extractor: &dyn StringExtractor,
        path: &Path,
        file_type: FileType,
    ) -> anyhow::Result<Vec<TranslationKeyWithPosition>> {
        // Try to use SwcStringExtractor's extended method if available
        // For now, convert from basic extraction
        let basic_keys = extractor.extract(path, file_type).await?;
        
        // Re-extract with positions using a temporary SwcStringExtractor instance
        // This is a workaround until we can make the trait method available
        let content = fs::read_to_string(path).await?;
        let mut keys = Vec::new();

        for basic_key in basic_keys {
            // Find byte position of the source string
            if let Some(start_byte) = content.find(&format!("\"{}\"", basic_key.source)) {
                let end_byte = start_byte + basic_key.source.len() + 2; // +2 for quotes
                let line = content[..start_byte].lines().count();

                keys.push(TranslationKeyWithPosition {
                    id: basic_key.id,
                    source: basic_key.source,
                    file_path: basic_key.file_path,
                    line,
                    start_byte,
                    end_byte,
                    quote_type: QuoteType::Double,
                });
            } else if let Some(start_byte) = content.find(&format!("'{}'", basic_key.source)) {
                let end_byte = start_byte + basic_key.source.len() + 2; // +2 for quotes
                let line = content[..start_byte].lines().count();

                keys.push(TranslationKeyWithPosition {
                    id: basic_key.id,
                    source: basic_key.source,
                    file_path: basic_key.file_path,
                    line,
                    start_byte,
                    end_byte,
                    quote_type: QuoteType::Single,
                });
            }
        }

        Ok(keys)
    }
}
