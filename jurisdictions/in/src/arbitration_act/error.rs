//! Arbitration Act Error Types

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ArbitrationActError {
    #[error("Arbitration Act: Compliance error - {reason}")]
    ComplianceError { reason: String },
}

pub type ArbitrationActResult<T> = Result<T, ArbitrationActError>;

#[derive(Debug, Clone, Default)]
pub struct ArbitrationComplianceReport {
    pub compliant: bool,
    pub errors: Vec<ArbitrationActError>,
    pub warnings: Vec<String>,
}
