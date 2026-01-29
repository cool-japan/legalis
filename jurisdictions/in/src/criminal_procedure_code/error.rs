//! Criminal Procedure Code Error Types

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum CrpcError {
    #[error("BNSS Section 35: Arrest without warrant not justified - {reason}")]
    IllegalArrest { reason: String },

    #[error("BNSS Section 167: Police remand exceeds 15 days maximum")]
    ExcessiveRemand,

    #[error("BNSS Section 187: Accused not produced before magistrate within 24 hours")]
    Not24HoursProduction,

    #[error("BNSS Section 193: Chargesheet not filed within {days} days - default bail applicable")]
    ChargesheetDelayed { days: u32 },

    #[error("BNSS Section 438: Anticipatory bail not applicable - {reason}")]
    AnticipatoryBailNotApplicable { reason: String },

    #[error("BNSS: Bail denied for non-bailable offence - {reason}")]
    BailDenied { reason: String },

    #[error("BNSS: Trial delayed beyond reasonable time - {days} days elapsed")]
    TrialDelayed { days: i64 },

    #[error("BNSS Section 320: Offence not compoundable")]
    NotCompoundable,

    #[error("BNSS: Appeal limitation expired - filed after {days} days")]
    AppealBarred { days: i64 },
}

pub type CrpcResult<T> = Result<T, CrpcError>;

#[derive(Debug, Clone, Default)]
pub struct CrpcComplianceReport {
    pub compliant: bool,
    pub status: String,
    pub errors: Vec<CrpcError>,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
}
