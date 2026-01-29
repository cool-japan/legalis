//! Environmental Law Error Types

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum EnvironmentalActError {
    #[error("Environmental Act: Compliance error - {reason}")]
    ComplianceError { reason: String },
}

pub type EnvironmentalActResult<T> = Result<T, EnvironmentalActError>;

#[derive(Debug, Clone, Default)]
pub struct EnvironmentalComplianceReport {
    pub compliant: bool,
    pub errors: Vec<EnvironmentalActError>,
    pub warnings: Vec<String>,
}
