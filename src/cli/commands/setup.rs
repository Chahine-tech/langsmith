use crate::cli::commands::{extract, replace, translate};
use crate::cli::progress::ProgressReporter;
use crate::cli::wizard::{Framework, TranslationApi, Wizard};
use anyhow::Result;
use clap::Parser;
use owo_colors::OwoColorize;

#[derive(Parser, Debug)]
pub struct SetupCmd {
    /// Skip interactive mode and use defaults
    #[arg(long)]
    pub non_interactive: bool,
}

impl SetupCmd {
    pub async fn run(self) -> Result<()> {
        if self.non_interactive {
            return Err(anyhow::anyhow!("Non-interactive mode not yet implemented"));
        }

        // Run wizard
        let config = Wizard::run().await?;

        let reporter = ProgressReporter::new();

        // Step 1: Extract strings
        println!("\n{}", "Step 1/4: Extracting strings...".bold());
        let extract_spinner = reporter.create_spinner("Scanning files");

        let extract_cmd = extract::ExtractCmd {
            source: config.source_dir.clone(),
            output: config.output_dir.clone(),
            lang: "fr".to_string(),
        };

        extract_cmd.run().await?;
        reporter.finish_with_success(&extract_spinner, "Extraction complete");

        // Step 2: Translate (if API chosen)
        if !matches!(config.api_choice, TranslationApi::Skip) {
            println!("\n{}", "Step 2/4: Translating...".bold());

            let to_langs = config.target_languages.join(",");
            let source_file = config.output_dir.join("fr.json");

            let translate_cmd = translate::TranslateCmd {
                source: source_file,
                to: to_langs,
                api: match config.api_choice {
                    TranslationApi::DeepL => "deepl".to_string(),
                    TranslationApi::OpenAI => "openai".to_string(),
                    _ => "deepl".to_string(),
                },
                api_key: config.api_key.clone(),
            };

            let translate_spinner = reporter.create_spinner("Translating to target languages");
            translate_cmd.run().await?;
            reporter.finish_with_success(&translate_spinner, "Translation complete");
        } else {
            println!(
                "\n{}",
                "Step 2/4: Skipping translation (manual mode)".dimmed()
            );
        }

        // Step 3: Replace strings
        println!("\n{}", "Step 3/4: Replacing strings...".bold());
        let replace_spinner = reporter.create_spinner("Generating i18n files");

        let strategy = match config.framework {
            Framework::React => "react-i18n",
            Framework::Vue => "vue-i18n",
            _ => "generic",
        };

        let translation_file = config.output_dir.join("fr.json");

        let replace_cmd = replace::ReplaceCmd {
            source: config.source_dir.clone(),
            translations: translation_file,
            strategy: strategy.to_string(),
            dry_run: false,
            in_place: false,
        };

        replace_cmd.run().await?;
        reporter.finish_with_success(&replace_spinner, "String replacement complete");

        // Step 4: Summary
        println!("\n{}", "✅ Setup Complete!".green().bold());
        println!("\n{}", "Next steps:".bold());
        println!("  1. Review generated .i18n.* files");
        println!("  2. Test your application");
        println!("  3. Run `langsmith merge` to replace original files (optional)");
        println!("\n{}", "Happy internationalizing! 🌍".cyan());

        Ok(())
    }
}
