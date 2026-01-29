//! Batch operations for processing multiple statutes in parallel.

use anyhow::{Context, Result};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Journal entry for tracking batch operation progress.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub file_path: String,
    pub status: EntryStatus,
    pub timestamp: u64,
    pub error: Option<String>,
}

/// Status of a journal entry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntryStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

/// Batch operation journal for tracking progress and enabling resume.
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchJournal {
    pub operation_type: String,
    pub started_at: u64,
    pub completed_at: Option<u64>,
    pub entries: HashMap<String, JournalEntry>,
    pub total_files: usize,
    pub completed_files: usize,
    pub failed_files: usize,
}

impl BatchJournal {
    /// Create a new batch journal.
    pub fn new(operation_type: String, files: &[PathBuf]) -> Self {
        let mut entries = HashMap::new();
        let started_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("System time is before UNIX_EPOCH")
            .as_secs();

        for file in files {
            let path_str = file.to_string_lossy().to_string();
            entries.insert(
                path_str.clone(),
                JournalEntry {
                    file_path: path_str,
                    status: EntryStatus::Pending,
                    timestamp: started_at,
                    error: None,
                },
            );
        }

        Self {
            operation_type,
            started_at,
            completed_at: None,
            total_files: files.len(),
            completed_files: 0,
            failed_files: 0,
            entries,
        }
    }

    /// Load journal from file.
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read journal file: {}", path.display()))?;
        let journal: BatchJournal =
            serde_json::from_str(&content).with_context(|| "Failed to parse journal file")?;
        Ok(journal)
    }

    /// Save journal to file.
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)
            .with_context(|| format!("Failed to write journal file: {}", path.display()))?;
        Ok(())
    }

    /// Mark an entry as completed.
    pub fn mark_completed(&mut self, file_path: &str) {
        if let Some(entry) = self.entries.get_mut(file_path) {
            entry.status = EntryStatus::Completed;
            entry.timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("System time is before UNIX_EPOCH")
                .as_secs();
            self.completed_files += 1;
        }
    }

    /// Mark an entry as failed.
    pub fn mark_failed(&mut self, file_path: &str, error: String) {
        if let Some(entry) = self.entries.get_mut(file_path) {
            entry.status = EntryStatus::Failed;
            entry.error = Some(error);
            entry.timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("System time is before UNIX_EPOCH")
                .as_secs();
            self.failed_files += 1;
        }
    }

    /// Mark an entry as in progress.
    pub fn mark_in_progress(&mut self, file_path: &str) {
        if let Some(entry) = self.entries.get_mut(file_path) {
            entry.status = EntryStatus::InProgress;
            entry.timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("System time is before UNIX_EPOCH")
                .as_secs();
        }
    }

    /// Get pending files.
    pub fn get_pending_files(&self) -> Vec<String> {
        self.entries
            .values()
            .filter(|e| e.status == EntryStatus::Pending || e.status == EntryStatus::InProgress)
            .map(|e| e.file_path.clone())
            .collect()
    }

    /// Complete the journal.
    pub fn complete(&mut self) {
        self.completed_at = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("System time is before UNIX_EPOCH")
                .as_secs(),
        );
    }

    /// Get completion percentage.
    pub fn completion_percentage(&self) -> f64 {
        if self.total_files == 0 {
            return 100.0;
        }
        (self.completed_files as f64 / self.total_files as f64) * 100.0
    }
}

/// Batch processor for parallel operations.
pub struct BatchProcessor {
    workers: usize,
    multi_progress: MultiProgress,
}

impl BatchProcessor {
    /// Create a new batch processor.
    pub fn new(workers: Option<usize>) -> Self {
        let workers = workers.unwrap_or_else(num_cpus::get);
        Self {
            workers,
            multi_progress: MultiProgress::new(),
        }
    }

    /// Process files in parallel with progress tracking and journaling.
    pub async fn process<F, T>(
        &self,
        files: Vec<PathBuf>,
        journal_path: &Path,
        resume: bool,
        operation_name: &str,
        processor: F,
    ) -> Result<Vec<(PathBuf, Result<T>)>>
    where
        F: Fn(PathBuf) -> Result<T> + Send + Sync + 'static,
        T: Send + std::fmt::Debug + 'static,
    {
        // Load or create journal
        let journal = if resume && journal_path.exists() {
            println!("Resuming from previous run...");
            BatchJournal::load(journal_path)?
        } else {
            BatchJournal::new(operation_name.to_string(), &files)
        };

        // Get files to process
        let files_to_process: Vec<PathBuf> = if resume {
            let pending = journal.get_pending_files();
            files
                .into_iter()
                .filter(|f| pending.contains(&f.to_string_lossy().to_string()))
                .collect()
        } else {
            files
        };

        if files_to_process.is_empty() {
            println!("No files to process.");
            return Ok(Vec::new());
        }

        // Create progress bar
        let pb = self
            .multi_progress
            .add(ProgressBar::new(files_to_process.len() as u64));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg} (ETA: {eta})")
                .expect("Failed to create progress bar template")
                .progress_chars("=>-"),
        );

        // Create results container
        let results = Arc::new(Mutex::new(Vec::new()));
        let journal_arc = Arc::new(Mutex::new(journal));
        let pb_clone = pb.clone();

        // Create tasks
        let mut tasks = Vec::new();
        let processor = Arc::new(processor);
        let start_time = Instant::now();

        for file in files_to_process {
            let file_clone = file.clone();
            let results_clone = Arc::clone(&results);
            let journal_clone = Arc::clone(&journal_arc);
            let pb_task = pb_clone.clone();
            let processor_clone = Arc::clone(&processor);
            let journal_path_clone = journal_path.to_path_buf();

            let task = tokio::spawn(async move {
                let file_str = file_clone.to_string_lossy().to_string();

                // Mark as in progress
                {
                    let mut j = journal_clone
                        .lock()
                        .expect("Failed to acquire journal lock");
                    j.mark_in_progress(&file_str);
                    let _ = j.save(&journal_path_clone);
                }

                // Process file
                let result = processor_clone(file_clone.clone());

                // Update journal
                {
                    let mut j = journal_clone
                        .lock()
                        .expect("Failed to acquire journal lock");
                    match &result {
                        Ok(_) => j.mark_completed(&file_str),
                        Err(e) => j.mark_failed(&file_str, e.to_string()),
                    }
                    let _ = j.save(&journal_path_clone);
                }

                // Store result
                {
                    let mut r = results_clone
                        .lock()
                        .expect("Failed to acquire results lock");
                    r.push((file_clone.clone(), result));
                }

                // Update progress
                pb_task.inc(1);
                pb_task.set_message(format!("Processing {}", file_clone.display()));
            });

            tasks.push(task);

            // Limit concurrent tasks
            if tasks.len() >= self.workers
                && let Some(task) = tasks.pop()
            {
                let _ = task.await;
            }
        }

        // Wait for remaining tasks
        for task in tasks {
            let _ = task.await;
        }

        // Complete journal
        {
            let mut j = journal_arc.lock().expect("Failed to acquire journal lock");
            j.complete();
            j.save(journal_path)?;
        }

        pb.finish_with_message("Complete");

        let elapsed = start_time.elapsed();
        println!("\nBatch operation completed in {:?}", elapsed);

        let final_results = Arc::try_unwrap(results)
            .map_err(|_| anyhow::anyhow!("Failed to unwrap Arc"))?
            .into_inner()
            .map_err(|e| anyhow::anyhow!("Failed to acquire mutex: {:?}", e))?;

        Ok(final_results)
    }

    /// Create a progress bar for ETA tracking.
    pub fn create_progress_bar(&self, total: u64, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new(total));
        pb.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "[{{elapsed_precise}}] {{bar:40.cyan/blue}} {{pos}}/{{len}} {} (ETA: {{eta}})",
                    message
                ))
                .expect("Failed to create progress bar template")
                .progress_chars("=>-"),
        );
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }
}

/// Expand glob pattern to file paths.
pub fn expand_glob_pattern(pattern: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    // Check if pattern is a directory
    let path = Path::new(pattern);
    if path.is_dir() {
        // Search for .ldsl files in directory
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();
            if file_path.is_file()
                && let Some(ext) = file_path.extension()
                && (ext == "ldsl" || ext == "leg")
            {
                files.push(file_path);
            }
        }
    } else {
        // Try to parse as glob pattern
        for entry in
            glob::glob(pattern).with_context(|| format!("Invalid glob pattern: {}", pattern))?
        {
            match entry {
                Ok(path) => {
                    if path.is_file() {
                        files.push(path);
                    }
                }
                Err(e) => eprintln!("Warning: Failed to read path: {}", e),
            }
        }
    }

    if files.is_empty() {
        anyhow::bail!("No files found matching pattern: {}", pattern);
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journal_creation() {
        let files = vec![PathBuf::from("test1.ldsl"), PathBuf::from("test2.ldsl")];
        let journal = BatchJournal::new("test_op".to_string(), &files);

        assert_eq!(journal.total_files, 2);
        assert_eq!(journal.completed_files, 0);
        assert_eq!(journal.entries.len(), 2);
    }

    #[test]
    fn test_journal_mark_completed() {
        let files = vec![PathBuf::from("test.ldsl")];
        let mut journal = BatchJournal::new("test_op".to_string(), &files);

        journal.mark_completed("test.ldsl");
        assert_eq!(journal.completed_files, 1);
        assert_eq!(
            journal.entries.get("test.ldsl").unwrap().status,
            EntryStatus::Completed
        );
    }

    #[test]
    fn test_journal_completion_percentage() {
        let files = vec![PathBuf::from("test1.ldsl"), PathBuf::from("test2.ldsl")];
        let mut journal = BatchJournal::new("test_op".to_string(), &files);

        assert_eq!(journal.completion_percentage(), 0.0);

        journal.mark_completed("test1.ldsl");
        assert_eq!(journal.completion_percentage(), 50.0);

        journal.mark_completed("test2.ldsl");
        assert_eq!(journal.completion_percentage(), 100.0);
    }
}
