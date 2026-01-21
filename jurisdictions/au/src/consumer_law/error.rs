//! Consumer law error types
//!
//! Error types for the Competition and Consumer Act 2010 (Cth) and
//! Australian Consumer Law (ACL) enforcement.

use thiserror::Error;

/// Consumer law error type
#[derive(Debug, Clone, Error)]
pub enum ConsumerLawError {
    // =========================================================================
    // Product Safety Errors
    // =========================================================================
    /// Product safety breach
    #[error(
        "Product safety breach under ACL Part 3-3. \
         Product: {product_name}. Breach: {breach_description}. \
         Reference: ACL s.{section}"
    )]
    ProductSafetyBreach {
        /// Product name
        product_name: String,
        /// Breach description
        breach_description: String,
        /// ACL section
        section: String,
    },

    /// Mandatory safety standard not met
    #[error(
        "Product does not comply with mandatory safety standard. \
         Product: {product_name}. Standard: {standard}. \
         Deficiency: {deficiency}"
    )]
    SafetyStandardNotMet {
        /// Product name
        product_name: String,
        /// Safety standard reference
        standard: String,
        /// Deficiency details
        deficiency: String,
    },

    /// Product recall required
    #[error(
        "Product recall required under ACL s.122. \
         Product: {product_name}. Reason: {reason}. \
         Recall type: {recall_type}"
    )]
    ProductRecallRequired {
        /// Product name
        product_name: String,
        /// Reason for recall
        reason: String,
        /// Recall type (voluntary/mandatory)
        recall_type: String,
    },

    /// Mandatory injury report not made
    #[error(
        "Mandatory injury report not made within 2 days under ACL s.131. \
         Product: {product_name}. Injury type: {injury_type}"
    )]
    InjuryReportNotMade {
        /// Product name
        product_name: String,
        /// Type of injury
        injury_type: String,
    },

    /// Banned product supplied
    #[error(
        "Banned product supplied contrary to ACL s.114. \
         Product: {product_name}. Ban reference: {ban_reference}"
    )]
    BannedProductSupplied {
        /// Product name
        product_name: String,
        /// Ban reference
        ban_reference: String,
    },

    // =========================================================================
    // ACCC Enforcement Errors
    // =========================================================================
    /// Infringement notice issued
    #[error(
        "ACCC infringement notice issued under Competition and Consumer Act 2010 s.134A. \
         Contravention: {contravention}. Penalty amount: ${penalty_amount:.2}"
    )]
    InfringementNoticeIssued {
        /// Contravention description
        contravention: String,
        /// Penalty amount
        penalty_amount: f64,
    },

    /// Court-ordered undertaking breach
    #[error(
        "Breach of court-ordered undertaking under CCA s.87B. \
         Undertaking: {undertaking}. Breach: {breach}"
    )]
    UndertakingBreach {
        /// Undertaking description
        undertaking: String,
        /// Breach details
        breach: String,
    },

    /// Civil penalty contravention
    #[error(
        "Civil penalty contravention under ACL. \
         Provision: ACL s.{section}. Maximum penalty: ${max_penalty:.2}. \
         Conduct: {conduct}"
    )]
    CivilPenaltyContravention {
        /// ACL section
        section: String,
        /// Maximum penalty
        max_penalty: f64,
        /// Conduct description
        conduct: String,
    },

    /// Substantiation notice not complied with
    #[error(
        "Substantiation notice not complied with under ACL s.219. \
         Notice date: {notice_date}. Claim: {claim}"
    )]
    SubstantiationNoticeNotComplied {
        /// Notice date
        notice_date: String,
        /// Claim that needed substantiation
        claim: String,
    },

    // =========================================================================
    // Unsolicited Consumer Agreement Errors
    // =========================================================================
    /// Unsolicited agreement requirements not met
    #[error(
        "Unsolicited consumer agreement requirements not met under ACL Part 3-2 Division 2. \
         Requirement: {requirement}. Section: s.{section}"
    )]
    UnsolicitedAgreementRequirementsNotMet {
        /// Requirement not met
        requirement: String,
        /// ACL section
        section: String,
    },

    /// Cooling-off rights not provided
    #[error(
        "Cooling-off rights not provided for unsolicited consumer agreement. \
         Required period: {cooling_off_days} business days under ACL s.82"
    )]
    CoolingOffRightsNotProvided {
        /// Cooling-off period in business days
        cooling_off_days: u32,
    },

    /// Prohibited contact times breach
    #[error(
        "Contact made during prohibited times under ACL s.73. \
         Contact time: {contact_time}. Permitted hours: 9am-6pm Mon-Fri, 9am-5pm Sat"
    )]
    ProhibitedContactTimesBreach {
        /// Time of contact
        contact_time: String,
    },

    /// Do Not Call Register violation
    #[error(
        "Contact made to number on Do Not Call Register. \
         Number: {phone_number}. Penalty: up to $2.1 million"
    )]
    DoNotCallRegisterViolation {
        /// Phone number
        phone_number: String,
    },

    // =========================================================================
    // Country of Origin Errors
    // =========================================================================
    /// Country of origin claim breach
    #[error(
        "Country of origin claim breach under ACL Part 5-3. \
         Claim: {claim}. Product: {product}. Issue: {issue}"
    )]
    CountryOfOriginBreach {
        /// Claim made
        claim: String,
        /// Product
        product: String,
        /// Issue with claim
        issue: String,
    },

    /// Safe harbour defence not available
    #[error(
        "Safe harbour defence not available for country of origin claim. \
         Claim: {claim}. Reason: {reason}"
    )]
    SafeHarbourDefenceNotAvailable {
        /// Claim made
        claim: String,
        /// Why safe harbour not available
        reason: String,
    },

    // =========================================================================
    // Lay-by Agreement Errors
    // =========================================================================
    /// Lay-by agreement requirements not met
    #[error(
        "Lay-by agreement requirements not met under ACL s.96. \
         Missing requirement: {requirement}"
    )]
    LayByRequirementsNotMet {
        /// Missing requirement
        requirement: String,
    },

    /// Lay-by termination breach
    #[error(
        "Lay-by termination breach under ACL s.98-99. \
         Issue: {issue}. Consumer right: {consumer_right}"
    )]
    LayByTerminationBreach {
        /// Issue with termination
        issue: String,
        /// Consumer right affected
        consumer_right: String,
    },

    // =========================================================================
    // General Errors
    // =========================================================================
    /// Validation error
    #[error("Consumer law validation error: {message}")]
    ValidationError {
        /// Error message
        message: String,
    },
}

/// Result type for consumer law operations
pub type Result<T> = std::result::Result<T, ConsumerLawError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_safety_breach() {
        let error = ConsumerLawError::ProductSafetyBreach {
            product_name: "Children's toy".to_string(),
            breach_description: "Contains small parts posing choking hazard".to_string(),
            section: "106".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("ACL Part 3-3"));
        assert!(msg.contains("choking hazard"));
    }

    #[test]
    fn test_infringement_notice() {
        let error = ConsumerLawError::InfringementNoticeIssued {
            contravention: "False or misleading representations".to_string(),
            penalty_amount: 13320.0,
        };
        let msg = format!("{}", error);
        assert!(msg.contains("s.134A"));
        assert!(msg.contains("13320.00"));
    }

    #[test]
    fn test_cooling_off_rights() {
        let error = ConsumerLawError::CoolingOffRightsNotProvided {
            cooling_off_days: 10,
        };
        let msg = format!("{}", error);
        assert!(msg.contains("s.82"));
        assert!(msg.contains("10 business days"));
    }

    #[test]
    fn test_country_of_origin() {
        let error = ConsumerLawError::CountryOfOriginBreach {
            claim: "Made in Australia".to_string(),
            product: "Clothing".to_string(),
            issue: "Components imported from overseas".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Part 5-3"));
        assert!(msg.contains("Components imported"));
    }
}
