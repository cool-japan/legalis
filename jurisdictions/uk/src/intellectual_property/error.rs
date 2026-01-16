//! IP-specific error types

use thiserror::Error;

/// IP law error type
#[derive(Debug, Clone, Error)]
pub enum IpError {
    /// Patent not novel (anticipation by prior art)
    #[error("Patent lacks novelty. Anticipated by: {prior_art}. See Patents Act 1977 s.2(1)")]
    LacksNovelty {
        /// Prior art reference
        prior_art: String,
    },

    /// Patent lacks inventive step (obvious to skilled person)
    #[error(
        "Patent lacks inventive step. Obvious in view of: {prior_art}. See Patents Act 1977 s.3"
    )]
    LacksInventiveStep {
        /// Prior art reference
        prior_art: String,
    },

    /// Not capable of industrial application
    #[error(
        "Patent not capable of industrial application. Reason: {reason}. See Patents Act 1977 s.4"
    )]
    NotIndustriallyApplicable {
        /// Reason for non-applicability
        reason: String,
    },

    /// Excluded subject matter (e.g., mathematical method, business method)
    #[error(
        "Subject matter excluded from patentability: {subject_matter}. See Patents Act 1977 s.1(2)"
    )]
    ExcludedSubjectMatter {
        /// Type of excluded subject matter
        subject_matter: String,
    },

    /// Trademark not distinctive (Trade Marks Act 1994 s.3(1)(b))
    #[error("Trade mark lacks distinctive character. See Trade Marks Act 1994 s.3(1)(b)")]
    LacksDistinctiveness,

    /// Trademark is descriptive (s.3(1)(c))
    #[error("Trade mark is descriptive of: {description}. See Trade Marks Act 1994 s.3(1)(c)")]
    Descriptive {
        /// Description of what it describes
        description: String,
    },

    /// Trademark is customary (s.3(1)(d))
    #[error("Trade mark has become customary in trade. See Trade Marks Act 1994 s.3(1)(d)")]
    Customary,

    /// Trademark is deceptive (s.3(3)(a))
    #[error("Trade mark is contrary to public policy or morality. See Trade Marks Act 1994 s.3(3)")]
    Deceptive,

    /// Earlier conflicting trademark (s.5)
    #[error(
        "Earlier identical/similar trade mark exists: {earlier_mark}. See Trade Marks Act 1994 s.5"
    )]
    ConflictingTradeMark {
        /// Earlier mark reference
        earlier_mark: String,
    },

    /// Likelihood of confusion (s.5(2))
    #[error("Likelihood of confusion with earlier trade mark: {earlier_mark}")]
    LikelihoodOfConfusion {
        /// Earlier mark causing confusion
        earlier_mark: String,
    },

    /// Copyright work not original
    #[error(
        "Work lacks originality. Must be author's own intellectual creation. See CDPA 1988 s.1"
    )]
    LacksOriginality,

    /// Not a copyright work (excluded type)
    #[error("Not a copyright work. Type '{work_type}' not protected under CDPA 1988 s.1")]
    NotCopyrightWork {
        /// Type of work
        work_type: String,
    },

    /// Copyright expired
    #[error("Copyright has expired. Protection ended on: {expiry_date}")]
    CopyrightExpired {
        /// Expiry date
        expiry_date: String,
    },

    /// Fair dealing defense applies (no infringement)
    #[error("Fair dealing applies for purpose: {purpose}. See CDPA 1988 ss.29-30")]
    FairDealing {
        /// Fair dealing purpose
        purpose: String,
    },

    /// Design lacks novelty
    #[error("Design lacks novelty. Similar design disclosed: {prior_design}")]
    DesignLacksNovelty {
        /// Prior design reference
        prior_design: String,
    },

    /// Design lacks individual character
    #[error("Design lacks individual character (not sufficiently different). See RDA 1949 s.1B")]
    LacksIndividualCharacter,

    /// Must-fit exclusion (design dictated by technical function)
    #[error("Design excluded: must-fit features dictated by technical function. See RDA 1949 s.1C")]
    MustFit,

    /// Invalid claim format
    #[error("Invalid claim format: {reason}")]
    InvalidClaim {
        /// Reason for invalidity
        reason: String,
    },

    /// Missing required information
    #[error("Missing required information: {field}")]
    MissingInformation {
        /// Missing field name
        field: String,
    },

    /// Filing deadline missed
    #[error("Filing deadline missed. Deadline was: {deadline}, filed: {filed}")]
    MissedDeadline {
        /// Deadline date
        deadline: String,
        /// Filing date
        filed: String,
    },

    /// Insufficient evidence of use
    #[error("Insufficient evidence of genuine use (Trade Marks Act 1994 s.46)")]
    InsufficientUse,
}

/// Result type for IP operations
pub type IpResult<T> = Result<T, IpError>;
