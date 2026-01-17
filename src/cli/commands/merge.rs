use clap::Parser;
use std::path::PathBuf;
use crate::application::merge_i18n::MergeI18nUseCase;
use owo_colors::OwoColorize;

#[derive(Parser, Debug)]
pub struct MergeCmd {
    /// Directory to scan for .i18n.* files
    #[arg(value_name = "PATH")]
    pub directory: PathBuf,

    /// Actually perform the merge (without this, only shows preview)
    #[arg(long)]
    pub confirm: bool,
}

impl MergeCmd {
    pub async fn run(self) -> anyhow::Result<()> {
        println!("\n{}", "🔀 Langsmith - Merge i18n Files".bold());
        println!("  Directory: {:?}\n", self.directory);

        if !self.directory.exists() {
            eprintln!("{} Directory not found: {:?}", "✗".red(), self.directory);
            anyhow::bail!("Directory does not exist");
        }

        // Scan for .i18n.* files
        println!("  {} Scanning for .i18n.* files...", "⏳".cyan());
        let summary = MergeI18nUseCase::scan(&self.directory).await?;

        if summary.files_to_merge.is_empty() {
            println!("\n  {} No .i18n.* files found to merge.", "ℹ".cyan());
            return Ok(());
        }

        // Show preview
        if !self.confirm {
            println!("\n  {} Preview Mode - No files will be modified", "ℹ".cyan());
            MergeI18nUseCase::show_preview(&summary);
            return Ok(());
        }

        // Actually perform merge
        println!("\n  {} Merging {} files...", "⏳".cyan(), summary.files_to_merge.len());
        let result = MergeI18nUseCase::execute_merge(&summary).await?;

        // Show results
        println!("\n  {} Merge Complete!", "✓".green());
        println!("    {} files merged successfully", result.successful.to_string().green());
        
        if result.failed > 0 {
            println!("    {} files failed", result.failed.to_string().red());
            println!("\n  Errors:");
            for (file, error) in &result.errors {
                println!("    {} {}: {}", "✗".red(), file.display(), error);
            }
            anyhow::bail!("Some files failed to merge");
        }

        println!();

        Ok(())
    }
}
