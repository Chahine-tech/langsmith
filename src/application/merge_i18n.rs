use std::path::{Path, PathBuf};
use tokio::fs;
use std::collections::HashMap;
use owo_colors::OwoColorize;

pub struct MergeI18nUseCase;

#[derive(Debug, Clone)]
pub struct MergeFile {
    pub i18n_file: PathBuf,
    pub original_file: PathBuf,
    pub file_size: u64,
}

pub struct MergeSummary {
    pub files_to_merge: Vec<MergeFile>,
    pub total_files: usize,
}

impl MergeI18nUseCase {
    /// Scans for .i18n.* files and prepares merge plan
    pub async fn scan(directory: &Path) -> anyhow::Result<MergeSummary> {
        let mut i18n_files = HashMap::new();
        let mut files_to_merge = Vec::new();

        // Walk through directory to find all .i18n.* files
        for entry in walkdir::WalkDir::new(directory)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if let Some(file_name) = path.file_name() {
                let file_name_str = file_name.to_string_lossy();
                if file_name_str.contains(".i18n.") {
                    i18n_files.insert(path.to_path_buf(), true);
                }
            }
        }

        // For each .i18n.* file, find the corresponding original
        for i18n_path in i18n_files.keys() {
            let original_path = Self::get_original_path(i18n_path);
            
            // Check if original file exists
            if original_path.exists() {
                let metadata = fs::metadata(&i18n_path).await?;
                files_to_merge.push(MergeFile {
                    i18n_file: i18n_path.clone(),
                    original_file: original_path,
                    file_size: metadata.len(),
                });
            }
        }

        Ok(MergeSummary {
            total_files: files_to_merge.len(),
            files_to_merge,
        })
    }

    /// Displays a preview of changes to be merged
    pub fn show_preview(summary: &MergeSummary) {
        println!("\n  Summary: {} .i18n.* files ready to merge\n", summary.total_files);

        if summary.files_to_merge.is_empty() {
            println!("  No .i18n.* files found to merge.");
            return;
        }

        println!("  Files to merge:");
        for (idx, merge_file) in summary.files_to_merge.iter().enumerate() {
            let i18n_name = merge_file.i18n_file.file_name().unwrap().to_string_lossy();
            let original_name = merge_file.original_file.file_name().unwrap().to_string_lossy();
            let size_kb = merge_file.file_size as f64 / 1024.0;
            
            println!("  {}. {} ({:.1} KB)", idx + 1, i18n_name, size_kb);
            println!("     {} {}", "→".dimmed(), original_name.bold());
        }

        println!("\n  Run with --confirm to merge these files.");
    }

    /// Performs the actual merge operation
    pub async fn execute_merge(summary: &MergeSummary) -> anyhow::Result<MergeResult> {
        let mut successful = 0;
        let mut failed = 0;
        let mut errors = Vec::new();

        for merge_file in &summary.files_to_merge {
            match Self::merge_single_file(merge_file).await {
                Ok(_) => {
                    successful += 1;
                }
                Err(e) => {
                    failed += 1;
                    errors.push((
                        merge_file.i18n_file.clone(),
                        e.to_string(),
                    ));
                }
            }
        }

        Ok(MergeResult {
            successful,
            failed,
            errors,
        })
    }

    /// Merges a single .i18n.* file with its original
    async fn merge_single_file(merge_file: &MergeFile) -> anyhow::Result<()> {
        // Read content from .i18n.* file
        let i18n_content = fs::read(&merge_file.i18n_file).await?;

        // Write to original file location
        fs::write(&merge_file.original_file, i18n_content).await?;

        // Remove .i18n.* file
        fs::remove_file(&merge_file.i18n_file).await?;

        tracing::info!(
            "Merged {} -> {}",
            merge_file.i18n_file.display(),
            merge_file.original_file.display()
        );

        Ok(())
    }

    /// Removes .i18n.* suffix from filename
    fn get_original_path(i18n_path: &Path) -> PathBuf {
        let file_name = i18n_path.file_name().unwrap().to_string_lossy();
        
        // Find .i18n. and remove it with the extension after
        if let Some(i18n_index) = file_name.find(".i18n.") {
            let base_name = &file_name[..i18n_index];
            let ext_start = i18n_index + 6; // ".i18n." length
            let extension = &file_name[ext_start..];
            
            let parent = i18n_path.parent().unwrap();
            parent.join(format!("{}.{}", base_name, extension))
        } else {
            i18n_path.to_path_buf()
        }
    }
}

pub struct MergeResult {
    pub successful: usize,
    pub failed: usize,
    pub errors: Vec<(PathBuf, String)>,
}
