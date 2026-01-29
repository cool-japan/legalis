//! Arbitration Act Validation

use super::error::ArbitrationComplianceReport;
use super::types::*;

pub fn validate_arbitration_agreement(
    _agreement: &ArbitrationAgreement,
) -> ArbitrationComplianceReport {
    ArbitrationComplianceReport {
        compliant: true,
        ..Default::default()
    }
}
