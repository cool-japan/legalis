//! Trust law errors

use thiserror::Error;

/// Trust law error type
#[derive(Debug, Clone, Error)]
pub enum TrustError {
    /// Lacks certainty of intention (Knight v Knight)
    #[error("Trust fails for lack of certainty of intention. See Knight v Knight [1840]")]
    LacksCertaintyIntention,

    /// Lacks certainty of subject matter (Palmer v Simmonds)
    #[error(
        "Trust fails for lack of certainty of subject matter. Words used: '{words}'. See Palmer v Simmonds [1854]"
    )]
    LacksCertaintySubjectMatter {
        /// Uncertain words used
        words: String,
    },

    /// Lacks certainty of objects (Re Gulbenkian)
    #[error("Trust fails for lack of certainty of objects. See Re Gulbenkian [1970]")]
    LacksCertaintyObjects,

    /// Trust not properly constituted (Milroy v Lord)
    #[error(
        "Trust not properly constituted. See Milroy v Lord [1862]: 'Equity will not perfect an imperfect gift'"
    )]
    NotProperlyConstituted,

    /// Formality requirement not met (s.53(1)(b) LPA 1925)
    #[error("Trust of land must be evidenced in writing. See s.53(1)(b) Law of Property Act 1925")]
    LandTrustNotInWriting,

    /// Disposition not in writing (s.53(1)(c) LPA 1925)
    #[error("Disposition of equitable interest must be in writing. See s.53(1)(c) LPA 1925")]
    DispositionNotInWriting,

    /// Breach of trust
    #[error("Breach of trust: {description}")]
    BreachOfTrust {
        /// Description of breach
        description: String,
    },

    /// Conflict of interest (Keech v Sandford)
    #[error("Trustee has conflict of interest. See Keech v Sandford [1726]: 'Inflexible rule'")]
    ConflictOfInterest,

    /// Unauthorized profit (Boardman v Phipps)
    #[error("Trustee made unauthorized profit. See Boardman v Phipps [1967]")]
    UnauthorizedProfit,

    /// Failed duty of care (Nestle v NatWest)
    #[error("Trustee failed duty of care. Standard: ordinary prudent business person")]
    FailedDutyOfCare,

    /// Not a charitable purpose (Charities Act 2011 s.3)
    #[error("Not a recognized charitable purpose under Charities Act 2011 s.3")]
    NotCharitablePurpose,

    /// Fails public benefit test (Charities Act 2011 s.4)
    #[error("Charitable trust fails public benefit test")]
    FailsPublicBenefit,

    /// Perpetuity period exceeded (Rule against perpetuities)
    #[error("Trust violates rule against perpetuities (must vest within perpetuity period)")]
    ViolatesPerpetuities,

    /// Invalid trustee appointment
    #[error("Invalid trustee appointment: {reason}")]
    InvalidAppointment {
        /// Reason for invalidity
        reason: String,
    },

    /// Invalid investment
    #[error("Investment not authorized or suitable. See Trustee Act 2000 ss.3-5")]
    InvalidInvestment,
}

/// Result type for trust operations
pub type TrustResult<T> = Result<T, TrustError>;
