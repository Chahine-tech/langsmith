mod application;
mod cli;
mod domain;
mod infrastructure;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("langsmith=debug".parse()?),
        )
        .init();

    let cli = Cli::parse();
    cli.execute().await
}
