//! Watch mode for continuous validation of legal documents.
//!
//! This module provides file watching and automatic validation capabilities
//! for DSL files during development.

use crate::validation::SemanticValidator;
use crate::{DslError, DslResult, LegalDslParser};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// Configuration for watch mode.
#[derive(Debug, Clone)]
pub struct WatchConfig {
    /// Paths to watch for changes
    pub paths: Vec<PathBuf>,
    /// Polling interval in milliseconds
    pub poll_interval_ms: u64,
    /// Whether to validate on startup
    pub validate_on_start: bool,
    /// Whether to show detailed validation output
    pub verbose: bool,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            paths: Vec::new(),
            poll_interval_ms: 1000,
            validate_on_start: true,
            verbose: false,
        }
    }
}

impl WatchConfig {
    /// Creates a new watch configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a path to watch.
    pub fn add_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.paths.push(path.into());
        self
    }

    /// Sets the polling interval.
    pub fn poll_interval(mut self, ms: u64) -> Self {
        self.poll_interval_ms = ms;
        self
    }

    /// Enables or disables validation on startup.
    pub fn validate_on_start(mut self, enabled: bool) -> Self {
        self.validate_on_start = enabled;
        self
    }

    /// Enables or disables verbose output.
    pub fn verbose(mut self, enabled: bool) -> Self {
        self.verbose = enabled;
        self
    }
}

/// File watcher that monitors DSL files for changes and validates them.
pub struct FileWatcher {
    config: WatchConfig,
    file_states: HashMap<PathBuf, SystemTime>,
    parser: LegalDslParser,
    validator: SemanticValidator,
}

impl FileWatcher {
    /// Creates a new file watcher with the given configuration.
    pub fn new(config: WatchConfig) -> Self {
        Self {
            config,
            file_states: HashMap::new(),
            parser: LegalDslParser::new(),
            validator: SemanticValidator::new(),
        }
    }

    /// Initializes the file states by scanning all watched paths.
    pub fn initialize(&mut self) -> DslResult<()> {
        for path in &self.config.paths.clone() {
            if path.is_file() {
                if let Ok(metadata) = fs::metadata(path) {
                    if let Ok(modified) = metadata.modified() {
                        self.file_states.insert(path.clone(), modified);
                    }
                }
            } else if path.is_dir() {
                self.scan_directory(path)?;
            }
        }
        Ok(())
    }

    /// Scans a directory for DSL files.
    fn scan_directory(&mut self, dir: &Path) -> DslResult<()> {
        let entries = fs::read_dir(dir).map_err(|e| {
            DslError::parse_error(format!("Failed to read directory {:?}: {}", dir, e))
        })?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "dsl" || ext == "legalis" {
                        if let Ok(metadata) = fs::metadata(&path) {
                            if let Ok(modified) = metadata.modified() {
                                self.file_states.insert(path, modified);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Validates a single file.
    fn validate_file(&self, path: &Path) -> DslResult<()> {
        let content = fs::read_to_string(path)
            .map_err(|e| DslError::parse_error(format!("Failed to read file {:?}: {}", path, e)))?;

        let doc = self.parser.parse_document(&content)?;
        if let Err(errors) = self.validator.validate_document(&doc) {
            let error_msg = errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("; ");
            return Err(DslError::parse_error(error_msg));
        }

        if self.config.verbose {
            println!("✓ {} validated successfully", path.display());
        }

        Ok(())
    }

    /// Checks for file changes and validates changed files.
    pub fn check_for_changes(&mut self) -> Vec<ValidationResult> {
        let mut results = Vec::new();

        // Check each watched path
        for path in &self.config.paths.clone() {
            if path.is_file() {
                if let Some(result) = self.check_file_changed(path) {
                    results.push(result);
                }
            } else if path.is_dir() {
                // Rescan directory for new files
                let _ = self.scan_directory(path);

                // Check all known files in directory
                let files: Vec<_> = self
                    .file_states
                    .keys()
                    .filter(|p| p.starts_with(path))
                    .cloned()
                    .collect();

                for file_path in files {
                    if let Some(result) = self.check_file_changed(&file_path) {
                        results.push(result);
                    }
                }
            }
        }

        results
    }

    /// Checks if a specific file has changed and validates it if so.
    fn check_file_changed(&mut self, path: &Path) -> Option<ValidationResult> {
        if let Ok(metadata) = fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                let is_new = !self.file_states.contains_key(path);
                let has_changed = self
                    .file_states
                    .get(path)
                    .map(|last_modified| modified > *last_modified)
                    .unwrap_or(true);

                if is_new || has_changed {
                    self.file_states.insert(path.to_path_buf(), modified);

                    let result = match self.validate_file(path) {
                        Ok(()) => ValidationResult {
                            path: path.to_path_buf(),
                            success: true,
                            errors: Vec::new(),
                        },
                        Err(e) => ValidationResult {
                            path: path.to_path_buf(),
                            success: false,
                            errors: vec![e.to_string()],
                        },
                    };

                    return Some(result);
                }
            }
        }
        None
    }

    /// Runs the watch loop, continuously checking for changes.
    /// This is a blocking operation that runs until interrupted.
    pub fn watch(&mut self) -> DslResult<()> {
        if self.config.validate_on_start {
            println!("Validating files on startup...");
            for path in &self.config.paths.clone() {
                if path.is_file() {
                    if let Err(e) = self.validate_file(path) {
                        eprintln!("✗ {}: {}", path.display(), e);
                    }
                }
            }
        }

        println!("Watching for changes... (Press Ctrl+C to stop)");

        loop {
            let results = self.check_for_changes();

            for result in results {
                if result.success {
                    println!("✓ {}", result.path.display());
                } else {
                    eprintln!("✗ {}:", result.path.display());
                    for error in &result.errors {
                        eprintln!("  {}", error);
                    }
                }
            }

            std::thread::sleep(Duration::from_millis(self.config.poll_interval_ms));
        }
    }
}

/// Result of validating a single file.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Path to the validated file
    pub path: PathBuf,
    /// Whether validation succeeded
    pub success: bool,
    /// List of error messages if validation failed
    pub errors: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_watch_config() {
        let config = WatchConfig::new()
            .add_path("/tmp/test.dsl")
            .poll_interval(500)
            .validate_on_start(false)
            .verbose(true);

        assert_eq!(config.paths.len(), 1);
        assert_eq!(config.poll_interval_ms, 500);
        assert!(!config.validate_on_start);
        assert!(config.verbose);
    }

    #[test]
    fn test_file_watcher_initialize() {
        let config = WatchConfig::new();
        let mut watcher = FileWatcher::new(config);

        // Should not error even with no paths
        assert!(watcher.initialize().is_ok());
    }

    #[test]
    fn test_validate_file_success() -> std::io::Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "STATUTE test-1: \"Test\" {{")?;
        writeln!(temp_file, "    WHEN AGE >= 18")?;
        writeln!(temp_file, "    THEN GRANT \"benefit\"")?;
        writeln!(temp_file, "}}")?;
        temp_file.flush()?;

        let config = WatchConfig::new();
        let watcher = FileWatcher::new(config);

        let result = watcher.validate_file(temp_file.path());
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_validate_file_error() -> std::io::Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        // Write invalid syntax that will definitely fail parsing
        writeln!(temp_file, "STATUTE {{")?; // Missing ID and title
        temp_file.flush()?;

        let config = WatchConfig::new();
        let watcher = FileWatcher::new(config);

        let result = watcher.validate_file(temp_file.path());
        assert!(
            result.is_err(),
            "Expected validation to fail for invalid syntax"
        );

        Ok(())
    }
}
