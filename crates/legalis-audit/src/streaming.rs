//! Streaming audit analysis for real-time processing.
//!
//! This module provides streaming analytics capabilities for processing
//! audit records in real-time as they arrive.

use crate::{AuditRecord, AuditResult, DecisionResult};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// Streaming window configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    /// Window size in seconds
    pub window_size_seconds: i64,
    /// Slide interval in seconds
    pub slide_interval_seconds: i64,
    /// Maximum buffer size
    pub max_buffer_size: usize,
    /// Window type
    pub window_type: WindowType,
    /// Session timeout (for session windows)
    pub session_timeout_seconds: i64,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            window_size_seconds: 300,   // 5 minutes
            slide_interval_seconds: 60, // 1 minute
            max_buffer_size: 10000,
            window_type: WindowType::Sliding,
            session_timeout_seconds: 300,
        }
    }
}

/// Window type for streaming.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WindowType {
    /// Tumbling window (non-overlapping)
    Tumbling,
    /// Sliding window (overlapping)
    Sliding,
    /// Session window (gaps trigger new window)
    Session,
}

/// Streaming analytics result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingMetrics {
    /// Window timestamp
    pub window_start: DateTime<Utc>,
    /// Window end
    pub window_end: DateTime<Utc>,
    /// Record count in window
    pub count: usize,
    /// Average rate (records/second)
    pub rate: f64,
    /// Override percentage
    pub override_pct: f64,
    /// Discretionary percentage
    pub discretionary_pct: f64,
    /// Void percentage
    pub void_pct: f64,
    /// Unique actors
    pub unique_actors: usize,
    /// Unique statutes
    pub unique_statutes: usize,
    /// Top statute ID
    pub top_statute: Option<String>,
}

/// Aggregation operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AggregationOp {
    Count,
    Sum,
    Average,
    Min,
    Max,
    DistinctCount,
}

/// Pattern to detect in stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamPattern {
    /// Pattern name
    pub name: String,
    /// Minimum occurrences
    pub min_occurrences: usize,
    /// Time window for pattern
    pub within_seconds: i64,
    /// Pattern type
    pub pattern_type: PatternType,
}

/// Pattern type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PatternType {
    /// Repeated overrides
    RepeatedOverrides,
    /// Same statute repeated
    RepeatedStatute(String),
    /// Same subject repeated
    RepeatedSubject(Uuid),
    /// Rapid decisions
    RapidDecisions,
}

/// Detected pattern instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedPattern {
    /// Pattern that matched
    pub pattern: StreamPattern,
    /// When it was detected
    pub detected_at: DateTime<Utc>,
    /// Related records
    pub related_records: Vec<Uuid>,
    /// Pattern score
    pub score: f64,
}

/// Streaming analyzer.
pub struct StreamingAnalyzer {
    config: StreamConfig,
    buffer: Arc<RwLock<VecDeque<AuditRecord>>>,
    patterns: Arc<RwLock<Vec<StreamPattern>>>,
    detected_patterns: Arc<RwLock<Vec<DetectedPattern>>>,
}

impl StreamingAnalyzer {
    /// Creates a new streaming analyzer.
    pub fn new() -> Self {
        Self::with_config(StreamConfig::default())
    }

    /// Creates with custom config.
    pub fn with_config(config: StreamConfig) -> Self {
        Self {
            config,
            buffer: Arc::new(RwLock::new(VecDeque::new())),
            patterns: Arc::new(RwLock::new(Vec::new())),
            detected_patterns: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Adds a pattern to detect.
    pub fn add_pattern(&self, pattern: StreamPattern) {
        let mut patterns = self.patterns.write().unwrap();
        patterns.push(pattern);
    }

    /// Removes a pattern.
    pub fn remove_pattern(&self, name: &str) {
        let mut patterns = self.patterns.write().unwrap();
        patterns.retain(|p| p.name != name);
    }

    /// Processes a new record.
    pub fn process(&self, record: AuditRecord) -> AuditResult<Vec<DetectedPattern>> {
        let mut buffer = self.buffer.write().unwrap();
        buffer.push_back(record);

        // Trim old records based on window type
        match self.config.window_type {
            WindowType::Tumbling => {
                // Keep records in current window only
                let window_start = self.calculate_tumbling_window_start();
                buffer.retain(|r| r.timestamp >= window_start);
            }
            WindowType::Sliding => {
                // Keep records within sliding window
                let cutoff = Utc::now() - Duration::seconds(self.config.window_size_seconds);
                while buffer.front().is_some_and(|r| r.timestamp < cutoff) {
                    buffer.pop_front();
                }
            }
            WindowType::Session => {
                // Session windows don't automatically trim
                // They close on timeout
            }
        }

        // Enforce max size
        if buffer.len() > self.config.max_buffer_size {
            buffer.pop_front();
        }

        drop(buffer);

        // Detect patterns
        self.detect_patterns()
    }

    /// Calculates tumbling window start time.
    fn calculate_tumbling_window_start(&self) -> DateTime<Utc> {
        let now = Utc::now();
        let window_seconds = self.config.window_size_seconds;
        let timestamp_seconds = now.timestamp();
        let window_start_seconds = (timestamp_seconds / window_seconds) * window_seconds;
        DateTime::from_timestamp(window_start_seconds, 0).unwrap_or(now)
    }

    /// Detects patterns in current buffer.
    fn detect_patterns(&self) -> AuditResult<Vec<DetectedPattern>> {
        let buffer = self.buffer.read().unwrap();
        let patterns = self.patterns.read().unwrap();
        let mut detected = Vec::new();

        for pattern in patterns.iter() {
            if let Some(detection) = self.check_pattern(&buffer, pattern) {
                detected.push(detection.clone());

                let mut detected_patterns = self.detected_patterns.write().unwrap();
                detected_patterns.push(detection);

                // Keep only recent detections
                let cutoff = Utc::now() - Duration::hours(24);
                detected_patterns.retain(|d| d.detected_at >= cutoff);
            }
        }

        Ok(detected)
    }

    /// Checks if a pattern matches.
    fn check_pattern(
        &self,
        buffer: &VecDeque<AuditRecord>,
        pattern: &StreamPattern,
    ) -> Option<DetectedPattern> {
        let cutoff = Utc::now() - Duration::seconds(pattern.within_seconds);
        let recent_records: Vec<_> = buffer.iter().filter(|r| r.timestamp >= cutoff).collect();

        let (matched, related_ids) = match &pattern.pattern_type {
            PatternType::RepeatedOverrides => {
                let override_records: Vec<_> = recent_records
                    .iter()
                    .filter(|r| matches!(r.result, DecisionResult::Overridden { .. }))
                    .collect();

                let matched = override_records.len() >= pattern.min_occurrences;
                let ids = override_records.iter().map(|r| r.id).collect();
                (matched, ids)
            }
            PatternType::RepeatedStatute(statute_id) => {
                let statute_records: Vec<_> = recent_records
                    .iter()
                    .filter(|r| r.statute_id == *statute_id)
                    .collect();

                let matched = statute_records.len() >= pattern.min_occurrences;
                let ids = statute_records.iter().map(|r| r.id).collect();
                (matched, ids)
            }
            PatternType::RepeatedSubject(subject_id) => {
                let subject_records: Vec<_> = recent_records
                    .iter()
                    .filter(|r| r.subject_id == *subject_id)
                    .collect();

                let matched = subject_records.len() >= pattern.min_occurrences;
                let ids = subject_records.iter().map(|r| r.id).collect();
                (matched, ids)
            }
            PatternType::RapidDecisions => {
                let matched = recent_records.len() >= pattern.min_occurrences;
                let ids = recent_records.iter().map(|r| r.id).collect();
                (matched, ids)
            }
        };

        if matched {
            Some(DetectedPattern {
                pattern: pattern.clone(),
                detected_at: Utc::now(),
                related_records: related_ids,
                score: 1.0,
            })
        } else {
            None
        }
    }

    /// Gets current window metrics.
    pub fn current_metrics(&self) -> StreamingMetrics {
        let buffer = self.buffer.read().unwrap();
        let now = Utc::now();
        let window_start = now - Duration::seconds(self.config.window_size_seconds);

        let count = buffer.len();
        let override_count = buffer
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::Overridden { .. }))
            .count();
        let discretionary_count = buffer
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::RequiresDiscretion { .. }))
            .count();
        let void_count = buffer
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::Void { .. }))
            .count();

        let rate = count as f64 / self.config.window_size_seconds as f64;
        let override_pct = if count > 0 {
            (override_count as f64 / count as f64) * 100.0
        } else {
            0.0
        };
        let discretionary_pct = if count > 0 {
            (discretionary_count as f64 / count as f64) * 100.0
        } else {
            0.0
        };
        let void_pct = if count > 0 {
            (void_count as f64 / count as f64) * 100.0
        } else {
            0.0
        };

        // Count unique actors
        let unique_actors = buffer
            .iter()
            .map(|r| match &r.actor {
                crate::Actor::System { component } => format!("system:{}", component),
                crate::Actor::User { user_id, .. } => format!("user:{}", user_id),
                crate::Actor::External { system_id } => format!("external:{}", system_id),
            })
            .collect::<std::collections::HashSet<_>>()
            .len();

        // Count unique statutes and find top statute
        let mut statute_counts: HashMap<String, usize> = HashMap::new();
        for record in buffer.iter() {
            *statute_counts.entry(record.statute_id.clone()).or_insert(0) += 1;
        }
        let unique_statutes = statute_counts.len();
        let top_statute = statute_counts
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(statute, _)| statute.clone());

        StreamingMetrics {
            window_start,
            window_end: now,
            count,
            rate,
            override_pct,
            discretionary_pct,
            void_pct,
            unique_actors,
            unique_statutes,
            top_statute,
        }
    }

    /// Gets detected patterns.
    pub fn get_detected_patterns(&self) -> Vec<DetectedPattern> {
        self.detected_patterns.read().unwrap().clone()
    }

    /// Clears detected patterns.
    pub fn clear_detected_patterns(&self) {
        self.detected_patterns.write().unwrap().clear();
    }

    /// Performs aggregation on the current window.
    pub fn aggregate(&self, op: AggregationOp, field: &str) -> AuditResult<f64> {
        let buffer = self.buffer.read().unwrap();

        match op {
            AggregationOp::Count => Ok(buffer.len() as f64),
            AggregationOp::DistinctCount => match field {
                "statute" => {
                    let unique: std::collections::HashSet<_> =
                        buffer.iter().map(|r| &r.statute_id).collect();
                    Ok(unique.len() as f64)
                }
                "subject" => {
                    let unique: std::collections::HashSet<_> =
                        buffer.iter().map(|r| r.subject_id).collect();
                    Ok(unique.len() as f64)
                }
                _ => Ok(buffer.len() as f64),
            },
            _ => {
                // For other aggregations, return count as placeholder
                Ok(buffer.len() as f64)
            }
        }
    }

    /// Returns configuration.
    pub fn config(&self) -> &StreamConfig {
        &self.config
    }

    /// Returns current buffer size.
    pub fn buffer_size(&self) -> usize {
        self.buffer.read().unwrap().len()
    }
}

impl Default for StreamingAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;

    fn create_test_record() -> AuditRecord {
        AuditRecord {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: EventType::AutomaticDecision,
            actor: Actor::System {
                component: "test".to_string(),
            },
            statute_id: "test".to_string(),
            subject_id: Uuid::new_v4(),
            context: DecisionContext::default(),
            result: DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: HashMap::new(),
            },
            previous_hash: None,
            record_hash: String::new(),
        }
    }

    fn create_override_record() -> AuditRecord {
        AuditRecord {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: EventType::HumanOverride,
            actor: Actor::User {
                user_id: "user-1".to_string(),
                role: "admin".to_string(),
            },
            statute_id: "test".to_string(),
            subject_id: Uuid::new_v4(),
            context: DecisionContext::default(),
            result: DecisionResult::Overridden {
                original_result: Box::new(DecisionResult::Deterministic {
                    effect_applied: "approved".to_string(),
                    parameters: HashMap::new(),
                }),
                new_result: Box::new(DecisionResult::Deterministic {
                    effect_applied: "denied".to_string(),
                    parameters: HashMap::new(),
                }),
                justification: "test".to_string(),
            },
            previous_hash: None,
            record_hash: String::new(),
        }
    }

    #[test]
    fn test_streaming_analyzer() {
        let analyzer = StreamingAnalyzer::new();

        for _ in 0..10 {
            analyzer.process(create_test_record()).unwrap();
        }

        let metrics = analyzer.current_metrics();
        assert_eq!(metrics.count, 10);
    }

    #[test]
    fn test_streaming_metrics() {
        let analyzer = StreamingAnalyzer::new();

        for _ in 0..5 {
            analyzer.process(create_test_record()).unwrap();
        }
        for _ in 0..3 {
            analyzer.process(create_override_record()).unwrap();
        }

        let metrics = analyzer.current_metrics();
        assert_eq!(metrics.count, 8);
        assert!(metrics.override_pct > 0.0);
        assert!(metrics.unique_actors > 0);
        assert!(metrics.unique_statutes > 0);
    }

    #[test]
    fn test_pattern_detection() {
        let analyzer = StreamingAnalyzer::new();

        let pattern = StreamPattern {
            name: "test-pattern".to_string(),
            min_occurrences: 3,
            within_seconds: 300,
            pattern_type: PatternType::RepeatedOverrides,
        };

        analyzer.add_pattern(pattern);

        for _ in 0..5 {
            let detections = analyzer.process(create_override_record()).unwrap();
            if !detections.is_empty() {
                assert_eq!(detections[0].pattern.name, "test-pattern");
                break;
            }
        }

        assert!(analyzer.buffer_size() > 0);
    }

    #[test]
    fn test_aggregation() {
        let analyzer = StreamingAnalyzer::new();

        for _ in 0..10 {
            analyzer.process(create_test_record()).unwrap();
        }

        let count = analyzer.aggregate(AggregationOp::Count, "").unwrap();
        assert_eq!(count, 10.0);

        let distinct = analyzer
            .aggregate(AggregationOp::DistinctCount, "statute")
            .unwrap();
        assert!(distinct >= 1.0);
    }

    #[test]
    fn test_window_types() {
        let config = StreamConfig {
            window_type: WindowType::Tumbling,
            ..Default::default()
        };
        let analyzer = StreamingAnalyzer::with_config(config);

        for _ in 0..5 {
            analyzer.process(create_test_record()).unwrap();
        }

        assert!(analyzer.buffer_size() > 0);
    }

    #[test]
    fn test_pattern_management() {
        let analyzer = StreamingAnalyzer::new();

        let pattern = StreamPattern {
            name: "test".to_string(),
            min_occurrences: 5,
            within_seconds: 300,
            pattern_type: PatternType::RapidDecisions,
        };

        analyzer.add_pattern(pattern);
        analyzer.remove_pattern("test");

        let detections = analyzer.get_detected_patterns();
        assert_eq!(detections.len(), 0);
    }
}
