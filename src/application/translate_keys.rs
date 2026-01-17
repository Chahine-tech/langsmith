use crate::domain::ports::Translator;
use crate::domain::models::LanguageFile;
use std::path::Path;
use std::collections::HashMap;

/// Use case: Translate extracted strings to target languages
pub struct TranslateKeysUseCase;

impl TranslateKeysUseCase {
    pub async fn execute(
        source_file: &Path,
        target_langs: &[&str],
        translator: &dyn Translator,
    ) -> anyhow::Result<()> {
        // 1. Load source language file
        let source_content = std::fs::read_to_string(source_file)?;
        let source_file_obj: HashMap<String, String> = serde_json::from_str(&source_content)?;

        tracing::info!("Loaded {} strings from source", source_file_obj.len());

        // 2. Translate to each target language
        for target_lang in target_langs {
            tracing::info!("Translating to {}", target_lang);

            let mut translated = LanguageFile::new();

            for (key, value) in &source_file_obj {
                // Skip very short strings
                if value.len() < 2 {
                    translated.insert(key.clone(), value.clone());
                    continue;
                }

                // Translate
                match translator.translate(value, target_lang).await {
                    Ok(translated_text) => {
                        translated.insert(key.clone(), translated_text);
                        tracing::debug!("✓ {}: {}", key, value);
                    }
                    Err(e) => {
                        tracing::warn!("✗ Failed to translate {}: {}", key, e);
                        // Fallback to source
                        translated.insert(key.clone(), value.clone());
                    }
                }

                // Rate limiting: small delay to avoid API throttling
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }

            // 3. Write translated file
            let output_file = source_file
                .parent()
                .ok_or_else(|| anyhow::anyhow!(
                    "Cannot extract parent directory from path: {}",
                    source_file.display()
                ))?
                .join(format!("{}.json", target_lang));

            let json = serde_json::to_string_pretty(&translated.translations)?;
            std::fs::write(&output_file, json)?;

            tracing::info!("Written {}", output_file.display());
        }

        Ok(())
    }
}
