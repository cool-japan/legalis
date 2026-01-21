//! Immigration and citizenship error types
//!
//! Error types for the Migration Act 1958 (Cth) and Australian Citizenship Act 2007 (Cth).

use thiserror::Error;

/// Immigration and citizenship error type
#[derive(Debug, Clone, Error)]
pub enum ImmigrationError {
    // =========================================================================
    // Visa Application Errors
    // =========================================================================
    /// Visa application refused
    #[error(
        "Visa application refused under Migration Act 1958 (Cth) s.65. \
         Visa subclass: {visa_subclass}. Reason: {reason}"
    )]
    VisaRefused {
        /// Visa subclass applied for
        visa_subclass: String,
        /// Reason for refusal
        reason: String,
    },

    /// Visa criteria not met
    #[error(
        "Visa criteria not satisfied under Migration Regulations 1994. \
         Subclass {visa_subclass}: {criterion} not met. Requirement: {requirement}"
    )]
    VisaCriteriaNotMet {
        /// Visa subclass
        visa_subclass: String,
        /// Criterion not met
        criterion: String,
        /// Requirement description
        requirement: String,
    },

    /// Public interest criteria not satisfied
    #[error(
        "Public Interest Criteria (PIC) not satisfied under Migration Regulations 1994 Sch 4. \
         PIC {pic_number}: {description}"
    )]
    PublicInterestCriteriaNotMet {
        /// PIC number
        pic_number: String,
        /// Description of PIC failure
        description: String,
    },

    /// Health requirement not met
    #[error(
        "Health requirement not satisfied under PIC 4005-4007. \
         Condition: {condition}. Significant cost threshold: ${threshold:.2}"
    )]
    HealthRequirementNotMet {
        /// Health condition
        condition: String,
        /// Significant cost threshold
        threshold: f64,
    },

    // =========================================================================
    // Character Test Errors (s.501)
    // =========================================================================
    /// Character test not passed
    #[error(
        "Character test not passed under Migration Act 1958 (Cth) s.501. \
         Ground: {ground}. Details: {details}"
    )]
    CharacterTestFailed {
        /// Ground for character concern
        ground: String,
        /// Details of character concern
        details: String,
    },

    /// Substantial criminal record
    #[error(
        "Person has substantial criminal record under s.501(7). \
         Sentence: {sentence}. Threshold: 12 months imprisonment"
    )]
    SubstantialCriminalRecord {
        /// Sentence details
        sentence: String,
    },

    /// Association with criminal conduct
    #[error(
        "Person has association with criminal conduct under s.501(6)(b). \
         Association type: {association_type}. Group: {group}"
    )]
    CriminalAssociation {
        /// Type of association
        association_type: String,
        /// Criminal group
        group: String,
    },

    // =========================================================================
    // Visa Cancellation Errors
    // =========================================================================
    /// Visa cancelled under s.116
    #[error(
        "Visa cancelled under Migration Act 1958 (Cth) s.116. \
         Ground: {ground}. Visa: {visa_subclass}"
    )]
    VisaCancelledS116 {
        /// Ground for cancellation
        ground: String,
        /// Visa subclass
        visa_subclass: String,
    },

    /// Visa cancelled under s.501
    #[error(
        "Visa cancelled on character grounds under s.501. \
         Character concern: {character_concern}. \
         Minister's decision: mandatory/discretionary cancellation"
    )]
    VisaCancelledS501 {
        /// Character concern
        character_concern: String,
    },

    /// Visa condition breached
    #[error(
        "Visa condition breached. Condition {condition_number}: {condition_description}. \
         Breach: {breach}. May result in visa cancellation under s.116(1)(b)"
    )]
    VisaConditionBreached {
        /// Condition number (e.g., 8101, 8501)
        condition_number: String,
        /// Condition description
        condition_description: String,
        /// Details of breach
        breach: String,
    },

    // =========================================================================
    // Unlawful Non-Citizen Errors
    // =========================================================================
    /// Unlawful non-citizen status
    #[error(
        "Person is unlawful non-citizen under Migration Act 1958 (Cth) s.14. \
         No valid visa held. Subject to detention under s.189"
    )]
    UnlawfulNonCitizen,

    /// Bridging visa refused
    #[error(
        "Bridging visa refused. Subclass {visa_subclass}. \
         Reason: {reason}. Person may be liable to detention"
    )]
    BridgingVisaRefused {
        /// Bridging visa subclass
        visa_subclass: String,
        /// Reason for refusal
        reason: String,
    },

    /// Removal from Australia
    #[error(
        "Person liable to removal under Migration Act 1958 (Cth) s.198. \
         Removal destination: {destination}. Bars to removal: {bars}"
    )]
    LiableToRemoval {
        /// Removal destination country
        destination: String,
        /// Any bars to removal
        bars: String,
    },

    // =========================================================================
    // Sponsorship Errors
    // =========================================================================
    /// Sponsorship application refused
    #[error(
        "Sponsorship/nomination refused under Migration Act 1958 (Cth). \
         Type: {sponsorship_type}. Reason: {reason}"
    )]
    SponsorshipRefused {
        /// Type of sponsorship
        sponsorship_type: String,
        /// Reason for refusal
        reason: String,
    },

    /// Sponsorship obligations breached
    #[error(
        "Sponsorship obligations breached under Migration Act 1958 (Cth) s.140H. \
         Obligation: {obligation}. Consequence: {consequence}"
    )]
    SponsorshipObligationsBreach {
        /// Obligation breached
        obligation: String,
        /// Consequence of breach
        consequence: String,
    },

    /// Standard business sponsor approval cancelled
    #[error(
        "Standard Business Sponsor (SBS) approval cancelled. \
         Ground: {ground}. Effect on sponsored workers: {effect}"
    )]
    SbsApprovalCancelled {
        /// Ground for cancellation
        ground: String,
        /// Effect on sponsored workers
        effect: String,
    },

    // =========================================================================
    // Employer Sanction Errors
    // =========================================================================
    /// Illegal worker employed
    #[error(
        "Illegal worker offence under Migration Act 1958 (Cth) s.245AB-245AC. \
         Number of workers: {num_workers}. Penalty: up to 2 years imprisonment"
    )]
    IllegalWorkerOffence {
        /// Number of illegal workers
        num_workers: u32,
    },

    /// Work condition breach by employer
    #[error(
        "Employer allowed breach of work condition under s.245ABA. \
         Condition breached: {condition}. Visa holder: {visa_subclass}"
    )]
    WorkConditionBreachByEmployer {
        /// Condition breached
        condition: String,
        /// Visa subclass
        visa_subclass: String,
    },

    // =========================================================================
    // Citizenship Errors
    // =========================================================================
    /// Citizenship application refused
    #[error(
        "Citizenship application refused under Australian Citizenship Act 2007 (Cth). \
         Stream: {citizenship_stream}. Reason: {reason}"
    )]
    CitizenshipRefused {
        /// Citizenship stream
        citizenship_stream: String,
        /// Reason for refusal
        reason: String,
    },

    /// Residence requirement not met
    #[error(
        "Residence requirement not satisfied under s.22 Australian Citizenship Act 2007. \
         Required: {required_days} days. Present: {present_days} days. \
         Absences during qualifying period: {absences} days"
    )]
    ResidenceRequirementNotMet {
        /// Required days
        required_days: u32,
        /// Days actually present
        present_days: u32,
        /// Days absent
        absences: u32,
    },

    /// Citizenship test not passed
    #[error(
        "Citizenship test not passed. \
         Score: {score}%. Pass mark: 75%. Attempts: {attempts}/3"
    )]
    CitizenshipTestNotPassed {
        /// Test score
        score: u32,
        /// Number of attempts
        attempts: u32,
    },

    /// Citizenship revoked
    #[error(
        "Citizenship revoked under Australian Citizenship Act 2007 (Cth) s.{section}. \
         Ground: {ground}"
    )]
    CitizenshipRevoked {
        /// Section of Act
        section: String,
        /// Ground for revocation
        ground: String,
    },

    // =========================================================================
    // Tribunal/Review Errors
    // =========================================================================
    /// AAT review time limit expired
    #[error(
        "Administrative Appeals Tribunal review time limit expired. \
         Decision type: {decision_type}. Time limit: {time_limit}. \
         Application under Migration Act 1958 (Cth) Part 5/Part 7"
    )]
    AatTimeLimitExpired {
        /// Type of decision
        decision_type: String,
        /// Time limit
        time_limit: String,
    },

    /// Ministerial intervention request
    #[error(
        "Ministerial intervention sought under Migration Act 1958 (Cth) s.{section}. \
         Grounds: {grounds}. Note: Minister has non-compellable, non-delegable power"
    )]
    MinisterialInterventionSought {
        /// Section (351, 417, 501J)
        section: String,
        /// Grounds for intervention
        grounds: String,
    },

    // =========================================================================
    // General Errors
    // =========================================================================
    /// Validation error
    #[error("Immigration validation error: {message}")]
    ValidationError {
        /// Error message
        message: String,
    },

    /// Invalid visa subclass
    #[error("Invalid visa subclass: {subclass}")]
    InvalidVisaSubclass {
        /// Subclass
        subclass: String,
    },
}

/// Result type for immigration operations
pub type Result<T> = std::result::Result<T, ImmigrationError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visa_refused_error() {
        let error = ImmigrationError::VisaRefused {
            visa_subclass: "189".to_string(),
            reason: "Points test not satisfied - 60 points required, 55 obtained".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Migration Act 1958"));
        assert!(msg.contains("189"));
    }

    #[test]
    fn test_character_test_error() {
        let error = ImmigrationError::CharacterTestFailed {
            ground: "Substantial criminal record".to_string(),
            details: "Sentenced to 18 months imprisonment for fraud".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("s.501"));
        assert!(msg.contains("criminal record"));
    }

    #[test]
    fn test_citizenship_residence_error() {
        let error = ImmigrationError::ResidenceRequirementNotMet {
            required_days: 1460,
            present_days: 1200,
            absences: 260,
        };
        let msg = format!("{}", error);
        assert!(msg.contains("s.22"));
        assert!(msg.contains("1460"));
        assert!(msg.contains("1200"));
    }

    #[test]
    fn test_unlawful_non_citizen_error() {
        let error = ImmigrationError::UnlawfulNonCitizen;
        let msg = format!("{}", error);
        assert!(msg.contains("s.14"));
        assert!(msg.contains("s.189"));
    }

    #[test]
    fn test_employer_sanction_error() {
        let error = ImmigrationError::IllegalWorkerOffence { num_workers: 5 };
        let msg = format!("{}", error);
        assert!(msg.contains("s.245AB"));
        assert!(msg.contains("2 years"));
    }
}
