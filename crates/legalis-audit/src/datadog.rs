//! Datadog integration for audit trail export.
//!
//! This module provides integration with Datadog for logs and metrics.

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for Datadog API.
#[derive(Debug, Clone)]
pub struct DatadogConfig {
    /// Datadog API endpoint (e.g., "https://http-intake.logs.datadoghq.com")
    pub api_endpoint: String,
    /// Datadog API key
    pub api_key: String,
    /// Service name for logs
    pub service: String,
    /// Environment (e.g., "production", "staging")
    pub environment: String,
    /// Hostname
    pub hostname: String,
    /// Additional tags
    pub tags: Vec<String>,
}

impl Default for DatadogConfig {
    fn default() -> Self {
        let hostname = hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "unknown".to_string());

        Self {
            api_endpoint: "https://http-intake.logs.datadoghq.com".to_string(),
            api_key: String::new(),
            service: "legalis-audit".to_string(),
            environment: "production".to_string(),
            hostname,
            tags: vec!["source:legalis".to_string()],
        }
    }
}

impl DatadogConfig {
    /// Creates a new Datadog configuration.
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            ..Default::default()
        }
    }

    /// Sets the service name.
    pub fn with_service(mut self, service: String) -> Self {
        self.service = service;
        self
    }

    /// Sets the environment.
    pub fn with_environment(mut self, environment: String) -> Self {
        self.environment = environment;
        self
    }

    /// Sets the hostname.
    pub fn with_hostname(mut self, hostname: String) -> Self {
        self.hostname = hostname;
        self
    }

    /// Adds tags.
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Adds a single tag.
    pub fn add_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }
}

/// Datadog log entry format.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DatadogLog {
    /// Timestamp in ISO 8601 format
    #[serde(rename = "ddsource")]
    dd_source: String,
    /// Service name
    #[serde(rename = "ddtags")]
    dd_tags: String,
    /// Hostname
    hostname: String,
    /// Message
    message: String,
    /// Service
    service: String,
    /// Status (info, warning, error)
    status: String,
    /// Timestamp
    timestamp: i64,
    /// Additional attributes
    #[serde(flatten)]
    attributes: HashMap<String, serde_json::Value>,
}

/// Datadog metric format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
struct DatadogMetric {
    /// Metric name
    metric: String,
    /// Metric type (gauge, count, rate)
    #[serde(rename = "type")]
    metric_type: String,
    /// Points (timestamp, value)
    points: Vec<(i64, f64)>,
    /// Tags
    tags: Vec<String>,
    /// Host
    host: String,
}

/// Datadog exporter for audit records.
pub struct DatadogExporter {
    config: DatadogConfig,
}

impl DatadogExporter {
    /// Creates a new Datadog exporter.
    pub fn new(config: DatadogConfig) -> Self {
        Self { config }
    }

    /// Converts an audit record to Datadog log format.
    fn to_datadog_log(&self, record: &AuditRecord) -> DatadogLog {
        let tags = format!(
            "env:{},{}",
            self.config.environment,
            self.config.tags.join(",")
        );

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
        attributes.insert("audit_id".to_string(), serde_json::json!(record.id));
        attributes.insert(
            "event_type".to_string(),
            serde_json::json!(record.event_type),
        );
        attributes.insert("actor".to_string(), serde_json::json!(record.actor));
        attributes.insert(
            "statute_id".to_string(),
            serde_json::json!(&record.statute_id),
        );
        attributes.insert(
            "subject_id".to_string(),
            serde_json::json!(record.subject_id),
        );
        attributes.insert("context".to_string(), serde_json::json!(&record.context));
        attributes.insert("result".to_string(), serde_json::json!(&record.result));
        attributes.insert(
            "record_hash".to_string(),
            serde_json::json!(&record.record_hash),
        );

        if let Some(prev_hash) = &record.previous_hash {
            attributes.insert("previous_hash".to_string(), serde_json::json!(prev_hash));
        }

        DatadogLog {
            dd_source: "legalis".to_string(),
            dd_tags: tags,
            hostname: self.config.hostname.clone(),
            message,
            service: self.config.service.clone(),
            status: "info".to_string(),
            timestamp: record.timestamp.timestamp_millis(),
            attributes,
        }
    }

    /// Exports a single audit record to Datadog log JSON format.
    pub fn export_log(&self, record: &AuditRecord) -> AuditResult<String> {
        let log = self.to_datadog_log(record);
        serde_json::to_string(&log).map_err(|e| AuditError::ExportError(e.to_string()))
    }

    /// Exports multiple audit records to Datadog log JSON format.
    pub fn export_logs(&self, records: &[AuditRecord]) -> AuditResult<Vec<String>> {
        records
            .iter()
            .map(|record| self.export_log(record))
            .collect()
    }

    /// Creates a metric for audit event count.
    #[allow(dead_code)]
    fn create_count_metric(
        &self,
        metric_name: &str,
        count: f64,
        timestamp: DateTime<Utc>,
        tags: Vec<String>,
    ) -> DatadogMetric {
        DatadogMetric {
            metric: metric_name.to_string(),
            metric_type: "count".to_string(),
            points: vec![(timestamp.timestamp(), count)],
            tags,
            host: self.config.hostname.clone(),
        }
    }

    /// Creates a gauge metric.
    #[allow(dead_code)]
    fn create_gauge_metric(
        &self,
        metric_name: &str,
        value: f64,
        timestamp: DateTime<Utc>,
        tags: Vec<String>,
    ) -> DatadogMetric {
        DatadogMetric {
            metric: metric_name.to_string(),
            metric_type: "gauge".to_string(),
            points: vec![(timestamp.timestamp(), value)],
            tags,
            host: self.config.hostname.clone(),
        }
    }

    /// Exports a metric to JSON format.
    #[allow(dead_code)]
    fn export_metric(&self, metric: &DatadogMetric) -> AuditResult<String> {
        serde_json::to_string(metric).map_err(|e| AuditError::ExportError(e.to_string()))
    }

    /// Exports multiple metrics to JSON format.
    #[allow(dead_code)]
    fn export_metrics(&self, metrics: &[DatadogMetric]) -> AuditResult<String> {
        let payload = serde_json::json!({
            "series": metrics
        });
        serde_json::to_string(&payload).map_err(|e| AuditError::ExportError(e.to_string()))
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
    fn test_datadog_config_default() {
        let config = DatadogConfig::default();
        assert_eq!(config.service, "legalis-audit");
        assert_eq!(config.environment, "production");
        assert!(!config.hostname.is_empty());
    }

    #[test]
    fn test_datadog_config_builder() {
        let config = DatadogConfig::new("api_key_123".to_string())
            .with_service("custom-service".to_string())
            .with_environment("staging".to_string())
            .add_tag("team:legal".to_string());

        assert_eq!(config.service, "custom-service");
        assert_eq!(config.environment, "staging");
        assert!(config.tags.contains(&"team:legal".to_string()));
    }

    #[test]
    fn test_export_single_log() {
        let config = DatadogConfig::new("api_key_123".to_string());
        let exporter = DatadogExporter::new(config);
        let record = create_test_record();

        let result = exporter.export_log(&record);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert!(json.contains("\"service\":\"legalis-audit\""));
        assert!(json.contains("\"statute-123\""));
    }

    #[test]
    fn test_export_multiple_logs() {
        let config = DatadogConfig::new("api_key_123".to_string());
        let exporter = DatadogExporter::new(config);

        let records: Vec<AuditRecord> = (0..3).map(|_| create_test_record()).collect();

        let result = exporter.export_logs(&records);
        assert!(result.is_ok());

        let logs = result.unwrap();
        assert_eq!(logs.len(), 3);

        for log in logs {
            assert!(log.contains("\"service\":\"legalis-audit\""));
        }
    }

    #[test]
    fn test_datadog_log_structure() {
        let config =
            DatadogConfig::new("api_key_123".to_string()).with_environment("test".to_string());

        let exporter = DatadogExporter::new(config);
        let record = create_test_record();

        let log = exporter.to_datadog_log(&record);

        assert_eq!(log.dd_source, "legalis");
        assert!(log.dd_tags.contains("env:test"));
        assert_eq!(log.service, "legalis-audit");
        assert_eq!(log.status, "info");
        assert!(log.attributes.contains_key("audit_id"));
    }

    #[test]
    fn test_create_count_metric() {
        let config = DatadogConfig::new("api_key_123".to_string());
        let exporter = DatadogExporter::new(config);
        let timestamp = Utc::now();

        let metric = exporter.create_count_metric(
            "legalis.audit.events",
            10.0,
            timestamp,
            vec!["event_type:automatic".to_string()],
        );

        assert_eq!(metric.metric, "legalis.audit.events");
        assert_eq!(metric.metric_type, "count");
        assert_eq!(metric.points.len(), 1);
        assert_eq!(metric.points[0].1, 10.0);
        assert!(metric.tags.contains(&"event_type:automatic".to_string()));
    }

    #[test]
    fn test_create_gauge_metric() {
        let config = DatadogConfig::new("api_key_123".to_string());
        let exporter = DatadogExporter::new(config);
        let timestamp = Utc::now();

        let metric = exporter.create_gauge_metric(
            "legalis.audit.trail_size",
            1000.0,
            timestamp,
            vec!["environment:production".to_string()],
        );

        assert_eq!(metric.metric, "legalis.audit.trail_size");
        assert_eq!(metric.metric_type, "gauge");
        assert_eq!(metric.points.len(), 1);
        assert_eq!(metric.points[0].1, 1000.0);
    }

    #[test]
    fn test_export_metric() {
        let config = DatadogConfig::new("api_key_123".to_string());
        let exporter = DatadogExporter::new(config);
        let timestamp = Utc::now();

        let metric = exporter.create_count_metric(
            "legalis.audit.events",
            5.0,
            timestamp,
            vec!["test:true".to_string()],
        );

        let result = exporter.export_metric(&metric);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert!(json.contains("\"metric\":\"legalis.audit.events\""));
        assert!(json.contains("\"type\":\"count\""));
    }

    #[test]
    fn test_export_metrics_batch() {
        let config = DatadogConfig::new("api_key_123".to_string());
        let exporter = DatadogExporter::new(config);
        let timestamp = Utc::now();

        let metrics = vec![
            exporter.create_count_metric("metric1", 1.0, timestamp, vec![]),
            exporter.create_count_metric("metric2", 2.0, timestamp, vec![]),
        ];

        let result = exporter.export_metrics(&metrics);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert!(json.contains("\"series\""));
        assert!(json.contains("\"metric1\""));
        assert!(json.contains("\"metric2\""));
    }

    #[test]
    fn test_datadog_log_json_format() {
        let config = DatadogConfig::default();
        let exporter = DatadogExporter::new(config);
        let record = create_test_record();

        let json = exporter.export_log(&record).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed.get("ddsource").is_some());
        assert!(parsed.get("ddtags").is_some());
        assert!(parsed.get("hostname").is_some());
        assert!(parsed.get("message").is_some());
        assert!(parsed.get("service").is_some());
        assert!(parsed.get("status").is_some());
        assert!(parsed.get("timestamp").is_some());
        assert!(parsed.get("audit_id").is_some());
    }
}
