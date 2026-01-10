//! Company Law Errors (Companies Act 2006)

#![allow(missing_docs)]

use thiserror::Error;

/// Errors related to UK company law compliance
#[derive(Debug, Clone, Error, PartialEq)]
pub enum CompanyLawError {
    // ============================================================================
    // Company Formation Errors (CA 2006 Part 2)
    // ============================================================================
    /// Company name does not comply with CA 2006 ss.53-81
    #[error(
        "Company name '{name}' is invalid: {reason}. See Companies Act 2006 sections 53-81 (Company Names)."
    )]
    InvalidCompanyName { name: String, reason: String },

    /// Company name missing required suffix
    #[error(
        "Company name '{name}' must end with '{required_suffix}' or abbreviated form. See CA 2006 s.59."
    )]
    MissingSuffix {
        name: String,
        required_suffix: String,
    },

    /// Company name contains sensitive words requiring approval
    #[error(
        "Company name '{name}' contains sensitive word '{word}' requiring Secretary of State approval. See CA 2006 s.55 and Company and Business Names (Miscellaneous Provisions) Regulations 2009."
    )]
    SensitiveWord { name: String, word: String },

    /// Company name too similar to existing company
    #[error(
        "Company name '{name}' is too similar to existing company '{existing}'. See CA 2006 s.66 (Name not to be the same as another)."
    )]
    TooSimilarToExisting { name: String, existing: String },

    /// Registered office not in correct jurisdiction
    #[error(
        "Registered office must be in {expected_country}, but address is in {actual_country}. See CA 2006 s.86."
    )]
    InvalidRegisteredOffice {
        expected_country: String,
        actual_country: String,
    },

    /// Insufficient share capital
    #[error(
        "Share capital of £{actual} is below minimum requirement of £{minimum} for {company_type}. See CA 2006 s.763 (Minimum share capital for public companies)."
    )]
    InsufficientShareCapital {
        actual: f64,
        minimum: f64,
        company_type: String,
    },

    /// Insufficient paid up capital for PLC
    #[error(
        "Public company must have at least 25% of nominal capital paid up. Current: {percentage:.1}% (£{paid_up} of £{nominal}). See CA 2006 s.586."
    )]
    InsufficientPaidUpCapital {
        paid_up: f64,
        nominal: f64,
        percentage: f64,
    },

    /// Insufficient number of directors
    #[error(
        "Company requires at least {minimum} director(s), but only {actual} provided. See CA 2006 s.154 (Companies required to have directors)."
    )]
    InsufficientDirectors { minimum: u32, actual: u32 },

    /// PLC missing company secretary
    #[error(
        "Public company must have a company secretary. See CA 2006 s.271 (Public companies: requirement to have secretary)."
    )]
    MissingCompanySecretary,

    /// Statement of compliance not provided
    #[error("Statement of compliance required under CA 2006 s.13 has not been provided.")]
    MissingStatementOfCompliance,

    // ============================================================================
    // Director Duties Errors (CA 2006 ss.171-177)
    // ============================================================================
    /// Breach of s.171: Act within powers
    #[error(
        "Director failed to act within powers: {details}. See CA 2006 s.171 (Duty to act within powers). Directors must act in accordance with the company's constitution and only exercise powers for the purposes for which they are conferred."
    )]
    BreachActWithinPowers { details: String },

    /// Breach of s.172: Promote success of company
    #[error(
        "Director failed to promote success of company: {details}. See CA 2006 s.172 (Duty to promote the success of the company). Directors must consider: (a) long term consequences, (b) employee interests, (c) business relationships, (d) community and environment, (e) reputation, (f) fairness between members."
    )]
    BreachPromoteSuccess { details: String },

    /// Breach of s.173: Independent judgment
    #[error(
        "Director failed to exercise independent judgment: {details}. See CA 2006 s.173 (Duty to exercise independent judgment)."
    )]
    BreachIndependentJudgment { details: String },

    /// Breach of s.174: Reasonable care, skill and diligence
    #[error(
        "Director failed to exercise reasonable care, skill and diligence: {details}. See CA 2006 s.174 (Duty to exercise reasonable care, skill and diligence). Standard: (a) care reasonably expected from person in that position (objective), (b) care expected given director's actual knowledge/skill/experience (subjective)."
    )]
    BreachReasonableCare { details: String },

    /// Breach of s.175: Conflicts of interest
    #[error(
        "Director has undeclared conflict of interest: {details}. See CA 2006 s.175 (Duty to avoid conflicts of interest). Director must avoid situations where direct or indirect interest conflicts with company interests."
    )]
    BreachAvoidConflicts { details: String },

    /// Breach of s.176: Benefits from third parties
    #[error(
        "Director accepted benefits from third parties: {details}. See CA 2006 s.176 (Duty not to accept benefits from third parties). Director must not accept benefits from third party by reason of being a director or doing/not doing anything as director."
    )]
    BreachThirdPartyBenefits { details: String },

    /// Breach of s.177: Declare interest
    #[error(
        "Director failed to declare interest in proposed transaction: {details}. See CA 2006 s.177 (Duty to declare interest in proposed transaction or arrangement). If director is interested in proposed transaction, must declare nature and extent of interest to other directors."
    )]
    BreachDeclareInterest { details: String },

    // ============================================================================
    // Share Capital Errors
    // ============================================================================
    /// Share allotment exceeds authorized capital
    #[error(
        "Share allotment of {allotment_shares} shares would exceed authorized capital of {authorized_shares} shares. See CA 2006 s.549 (Exercise of directors' power to allot shares)."
    )]
    ExceedsAuthorizedCapital {
        allotment_shares: u64,
        authorized_shares: u64,
    },

    /// Invalid share class rights
    #[error(
        "Share class '{class_name}' has invalid rights: {reason}. See CA 2006 s.629 (Classes of shares)."
    )]
    InvalidShareClassRights { class_name: String, reason: String },

    /// Payment for shares not received
    #[error(
        "Payment for {number_of_shares} shares (£{amount}) not received. See CA 2006 s.582 (Liability of subsequent holders of shares)."
    )]
    SharesNotPaidFor { number_of_shares: u64, amount: f64 },

    // ============================================================================
    // Annual Accounts Errors (CA 2006 Part 15)
    // ============================================================================
    /// Annual accounts not filed by deadline
    #[error(
        "Annual accounts for year ending {year_end} not filed by deadline {deadline}. Late filing penalties apply. See CA 2006 s.441 (Duty to file accounts)."
    )]
    AccountsNotFiled { year_end: String, deadline: String },

    /// Accounts require audit but none obtained
    #[error(
        "Annual accounts require audit (turnover £{turnover} > £10.2m or balance sheet £{balance_sheet} > £5.1m) but no audit obtained. See CA 2006 s.475 (Requirement for audited accounts)."
    )]
    MissingRequiredAudit { turnover: f64, balance_sheet: f64 },

    // ============================================================================
    // Corporate Governance Errors
    // ============================================================================
    /// AGM not held within required timeframe (PLC only)
    #[error(
        "Public company must hold AGM within 6 months of financial year end. Last AGM: {last_agm_date}. See CA 2006 s.336 (Public companies: annual general meeting)."
    )]
    AgmNotHeld { last_agm_date: String },

    /// Resolution does not meet required majority
    #[error(
        "Resolution failed: {votes_for} votes for ({percentage_for:.1}%) does not meet {resolution_type} resolution requirement of >{required_percentage:.0}%. See CA 2006 s.282 (Ordinary resolutions) and s.283 (Special resolutions)."
    )]
    ResolutionFailed {
        resolution_type: String,
        votes_for: u64,
        votes_against: u64,
        percentage_for: f64,
        required_percentage: f64,
    },

    /// Insufficient notice for general meeting
    #[error(
        "General meeting requires {required_days} days' notice, but only {actual_days} days given. See CA 2006 s.307 (Notice required of general meeting)."
    )]
    InsufficientNotice {
        required_days: u32,
        actual_days: u32,
    },

    // ============================================================================
    // General Errors
    // ============================================================================
    /// Generic validation error
    #[error("Company law validation error: {message}")]
    ValidationError { message: String },

    /// Multiple validation errors
    #[error("Multiple company law errors: {errors:?}")]
    MultipleErrors { errors: Vec<String> },
}

/// Result type for company law operations
pub type Result<T> = std::result::Result<T, CompanyLawError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages_contain_statute_references() {
        let error = CompanyLawError::InvalidCompanyName {
            name: "Test Company".to_string(),
            reason: "Missing suffix".to_string(),
        };
        assert!(error.to_string().contains("Companies Act 2006"));
        assert!(error.to_string().contains("53-81"));

        let error = CompanyLawError::BreachPromoteSuccess {
            details: "Failed to consider employee interests".to_string(),
        };
        assert!(error.to_string().contains("s.172"));
        assert!(error.to_string().contains("employee interests"));
    }

    #[test]
    fn test_director_duty_errors() {
        let duties = [
            CompanyLawError::BreachActWithinPowers {
                details: "test".to_string(),
            },
            CompanyLawError::BreachPromoteSuccess {
                details: "test".to_string(),
            },
            CompanyLawError::BreachIndependentJudgment {
                details: "test".to_string(),
            },
            CompanyLawError::BreachReasonableCare {
                details: "test".to_string(),
            },
            CompanyLawError::BreachAvoidConflicts {
                details: "test".to_string(),
            },
            CompanyLawError::BreachThirdPartyBenefits {
                details: "test".to_string(),
            },
            CompanyLawError::BreachDeclareInterest {
                details: "test".to_string(),
            },
        ];

        // All seven director duties should have distinct error types
        assert_eq!(duties.len(), 7);

        // Each should reference the correct section
        for (i, duty) in duties.iter().enumerate() {
            let section = format!("s.{}", 171 + i);
            assert!(duty.to_string().contains(&section));
        }
    }
}
