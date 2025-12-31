//! Conversion metrics and logging for legalis-interop.
//!
//! This module provides comprehensive metrics tracking and logging for conversion operations.

use crate::{ConversionReport, LegalFormat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Metrics for a single conversion operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionMetrics {
    /// Conversion ID (unique identifier)
    pub id: String,
    /// Source format
    pub source_format: LegalFormat,
    /// Target format
    pub target_format: LegalFormat,
    /// Start time (Unix timestamp in milliseconds)
    pub start_time: u64,
    /// End time (Unix timestamp in milliseconds)
    pub end_time: Option<u64>,
    /// Duration in milliseconds
    pub duration_ms: Option<u64>,
    /// Number of statutes converted
    pub statutes_converted: usize,
    /// Conversion confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Number of unsupported features
    pub unsupported_count: usize,
    /// Number of warnings
    pub warning_count: usize,
    /// Success flag
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Input size in bytes
    pub input_size: usize,
    /// Output size in bytes
    pub output_size: Option<usize>,
}

impl ConversionMetrics {
    /// Creates a new metrics instance for a conversion.
    pub fn new(id: String, source: LegalFormat, target: LegalFormat, input_size: usize) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Self {
            id,
            source_format: source,
            target_format: target,
            start_time: now,
            end_time: None,
            duration_ms: None,
            statutes_converted: 0,
            confidence: 0.0,
            unsupported_count: 0,
            warning_count: 0,
            success: false,
            error: None,
            input_size,
            output_size: None,
        }
    }

    /// Marks the conversion as completed successfully.
    pub fn complete(&mut self, output_size: usize, report: &ConversionReport) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        self.end_time = Some(now);
        self.duration_ms = Some(now.saturating_sub(self.start_time));
        self.output_size = Some(output_size);
        self.statutes_converted = report.statutes_converted;
        self.confidence = report.confidence;
        self.unsupported_count = report.unsupported_features.len();
        self.warning_count = report.warnings.len();
        self.success = true;
    }

    /// Marks the conversion as failed.
    pub fn fail(&mut self, error: String) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        self.end_time = Some(now);
        self.duration_ms = Some(now.saturating_sub(self.start_time));
        self.success = false;
        self.error = Some(error);
    }

    /// Returns the duration as a Duration type.
    pub fn duration(&self) -> Option<Duration> {
        self.duration_ms.map(Duration::from_millis)
    }
}

/// Aggregated metrics across multiple conversions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AggregatedMetrics {
    /// Total number of conversions
    pub total_conversions: usize,
    /// Number of successful conversions
    pub successful_conversions: usize,
    /// Number of failed conversions
    pub failed_conversions: usize,
    /// Total duration in milliseconds
    pub total_duration_ms: u64,
    /// Average duration in milliseconds
    pub avg_duration_ms: f64,
    /// Minimum duration in milliseconds
    pub min_duration_ms: Option<u64>,
    /// Maximum duration in milliseconds
    pub max_duration_ms: Option<u64>,
    /// Total statutes converted
    pub total_statutes: usize,
    /// Average confidence score
    pub avg_confidence: f64,
    /// Total input size in bytes
    pub total_input_bytes: usize,
    /// Total output size in bytes
    pub total_output_bytes: usize,
    /// Conversion counts by format pair
    pub format_pairs: HashMap<String, usize>,
}

impl AggregatedMetrics {
    /// Creates aggregated metrics from a list of individual metrics.
    pub fn from_metrics(metrics: &[ConversionMetrics]) -> Self {
        if metrics.is_empty() {
            return Self::default();
        }

        let mut agg = Self::default();
        let mut total_confidence = 0.0;
        let mut format_pairs = HashMap::new();

        for m in metrics {
            agg.total_conversions += 1;
            if m.success {
                agg.successful_conversions += 1;
            } else {
                agg.failed_conversions += 1;
            }

            if let Some(duration) = m.duration_ms {
                agg.total_duration_ms += duration;
                agg.min_duration_ms = Some(
                    agg.min_duration_ms
                        .map(|min| min.min(duration))
                        .unwrap_or(duration),
                );
                agg.max_duration_ms = Some(
                    agg.max_duration_ms
                        .map(|max| max.max(duration))
                        .unwrap_or(duration),
                );
            }

            agg.total_statutes += m.statutes_converted;
            total_confidence += m.confidence;
            agg.total_input_bytes += m.input_size;
            if let Some(output_size) = m.output_size {
                agg.total_output_bytes += output_size;
            }

            // Track format pairs
            let pair_key = format!("{:?} -> {:?}", m.source_format, m.target_format);
            *format_pairs.entry(pair_key).or_insert(0) += 1;
        }

        agg.avg_duration_ms = if agg.total_conversions > 0 {
            agg.total_duration_ms as f64 / agg.total_conversions as f64
        } else {
            0.0
        };

        agg.avg_confidence = if agg.total_conversions > 0 {
            total_confidence / agg.total_conversions as f64
        } else {
            0.0
        };

        agg.format_pairs = format_pairs;

        agg
    }

    /// Returns the success rate (0.0 - 1.0).
    pub fn success_rate(&self) -> f64 {
        if self.total_conversions == 0 {
            return 0.0;
        }
        self.successful_conversions as f64 / self.total_conversions as f64
    }

    /// Returns the average throughput in statutes per second.
    pub fn throughput(&self) -> f64 {
        if self.total_duration_ms == 0 {
            return 0.0;
        }
        (self.total_statutes as f64 * 1000.0) / self.total_duration_ms as f64
    }
}

/// Metrics collector for tracking conversion operations.
pub struct MetricsCollector {
    metrics: Vec<ConversionMetrics>,
    max_entries: usize,
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new(1000)
    }
}

impl MetricsCollector {
    /// Creates a new metrics collector with a maximum number of entries.
    pub fn new(max_entries: usize) -> Self {
        Self {
            metrics: Vec::new(),
            max_entries,
        }
    }

    /// Records a new conversion metric.
    pub fn record(&mut self, metric: ConversionMetrics) {
        self.metrics.push(metric);

        // Trim to max size (keep most recent)
        if self.metrics.len() > self.max_entries {
            let excess = self.metrics.len() - self.max_entries;
            self.metrics.drain(0..excess);
        }
    }

    /// Returns all recorded metrics.
    pub fn metrics(&self) -> &[ConversionMetrics] {
        &self.metrics
    }

    /// Returns aggregated metrics.
    pub fn aggregated(&self) -> AggregatedMetrics {
        AggregatedMetrics::from_metrics(&self.metrics)
    }

    /// Filters metrics by source format.
    pub fn filter_by_source(&self, format: LegalFormat) -> Vec<&ConversionMetrics> {
        self.metrics
            .iter()
            .filter(|m| m.source_format == format)
            .collect()
    }

    /// Filters metrics by target format.
    pub fn filter_by_target(&self, format: LegalFormat) -> Vec<&ConversionMetrics> {
        self.metrics
            .iter()
            .filter(|m| m.target_format == format)
            .collect()
    }

    /// Filters successful conversions.
    pub fn successful(&self) -> Vec<&ConversionMetrics> {
        self.metrics.iter().filter(|m| m.success).collect()
    }

    /// Filters failed conversions.
    pub fn failed(&self) -> Vec<&ConversionMetrics> {
        self.metrics.iter().filter(|m| !m.success).collect()
    }

    /// Clears all metrics.
    pub fn clear(&mut self) {
        self.metrics.clear();
    }

    /// Returns the number of recorded metrics.
    pub fn len(&self) -> usize {
        self.metrics.len()
    }

    /// Returns true if no metrics are recorded.
    pub fn is_empty(&self) -> bool {
        self.metrics.is_empty()
    }
}

/// Logger for conversion operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// Log entry for conversion operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Timestamp (Unix timestamp in milliseconds)
    pub timestamp: u64,
    /// Log level
    pub level: String,
    /// Conversion ID (if applicable)
    pub conversion_id: Option<String>,
    /// Message
    pub message: String,
    /// Additional context
    pub context: HashMap<String, String>,
}

impl LogEntry {
    /// Creates a new log entry.
    pub fn new(level: LogLevel, message: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Self {
            timestamp,
            level: format!("{:?}", level),
            conversion_id: None,
            message,
            context: HashMap::new(),
        }
    }

    /// Sets the conversion ID.
    pub fn with_conversion_id(mut self, id: String) -> Self {
        self.conversion_id = Some(id);
        self
    }

    /// Adds context information.
    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.context.insert(key, value);
        self
    }
}

/// Logger for tracking conversion operations.
pub struct ConversionLogger {
    entries: Vec<LogEntry>,
    max_entries: usize,
    min_level: LogLevel,
}

impl Default for ConversionLogger {
    fn default() -> Self {
        Self::new(10000, LogLevel::Info)
    }
}

impl ConversionLogger {
    /// Creates a new logger with maximum entries and minimum log level.
    pub fn new(max_entries: usize, min_level: LogLevel) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
            min_level,
        }
    }

    /// Logs a message at the specified level.
    pub fn log(&mut self, level: LogLevel, message: String) {
        if level < self.min_level {
            return;
        }

        let entry = LogEntry::new(level, message);
        self.entries.push(entry);

        // Trim to max size
        if self.entries.len() > self.max_entries {
            let excess = self.entries.len() - self.max_entries;
            self.entries.drain(0..excess);
        }
    }

    /// Logs a debug message.
    pub fn debug(&mut self, message: String) {
        self.log(LogLevel::Debug, message);
    }

    /// Logs an info message.
    pub fn info(&mut self, message: String) {
        self.log(LogLevel::Info, message);
    }

    /// Logs a warning message.
    pub fn warning(&mut self, message: String) {
        self.log(LogLevel::Warning, message);
    }

    /// Logs an error message.
    pub fn error(&mut self, message: String) {
        self.log(LogLevel::Error, message);
    }

    /// Logs a conversion start.
    pub fn log_conversion_start(&mut self, id: &str, source: LegalFormat, target: LegalFormat) {
        let message = format!(
            "Starting conversion {} from {:?} to {:?}",
            id, source, target
        );
        self.info(message);
    }

    /// Logs a conversion completion.
    pub fn log_conversion_complete(&mut self, metrics: &ConversionMetrics) {
        let message = format!(
            "Completed conversion {} in {}ms ({} statutes, confidence: {:.2})",
            metrics.id,
            metrics.duration_ms.unwrap_or(0),
            metrics.statutes_converted,
            metrics.confidence
        );
        self.info(message);
    }

    /// Logs a conversion failure.
    pub fn log_conversion_failed(&mut self, id: &str, error: &str) {
        let message = format!("Conversion {} failed: {}", id, error);
        self.error(message);
    }

    /// Returns all log entries.
    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }

    /// Clears all log entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns the number of log entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if no entries are logged.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Filters entries by level.
    pub fn filter_by_level(&self, level: LogLevel) -> Vec<&LogEntry> {
        let level_str = format!("{:?}", level);
        self.entries
            .iter()
            .filter(|e| e.level == level_str)
            .collect()
    }

    /// Filters entries by conversion ID.
    pub fn filter_by_conversion(&self, id: &str) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|e| e.conversion_id.as_deref() == Some(id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_metrics_new() {
        let metrics = ConversionMetrics::new(
            "test-1".to_string(),
            LegalFormat::Catala,
            LegalFormat::L4,
            1024,
        );

        assert_eq!(metrics.id, "test-1");
        assert_eq!(metrics.source_format, LegalFormat::Catala);
        assert_eq!(metrics.target_format, LegalFormat::L4);
        assert_eq!(metrics.input_size, 1024);
        assert!(!metrics.success);
    }

    #[test]
    fn test_conversion_metrics_complete() {
        let mut metrics = ConversionMetrics::new(
            "test-1".to_string(),
            LegalFormat::Catala,
            LegalFormat::L4,
            1024,
        );

        let mut report = ConversionReport::new(LegalFormat::Catala, LegalFormat::L4);
        report.statutes_converted = 5;
        report.confidence = 0.95;
        report.add_warning("test warning");

        metrics.complete(2048, &report);

        assert!(metrics.success);
        assert_eq!(metrics.output_size, Some(2048));
        assert_eq!(metrics.statutes_converted, 5);
        // Confidence is 0.95 - 0.05 (from add_warning) = 0.90
        assert!((metrics.confidence - 0.90).abs() < 0.01);
        assert_eq!(metrics.warning_count, 1);
        assert!(metrics.duration_ms.is_some());
    }

    #[test]
    fn test_conversion_metrics_fail() {
        let mut metrics = ConversionMetrics::new(
            "test-1".to_string(),
            LegalFormat::Catala,
            LegalFormat::L4,
            1024,
        );

        metrics.fail("Parse error".to_string());

        assert!(!metrics.success);
        assert_eq!(metrics.error, Some("Parse error".to_string()));
        assert!(metrics.duration_ms.is_some());
    }

    #[test]
    fn test_aggregated_metrics() {
        let mut metrics = vec![];

        for i in 0..10 {
            let mut m = ConversionMetrics::new(
                format!("test-{}", i),
                LegalFormat::Catala,
                LegalFormat::L4,
                1024,
            );

            let mut report = ConversionReport::new(LegalFormat::Catala, LegalFormat::L4);
            report.statutes_converted = 3;
            report.confidence = 0.9;

            m.complete(2048, &report);
            metrics.push(m);
        }

        let agg = AggregatedMetrics::from_metrics(&metrics);

        assert_eq!(agg.total_conversions, 10);
        assert_eq!(agg.successful_conversions, 10);
        assert_eq!(agg.failed_conversions, 0);
        assert_eq!(agg.total_statutes, 30);
        assert!((agg.avg_confidence - 0.9).abs() < 0.01);
        assert_eq!(agg.success_rate(), 1.0);
    }

    #[test]
    fn test_metrics_collector() {
        let mut collector = MetricsCollector::new(5);

        for i in 0..10 {
            let metrics = ConversionMetrics::new(
                format!("test-{}", i),
                LegalFormat::Catala,
                LegalFormat::L4,
                1024,
            );
            collector.record(metrics);
        }

        // Should only keep last 5
        assert_eq!(collector.len(), 5);
        assert_eq!(collector.metrics()[0].id, "test-5");
    }

    #[test]
    fn test_metrics_collector_filter() {
        let mut collector = MetricsCollector::new(100);

        let m1 = ConversionMetrics::new(
            "test-1".to_string(),
            LegalFormat::Catala,
            LegalFormat::L4,
            1024,
        );
        let m2 = ConversionMetrics::new(
            "test-2".to_string(),
            LegalFormat::Stipula,
            LegalFormat::L4,
            1024,
        );

        collector.record(m1);
        collector.record(m2);

        let catala_metrics = collector.filter_by_source(LegalFormat::Catala);
        assert_eq!(catala_metrics.len(), 1);

        let l4_metrics = collector.filter_by_target(LegalFormat::L4);
        assert_eq!(l4_metrics.len(), 2);
    }

    #[test]
    fn test_conversion_logger() {
        let mut logger = ConversionLogger::new(100, LogLevel::Info);

        logger.debug("Debug message".to_string());
        logger.info("Info message".to_string());
        logger.warning("Warning message".to_string());
        logger.error("Error message".to_string());

        // Debug should be filtered out
        assert_eq!(logger.len(), 3);

        let errors = logger.filter_by_level(LogLevel::Error);
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_logger_conversion_tracking() {
        let mut logger = ConversionLogger::default();

        logger.log_conversion_start("test-1", LegalFormat::Catala, LegalFormat::L4);

        let mut metrics = ConversionMetrics::new(
            "test-1".to_string(),
            LegalFormat::Catala,
            LegalFormat::L4,
            1024,
        );
        let report = ConversionReport::new(LegalFormat::Catala, LegalFormat::L4);
        metrics.complete(2048, &report);

        logger.log_conversion_complete(&metrics);

        assert_eq!(logger.len(), 2);
    }

    #[test]
    fn test_log_entry() {
        let entry = LogEntry::new(LogLevel::Info, "Test message".to_string())
            .with_conversion_id("test-1".to_string())
            .with_context("key1".to_string(), "value1".to_string());

        assert_eq!(entry.level, "Info");
        assert_eq!(entry.message, "Test message");
        assert_eq!(entry.conversion_id, Some("test-1".to_string()));
        assert_eq!(entry.context.get("key1"), Some(&"value1".to_string()));
    }
}
