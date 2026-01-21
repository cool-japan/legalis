//! ServiceNow integration for audit trail export.
//!
//! This module provides integration with ServiceNow for incident management and table API.

use crate::{AuditError, AuditRecord, AuditResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for ServiceNow API.
#[derive(Debug, Clone, Default)]
pub struct ServiceNowConfig {
    /// ServiceNow instance URL (e.g., "<https://instance.service-now.com>")
    pub instance_url: String,
    /// Username for basic authentication
    pub username: String,
    /// Password for basic authentication
    pub password: String,
    /// Default caller ID (user sys_id)
    pub caller_id: Option<String>,
    /// Default assignment group
    pub assignment_group: Option<String>,
}

impl ServiceNowConfig {
    /// Creates a new ServiceNow configuration.
    pub fn new(instance_url: String, username: String, password: String) -> Self {
        Self {
            instance_url,
            username,
            password,
            caller_id: None,
            assignment_group: None,
        }
    }

    /// Sets the default caller ID.
    pub fn with_caller_id(mut self, caller_id: String) -> Self {
        self.caller_id = Some(caller_id);
        self
    }

    /// Sets the default assignment group.
    pub fn with_assignment_group(mut self, assignment_group: String) -> Self {
        self.assignment_group = Some(assignment_group);
        self
    }
}

/// ServiceNow incident severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncidentSeverity {
    /// Critical (1)
    Critical,
    /// High (2)
    High,
    /// Medium (3)
    Medium,
    /// Low (4)
    Low,
}

impl IncidentSeverity {
    /// Returns the ServiceNow impact value.
    pub fn impact(&self) -> i32 {
        match self {
            IncidentSeverity::Critical => 1,
            IncidentSeverity::High => 2,
            IncidentSeverity::Medium => 3,
            IncidentSeverity::Low => 4,
        }
    }

    /// Returns the ServiceNow urgency value.
    pub fn urgency(&self) -> i32 {
        match self {
            IncidentSeverity::Critical => 1,
            IncidentSeverity::High => 2,
            IncidentSeverity::Medium => 3,
            IncidentSeverity::Low => 4,
        }
    }
}

/// ServiceNow incident format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceNowIncident {
    /// Short description
    pub short_description: String,
    /// Description
    pub description: String,
    /// Impact (1-4)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<i32>,
    /// Urgency (1-4)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<i32>,
    /// Caller ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_id: Option<String>,
    /// Assignment group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_group: Option<String>,
    /// Category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Subcategory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
}

/// ServiceNow table entry format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceNowTableEntry {
    /// Table fields
    #[serde(flatten)]
    pub fields: HashMap<String, serde_json::Value>,
}

/// ServiceNow exporter for audit records.
pub struct ServiceNowExporter {
    config: ServiceNowConfig,
}

impl ServiceNowExporter {
    /// Creates a new ServiceNow exporter.
    pub fn new(config: ServiceNowConfig) -> Self {
        Self { config }
    }

    /// Converts an audit record to ServiceNow incident format.
    fn to_servicenow_incident(
        &self,
        record: &AuditRecord,
        severity: IncidentSeverity,
    ) -> ServiceNowIncident {
        let short_description = format!(
            "Audit Event: {} - {}",
            match record.event_type {
                crate::EventType::AutomaticDecision => "Automatic Decision",
                crate::EventType::DiscretionaryReview => "Discretionary Review",
                crate::EventType::HumanOverride => "Human Override",
                crate::EventType::Appeal => "Appeal",
                crate::EventType::StatuteModified => "Statute Modified",
                crate::EventType::SimulationRun => "Simulation Run",
            },
            record.statute_id
        );

        let description = format!(
            "Audit Record Details:\n\
             ID: {}\n\
             Timestamp: {}\n\
             Event Type: {:?}\n\
             Actor: {:?}\n\
             Statute ID: {}\n\
             Subject ID: {}\n\
             Record Hash: {}\n\
             Previous Hash: {}\n\
             \n\
             Context: {:?}\n\
             Result: {:?}",
            record.id,
            record.timestamp,
            record.event_type,
            record.actor,
            record.statute_id,
            record.subject_id,
            record.record_hash,
            record.previous_hash.as_ref().unwrap_or(&"None".to_string()),
            record.context,
            record.result
        );

        ServiceNowIncident {
            short_description,
            description,
            impact: Some(severity.impact()),
            urgency: Some(severity.urgency()),
            caller_id: self.config.caller_id.clone(),
            assignment_group: self.config.assignment_group.clone(),
            category: Some("Audit".to_string()),
            subcategory: Some("Legal Compliance".to_string()),
        }
    }

    /// Converts an audit record to ServiceNow table entry format.
    fn to_servicenow_table_entry(&self, record: &AuditRecord) -> ServiceNowTableEntry {
        let mut fields = HashMap::new();
        fields.insert("audit_id".to_string(), serde_json::json!(record.id));
        fields.insert(
            "timestamp".to_string(),
            serde_json::json!(record.timestamp.to_rfc3339()),
        );
        fields.insert(
            "event_type".to_string(),
            serde_json::json!(match record.event_type {
                crate::EventType::AutomaticDecision => "AutomaticDecision",
                crate::EventType::DiscretionaryReview => "DiscretionaryReview",
                crate::EventType::HumanOverride => "HumanOverride",
                crate::EventType::Appeal => "Appeal",
                crate::EventType::StatuteModified => "StatuteModified",
                crate::EventType::SimulationRun => "SimulationRun",
            }),
        );
        fields.insert("actor".to_string(), serde_json::json!(record.actor));
        fields.insert(
            "statute_id".to_string(),
            serde_json::json!(&record.statute_id),
        );
        fields.insert(
            "subject_id".to_string(),
            serde_json::json!(record.subject_id),
        );
        fields.insert(
            "record_hash".to_string(),
            serde_json::json!(&record.record_hash),
        );

        if let Some(prev_hash) = &record.previous_hash {
            fields.insert("previous_hash".to_string(), serde_json::json!(prev_hash));
        }

        ServiceNowTableEntry { fields }
    }

    /// Exports an audit record as a ServiceNow incident JSON.
    pub fn export_incident(
        &self,
        record: &AuditRecord,
        severity: IncidentSeverity,
    ) -> AuditResult<String> {
        let incident = self.to_servicenow_incident(record, severity);
        serde_json::to_string(&incident).map_err(|e| AuditError::ExportError(e.to_string()))
    }

    /// Exports an audit record as a ServiceNow table entry JSON.
    pub fn export_table_entry(&self, record: &AuditRecord) -> AuditResult<String> {
        let entry = self.to_servicenow_table_entry(record);
        serde_json::to_string(&entry).map_err(|e| AuditError::ExportError(e.to_string()))
    }

    /// Exports multiple audit records as ServiceNow table entries.
    pub fn export_table_entries(&self, records: &[AuditRecord]) -> AuditResult<Vec<String>> {
        records
            .iter()
            .map(|record| self.export_table_entry(record))
            .collect()
    }

    /// Returns the incident API endpoint URL.
    pub fn incident_endpoint(&self) -> String {
        format!("{}/api/now/table/incident", self.config.instance_url)
    }

    /// Returns the table API endpoint URL for a specific table.
    pub fn table_endpoint(&self, table_name: &str) -> String {
        format!("{}/api/now/table/{}", self.config.instance_url, table_name)
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
    fn test_servicenow_config() {
        let config = ServiceNowConfig::new(
            "https://instance.service-now.com".to_string(),
            "user".to_string(),
            "pass".to_string(),
        )
        .with_caller_id("caller123".to_string())
        .with_assignment_group("Legal Team".to_string());

        assert_eq!(config.instance_url, "https://instance.service-now.com");
        assert_eq!(config.caller_id, Some("caller123".to_string()));
        assert_eq!(config.assignment_group, Some("Legal Team".to_string()));
    }

    #[test]
    fn test_incident_severity() {
        assert_eq!(IncidentSeverity::Critical.impact(), 1);
        assert_eq!(IncidentSeverity::High.impact(), 2);
        assert_eq!(IncidentSeverity::Medium.impact(), 3);
        assert_eq!(IncidentSeverity::Low.impact(), 4);

        assert_eq!(IncidentSeverity::Critical.urgency(), 1);
        assert_eq!(IncidentSeverity::High.urgency(), 2);
        assert_eq!(IncidentSeverity::Medium.urgency(), 3);
        assert_eq!(IncidentSeverity::Low.urgency(), 4);
    }

    #[test]
    fn test_export_incident() {
        let config = ServiceNowConfig::new(
            "https://instance.service-now.com".to_string(),
            "user".to_string(),
            "pass".to_string(),
        );
        let exporter = ServiceNowExporter::new(config);
        let record = create_test_record();

        let result = exporter.export_incident(&record, IncidentSeverity::Medium);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert!(json.contains("\"short_description\""));
        assert!(json.contains("\"description\""));
        assert!(json.contains("\"impact\":3"));
        assert!(json.contains("\"urgency\":3"));
    }

    #[test]
    fn test_export_table_entry() {
        let config = ServiceNowConfig::new(
            "https://instance.service-now.com".to_string(),
            "user".to_string(),
            "pass".to_string(),
        );
        let exporter = ServiceNowExporter::new(config);
        let record = create_test_record();

        let result = exporter.export_table_entry(&record);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert!(json.contains("\"audit_id\""));
        assert!(json.contains("\"statute_id\""));
    }

    #[test]
    fn test_export_multiple_table_entries() {
        let config = ServiceNowConfig::new(
            "https://instance.service-now.com".to_string(),
            "user".to_string(),
            "pass".to_string(),
        );
        let exporter = ServiceNowExporter::new(config);

        let records: Vec<AuditRecord> = (0..3).map(|_| create_test_record()).collect();

        let result = exporter.export_table_entries(&records);
        assert!(result.is_ok());

        let entries = result.unwrap();
        assert_eq!(entries.len(), 3);

        for entry in entries {
            assert!(entry.contains("\"audit_id\""));
        }
    }

    #[test]
    fn test_servicenow_incident_structure() {
        let config = ServiceNowConfig::new(
            "https://instance.service-now.com".to_string(),
            "user".to_string(),
            "pass".to_string(),
        )
        .with_caller_id("caller123".to_string());

        let exporter = ServiceNowExporter::new(config);
        let record = create_test_record();

        let incident = exporter.to_servicenow_incident(&record, IncidentSeverity::High);

        assert!(!incident.short_description.is_empty());
        assert!(!incident.description.is_empty());
        assert_eq!(incident.impact, Some(2));
        assert_eq!(incident.urgency, Some(2));
        assert_eq!(incident.caller_id, Some("caller123".to_string()));
        assert_eq!(incident.category, Some("Audit".to_string()));
    }

    #[test]
    fn test_servicenow_table_entry_structure() {
        let config = ServiceNowConfig::default();
        let exporter = ServiceNowExporter::new(config);
        let record = create_test_record();

        let entry = exporter.to_servicenow_table_entry(&record);

        assert!(entry.fields.contains_key("audit_id"));
        assert!(entry.fields.contains_key("timestamp"));
        assert!(entry.fields.contains_key("event_type"));
        assert!(entry.fields.contains_key("statute_id"));
    }

    #[test]
    fn test_endpoint_urls() {
        let config = ServiceNowConfig::new(
            "https://instance.service-now.com".to_string(),
            "user".to_string(),
            "pass".to_string(),
        );
        let exporter = ServiceNowExporter::new(config);

        let incident_url = exporter.incident_endpoint();
        assert_eq!(
            incident_url,
            "https://instance.service-now.com/api/now/table/incident"
        );

        let table_url = exporter.table_endpoint("x_legalis_audit");
        assert_eq!(
            table_url,
            "https://instance.service-now.com/api/now/table/x_legalis_audit"
        );
    }

    #[test]
    fn test_incident_json_format() {
        let config = ServiceNowConfig::default();
        let exporter = ServiceNowExporter::new(config);
        let record = create_test_record();

        let json = exporter
            .export_incident(&record, IncidentSeverity::Critical)
            .unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed.get("short_description").is_some());
        assert!(parsed.get("description").is_some());
        assert!(parsed.get("impact").is_some());
        assert!(parsed.get("urgency").is_some());
        assert_eq!(parsed.get("impact").unwrap().as_i64(), Some(1));
    }

    #[test]
    fn test_table_entry_json_format() {
        let config = ServiceNowConfig::default();
        let exporter = ServiceNowExporter::new(config);
        let record = create_test_record();

        let json = exporter.export_table_entry(&record).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed.get("audit_id").is_some());
        assert!(parsed.get("timestamp").is_some());
        assert!(parsed.get("event_type").is_some());
        assert!(parsed.get("statute_id").is_some());
    }
}
