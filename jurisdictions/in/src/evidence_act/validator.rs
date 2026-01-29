//! Evidence Act Validation

use super::error::{EvidenceActError, EvidenceActResult, EvidenceComplianceReport};
use super::types::*;

pub fn validate_evidence_admissibility(evidence: &Evidence) -> EvidenceActResult<()> {
    match evidence.admissibility {
        Admissibility::Admissible => Ok(()),
        Admissibility::Inadmissible => Err(EvidenceActError::Inadmissible {
            reason: format!("{:?} evidence is inadmissible", evidence.evidence_type),
        }),
        Admissibility::ConditionallyAdmissible => Ok(()),
    }
}

pub fn check_hearsay_exception(evidence: &Evidence) -> bool {
    matches!(evidence.evidence_type, EvidenceType::Oral)
}

pub fn validate_electronic_evidence(evidence: &Evidence) -> EvidenceComplianceReport {
    let mut report = EvidenceComplianceReport {
        compliant: true,
        admissible: true,
        ..Default::default()
    };

    if matches!(evidence.evidence_type, EvidenceType::Electronic) {
        report.warnings.push(
            "Electronic evidence must comply with Section 65B certificate requirements".to_string(),
        );
    }

    report
}
