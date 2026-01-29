//! Evidence Act Error Types

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum EvidenceActError {
    #[error("BSA: Evidence inadmissible - {reason}")]
    Inadmissible { reason: String },

    #[error("BSA Section 45: Expert opinion not admissible - {reason}")]
    ExpertOpinionRejected { reason: String },

    #[error("BSA Section 60: Hearsay evidence not admissible")]
    HearsayNotAdmissible,

    #[error("BSA Section 24: Confession obtained by inducement/threat/promise")]
    ConfessionInvoluntary,
}

pub type EvidenceActResult<T> = Result<T, EvidenceActError>;

#[derive(Debug, Clone, Default)]
pub struct EvidenceComplianceReport {
    pub compliant: bool,
    pub admissible: bool,
    pub errors: Vec<EvidenceActError>,
    pub warnings: Vec<String>,
}
