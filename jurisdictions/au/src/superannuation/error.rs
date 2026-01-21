//! Superannuation-specific error types

use thiserror::Error;

/// Superannuation law error type
#[derive(Debug, Clone, Error)]
pub enum SuperannuationError {
    // =========================================================================
    // Superannuation Guarantee Errors (SG Act 1992)
    // =========================================================================
    /// SG contribution not made
    #[error(
        "Superannuation guarantee contribution not made in compliance with \
         Superannuation Guarantee (Administration) Act 1992 s.16. \
         Shortfall: ${shortfall:.2}. Quarter: {quarter}"
    )]
    SgShortfall {
        /// Contribution shortfall amount
        shortfall: f64,
        /// Quarter affected
        quarter: String,
    },

    /// SG charge applies
    #[error(
        "Superannuation guarantee charge applies under SG Act 1992 s.31. \
         Nominal interest: ${interest:.2}. Administration fee: ${admin_fee:.2}"
    )]
    SgChargeApplies {
        /// Nominal interest component
        interest: f64,
        /// Administration fee component
        admin_fee: f64,
    },

    /// Late contribution
    #[error(
        "Superannuation contribution made after due date. \
         Due: {due_date}. Paid: {paid_date}. SG charge may apply"
    )]
    LateContribution {
        /// Due date
        due_date: String,
        /// Payment date
        paid_date: String,
    },

    /// Employee not eligible
    #[error(
        "Employee not eligible for SG under SG Act 1992 s.27. \
         Reason: {reason}"
    )]
    EmployeeNotEligible {
        /// Reason for ineligibility
        reason: String,
    },

    // =========================================================================
    // SIS Act Errors (SIS Act 1993)
    // =========================================================================
    /// Fund not complying
    #[error(
        "Superannuation fund not a complying fund under SIS Act 1993 Part 5. \
         Reason: {reason}. Tax consequences apply"
    )]
    FundNotComplying {
        /// Reason for non-compliance
        reason: String,
    },

    /// Sole purpose test breach
    #[error(
        "Breach of sole purpose test under SIS Act 1993 s.62. \
         Fund assets must be maintained solely for retirement benefits. \
         Violation: {violation}"
    )]
    SolePurposeTestBreach {
        /// Description of violation
        violation: String,
    },

    /// In-house asset breach
    #[error(
        "In-house asset limit exceeded under SIS Act 1993 s.82. \
         Limit: 5% of total assets. Current: {current_percentage:.2}%"
    )]
    InHouseAssetBreach {
        /// Current percentage of in-house assets
        current_percentage: f64,
    },

    /// Illegal early release
    #[error(
        "Illegal early release of superannuation under SIS Act 1993 s.17A. \
         Amount: ${amount:.2}. Member must meet condition of release"
    )]
    IllegalEarlyRelease {
        /// Amount illegally released
        amount: f64,
    },

    /// Preservation requirements not met
    #[error(
        "Preservation requirements not met under SIS Act 1993 Part 6. \
         Benefit type: {benefit_type}. Condition of release required: {required_condition}"
    )]
    PreservationRequirementsNotMet {
        /// Type of benefit
        benefit_type: String,
        /// Required condition of release
        required_condition: String,
    },

    // =========================================================================
    // SMSF Errors
    // =========================================================================
    /// SMSF trustee breach
    #[error(
        "SMSF trustee has breached trustee duties under SIS Act 1993 s.52B. \
         Breach: {breach}. Civil and criminal penalties may apply"
    )]
    SmsfTrusteeBreach {
        /// Description of breach
        breach: String,
    },

    /// SMSF investment strategy breach
    #[error(
        "SMSF investment strategy does not comply with SIS Regulations reg 4.09. \
         Missing consideration: {missing}"
    )]
    InvestmentStrategyBreach {
        /// Missing strategy element
        missing: String,
    },

    /// SMSF audit requirement
    #[error(
        "SMSF annual audit not completed as required by SIS Act 1993 s.35C. \
         Financial year: {financial_year}"
    )]
    AuditNotCompleted {
        /// Financial year
        financial_year: String,
    },

    /// SMSF member limit exceeded
    #[error("SMSF member limit exceeded. Maximum: 6 members. Current: {current}")]
    SmsfMemberLimitExceeded {
        /// Current number of members
        current: u32,
    },

    /// Disqualified trustee
    #[error(
        "Person disqualified from being SMSF trustee under SIS Act 1993 s.120A. \
         Reason: {reason}"
    )]
    DisqualifiedTrustee {
        /// Reason for disqualification
        reason: String,
    },

    // =========================================================================
    // Contribution Errors
    // =========================================================================
    /// Contribution cap exceeded
    #[error(
        "Contribution cap exceeded for {cap_type}. \
         Cap: ${cap:.2}. Contributed: ${contributed:.2}. \
         Excess may be taxed at top marginal rate"
    )]
    ContributionCapExceeded {
        /// Type of cap (concessional/non-concessional)
        cap_type: String,
        /// Cap amount
        cap: f64,
        /// Amount contributed
        contributed: f64,
    },

    /// Invalid contribution
    #[error(
        "Invalid superannuation contribution under SIS Act 1993. \
         Reason: {reason}"
    )]
    InvalidContribution {
        /// Reason contribution is invalid
        reason: String,
    },

    /// Work test not met
    #[error(
        "Work test not satisfied for personal contributions. \
         Member aged 67-74 must work 40+ hours in 30 consecutive days"
    )]
    WorkTestNotMet,

    // =========================================================================
    // Benefit Errors
    // =========================================================================
    /// Invalid benefit payment
    #[error(
        "Invalid benefit payment. Reason: {reason}. \
         Member must meet a condition of release under SIS Regulations Schedule 1"
    )]
    InvalidBenefitPayment {
        /// Reason payment is invalid
        reason: String,
    },

    /// Death benefit nomination invalid
    #[error(
        "Death benefit nomination invalid under SIS Act 1993 s.59. \
         Reason: {reason}"
    )]
    InvalidDeathBenefitNomination {
        /// Reason nomination is invalid
        reason: String,
    },

    // =========================================================================
    // General Errors
    // =========================================================================
    /// Validation error
    #[error("Validation error: {message}")]
    ValidationError {
        /// Error message
        message: String,
    },

    /// APRA prudential standard breach
    #[error(
        "Breach of APRA prudential standard {standard}. \
         Description: {description}"
    )]
    PrudentialStandardBreach {
        /// Standard breached
        standard: String,
        /// Description of breach
        description: String,
    },
}

/// Result type for superannuation operations
pub type Result<T> = std::result::Result<T, SuperannuationError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sg_shortfall_error() {
        let error = SuperannuationError::SgShortfall {
            shortfall: 1500.0,
            quarter: "Q2 2024-25".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Superannuation Guarantee"));
        assert!(msg.contains("1992"));
        assert!(msg.contains("$1500.00"));
    }

    #[test]
    fn test_sole_purpose_breach_error() {
        let error = SuperannuationError::SolePurposeTestBreach {
            violation: "Private use of fund asset".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("s.62"));
        assert!(msg.contains("retirement benefits"));
    }

    #[test]
    fn test_contribution_cap_error() {
        let error = SuperannuationError::ContributionCapExceeded {
            cap_type: "concessional".to_string(),
            cap: 30000.0,
            contributed: 35000.0,
        };
        let msg = format!("{}", error);
        assert!(msg.contains("concessional"));
        assert!(msg.contains("$30000.00"));
    }
}
