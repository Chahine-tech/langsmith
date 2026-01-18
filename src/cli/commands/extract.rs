use crate::application::ExtractStringsUseCase;
use crate::cli::presenter::Presenter;
use crate::infrastructure::{FileSystemScanner, FileSystemWriter, SwcStringExtractor};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct ExtractCmd {
    /// Source directory to scan for translatable strings
    #[arg(value_name = "PATH")]
    pub source: PathBuf,

    /// Output directory for translation files
    #[arg(short, long, value_name = "PATH", default_value = "./i18n")]
    pub output: PathBuf,

    /// Base language code
    #[arg(short, long, default_value = "fr")]
    pub lang: String,
}

impl ExtractCmd {
    pub async fn run(self) -> anyhow::Result<()> {
        Presenter::header("🌍 Langsmith - String Extraction");

        // Validate paths
        if !self.source.exists() {
            Presenter::error(format!("Source directory not found: {:?}", self.source));
            return Err(anyhow::anyhow!("Source directory not found"));
        }

        if !self.source.is_dir() {
            Presenter::error(format!("Source is not a directory: {:?}", self.source));
            return Err(anyhow::anyhow!("Source is not a directory"));
        }

        Presenter::info(format!("Scanning: {:?}", self.source));
        Presenter::info(format!("Output: {:?}", self.output));
        Presenter::info(format!("Base language: {}", self.lang));

        // Initialize infrastructure
        let scanner = FileSystemScanner;
        let extractor = SwcStringExtractor;
        let writer = FileSystemWriter;

        // Execute use case
        ExtractStringsUseCase::execute(
            &self.source,
            &self.output,
            &self.lang,
            &scanner,
            &extractor,
            &writer,
        )
        .await?;

        Presenter::success("Extraction complete!");
        Ok(())
    }
}
