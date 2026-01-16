//! Splunk integration for audit trail export.
//!
//! This module provides integration with Splunk using the HTTP Event Collector (HEC).

use crate::{AuditError, AuditRecord, AuditResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for Splunk HTTP Event Collector.
#[derive(Debug, Clone)]
pub struct SplunkConfig {
    /// Splunk HEC endpoint URL (e.g., "https://splunk.example.com:8088/services/collector/event")
    pub hec_url: String,
    /// HEC authentication token
    pub hec_token: String,
    /// Source type for events (default: "legalis:audit")
    pub source_type: String,
    /// Source name (default: hostname)
    pub source: String,
    /// Index to send events to (optional)
    pub index: Option<String>,
    /// Host field (default: hostname)
    pub host: String,
}

impl Default for SplunkConfig {
    fn default() -> Self {
        let hostname = hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "unknown".to_string());

        Self {
            hec_url: String::new(),
            hec_token: String::new(),
            source_type: "legalis:audit".to_string(),
            source: hostname.clone(),
            index: None,
            host: hostname,
        }
    }
}

impl SplunkConfig {
    /// Creates a new Splunk configuration.
    pub fn new(hec_url: String, hec_token: String) -> Self {
        Self {
            hec_url,
            hec_token,
            ..Default::default()
        }
    }

    /// Sets the source type.
    pub fn with_source_type(mut self, source_type: String) -> Self {
        self.source_type = source_type;
        self
    }

    /// Sets the source.
    pub fn with_source(mut self, source: String) -> Self {
        self.source = source;
        self
    }

    /// Sets the index.
    pub fn with_index(mut self, index: String) -> Self {
        self.index = Some(index);
        self
    }

    /// Sets the host.
    pub fn with_host(mut self, host: String) -> Self {
        self.host = host;
        self
    }
}

/// Splunk HEC event format.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SplunkEvent {
    /// Event timestamp (epoch time in seconds)
    time: i64,
    /// Host field
    host: String,
    /// Source field
    source: String,
    /// Source type field
    sourcetype: String,
    /// Index (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<String>,
    /// Event data
    event: serde_json::Value,
}

/// Splunk exporter for audit records.
pub struct SplunkExporter {
    config: SplunkConfig,
}

impl SplunkExporter {
    /// Creates a new Splunk exporter.
    pub fn new(config: SplunkConfig) -> Self {
        Self { config }
    }

    /// Converts an audit record to Splunk HEC event format.
    fn to_splunk_event(&self, record: &AuditRecord) -> SplunkEvent {
        let mut event_data = HashMap::new();
        event_data.insert("id", serde_json::json!(record.id));
        event_data.insert("event_type", serde_json::json!(record.event_type));
        event_data.insert("actor", serde_json::json!(record.actor));
        event_data.insert("statute_id", serde_json::json!(&record.statute_id));
        event_data.insert("subject_id", serde_json::json!(record.subject_id));
        event_data.insert("context", serde_json::json!(&record.context));
        event_data.insert("result", serde_json::json!(&record.result));
        event_data.insert("record_hash", serde_json::json!(&record.record_hash));

        if let Some(prev_hash) = &record.previous_hash {
            event_data.insert("previous_hash", serde_json::json!(prev_hash));
        }

        SplunkEvent {
            time: record.timestamp.timestamp(),
            host: self.config.host.clone(),
            source: self.config.source.clone(),
            sourcetype: self.config.source_type.clone(),
            index: self.config.index.clone(),
            event: serde_json::json!(event_data),
        }
    }

    /// Exports a single audit record to Splunk HEC JSON format.
    pub fn export_record(&self, record: &AuditRecord) -> AuditResult<String> {
        let event = self.to_splunk_event(record);
        serde_json::to_string(&event).map_err(|e| AuditError::ExportError(e.to_string()))
    }

    /// Exports multiple audit records to Splunk HEC JSON format (batch).
    pub fn export_records(&self, records: &[AuditRecord]) -> AuditResult<Vec<String>> {
        records
            .iter()
            .map(|record| self.export_record(record))
            .collect()
    }

    /// Exports multiple audit records as a single batch payload.
    pub fn export_batch(&self, records: &[AuditRecord]) -> AuditResult<String> {
        let events: Vec<String> = self.export_records(records)?;
        Ok(events.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;
    use uuid::Uuid;

    fn create_test_record() -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "statute-123".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_splunk_config_default() {
        let config = SplunkConfig::default();
        assert_eq!(config.source_type, "legalis:audit");
        assert!(!config.source.is_empty());
        assert!(!config.host.is_empty());
    }

    #[test]
    fn test_splunk_config_builder() {
        let config = SplunkConfig::new(
            "https://splunk.example.com:8088/services/collector/event".to_string(),
            "token123".to_string(),
        )
        .with_source_type("custom:audit".to_string())
        .with_index("audit_index".to_string())
        .with_host("testhost".to_string());

        assert_eq!(config.source_type, "custom:audit");
        assert_eq!(config.index, Some("audit_index".to_string()));
        assert_eq!(config.host, "testhost");
    }

    #[test]
    fn test_export_single_record() {
        let config = SplunkConfig::new(
            "https://splunk.example.com:8088/services/collector/event".to_string(),
            "token123".to_string(),
        );
        let exporter = SplunkExporter::new(config);
        let record = create_test_record();

        let result = exporter.export_record(&record);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert!(json.contains("\"sourcetype\":\"legalis:audit\""));
        assert!(json.contains("\"statute-123\""));
    }

    #[test]
    fn test_export_multiple_records() {
        let config = SplunkConfig::new(
            "https://splunk.example.com:8088/services/collector/event".to_string(),
            "token123".to_string(),
        );
        let exporter = SplunkExporter::new(config);

        let records: Vec<AuditRecord> = (0..3).map(|_| create_test_record()).collect();

        let result = exporter.export_records(&records);
        assert!(result.is_ok());

        let events = result.unwrap();
        assert_eq!(events.len(), 3);

        for event in events {
            assert!(event.contains("\"sourcetype\":\"legalis:audit\""));
        }
    }

    #[test]
    fn test_export_batch() {
        let config = SplunkConfig::new(
            "https://splunk.example.com:8088/services/collector/event".to_string(),
            "token123".to_string(),
        );
        let exporter = SplunkExporter::new(config);

        let records: Vec<AuditRecord> = (0..3).map(|_| create_test_record()).collect();

        let result = exporter.export_batch(&records);
        assert!(result.is_ok());

        let batch = result.unwrap();
        // Batch should contain newline-separated JSON events
        assert_eq!(batch.matches('\n').count(), 2); // 3 events = 2 newlines
        assert!(batch.contains("\"sourcetype\":\"legalis:audit\""));
    }

    #[test]
    fn test_splunk_event_structure() {
        let config = SplunkConfig::new(
            "https://splunk.example.com:8088/services/collector/event".to_string(),
            "token123".to_string(),
        )
        .with_index("test_index".to_string());

        let exporter = SplunkExporter::new(config);
        let record = create_test_record();

        let event = exporter.to_splunk_event(&record);

        assert_eq!(event.time, record.timestamp.timestamp());
        assert_eq!(event.sourcetype, "legalis:audit");
        assert_eq!(event.index, Some("test_index".to_string()));
        assert!(event.event.is_object());
    }

    #[test]
    fn test_splunk_event_json_format() {
        let config = SplunkConfig::default();
        let exporter = SplunkExporter::new(config);
        let record = create_test_record();

        let json = exporter.export_record(&record).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed.get("time").is_some());
        assert!(parsed.get("host").is_some());
        assert!(parsed.get("source").is_some());
        assert!(parsed.get("sourcetype").is_some());
        assert!(parsed.get("event").is_some());

        let event = parsed.get("event").unwrap();
        assert!(event.get("id").is_some());
        assert!(event.get("statute_id").is_some());
        assert!(event.get("record_hash").is_some());
    }
}
