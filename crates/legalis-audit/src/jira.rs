//! Jira integration for audit trail export.
//!
//! This module provides integration with Jira for issue tracking and audit logging.

use crate::{AuditError, AuditRecord, AuditResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for Jira API.
#[derive(Debug, Clone)]
pub struct JiraConfig {
    /// Jira base URL (e.g., "https://company.atlassian.net")
    pub base_url: String,
    /// Username or email for authentication
    pub username: String,
    /// API token or password
    pub api_token: String,
    /// Default project key
    pub project_key: String,
    /// Default issue type (e.g., "Task", "Bug", "Story")
    pub default_issue_type: String,
}

impl JiraConfig {
    /// Creates a new Jira configuration.
    pub fn new(base_url: String, username: String, api_token: String, project_key: String) -> Self {
        Self {
            base_url,
            username,
            api_token,
            project_key,
            default_issue_type: "Task".to_string(),
        }
    }

    /// Sets the default issue type.
    pub fn with_issue_type(mut self, issue_type: String) -> Self {
        self.default_issue_type = issue_type;
        self
    }
}

/// Jira issue priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssuePriority {
    /// Highest priority
    Highest,
    /// High priority
    High,
    /// Medium priority
    Medium,
    /// Low priority
    Low,
    /// Lowest priority
    Lowest,
}

impl IssuePriority {
    /// Returns the Jira priority name.
    pub fn as_str(&self) -> &'static str {
        match self {
            IssuePriority::Highest => "Highest",
            IssuePriority::High => "High",
            IssuePriority::Medium => "Medium",
            IssuePriority::Low => "Low",
            IssuePriority::Lowest => "Lowest",
        }
    }
}

/// Jira issue format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraIssue {
    /// Issue fields
    pub fields: JiraIssueFields,
}

/// Jira issue fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraIssueFields {
    /// Project
    pub project: JiraProject,
    /// Summary (title)
    pub summary: String,
    /// Description
    pub description: String,
    /// Issue type
    pub issuetype: JiraIssueType,
    /// Priority
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<JiraPriority>,
    /// Labels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Custom fields
    #[serde(flatten)]
    pub custom_fields: HashMap<String, serde_json::Value>,
}

/// Jira project reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraProject {
    /// Project key
    pub key: String,
}

/// Jira issue type reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraIssueType {
    /// Issue type name
    pub name: String,
}

/// Jira priority reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraPriority {
    /// Priority name
    pub name: String,
}

/// Jira exporter for audit records.
pub struct JiraExporter {
    config: JiraConfig,
}

impl JiraExporter {
    /// Creates a new Jira exporter.
    pub fn new(config: JiraConfig) -> Self {
        Self { config }
    }

    /// Converts an audit record to Jira issue format.
    fn to_jira_issue(&self, record: &AuditRecord, priority: IssuePriority) -> JiraIssue {
        let summary = format!(
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
            "h2. Audit Record Details\n\
             \n\
             ||Field||Value||\n\
             |ID|{}|\n\
             |Timestamp|{}|\n\
             |Event Type|{:?}|\n\
             |Actor|{:?}|\n\
             |Statute ID|{}|\n\
             |Subject ID|{}|\n\
             |Record Hash|{{monospace}}{}{{monospace}}|\n\
             |Previous Hash|{{monospace}}{}{{monospace}}|\n\
             \n\
             h3. Context\n\
             {{code:json}}\n\
             {}\n\
             {{code}}\n\
             \n\
             h3. Result\n\
             {{code:json}}\n\
             {}\n\
             {{code}}",
            record.id,
            record.timestamp,
            record.event_type,
            record.actor,
            record.statute_id,
            record.subject_id,
            record.record_hash,
            record.previous_hash.as_ref().unwrap_or(&"None".to_string()),
            serde_json::to_string_pretty(&record.context).unwrap_or_default(),
            serde_json::to_string_pretty(&record.result).unwrap_or_default()
        );

        let mut custom_fields = HashMap::new();
        custom_fields.insert("audit_id".to_string(), serde_json::json!(record.id));
        custom_fields.insert(
            "audit_timestamp".to_string(),
            serde_json::json!(record.timestamp.to_rfc3339()),
        );

        JiraIssue {
            fields: JiraIssueFields {
                project: JiraProject {
                    key: self.config.project_key.clone(),
                },
                summary,
                description,
                issuetype: JiraIssueType {
                    name: self.config.default_issue_type.clone(),
                },
                priority: Some(JiraPriority {
                    name: priority.as_str().to_string(),
                }),
                labels: Some(vec![
                    "audit".to_string(),
                    "legalis".to_string(),
                    format!("statute:{}", record.statute_id),
                ]),
                custom_fields,
            },
        }
    }

    /// Exports an audit record as a Jira issue JSON.
    pub fn export_issue(
        &self,
        record: &AuditRecord,
        priority: IssuePriority,
    ) -> AuditResult<String> {
        let issue = self.to_jira_issue(record, priority);
        serde_json::to_string(&issue).map_err(|e| AuditError::ExportError(e.to_string()))
    }

    /// Exports multiple audit records as Jira issues.
    pub fn export_issues(
        &self,
        records: &[AuditRecord],
        priority: IssuePriority,
    ) -> AuditResult<Vec<String>> {
        records
            .iter()
            .map(|record| self.export_issue(record, priority))
            .collect()
    }

    /// Creates a batch issue creation payload.
    pub fn export_batch_issues(
        &self,
        records: &[AuditRecord],
        priority: IssuePriority,
    ) -> AuditResult<String> {
        let issues: Vec<JiraIssue> = records
            .iter()
            .map(|record| self.to_jira_issue(record, priority))
            .collect();

        let payload = serde_json::json!({
            "issueUpdates": issues
        });

        serde_json::to_string(&payload).map_err(|e| AuditError::ExportError(e.to_string()))
    }

    /// Returns the issue creation API endpoint URL.
    pub fn issue_endpoint(&self) -> String {
        format!("{}/rest/api/3/issue", self.config.base_url)
    }

    /// Returns the bulk issue creation API endpoint URL.
    pub fn bulk_issue_endpoint(&self) -> String {
        format!("{}/rest/api/3/issue/bulk", self.config.base_url)
    }

    /// Returns the search API endpoint URL.
    pub fn search_endpoint(&self) -> String {
        format!("{}/rest/api/3/search", self.config.base_url)
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
    fn test_jira_config() {
        let config = JiraConfig::new(
            "https://company.atlassian.net".to_string(),
            "user@example.com".to_string(),
            "api_token".to_string(),
            "LEG".to_string(),
        )
        .with_issue_type("Bug".to_string());

        assert_eq!(config.base_url, "https://company.atlassian.net");
        assert_eq!(config.project_key, "LEG");
        assert_eq!(config.default_issue_type, "Bug");
    }

    #[test]
    fn test_issue_priority() {
        assert_eq!(IssuePriority::Highest.as_str(), "Highest");
        assert_eq!(IssuePriority::High.as_str(), "High");
        assert_eq!(IssuePriority::Medium.as_str(), "Medium");
        assert_eq!(IssuePriority::Low.as_str(), "Low");
        assert_eq!(IssuePriority::Lowest.as_str(), "Lowest");
    }

    #[test]
    fn test_export_issue() {
        let config = JiraConfig::new(
            "https://company.atlassian.net".to_string(),
            "user@example.com".to_string(),
            "api_token".to_string(),
            "LEG".to_string(),
        );
        let exporter = JiraExporter::new(config);
        let record = create_test_record();

        let result = exporter.export_issue(&record, IssuePriority::Medium);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert!(json.contains("\"summary\""));
        assert!(json.contains("\"description\""));
        assert!(json.contains("\"project\""));
        assert!(json.contains("\"LEG\""));
    }

    #[test]
    fn test_export_multiple_issues() {
        let config = JiraConfig::new(
            "https://company.atlassian.net".to_string(),
            "user@example.com".to_string(),
            "api_token".to_string(),
            "LEG".to_string(),
        );
        let exporter = JiraExporter::new(config);

        let records: Vec<AuditRecord> = (0..3).map(|_| create_test_record()).collect();

        let result = exporter.export_issues(&records, IssuePriority::High);
        assert!(result.is_ok());

        let issues = result.unwrap();
        assert_eq!(issues.len(), 3);

        for issue in issues {
            assert!(issue.contains("\"summary\""));
            assert!(issue.contains("\"LEG\""));
        }
    }

    #[test]
    fn test_export_batch_issues() {
        let config = JiraConfig::new(
            "https://company.atlassian.net".to_string(),
            "user@example.com".to_string(),
            "api_token".to_string(),
            "LEG".to_string(),
        );
        let exporter = JiraExporter::new(config);

        let records: Vec<AuditRecord> = (0..3).map(|_| create_test_record()).collect();

        let result = exporter.export_batch_issues(&records, IssuePriority::Lowest);
        assert!(result.is_ok());

        let batch = result.unwrap();
        assert!(batch.contains("\"issueUpdates\""));
        assert!(batch.contains("\"LEG\""));
    }

    #[test]
    fn test_jira_issue_structure() {
        let config = JiraConfig::new(
            "https://company.atlassian.net".to_string(),
            "user@example.com".to_string(),
            "api_token".to_string(),
            "LEG".to_string(),
        );
        let exporter = JiraExporter::new(config);
        let record = create_test_record();

        let issue = exporter.to_jira_issue(&record, IssuePriority::High);

        assert_eq!(issue.fields.project.key, "LEG");
        assert!(!issue.fields.summary.is_empty());
        assert!(!issue.fields.description.is_empty());
        assert_eq!(issue.fields.issuetype.name, "Task");
        assert_eq!(issue.fields.priority.as_ref().unwrap().name, "High");
        assert!(issue.fields.labels.is_some());
        assert!(issue.fields.custom_fields.contains_key("audit_id"));
    }

    #[test]
    fn test_endpoint_urls() {
        let config = JiraConfig::new(
            "https://company.atlassian.net".to_string(),
            "user@example.com".to_string(),
            "api_token".to_string(),
            "LEG".to_string(),
        );
        let exporter = JiraExporter::new(config);

        let issue_url = exporter.issue_endpoint();
        assert_eq!(issue_url, "https://company.atlassian.net/rest/api/3/issue");

        let bulk_url = exporter.bulk_issue_endpoint();
        assert_eq!(
            bulk_url,
            "https://company.atlassian.net/rest/api/3/issue/bulk"
        );

        let search_url = exporter.search_endpoint();
        assert_eq!(
            search_url,
            "https://company.atlassian.net/rest/api/3/search"
        );
    }

    #[test]
    fn test_issue_json_format() {
        let config = JiraConfig::new(
            "https://company.atlassian.net".to_string(),
            "user@example.com".to_string(),
            "api_token".to_string(),
            "LEG".to_string(),
        );
        let exporter = JiraExporter::new(config);
        let record = create_test_record();

        let json = exporter
            .export_issue(&record, IssuePriority::Medium)
            .unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed.get("fields").is_some());
        let fields = parsed.get("fields").unwrap();
        assert!(fields.get("project").is_some());
        assert!(fields.get("summary").is_some());
        assert!(fields.get("description").is_some());
        assert!(fields.get("issuetype").is_some());
        assert!(fields.get("priority").is_some());
        assert!(fields.get("labels").is_some());
    }

    #[test]
    fn test_issue_description_format() {
        let config = JiraConfig::new(
            "https://company.atlassian.net".to_string(),
            "user@example.com".to_string(),
            "api_token".to_string(),
            "LEG".to_string(),
        );
        let exporter = JiraExporter::new(config);
        let record = create_test_record();

        let issue = exporter.to_jira_issue(&record, IssuePriority::Low);

        // Verify Jira Wiki markup is used
        assert!(issue.fields.description.contains("h2."));
        assert!(issue.fields.description.contains("||Field||Value||"));
        assert!(issue.fields.description.contains("{code:json}"));
        assert!(issue.fields.description.contains("{monospace}"));
    }

    #[test]
    fn test_issue_labels() {
        let config = JiraConfig::new(
            "https://company.atlassian.net".to_string(),
            "user@example.com".to_string(),
            "api_token".to_string(),
            "LEG".to_string(),
        );
        let exporter = JiraExporter::new(config);
        let record = create_test_record();

        let issue = exporter.to_jira_issue(&record, IssuePriority::Medium);

        let labels = issue.fields.labels.unwrap();
        assert!(labels.contains(&"audit".to_string()));
        assert!(labels.contains(&"legalis".to_string()));
        assert!(labels.iter().any(|l| l.starts_with("statute:")));
    }
}
