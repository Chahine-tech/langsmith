use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::time::Duration;

pub struct ProgressReporter {
    multi: MultiProgress,
}

impl Default for ProgressReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressReporter {
    pub fn new() -> Self {
        Self {
            multi: MultiProgress::new(),
        }
    }

    /// Create a progress bar for file scanning
    pub fn create_spinner(&self, msg: &str) -> ProgressBar {
        let pb = self.multi.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .expect("Invalid progress template"),
        );
        pb.set_message(msg.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }

    /// Create a progress bar for known-length operations
    #[allow(dead_code)]
    pub fn create_bar(&self, len: u64, msg: &str) -> ProgressBar {
        let pb = self.multi.add(ProgressBar::new(len));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
                .expect("Invalid progress template")
                .progress_chars("█▓▒░"),
        );
        pb.set_message(msg.to_string());
        pb
    }

    /// Finish with success message
    pub fn finish_with_success(&self, pb: &ProgressBar, msg: &str) {
        pb.finish_with_message(format!("{} {}", "✓".green(), msg));
    }

    /// Finish with error message
    #[allow(dead_code)]
    pub fn finish_with_error(&self, pb: &ProgressBar, msg: &str) {
        pb.finish_with_message(format!("{} {}", "✗".red(), msg));
    }
}

/// Summary statistics for extraction
#[allow(dead_code)]
pub struct ExtractionSummary {
    pub total_strings: usize,
    pub double_quotes: usize,
    pub single_quotes: usize,
    pub template_literals: usize,
    pub jsx_text: usize,
    pub html_attributes: usize,
}

impl ExtractionSummary {
    #[allow(dead_code)]
    pub fn print(&self) {
        println!("\n{}", "📊 Extraction Summary:".bold());
        println!(
            "  {} Total strings found",
            self.total_strings.to_string().green()
        );
        println!("    ├─ {} double quotes", self.double_quotes);
        println!("    ├─ {} single quotes", self.single_quotes);
        println!("    ├─ {} template literals", self.template_literals);
        println!("    ├─ {} JSX text nodes", self.jsx_text);
        println!("    └─ {} HTML attributes", self.html_attributes);
    }
}

/// Summary statistics for translation
#[allow(dead_code)]
pub struct TranslationSummary {
    pub total: usize,
    pub successful: usize,
    pub failed: usize,
    pub languages: Vec<String>,
}

impl TranslationSummary {
    #[allow(dead_code)]
    pub fn print(&self) {
        println!("\n{}", "🌍 Translation Summary:".bold());
        println!("  {} Successful", self.successful.to_string().green());
        if self.failed > 0 {
            println!("  {} Failed", self.failed.to_string().red());
        }
        println!("  Languages: {}", self.languages.join(", "));
    }
}

/// Summary statistics for replacement
#[allow(dead_code)]
pub struct ReplacementSummary {
    pub files_processed: usize,
    pub strings_replaced: usize,
}

impl ReplacementSummary {
    #[allow(dead_code)]
    pub fn print(&self) {
        println!("\n{}", "📝 Replacement Summary:".bold());
        println!(
            "  {} Files modified",
            self.files_processed.to_string().green()
        );
        println!(
            "  {} Strings replaced",
            self.strings_replaced.to_string().green()
        );
    }
}
