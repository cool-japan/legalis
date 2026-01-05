//! Elasticsearch export for audit trails.
//!
//! This module provides functionality to export audit records to Elasticsearch
//! using the Bulk API format.

use crate::{AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Elasticsearch export configuration.
#[derive(Debug, Clone)]
pub struct ElasticsearchConfig {
    /// Index name
    pub index: String,
    /// Index prefix (e.g., "audit-log-")
    pub index_prefix: Option<String>,
    /// Use time-based indices (e.g., "audit-log-2024.01")
    pub time_based_indices: bool,
    /// Document type (deprecated in ES 7.x+, kept for compatibility)
    pub doc_type: Option<String>,
}

impl Default for ElasticsearchConfig {
    fn default() -> Self {
        Self {
            index: "audit-trail".to_string(),
            index_prefix: None,
            time_based_indices: false,
            doc_type: None,
        }
    }
}

impl ElasticsearchConfig {
    /// Creates a new configuration with the given index name.
    pub fn new(index: impl Into<String>) -> Self {
        Self {
            index: index.into(),
            ..Default::default()
        }
    }

    /// Sets the index prefix for time-based indices.
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.index_prefix = Some(prefix.into());
        self
    }

    /// Enables time-based indices (e.g., "audit-log-2024.01").
    pub fn with_time_based_indices(mut self) -> Self {
        self.time_based_indices = true;
        self
    }

    /// Gets the index name for a specific timestamp.
    fn get_index_name(&self, timestamp: &DateTime<Utc>) -> String {
        if self.time_based_indices {
            let prefix = self.index_prefix.as_deref().unwrap_or(&self.index);
            format!("{}-{}", prefix, timestamp.format("%Y.%m"))
        } else {
            self.index.clone()
        }
    }
}

/// Elasticsearch document for an audit record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElasticsearchDocument {
    /// Record ID
    pub id: String,
    /// Timestamp
    #[serde(rename = "@timestamp")]
    pub timestamp: DateTime<Utc>,
    /// Event type
    pub event_type: String,
    /// Actor information
    pub actor: serde_json::Value,
    /// Statute ID
    pub statute_id: String,
    /// Subject ID
    pub subject_id: String,
    /// Decision context
    pub context: serde_json::Value,
    /// Decision result
    pub result: serde_json::Value,
    /// Previous hash
    pub previous_hash: Option<String>,
    /// Record hash
    pub record_hash: String,
}

impl From<&AuditRecord> for ElasticsearchDocument {
    fn from(record: &AuditRecord) -> Self {
        Self {
            id: record.id.to_string(),
            timestamp: record.timestamp,
            event_type: format!("{:?}", record.event_type),
            actor: serde_json::to_value(&record.actor).unwrap_or_default(),
            statute_id: record.statute_id.clone(),
            subject_id: record.subject_id.to_string(),
            context: serde_json::to_value(&record.context).unwrap_or_default(),
            result: serde_json::to_value(&record.result).unwrap_or_default(),
            previous_hash: record.previous_hash.clone(),
            record_hash: record.record_hash.clone(),
        }
    }
}

/// Elasticsearch bulk API exporter.
pub struct ElasticsearchExporter {
    config: ElasticsearchConfig,
}

impl ElasticsearchExporter {
    /// Creates a new Elasticsearch exporter with the given configuration.
    pub fn new(config: ElasticsearchConfig) -> Self {
        Self { config }
    }

    /// Creates a new Elasticsearch exporter with default configuration.
    pub fn with_index(index: impl Into<String>) -> Self {
        Self::new(ElasticsearchConfig::new(index))
    }

    /// Exports a single audit record as a bulk API operation.
    pub fn export_record(&self, record: &AuditRecord) -> AuditResult<String> {
        let index_name = self.config.get_index_name(&record.timestamp);
        let doc = ElasticsearchDocument::from(record);

        let action = json!({
            "index": {
                "_index": index_name,
                "_id": record.id.to_string(),
            }
        });

        let document = serde_json::to_value(&doc)?;

        Ok(format!(
            "{}\n{}\n",
            serde_json::to_string(&action)?,
            serde_json::to_string(&document)?
        ))
    }

    /// Exports multiple audit records as bulk API operations.
    pub fn export_bulk(&self, records: &[AuditRecord]) -> AuditResult<String> {
        let mut bulk = String::new();
        for record in records {
            bulk.push_str(&self.export_record(record)?);
        }
        Ok(bulk)
    }

    /// Exports records as newline-delimited JSON (NDJSON).
    pub fn export_ndjson(&self, records: &[AuditRecord]) -> AuditResult<String> {
        let mut ndjson = String::new();
        for record in records {
            let doc = ElasticsearchDocument::from(record);
            ndjson.push_str(&serde_json::to_string(&doc)?);
            ndjson.push('\n');
        }
        Ok(ndjson)
    }

    /// Exports records as a JSON array (for debugging).
    pub fn export_json_array(&self, records: &[AuditRecord]) -> AuditResult<serde_json::Value> {
        let docs: Vec<ElasticsearchDocument> = records.iter().map(|r| r.into()).collect();
        Ok(serde_json::to_value(&docs)?)
    }

    /// Creates an Elasticsearch index template for audit records.
    pub fn create_index_template(&self) -> serde_json::Value {
        json!({
            "index_patterns": [format!("{}*", self.config.index)],
            "settings": {
                "number_of_shards": 1,
                "number_of_replicas": 1,
                "index": {
                    "refresh_interval": "5s"
                }
            },
            "mappings": {
                "properties": {
                    "@timestamp": { "type": "date" },
                    "id": { "type": "keyword" },
                    "event_type": { "type": "keyword" },
                    "statute_id": { "type": "keyword" },
                    "subject_id": { "type": "keyword" },
                    "record_hash": { "type": "keyword" },
                    "previous_hash": { "type": "keyword" },
                    "actor": { "type": "object" },
                    "context": { "type": "object" },
                    "result": { "type": "object" }
                }
            }
        })
    }
}

/// Elasticsearch query builder for searching audit records.
#[derive(Debug, Clone, Default)]
pub struct ElasticsearchQuery {
    must: Vec<serde_json::Value>,
    filter: Vec<serde_json::Value>,
    range: Option<serde_json::Value>,
    size: usize,
    from: usize,
}

impl ElasticsearchQuery {
    /// Creates a new Elasticsearch query.
    pub fn new() -> Self {
        Self {
            must: Vec::new(),
            filter: Vec::new(),
            range: None,
            size: 100,
            from: 0,
        }
    }

    /// Adds a statute ID filter.
    pub fn statute_id(mut self, statute_id: impl Into<String>) -> Self {
        self.filter.push(json!({
            "term": {
                "statute_id": statute_id.into()
            }
        }));
        self
    }

    /// Adds a subject ID filter.
    pub fn subject_id(mut self, subject_id: impl Into<String>) -> Self {
        self.filter.push(json!({
            "term": {
                "subject_id": subject_id.into()
            }
        }));
        self
    }

    /// Adds an event type filter.
    pub fn event_type(mut self, event_type: impl Into<String>) -> Self {
        self.filter.push(json!({
            "term": {
                "event_type": event_type.into()
            }
        }));
        self
    }

    /// Adds a time range filter.
    pub fn time_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.range = Some(json!({
            "range": {
                "@timestamp": {
                    "gte": start.to_rfc3339(),
                    "lte": end.to_rfc3339()
                }
            }
        }));
        self
    }

    /// Sets the result size (default: 100).
    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }

    /// Sets the offset for pagination.
    pub fn from(mut self, from: usize) -> Self {
        self.from = from;
        self
    }

    /// Builds the Elasticsearch query JSON.
    pub fn build(&self) -> serde_json::Value {
        let mut bool_query = json!({});

        if !self.must.is_empty() {
            bool_query["must"] = json!(self.must);
        }

        if !self.filter.is_empty() {
            bool_query["filter"] = json!(self.filter);
        }

        if let Some(ref range) = self.range {
            bool_query["filter"] = if bool_query.get("filter").is_some() {
                let mut filters = self.filter.clone();
                filters.push(range.clone());
                json!(filters)
            } else {
                json!([range])
            };
        }

        json!({
            "query": {
                "bool": bool_query
            },
            "size": self.size,
            "from": self.from,
            "sort": [
                { "@timestamp": { "order": "desc" } }
            ]
        })
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
                component: "test-engine".to_string(),
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
    fn test_elasticsearch_document_conversion() {
        let record = create_test_record();
        let doc = ElasticsearchDocument::from(&record);

        assert_eq!(doc.id, record.id.to_string());
        assert_eq!(doc.timestamp, record.timestamp);
        assert_eq!(doc.statute_id, record.statute_id);
        assert_eq!(doc.subject_id, record.subject_id.to_string());
    }

    #[test]
    fn test_export_single_record() {
        let exporter = ElasticsearchExporter::with_index("audit-log");
        let record = create_test_record();

        let bulk = exporter.export_record(&record).unwrap();
        assert!(bulk.contains("\"index\""));
        assert!(bulk.contains("audit-log"));
        assert!(bulk.contains(&record.id.to_string()));
    }

    #[test]
    fn test_export_bulk() {
        let exporter = ElasticsearchExporter::with_index("audit-log");
        let records = vec![create_test_record(), create_test_record()];

        let bulk = exporter.export_bulk(&records).unwrap();
        let lines: Vec<&str> = bulk.lines().collect();
        // Each record produces 2 lines (action + document)
        assert_eq!(lines.len(), 4);
    }

    #[test]
    fn test_export_ndjson() {
        let exporter = ElasticsearchExporter::with_index("audit-log");
        let records = vec![create_test_record(), create_test_record()];

        let ndjson = exporter.export_ndjson(&records).unwrap();
        let lines: Vec<&str> = ndjson.lines().collect();
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn test_time_based_indices() {
        let config = ElasticsearchConfig::new("audit-log")
            .with_prefix("audit")
            .with_time_based_indices();
        let exporter = ElasticsearchExporter::new(config);
        let record = create_test_record();

        let bulk = exporter.export_record(&record).unwrap();
        assert!(bulk.contains("audit-"));
        assert!(bulk.contains(&record.timestamp.format("%Y.%m").to_string()));
    }

    #[test]
    fn test_index_template() {
        let exporter = ElasticsearchExporter::with_index("audit-log");
        let template = exporter.create_index_template();

        assert!(template.get("index_patterns").is_some());
        assert!(template.get("mappings").is_some());
        assert!(template.get("settings").is_some());
    }

    #[test]
    fn test_query_builder() {
        let query = ElasticsearchQuery::new()
            .statute_id("statute-123")
            .event_type("AutomaticDecision")
            .size(50)
            .from(10)
            .build();

        assert!(query.get("query").is_some());
        assert_eq!(query["size"], 50);
        assert_eq!(query["from"], 10);
    }

    #[test]
    fn test_query_time_range() {
        let start = Utc::now();
        let end = start + chrono::Duration::hours(1);

        let query = ElasticsearchQuery::new().time_range(start, end).build();

        assert!(query["query"]["bool"]["filter"].is_array());
    }
}
