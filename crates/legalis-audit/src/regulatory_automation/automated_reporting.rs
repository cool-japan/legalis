//! Automated compliance reporting system.
//!
//! Provides scheduled generation and delivery of compliance reports
//! for various regulatory frameworks (GDPR, SOX, HIPAA, etc.).

use crate::{AuditRecord, AuditResult, ComplianceReport};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Automated compliance reporter
pub struct AutomatedReporter {
    schedules: Vec<ComplianceReportSchedule>,
    templates: HashMap<String, ReportTemplate>,
    last_generated: HashMap<String, DateTime<Utc>>,
}

impl AutomatedReporter {
    /// Create a new automated reporter
    pub fn new() -> Self {
        Self {
            schedules: Vec::new(),
            templates: Self::default_templates(),
            last_generated: HashMap::new(),
        }
    }

    /// Add a reporting schedule
    pub fn add_schedule(&mut self, schedule: ComplianceReportSchedule) {
        self.schedules.push(schedule);
    }

    /// Add a custom report template
    pub fn add_template(&mut self, template: ReportTemplate) {
        self.templates.insert(template.id.clone(), template);
    }

    /// Check and generate due reports
    pub fn generate_due_reports(
        &mut self,
        records: &[AuditRecord],
        report: &ComplianceReport,
    ) -> AuditResult<Vec<GeneratedReport>> {
        let now = Utc::now();
        let mut generated_reports = Vec::new();

        for schedule in &self.schedules {
            if self.is_report_due(schedule, now) {
                let generated = self.generate_report(schedule, records, report, now)?;
                generated_reports.push(generated);
                self.last_generated.insert(schedule.id.clone(), now);
            }
        }

        Ok(generated_reports)
    }

    /// Check if a report is due
    fn is_report_due(&self, schedule: &ComplianceReportSchedule, now: DateTime<Utc>) -> bool {
        if !schedule.enabled {
            return false;
        }

        let last_gen = self.last_generated.get(&schedule.id);
        match last_gen {
            None => true, // Never generated, so it's due
            Some(last) => {
                let elapsed = now.signed_duration_since(*last);
                elapsed >= schedule.frequency.to_duration()
            }
        }
    }

    /// Generate a compliance report
    fn generate_report(
        &self,
        schedule: &ComplianceReportSchedule,
        records: &[AuditRecord],
        report: &ComplianceReport,
        generated_at: DateTime<Utc>,
    ) -> AuditResult<GeneratedReport> {
        let template = self.templates.get(&schedule.template_id).ok_or_else(|| {
            crate::AuditError::InvalidRecord(format!(
                "Template not found: {}",
                schedule.template_id
            ))
        })?;

        let content = self.render_report(template, records, report)?;

        Ok(GeneratedReport {
            schedule_id: schedule.id.clone(),
            regulation: schedule.regulation.clone(),
            template_id: schedule.template_id.clone(),
            generated_at,
            content,
            format: template.format,
            recipients: schedule.recipients.clone(),
        })
    }

    /// Render a report using a template
    fn render_report(
        &self,
        template: &ReportTemplate,
        records: &[AuditRecord],
        report: &ComplianceReport,
    ) -> AuditResult<String> {
        let mut content = template.content.clone();

        // Replace placeholders
        content = content.replace("{{TOTAL_DECISIONS}}", &report.total_decisions.to_string());
        content = content.replace(
            "{{AUTOMATIC_DECISIONS}}",
            &report.automatic_decisions.to_string(),
        );
        content = content.replace(
            "{{DISCRETIONARY_DECISIONS}}",
            &report.discretionary_decisions.to_string(),
        );
        content = content.replace("{{HUMAN_OVERRIDES}}", &report.human_overrides.to_string());
        content = content.replace(
            "{{INTEGRITY_VERIFIED}}",
            &report.integrity_verified.to_string(),
        );
        content = content.replace("{{GENERATED_AT}}", &report.generated_at.to_rfc3339());

        // Add record count
        content = content.replace("{{RECORD_COUNT}}", &records.len().to_string());

        Ok(content)
    }

    /// Get default report templates
    fn default_templates() -> HashMap<String, ReportTemplate> {
        let mut templates = HashMap::new();

        templates.insert(
            "gdpr-monthly".to_string(),
            ReportTemplate {
                id: "gdpr-monthly".to_string(),
                name: "GDPR Monthly Report".to_string(),
                regulation: "GDPR".to_string(),
                format: ReportFormat::Json,
                content: r#"{
  "report_type": "GDPR Compliance",
  "period": "Monthly",
  "total_decisions": {{TOTAL_DECISIONS}},
  "automatic_decisions": {{AUTOMATIC_DECISIONS}},
  "discretionary_decisions": {{DISCRETIONARY_DECISIONS}},
  "human_overrides": {{HUMAN_OVERRIDES}},
  "integrity_verified": {{INTEGRITY_VERIFIED}},
  "generated_at": "{{GENERATED_AT}}"
}"#
                .to_string(),
            },
        );

        templates.insert(
            "sox-quarterly".to_string(),
            ReportTemplate {
                id: "sox-quarterly".to_string(),
                name: "SOX Quarterly Report".to_string(),
                regulation: "SOX".to_string(),
                format: ReportFormat::Xml,
                content: r#"<?xml version="1.0" encoding="UTF-8"?>
<SOXComplianceReport>
  <Period>Quarterly</Period>
  <TotalDecisions>{{TOTAL_DECISIONS}}</TotalDecisions>
  <AutomaticDecisions>{{AUTOMATIC_DECISIONS}}</AutomaticDecisions>
  <DiscretionaryDecisions>{{DISCRETIONARY_DECISIONS}}</DiscretionaryDecisions>
  <HumanOverrides>{{HUMAN_OVERRIDES}}</HumanOverrides>
  <IntegrityVerified>{{INTEGRITY_VERIFIED}}</IntegrityVerified>
  <GeneratedAt>{{GENERATED_AT}}</GeneratedAt>
</SOXComplianceReport>"#
                    .to_string(),
            },
        );

        templates.insert(
            "hipaa-annual".to_string(),
            ReportTemplate {
                id: "hipaa-annual".to_string(),
                name: "HIPAA Annual Report".to_string(),
                regulation: "HIPAA".to_string(),
                format: ReportFormat::Pdf,
                content: "HIPAA Annual Compliance Report\n\nTotal Decisions: {{TOTAL_DECISIONS}}\nAutomatic Decisions: {{AUTOMATIC_DECISIONS}}\nDiscretionary Decisions: {{DISCRETIONARY_DECISIONS}}\nHuman Overrides: {{HUMAN_OVERRIDES}}\nIntegrity Verified: {{INTEGRITY_VERIFIED}}\nGenerated At: {{GENERATED_AT}}".to_string(),
            },
        );

        templates
    }

    /// Get all schedules
    pub fn schedules(&self) -> &[ComplianceReportSchedule] {
        &self.schedules
    }

    /// Get schedule by ID
    pub fn get_schedule(&self, id: &str) -> Option<&ComplianceReportSchedule> {
        self.schedules.iter().find(|s| s.id == id)
    }

    /// Enable/disable a schedule
    pub fn set_schedule_enabled(&mut self, id: &str, enabled: bool) -> AuditResult<()> {
        let schedule = self
            .schedules
            .iter_mut()
            .find(|s| s.id == id)
            .ok_or_else(|| {
                crate::AuditError::InvalidRecord(format!("Schedule not found: {}", id))
            })?;
        schedule.enabled = enabled;
        Ok(())
    }
}

impl Default for AutomatedReporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Compliance report schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReportSchedule {
    pub id: String,
    pub name: String,
    pub regulation: String,
    pub template_id: String,
    pub frequency: ReportFrequency,
    pub recipients: Vec<String>,
    pub enabled: bool,
}

/// Report frequency
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ReportFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annual,
}

impl ReportFrequency {
    /// Convert frequency to duration
    pub fn to_duration(self) -> Duration {
        match self {
            ReportFrequency::Daily => Duration::days(1),
            ReportFrequency::Weekly => Duration::weeks(1),
            ReportFrequency::Monthly => Duration::days(30),
            ReportFrequency::Quarterly => Duration::days(90),
            ReportFrequency::Annual => Duration::days(365),
        }
    }
}

/// Report template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplate {
    pub id: String,
    pub name: String,
    pub regulation: String,
    pub format: ReportFormat,
    pub content: String,
}

/// Report format
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ReportFormat {
    Json,
    Xml,
    Csv,
    Pdf,
    Html,
}

/// Generated compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedReport {
    pub schedule_id: String,
    pub regulation: String,
    pub template_id: String,
    pub generated_at: DateTime<Utc>,
    pub content: String,
    pub format: ReportFormat,
    pub recipients: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use uuid::Uuid;

    fn create_test_records(count: usize) -> Vec<AuditRecord> {
        (0..count)
            .map(|_| {
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
            })
            .collect()
    }

    fn create_test_compliance_report() -> ComplianceReport {
        ComplianceReport {
            total_decisions: 100,
            automatic_decisions: 80,
            discretionary_decisions: 15,
            human_overrides: 5,
            integrity_verified: true,
            generated_at: Utc::now(),
        }
    }

    #[test]
    fn test_automated_reporter_creation() {
        let reporter = AutomatedReporter::new();
        assert!(reporter.schedules.is_empty());
        assert!(!reporter.templates.is_empty());
    }

    #[test]
    fn test_add_schedule() {
        let mut reporter = AutomatedReporter::new();
        let schedule = ComplianceReportSchedule {
            id: "test-schedule".to_string(),
            name: "Test Schedule".to_string(),
            regulation: "GDPR".to_string(),
            template_id: "gdpr-monthly".to_string(),
            frequency: ReportFrequency::Monthly,
            recipients: vec!["admin@example.com".to_string()],
            enabled: true,
        };

        reporter.add_schedule(schedule);
        assert_eq!(reporter.schedules.len(), 1);
    }

    #[test]
    fn test_generate_due_reports() {
        let mut reporter = AutomatedReporter::new();
        let schedule = ComplianceReportSchedule {
            id: "test-schedule".to_string(),
            name: "Test Schedule".to_string(),
            regulation: "GDPR".to_string(),
            template_id: "gdpr-monthly".to_string(),
            frequency: ReportFrequency::Daily,
            recipients: vec!["admin@example.com".to_string()],
            enabled: true,
        };

        reporter.add_schedule(schedule);

        let records = create_test_records(10);
        let report = create_test_compliance_report();

        let generated = reporter.generate_due_reports(&records, &report).unwrap();
        assert_eq!(generated.len(), 1);
        assert_eq!(generated[0].regulation, "GDPR");
    }

    #[test]
    fn test_report_frequency_conversion() {
        assert_eq!(ReportFrequency::Daily.to_duration(), Duration::days(1));
        assert_eq!(ReportFrequency::Weekly.to_duration(), Duration::weeks(1));
        assert_eq!(ReportFrequency::Monthly.to_duration(), Duration::days(30));
        assert_eq!(ReportFrequency::Quarterly.to_duration(), Duration::days(90));
        assert_eq!(ReportFrequency::Annual.to_duration(), Duration::days(365));
    }

    #[test]
    fn test_enable_disable_schedule() {
        let mut reporter = AutomatedReporter::new();
        let schedule = ComplianceReportSchedule {
            id: "test-schedule".to_string(),
            name: "Test Schedule".to_string(),
            regulation: "GDPR".to_string(),
            template_id: "gdpr-monthly".to_string(),
            frequency: ReportFrequency::Monthly,
            recipients: vec!["admin@example.com".to_string()],
            enabled: true,
        };

        reporter.add_schedule(schedule);
        reporter
            .set_schedule_enabled("test-schedule", false)
            .unwrap();

        let schedule = reporter.get_schedule("test-schedule").unwrap();
        assert!(!schedule.enabled);
    }

    #[test]
    fn test_default_templates() {
        let reporter = AutomatedReporter::new();
        assert!(reporter.templates.contains_key("gdpr-monthly"));
        assert!(reporter.templates.contains_key("sox-quarterly"));
        assert!(reporter.templates.contains_key("hipaa-annual"));
    }

    #[test]
    fn test_disabled_schedule_not_generated() {
        let mut reporter = AutomatedReporter::new();
        let schedule = ComplianceReportSchedule {
            id: "test-schedule".to_string(),
            name: "Test Schedule".to_string(),
            regulation: "GDPR".to_string(),
            template_id: "gdpr-monthly".to_string(),
            frequency: ReportFrequency::Daily,
            recipients: vec!["admin@example.com".to_string()],
            enabled: false, // Disabled
        };

        reporter.add_schedule(schedule);

        let records = create_test_records(10);
        let report = create_test_compliance_report();

        let generated = reporter.generate_due_reports(&records, &report).unwrap();
        assert_eq!(generated.len(), 0);
    }
}
