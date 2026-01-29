//! Competition Act Validation

use super::error::CompetitionComplianceReport;
use super::types::*;

pub fn validate_combination(_combination: &CombinationNotification) -> CompetitionComplianceReport {
    CompetitionComplianceReport {
        compliant: true,
        ..Default::default()
    }
}
