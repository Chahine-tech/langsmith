use clap::Parser;
use std::path::PathBuf;
use crate::application::TranslateKeysUseCase;
use crate::infrastructure::{ConfigManager, DeepLTranslator, OpenAITranslator};
use crate::cli::presenter::Presenter;

#[derive(Parser, Debug)]
pub struct TranslateCmd {
    /// Path to source translation file (e.g., i18n/fr.json)
    #[arg(value_name = "FILE")]
    pub source: PathBuf,

    /// Target languages (comma-separated, e.g., en,es,de)
    #[arg(short, long, value_name = "LANGS")]
    pub to: String,

    /// Translation API provider (deepl or openai)
    #[arg(short, long, default_value = "deepl")]
    pub api: String,

    /// API key (overrides environment variable)
    #[arg(long, value_name = "KEY")]
    pub api_key: Option<String>,
}

impl TranslateCmd {
    pub async fn run(self) -> anyhow::Result<()> {
        Presenter::header("🌍 Langsmith - Translation");

        // Validate source file
        if !self.source.exists() {
            Presenter::error(format!("Source file not found: {:?}", self.source));
            return Err(anyhow::anyhow!("Source file not found"));
        }

        if !self.source.is_file() {
            Presenter::error(format!("Source is not a file: {:?}", self.source));
            return Err(anyhow::anyhow!("Source is not a file"));
        }

        Presenter::info(format!("Source: {:?}", self.source));
        Presenter::info(format!("Target languages: {}", self.to));
        Presenter::info(format!("API: {}", self.api));

        // Get API configuration
        let api_config = ConfigManager::get_api_config(&self.api, self.api_key.as_deref())?;

        // Parse target languages
        let target_langs: Vec<&str> = self.to.split(',').map(|s| s.trim()).collect();
        if target_langs.is_empty() {
            Presenter::error("No target languages specified");
            return Err(anyhow::anyhow!("No target languages specified"));
        }

        Presenter::info(format!("Translating to: {}", target_langs.join(", ")));

        // Execute translation based on provider
        match api_config.provider {
            crate::infrastructure::ApiProvider::DeepL => {
                let translator = DeepLTranslator::new(api_config.api_key);
                TranslateKeysUseCase::execute(&self.source, &target_langs, &translator).await?;
            }
            crate::infrastructure::ApiProvider::OpenAI => {
                let translator = OpenAITranslator::new(api_config.api_key);
                TranslateKeysUseCase::execute(&self.source, &target_langs, &translator).await?;
            }
        }

        Presenter::success("Translation complete!");
        Ok(())
    }
}
