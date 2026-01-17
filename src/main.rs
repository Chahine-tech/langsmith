mod cli;
mod domain;
mod application;
mod infrastructure;

use cli::Cli;
use clap::Parser;

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
