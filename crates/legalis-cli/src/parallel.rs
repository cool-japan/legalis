//! Parallel processing utilities for batch operations.

use crate::progress::ProgressTracker;
use anyhow::Result;
use rayon::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Result of a batch operation.
#[derive(Debug, Clone)]
pub struct BatchResult<T> {
    /// Successful results
    pub successes: Vec<T>,

    /// Failed operations with errors
    pub failures: Vec<(String, String)>, // (file_path, error_message)
}

impl<T> Default for BatchResult<T> {
    fn default() -> Self {
        Self {
            successes: Vec::new(),
            failures: Vec::new(),
        }
    }
}

impl<T> BatchResult<T> {
    /// Create a new empty batch result.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if all operations succeeded.
    pub fn all_succeeded(&self) -> bool {
        self.failures.is_empty()
    }

    /// Get the number of successful operations.
    pub fn success_count(&self) -> usize {
        self.successes.len()
    }

    /// Get the number of failed operations.
    pub fn failure_count(&self) -> usize {
        self.failures.len()
    }

    /// Get total number of operations.
    pub fn total_count(&self) -> usize {
        self.success_count() + self.failure_count()
    }
}

/// Parallel batch processor.
pub struct ParallelProcessor;

impl ParallelProcessor {
    /// Process multiple files in parallel with a given operation.
    pub fn process_files<F, T, P>(files: &[P], operation: F) -> BatchResult<T>
    where
        F: Fn(&Path) -> Result<T> + Send + Sync,
        T: Send,
        P: AsRef<Path> + Sync,
    {
        let successes = Arc::new(Mutex::new(Vec::new()));
        let failures = Arc::new(Mutex::new(Vec::new()));

        files.par_iter().for_each(|file| {
            let path = file.as_ref();
            match operation(path) {
                Ok(result) => {
                    if let Ok(mut s) = successes.lock() {
                        s.push(result);
                    }
                }
                Err(err) => {
                    if let Ok(mut f) = failures.lock() {
                        f.push((path.display().to_string(), err.to_string()));
                    }
                }
            }
        });

        BatchResult {
            successes: Arc::try_unwrap(successes)
                .map(|m| m.into_inner().unwrap_or_default())
                .unwrap_or_default(),
            failures: Arc::try_unwrap(failures)
                .map(|m| m.into_inner().unwrap_or_default())
                .unwrap_or_default(),
        }
    }

    /// Process files with a progress callback.
    pub fn process_files_with_progress<F, T, PF, PR>(
        files: &[PF],
        operation: F,
        progress: PR,
    ) -> BatchResult<T>
    where
        F: Fn(&Path) -> Result<T> + Send + Sync,
        T: Send,
        PF: AsRef<Path> + Sync,
        PR: Fn(usize, usize) + Send + Sync,
    {
        let successes = Arc::new(Mutex::new(Vec::new()));
        let failures = Arc::new(Mutex::new(Vec::new()));
        let completed = Arc::new(Mutex::new(0usize));
        let total = files.len();

        files.par_iter().for_each(|file| {
            let path = file.as_ref();
            match operation(path) {
                Ok(result) => {
                    if let Ok(mut s) = successes.lock() {
                        s.push(result);
                    }
                }
                Err(err) => {
                    if let Ok(mut f) = failures.lock() {
                        f.push((path.display().to_string(), err.to_string()));
                    }
                }
            }

            if let Ok(mut count) = completed.lock() {
                *count += 1;
                progress(*count, total);
            }
        });

        BatchResult {
            successes: Arc::try_unwrap(successes)
                .map(|m| m.into_inner().unwrap_or_default())
                .unwrap_or_default(),
            failures: Arc::try_unwrap(failures)
                .map(|m| m.into_inner().unwrap_or_default())
                .unwrap_or_default(),
        }
    }

    /// Process items in parallel chunks.
    pub fn process_in_chunks<F, T, I>(
        items: Vec<I>,
        chunk_size: usize,
        operation: F,
    ) -> BatchResult<T>
    where
        F: Fn(&[I]) -> Result<Vec<T>> + Send + Sync,
        T: Send,
        I: Send + Sync,
    {
        let successes = Arc::new(Mutex::new(Vec::new()));
        let failures = Arc::new(Mutex::new(Vec::new()));

        items
            .par_chunks(chunk_size)
            .enumerate()
            .for_each(|(chunk_idx, chunk)| match operation(chunk) {
                Ok(results) => {
                    if let Ok(mut s) = successes.lock() {
                        s.extend(results);
                    }
                }
                Err(err) => {
                    if let Ok(mut f) = failures.lock() {
                        f.push((format!("chunk-{}", chunk_idx), err.to_string()));
                    }
                }
            });

        BatchResult {
            successes: Arc::try_unwrap(successes)
                .map(|m| m.into_inner().unwrap_or_default())
                .unwrap_or_default(),
            failures: Arc::try_unwrap(failures)
                .map(|m| m.into_inner().unwrap_or_default())
                .unwrap_or_default(),
        }
    }

    /// Get the number of threads available for parallel processing.
    pub fn num_threads() -> usize {
        rayon::current_num_threads()
    }

    /// Set the number of threads to use for parallel processing.
    pub fn set_num_threads(num_threads: usize) -> Result<()> {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .map_err(|e| anyhow::anyhow!("Failed to set thread pool size: {}", e))
    }

    /// Process files with integrated progress tracking and ETA estimation.
    pub fn process_files_with_progress_tracker<F, T, P>(
        files: &[P],
        operation: F,
        progress_tracker: &ProgressTracker,
    ) -> BatchResult<T>
    where
        F: Fn(&Path) -> Result<T> + Send + Sync,
        T: Send,
        P: AsRef<Path> + Sync,
    {
        let successes = Arc::new(Mutex::new(Vec::new()));
        let failures = Arc::new(Mutex::new(Vec::new()));

        files.par_iter().for_each(|file| {
            let path = file.as_ref();
            match operation(path) {
                Ok(result) => {
                    if let Ok(mut s) = successes.lock() {
                        s.push(result);
                    }
                }
                Err(err) => {
                    if let Ok(mut f) = failures.lock() {
                        f.push((path.display().to_string(), err.to_string()));
                    }
                }
            }

            // Update progress
            progress_tracker.inc(1);
        });

        progress_tracker.finish();

        BatchResult {
            successes: Arc::try_unwrap(successes)
                .map(|m| m.into_inner().unwrap_or_default())
                .unwrap_or_default(),
            failures: Arc::try_unwrap(failures)
                .map(|m| m.into_inner().unwrap_or_default())
                .unwrap_or_default(),
        }
    }

    /// Process items in chunks with progress tracking.
    pub fn process_in_chunks_with_tracker<F, T, I>(
        items: Vec<I>,
        chunk_size: usize,
        operation: F,
        progress_tracker: &ProgressTracker,
    ) -> BatchResult<T>
    where
        F: Fn(&[I]) -> Result<Vec<T>> + Send + Sync,
        T: Send,
        I: Send + Sync,
    {
        let successes = Arc::new(Mutex::new(Vec::new()));
        let failures = Arc::new(Mutex::new(Vec::new()));

        items
            .par_chunks(chunk_size)
            .enumerate()
            .for_each(|(chunk_idx, chunk)| {
                match operation(chunk) {
                    Ok(results) => {
                        if let Ok(mut s) = successes.lock() {
                            s.extend(results);
                        }
                    }
                    Err(err) => {
                        if let Ok(mut f) = failures.lock() {
                            f.push((format!("chunk-{}", chunk_idx), err.to_string()));
                        }
                    }
                }

                progress_tracker.inc(1);
            });

        progress_tracker.finish();

        BatchResult {
            successes: Arc::try_unwrap(successes)
                .map(|m| m.into_inner().unwrap_or_default())
                .unwrap_or_default(),
            failures: Arc::try_unwrap(failures)
                .map(|m| m.into_inner().unwrap_or_default())
                .unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_batch_result() {
        let mut result = BatchResult::<String>::new();
        assert!(result.all_succeeded());
        assert_eq!(result.success_count(), 0);
        assert_eq!(result.failure_count(), 0);

        result.successes.push("success1".to_string());
        result.successes.push("success2".to_string());
        assert_eq!(result.success_count(), 2);
        assert!(result.all_succeeded());

        result
            .failures
            .push(("file1".to_string(), "error1".to_string()));
        assert!(!result.all_succeeded());
        assert_eq!(result.failure_count(), 1);
        assert_eq!(result.total_count(), 3);
    }

    #[test]
    fn test_process_files() {
        let files = vec![
            PathBuf::from("file1.txt"),
            PathBuf::from("file2.txt"),
            PathBuf::from("file3.txt"),
        ];

        let result = ParallelProcessor::process_files(&files, |path| {
            // Simulate processing - just return the file name
            Ok(path.file_name().unwrap().to_string_lossy().to_string())
        });

        assert_eq!(result.success_count(), 3);
        assert_eq!(result.failure_count(), 0);
        assert!(result.all_succeeded());
    }

    #[test]
    fn test_process_files_with_errors() {
        let files = vec![
            PathBuf::from("file1.txt"),
            PathBuf::from("error.txt"),
            PathBuf::from("file3.txt"),
        ];

        let result = ParallelProcessor::process_files(&files, |path| {
            if path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .contains("error")
            {
                anyhow::bail!("Error processing file");
            }
            Ok(path.file_name().unwrap().to_string_lossy().to_string())
        });

        assert_eq!(result.success_count(), 2);
        assert_eq!(result.failure_count(), 1);
        assert!(!result.all_succeeded());
    }

    #[test]
    fn test_num_threads() {
        let num_threads = ParallelProcessor::num_threads();
        assert!(num_threads > 0);
    }
}
