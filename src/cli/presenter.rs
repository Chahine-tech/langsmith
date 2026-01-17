use owo_colors::OwoColorize;

/// Handles all CLI output formatting
#[allow(dead_code)]
pub struct Presenter;

#[allow(dead_code)]
impl Presenter {
    pub fn success(msg: impl AsRef<str>) {
        println!("{} {}", "✓".green(), msg.as_ref());
    }

    pub fn error(msg: impl AsRef<str>) {
        eprintln!("{} {}", "✗".red(), msg.as_ref());
    }

    pub fn info(msg: impl AsRef<str>) {
        println!("{} {}", "ℹ".cyan(), msg.as_ref());
    }

    pub fn header(msg: impl AsRef<str>) {
        println!("\n{}\n", msg.as_ref().bold().underline());
    }

    pub fn table_row(col1: &str, col2: &str) {
        println!("  {:<30} {}", col1.dimmed(), col2);
    }
}
