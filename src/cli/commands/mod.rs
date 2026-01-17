pub mod extract;
pub mod translate;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Extract translatable strings from your codebase
    Extract(extract::ExtractCmd),
    /// Translate extracted strings to target languages
    Translate(translate::TranslateCmd),
}
