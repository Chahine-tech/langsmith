use clap::Parser;
use std::path::PathBuf;
use crate::domain::models::ReplacementStrategy;
use crate::application::replace_strings::ReplaceStringsUseCase;
use crate::infrastructure::*;

#[derive(Parser, Debug)]
pub struct ReplaceCmd {
    /// Source directory to scan for strings to replace
    #[arg(value_name = "PATH")]
    pub source: PathBuf,

    /// Translation file (e.g., i18n/fr.json)
    #[arg(short, long, value_name = "FILE")]
    pub translations: PathBuf,

    /// Replacement strategy (react-i18n, vue-i18n, generic)
    #[arg(short, long, default_value = "react-i18n")]
    pub strategy: String,

    /// Preview changes without writing files
    #[arg(long)]
    pub dry_run: bool,

    /// Replace in original files (destructive - replaces instead of creating .i18n.* files)
    #[arg(long)]
    pub in_place: bool,
}

impl ReplaceCmd {
    pub async fn run(self) -> anyhow::Result<()> {
        println!("🔄 Langsmith - String Replacement\n");
        println!("  Source: {:?}", self.source);
        println!("  Translations: {:?}", self.translations);
        println!("  Strategy: {}\n", self.strategy);

        if self.dry_run {
            println!("  ℹ DRY RUN MODE - No files will be modified\n");
        }

        if self.in_place {
            println!("  ⚠ IN-PLACE MODE - Original files will be modified\n");
        }

        let strategy = ReplacementStrategy::from_str(&self.strategy)?;

        let scanner = FileSystemScanner;
        let extractor = SwcStringExtractor;
        let replacer = RegexReplacer;
        let import_mgr = SimpleImportManager;

        ReplaceStringsUseCase::execute(
            &self.source,
            &self.translations,
            strategy,
            self.dry_run,
            self.in_place,
            &scanner,
            &extractor,
            &replacer,
            &import_mgr,
        ).await?;

        println!("\n✓ Replacement complete!");

        Ok(())
    }
}
