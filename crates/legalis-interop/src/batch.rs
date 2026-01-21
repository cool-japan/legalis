//! Batch processing module for converting multiple files.
//!
//! This module provides:
//! - Directory-based batch conversion
//! - Watch mode for continuous conversion
//! - Conversion pipeline configuration
//! - Resume capability for interrupted conversions
//! - Parallel multi-format export

#[cfg(feature = "batch")]
use std::fs;
#[cfg(feature = "batch")]
use std::path::{Path, PathBuf};
#[cfg(feature = "batch")]
use std::sync::mpsc::{Receiver, channel};

#[cfg(feature = "batch")]
use notify::{
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher,
};
#[cfg(feature = "batch")]
use rayon::prelude::*;
#[cfg(feature = "batch")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "batch")]
use walkdir::WalkDir;

use crate::{ConversionReport, InteropError, InteropResult, LegalConverter, LegalFormat, Statute};

/// Configuration for batch conversion.
#[cfg(feature = "batch")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConfig {
    /// Source directory to scan
    pub source_dir: PathBuf,
    /// Output directory for converted files
    pub output_dir: PathBuf,
    /// Source format (if known, otherwise auto-detect)
    pub source_format: Option<LegalFormat>,
    /// Target format(s) for conversion
    pub target_formats: Vec<LegalFormat>,
    /// Whether to process subdirectories recursively
    pub recursive: bool,
    /// File pattern to match (glob pattern)
    pub pattern: Option<String>,
    /// Maximum number of parallel conversions (0 = number of CPUs)
    pub max_parallel: usize,
    /// Whether to enable caching
    pub enable_cache: bool,
    /// Cache size (if caching enabled)
    pub cache_size: usize,
    /// Whether to skip files that already exist in output directory
    pub skip_existing: bool,
    /// Whether to create checkpoints for resume capability
    pub enable_checkpoints: bool,
    /// Checkpoint file path
    pub checkpoint_file: Option<PathBuf>,
}

#[cfg(feature = "batch")]
impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            source_dir: PathBuf::from("."),
            output_dir: PathBuf::from("./output"),
            source_format: None,
            target_formats: vec![],
            recursive: true,
            pattern: None,
            max_parallel: 0,
            enable_cache: true,
            cache_size: 100,
            skip_existing: false,
            enable_checkpoints: false,
            checkpoint_file: None,
        }
    }
}

#[cfg(feature = "batch")]
impl BatchConfig {
    /// Creates a new batch configuration.
    pub fn new(source_dir: impl Into<PathBuf>, output_dir: impl Into<PathBuf>) -> Self {
        Self {
            source_dir: source_dir.into(),
            output_dir: output_dir.into(),
            ..Default::default()
        }
    }

    /// Sets the source format.
    pub fn with_source_format(mut self, format: LegalFormat) -> Self {
        self.source_format = Some(format);
        self
    }

    /// Adds a target format.
    pub fn with_target_format(mut self, format: LegalFormat) -> Self {
        self.target_formats.push(format);
        self
    }

    /// Sets whether to process recursively.
    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    /// Sets the file pattern.
    pub fn with_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    /// Sets the maximum number of parallel conversions.
    pub fn with_max_parallel(mut self, max_parallel: usize) -> Self {
        self.max_parallel = max_parallel;
        self
    }

    /// Enables or disables caching.
    pub fn with_cache(mut self, enable: bool, size: usize) -> Self {
        self.enable_cache = enable;
        self.cache_size = size;
        self
    }

    /// Sets whether to skip existing files.
    pub fn with_skip_existing(mut self, skip: bool) -> Self {
        self.skip_existing = skip;
        self
    }

    /// Enables checkpoints for resume capability.
    pub fn with_checkpoints(mut self, checkpoint_file: impl Into<PathBuf>) -> Self {
        self.enable_checkpoints = true;
        self.checkpoint_file = Some(checkpoint_file.into());
        self
    }

    /// Loads configuration from a YAML file.
    pub fn from_yaml_file(path: impl AsRef<Path>) -> InteropResult<Self> {
        let content = fs::read_to_string(path)?;
        serde_yaml::from_str(&content)
            .map_err(|e| InteropError::ParseError(format!("Failed to parse config file: {}", e)))
    }

    /// Saves configuration to a YAML file.
    pub fn to_yaml_file(&self, path: impl AsRef<Path>) -> InteropResult<()> {
        let content = serde_yaml::to_string(self).map_err(|e| {
            InteropError::SerializationError(format!("Failed to serialize config: {}", e))
        })?;
        fs::write(path, content)?;
        Ok(())
    }
}

/// Progress checkpoint for resume capability.
#[cfg(feature = "batch")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCheckpoint {
    /// Files that have been successfully processed
    pub completed_files: Vec<PathBuf>,
    /// Files that failed to process
    pub failed_files: Vec<(PathBuf, String)>,
    /// Total files to process
    pub total_files: usize,
    /// Timestamp of last update
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[cfg(feature = "batch")]
impl BatchCheckpoint {
    /// Creates a new checkpoint.
    pub fn new(total_files: usize) -> Self {
        Self {
            completed_files: Vec::new(),
            failed_files: Vec::new(),
            total_files,
            last_updated: chrono::Utc::now(),
        }
    }

    /// Loads checkpoint from file.
    pub fn load(path: impl AsRef<Path>) -> InteropResult<Self> {
        let content = fs::read_to_string(path)?;
        serde_yaml::from_str(&content)
            .map_err(|e| InteropError::ParseError(format!("Failed to parse checkpoint: {}", e)))
    }

    /// Saves checkpoint to file.
    pub fn save(&self, path: impl AsRef<Path>) -> InteropResult<()> {
        let content = serde_yaml::to_string(self).map_err(|e| {
            InteropError::SerializationError(format!("Failed to serialize checkpoint: {}", e))
        })?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Marks a file as completed.
    pub fn mark_completed(&mut self, file: PathBuf) {
        self.completed_files.push(file);
        self.last_updated = chrono::Utc::now();
    }

    /// Marks a file as failed.
    pub fn mark_failed(&mut self, file: PathBuf, error: String) {
        self.failed_files.push((file, error));
        self.last_updated = chrono::Utc::now();
    }

    /// Returns progress as a percentage (0.0 - 100.0).
    pub fn progress(&self) -> f64 {
        if self.total_files == 0 {
            return 100.0;
        }
        let processed = self.completed_files.len() + self.failed_files.len();
        (processed as f64 / self.total_files as f64) * 100.0
    }

    /// Returns whether all files have been processed.
    pub fn is_complete(&self) -> bool {
        self.completed_files.len() + self.failed_files.len() >= self.total_files
    }
}

/// Result of a batch conversion operation.
#[cfg(feature = "batch")]
#[derive(Debug, Clone)]
pub struct BatchResult {
    /// Total files processed
    pub total_files: usize,
    /// Successfully converted files
    pub successful: usize,
    /// Failed files
    pub failed: usize,
    /// Skipped files (already exist)
    pub skipped: usize,
    /// Individual file results
    pub file_results: Vec<FileConversionResult>,
}

#[cfg(feature = "batch")]
impl BatchResult {
    /// Creates a new batch result.
    pub fn new() -> Self {
        Self {
            total_files: 0,
            successful: 0,
            failed: 0,
            skipped: 0,
            file_results: Vec::new(),
        }
    }

    /// Adds a file result.
    pub fn add_result(&mut self, result: FileConversionResult) {
        self.total_files += 1;
        match &result.status {
            ConversionStatus::Success => self.successful += 1,
            ConversionStatus::Failed(_) => self.failed += 1,
            ConversionStatus::Skipped => self.skipped += 1,
        }
        self.file_results.push(result);
    }

    /// Returns success rate as a percentage.
    pub fn success_rate(&self) -> f64 {
        if self.total_files == 0 {
            return 100.0;
        }
        (self.successful as f64 / self.total_files as f64) * 100.0
    }
}

#[cfg(feature = "batch")]
impl Default for BatchResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of converting a single file.
#[cfg(feature = "batch")]
#[derive(Debug, Clone)]
pub struct FileConversionResult {
    /// Source file path
    pub source_file: PathBuf,
    /// Output file path(s)
    pub output_files: Vec<PathBuf>,
    /// Conversion status
    pub status: ConversionStatus,
    /// Conversion reports (one per target format)
    pub reports: Vec<ConversionReport>,
}

#[cfg(feature = "batch")]
#[derive(Debug, Clone)]
pub enum ConversionStatus {
    /// Conversion succeeded
    Success,
    /// Conversion failed with error
    Failed(String),
    /// File was skipped
    Skipped,
}

/// Batch processor for converting multiple files.
#[cfg(feature = "batch")]
pub struct BatchProcessor {
    config: BatchConfig,
    converter: LegalConverter,
    checkpoint: Option<BatchCheckpoint>,
}

#[cfg(feature = "batch")]
impl BatchProcessor {
    /// Creates a new batch processor.
    pub fn new(config: BatchConfig) -> Self {
        let mut converter = LegalConverter::new();
        if config.enable_cache {
            converter.enable_cache(config.cache_size);
        }

        // Try to load existing checkpoint
        let checkpoint = if config.enable_checkpoints {
            if let Some(ref checkpoint_file) = config.checkpoint_file {
                BatchCheckpoint::load(checkpoint_file).ok()
            } else {
                None
            }
        } else {
            None
        };

        Self {
            config,
            converter,
            checkpoint,
        }
    }

    /// Processes all files in the configured directory.
    pub fn process(&mut self) -> InteropResult<BatchResult> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(&self.config.output_dir)?;

        // Collect files to process
        let files = self.collect_files()?;

        // Initialize checkpoint if enabled
        if self.config.enable_checkpoints && self.checkpoint.is_none() {
            self.checkpoint = Some(BatchCheckpoint::new(files.len()));
        }

        let mut result = BatchResult::new();

        // Process files (in parallel if configured)
        if self.config.max_parallel > 1 {
            result = self.process_parallel(&files)?;
        } else {
            for file in &files {
                // Skip if already processed (resume capability)
                if let Some(ref checkpoint) = self.checkpoint
                    && checkpoint.completed_files.contains(file)
                {
                    result.skipped += 1;
                    continue;
                }

                let file_result = self.process_file(file)?;
                result.add_result(file_result.clone());

                // Update checkpoint
                if let Some(ref mut checkpoint) = self.checkpoint {
                    match &file_result.status {
                        ConversionStatus::Success | ConversionStatus::Skipped => {
                            checkpoint.mark_completed(file.clone());
                        }
                        ConversionStatus::Failed(err) => {
                            checkpoint.mark_failed(file.clone(), err.clone());
                        }
                    }

                    // Save checkpoint periodically
                    if let Some(ref checkpoint_file) = self.config.checkpoint_file {
                        let _ = checkpoint.save(checkpoint_file);
                    }
                }
            }
        }

        Ok(result)
    }

    /// Processes files in parallel.
    fn process_parallel(&mut self, files: &[PathBuf]) -> InteropResult<BatchResult> {
        let config = self.config.clone();
        let checkpoint = self.checkpoint.clone();

        // Filter out already-processed files
        let files_to_process: Vec<_> = files
            .iter()
            .filter(|f| {
                if let Some(ref cp) = checkpoint {
                    !cp.completed_files.contains(f)
                } else {
                    true
                }
            })
            .cloned()
            .collect();

        let results: Vec<FileConversionResult> = files_to_process
            .par_iter()
            .map(|file| {
                // Create a new converter for each thread
                let mut converter = LegalConverter::new();
                if config.enable_cache {
                    converter.enable_cache(config.cache_size);
                }

                Self::process_file_static(file, &config, &mut converter).unwrap_or_else(|e| {
                    FileConversionResult {
                        source_file: file.clone(),
                        output_files: Vec::new(),
                        status: ConversionStatus::Failed(e.to_string()),
                        reports: Vec::new(),
                    }
                })
            })
            .collect();

        let mut batch_result = BatchResult::new();
        for result in results {
            batch_result.add_result(result);
        }

        // Add skipped files from checkpoint
        if let Some(ref checkpoint) = checkpoint {
            batch_result.skipped += checkpoint.completed_files.len();
        }

        Ok(batch_result)
    }

    /// Processes a single file.
    fn process_file(&mut self, file: &Path) -> InteropResult<FileConversionResult> {
        Self::process_file_static(file, &self.config, &mut self.converter)
    }

    /// Static version of process_file for parallel processing.
    fn process_file_static(
        file: &Path,
        config: &BatchConfig,
        converter: &mut LegalConverter,
    ) -> InteropResult<FileConversionResult> {
        let mut output_files = Vec::new();
        let mut reports = Vec::new();

        // Read source file
        let source_content = fs::read_to_string(file)?;

        // Detect or use configured format
        let source_format = if let Some(format) = config.source_format {
            format
        } else {
            // Try to detect format
            let (_, report) = converter.auto_import(&source_content)?;
            report.source_format.ok_or_else(|| {
                InteropError::UnsupportedFormat("Could not detect format".to_string())
            })?
        };

        // Convert to each target format
        for &target_format in &config.target_formats {
            // Generate output filename
            let output_file = Self::generate_output_path(file, config, target_format);

            // Check if output already exists and skip_existing is enabled
            if config.skip_existing && output_file.exists() {
                return Ok(FileConversionResult {
                    source_file: file.to_path_buf(),
                    output_files: vec![output_file],
                    status: ConversionStatus::Skipped,
                    reports: Vec::new(),
                });
            }

            // Create output directory if needed
            if let Some(parent) = output_file.parent() {
                fs::create_dir_all(parent)?;
            }

            // Convert
            let (output_content, report) =
                converter.convert(&source_content, source_format, target_format)?;

            // Write output file
            fs::write(&output_file, output_content)?;

            output_files.push(output_file);
            reports.push(report);
        }

        Ok(FileConversionResult {
            source_file: file.to_path_buf(),
            output_files,
            status: ConversionStatus::Success,
            reports,
        })
    }

    /// Collects files to process based on configuration.
    fn collect_files(&self) -> InteropResult<Vec<PathBuf>> {
        let mut files = Vec::new();

        let walker = if self.config.recursive {
            WalkDir::new(&self.config.source_dir)
        } else {
            WalkDir::new(&self.config.source_dir).max_depth(1)
        };

        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            // Check pattern if specified
            if let Some(ref pattern) = self.config.pattern
                && let Some(filename) = path.file_name().and_then(|n| n.to_str())
                && !Self::matches_pattern(filename, pattern)
            {
                continue;
            }

            files.push(path.to_path_buf());
        }

        Ok(files)
    }

    /// Simple pattern matching (supports * wildcard).
    fn matches_pattern(filename: &str, pattern: &str) -> bool {
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                return filename.starts_with(parts[0]) && filename.ends_with(parts[1]);
            }
        }
        filename == pattern
    }

    /// Generates output file path based on configuration.
    fn generate_output_path(
        source_file: &Path,
        config: &BatchConfig,
        target_format: LegalFormat,
    ) -> PathBuf {
        let filename = source_file
            .file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("output");

        let extension = target_format.extension();

        // Preserve directory structure if processing recursively
        if config.recursive
            && let Ok(relative) = source_file.strip_prefix(&config.source_dir)
        {
            let mut output_path = config.output_dir.join(relative);
            output_path.set_file_name(format!("{}.{}", filename, extension));
            return output_path;
        }

        config
            .output_dir
            .join(format!("{}.{}", filename, extension))
    }

    /// Returns the current checkpoint (if enabled).
    pub fn checkpoint(&self) -> Option<&BatchCheckpoint> {
        self.checkpoint.as_ref()
    }
}

/// Watch mode processor for continuous conversion.
#[cfg(feature = "batch")]
pub struct WatchProcessor {
    config: BatchConfig,
    converter: LegalConverter,
}

#[cfg(feature = "batch")]
impl WatchProcessor {
    /// Creates a new watch processor.
    pub fn new(config: BatchConfig) -> Self {
        let mut converter = LegalConverter::new();
        if config.enable_cache {
            converter.enable_cache(config.cache_size);
        }

        Self { config, converter }
    }

    /// Starts watching the configured directory for changes.
    ///
    /// This function blocks until an error occurs.
    pub fn watch<F>(&mut self, callback: F) -> InteropResult<()>
    where
        F: FnMut(FileConversionResult) + Send + 'static,
    {
        let (tx, rx) = channel();

        let mut watcher: RecommendedWatcher = Watcher::new(
            move |res: NotifyResult<Event>| {
                if let Ok(event) = res {
                    let _ = tx.send(event);
                }
            },
            Config::default(),
        )
        .map_err(|e| InteropError::IoError(std::io::Error::other(e)))?;

        watcher
            .watch(&self.config.source_dir, RecursiveMode::Recursive)
            .map_err(|e| InteropError::IoError(std::io::Error::other(e)))?;

        self.process_events(rx, callback)?;

        Ok(())
    }

    /// Processes file system events.
    fn process_events<F>(&mut self, rx: Receiver<Event>, mut callback: F) -> InteropResult<()>
    where
        F: FnMut(FileConversionResult),
    {
        for event in rx {
            if !matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_)) {
                continue;
            }

            for path in event.paths {
                if !path.is_file() {
                    continue;
                }

                // Check pattern if specified
                if let Some(ref pattern) = self.config.pattern
                    && let Some(filename) = path.file_name().and_then(|n| n.to_str())
                    && !BatchProcessor::matches_pattern(filename, pattern)
                {
                    continue;
                }

                // Process file
                match BatchProcessor::process_file_static(&path, &self.config, &mut self.converter)
                {
                    Ok(result) => callback(result),
                    Err(e) => {
                        let result = FileConversionResult {
                            source_file: path.clone(),
                            output_files: Vec::new(),
                            status: ConversionStatus::Failed(e.to_string()),
                            reports: Vec::new(),
                        };
                        callback(result);
                    }
                }
            }
        }

        Ok(())
    }
}

/// Conversion pipeline for complex multi-step conversions.
#[cfg(feature = "batch")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionPipeline {
    /// Pipeline name
    pub name: String,
    /// Pipeline steps
    pub steps: Vec<PipelineStep>,
}

#[cfg(feature = "batch")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStep {
    /// Step name
    pub name: String,
    /// Source format (None = use input format)
    pub source_format: Option<LegalFormat>,
    /// Target format
    pub target_format: LegalFormat,
    /// Whether to save intermediate result
    pub save_intermediate: bool,
}

#[cfg(feature = "batch")]
impl ConversionPipeline {
    /// Creates a new pipeline.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            steps: Vec::new(),
        }
    }

    /// Adds a step to the pipeline.
    pub fn add_step(
        mut self,
        name: impl Into<String>,
        target_format: LegalFormat,
        save_intermediate: bool,
    ) -> Self {
        self.steps.push(PipelineStep {
            name: name.into(),
            source_format: None,
            target_format,
            save_intermediate,
        });
        self
    }

    /// Executes the pipeline on a set of statutes.
    pub fn execute(
        &self,
        mut statutes: Vec<Statute>,
        converter: &mut LegalConverter,
    ) -> InteropResult<Vec<(String, ConversionReport)>> {
        let mut results = Vec::new();

        for step in &self.steps {
            let (output, report) = converter.export(&statutes, step.target_format)?;

            if step.save_intermediate {
                results.push((output.clone(), report.clone()));
            }

            // Re-import for next step
            if self
                .steps
                .iter()
                .skip_while(|s| s.name != step.name)
                .nth(1)
                .is_some()
            {
                let (new_statutes, _) = converter.import(&output, step.target_format)?;
                statutes = new_statutes;
            } else {
                // Last step
                results.push((output, report));
            }
        }

        Ok(results)
    }

    /// Loads pipeline from YAML file.
    pub fn from_yaml_file(path: impl AsRef<Path>) -> InteropResult<Self> {
        let content = fs::read_to_string(path)?;
        serde_yaml::from_str(&content)
            .map_err(|e| InteropError::ParseError(format!("Failed to parse pipeline file: {}", e)))
    }

    /// Saves pipeline to YAML file.
    pub fn to_yaml_file(&self, path: impl AsRef<Path>) -> InteropResult<()> {
        let content = serde_yaml::to_string(self).map_err(|e| {
            InteropError::SerializationError(format!("Failed to serialize pipeline: {}", e))
        })?;
        fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(all(test, feature = "batch"))]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    #[test]
    fn test_batch_config_default() {
        let config = BatchConfig::default();
        assert_eq!(config.source_dir, PathBuf::from("."));
        assert_eq!(config.output_dir, PathBuf::from("./output"));
        assert!(config.recursive);
        assert!(config.enable_cache);
    }

    #[test]
    fn test_batch_config_builder() {
        let config = BatchConfig::new("/tmp/source", "/tmp/output")
            .with_source_format(LegalFormat::Catala)
            .with_target_format(LegalFormat::L4)
            .with_recursive(false)
            .with_pattern("*.catala")
            .with_max_parallel(4)
            .with_skip_existing(true);

        assert_eq!(config.source_dir, PathBuf::from("/tmp/source"));
        assert_eq!(config.output_dir, PathBuf::from("/tmp/output"));
        assert_eq!(config.source_format, Some(LegalFormat::Catala));
        assert_eq!(config.target_formats, vec![LegalFormat::L4]);
        assert!(!config.recursive);
        assert_eq!(config.pattern, Some("*.catala".to_string()));
        assert_eq!(config.max_parallel, 4);
        assert!(config.skip_existing);
    }

    #[test]
    fn test_batch_checkpoint() {
        let mut checkpoint = BatchCheckpoint::new(10);
        assert_eq!(checkpoint.total_files, 10);
        assert_eq!(checkpoint.progress(), 0.0);

        checkpoint.mark_completed(PathBuf::from("file1.txt"));
        assert_eq!(checkpoint.progress(), 10.0);

        checkpoint.mark_completed(PathBuf::from("file2.txt"));
        assert_eq!(checkpoint.progress(), 20.0);

        checkpoint.mark_failed(PathBuf::from("file3.txt"), "Error".to_string());
        assert_eq!(checkpoint.progress(), 30.0);
        assert!(!checkpoint.is_complete());

        for i in 4..=10 {
            checkpoint.mark_completed(PathBuf::from(format!("file{}.txt", i)));
        }
        assert!(checkpoint.is_complete());
    }

    #[test]
    fn test_batch_result() {
        let mut result = BatchResult::new();
        assert_eq!(result.success_rate(), 100.0);

        result.add_result(FileConversionResult {
            source_file: PathBuf::from("file1.txt"),
            output_files: vec![PathBuf::from("file1.l4")],
            status: ConversionStatus::Success,
            reports: vec![],
        });
        assert_eq!(result.successful, 1);
        assert_eq!(result.success_rate(), 100.0);

        result.add_result(FileConversionResult {
            source_file: PathBuf::from("file2.txt"),
            output_files: vec![],
            status: ConversionStatus::Failed("Error".to_string()),
            reports: vec![],
        });
        assert_eq!(result.failed, 1);
        assert_eq!(result.success_rate(), 50.0);

        result.add_result(FileConversionResult {
            source_file: PathBuf::from("file3.txt"),
            output_files: vec![PathBuf::from("file3.l4")],
            status: ConversionStatus::Skipped,
            reports: vec![],
        });
        assert_eq!(result.skipped, 1);
        assert_eq!(result.total_files, 3);
    }

    #[test]
    fn test_pattern_matching() {
        assert!(BatchProcessor::matches_pattern("test.catala", "*.catala"));
        assert!(BatchProcessor::matches_pattern("test.catala", "test.*"));
        assert!(BatchProcessor::matches_pattern(
            "test.catala",
            "test.catala"
        ));
        assert!(!BatchProcessor::matches_pattern("test.l4", "*.catala"));
    }

    #[test]
    fn test_conversion_pipeline() {
        let pipeline = ConversionPipeline::new("test-pipeline")
            .add_step("to-l4", LegalFormat::L4, true)
            .add_step("to-catala", LegalFormat::Catala, true);

        assert_eq!(pipeline.name, "test-pipeline");
        assert_eq!(pipeline.steps.len(), 2);
        assert_eq!(pipeline.steps[0].target_format, LegalFormat::L4);
        assert_eq!(pipeline.steps[1].target_format, LegalFormat::Catala);
    }

    #[test]
    fn test_pipeline_execute() {
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "test"),
        );

        let pipeline = ConversionPipeline::new("test")
            .add_step("to-l4", LegalFormat::L4, false)
            .add_step("to-catala", LegalFormat::Catala, false);

        let mut converter = LegalConverter::new();
        let results = pipeline.execute(vec![statute], &mut converter).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].0.contains("declaration scope"));
    }

    #[test]
    fn test_generate_output_path() {
        let config =
            BatchConfig::new("/tmp/source", "/tmp/output").with_target_format(LegalFormat::L4);

        let source_file = PathBuf::from("/tmp/source/test.catala");
        let output_path =
            BatchProcessor::generate_output_path(&source_file, &config, LegalFormat::L4);

        assert_eq!(output_path, PathBuf::from("/tmp/output/test.l4"));
    }

    #[test]
    fn test_generate_output_path_recursive() {
        let config = BatchConfig::new("/tmp/source", "/tmp/output")
            .with_target_format(LegalFormat::L4)
            .with_recursive(true);

        let source_file = PathBuf::from("/tmp/source/subdir/test.catala");
        let output_path =
            BatchProcessor::generate_output_path(&source_file, &config, LegalFormat::L4);

        assert_eq!(output_path, PathBuf::from("/tmp/output/subdir/test.l4"));
    }
}
