pub mod extract;
pub mod translate;
pub mod replace;
pub mod merge;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Extract translatable strings from your codebase
    Extract(extract::ExtractCmd),
    /// Translate extracted strings to target languages
    Translate(translate::TranslateCmd),
    /// Replace hardcoded strings with translation function calls
    Replace(replace::ReplaceCmd),
    /// Merge .i18n.* files back to original files
    Merge(merge::MergeCmd),
}
