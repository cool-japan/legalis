//! IBC Error Types

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum IbcError {
    #[error("IBC: Compliance error - {reason}")]
    ComplianceError { reason: String },
}

pub type IbcResult<T> = Result<T, IbcError>;

#[derive(Debug, Clone, Default)]
pub struct IbcComplianceReport {
    pub compliant: bool,
    pub errors: Vec<IbcError>,
    pub warnings: Vec<String>,
}
