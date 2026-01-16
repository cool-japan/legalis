//! UK Family Law - Error Types
//!
//! Errors for UK family law validation with statute and case law references.

use std::fmt;

/// Result type for family law operations
pub type Result<T> = std::result::Result<T, FamilyLawError>;

/// Errors in UK family law validation
#[derive(Debug, Clone, PartialEq)]
pub enum FamilyLawError {
    // ========================================================================
    // Marriage/Civil Partnership Errors
    // ========================================================================
    /// Marriage void - parties within prohibited degrees (MA 1949 s.1)
    ProhibitedDegreesMarriage {
        /// Relationship between parties
        relationship: String,
    },

    /// Marriage void - party already married (MA 1949 s.1)
    Bigamy {
        /// Party who is already married
        party: String,
    },

    /// Marriage void - party under 18 (MA 1949 as amended)
    UnderageMarriage {
        /// Party's age
        age: u32,
    },

    /// Marriage void - formality defect (MA 1949 s.49)
    MarriageFormalityDefect {
        /// Nature of defect
        defect: String,
    },

    /// Civil partnership void - within prohibited degrees (CPA 2004 Sch 1)
    ProhibitedDegreesCivilPartnership {
        /// Relationship between parties
        relationship: String,
    },

    /// Civil partnership void - party already in relationship
    AlreadyInCivilPartnership {
        /// Party who is already in relationship
        party: String,
    },

    // ========================================================================
    // Divorce/Dissolution Errors
    // ========================================================================
    /// Cannot apply for divorce - marriage under 1 year (MCA 1973 s.3)
    MarriageUnderOneYear {
        /// Marriage date
        marriage_date: String,
        /// Application date
        application_date: String,
    },

    /// No jurisdiction for divorce (Domicile and Matrimonial Proceedings Act 1973)
    NoJurisdiction {
        /// Reason for lack of jurisdiction
        reason: String,
    },

    /// Statement of irretrievable breakdown not made (DDSA 2020 s.1)
    NoStatementOfBreakdown,

    /// Application period not observed (DDSA 2020 - minimum 20 weeks)
    ApplicationPeriodNotObserved {
        /// Weeks elapsed
        weeks_elapsed: u32,
        /// Required weeks
        required_weeks: u32,
    },

    /// Conditional order not obtained before final order
    ConditionalOrderNotObtained,

    /// Final order application too early (6 weeks minimum from conditional)
    FinalOrderTooEarly {
        /// Weeks since conditional order
        weeks_since_conditional: u32,
    },

    // ========================================================================
    // Children Errors
    // ========================================================================
    /// No standing to apply for child arrangements order
    NoStandingChildArrangements {
        /// Applicant
        applicant: String,
        /// Reason
        reason: String,
    },

    /// Welfare checklist not considered (CA 1989 s.1(3))
    WelfareChecklistNotConsidered {
        /// Missing factors
        missing_factors: Vec<String>,
    },

    /// No welfare report when required (CA 1989 s.7)
    WelfareReportRequired {
        /// Reason report needed
        reason: String,
    },

    /// Child not ascertained wishes in age-appropriate case (CA 1989 s.1(3)(a))
    ChildWishesNotAscertained {
        /// Child's age
        age: u32,
    },

    /// Leave required but not obtained (CA 1989 s.10)
    LeaveNotObtained {
        /// Type of application
        application_type: String,
    },

    /// Parental responsibility application by unsuitable person
    UnsuitablePRApplication {
        /// Reason
        reason: String,
    },

    /// Special guardianship - notice period not observed (CA 1989 s.14A(7))
    SpecialGuardianshipNoticeNotGiven {
        /// Days notice given
        days_given: u32,
        /// Required (3 months)
        required_days: u32,
    },

    // ========================================================================
    // Financial Remedy Errors
    // ========================================================================
    /// Form E not filed (FPR 2010 r.9.14)
    FormENotFiled {
        /// Party who has not filed
        party: String,
    },

    /// Non-disclosure of assets (MCA 1973 / Rose v Rose)
    NonDisclosure {
        /// Nature of non-disclosure
        description: String,
    },

    /// Section 25 factors not addressed (MCA 1973 s.25)
    Section25FactorsNotAddressed {
        /// Missing factors
        missing_factors: Vec<String>,
    },

    /// Clean break not considered (MCA 1973 s.25A)
    CleanBreakNotConsidered,

    /// Pension on divorce not properly valued
    PensionNotValued {
        /// Pension scheme
        scheme: String,
    },

    /// Needs not properly assessed
    NeedsNotAssessed {
        /// Party
        party: String,
    },

    // ========================================================================
    // Domestic Abuse/Protection Errors
    // ========================================================================
    /// Not an associated person (FLA 1996 s.62)
    NotAssociatedPerson {
        /// Alleged relationship
        relationship: String,
    },

    /// No evidence of molestation (FLA 1996 s.42)
    NoEvidenceOfMolestation,

    /// Balance of harm test not applied (FLA 1996 s.33(7))
    BalanceOfHarmNotApplied,

    /// Occupation order - applicant not entitled and no alternative grounds
    OccupationNoEntitlement {
        /// Applicant
        applicant: String,
    },

    /// Undertaking accepted when inappropriate (domestic abuse)
    UndertakingInappropriate {
        /// Reason
        reason: String,
    },

    /// Without notice application - not urgent or necessary
    WithoutNoticeNotJustified {
        /// Reason
        reason: String,
    },

    // ========================================================================
    // Procedure Errors
    // ========================================================================
    /// Wrong court for application
    WrongCourt {
        /// Application type
        application_type: String,
        /// Court used
        court_used: String,
        /// Correct court
        correct_court: String,
    },

    /// Time limit expired
    TimeLimitExpired {
        /// Limit type
        limit_type: String,
        /// Deadline
        deadline: String,
    },

    /// No MIAM attendance (FPR 2010 r.3)
    MIAMNotAttended {
        /// Exemption claimed
        exemption_claimed: Option<String>,
        /// Whether exemption valid
        exemption_valid: bool,
    },

    /// Service not effected
    ServiceNotEffected {
        /// Document
        document: String,
        /// Respondent
        respondent: String,
    },

    // ========================================================================
    // General Errors
    // ========================================================================
    /// Invalid date
    InvalidDate {
        /// Field name
        field: String,
        /// Provided value
        value: String,
    },

    /// Missing required information
    MissingInformation {
        /// Field name
        field: String,
    },

    /// Validation error
    ValidationError {
        /// Description
        description: String,
    },
}

impl fmt::Display for FamilyLawError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Marriage/Civil Partnership
            Self::ProhibitedDegreesMarriage { relationship } => {
                write!(
                    f,
                    "Marriage void under Marriage Act 1949 s.1 - parties within prohibited \
                     degrees of relationship: {}",
                    relationship
                )
            }
            Self::Bigamy { party } => {
                write!(
                    f,
                    "Marriage void under Marriage Act 1949 s.1 - {} is already married",
                    party
                )
            }
            Self::UnderageMarriage { age } => {
                write!(
                    f,
                    "Marriage void under Marriage Act 1949 (as amended by Marriage and Civil \
                     Partnership (Minimum Age) Act 2022) - party aged {} is under 18",
                    age
                )
            }
            Self::MarriageFormalityDefect { defect } => {
                write!(
                    f,
                    "Marriage void under Marriage Act 1949 s.49 - formality defect: {}",
                    defect
                )
            }
            Self::ProhibitedDegreesCivilPartnership { relationship } => {
                write!(
                    f,
                    "Civil partnership void under Civil Partnership Act 2004 Schedule 1 - \
                     parties within prohibited degrees: {}",
                    relationship
                )
            }
            Self::AlreadyInCivilPartnership { party } => {
                write!(
                    f,
                    "Civil partnership void - {} is already in a civil partnership or marriage",
                    party
                )
            }

            // Divorce/Dissolution
            Self::MarriageUnderOneYear {
                marriage_date,
                application_date,
            } => {
                write!(
                    f,
                    "Cannot apply for divorce - marriage under one year old (MCA 1973 s.3). \
                     Marriage: {}, Application: {}",
                    marriage_date, application_date
                )
            }
            Self::NoJurisdiction { reason } => {
                write!(
                    f,
                    "No jurisdiction for divorce under Domicile and Matrimonial Proceedings \
                     Act 1973: {}",
                    reason
                )
            }
            Self::NoStatementOfBreakdown => {
                write!(
                    f,
                    "Application for divorce must include statement that marriage has broken \
                     down irretrievably (Divorce, Dissolution and Separation Act 2020 s.1)"
                )
            }
            Self::ApplicationPeriodNotObserved {
                weeks_elapsed,
                required_weeks,
            } => {
                write!(
                    f,
                    "Minimum application period not observed (DDSA 2020). {} weeks elapsed, \
                     {} weeks required",
                    weeks_elapsed, required_weeks
                )
            }
            Self::ConditionalOrderNotObtained => {
                write!(
                    f,
                    "Cannot apply for final order - conditional order not yet obtained"
                )
            }
            Self::FinalOrderTooEarly {
                weeks_since_conditional,
            } => {
                write!(
                    f,
                    "Final order application too early - only {} weeks since conditional order \
                     (minimum 6 weeks required)",
                    weeks_since_conditional
                )
            }

            // Children
            Self::NoStandingChildArrangements { applicant, reason } => {
                write!(
                    f,
                    "{} has no standing to apply for child arrangements order (CA 1989 s.10): {}",
                    applicant, reason
                )
            }
            Self::WelfareChecklistNotConsidered { missing_factors } => {
                write!(
                    f,
                    "Welfare checklist (CA 1989 s.1(3)) not fully considered. Missing: {}",
                    missing_factors.join(", ")
                )
            }
            Self::WelfareReportRequired { reason } => {
                write!(f, "Welfare report required under CA 1989 s.7: {}", reason)
            }
            Self::ChildWishesNotAscertained { age } => {
                write!(
                    f,
                    "Child's ascertainable wishes and feelings not considered (CA 1989 s.1(3)(a)). \
                     Child aged {} - age appropriate to ascertain views",
                    age
                )
            }
            Self::LeaveNotObtained { application_type } => {
                write!(
                    f,
                    "Leave of court required but not obtained (CA 1989 s.10) for: {}",
                    application_type
                )
            }
            Self::UnsuitablePRApplication { reason } => {
                write!(
                    f,
                    "Parental responsibility application unsuitable: {}",
                    reason
                )
            }
            Self::SpecialGuardianshipNoticeNotGiven {
                days_given,
                required_days,
            } => {
                write!(
                    f,
                    "Special guardianship notice period not observed (CA 1989 s.14A(7)). \
                     {} days given, {} days required",
                    days_given, required_days
                )
            }

            // Financial Remedy
            Self::FormENotFiled { party } => {
                write!(
                    f,
                    "Form E (financial statement) not filed by {} (FPR 2010 r.9.14)",
                    party
                )
            }
            Self::NonDisclosure { description } => {
                write!(
                    f,
                    "Non-disclosure of assets (duty per Rose v Rose [2002] EWCA Civ 208): {}",
                    description
                )
            }
            Self::Section25FactorsNotAddressed { missing_factors } => {
                write!(
                    f,
                    "Section 25 MCA 1973 factors not addressed: {}",
                    missing_factors.join(", ")
                )
            }
            Self::CleanBreakNotConsidered => {
                write!(
                    f,
                    "Court's duty to consider clean break not discharged (MCA 1973 s.25A)"
                )
            }
            Self::PensionNotValued { scheme } => {
                write!(
                    f,
                    "Pension not properly valued for divorce purposes: {}",
                    scheme
                )
            }
            Self::NeedsNotAssessed { party } => {
                write!(
                    f,
                    "Needs of {} not properly assessed (MCA 1973 s.25(2)(b))",
                    party
                )
            }

            // Domestic Abuse/Protection
            Self::NotAssociatedPerson { relationship } => {
                write!(
                    f,
                    "Applicant not an 'associated person' under FLA 1996 s.62. \
                     Alleged relationship: {}",
                    relationship
                )
            }
            Self::NoEvidenceOfMolestation => {
                write!(
                    f,
                    "No evidence of molestation to support non-molestation order (FLA 1996 s.42)"
                )
            }
            Self::BalanceOfHarmNotApplied => {
                write!(f, "Balance of harm test not applied (FLA 1996 s.33(7))")
            }
            Self::OccupationNoEntitlement { applicant } => {
                write!(
                    f,
                    "{} has no entitlement to occupy and no alternative grounds for \
                     occupation order (FLA 1996 ss.33-38)",
                    applicant
                )
            }
            Self::UndertakingInappropriate { reason } => {
                write!(
                    f,
                    "Undertaking inappropriate and should not be accepted (FLA 1996 s.46(3A)): {}",
                    reason
                )
            }
            Self::WithoutNoticeNotJustified { reason } => {
                write!(
                    f,
                    "Without notice application not justified (FPR 2010 r.10.2): {}",
                    reason
                )
            }

            // Procedure
            Self::WrongCourt {
                application_type,
                court_used,
                correct_court,
            } => {
                write!(
                    f,
                    "Application for {} issued in wrong court ({}). Should be: {}",
                    application_type, court_used, correct_court
                )
            }
            Self::TimeLimitExpired {
                limit_type,
                deadline,
            } => {
                write!(
                    f,
                    "Time limit expired for {}: deadline was {}",
                    limit_type, deadline
                )
            }
            Self::MIAMNotAttended {
                exemption_claimed,
                exemption_valid,
            } => match exemption_claimed {
                Some(exemption) if *exemption_valid => {
                    write!(
                        f,
                        "MIAM not attended but valid exemption claimed: {}",
                        exemption
                    )
                }
                Some(exemption) => {
                    write!(
                        f,
                        "MIAM not attended (FPR 2010 r.3). Exemption '{}' claimed but not valid",
                        exemption
                    )
                }
                None => {
                    write!(
                        f,
                        "MIAM not attended and no exemption claimed (FPR 2010 r.3)"
                    )
                }
            },
            Self::ServiceNotEffected {
                document,
                respondent,
            } => {
                write!(f, "{} not served on {}", document, respondent)
            }

            // General
            Self::InvalidDate { field, value } => {
                write!(f, "Invalid date for {}: {}", field, value)
            }
            Self::MissingInformation { field } => {
                write!(f, "Missing required information: {}", field)
            }
            Self::ValidationError { description } => {
                write!(f, "Validation error: {}", description)
            }
        }
    }
}

impl std::error::Error for FamilyLawError {}
