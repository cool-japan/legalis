//! IP-specific error types for Australian law

use thiserror::Error;

/// IP law error type
#[derive(Debug, Clone, Error)]
pub enum IpError {
    // =========================================================================
    // Patent Errors (Patents Act 1990)
    // =========================================================================
    /// Not a manner of manufacture (s.18(1)(a))
    #[error(
        "Invention is not a manner of manufacture. Reason: {reason}. \
         See Patents Act 1990 s.18(1)(a), NRDC v Commissioner of Patents (1959)"
    )]
    NotMannerOfManufacture {
        /// Reason not a manner of manufacture
        reason: String,
    },

    /// Patent not novel (s.18(1)(b)(i))
    #[error(
        "Patent lacks novelty. Anticipated by: {prior_art}. \
         See Patents Act 1990 s.18(1)(b)(i), s.7(1)"
    )]
    LacksNovelty {
        /// Prior art reference
        prior_art: String,
    },

    /// Patent lacks inventive step (s.18(1)(b)(ii))
    #[error(
        "Patent lacks inventive step. Obvious in view of: {prior_art}. \
         See Patents Act 1990 s.18(1)(b)(ii), s.7(2)"
    )]
    LacksInventiveStep {
        /// Prior art reference
        prior_art: String,
    },

    /// Not useful (s.18(1)(c))
    #[error("Invention is not useful. Reason: {reason}. See Patents Act 1990 s.18(1)(c)")]
    NotUseful {
        /// Reason not useful
        reason: String,
    },

    /// Excluded subject matter (human beings, biological processes - s.18(2))
    #[error(
        "Subject matter excluded from patentability: {subject_matter}. \
         See Patents Act 1990 s.18(2), D'Arcy v Myriad Genetics (2015)"
    )]
    ExcludedSubjectMatter {
        /// Type of excluded subject matter
        subject_matter: String,
    },

    /// Insufficient disclosure (s.40)
    #[error(
        "Specification does not sufficiently describe the invention. \
         Deficiency: {deficiency}. See Patents Act 1990 s.40(2)(a)"
    )]
    InsufficientDisclosure {
        /// Disclosure deficiency
        deficiency: String,
    },

    /// Claims not supported (s.40(3))
    #[error(
        "Claims not supported by matter disclosed. Claim: {claim_number}. \
         See Patents Act 1990 s.40(3)"
    )]
    ClaimsNotSupported {
        /// Unsupported claim number
        claim_number: u32,
    },

    /// Invalid claim format
    #[error("Invalid claim format: {reason}")]
    InvalidClaim {
        /// Reason for invalidity
        reason: String,
    },

    // =========================================================================
    // Trade Mark Errors (Trade Marks Act 1995)
    // =========================================================================
    /// Not capable of distinguishing (s.41)
    #[error(
        "Trade mark not capable of distinguishing applicant's goods/services. \
         See Trade Marks Act 1995 s.41"
    )]
    NotCapableOfDistinguishing,

    /// Trade mark is descriptive (s.41(2))
    #[error(
        "Trade mark is descriptive of: {description}. \
         See Trade Marks Act 1995 s.41(2)"
    )]
    Descriptive {
        /// Description of what it describes
        description: String,
    },

    /// Trade mark is customary in trade (s.41(2))
    #[error(
        "Trade mark has become customary in current language or trade practices. \
         See Trade Marks Act 1995 s.41(2)"
    )]
    Customary,

    /// Trade mark contrary to law (s.42(a))
    #[error("Trade mark use would be contrary to law. See Trade Marks Act 1995 s.42(a)")]
    ContraryToLaw,

    /// Trade mark scandalous or offensive (s.42(a))
    #[error(
        "Trade mark is scandalous or contrary to accepted principles of morality. \
         See Trade Marks Act 1995 s.42(a)"
    )]
    Scandalous,

    /// Trade mark likely to deceive or cause confusion (s.43)
    #[error(
        "Trade mark likely to deceive or cause confusion. Reason: {reason}. \
         See Trade Marks Act 1995 s.43"
    )]
    DeceptiveOrConfusing {
        /// Reason for deception/confusion
        reason: String,
    },

    /// Earlier conflicting trade mark (s.44)
    #[error(
        "Earlier identical/similar trade mark exists: {earlier_mark}. \
         See Trade Marks Act 1995 s.44"
    )]
    ConflictingTradeMark {
        /// Earlier mark reference
        earlier_mark: String,
    },

    /// Likelihood of confusion with earlier mark (s.120)
    #[error(
        "Likelihood of confusion with registered trade mark: {earlier_mark}. \
         See Trade Marks Act 1995 s.120"
    )]
    LikelihoodOfConfusion {
        /// Earlier mark causing confusion
        earlier_mark: String,
    },

    /// Non-use (s.92)
    #[error(
        "Trade mark has not been used in good faith for 3+ years. \
         See Trade Marks Act 1995 s.92"
    )]
    NonUse,

    // =========================================================================
    // Copyright Errors (Copyright Act 1968)
    // =========================================================================
    /// Work lacks originality
    #[error(
        "Work lacks originality. Must involve independent intellectual effort. \
         See Copyright Act 1968 s.32, IceTV v Nine Network (2009)"
    )]
    LacksOriginality,

    /// Not a copyright work
    #[error("Not a copyright work. Type '{work_type}' not protected under Copyright Act 1968")]
    NotCopyrightWork {
        /// Type of work
        work_type: String,
    },

    /// Copyright expired
    #[error(
        "Copyright has expired. Protection ended on: {expiry_date}. \
         See Copyright Act 1968 s.33"
    )]
    CopyrightExpired {
        /// Expiry date
        expiry_date: String,
    },

    /// Fair dealing applies
    #[error(
        "Fair dealing exception applies for purpose: {purpose}. \
         See Copyright Act 1968 ss.40-43"
    )]
    FairDealing {
        /// Fair dealing purpose
        purpose: String,
    },

    /// No subsistence - authorship requirements not met
    #[error(
        "Copyright does not subsist. Author must be qualified person. \
         See Copyright Act 1968 s.32"
    )]
    AuthorshipNotQualified,

    // =========================================================================
    // Design Errors (Designs Act 2003)
    // =========================================================================
    /// Design not new (s.16(1))
    #[error(
        "Design is not new. Identical design exists: {prior_design}. \
         See Designs Act 2003 s.16(1)"
    )]
    DesignNotNew {
        /// Prior design reference
        prior_design: String,
    },

    /// Design not distinctive (s.16(2))
    #[error(
        "Design not distinctive compared to prior art base. \
         See Designs Act 2003 s.16(2)"
    )]
    DesignNotDistinctive,

    /// Excluded design (s.17)
    #[error(
        "Design excluded from registration. Reason: {reason}. \
         See Designs Act 2003 s.17"
    )]
    DesignExcluded {
        /// Reason for exclusion
        reason: String,
    },

    // =========================================================================
    // General Errors
    // =========================================================================
    /// Missing required information
    #[error("Missing required information: {field}")]
    MissingInformation {
        /// Missing field name
        field: String,
    },

    /// Filing deadline missed
    #[error(
        "Filing deadline missed. Deadline was: {deadline}, filed: {filed}. \
         Priority may be lost"
    )]
    MissedDeadline {
        /// Deadline date
        deadline: String,
        /// Filing date
        filed: String,
    },

    /// Renewal fee not paid
    #[error("Renewal fee not paid. Due date: {due_date}. Right will lapse")]
    RenewalNotPaid {
        /// Due date for payment
        due_date: String,
    },

    /// Validation error
    #[error("Validation error: {message}")]
    ValidationError {
        /// Error message
        message: String,
    },
}

/// Result type for IP operations
pub type Result<T> = std::result::Result<T, IpError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patent_errors() {
        let error = IpError::LacksNovelty {
            prior_art: "AU2020123456".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Patents Act 1990"));
        assert!(msg.contains("novelty"));
    }

    #[test]
    fn test_trademark_errors() {
        let error = IpError::NotCapableOfDistinguishing;
        let msg = format!("{}", error);
        assert!(msg.contains("Trade Marks Act 1995"));
        assert!(msg.contains("s.41"));
    }

    #[test]
    fn test_copyright_errors() {
        let error = IpError::LacksOriginality;
        let msg = format!("{}", error);
        assert!(msg.contains("Copyright Act 1968"));
        assert!(msg.contains("IceTV"));
    }

    #[test]
    fn test_design_errors() {
        let error = IpError::DesignNotDistinctive;
        let msg = format!("{}", error);
        assert!(msg.contains("Designs Act 2003"));
    }
}
