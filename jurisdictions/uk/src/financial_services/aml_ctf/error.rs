//! AML/CTF Errors (Money Laundering Regulations 2017, POCA 2002, Terrorism Act 2000)

use thiserror::Error;

/// Errors related to Anti-Money Laundering and Counter-Terrorist Financing compliance
#[derive(Debug, Clone, Error, PartialEq)]
pub enum AmlCtfError {
    // ============================================================================
    // Customer Due Diligence Errors (MLR 2017)
    // ============================================================================
    /// Customer Due Diligence not performed (MLR 2017 Reg 27)
    #[error(
        "Customer Due Diligence not performed for customer '{customer_name}'. MLR 2017 Regulation 27 requires firms to apply CDD measures when: (a) establishing a business relationship, (b) carrying out an occasional transaction ≥€15,000, (c) suspecting money laundering or terrorist financing, (d) doubting the veracity of previously obtained customer identification."
    )]
    CddNotPerformed {
        /// Name of customer
        customer_name: String,
    },

    /// Customer identity not verified (MLR 2017 Reg 28(2))
    #[error(
        "Customer identity not verified for '{customer_name}'. MLR 2017 Regulation 28(2) requires firms to verify customer identity using documents, data or information obtained from a reliable and independent source. Acceptable documents include passport, driving licence, or certified identity documents."
    )]
    IdentityNotVerified {
        /// Name of customer
        customer_name: String,
    },

    /// Beneficial ownership not established (MLR 2017 Reg 5, 28(3))
    #[error(
        "Beneficial ownership not established for entity '{entity_name}'. MLR 2017 Regulation 5 defines beneficial owner as any individual who ultimately owns or controls more than 25% of the shares or voting rights in the entity. Regulation 28(3)(b) requires firms to identify the beneficial owner and take reasonable measures to verify the beneficial owner's identity."
    )]
    BeneficialOwnershipNotEstablished {
        /// Name of entity
        entity_name: String,
    },

    /// Purpose of business relationship not established (MLR 2017 Reg 28(3)(c))
    #[error(
        "Purpose and intended nature of business relationship not established for '{customer_name}'. MLR 2017 Regulation 28(3)(c) requires firms to obtain information on the purpose and intended nature of the business relationship."
    )]
    PurposeNotEstablished {
        /// Name of customer
        customer_name: String,
    },

    // ============================================================================
    // Enhanced Due Diligence Errors (MLR 2017 Reg 33-35)
    // ============================================================================
    /// Enhanced Due Diligence not performed when required (MLR 2017 Reg 33)
    #[error(
        "Enhanced Due Diligence not performed for high-risk customer '{customer_name}'. MLR 2017 Regulation 33 requires Enhanced DD for: (a) customers not physically present for identification, (b) customers from high-risk third countries, (c) transactions or business relationships involving high-risk third countries."
    )]
    EddNotPerformed {
        /// Name of customer
        customer_name: String,

        /// Reason EDD is required
        reason: String,
    },

    /// Enhanced Due Diligence not performed for PEP (MLR 2017 Reg 35)
    #[error(
        "Enhanced Due Diligence not performed for Politically Exposed Person '{pep_name}' ({position}). MLR 2017 Regulation 35(4) requires firms to: (a) have approval from senior management for establishing or continuing business relationship, (b) take adequate measures to establish the source of wealth and source of funds, (c) conduct enhanced ongoing monitoring of the business relationship."
    )]
    EddNotPerformedForPep {
        /// Name of PEP
        pep_name: String,

        /// PEP position/status
        position: String,
    },

    /// Source of wealth/funds not established for Enhanced DD (MLR 2017 Reg 35(4)(b))
    #[error(
        "Source of wealth and source of funds not established for PEP '{pep_name}'. MLR 2017 Regulation 35(4)(b) requires adequate measures to establish the source of wealth and source of funds involved in business relationships or transactions with PEPs."
    )]
    SourceOfWealthNotEstablished {
        /// Name of PEP
        pep_name: String,
    },

    /// Senior management approval not obtained for PEP (MLR 2017 Reg 35(4)(a))
    #[error(
        "Senior management approval not obtained for establishing business relationship with PEP '{pep_name}'. MLR 2017 Regulation 35(4)(a) requires approval from senior management for establishing (or continuing, where the PEP status was not known at outset) a business relationship with a PEP."
    )]
    SeniorManagementApprovalNotObtained {
        /// Name of PEP
        pep_name: String,
    },

    // ============================================================================
    // Suspicious Activity Reporting Errors (POCA 2002, Terrorism Act 2000)
    // ============================================================================
    /// Suspicious Activity Report not filed when required (POCA 2002 s.330, Terrorism Act 2000 s.21A)
    #[error(
        "Suspicious Activity Report not filed for suspicious transaction involving '{subject_name}'. {details}. POCA 2002 s.330 (regulated sector) and Terrorism Act 2000 s.21A create a criminal offence for failure to disclose knowledge or suspicion of money laundering or terrorist financing to the National Crime Agency. Penalty: up to 5 years imprisonment and/or unlimited fine."
    )]
    SarNotFiled {
        /// Subject of suspicion
        subject_name: String,

        /// Additional details
        details: String,
    },

    /// SAR missing required information
    #[error(
        "Suspicious Activity Report missing required information: {missing_fields}. SARs must include sufficient information to allow NCA to identify the subject and assess the suspicion."
    )]
    SarIncomplete {
        /// Fields that are missing
        missing_fields: String,
    },

    /// Tipping off offence risk (POCA 2002 s.333A, Terrorism Act 2000 s.21D)
    #[error(
        "Risk of tipping off offence for disclosure to '{person}'. POCA 2002 s.333A and Terrorism Act 2000 s.21D make it an offence to make a disclosure likely to prejudice an investigation, if the person knows or suspects that a disclosure under s.330 (SAR) has been made. Penalty: up to 5 years imprisonment and/or unlimited fine."
    )]
    TippingOffRisk {
        /// Person to whom disclosure would be made
        person: String,
    },

    // ============================================================================
    // Sanctions Errors (Sanctions and Anti-Money Laundering Act 2018)
    // ============================================================================
    /// Sanctions violation detected
    #[error(
        "Sanctions violation: {details}. Trading with, or making funds or economic resources available to, sanctioned persons or entities is a criminal offence under the Sanctions and Anti-Money Laundering Act 2018. UK sanctions are administered by the Office of Financial Sanctions Implementation (OFSI). Penalties: up to 7 years imprisonment and/or unlimited fine."
    )]
    SanctionsViolation {
        /// Details of violation
        details: String,
    },

    /// Sanctions screening not performed
    #[error(
        "Sanctions screening not performed for '{subject_name}'. Firms must screen all customers and transactions against UK, UN, and other applicable sanctions lists before establishing business relationships or processing transactions."
    )]
    SanctionsScreeningNotPerformed {
        /// Name of subject
        subject_name: String,
    },

    /// Sanctions match not resolved
    #[error(
        "Sanctions screening match for '{subject_name}' not resolved. Potential matches against sanctions lists must be investigated and either confirmed as true positive (transaction prohibited) or false positive (transaction may proceed with documented rationale)."
    )]
    SanctionsMatchNotResolved {
        /// Name of subject
        subject_name: String,

        /// Sanctions list with match
        sanctions_list: String,
    },

    // ============================================================================
    // Cryptoasset Travel Rule Errors (MLR 2017 reg 14A)
    // ============================================================================
    /// Travel Rule violation for cryptoasset transfer (MLR 2017 reg 14A)
    #[error(
        "Travel Rule non-compliance for cryptoasset transfer of £{amount_gbp:.2}. MLR 2017 Regulation 14A requires cryptoasset exchange providers to obtain and transmit information on the originator and beneficiary for transfers ≥£1,000. This implements FATF Recommendation 16 (Travel Rule) for virtual assets."
    )]
    TravelRuleViolation {
        /// Transfer amount in GBP
        amount_gbp: f64,
    },

    /// Incomplete originator information for Travel Rule
    #[error(
        "Incomplete originator information for cryptoasset transfer of £{amount_gbp:.2}. MLR 2017 reg 14A(3) requires: (a) originator's name, (b) originator's account number (or unique transaction reference), (c) originator's address, national identity number, customer identification number, or date and place of birth."
    )]
    IncompleteOriginatorInfo {
        /// Transfer amount in GBP
        amount_gbp: f64,
    },

    /// Incomplete beneficiary information for Travel Rule
    #[error(
        "Incomplete beneficiary information for cryptoasset transfer of £{amount_gbp:.2}. MLR 2017 reg 14A(4) requires: (a) beneficiary's name, (b) beneficiary's account number (or unique transaction reference)."
    )]
    IncompleteBeneficiaryInfo {
        /// Transfer amount in GBP
        amount_gbp: f64,
    },

    // ============================================================================
    // Ongoing Monitoring Errors (MLR 2017 Reg 28(4))
    // ============================================================================
    /// Ongoing monitoring not performed (MLR 2017 Reg 28(4))
    #[error(
        "Ongoing monitoring not performed for customer '{customer_name}'. MLR 2017 Regulation 28(4) requires firms to conduct ongoing monitoring of business relationship, including scrutiny of transactions to ensure they are consistent with the firm's knowledge of the customer, their business and risk profile."
    )]
    OngoingMonitoringNotPerformed {
        /// Name of customer
        customer_name: String,
    },

    /// CDD review overdue
    #[error(
        "CDD review overdue for customer '{customer_name}'. Last review: {last_review_date}. Review frequency: {review_frequency}. Ongoing monitoring requires periodic review of CDD information, with frequency based on risk assessment."
    )]
    CddReviewOverdue {
        /// Name of customer
        customer_name: String,

        /// Date of last review
        last_review_date: String,

        /// Required review frequency
        review_frequency: String,
    },

    // ============================================================================
    // General Validation Errors
    // ============================================================================
    /// AML/CTF validation error
    #[error("AML/CTF validation error: {message}")]
    ValidationError {
        /// Error message
        message: String,
    },

    /// Multiple AML/CTF errors
    #[error("Multiple AML/CTF errors detected: {count} errors")]
    MultipleErrors {
        /// Number of errors
        count: usize,

        /// Error details
        errors: Vec<String>,
    },
}

/// Result type for AML/CTF operations
pub type Result<T> = std::result::Result<T, AmlCtfError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages_contain_references() {
        // Test CDD error includes MLR 2017 reference
        let error = AmlCtfError::CddNotPerformed {
            customer_name: "Test Customer".to_string(),
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("MLR 2017"));
        assert!(error_msg.contains("Regulation 27"));

        // Test PEP error includes MLR 2017 Reg 35
        let error = AmlCtfError::EddNotPerformedForPep {
            pep_name: "John Doe".to_string(),
            position: "Minister".to_string(),
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("MLR 2017"));
        assert!(error_msg.contains("Regulation 35(4)"));

        // Test SAR error includes POCA 2002
        let error = AmlCtfError::SarNotFiled {
            subject_name: "Suspicious Person".to_string(),
            details: "Large cash deposit".to_string(),
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("POCA 2002"));
        assert!(error_msg.contains("s.330"));

        // Test sanctions error
        let error = AmlCtfError::SanctionsViolation {
            details: "Transaction with sanctioned entity".to_string(),
        };
        let error_msg = error.to_string();
        assert!(error_msg.contains("Sanctions and Anti-Money Laundering Act 2018"));
        assert!(error_msg.contains("OFSI"));

        // Test Travel Rule error
        let error = AmlCtfError::TravelRuleViolation { amount_gbp: 5000.0 };
        let error_msg = error.to_string();
        assert!(error_msg.contains("MLR 2017"));
        assert!(error_msg.contains("Regulation 14A"));
        assert!(error_msg.contains("FATF Recommendation 16"));
    }

    #[test]
    fn test_error_includes_penalties() {
        // SAR error should mention imprisonment penalty
        let error = AmlCtfError::SarNotFiled {
            subject_name: "Test".to_string(),
            details: "Test details".to_string(),
        };
        assert!(error.to_string().contains("5 years imprisonment"));

        // Sanctions error should mention penalty
        let error = AmlCtfError::SanctionsViolation {
            details: "Test".to_string(),
        };
        assert!(error.to_string().contains("7 years imprisonment"));
    }
}
