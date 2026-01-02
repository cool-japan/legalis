//! Streaming audit analysis for real-time processing.
//!
//! This module provides streaming analytics capabilities for processing
//! audit records in real-time as they arrive.

use crate::{AuditRecord, AuditResult};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};

/// Streaming window configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    /// Window size in seconds
    pub window_size_seconds: i64,
    /// Slide interval in seconds
    pub slide_interval_seconds: i64,
    /// Maximum buffer size
    pub max_buffer_size: usize,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            window_size_seconds: 300,   // 5 minutes
            slide_interval_seconds: 60, // 1 minute
            max_buffer_size: 10000,
        }
    }
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
}

/// Streaming analyzer.
pub struct StreamingAnalyzer {
    config: StreamConfig,
    buffer: Arc<RwLock<VecDeque<AuditRecord>>>,
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
        }
    }

    /// Processes a new record.
    pub fn process(&self, record: AuditRecord) -> AuditResult<()> {
        let mut buffer = self.buffer.write().unwrap();
        buffer.push_back(record);

        // Trim old records
        let cutoff = Utc::now() - Duration::seconds(self.config.window_size_seconds);
        while buffer.front().map_or(false, |r| r.timestamp < cutoff) {
            buffer.pop_front();
        }

        // Enforce max size
        if buffer.len() > self.config.max_buffer_size {
            buffer.pop_front();
        }

        Ok(())
    }

    /// Gets current window metrics.
    pub fn current_metrics(&self) -> StreamingMetrics {
        let buffer = self.buffer.read().unwrap();
        let now = Utc::now();
        let window_start = now - Duration::seconds(self.config.window_size_seconds);

        let count = buffer.len();
        let override_count = buffer
            .iter()
            .filter(|r| matches!(r.result, crate::DecisionResult::Overridden { .. }))
            .count();

        let rate = count as f64 / self.config.window_size_seconds as f64;
        let override_pct = if count > 0 {
            (override_count as f64 / count as f64) * 100.0
        } else {
            0.0
        };

        StreamingMetrics {
            window_start,
            window_end: now,
            count,
            rate,
            override_pct,
        }
    }

    /// Returns configuration.
    pub fn config(&self) -> &StreamConfig {
        &self.config
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
    use uuid::Uuid;

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

    #[test]
    fn test_streaming_analyzer() {
        let analyzer = StreamingAnalyzer::new();

        for _ in 0..10 {
            analyzer.process(create_test_record()).unwrap();
        }

        let metrics = analyzer.current_metrics();
        assert_eq!(metrics.count, 10);
    }
}
