//! FEMA Error Types

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum FemaError {
    #[error("FEMA: Compliance error - {reason}")]
    ComplianceError { reason: String },
}

pub type FemaResult<T> = Result<T, FemaError>;

#[derive(Debug, Clone, Default)]
pub struct FemaComplianceReport {
    pub compliant: bool,
    pub errors: Vec<FemaError>,
    pub warnings: Vec<String>,
}
