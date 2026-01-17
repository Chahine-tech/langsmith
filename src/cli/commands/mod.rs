pub mod extract;
pub mod translate;
pub mod replace;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Extract translatable strings from your codebase
    Extract(extract::ExtractCmd),
    /// Translate extracted strings to target languages
    Translate(translate::TranslateCmd),
    /// Replace hardcoded strings with translation function calls
    Replace(replace::ReplaceCmd),
}
