//! Streaming Conversion v2 - Advanced streaming capabilities for large-scale legal document conversion.
//!
//! This module provides:
//! - Chunked conversion for processing large files without loading everything into memory
//! - Parallel format processing for converting to multiple formats simultaneously
//! - Incremental conversion updates for efficient re-conversion of modified documents
//! - Resumable conversion jobs with checkpoint/restore capabilities
//! - Progress reporting and time estimation for long-running conversions

use crate::{ConversionReport, InteropError, InteropResult, LegalConverter, LegalFormat};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write as _};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Configuration for chunked conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkConfig {
    /// Maximum number of statutes to process in one chunk
    pub chunk_size: usize,
    /// Whether to process chunks in parallel (requires 'parallel' feature)
    pub parallel: bool,
    /// Buffer size for reading input files
    pub buffer_size: usize,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            chunk_size: 100,
            parallel: false,
            buffer_size: 8192,
        }
    }
}

/// Chunked converter for processing large documents
pub struct ChunkedConverter {
    converter: LegalConverter,
    config: ChunkConfig,
}

impl ChunkedConverter {
    /// Creates a new chunked converter with default configuration
    pub fn new() -> Self {
        Self {
            converter: LegalConverter::new(),
            config: ChunkConfig::default(),
        }
    }

    /// Creates a new chunked converter with custom configuration
    pub fn with_config(config: ChunkConfig) -> Self {
        Self {
            converter: LegalConverter::new(),
            config,
        }
    }

    /// Converts a large file in chunks
    pub fn convert_file<P: AsRef<Path>>(
        &mut self,
        input_path: P,
        source_format: LegalFormat,
        target_format: LegalFormat,
        output_path: P,
    ) -> InteropResult<ConversionReport> {
        let file = File::open(input_path)?;
        let reader = BufReader::with_capacity(self.config.buffer_size, file);

        let mut output = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(output_path)?;

        let mut combined_report = ConversionReport::new(source_format, target_format);
        let mut buffer = String::new();
        let mut line_count = 0;

        for line in reader.lines() {
            let line = line?;
            buffer.push_str(&line);
            buffer.push('\n');
            line_count += 1;

            // Process when chunk is full
            if line_count >= self.config.chunk_size {
                let (chunk_output, chunk_report) =
                    self.converter
                        .convert(&buffer, source_format, target_format)?;

                writeln!(output, "{}", chunk_output)?;
                self.merge_report(&mut combined_report, chunk_report);

                buffer.clear();
                line_count = 0;
            }
        }

        // Process remaining lines
        if !buffer.is_empty() {
            let (chunk_output, chunk_report) =
                self.converter
                    .convert(&buffer, source_format, target_format)?;

            writeln!(output, "{}", chunk_output)?;
            self.merge_report(&mut combined_report, chunk_report);
        }

        Ok(combined_report)
    }

    /// Merges chunk report into combined report
    fn merge_report(&self, combined: &mut ConversionReport, chunk: ConversionReport) {
        combined.statutes_converted += chunk.statutes_converted;
        combined
            .unsupported_features
            .extend(chunk.unsupported_features);
        combined.warnings.extend(chunk.warnings);
        combined.confidence = (combined.confidence + chunk.confidence) / 2.0;
    }
}

impl Default for ChunkedConverter {
    fn default() -> Self {
        Self::new()
    }
}

/// Parallel format processor for converting to multiple formats simultaneously
pub struct ParallelFormatProcessor {
    converter: LegalConverter,
}

impl ParallelFormatProcessor {
    /// Creates a new parallel format processor
    pub fn new() -> Self {
        Self {
            converter: LegalConverter::new(),
        }
    }

    /// Converts statutes to multiple target formats in parallel
    #[cfg(feature = "parallel")]
    pub fn convert_to_multiple_formats(
        &self,
        statutes: &[Statute],
        target_formats: &[LegalFormat],
    ) -> InteropResult<Vec<(LegalFormat, String, ConversionReport)>> {
        use rayon::prelude::*;

        let results: Vec<_> = target_formats
            .par_iter()
            .map(|&format| {
                let mut converter = LegalConverter::new();
                match converter.export(statutes, format) {
                    Ok((output, report)) => (format, output, report),
                    Err(e) => {
                        let mut report = ConversionReport::new(LegalFormat::Legalis, format);
                        report.add_warning(format!("Export failed: {}", e));
                        report.confidence = 0.0;
                        (format, String::new(), report)
                    }
                }
            })
            .collect();

        Ok(results)
    }

    /// Converts statutes to multiple target formats sequentially
    #[cfg(not(feature = "parallel"))]
    pub fn convert_to_multiple_formats(
        &mut self,
        statutes: &[Statute],
        target_formats: &[LegalFormat],
    ) -> InteropResult<Vec<(LegalFormat, String, ConversionReport)>> {
        let mut results = Vec::with_capacity(target_formats.len());

        for &format in target_formats {
            match self.converter.export(statutes, format) {
                Ok((output, report)) => results.push((format, output, report)),
                Err(e) => {
                    let mut report = ConversionReport::new(LegalFormat::Legalis, format);
                    report.add_warning(format!("Export failed: {}", e));
                    report.confidence = 0.0;
                    results.push((format, String::new(), report));
                }
            }
        }

        Ok(results)
    }

    /// Processes a directory of files to multiple formats
    pub fn process_directory<P: AsRef<Path>>(
        &mut self,
        input_dir: P,
        source_format: LegalFormat,
        target_formats: &[LegalFormat],
        output_dir: P,
    ) -> InteropResult<HashMap<PathBuf, Vec<(LegalFormat, ConversionReport)>>> {
        let input_dir = input_dir.as_ref();
        let output_dir = output_dir.as_ref();

        std::fs::create_dir_all(output_dir)?;

        let mut results = HashMap::new();

        for entry in std::fs::read_dir(input_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let source = std::fs::read_to_string(&path)?;
                let (statutes, _) = self.converter.import(&source, source_format)?;

                let format_results = self.convert_to_multiple_formats(&statutes, target_formats)?;

                let mut file_results = Vec::new();
                for (format, output, report) in format_results {
                    let output_path = output_dir.join(format!(
                        "{}.{}",
                        path.file_stem().unwrap().to_string_lossy(),
                        format.extension()
                    ));

                    std::fs::write(&output_path, output)?;
                    file_results.push((format, report));
                }

                results.insert(path, file_results);
            }
        }

        Ok(results)
    }
}

impl Default for ParallelFormatProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Incremental conversion updater for efficient re-conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalUpdate {
    /// Source file path
    pub source_path: PathBuf,
    /// Last modification time
    pub last_modified: std::time::SystemTime,
    /// Content hash
    pub content_hash: u64,
    /// Number of statutes
    pub statute_count: usize,
}

pub struct IncrementalUpdater {
    converter: LegalConverter,
    state_file: PathBuf,
    state: HashMap<PathBuf, IncrementalUpdate>,
}

impl IncrementalUpdater {
    /// Creates a new incremental updater
    pub fn new<P: AsRef<Path>>(state_file: P) -> Self {
        let state_file = state_file.as_ref().to_path_buf();
        let state = Self::load_state(&state_file).unwrap_or_default();

        Self {
            converter: LegalConverter::new(),
            state_file,
            state,
        }
    }

    /// Checks if a file needs re-conversion
    pub fn needs_update<P: AsRef<Path>>(&self, path: P) -> InteropResult<bool> {
        let path = path.as_ref();
        let metadata = std::fs::metadata(path)?;
        let modified = metadata.modified()?;

        match self.state.get(path) {
            Some(update) => Ok(update.last_modified != modified),
            None => Ok(true),
        }
    }

    /// Converts a file only if it has been modified
    pub fn convert_if_modified<P: AsRef<Path>>(
        &mut self,
        input_path: P,
        source_format: LegalFormat,
        target_format: LegalFormat,
        output_path: P,
    ) -> InteropResult<Option<ConversionReport>> {
        let input_path = input_path.as_ref();

        if !self.needs_update(input_path)? {
            return Ok(None);
        }

        let source = std::fs::read_to_string(input_path)?;
        let (statutes, _) = self.converter.import(&source, source_format)?;
        let (output, report) = self.converter.export(&statutes, target_format)?;

        std::fs::write(&output_path, output)?;

        let metadata = std::fs::metadata(input_path)?;
        let update = IncrementalUpdate {
            source_path: input_path.to_path_buf(),
            last_modified: metadata.modified()?,
            content_hash: Self::hash_content(&source),
            statute_count: statutes.len(),
        };

        self.state.insert(input_path.to_path_buf(), update);
        self.save_state()?;

        Ok(Some(report))
    }

    /// Computes a simple hash of content
    fn hash_content(content: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        hasher.finish()
    }

    /// Loads state from file
    fn load_state(path: &Path) -> InteropResult<HashMap<PathBuf, IncrementalUpdate>> {
        if !path.exists() {
            return Ok(HashMap::new());
        }

        let content = std::fs::read_to_string(path)?;
        serde_json::from_str(&content).map_err(|e| InteropError::SerializationError(e.to_string()))
    }

    /// Saves state to file
    fn save_state(&self) -> InteropResult<()> {
        let content = serde_json::to_string_pretty(&self.state)
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;
        std::fs::write(&self.state_file, content)?;
        Ok(())
    }
}

/// Resumable conversion job with checkpoint/restore
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobCheckpoint {
    /// Job ID
    pub job_id: String,
    /// Files processed so far
    pub processed_files: Vec<PathBuf>,
    /// Total files to process
    pub total_files: usize,
    /// Current progress (0.0 - 1.0)
    pub progress: f64,
    /// Start time
    pub start_time: std::time::SystemTime,
    /// Last checkpoint time
    pub last_checkpoint: std::time::SystemTime,
}

pub struct ResumableJob {
    job_id: String,
    checkpoint_file: PathBuf,
    checkpoint: Option<JobCheckpoint>,
    converter: LegalConverter,
}

impl ResumableJob {
    /// Creates a new resumable job
    pub fn new<P: AsRef<Path>>(job_id: String, checkpoint_dir: P) -> Self {
        let checkpoint_file = checkpoint_dir
            .as_ref()
            .join(format!("{}.checkpoint", job_id));
        let checkpoint = Self::load_checkpoint(&checkpoint_file).ok();

        Self {
            job_id,
            checkpoint_file,
            checkpoint,
            converter: LegalConverter::new(),
        }
    }

    /// Processes a batch of files with checkpointing
    pub fn process_files<P: AsRef<Path>>(
        &mut self,
        input_files: &[PathBuf],
        source_format: LegalFormat,
        target_format: LegalFormat,
        output_dir: P,
    ) -> InteropResult<Vec<ConversionReport>> {
        let output_dir = output_dir.as_ref();
        std::fs::create_dir_all(output_dir)?;

        let start_index = self
            .checkpoint
            .as_ref()
            .map(|c| c.processed_files.len())
            .unwrap_or(0);

        let mut processed_files = self
            .checkpoint
            .as_ref()
            .map(|c| c.processed_files.clone())
            .unwrap_or_default();

        let start_time = self
            .checkpoint
            .as_ref()
            .and_then(|c| c.start_time.elapsed().ok())
            .map(|_| std::time::SystemTime::now())
            .unwrap_or_else(std::time::SystemTime::now);

        let mut reports = Vec::new();

        for (i, input_path) in input_files.iter().enumerate().skip(start_index) {
            let source = std::fs::read_to_string(input_path)?;
            let (output, report) = self
                .converter
                .convert(&source, source_format, target_format)?;

            let output_path = output_dir.join(format!(
                "{}.{}",
                input_path.file_stem().unwrap().to_string_lossy(),
                target_format.extension()
            ));

            std::fs::write(output_path, output)?;
            reports.push(report);
            processed_files.push(input_path.clone());

            // Save checkpoint every 10 files
            if (i + 1) % 10 == 0 {
                self.save_checkpoint(processed_files.clone(), input_files.len(), start_time)?;
            }
        }

        // Final checkpoint
        self.save_checkpoint(processed_files, input_files.len(), start_time)?;

        Ok(reports)
    }

    /// Saves a checkpoint
    fn save_checkpoint(
        &mut self,
        processed_files: Vec<PathBuf>,
        total_files: usize,
        start_time: std::time::SystemTime,
    ) -> InteropResult<()> {
        let progress = processed_files.len() as f64 / total_files as f64;

        let checkpoint = JobCheckpoint {
            job_id: self.job_id.clone(),
            processed_files,
            total_files,
            progress,
            start_time,
            last_checkpoint: std::time::SystemTime::now(),
        };

        let content = serde_json::to_string_pretty(&checkpoint)
            .map_err(|e| InteropError::SerializationError(e.to_string()))?;

        std::fs::write(&self.checkpoint_file, content)?;
        self.checkpoint = Some(checkpoint);

        Ok(())
    }

    /// Loads a checkpoint from file
    fn load_checkpoint(path: &Path) -> InteropResult<JobCheckpoint> {
        let content = std::fs::read_to_string(path)?;
        serde_json::from_str(&content).map_err(|e| InteropError::SerializationError(e.to_string()))
    }

    /// Returns current progress (0.0 - 1.0)
    pub fn progress(&self) -> f64 {
        self.checkpoint.as_ref().map(|c| c.progress).unwrap_or(0.0)
    }

    /// Returns whether job is complete
    pub fn is_complete(&self) -> bool {
        self.checkpoint
            .as_ref()
            .map(|c| c.processed_files.len() >= c.total_files)
            .unwrap_or(false)
    }
}

/// Progress tracker with time estimation
#[derive(Debug, Clone)]
pub struct ProgressTracker {
    total_items: usize,
    processed_items: Arc<Mutex<usize>>,
    start_time: Instant,
    last_update: Arc<Mutex<Instant>>,
}

impl ProgressTracker {
    /// Creates a new progress tracker
    pub fn new(total_items: usize) -> Self {
        Self {
            total_items,
            processed_items: Arc::new(Mutex::new(0)),
            start_time: Instant::now(),
            last_update: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Increments the processed item count
    pub fn increment(&self) {
        let mut processed = self.processed_items.lock().unwrap();
        *processed += 1;
        *self.last_update.lock().unwrap() = Instant::now();
    }

    /// Returns current progress (0.0 - 1.0)
    pub fn progress(&self) -> f64 {
        let processed = *self.processed_items.lock().unwrap();
        if self.total_items == 0 {
            return 1.0;
        }
        processed as f64 / self.total_items as f64
    }

    /// Returns number of items processed
    pub fn processed(&self) -> usize {
        *self.processed_items.lock().unwrap()
    }

    /// Returns total number of items
    pub fn total(&self) -> usize {
        self.total_items
    }

    /// Returns elapsed time
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Estimates remaining time
    pub fn estimated_remaining(&self) -> Option<Duration> {
        let processed = *self.processed_items.lock().unwrap();
        if processed == 0 {
            return None;
        }

        let elapsed = self.elapsed();
        let remaining = self.total_items - processed;
        let time_per_item = elapsed.as_secs_f64() / processed as f64;

        Some(Duration::from_secs_f64(time_per_item * remaining as f64))
    }

    /// Returns estimated total time
    pub fn estimated_total(&self) -> Option<Duration> {
        let processed = *self.processed_items.lock().unwrap();
        if processed == 0 {
            return None;
        }

        let elapsed = self.elapsed();
        let time_per_item = elapsed.as_secs_f64() / processed as f64;

        Some(Duration::from_secs_f64(
            time_per_item * self.total_items as f64,
        ))
    }

    /// Returns current throughput (items per second)
    pub fn throughput(&self) -> f64 {
        let processed = *self.processed_items.lock().unwrap();
        let elapsed = self.elapsed().as_secs_f64();

        if elapsed == 0.0 {
            return 0.0;
        }

        processed as f64 / elapsed
    }

    /// Formats progress as a percentage string
    pub fn format_progress(&self) -> String {
        format!("{:.1}%", self.progress() * 100.0)
    }

    /// Formats a detailed progress report
    pub fn format_report(&self) -> String {
        let processed = self.processed();
        let total = self.total();
        let progress = self.format_progress();
        let elapsed = self.elapsed();
        let throughput = self.throughput();

        let remaining_str = match self.estimated_remaining() {
            Some(remaining) => format!("{:.1}s", remaining.as_secs_f64()),
            None => "unknown".to_string(),
        };

        format!(
            "[{}/{}] {} | Elapsed: {:.1}s | Remaining: {} | {:.1} items/s",
            processed,
            total,
            progress,
            elapsed.as_secs_f64(),
            remaining_str,
            throughput
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};
    use tempfile::TempDir;

    #[test]
    fn test_chunk_config_default() {
        let config = ChunkConfig::default();
        assert_eq!(config.chunk_size, 100);
        assert!(!config.parallel);
        assert_eq!(config.buffer_size, 8192);
    }

    #[test]
    fn test_chunked_converter_new() {
        let converter = ChunkedConverter::new();
        assert_eq!(converter.config.chunk_size, 100);
    }

    #[test]
    fn test_chunked_converter_with_config() {
        let config = ChunkConfig {
            chunk_size: 50,
            parallel: true,
            buffer_size: 4096,
        };
        let converter = ChunkedConverter::with_config(config.clone());
        assert_eq!(converter.config.chunk_size, 50);
        assert_eq!(converter.config.buffer_size, 4096);
    }

    #[test]
    fn test_parallel_format_processor_new() {
        let processor = ParallelFormatProcessor::new();
        assert!(processor.converter.supported_exports().len() > 0);
    }

    #[test]
    #[allow(unused_mut)]
    fn test_parallel_format_processor_convert() {
        let mut processor = ParallelFormatProcessor::new();

        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "test"),
        );

        let formats = vec![LegalFormat::Catala, LegalFormat::L4];
        let results = processor
            .convert_to_multiple_formats(&[statute], &formats)
            .unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, LegalFormat::Catala);
        assert_eq!(results[1].0, LegalFormat::L4);
        assert!(!results[0].1.is_empty());
        assert!(!results[1].1.is_empty());
    }

    #[test]
    fn test_incremental_updater_new() {
        let temp_dir = TempDir::new().unwrap();
        let state_file = temp_dir.path().join("state.json");

        let updater = IncrementalUpdater::new(&state_file);
        assert_eq!(updater.state.len(), 0);
    }

    #[test]
    fn test_incremental_updater_needs_update() {
        let temp_dir = TempDir::new().unwrap();
        let state_file = temp_dir.path().join("state.json");
        let test_file = temp_dir.path().join("test.txt");

        std::fs::write(&test_file, "test content").unwrap();

        let updater = IncrementalUpdater::new(&state_file);
        let needs_update = updater.needs_update(&test_file).unwrap();
        assert!(needs_update); // Should need update on first check
    }

    #[test]
    fn test_resumable_job_new() {
        let temp_dir = TempDir::new().unwrap();
        let job = ResumableJob::new("test-job".to_string(), temp_dir.path());

        assert_eq!(job.job_id, "test-job");
        assert_eq!(job.progress(), 0.0);
        assert!(!job.is_complete());
    }

    #[test]
    fn test_resumable_job_progress() {
        let temp_dir = TempDir::new().unwrap();
        let mut job = ResumableJob::new("test-job".to_string(), temp_dir.path());

        // Manually create a checkpoint
        let checkpoint = JobCheckpoint {
            job_id: "test-job".to_string(),
            processed_files: vec![PathBuf::from("file1.txt")],
            total_files: 10,
            progress: 0.1,
            start_time: std::time::SystemTime::now(),
            last_checkpoint: std::time::SystemTime::now(),
        };
        job.checkpoint = Some(checkpoint);

        assert_eq!(job.progress(), 0.1);
        assert!(!job.is_complete());
    }

    #[test]
    fn test_progress_tracker_new() {
        let tracker = ProgressTracker::new(100);
        assert_eq!(tracker.total(), 100);
        assert_eq!(tracker.processed(), 0);
        assert_eq!(tracker.progress(), 0.0);
    }

    #[test]
    fn test_progress_tracker_increment() {
        let tracker = ProgressTracker::new(10);

        tracker.increment();
        assert_eq!(tracker.processed(), 1);
        assert_eq!(tracker.progress(), 0.1);

        tracker.increment();
        assert_eq!(tracker.processed(), 2);
        assert_eq!(tracker.progress(), 0.2);
    }

    #[test]
    fn test_progress_tracker_progress() {
        let tracker = ProgressTracker::new(100);

        for _ in 0..50 {
            tracker.increment();
        }

        assert_eq!(tracker.processed(), 50);
        assert_eq!(tracker.progress(), 0.5);
    }

    #[test]
    fn test_progress_tracker_format() {
        let tracker = ProgressTracker::new(100);

        for _ in 0..25 {
            tracker.increment();
        }

        let progress_str = tracker.format_progress();
        assert!(progress_str.contains("25"));
    }

    #[test]
    fn test_progress_tracker_report() {
        let tracker = ProgressTracker::new(100);

        for _ in 0..10 {
            tracker.increment();
        }

        let report = tracker.format_report();
        assert!(report.contains("10/100"));
        assert!(report.contains("10.0%"));
    }

    #[test]
    fn test_progress_tracker_zero_items() {
        let tracker = ProgressTracker::new(0);
        assert_eq!(tracker.progress(), 1.0);
    }

    #[test]
    fn test_progress_tracker_throughput() {
        let tracker = ProgressTracker::new(100);

        tracker.increment();
        tracker.increment();

        let throughput = tracker.throughput();
        assert!(throughput >= 0.0);
    }

    #[test]
    fn test_incremental_update_serialization() {
        let update = IncrementalUpdate {
            source_path: PathBuf::from("/test/path.txt"),
            last_modified: std::time::SystemTime::now(),
            content_hash: 12345,
            statute_count: 10,
        };

        let json = serde_json::to_string(&update).unwrap();
        let deserialized: IncrementalUpdate = serde_json::from_str(&json).unwrap();

        assert_eq!(update.source_path, deserialized.source_path);
        assert_eq!(update.content_hash, deserialized.content_hash);
        assert_eq!(update.statute_count, deserialized.statute_count);
    }

    #[test]
    fn test_job_checkpoint_serialization() {
        let checkpoint = JobCheckpoint {
            job_id: "test-123".to_string(),
            processed_files: vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")],
            total_files: 10,
            progress: 0.2,
            start_time: std::time::SystemTime::now(),
            last_checkpoint: std::time::SystemTime::now(),
        };

        let json = serde_json::to_string(&checkpoint).unwrap();
        let deserialized: JobCheckpoint = serde_json::from_str(&json).unwrap();

        assert_eq!(checkpoint.job_id, deserialized.job_id);
        assert_eq!(checkpoint.total_files, deserialized.total_files);
        assert_eq!(
            checkpoint.processed_files.len(),
            deserialized.processed_files.len()
        );
    }
}
