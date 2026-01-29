//! Competition Act Error Types

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum CompetitionActError {
    #[error("Competition Act: Compliance error - {reason}")]
    ComplianceError { reason: String },
}

pub type CompetitionActResult<T> = Result<T, CompetitionActError>;

#[derive(Debug, Clone, Default)]
pub struct CompetitionComplianceReport {
    pub compliant: bool,
    pub errors: Vec<CompetitionActError>,
    pub warnings: Vec<String>,
}
