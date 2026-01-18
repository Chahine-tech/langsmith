use dialoguer::{Select, Input, MultiSelect, Confirm};
use owo_colors::OwoColorize;
use std::path::PathBuf;
use anyhow::Result;

pub struct WizardConfig {
    pub framework: Framework,
    pub source_dir: PathBuf,
    pub output_dir: PathBuf,
    pub target_languages: Vec<String>,
    pub api_choice: TranslationApi,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Framework {
    React,
    Vue,
    Angular,
    Vanilla,
}

impl std::fmt::Display for Framework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Framework::React => write!(f, "React"),
            Framework::Vue => write!(f, "Vue"),
            Framework::Angular => write!(f, "Angular"),
            Framework::Vanilla => write!(f, "Vanilla JS"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TranslationApi {
    DeepL,
    OpenAI,
    Skip,
}

impl std::fmt::Display for TranslationApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranslationApi::DeepL => write!(f, "DeepL"),
            TranslationApi::OpenAI => write!(f, "OpenAI"),
            TranslationApi::Skip => write!(f, "Skip"),
        }
    }
}

pub struct Wizard;

impl Wizard {
    /// Run the interactive setup wizard
    pub async fn run() -> Result<WizardConfig> {
        Self::print_welcome();

        let framework = Self::ask_framework()?;
        let source_dir = Self::ask_source_dir()?;
        let output_dir = Self::ask_output_dir()?;
        let target_languages = Self::ask_target_languages()?;
        let api_choice = Self::ask_api()?;
        let api_key = Self::ask_api_key(&api_choice)?;

        let config = WizardConfig {
            framework,
            source_dir,
            output_dir,
            target_languages,
            api_choice,
            api_key,
        };

        Self::confirm_and_execute(config).await
    }

    fn print_welcome() {
        println!("\n{}\n", "🧙 Langsmith Interactive Setup".bold().underline());
        println!("This wizard will guide you through setting up i18n for your project.\n");
    }

    fn ask_framework() -> Result<Framework> {
        let frameworks = vec!["React", "Vue", "Angular", "Vanilla JS"];

        let selection = Select::new()
            .with_prompt("What framework are you using?")
            .items(&frameworks)
            .default(0)
            .interact()?;

        let framework = match selection {
            0 => Framework::React,
            1 => Framework::Vue,
            2 => Framework::Angular,
            3 => Framework::Vanilla,
            _ => unreachable!(),
        };

        Ok(framework)
    }

    fn ask_source_dir() -> Result<PathBuf> {
        let dir: String = Input::new()
            .with_prompt("Where are your source files?")
            .default("./src".to_string())
            .interact_text()?;

        Ok(PathBuf::from(dir))
    }

    fn ask_output_dir() -> Result<PathBuf> {
        let dir: String = Input::new()
            .with_prompt("Where should translation files be saved?")
            .default("./i18n".to_string())
            .interact_text()?;

        Ok(PathBuf::from(dir))
    }

    fn ask_target_languages() -> Result<Vec<String>> {
        let languages = vec![
            "English (en)",
            "French (fr)",
            "Spanish (es)",
            "German (de)",
            "Italian (it)",
            "Japanese (ja)",
            "Chinese (zh)",
        ];

        let selections = MultiSelect::new()
            .with_prompt("Select target languages (Space to select, Enter to confirm)")
            .items(&languages)
            .interact()?;

        let selected_langs: Vec<String> = selections
            .iter()
            .map(|&i| {
                // Extract language code from "Language (code)"
                languages[i]
                    .split('(')
                    .nth(1)
                    .and_then(|s| s.split(')').next())
                    .unwrap_or("en")
                    .to_string()
            })
            .collect();

        Ok(selected_langs)
    }

    fn ask_api() -> Result<TranslationApi> {
        let apis = vec![
            "DeepL (recommended for quality)",
            "OpenAI (GPT-4)",
            "Skip (translate manually later)",
        ];

        let selection = Select::new()
            .with_prompt("How do you want to translate?")
            .items(&apis)
            .default(0)
            .interact()?;

        let api = match selection {
            0 => TranslationApi::DeepL,
            1 => TranslationApi::OpenAI,
            2 => TranslationApi::Skip,
            _ => unreachable!(),
        };

        Ok(api)
    }

    fn ask_api_key(api: &TranslationApi) -> Result<Option<String>> {
        match api {
            TranslationApi::Skip => Ok(None),
            TranslationApi::DeepL => {
                let key: String = Input::new()
                    .with_prompt("Enter your DeepL API key")
                    .interact_text()?;
                Ok(Some(key))
            }
            TranslationApi::OpenAI => {
                let key: String = Input::new()
                    .with_prompt("Enter your OpenAI API key")
                    .interact_text()?;
                Ok(Some(key))
            }
        }
    }

    async fn confirm_and_execute(config: WizardConfig) -> Result<WizardConfig> {
        // Show summary
        println!("\n{}", "📋 Configuration Summary:".bold());
        println!("  Framework: {}", config.framework);
        println!("  Source: {}", config.source_dir.display());
        println!("  Output: {}", config.output_dir.display());
        println!("  Languages: {}", config.target_languages.join(", "));
        println!("  API: {}\n", config.api_choice);

        let proceed = Confirm::new()
            .with_prompt("Proceed with this configuration?")
            .default(true)
            .interact()?;

        if !proceed {
            anyhow::bail!("Setup cancelled by user");
        }

        Ok(config)
    }
}
