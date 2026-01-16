//! New Relic integration for audit trail export.
//!
//! This module provides integration with New Relic for events and logs.

use crate::{AuditError, AuditRecord, AuditResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for New Relic API.
#[derive(Debug, Clone)]
pub struct NewRelicConfig {
    /// New Relic account ID
    pub account_id: String,
    /// New Relic Insights insert key (for Events API)
    pub insert_key: String,
    /// New Relic license key (for Logs API)
    pub license_key: String,
    /// API endpoint region (US or EU)
    pub region: NewRelicRegion,
    /// Application name
    pub app_name: String,
}

/// New Relic API region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewRelicRegion {
    /// US region
    US,
    /// EU region
    EU,
}

impl NewRelicRegion {
    /// Returns the Events API endpoint for the region.
    pub fn events_endpoint(&self) -> &'static str {
        match self {
            NewRelicRegion::US => "https://insights-collector.newrelic.com/v1/accounts",
            NewRelicRegion::EU => "https://insights-collector.eu01.nr-data.net/v1/accounts",
        }
    }

    /// Returns the Logs API endpoint for the region.
    pub fn logs_endpoint(&self) -> &'static str {
        match self {
            NewRelicRegion::US => "https://log-api.newrelic.com/log/v1",
            NewRelicRegion::EU => "https://log-api.eu.newrelic.com/log/v1",
        }
    }
}

impl Default for NewRelicConfig {
    fn default() -> Self {
        Self {
            account_id: String::new(),
            insert_key: String::new(),
            license_key: String::new(),
            region: NewRelicRegion::US,
            app_name: "legalis-audit".to_string(),
        }
    }
}

impl NewRelicConfig {
    /// Creates a new New Relic configuration.
    pub fn new(account_id: String, insert_key: String, license_key: String) -> Self {
        Self {
            account_id,
            insert_key,
            license_key,
            ..Default::default()
        }
    }

    /// Sets the region.
    pub fn with_region(mut self, region: NewRelicRegion) -> Self {
        self.region = region;
        self
    }

    /// Sets the application name.
    pub fn with_app_name(mut self, app_name: String) -> Self {
        self.app_name = app_name;
        self
    }
}

/// New Relic custom event format.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct NewRelicEvent {
    #[serde(rename = "eventType")]
    event_type: String,
    /// Unix timestamp in seconds
    timestamp: i64,
    /// Event attributes
    #[serde(flatten)]
    attributes: HashMap<String, serde_json::Value>,
}

/// New Relic log entry format.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct NewRelicLog {
    /// Message
    message: String,
    /// Unix timestamp in milliseconds
    timestamp: i64,
    /// Log attributes
    #[serde(flatten)]
    attributes: HashMap<String, serde_json::Value>,
}

/// New Relic exporter for audit records.
pub struct NewRelicExporter {
    config: NewRelicConfig,
}

impl NewRelicExporter {
    /// Creates a new New Relic exporter.
    pub fn new(config: NewRelicConfig) -> Self {
        Self { config }
    }

    /// Converts an audit record to New Relic custom event format.
    fn to_newrelic_event(&self, record: &AuditRecord) -> NewRelicEvent {
        let mut attributes = HashMap::new();
        attributes.insert("auditId".to_string(), serde_json::json!(record.id));
        attributes.insert(
            "auditEventType".to_string(),
            serde_json::json!(match record.event_type {
                crate::EventType::AutomaticDecision => "AutomaticDecision",
                crate::EventType::DiscretionaryReview => "DiscretionaryReview",
                crate::EventType::HumanOverride => "HumanOverride",
                crate::EventType::Appeal => "Appeal",
                crate::EventType::StatuteModified => "StatuteModified",
                crate::EventType::SimulationRun => "SimulationRun",
            }),
        );
        attributes.insert("actor".to_string(), serde_json::json!(record.actor));
        attributes.insert(
            "statuteId".to_string(),
            serde_json::json!(&record.statute_id),
        );
        attributes.insert(
            "subjectId".to_string(),
            serde_json::json!(record.subject_id),
        );
        attributes.insert(
            "recordHash".to_string(),
            serde_json::json!(&record.record_hash),
        );
        attributes.insert(
            "appName".to_string(),
            serde_json::json!(&self.config.app_name),
        );

        if let Some(prev_hash) = &record.previous_hash {
            attributes.insert("previousHash".to_string(), serde_json::json!(prev_hash));
        }

        NewRelicEvent {
            event_type: "LegalisAuditEvent".to_string(),
            timestamp: record.timestamp.timestamp(),
            attributes,
        }
    }

    /// Converts an audit record to New Relic log format.
    fn to_newrelic_log(&self, record: &AuditRecord) -> NewRelicLog {
        let message = format!(
            "Audit event: {} for statute {} by {:?}",
            match record.event_type {
                crate::EventType::AutomaticDecision => "AutomaticDecision",
                crate::EventType::DiscretionaryReview => "DiscretionaryReview",
                crate::EventType::HumanOverride => "HumanOverride",
                crate::EventType::Appeal => "Appeal",
                crate::EventType::StatuteModified => "StatuteModified",
                crate::EventType::SimulationRun => "SimulationRun",
            },
            record.statute_id,
            record.actor
        );

        let mut attributes = HashMap::new();
        attributes.insert("auditId".to_string(), serde_json::json!(record.id));
        attributes.insert(
            "statuteId".to_string(),
            serde_json::json!(&record.statute_id),
        );
        attributes.insert(
            "subjectId".to_string(),
            serde_json::json!(record.subject_id),
        );
        attributes.insert(
            "recordHash".to_string(),
            serde_json::json!(&record.record_hash),
        );
        attributes.insert(
            "appName".to_string(),
            serde_json::json!(&self.config.app_name),
        );
        attributes.insert("logtype".to_string(), serde_json::json!("legalis-audit"));

        NewRelicLog {
            message,
            timestamp: record.timestamp.timestamp_millis(),
            attributes,
        }
    }

    /// Exports a single audit record to New Relic event JSON format.
    pub fn export_event(&self, record: &AuditRecord) -> AuditResult<String> {
        let event = self.to_newrelic_event(record);
        serde_json::to_string(&event).map_err(|e| AuditError::ExportError(e.to_string()))
    }

    /// Exports multiple audit records to New Relic event JSON format.
    pub fn export_events(&self, records: &[AuditRecord]) -> AuditResult<String> {
        let events: Vec<NewRelicEvent> = records
            .iter()
            .map(|record| self.to_newrelic_event(record))
            .collect();

        serde_json::to_string(&events).map_err(|e| AuditError::ExportError(e.to_string()))
    }

    /// Exports a single audit record to New Relic log JSON format.
    pub fn export_log(&self, record: &AuditRecord) -> AuditResult<String> {
        let log = self.to_newrelic_log(record);
        serde_json::to_string(&log).map_err(|e| AuditError::ExportError(e.to_string()))
    }

    /// Exports multiple audit records to New Relic log JSON format.
    pub fn export_logs(&self, records: &[AuditRecord]) -> AuditResult<String> {
        let logs: Vec<NewRelicLog> = records
            .iter()
            .map(|record| self.to_newrelic_log(record))
            .collect();

        let payload = serde_json::json!([{
            "logs": logs
        }]);

        serde_json::to_string(&payload).map_err(|e| AuditError::ExportError(e.to_string()))
    }

    /// Returns the Events API endpoint URL.
    pub fn events_endpoint_url(&self) -> String {
        format!(
            "{}/{}/events",
            self.config.region.events_endpoint(),
            self.config.account_id
        )
    }

    /// Returns the Logs API endpoint URL.
    pub fn logs_endpoint_url(&self) -> String {
        self.config.region.logs_endpoint().to_string()
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
    fn test_newrelic_config_default() {
        let config = NewRelicConfig::default();
        assert_eq!(config.region, NewRelicRegion::US);
        assert_eq!(config.app_name, "legalis-audit");
    }

    #[test]
    fn test_newrelic_config_builder() {
        let config = NewRelicConfig::new(
            "account123".to_string(),
            "insert_key".to_string(),
            "license_key".to_string(),
        )
        .with_region(NewRelicRegion::EU)
        .with_app_name("custom-app".to_string());

        assert_eq!(config.region, NewRelicRegion::EU);
        assert_eq!(config.app_name, "custom-app");
    }

    #[test]
    fn test_region_endpoints() {
        let us_region = NewRelicRegion::US;
        assert!(us_region.events_endpoint().contains("newrelic.com"));
        assert!(us_region.logs_endpoint().contains("newrelic.com"));

        let eu_region = NewRelicRegion::EU;
        assert!(eu_region.events_endpoint().contains("eu01.nr-data.net"));
        assert!(eu_region.logs_endpoint().contains("eu.newrelic.com"));
    }

    #[test]
    fn test_export_single_event() {
        let config = NewRelicConfig::new(
            "account123".to_string(),
            "insert_key".to_string(),
            "license_key".to_string(),
        );
        let exporter = NewRelicExporter::new(config);
        let record = create_test_record();

        let result = exporter.export_event(&record);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert!(json.contains("\"eventType\":\"LegalisAuditEvent\""));
        assert!(json.contains("\"statute-123\""));
    }

    #[test]
    fn test_export_multiple_events() {
        let config = NewRelicConfig::new(
            "account123".to_string(),
            "insert_key".to_string(),
            "license_key".to_string(),
        );
        let exporter = NewRelicExporter::new(config);

        let records: Vec<AuditRecord> = (0..3).map(|_| create_test_record()).collect();

        let result = exporter.export_events(&records);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert!(json.starts_with('['));
        assert!(json.ends_with(']'));
        assert!(json.contains("\"eventType\":\"LegalisAuditEvent\""));
    }

    #[test]
    fn test_export_single_log() {
        let config = NewRelicConfig::new(
            "account123".to_string(),
            "insert_key".to_string(),
            "license_key".to_string(),
        );
        let exporter = NewRelicExporter::new(config);
        let record = create_test_record();

        let result = exporter.export_log(&record);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert!(json.contains("\"message\""));
        assert!(json.contains("\"timestamp\""));
    }

    #[test]
    fn test_export_multiple_logs() {
        let config = NewRelicConfig::new(
            "account123".to_string(),
            "insert_key".to_string(),
            "license_key".to_string(),
        );
        let exporter = NewRelicExporter::new(config);

        let records: Vec<AuditRecord> = (0..3).map(|_| create_test_record()).collect();

        let result = exporter.export_logs(&records);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert!(json.contains("\"logs\""));
    }

    #[test]
    fn test_newrelic_event_structure() {
        let config = NewRelicConfig::default();
        let exporter = NewRelicExporter::new(config);
        let record = create_test_record();

        let event = exporter.to_newrelic_event(&record);

        assert_eq!(event.event_type, "LegalisAuditEvent");
        assert_eq!(event.timestamp, record.timestamp.timestamp());
        assert!(event.attributes.contains_key("auditId"));
        assert!(event.attributes.contains_key("statuteId"));
    }

    #[test]
    fn test_newrelic_log_structure() {
        let config = NewRelicConfig::default();
        let exporter = NewRelicExporter::new(config);
        let record = create_test_record();

        let log = exporter.to_newrelic_log(&record);

        assert!(!log.message.is_empty());
        assert_eq!(log.timestamp, record.timestamp.timestamp_millis());
        assert!(log.attributes.contains_key("auditId"));
        assert!(log.attributes.contains_key("logtype"));
    }

    #[test]
    fn test_endpoint_urls() {
        let config = NewRelicConfig::new(
            "account123".to_string(),
            "insert_key".to_string(),
            "license_key".to_string(),
        );
        let exporter = NewRelicExporter::new(config);

        let events_url = exporter.events_endpoint_url();
        assert!(events_url.contains("account123"));
        assert!(events_url.contains("events"));

        let logs_url = exporter.logs_endpoint_url();
        assert!(logs_url.contains("log-api"));
    }

    #[test]
    fn test_event_json_format() {
        let config = NewRelicConfig::default();
        let exporter = NewRelicExporter::new(config);
        let record = create_test_record();

        let json = exporter.export_event(&record).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed.get("eventType").is_some());
        assert!(parsed.get("timestamp").is_some());
        assert!(parsed.get("auditId").is_some());
        assert!(parsed.get("statuteId").is_some());
    }

    #[test]
    fn test_log_json_format() {
        let config = NewRelicConfig::default();
        let exporter = NewRelicExporter::new(config);
        let record = create_test_record();

        let json = exporter.export_log(&record).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed.get("message").is_some());
        assert!(parsed.get("timestamp").is_some());
        assert!(parsed.get("auditId").is_some());
        assert!(parsed.get("logtype").is_some());
    }
}
