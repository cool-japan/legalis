//! UK Trust Law (Equity)
//!
//! Implementation of English trust law principles developed through equity.
//!
//! ## Definition of Trust
//!
//! A trust is an equitable obligation binding a trustee to deal with property
//! for the benefit of beneficiaries or for a charitable purpose.
//!
//! ## Three Certainties (Knight v Knight \[1840\])
//!
//! A trust is only valid if there are three certainties:
//! 1. **Certainty of Intention** - Clear intent to create a trust
//! 2. **Certainty of Subject Matter** - Clear identification of trust property and beneficial interests
//! 3. **Certainty of Objects** - Clear identification of beneficiaries (or charitable purpose)
//!
//! ## Constitution of Trust
//!
//! A trust must be properly constituted:
//! - **Transfer to trustees** - "Equity will not perfect an imperfect gift"
//! - **Declaration of self as trustee** - Settlor declares themselves trustee
//! - **Disposition of equitable interest** - Assignment of beneficial interest (must comply with s.53(1)(c) LPA 1925)
//!
//! ## Trustee Duties
//!
//! ### Fiduciary Duties
//! - **Duty of care** (Speight v Gaunt \[1883\]) - Standard of ordinary prudent business person
//! - **Duty of undivided loyalty** - No conflict of interest (Keech v Sandford \[1726\])
//! - **Duty not to profit** - No unauthorized profit from trust (Boardman v Phipps \[1967\])
//! - **Duty of impartiality** - Fair treatment of beneficiaries
//!
//! ### Statutory Duties (Trustee Act 2000)
//! - **Duty of care** (s.1) - Statutory duty of care when exercising powers
//! - **Investment duties** (ss.3-8) - Standard investment criteria, diversification
//! - **Duty to review investments** - Regular review required
//! - **Duty to obtain advice** - Investment advice requirement
//!
//! ## Irreducible Core of Trustee Obligations (Armitage v Nurse \[1998\])
//!
//! Cannot exclude:
//! - Duty to perform trust honestly and in good faith
//! - Minimum duty of care (cannot exclude gross negligence/willful default)
//!
//! ## Key Cases
//!
//! - **Knight v Knight \[1840\]**: Three certainties
//! - **Keech v Sandford \[1726\]**: No conflict rule
//! - **Speight v Gaunt \[1883\]**: Trustee standard of care
//! - **Boardman v Phipps \[1967\]**: No unauthorized profit
//! - **Armitage v Nurse \[1998\]**: Irreducible core
//! - **Nestle v National Westminster Bank \[1993\]**: Investment duty
//! - **Pitt v Holt \[2013\]**: Mistake and setting aside
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use legalis_uk::trusts::*;
//!
//! // Check three certainties
//! let trust = TrustDeclaration {
//!     settlor: "John Smith".to_string(),
//!     property: "£100,000 in XYZ shares".to_string(),
//!     beneficiaries: vec!["My children".to_string()],
//!     intention_words: "I declare myself trustee".to_string(),
//! };
//!
//! let certainties = check_three_certainties(&trust)?;
//! assert!(certainties.intention && certainties.subject_matter && certainties.objects);
//! ```

pub mod breach;
pub mod charitable;
pub mod creation;
pub mod error;
pub mod trustees;
pub mod types;

// Re-exports
pub use breach::{
    BreachOfTrust, BreachRemedy, BreachSeverity, DishonestAssistance, KnowingReceipt,
    TracingMethod, assess_breach_of_trust, calculate_tracing_remedy, validate_dishonest_assistance,
    validate_knowing_receipt,
};
pub use charitable::{
    CharitablePurpose, CharitableTrust, CyPresScheme, PublicBenefitTest,
    validate_charitable_purpose, validate_cy_pres, validate_public_benefit,
};
pub use creation::{
    CertaintyOfIntention, CertaintyOfObjects, CertaintyOfSubjectMatter, ConstitutionMethod,
    ThreeCertainties, TrustConstitution, TrustDeclaration, check_certainty_intention,
    check_certainty_objects, check_certainty_subject_matter, check_three_certainties,
    validate_trust_constitution,
};
pub use error::{TrustError, TrustResult};
pub use trustees::{
    ConflictOfInterest, DutyOfCare, InvestmentDecision, TrusteeAppointment, TrusteeDuty,
    TrusteePower, assess_conflict_of_interest, check_duty_of_care, validate_investment_decision,
    validate_trustee_appointment,
};
pub use types::{Beneficiary, BeneficiaryType, Trust, TrustProperty, TrustType, Trustee};
