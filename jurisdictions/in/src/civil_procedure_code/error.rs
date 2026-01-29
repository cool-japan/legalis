//! Code of Civil Procedure 1908 Error Types
//!
//! Error types for civil procedure compliance

use thiserror::Error;

/// CPC errors
#[derive(Debug, Clone, PartialEq, Error)]
pub enum CpcError {
    // Jurisdiction errors
    /// No territorial jurisdiction (Section 15-20)
    #[error("CPC Section 20: Court lacks territorial jurisdiction - {reason}")]
    NoTerritorialJurisdiction { reason: String },

    /// No pecuniary jurisdiction
    #[error(
        "CPC: Court lacks pecuniary jurisdiction - suit value {suit_value} exceeds limit {limit}"
    )]
    NoPecuniaryJurisdiction { suit_value: f64, limit: f64 },

    // Plaint errors
    /// Plaint rejection under Order 7 Rule 11
    #[error("CPC Order 7 Rule 11: Plaint rejected - {ground}")]
    PlaintRejected { ground: String },

    /// Insufficient court fees (Court Fees Act 1870)
    #[error("Court Fees Act: Insufficient court fees - paid {paid}, required {required}")]
    InsufficientCourtFees { paid: f64, required: f64 },

    /// Suit barred by limitation (Limitation Act 1963)
    #[error(
        "Limitation Act: Suit barred by limitation - filed on {filing_date}, limitation expired on {limitation_date}"
    )]
    BarredByLimitation {
        filing_date: String,
        limitation_date: String,
    },

    // Pleading errors
    /// Defective pleading (Order 6)
    #[error("CPC Order 6: Defective pleading - {reason}")]
    DefectivePleading { reason: String },

    /// Written statement not filed within time (Order 8 Rule 1)
    #[error("CPC Order 8 Rule 1: Written statement not filed within 120 days")]
    WrittenStatementDelayed,

    /// Amendment not permitted (Order 6 Rule 17)
    #[error("CPC Order 6 Rule 17: Amendment not permitted - {reason}")]
    AmendmentNotPermitted { reason: String },

    // Interim order errors
    /// Temporary injunction not granted (Order 39 Rule 1-2)
    #[error("CPC Order 39: Temporary injunction refused - {reason}")]
    InjunctionRefused { reason: String },

    /// Attachment before judgment not allowed (Order 38)
    #[error("CPC Order 38: Attachment before judgment not allowed - {reason}")]
    AttachmentNotAllowed { reason: String },

    // Appeal errors
    /// Appeal not maintainable (Section 104)
    #[error("CPC Section 104: Appeal not maintainable - order not appealable")]
    AppealNotMaintainable,

    /// Appeal barred by limitation (Limitation Act)
    #[error("Limitation Act: Appeal barred - limitation period {period} days expired")]
    AppealBarredByLimitation { period: u32 },

    /// Security not deposited (Order 41 Rule 1)
    #[error("CPC Order 41 Rule 1: Security not deposited for stay of execution")]
    SecurityNotDeposited,

    /// Second appeal on question of fact (Section 100)
    #[error("CPC Section 100: Second appeal not maintainable - no substantial question of law")]
    SecondAppealOnFact,

    // Execution errors
    /// Execution barred by limitation (12 years)
    #[error("CPC Section 48: Execution barred by limitation - decree more than 12 years old")]
    ExecutionBarred,

    /// Property not attachable (Order 21 Rule 54, CPC Schedule)
    #[error("CPC Order 21 Rule 54: Property not attachable - {reason}")]
    PropertyNotAttachable { reason: String },

    /// Arrest not permissible
    #[error("CPC Order 21 Rule 37: Arrest in execution not permissible - {reason}")]
    ArrestNotPermitted { reason: String },

    // Procedural errors
    /// Non-compliance with Order
    #[error("CPC: Non-compliance with court order - {details}")]
    NonCompliance { details: String },

    /// Dismissal for default (Order 9 Rule 8)
    #[error("CPC Order 9 Rule 8: Suit dismissed for default of plaintiff")]
    DismissedForDefault,

    /// Ex-parte decree (Order 9 Rule 6)
    #[error("CPC Order 9 Rule 6: Ex-parte decree against defendant")]
    ExParteDecree,
}

/// Result type for CPC operations
pub type CpcResult<T> = Result<T, CpcError>;

/// CPC compliance report
#[derive(Debug, Clone, PartialEq)]
pub struct CpcComplianceReport {
    /// Is suit/appeal compliant
    pub compliant: bool,
    /// Status description
    pub status: String,
    /// List of errors
    pub errors: Vec<CpcError>,
    /// Warnings
    pub warnings: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

impl Default for CpcComplianceReport {
    fn default() -> Self {
        Self {
            compliant: true,
            status: "Compliant".to_string(),
            errors: Vec::new(),
            warnings: Vec::new(),
            recommendations: Vec::new(),
        }
    }
}
