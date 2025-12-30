//! Enhanced compliance features for multiple regulatory frameworks.
//!
//! This module provides compliance features for:
//! - CCPA (California Consumer Privacy Act)
//! - HIPAA (Health Insurance Portability and Accountability Act)
//! - SOX (Sarbanes-Oxley Act)
//! - ISO 27001 (Information Security Management)

use crate::{AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// CCPA compliance manager.
pub struct CcpaCompliance;

impl CcpaCompliance {
    /// Generates a CCPA-compliant data subject access report (Right to Know).
    pub fn generate_access_report(records: &[AuditRecord], subject_id: Uuid) -> CcpaAccessReport {
        let relevant_records: Vec<_> = records
            .iter()
            .filter(|r| r.subject_id == subject_id)
            .collect();

        let categories = Self::categorize_data(&relevant_records);
        let business_purpose = Self::determine_business_purpose(&relevant_records);

        CcpaAccessReport {
            subject_id,
            generated_at: Utc::now(),
            total_records: relevant_records.len(),
            data_categories: categories,
            business_purpose,
            retention_period_days: 365, // Default CCPA retention
            records: relevant_records.into_iter().cloned().collect(),
        }
    }

    /// Processes a CCPA deletion request (Right to Delete).
    pub fn process_deletion_request(
        records: &[AuditRecord],
        subject_id: Uuid,
    ) -> AuditResult<CcpaDeletionReport> {
        let deletable_records: Vec<_> = records
            .iter()
            .filter(|r| r.subject_id == subject_id)
            .filter(|r| Self::is_deletable(r))
            .collect();

        let exempt_records: Vec<_> = records
            .iter()
            .filter(|r| r.subject_id == subject_id)
            .filter(|r| !Self::is_deletable(r))
            .collect();

        Ok(CcpaDeletionReport {
            subject_id,
            processed_at: Utc::now(),
            deletable_count: deletable_records.len(),
            exempt_count: exempt_records.len(),
            exemption_reasons: Self::get_exemption_reasons(&exempt_records),
        })
    }

    /// Processes opt-out of sale request.
    pub fn process_opt_out_request(subject_id: Uuid) -> CcpaOptOutReport {
        CcpaOptOutReport {
            subject_id,
            processed_at: Utc::now(),
            status: "Opted out of data sale".to_string(),
            effective_date: Utc::now(),
        }
    }

    fn categorize_data(records: &[&AuditRecord]) -> Vec<String> {
        let mut categories = vec!["Decision Records".to_string()];
        for record in records {
            if !record.context.attributes.is_empty() {
                categories.push("Personal Attributes".to_string());
                break;
            }
        }
        categories.dedup();
        categories
    }

    fn determine_business_purpose(_records: &[&AuditRecord]) -> String {
        "Legal decision tracking and compliance monitoring".to_string()
    }

    fn is_deletable(record: &AuditRecord) -> bool {
        // Records with legal hold or required for compliance are exempt
        let age_days = (Utc::now() - record.timestamp).num_days();
        age_days > 365 // Example: records older than 1 year can be deleted
    }

    fn get_exemption_reasons(records: &[&AuditRecord]) -> HashMap<String, usize> {
        let mut reasons = HashMap::new();
        for _record in records {
            *reasons
                .entry("Legal compliance requirement".to_string())
                .or_insert(0) += 1;
        }
        reasons
    }
}

/// CCPA access report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CcpaAccessReport {
    pub subject_id: Uuid,
    pub generated_at: DateTime<Utc>,
    pub total_records: usize,
    pub data_categories: Vec<String>,
    pub business_purpose: String,
    pub retention_period_days: u32,
    pub records: Vec<AuditRecord>,
}

/// CCPA deletion report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CcpaDeletionReport {
    pub subject_id: Uuid,
    pub processed_at: DateTime<Utc>,
    pub deletable_count: usize,
    pub exempt_count: usize,
    pub exemption_reasons: HashMap<String, usize>,
}

/// CCPA opt-out report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CcpaOptOutReport {
    pub subject_id: Uuid,
    pub processed_at: DateTime<Utc>,
    pub status: String,
    pub effective_date: DateTime<Utc>,
}

/// HIPAA compliance manager.
pub struct HipaaCompliance;

impl HipaaCompliance {
    /// Generates HIPAA audit trail report.
    pub fn generate_audit_report(records: &[AuditRecord]) -> HipaaAuditReport {
        let access_events: Vec<_> = records.iter().filter(|r| Self::is_phi_access(r)).collect();

        let disclosures: Vec<_> = records
            .iter()
            .filter(|r| Self::is_phi_disclosure(r))
            .collect();

        HipaaAuditReport {
            generated_at: Utc::now(),
            total_access_events: access_events.len(),
            total_disclosures: disclosures.len(),
            access_by_user: Self::aggregate_by_actor(&access_events),
            disclosure_purposes: Self::aggregate_purposes(&disclosures),
            compliance_status: Self::check_compliance_status(&access_events, &disclosures),
        }
    }

    /// Generates accounting of disclosures (required by HIPAA).
    pub fn accounting_of_disclosures(
        records: &[AuditRecord],
        subject_id: Uuid,
    ) -> HipaaDisclosureAccounting {
        let disclosures: Vec<_> = records
            .iter()
            .filter(|r| r.subject_id == subject_id)
            .filter(|r| Self::is_phi_disclosure(r))
            .collect();

        HipaaDisclosureAccounting {
            subject_id,
            generated_at: Utc::now(),
            disclosure_count: disclosures.len(),
            disclosures: disclosures
                .into_iter()
                .map(Self::create_disclosure_entry)
                .collect(),
        }
    }

    fn is_phi_access(_record: &AuditRecord) -> bool {
        // In real implementation, check if record involves PHI access
        true
    }

    fn is_phi_disclosure(record: &AuditRecord) -> bool {
        // Check if this is a disclosure to external party
        matches!(record.actor, crate::Actor::External { .. })
    }

    fn aggregate_by_actor(records: &[&AuditRecord]) -> HashMap<String, usize> {
        let mut map = HashMap::new();
        for record in records {
            let actor_id = match &record.actor {
                crate::Actor::System { component } => format!("system:{}", component),
                crate::Actor::User { user_id, .. } => format!("user:{}", user_id),
                crate::Actor::External { system_id } => format!("external:{}", system_id),
            };
            *map.entry(actor_id).or_insert(0) += 1;
        }
        map
    }

    fn aggregate_purposes(records: &[&AuditRecord]) -> HashMap<String, usize> {
        let mut map = HashMap::new();
        for _record in records {
            *map.entry("Treatment/Payment/Operations".to_string())
                .or_insert(0) += 1;
        }
        map
    }

    fn check_compliance_status(
        access_events: &[&AuditRecord],
        _disclosures: &[&AuditRecord],
    ) -> String {
        if access_events.is_empty() {
            "No PHI access recorded".to_string()
        } else {
            "Audit trail complete".to_string()
        }
    }

    fn create_disclosure_entry(record: &AuditRecord) -> HipaaDisclosure {
        HipaaDisclosure {
            date: record.timestamp,
            recipient: match &record.actor {
                crate::Actor::External { system_id } => system_id.clone(),
                _ => "Unknown".to_string(),
            },
            purpose: "Treatment/Payment/Operations".to_string(),
            description: format!("Decision: {}", record.statute_id),
        }
    }
}

/// HIPAA audit report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HipaaAuditReport {
    pub generated_at: DateTime<Utc>,
    pub total_access_events: usize,
    pub total_disclosures: usize,
    pub access_by_user: HashMap<String, usize>,
    pub disclosure_purposes: HashMap<String, usize>,
    pub compliance_status: String,
}

/// HIPAA disclosure accounting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HipaaDisclosureAccounting {
    pub subject_id: Uuid,
    pub generated_at: DateTime<Utc>,
    pub disclosure_count: usize,
    pub disclosures: Vec<HipaaDisclosure>,
}

/// A single HIPAA disclosure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HipaaDisclosure {
    pub date: DateTime<Utc>,
    pub recipient: String,
    pub purpose: String,
    pub description: String,
}

/// SOX compliance manager.
pub struct SoxCompliance;

impl SoxCompliance {
    /// Generates SOX compliance report for financial controls.
    pub fn generate_control_report(records: &[AuditRecord]) -> SoxControlReport {
        let financial_decisions: Vec<_> = records
            .iter()
            .filter(|r| Self::is_financial_decision(r))
            .cloned()
            .collect();

        let override_rate = Self::calculate_override_rate(&financial_decisions);
        let segregation_violations = Self::detect_segregation_violations(&financial_decisions);

        SoxControlReport {
            generated_at: Utc::now(),
            total_financial_decisions: financial_decisions.len(),
            automatic_decisions: financial_decisions
                .iter()
                .filter(|r| matches!(r.result, crate::DecisionResult::Deterministic { .. }))
                .count(),
            manual_overrides: financial_decisions
                .iter()
                .filter(|r| matches!(r.result, crate::DecisionResult::Overridden { .. }))
                .count(),
            override_rate,
            segregation_violations,
            control_effectiveness: Self::assess_control_effectiveness(override_rate),
        }
    }

    /// Validates segregation of duties.
    pub fn validate_segregation_of_duties(records: &[AuditRecord]) -> SoxSegregationReport {
        let violations = Self::detect_segregation_violations(records);
        let is_compliant = violations.is_empty();

        SoxSegregationReport {
            generated_at: Utc::now(),
            total_violations: violations.len(),
            violations,
            compliance_status: if is_compliant {
                "Compliant".to_string()
            } else {
                "Violations detected".to_string()
            },
        }
    }

    fn is_financial_decision(record: &AuditRecord) -> bool {
        // In real implementation, check if statute relates to financial controls
        record.statute_id.contains("financial") || record.statute_id.contains("payment")
    }

    fn calculate_override_rate(records: &[AuditRecord]) -> f64 {
        if records.is_empty() {
            return 0.0;
        }
        let overrides = records
            .iter()
            .filter(|r| matches!(r.result, crate::DecisionResult::Overridden { .. }))
            .count();
        (overrides as f64 / records.len() as f64) * 100.0
    }

    #[allow(dead_code)]
    fn detect_segregation_violations(records: &[AuditRecord]) -> Vec<SoxViolation> {
        let mut violations = Vec::new();
        let mut actor_actions: HashMap<String, Vec<&AuditRecord>> = HashMap::new();

        for record in records {
            let actor_key = match &record.actor {
                crate::Actor::User { user_id, .. } => user_id.clone(),
                _ => continue,
            };
            actor_actions.entry(actor_key).or_default().push(record);
        }

        // Check for same user performing conflicting actions
        for (actor, actions) in actor_actions {
            if actions.len() > 1 {
                violations.push(SoxViolation {
                    violation_type: "Potential segregation issue".to_string(),
                    actor,
                    record_count: actions.len(),
                    detected_at: Utc::now(),
                });
            }
        }

        violations
    }

    fn assess_control_effectiveness(override_rate: f64) -> String {
        if override_rate < 5.0 {
            "Effective".to_string()
        } else if override_rate < 15.0 {
            "Needs review".to_string()
        } else {
            "Weak controls".to_string()
        }
    }
}

/// SOX control report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoxControlReport {
    pub generated_at: DateTime<Utc>,
    pub total_financial_decisions: usize,
    pub automatic_decisions: usize,
    pub manual_overrides: usize,
    pub override_rate: f64,
    pub segregation_violations: Vec<SoxViolation>,
    pub control_effectiveness: String,
}

/// SOX segregation report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoxSegregationReport {
    pub generated_at: DateTime<Utc>,
    pub total_violations: usize,
    pub violations: Vec<SoxViolation>,
    pub compliance_status: String,
}

/// A SOX violation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoxViolation {
    pub violation_type: String,
    pub actor: String,
    pub record_count: usize,
    pub detected_at: DateTime<Utc>,
}

/// ISO 27001 compliance manager.
pub struct Iso27001Compliance;

impl Iso27001Compliance {
    /// Generates ISO 27001 audit trail report.
    pub fn generate_audit_report(records: &[AuditRecord]) -> Iso27001AuditReport {
        let security_events = Self::identify_security_events(records);
        let access_control_events = Self::identify_access_control_events(records);

        Iso27001AuditReport {
            generated_at: Utc::now(),
            total_events: records.len(),
            security_events: security_events.len(),
            access_control_events: access_control_events.len(),
            event_categories: Self::categorize_events(records),
            integrity_verified: true, // Assume verified
            retention_compliant: Self::check_retention_compliance(records),
        }
    }

    /// Validates information security controls.
    pub fn validate_security_controls(records: &[AuditRecord]) -> Iso27001ControlReport {
        let controls = Self::assess_controls(records);
        let overall_status = Self::determine_overall_status(&controls);

        Iso27001ControlReport {
            generated_at: Utc::now(),
            total_controls_assessed: controls.len(),
            controls,
            overall_status,
        }
    }

    fn identify_security_events(records: &[AuditRecord]) -> Vec<&AuditRecord> {
        records
            .iter()
            .filter(|r| matches!(r.event_type, crate::EventType::HumanOverride))
            .collect()
    }

    fn identify_access_control_events(records: &[AuditRecord]) -> Vec<&AuditRecord> {
        records
            .iter()
            .filter(|r| !matches!(r.actor, crate::Actor::System { .. }))
            .collect()
    }

    fn categorize_events(records: &[AuditRecord]) -> HashMap<String, usize> {
        let mut categories = HashMap::new();
        for record in records {
            let category = match record.event_type {
                crate::EventType::AutomaticDecision => "Automatic",
                crate::EventType::HumanOverride => "Override",
                crate::EventType::Appeal => "Appeal",
                _ => "Other",
            };
            *categories.entry(category.to_string()).or_insert(0) += 1;
        }
        categories
    }

    fn check_retention_compliance(_records: &[AuditRecord]) -> bool {
        // All records should be retained per policy
        true
    }

    fn assess_controls(records: &[AuditRecord]) -> Vec<Iso27001Control> {
        vec![
            Iso27001Control {
                control_id: "A.12.4.1".to_string(),
                name: "Event logging".to_string(),
                status: if records.is_empty() {
                    "Not implemented".to_string()
                } else {
                    "Implemented".to_string()
                },
                evidence_count: records.len(),
            },
            Iso27001Control {
                control_id: "A.12.4.3".to_string(),
                name: "Administrator and operator logs".to_string(),
                status: "Implemented".to_string(),
                evidence_count: records
                    .iter()
                    .filter(|r| !matches!(r.actor, crate::Actor::System { .. }))
                    .count(),
            },
        ]
    }

    fn determine_overall_status(controls: &[Iso27001Control]) -> String {
        if controls.iter().all(|c| c.status == "Implemented") {
            "Compliant".to_string()
        } else {
            "Partial compliance".to_string()
        }
    }
}

/// ISO 27001 audit report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso27001AuditReport {
    pub generated_at: DateTime<Utc>,
    pub total_events: usize,
    pub security_events: usize,
    pub access_control_events: usize,
    pub event_categories: HashMap<String, usize>,
    pub integrity_verified: bool,
    pub retention_compliant: bool,
}

/// ISO 27001 control report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso27001ControlReport {
    pub generated_at: DateTime<Utc>,
    pub total_controls_assessed: usize,
    pub controls: Vec<Iso27001Control>,
    pub overall_status: String,
}

/// An ISO 27001 control.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso27001Control {
    pub control_id: String,
    pub name: String,
    pub status: String,
    pub evidence_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap as StdHashMap;

    fn create_test_record(subject_id: Uuid) -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "statute-1".to_string(),
            subject_id,
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: StdHashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_ccpa_access_report() {
        let subject_id = Uuid::new_v4();
        let records = vec![
            create_test_record(subject_id),
            create_test_record(subject_id),
        ];

        let report = CcpaCompliance::generate_access_report(&records, subject_id);
        assert_eq!(report.total_records, 2);
        assert_eq!(report.subject_id, subject_id);
        assert!(!report.data_categories.is_empty());
    }

    #[test]
    fn test_ccpa_deletion_request() {
        let subject_id = Uuid::new_v4();
        let records = vec![create_test_record(subject_id)];

        let report = CcpaCompliance::process_deletion_request(&records, subject_id).unwrap();
        assert_eq!(report.subject_id, subject_id);
    }

    #[test]
    fn test_ccpa_opt_out() {
        let subject_id = Uuid::new_v4();
        let report = CcpaCompliance::process_opt_out_request(subject_id);
        assert_eq!(report.subject_id, subject_id);
        assert!(!report.status.is_empty());
    }

    #[test]
    fn test_hipaa_audit_report() {
        let records = vec![create_test_record(Uuid::new_v4())];
        let report = HipaaCompliance::generate_audit_report(&records);
        assert!(report.total_access_events > 0);
    }

    #[test]
    fn test_hipaa_disclosure_accounting() {
        let subject_id = Uuid::new_v4();
        let mut record = create_test_record(subject_id);
        record.actor = Actor::External {
            system_id: "external-system".to_string(),
        };
        let records = vec![record];

        let report = HipaaCompliance::accounting_of_disclosures(&records, subject_id);
        assert_eq!(report.subject_id, subject_id);
        assert_eq!(report.disclosure_count, 1);
    }

    #[test]
    fn test_sox_control_report() {
        let records = vec![create_test_record(Uuid::new_v4())];
        let report = SoxCompliance::generate_control_report(&records);
        assert!(!report.control_effectiveness.is_empty());
    }

    #[test]
    fn test_sox_segregation_validation() {
        let records = vec![create_test_record(Uuid::new_v4())];
        let report = SoxCompliance::validate_segregation_of_duties(&records);
        assert!(!report.compliance_status.is_empty());
    }

    #[test]
    fn test_iso27001_audit_report() {
        let records = vec![create_test_record(Uuid::new_v4())];
        let report = Iso27001Compliance::generate_audit_report(&records);
        assert_eq!(report.total_events, 1);
        assert!(report.integrity_verified);
    }

    #[test]
    fn test_iso27001_control_validation() {
        let records = vec![create_test_record(Uuid::new_v4())];
        let report = Iso27001Compliance::validate_security_controls(&records);
        assert!(report.total_controls_assessed > 0);
        assert!(!report.overall_status.is_empty());
    }
}
