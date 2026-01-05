//! Regulatory audit log export functionality.
//!
//! This module provides standardized export formats for regulatory compliance,
//! including formats specific to various regulatory bodies (GDPR, CCPA, SOX, etc.).

use crate::{Actor, AuditError, AuditRecord, AuditResult, ComplianceReport, EventType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// Regulatory export format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegulatoryFormat {
    /// Generic CSV format suitable for most regulators
    StandardCsv,
    /// Detailed JSON format with full context
    DetailedJson,
    /// XML format (common for government agencies)
    Xml,
    /// GDPR-compliant export (Article 15)
    GdprCompliant,
    /// SOX-compliant audit trail export
    SoxCompliant,
    /// HIPAA-compliant audit log
    HipaaCompliant,
}

/// Regulatory export configuration.
#[derive(Debug, Clone)]
pub struct ExportConfig {
    /// Export format
    pub format: RegulatoryFormat,
    /// Include full decision context
    pub include_context: bool,
    /// Include actor details
    pub include_actor_details: bool,
    /// Include integrity verification data
    pub include_integrity_data: bool,
    /// Time range filter
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Statute ID filter
    pub statute_filter: Option<Vec<String>>,
}

impl ExportConfig {
    /// Creates a new export configuration.
    pub fn new(format: RegulatoryFormat) -> Self {
        Self {
            format,
            include_context: true,
            include_actor_details: true,
            include_integrity_data: true,
            time_range: None,
            statute_filter: None,
        }
    }

    /// Sets whether to include decision context.
    pub fn with_context(mut self, include: bool) -> Self {
        self.include_context = include;
        self
    }

    /// Sets whether to include actor details.
    pub fn with_actor_details(mut self, include: bool) -> Self {
        self.include_actor_details = include;
        self
    }

    /// Sets whether to include integrity verification data.
    pub fn with_integrity_data(mut self, include: bool) -> Self {
        self.include_integrity_data = include;
        self
    }

    /// Sets a time range filter.
    pub fn with_time_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.time_range = Some((start, end));
        self
    }

    /// Sets a statute ID filter.
    pub fn with_statute_filter(mut self, statutes: Vec<String>) -> Self {
        self.statute_filter = Some(statutes);
        self
    }

    /// Filters records based on configuration.
    pub fn filter_records<'a>(&self, records: &'a [AuditRecord]) -> Vec<&'a AuditRecord> {
        records
            .iter()
            .filter(|r| {
                // Time range filter
                if let Some((start, end)) = self.time_range {
                    if r.timestamp < start || r.timestamp > end {
                        return false;
                    }
                }

                // Statute filter
                if let Some(ref statutes) = self.statute_filter {
                    if !statutes.contains(&r.statute_id) {
                        return false;
                    }
                }

                true
            })
            .collect()
    }
}

/// Regulatory audit log exporter.
pub struct RegulatoryExporter;

impl RegulatoryExporter {
    /// Exports audit records in the specified regulatory format.
    pub fn export(
        records: &[AuditRecord],
        config: &ExportConfig,
        report: &ComplianceReport,
    ) -> AuditResult<String> {
        let filtered_records = config.filter_records(records);

        match config.format {
            RegulatoryFormat::StandardCsv => Self::export_standard_csv(&filtered_records, config),
            RegulatoryFormat::DetailedJson => {
                Self::export_detailed_json(&filtered_records, config, report)
            }
            RegulatoryFormat::Xml => Self::export_xml(&filtered_records, config, report),
            RegulatoryFormat::GdprCompliant => Self::export_gdpr(&filtered_records, config),
            RegulatoryFormat::SoxCompliant => Self::export_sox(&filtered_records, config, report),
            RegulatoryFormat::HipaaCompliant => Self::export_hipaa(&filtered_records, config),
        }
    }

    /// Exports in standard CSV format.
    fn export_standard_csv(records: &[&AuditRecord], config: &ExportConfig) -> AuditResult<String> {
        let mut output = Vec::new();

        // Write header
        writeln!(
            output,
            "record_id,timestamp,event_type,actor_type,actor_id,statute_id,subject_id,decision_outcome,{}{}",
            if config.include_context {
                "context,"
            } else {
                ""
            },
            if config.include_integrity_data {
                "record_hash,previous_hash"
            } else {
                ""
            }
        )?;

        // Write records
        for record in records {
            let actor_info = Self::format_actor(&record.actor);
            let outcome = Self::format_outcome(record);

            write!(
                output,
                "{},{},{},{},{},{},{},{}",
                record.id,
                record.timestamp.to_rfc3339(),
                Self::format_event_type(&record.event_type),
                actor_info.0,
                actor_info.1,
                record.statute_id,
                record.subject_id,
                outcome
            )?;

            if config.include_context {
                write!(output, ",\"{}\"", serde_json::to_string(&record.context)?)?;
            }

            if config.include_integrity_data {
                write!(
                    output,
                    ",{},{}",
                    record.record_hash,
                    record.previous_hash.as_deref().unwrap_or("")
                )?;
            }

            writeln!(output)?;
        }

        String::from_utf8(output).map_err(|e| AuditError::ExportError(e.to_string()))
    }

    /// Exports in detailed JSON format.
    fn export_detailed_json(
        records: &[&AuditRecord],
        config: &ExportConfig,
        report: &ComplianceReport,
    ) -> AuditResult<String> {
        #[derive(Serialize)]
        struct Export<'a> {
            export_metadata: ExportMetadata,
            compliance_summary: &'a ComplianceReport,
            audit_records: Vec<&'a AuditRecord>,
        }

        let export = Export {
            export_metadata: ExportMetadata {
                export_timestamp: Utc::now(),
                record_count: records.len(),
                format: "detailed_json".to_string(),
                includes_context: config.include_context,
                includes_integrity_data: config.include_integrity_data,
            },
            compliance_summary: report,
            audit_records: records.to_vec(),
        };

        Ok(serde_json::to_string_pretty(&export)?)
    }

    /// Exports in XML format.
    fn export_xml(
        records: &[&AuditRecord],
        config: &ExportConfig,
        report: &ComplianceReport,
    ) -> AuditResult<String> {
        let mut output = String::new();
        output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        output.push_str("<audit_log_export>\n");

        // Metadata
        output.push_str("  <export_metadata>\n");
        output.push_str(&format!(
            "    <timestamp>{}</timestamp>\n",
            Utc::now().to_rfc3339()
        ));
        output.push_str(&format!(
            "    <record_count>{}</record_count>\n",
            records.len()
        ));
        output.push_str("  </export_metadata>\n");

        // Compliance summary
        output.push_str("  <compliance_summary>\n");
        output.push_str(&format!(
            "    <total_decisions>{}</total_decisions>\n",
            report.total_decisions
        ));
        output.push_str(&format!(
            "    <automatic_decisions>{}</automatic_decisions>\n",
            report.automatic_decisions
        ));
        output.push_str(&format!(
            "    <discretionary_decisions>{}</discretionary_decisions>\n",
            report.discretionary_decisions
        ));
        output.push_str(&format!(
            "    <human_overrides>{}</human_overrides>\n",
            report.human_overrides
        ));
        output.push_str(&format!(
            "    <integrity_verified>{}</integrity_verified>\n",
            report.integrity_verified
        ));
        output.push_str("  </compliance_summary>\n");

        // Records
        output.push_str("  <audit_records>\n");
        for record in records {
            output.push_str("    <record>\n");
            output.push_str(&format!("      <id>{}</id>\n", record.id));
            output.push_str(&format!(
                "      <timestamp>{}</timestamp>\n",
                record.timestamp.to_rfc3339()
            ));
            output.push_str(&format!(
                "      <event_type>{}</event_type>\n",
                Self::format_event_type(&record.event_type)
            ));
            output.push_str(&format!(
                "      <statute_id>{}</statute_id>\n",
                xml_escape(&record.statute_id)
            ));
            output.push_str(&format!(
                "      <subject_id>{}</subject_id>\n",
                record.subject_id
            ));

            let actor_info = Self::format_actor(&record.actor);
            output.push_str("      <actor>\n");
            output.push_str(&format!("        <type>{}</type>\n", actor_info.0));
            output.push_str(&format!("        <id>{}</id>\n", xml_escape(&actor_info.1)));
            output.push_str("      </actor>\n");

            if config.include_integrity_data {
                output.push_str("      <integrity>\n");
                output.push_str(&format!(
                    "        <record_hash>{}</record_hash>\n",
                    record.record_hash
                ));
                if let Some(ref prev) = record.previous_hash {
                    output.push_str(&format!(
                        "        <previous_hash>{}</previous_hash>\n",
                        prev
                    ));
                }
                output.push_str("      </integrity>\n");
            }

            output.push_str("    </record>\n");
        }
        output.push_str("  </audit_records>\n");
        output.push_str("</audit_log_export>\n");

        Ok(output)
    }

    /// Exports in GDPR-compliant format.
    fn export_gdpr(records: &[&AuditRecord], _config: &ExportConfig) -> AuditResult<String> {
        #[derive(Serialize)]
        struct GdprExport<'a> {
            controller_information: ControllerInfo,
            data_subject_rights: DataSubjectRights,
            processing_activities: Vec<ProcessingActivity<'a>>,
            legal_basis: String,
        }

        #[derive(Serialize)]
        struct ControllerInfo {
            export_date: DateTime<Utc>,
            data_retention_period: String,
        }

        #[derive(Serialize)]
        struct DataSubjectRights {
            right_to_access: bool,
            right_to_rectification: bool,
            right_to_erasure: bool,
            right_to_explanation: bool,
        }

        #[derive(Serialize)]
        struct ProcessingActivity<'a> {
            record: &'a AuditRecord,
            purpose: String,
            legal_basis: String,
        }

        let processing_activities: Vec<_> = records
            .iter()
            .map(|r| ProcessingActivity {
                record: r,
                purpose: "Automated legal decision processing".to_string(),
                legal_basis: "Article 6(1)(c) - Legal obligation".to_string(),
            })
            .collect();

        let export = GdprExport {
            controller_information: ControllerInfo {
                export_date: Utc::now(),
                data_retention_period: "As per regulatory requirements".to_string(),
            },
            data_subject_rights: DataSubjectRights {
                right_to_access: true,
                right_to_rectification: false,
                right_to_erasure: true,
                right_to_explanation: true,
            },
            processing_activities,
            legal_basis: "Compliance with legal obligations".to_string(),
        };

        Ok(serde_json::to_string_pretty(&export)?)
    }

    /// Exports in SOX-compliant format.
    fn export_sox(
        records: &[&AuditRecord],
        _config: &ExportConfig,
        report: &ComplianceReport,
    ) -> AuditResult<String> {
        #[derive(Serialize)]
        struct SoxExport<'a> {
            control_information: SoxControlInfo,
            audit_trail: Vec<SoxAuditEntry<'a>>,
            control_effectiveness: ControlEffectiveness,
        }

        #[derive(Serialize)]
        struct SoxControlInfo {
            control_id: String,
            control_description: String,
            control_owner: String,
            review_date: DateTime<Utc>,
        }

        #[derive(Serialize)]
        struct SoxAuditEntry<'a> {
            transaction_id: String,
            timestamp: DateTime<Utc>,
            user: String,
            action: String,
            business_process: String,
            record: &'a AuditRecord,
        }

        #[derive(Serialize)]
        struct ControlEffectiveness {
            total_transactions: usize,
            automated_controls: usize,
            manual_reviews: usize,
            exceptions: usize,
            integrity_verified: bool,
        }

        let audit_trail: Vec<_> = records
            .iter()
            .map(|r| {
                let user = match &r.actor {
                    Actor::User { user_id, .. } => user_id.clone(),
                    Actor::System { component } => format!("SYSTEM:{}", component),
                    Actor::External { system_id } => format!("EXTERNAL:{}", system_id),
                };

                SoxAuditEntry {
                    transaction_id: r.id.to_string(),
                    timestamp: r.timestamp,
                    user,
                    action: Self::format_event_type(&r.event_type),
                    business_process: format!("Legal Decision: {}", r.statute_id),
                    record: r,
                }
            })
            .collect();

        let export = SoxExport {
            control_information: SoxControlInfo {
                control_id: "LEGAL-AUD-001".to_string(),
                control_description: "Automated legal decision audit trail".to_string(),
                control_owner: "Compliance Department".to_string(),
                review_date: Utc::now(),
            },
            audit_trail,
            control_effectiveness: ControlEffectiveness {
                total_transactions: report.total_decisions,
                automated_controls: report.automatic_decisions,
                manual_reviews: report.discretionary_decisions,
                exceptions: report.human_overrides,
                integrity_verified: report.integrity_verified,
            },
        };

        Ok(serde_json::to_string_pretty(&export)?)
    }

    /// Exports in HIPAA-compliant format.
    fn export_hipaa(records: &[&AuditRecord], _config: &ExportConfig) -> AuditResult<String> {
        #[derive(Serialize)]
        struct HipaaExport<'a> {
            security_rule_compliance: SecurityRuleInfo,
            access_log: Vec<HipaaAccessEntry<'a>>,
        }

        #[derive(Serialize)]
        struct SecurityRuleInfo {
            audit_controls: String,
            integrity_controls: String,
            transmission_security: String,
        }

        #[derive(Serialize)]
        struct HipaaAccessEntry<'a> {
            date_time: DateTime<Utc>,
            user_id: String,
            access_type: String,
            phi_accessed: String,
            purpose: String,
            record: &'a AuditRecord,
        }

        let access_log: Vec<_> = records
            .iter()
            .map(|r| {
                let user_id = match &r.actor {
                    Actor::User { user_id, .. } => user_id.clone(),
                    Actor::System { component } => format!("SYSTEM:{}", component),
                    Actor::External { system_id } => format!("EXTERNAL:{}", system_id),
                };

                HipaaAccessEntry {
                    date_time: r.timestamp,
                    user_id,
                    access_type: Self::format_event_type(&r.event_type),
                    phi_accessed: format!("Subject: {}", r.subject_id),
                    purpose: "Legal decision processing".to_string(),
                    record: r,
                }
            })
            .collect();

        let export = HipaaExport {
            security_rule_compliance: SecurityRuleInfo {
                audit_controls: "45 CFR 164.312(b) - Implemented".to_string(),
                integrity_controls: "45 CFR 164.312(c)(1) - Implemented with hash chain"
                    .to_string(),
                transmission_security: "45 CFR 164.312(e)(1) - Encryption at rest available"
                    .to_string(),
            },
            access_log,
        };

        Ok(serde_json::to_string_pretty(&export)?)
    }

    /// Formats an actor as (type, id).
    fn format_actor(actor: &Actor) -> (String, String) {
        match actor {
            Actor::System { component } => ("System".to_string(), component.clone()),
            Actor::User { user_id, .. } => ("User".to_string(), user_id.clone()),
            Actor::External { system_id } => ("External".to_string(), system_id.clone()),
        }
    }

    /// Formats an event type.
    fn format_event_type(event_type: &EventType) -> String {
        match event_type {
            EventType::AutomaticDecision => "AutomaticDecision".to_string(),
            EventType::DiscretionaryReview => "DiscretionaryReview".to_string(),
            EventType::HumanOverride => "HumanOverride".to_string(),
            EventType::Appeal => "Appeal".to_string(),
            EventType::StatuteModified => "StatuteModified".to_string(),
            EventType::SimulationRun => "SimulationRun".to_string(),
        }
    }

    /// Formats the decision outcome.
    fn format_outcome(record: &AuditRecord) -> String {
        match &record.result {
            crate::DecisionResult::Deterministic { effect_applied, .. } => {
                format!("Deterministic:{}", effect_applied)
            }
            crate::DecisionResult::RequiresDiscretion { issue, .. } => {
                format!("RequiresDiscretion:{}", issue)
            }
            crate::DecisionResult::Void { reason } => format!("Void:{}", reason),
            crate::DecisionResult::Overridden { .. } => "Overridden".to_string(),
        }
    }
}

/// Export metadata.
#[derive(Debug, Serialize, Deserialize)]
struct ExportMetadata {
    export_timestamp: DateTime<Utc>,
    record_count: usize,
    format: String,
    includes_context: bool,
    includes_integrity_data: bool,
}

/// Escapes XML special characters.
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DecisionContext, DecisionResult};
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn test_export_standard_csv() {
        let records = create_test_records();
        let config = ExportConfig::new(RegulatoryFormat::StandardCsv);
        let report = create_test_report();

        let output = RegulatoryExporter::export(&records, &config, &report).unwrap();
        assert!(output.contains("record_id,timestamp"));
        assert!(output.contains("AutomaticDecision"));
    }

    #[test]
    fn test_export_detailed_json() {
        let records = create_test_records();
        let config = ExportConfig::new(RegulatoryFormat::DetailedJson);
        let report = create_test_report();

        let output = RegulatoryExporter::export(&records, &config, &report).unwrap();
        assert!(output.contains("export_metadata"));
        assert!(output.contains("compliance_summary"));
        assert!(output.contains("audit_records"));
    }

    #[test]
    fn test_export_xml() {
        let records = create_test_records();
        let config = ExportConfig::new(RegulatoryFormat::Xml);
        let report = create_test_report();

        let output = RegulatoryExporter::export(&records, &config, &report).unwrap();
        assert!(output.contains("<?xml version=\"1.0\""));
        assert!(output.contains("<audit_log_export>"));
        assert!(output.contains("</audit_log_export>"));
    }

    #[test]
    fn test_export_gdpr() {
        let records = create_test_records();
        let config = ExportConfig::new(RegulatoryFormat::GdprCompliant);
        let report = create_test_report();

        let output = RegulatoryExporter::export(&records, &config, &report).unwrap();
        assert!(output.contains("controller_information"));
        assert!(output.contains("data_subject_rights"));
        assert!(output.contains("processing_activities"));
    }

    #[test]
    fn test_export_sox() {
        let records = create_test_records();
        let config = ExportConfig::new(RegulatoryFormat::SoxCompliant);
        let report = create_test_report();

        let output = RegulatoryExporter::export(&records, &config, &report).unwrap();
        assert!(output.contains("control_information"));
        assert!(output.contains("control_effectiveness"));
    }

    #[test]
    fn test_export_hipaa() {
        let records = create_test_records();
        let config = ExportConfig::new(RegulatoryFormat::HipaaCompliant);
        let report = create_test_report();

        let output = RegulatoryExporter::export(&records, &config, &report).unwrap();
        assert!(output.contains("security_rule_compliance"));
        assert!(output.contains("access_log"));
    }

    #[test]
    fn test_time_range_filter() {
        let records = create_test_records();
        let now = Utc::now();
        let config = ExportConfig::new(RegulatoryFormat::StandardCsv).with_time_range(
            now - chrono::Duration::hours(1),
            now + chrono::Duration::hours(1),
        );

        let filtered = config.filter_records(&records);
        assert!(!filtered.is_empty());
    }

    fn create_test_records() -> Vec<AuditRecord> {
        vec![
            AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "test".to_string(),
                },
                "statute-1".to_string(),
                Uuid::new_v4(),
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "approved".to_string(),
                    parameters: HashMap::new(),
                },
                None,
            ),
            AuditRecord::new(
                EventType::DiscretionaryReview,
                Actor::User {
                    user_id: "user-123".to_string(),
                    role: "admin".to_string(),
                },
                "statute-2".to_string(),
                Uuid::new_v4(),
                DecisionContext::default(),
                DecisionResult::RequiresDiscretion {
                    issue: "complex case".to_string(),
                    narrative_hint: None,
                    assigned_to: Some("reviewer-1".to_string()),
                },
                None,
            ),
        ]
    }

    fn create_test_report() -> ComplianceReport {
        ComplianceReport {
            total_decisions: 2,
            automatic_decisions: 1,
            discretionary_decisions: 1,
            human_overrides: 0,
            integrity_verified: true,
            generated_at: Utc::now(),
        }
    }
}
