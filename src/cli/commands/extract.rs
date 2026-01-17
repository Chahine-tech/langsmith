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
        println!("Extracting strings from {:?}", self.source);
        println!("Output to {:?}", self.output);
        Ok(())
    }
}
