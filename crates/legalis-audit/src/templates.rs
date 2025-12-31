//! Custom report templates for flexible audit reporting.
//!
//! This module provides functionality for creating custom report templates
//! with flexible layouts and formatting options.

use crate::{AuditRecord, AuditResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Template field type for extracting data from audit records.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    /// Record ID
    Id,
    /// Timestamp
    Timestamp,
    /// Event type
    EventType,
    /// Actor information
    Actor,
    /// Statute ID
    StatuteId,
    /// Subject ID
    SubjectId,
    /// Decision result type
    ResultType,
    /// Record hash
    RecordHash,
    /// Previous hash
    PreviousHash,
    /// Custom field from context attributes
    ContextAttribute(String),
    /// Custom field from metadata
    Metadata(String),
    /// Decision effect (for deterministic results)
    Effect,
    /// Override justification (for overridden results)
    OverrideJustification,
}

/// Field formatting options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldFormat {
    /// Field label/header
    pub label: String,
    /// Field type
    pub field_type: FieldType,
    /// Custom date/time format (for timestamp fields)
    pub date_format: Option<String>,
    /// Whether to truncate long values
    pub truncate: Option<usize>,
    /// Default value if field is missing
    pub default_value: Option<String>,
}

impl FieldFormat {
    /// Creates a new field format.
    pub fn new(label: String, field_type: FieldType) -> Self {
        Self {
            label,
            field_type,
            date_format: None,
            truncate: None,
            default_value: None,
        }
    }

    /// Sets the date format.
    pub fn with_date_format(mut self, format: String) -> Self {
        self.date_format = Some(format);
        self
    }

    /// Sets truncation length.
    pub fn with_truncate(mut self, length: usize) -> Self {
        self.truncate = Some(length);
        self
    }

    /// Sets default value.
    pub fn with_default(mut self, value: String) -> Self {
        self.default_value = Some(value);
        self
    }

    /// Extracts the field value from an audit record.
    pub fn extract(&self, record: &AuditRecord) -> String {
        let value = match &self.field_type {
            FieldType::Id => record.id.to_string(),
            FieldType::Timestamp => {
                if let Some(ref format) = self.date_format {
                    record.timestamp.format(format).to_string()
                } else {
                    record.timestamp.to_rfc3339()
                }
            }
            FieldType::EventType => format!("{:?}", record.event_type),
            FieldType::Actor => match &record.actor {
                crate::Actor::System { component } => format!("System: {}", component),
                crate::Actor::User { user_id, role } => format!("User: {} ({})", user_id, role),
                crate::Actor::External { system_id } => format!("External: {}", system_id),
            },
            FieldType::StatuteId => record.statute_id.clone(),
            FieldType::SubjectId => record.subject_id.to_string(),
            FieldType::ResultType => match &record.result {
                crate::DecisionResult::Deterministic { .. } => "Deterministic".to_string(),
                crate::DecisionResult::RequiresDiscretion { .. } => {
                    "RequiresDiscretion".to_string()
                }
                crate::DecisionResult::Void { .. } => "Void".to_string(),
                crate::DecisionResult::Overridden { .. } => "Overridden".to_string(),
            },
            FieldType::RecordHash => record.record_hash.clone(),
            FieldType::PreviousHash => record
                .previous_hash
                .clone()
                .unwrap_or_else(|| "None".to_string()),
            FieldType::ContextAttribute(key) => record
                .context
                .attributes
                .get(key)
                .cloned()
                .or_else(|| self.default_value.clone())
                .unwrap_or_else(|| "N/A".to_string()),
            FieldType::Metadata(key) => record
                .context
                .metadata
                .get(key)
                .cloned()
                .or_else(|| self.default_value.clone())
                .unwrap_or_else(|| "N/A".to_string()),
            FieldType::Effect => match &record.result {
                crate::DecisionResult::Deterministic { effect_applied, .. } => {
                    effect_applied.clone()
                }
                _ => self
                    .default_value
                    .clone()
                    .unwrap_or_else(|| "N/A".to_string()),
            },
            FieldType::OverrideJustification => match &record.result {
                crate::DecisionResult::Overridden { justification, .. } => justification.clone(),
                _ => self
                    .default_value
                    .clone()
                    .unwrap_or_else(|| "N/A".to_string()),
            },
        };

        if let Some(len) = self.truncate {
            if value.len() > len {
                return format!("{}...", &value[..len.saturating_sub(3)]);
            }
        }

        value
    }
}

/// Report template defining the structure and content of a custom report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplate {
    /// Template name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Fields to include in the report
    pub fields: Vec<FieldFormat>,
    /// Whether to include a summary section
    pub include_summary: bool,
    /// Custom summary fields
    pub summary_fields: Vec<SummaryField>,
    /// Sorting field index (None for no sorting)
    pub sort_by: Option<usize>,
    /// Sort in reverse order
    pub sort_reverse: bool,
    /// Filter expression (reserved for future use)
    pub filter: Option<String>,
}

/// Summary field for aggregated statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SummaryField {
    /// Total record count
    TotalRecords,
    /// Count by event type
    CountByEventType,
    /// Count by actor type
    CountByActorType,
    /// Count by result type
    CountByResultType,
    /// Count by statute
    CountByStatute,
    /// Time range covered
    TimeRange,
    /// Custom text
    CustomText(String),
}

impl ReportTemplate {
    /// Creates a new report template.
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            fields: Vec::new(),
            include_summary: true,
            summary_fields: vec![
                SummaryField::TotalRecords,
                SummaryField::TimeRange,
                SummaryField::CountByEventType,
            ],
            sort_by: None,
            sort_reverse: false,
            filter: None,
        }
    }

    /// Adds a field to the template.
    pub fn add_field(mut self, field: FieldFormat) -> Self {
        self.fields.push(field);
        self
    }

    /// Sets the description.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets whether to include summary.
    pub fn with_summary(mut self, include: bool) -> Self {
        self.include_summary = include;
        self
    }

    /// Sets summary fields.
    pub fn with_summary_fields(mut self, fields: Vec<SummaryField>) -> Self {
        self.summary_fields = fields;
        self
    }

    /// Sets the sorting field.
    pub fn sort_by_field(mut self, index: usize, reverse: bool) -> Self {
        self.sort_by = Some(index);
        self.sort_reverse = reverse;
        self
    }

    /// Generates a summary section.
    fn generate_summary(&self, records: &[AuditRecord]) -> HashMap<String, String> {
        let mut summary = HashMap::new();

        for field in &self.summary_fields {
            match field {
                SummaryField::TotalRecords => {
                    summary.insert("Total Records".to_string(), records.len().to_string());
                }
                SummaryField::CountByEventType => {
                    let mut counts: HashMap<String, usize> = HashMap::new();
                    for record in records {
                        *counts
                            .entry(format!("{:?}", record.event_type))
                            .or_insert(0) += 1;
                    }
                    for (event_type, count) in counts {
                        summary.insert(format!("Event Type: {}", event_type), count.to_string());
                    }
                }
                SummaryField::CountByActorType => {
                    let mut counts: HashMap<String, usize> = HashMap::new();
                    for record in records {
                        let actor_type = match &record.actor {
                            crate::Actor::System { .. } => "System",
                            crate::Actor::User { .. } => "User",
                            crate::Actor::External { .. } => "External",
                        };
                        *counts.entry(actor_type.to_string()).or_insert(0) += 1;
                    }
                    for (actor_type, count) in counts {
                        summary.insert(format!("Actor Type: {}", actor_type), count.to_string());
                    }
                }
                SummaryField::CountByResultType => {
                    let mut counts: HashMap<String, usize> = HashMap::new();
                    for record in records {
                        let result_type = match &record.result {
                            crate::DecisionResult::Deterministic { .. } => "Deterministic",
                            crate::DecisionResult::RequiresDiscretion { .. } => {
                                "RequiresDiscretion"
                            }
                            crate::DecisionResult::Void { .. } => "Void",
                            crate::DecisionResult::Overridden { .. } => "Overridden",
                        };
                        *counts.entry(result_type.to_string()).or_insert(0) += 1;
                    }
                    for (result_type, count) in counts {
                        summary.insert(format!("Result Type: {}", result_type), count.to_string());
                    }
                }
                SummaryField::CountByStatute => {
                    let mut counts: HashMap<String, usize> = HashMap::new();
                    for record in records {
                        *counts.entry(record.statute_id.clone()).or_insert(0) += 1;
                    }
                    for (statute, count) in counts {
                        summary.insert(format!("Statute: {}", statute), count.to_string());
                    }
                }
                SummaryField::TimeRange => {
                    if let (Some(first), Some(last)) = (records.first(), records.last()) {
                        summary
                            .insert("Time Range Start".to_string(), first.timestamp.to_rfc3339());
                        summary.insert("Time Range End".to_string(), last.timestamp.to_rfc3339());
                    }
                }
                SummaryField::CustomText(text) => {
                    summary.insert("Note".to_string(), text.clone());
                }
            }
        }

        summary
    }

    /// Renders the template to CSV format.
    pub fn render_csv(&self, records: &[AuditRecord]) -> AuditResult<String> {
        let mut output = Vec::new();

        // Add summary if requested
        if self.include_summary {
            let summary = self.generate_summary(records);
            output.push("# SUMMARY".to_string());
            for (key, value) in summary.iter() {
                output.push(format!("# {}: {}", key, value));
            }
            output.push("".to_string());
        }

        // Add header
        let headers: Vec<String> = self.fields.iter().map(|f| f.label.clone()).collect();
        output.push(headers.join(","));

        // Sort records if requested
        let mut sorted_records = records.to_vec();
        if let Some(sort_index) = self.sort_by {
            if sort_index < self.fields.len() {
                let field = &self.fields[sort_index];
                sorted_records.sort_by(|a, b| {
                    let val_a = field.extract(a);
                    let val_b = field.extract(b);
                    if self.sort_reverse {
                        val_b.cmp(&val_a)
                    } else {
                        val_a.cmp(&val_b)
                    }
                });
            }
        }

        // Add rows
        for record in &sorted_records {
            let values: Vec<String> = self.fields.iter().map(|f| f.extract(record)).collect();
            output.push(values.join(","));
        }

        Ok(output.join("\n"))
    }

    /// Renders the template to HTML table format.
    pub fn render_html(&self, records: &[AuditRecord]) -> AuditResult<String> {
        let mut html = String::from("<html><head><style>");
        html.push_str("table { border-collapse: collapse; width: 100%; margin-top: 20px; }");
        html.push_str("th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }");
        html.push_str("th { background-color: #4CAF50; color: white; }");
        html.push_str("tr:nth-child(even) { background-color: #f2f2f2; }");
        html.push_str(".summary { background-color: #f9f9f9; padding: 15px; margin-bottom: 20px; border-left: 4px solid #4CAF50; }");
        html.push_str("</style></head><body>");

        if let Some(ref desc) = self.description {
            html.push_str(&format!("<h1>{}</h1>", self.name));
            html.push_str(&format!("<p>{}</p>", desc));
        } else {
            html.push_str(&format!("<h1>{}</h1>", self.name));
        }

        // Add summary if requested
        if self.include_summary {
            let summary = self.generate_summary(records);
            html.push_str("<div class='summary'>");
            html.push_str("<h2>Summary</h2>");
            for (key, value) in summary.iter() {
                html.push_str(&format!("<p><strong>{}:</strong> {}</p>", key, value));
            }
            html.push_str("</div>");
        }

        // Sort records if requested
        let mut sorted_records = records.to_vec();
        if let Some(sort_index) = self.sort_by {
            if sort_index < self.fields.len() {
                let field = &self.fields[sort_index];
                sorted_records.sort_by(|a, b| {
                    let val_a = field.extract(a);
                    let val_b = field.extract(b);
                    if self.sort_reverse {
                        val_b.cmp(&val_a)
                    } else {
                        val_a.cmp(&val_b)
                    }
                });
            }
        }

        // Add table
        html.push_str("<table>");

        // Add header
        html.push_str("<thead><tr>");
        for field in &self.fields {
            html.push_str(&format!("<th>{}</th>", field.label));
        }
        html.push_str("</tr></thead>");

        // Add rows
        html.push_str("<tbody>");
        for record in &sorted_records {
            html.push_str("<tr>");
            for field in &self.fields {
                html.push_str(&format!("<td>{}</td>", field.extract(record)));
            }
            html.push_str("</tr>");
        }
        html.push_str("</tbody>");

        html.push_str("</table>");
        html.push_str("</body></html>");

        Ok(html)
    }
}

/// Predefined templates for common use cases.
pub struct TemplateLibrary;

impl TemplateLibrary {
    /// Standard audit log template.
    pub fn standard_audit() -> ReportTemplate {
        ReportTemplate::new("Standard Audit Log".to_string())
            .with_description("Standard audit trail report with all key fields".to_string())
            .add_field(FieldFormat::new("ID".to_string(), FieldType::Id))
            .add_field(
                FieldFormat::new("Timestamp".to_string(), FieldType::Timestamp)
                    .with_date_format("%Y-%m-%d %H:%M:%S".to_string()),
            )
            .add_field(FieldFormat::new("Event".to_string(), FieldType::EventType))
            .add_field(FieldFormat::new("Actor".to_string(), FieldType::Actor))
            .add_field(FieldFormat::new(
                "Statute".to_string(),
                FieldType::StatuteId,
            ))
            .add_field(FieldFormat::new(
                "Subject".to_string(),
                FieldType::SubjectId,
            ))
            .add_field(FieldFormat::new(
                "Result".to_string(),
                FieldType::ResultType,
            ))
    }

    /// Compliance summary template.
    pub fn compliance_summary() -> ReportTemplate {
        ReportTemplate::new("Compliance Summary".to_string())
            .with_description("High-level compliance overview".to_string())
            .add_field(
                FieldFormat::new("Date".to_string(), FieldType::Timestamp)
                    .with_date_format("%Y-%m-%d".to_string()),
            )
            .add_field(FieldFormat::new(
                "Statute".to_string(),
                FieldType::StatuteId,
            ))
            .add_field(FieldFormat::new(
                "Result".to_string(),
                FieldType::ResultType,
            ))
            .add_field(FieldFormat::new("Actor Type".to_string(), FieldType::Actor))
            .with_summary_fields(vec![
                SummaryField::TotalRecords,
                SummaryField::CountByStatute,
                SummaryField::CountByResultType,
            ])
    }

    /// Override audit template.
    pub fn override_audit() -> ReportTemplate {
        ReportTemplate::new("Override Audit".to_string())
            .with_description("Detailed report of human overrides".to_string())
            .add_field(
                FieldFormat::new("Timestamp".to_string(), FieldType::Timestamp)
                    .with_date_format("%Y-%m-%d %H:%M:%S".to_string()),
            )
            .add_field(FieldFormat::new("Actor".to_string(), FieldType::Actor))
            .add_field(FieldFormat::new(
                "Statute".to_string(),
                FieldType::StatuteId,
            ))
            .add_field(FieldFormat::new(
                "Justification".to_string(),
                FieldType::OverrideJustification,
            ))
            .with_summary_fields(vec![
                SummaryField::TotalRecords,
                SummaryField::CountByActorType,
            ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn test_field_extraction() {
        let record = AuditRecord::new(
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
        );

        let field = FieldFormat::new("Statute".to_string(), FieldType::StatuteId);
        assert_eq!(field.extract(&record), "statute-123");

        let field = FieldFormat::new("Effect".to_string(), FieldType::Effect);
        assert_eq!(field.extract(&record), "approved");
    }

    #[test]
    fn test_template_csv_rendering() {
        let record = AuditRecord::new(
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
        );

        let template = ReportTemplate::new("Test Template".to_string())
            .add_field(FieldFormat::new(
                "Statute".to_string(),
                FieldType::StatuteId,
            ))
            .add_field(FieldFormat::new(
                "Result".to_string(),
                FieldType::ResultType,
            ))
            .with_summary(false);

        let csv = template.render_csv(&[record]).unwrap();
        assert!(csv.contains("Statute,Result"));
        assert!(csv.contains("statute-123,Deterministic"));
    }

    #[test]
    fn test_template_html_rendering() {
        let record = AuditRecord::new(
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
        );

        let template = ReportTemplate::new("Test Template".to_string())
            .add_field(FieldFormat::new(
                "Statute".to_string(),
                FieldType::StatuteId,
            ))
            .with_summary(false);

        let html = template.render_html(&[record]).unwrap();
        assert!(html.contains("<table>"));
        assert!(html.contains("statute-123"));
    }

    #[test]
    fn test_standard_audit_template() {
        let template = TemplateLibrary::standard_audit();
        assert_eq!(template.name, "Standard Audit Log");
        assert!(!template.fields.is_empty());
    }

    #[test]
    fn test_template_sorting() {
        let mut records = vec![];
        for i in 0..3 {
            let record = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "test".to_string(),
                },
                format!("statute-{}", 3 - i),
                Uuid::new_v4(),
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "test".to_string(),
                    parameters: HashMap::new(),
                },
                None,
            );
            records.push(record);
        }

        let template = ReportTemplate::new("Test".to_string())
            .add_field(FieldFormat::new(
                "Statute".to_string(),
                FieldType::StatuteId,
            ))
            .sort_by_field(0, false)
            .with_summary(false);

        let csv = template.render_csv(&records).unwrap();
        let lines: Vec<&str> = csv.lines().collect();
        assert!(lines[1].contains("statute-1"));
        assert!(lines[3].contains("statute-3"));
    }
}
