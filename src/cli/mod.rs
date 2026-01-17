pub mod commands;
pub mod presenter;

use clap::Parser;
use commands::Command;

/// 🌍 Langsmith - Automatic i18n extraction and translation CLI
#[derive(Parser, Debug)]
#[command(name = "langsmith")]
#[command(about = "Automatically extract and manage translations in your codebase", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub async fn execute(self) -> anyhow::Result<()> {
        match self.command {
            Command::Extract(cmd) => cmd.run().await,
            Command::Translate(cmd) => cmd.run().await,
        }
    }
}
