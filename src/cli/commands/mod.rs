pub mod extract;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Extract translatable strings from your codebase
    Extract(extract::ExtractCmd),
}
