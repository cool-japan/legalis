//! Multi-regulation tracking system.
//!
//! Tracks compliance status across multiple regulatory frameworks
//! (GDPR, SOX, HIPAA, CCPA, etc.) and provides unified compliance views.

use crate::{AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Multi-regulation compliance tracker
pub struct MultiRegulationTracker {
    regulations: HashMap<String, RegulationCompliance>,
    audit_mappings: HashMap<RegulationType, Vec<ComplianceRequirement>>,
}

impl MultiRegulationTracker {
    /// Create a new multi-regulation tracker
    pub fn new() -> Self {
        Self {
            regulations: HashMap::new(),
            audit_mappings: Self::default_mappings(),
        }
    }

    /// Register a regulation for tracking
    pub fn register_regulation(&mut self, regulation: RegulationCompliance) {
        self.regulations
            .insert(regulation.regulation_type.to_string(), regulation);
    }

    /// Update compliance status for a regulation
    pub fn update_compliance(
        &mut self,
        regulation_type: RegulationType,
        records: &[AuditRecord],
    ) -> AuditResult<ComplianceStatus> {
        let requirements = self.audit_mappings.get(&regulation_type).ok_or_else(|| {
            crate::AuditError::InvalidRecord(format!("Unknown regulation: {:?}", regulation_type))
        })?;

        let mut met_requirements = 0;
        let total_requirements = requirements.len();
        let mut issues = Vec::new();

        for requirement in requirements {
            if self.check_requirement(requirement, records) {
                met_requirements += 1;
            } else {
                issues.push(requirement.id.clone());
            }
        }

        let compliance_percentage = if total_requirements > 0 {
            (met_requirements as f64 / total_requirements as f64) * 100.0
        } else {
            100.0
        };

        let status = if compliance_percentage == 100.0 {
            ComplianceStatus::Compliant
        } else if compliance_percentage >= 75.0 {
            ComplianceStatus::PartiallyCompliant
        } else {
            ComplianceStatus::NonCompliant
        };

        // Update regulation compliance
        if let Some(regulation) = self.regulations.get_mut(&regulation_type.to_string()) {
            regulation.status = status;
            regulation.last_assessed = Utc::now();
            regulation.compliance_percentage = compliance_percentage;
            regulation.outstanding_issues = issues;
        }

        Ok(status)
    }

    /// Get compliance status for a regulation
    pub fn get_compliance(
        &self,
        regulation_type: &RegulationType,
    ) -> Option<&RegulationCompliance> {
        self.regulations.get(&regulation_type.to_string())
    }

    /// Get all tracked regulations
    pub fn get_all_regulations(&self) -> Vec<&RegulationCompliance> {
        self.regulations.values().collect()
    }

    /// Get overall compliance summary
    pub fn get_compliance_summary(&self) -> ComplianceSummary {
        let total_regulations = self.regulations.len();
        let compliant = self
            .regulations
            .values()
            .filter(|r| r.status == ComplianceStatus::Compliant)
            .count();
        let partially_compliant = self
            .regulations
            .values()
            .filter(|r| r.status == ComplianceStatus::PartiallyCompliant)
            .count();
        let non_compliant = self
            .regulations
            .values()
            .filter(|r| r.status == ComplianceStatus::NonCompliant)
            .count();

        let average_compliance = if total_regulations > 0 {
            self.regulations
                .values()
                .map(|r| r.compliance_percentage)
                .sum::<f64>()
                / total_regulations as f64
        } else {
            0.0
        };

        ComplianceSummary {
            total_regulations,
            compliant,
            partially_compliant,
            non_compliant,
            average_compliance,
            last_updated: Utc::now(),
        }
    }

    /// Check if a requirement is met
    fn check_requirement(
        &self,
        requirement: &ComplianceRequirement,
        records: &[AuditRecord],
    ) -> bool {
        match requirement.requirement_type {
            RequirementType::AuditTrailExists => !records.is_empty(),
            RequirementType::IntegrityVerification => {
                // Simplified check - in production, actually verify integrity
                records.iter().all(|r| r.verify())
            }
            RequirementType::MinimumRetention => {
                // Check if oldest record meets retention requirement
                records
                    .first()
                    .map(|r| {
                        let age = Utc::now().signed_duration_since(r.timestamp);
                        age.num_days() >= requirement.threshold as i64
                    })
                    .unwrap_or(false)
            }
            RequirementType::AccessControl => {
                // Check if access control is enforced (all records have actors)
                records
                    .iter()
                    .all(|r| !matches!(r.actor, crate::Actor::System { .. }))
            }
            RequirementType::DataMinimization => {
                // Check if data minimization is applied
                true // Placeholder
            }
        }
    }

    /// Get default compliance requirement mappings
    fn default_mappings() -> HashMap<RegulationType, Vec<ComplianceRequirement>> {
        let mut mappings = HashMap::new();

        mappings.insert(
            RegulationType::GDPR,
            vec![
                ComplianceRequirement {
                    id: "gdpr-1".to_string(),
                    name: "Audit trail exists".to_string(),
                    requirement_type: RequirementType::AuditTrailExists,
                    threshold: 0,
                },
                ComplianceRequirement {
                    id: "gdpr-2".to_string(),
                    name: "Integrity verification".to_string(),
                    requirement_type: RequirementType::IntegrityVerification,
                    threshold: 0,
                },
                ComplianceRequirement {
                    id: "gdpr-3".to_string(),
                    name: "Data minimization".to_string(),
                    requirement_type: RequirementType::DataMinimization,
                    threshold: 0,
                },
            ],
        );

        mappings.insert(
            RegulationType::SOX,
            vec![
                ComplianceRequirement {
                    id: "sox-1".to_string(),
                    name: "Audit trail exists".to_string(),
                    requirement_type: RequirementType::AuditTrailExists,
                    threshold: 0,
                },
                ComplianceRequirement {
                    id: "sox-2".to_string(),
                    name: "Integrity verification".to_string(),
                    requirement_type: RequirementType::IntegrityVerification,
                    threshold: 0,
                },
                ComplianceRequirement {
                    id: "sox-3".to_string(),
                    name: "7-year retention".to_string(),
                    requirement_type: RequirementType::MinimumRetention,
                    threshold: 2555, // 7 years in days
                },
            ],
        );

        mappings.insert(
            RegulationType::HIPAA,
            vec![
                ComplianceRequirement {
                    id: "hipaa-1".to_string(),
                    name: "Audit trail exists".to_string(),
                    requirement_type: RequirementType::AuditTrailExists,
                    threshold: 0,
                },
                ComplianceRequirement {
                    id: "hipaa-2".to_string(),
                    name: "Access control".to_string(),
                    requirement_type: RequirementType::AccessControl,
                    threshold: 0,
                },
                ComplianceRequirement {
                    id: "hipaa-3".to_string(),
                    name: "6-year retention".to_string(),
                    requirement_type: RequirementType::MinimumRetention,
                    threshold: 2190, // 6 years in days
                },
            ],
        );

        mappings.insert(
            RegulationType::CCPA,
            vec![
                ComplianceRequirement {
                    id: "ccpa-1".to_string(),
                    name: "Audit trail exists".to_string(),
                    requirement_type: RequirementType::AuditTrailExists,
                    threshold: 0,
                },
                ComplianceRequirement {
                    id: "ccpa-2".to_string(),
                    name: "Data minimization".to_string(),
                    requirement_type: RequirementType::DataMinimization,
                    threshold: 0,
                },
            ],
        );

        mappings
    }
}

impl Default for MultiRegulationTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Regulation compliance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulationCompliance {
    pub regulation_type: RegulationType,
    pub status: ComplianceStatus,
    pub last_assessed: DateTime<Utc>,
    pub compliance_percentage: f64,
    pub outstanding_issues: Vec<String>,
}

/// Regulation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegulationType {
    GDPR,
    SOX,
    HIPAA,
    CCPA,
    PciDss,
    Iso27001,
}

impl std::fmt::Display for RegulationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegulationType::GDPR => write!(f, "GDPR"),
            RegulationType::SOX => write!(f, "SOX"),
            RegulationType::HIPAA => write!(f, "HIPAA"),
            RegulationType::CCPA => write!(f, "CCPA"),
            RegulationType::PciDss => write!(f, "PCI_DSS"),
            RegulationType::Iso27001 => write!(f, "ISO27001"),
        }
    }
}

/// Compliance status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    PartiallyCompliant,
    NonCompliant,
    Unknown,
}

/// Compliance requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirement {
    pub id: String,
    pub name: String,
    pub requirement_type: RequirementType,
    pub threshold: usize,
}

/// Requirement type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequirementType {
    AuditTrailExists,
    IntegrityVerification,
    MinimumRetention,
    AccessControl,
    DataMinimization,
}

/// Compliance summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceSummary {
    pub total_regulations: usize,
    pub compliant: usize,
    pub partially_compliant: usize,
    pub non_compliant: usize,
    pub average_compliance: f64,
    pub last_updated: DateTime<Utc>,
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
                    Actor::User {
                        user_id: "user123".to_string(),
                        role: "admin".to_string(),
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

    #[test]
    fn test_multi_regulation_tracker_creation() {
        let tracker = MultiRegulationTracker::new();
        assert!(tracker.regulations.is_empty());
        assert!(!tracker.audit_mappings.is_empty());
    }

    #[test]
    fn test_register_regulation() {
        let mut tracker = MultiRegulationTracker::new();
        let regulation = RegulationCompliance {
            regulation_type: RegulationType::GDPR,
            status: ComplianceStatus::Unknown,
            last_assessed: Utc::now(),
            compliance_percentage: 0.0,
            outstanding_issues: Vec::new(),
        };

        tracker.register_regulation(regulation);
        assert_eq!(tracker.regulations.len(), 1);
    }

    #[test]
    fn test_update_compliance() {
        let mut tracker = MultiRegulationTracker::new();
        let regulation = RegulationCompliance {
            regulation_type: RegulationType::GDPR,
            status: ComplianceStatus::Unknown,
            last_assessed: Utc::now(),
            compliance_percentage: 0.0,
            outstanding_issues: Vec::new(),
        };

        tracker.register_regulation(regulation);

        let records = create_test_records(10);
        let status = tracker
            .update_compliance(RegulationType::GDPR, &records)
            .unwrap();

        assert_ne!(status, ComplianceStatus::Unknown);
    }

    #[test]
    fn test_get_compliance() {
        let mut tracker = MultiRegulationTracker::new();
        let regulation = RegulationCompliance {
            regulation_type: RegulationType::SOX,
            status: ComplianceStatus::Compliant,
            last_assessed: Utc::now(),
            compliance_percentage: 100.0,
            outstanding_issues: Vec::new(),
        };

        tracker.register_regulation(regulation);

        let compliance = tracker.get_compliance(&RegulationType::SOX).unwrap();
        assert_eq!(compliance.status, ComplianceStatus::Compliant);
    }

    #[test]
    fn test_compliance_summary() {
        let mut tracker = MultiRegulationTracker::new();

        tracker.register_regulation(RegulationCompliance {
            regulation_type: RegulationType::GDPR,
            status: ComplianceStatus::Compliant,
            last_assessed: Utc::now(),
            compliance_percentage: 100.0,
            outstanding_issues: Vec::new(),
        });

        tracker.register_regulation(RegulationCompliance {
            regulation_type: RegulationType::SOX,
            status: ComplianceStatus::PartiallyCompliant,
            last_assessed: Utc::now(),
            compliance_percentage: 75.0,
            outstanding_issues: vec!["sox-3".to_string()],
        });

        tracker.register_regulation(RegulationCompliance {
            regulation_type: RegulationType::HIPAA,
            status: ComplianceStatus::NonCompliant,
            last_assessed: Utc::now(),
            compliance_percentage: 50.0,
            outstanding_issues: vec!["hipaa-2".to_string(), "hipaa-3".to_string()],
        });

        let summary = tracker.get_compliance_summary();
        assert_eq!(summary.total_regulations, 3);
        assert_eq!(summary.compliant, 1);
        assert_eq!(summary.partially_compliant, 1);
        assert_eq!(summary.non_compliant, 1);
        assert_eq!(summary.average_compliance, 75.0);
    }

    #[test]
    fn test_regulation_type_display() {
        assert_eq!(RegulationType::GDPR.to_string(), "GDPR");
        assert_eq!(RegulationType::SOX.to_string(), "SOX");
        assert_eq!(RegulationType::HIPAA.to_string(), "HIPAA");
    }

    #[test]
    fn test_default_mappings() {
        let tracker = MultiRegulationTracker::new();
        assert!(tracker.audit_mappings.contains_key(&RegulationType::GDPR));
        assert!(tracker.audit_mappings.contains_key(&RegulationType::SOX));
        assert!(tracker.audit_mappings.contains_key(&RegulationType::HIPAA));
        assert!(tracker.audit_mappings.contains_key(&RegulationType::CCPA));
    }

    #[test]
    fn test_empty_records_compliance() {
        let mut tracker = MultiRegulationTracker::new();
        tracker.register_regulation(RegulationCompliance {
            regulation_type: RegulationType::GDPR,
            status: ComplianceStatus::Unknown,
            last_assessed: Utc::now(),
            compliance_percentage: 0.0,
            outstanding_issues: Vec::new(),
        });

        let records: Vec<AuditRecord> = vec![];
        let status = tracker
            .update_compliance(RegulationType::GDPR, &records)
            .unwrap();

        assert_eq!(status, ComplianceStatus::NonCompliant);
    }
}
