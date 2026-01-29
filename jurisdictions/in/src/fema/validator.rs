//! FEMA Validation

use super::error::FemaComplianceReport;
use super::types::*;

pub fn validate_fdi(_investment: &ForeignInvestment) -> FemaComplianceReport {
    FemaComplianceReport {
        compliant: true,
        ..Default::default()
    }
}
