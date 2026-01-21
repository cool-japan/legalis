//! Financial Services Errors (Corporations Act 2001 Chapter 7)

use thiserror::Error;

/// Errors related to Australian financial services regulation
#[derive(Debug, Clone, Error, PartialEq)]
pub enum FinancialServicesError {
    // ============================================================================
    // AFSL Authorization Errors (s.911A)
    // ============================================================================
    /// Person not holding required AFSL
    #[error(
        "'{entity_name}' does not hold an Australian Financial Services License required to provide financial services. \
         See Corporations Act 2001 s.911A. Providing financial services without a license is a criminal offence \
         punishable by up to 5 years imprisonment and/or 500 penalty units."
    )]
    NoAfsl { entity_name: String },

    /// AFSL suspended or cancelled
    #[error(
        "AFSL {license_number} is {status}. Cannot conduct financial services while license is not current. \
         See Corporations Act 2001 s.915A-915C."
    )]
    LicenseNotCurrent {
        license_number: String,
        status: String,
    },

    /// Service not authorized under AFSL
    #[error(
        "Service '{service}' is not authorized under AFSL {license_number}. \
         Authorized services: {authorized_services}. See Corporations Act 2001 s.911A(2)."
    )]
    ServiceNotAuthorized {
        service: String,
        license_number: String,
        authorized_services: String,
    },

    /// License condition breach
    #[error(
        "Breach of AFSL condition: {condition}. Details: {details}. \
         See Corporations Act 2001 s.912A(1)(b). Breaching license conditions may result in \
         suspension or cancellation of the license."
    )]
    LicenseConditionBreach { condition: String, details: String },

    // ============================================================================
    // General Obligations Errors (s.912A)
    // ============================================================================
    /// Failure to provide services efficiently, honestly and fairly
    #[error(
        "Breach of general obligation: Failure to provide financial services efficiently, honestly and fairly. \
         Details: {details}. See Corporations Act 2001 s.912A(1)(a)."
    )]
    BreachEfficientHonestFair { details: String },

    /// Inadequate conflicts management
    #[error(
        "Breach of general obligation: Inadequate arrangements for managing conflicts of interest. \
         Details: {details}. See Corporations Act 2001 s.912A(1)(aa)."
    )]
    InadequateConflictsManagement { details: String },

    /// Failure to comply with financial services laws
    #[error(
        "Breach of general obligation: Failure to comply with financial services laws. \
         Details: {details}. See Corporations Act 2001 s.912A(1)(c)."
    )]
    BreachFinancialServicesLaws { details: String },

    /// Inadequate risk management
    #[error(
        "Breach of general obligation: Inadequate risk management systems. \
         Details: {details}. See Corporations Act 2001 s.912A(1)(h)."
    )]
    InadequateRiskManagement { details: String },

    /// Inadequate competence
    #[error(
        "Breach of general obligation: Failure to maintain competence to provide financial services. \
         Details: {details}. See Corporations Act 2001 s.912A(1)(e)."
    )]
    InadequateCompetence { details: String },

    /// Inadequate training
    #[error(
        "Breach of general obligation: Representatives not adequately trained or competent. \
         Details: {details}. See Corporations Act 2001 s.912A(1)(f)."
    )]
    InadequateTraining { details: String },

    /// Inadequate resources
    #[error(
        "Breach of general obligation: Inadequate resources to provide financial services. \
         Details: {details}. See Corporations Act 2001 s.912A(1)(d)."
    )]
    InadequateResources { details: String },

    // ============================================================================
    // Client Classification Errors (s.761G)
    // ============================================================================
    /// Invalid wholesale client classification
    #[error(
        "Invalid wholesale client classification for '{client_name}'. Basis '{basis}' does not meet requirements. \
         {reason}. See Corporations Act 2001 s.761G(7)."
    )]
    InvalidWholesaleClassification {
        client_name: String,
        basis: String,
        reason: String,
    },

    /// Missing accountant certificate
    #[error(
        "Wholesale client classification requires accountant certificate. \
         Client '{client_name}' has not provided required certificate for {test_type}. \
         See Corporations Act 2001 s.761G(7)(c) and Corporations Regulations 7.1.26."
    )]
    MissingAccountantCertificate {
        client_name: String,
        test_type: String,
    },

    /// Classification expired
    #[error(
        "Wholesale client classification for '{client_name}' expired on {expiry_date}. \
         Accountant certificates are valid for 2 years. Must treat as retail client or obtain new certificate."
    )]
    ClassificationExpired {
        client_name: String,
        expiry_date: String,
    },

    // ============================================================================
    // Best Interests Duty Errors (s.961B)
    // ============================================================================
    /// Failure to act in best interests
    #[error(
        "Breach of best interests duty: Adviser failed to act in the best interests of the client. \
         Details: {details}. See Corporations Act 2001 s.961B(1). \
         Breach may result in civil penalties up to $1.11M for individuals."
    )]
    BreachBestInterestsDuty { details: String },

    /// Failure to complete safe harbour steps
    #[error(
        "Best interests duty: Safe harbour step not completed: {step}. Details: {details}. \
         See Corporations Act 2001 s.961B(2)."
    )]
    SafeHarbourStepMissing { step: String, details: String },

    /// Breach of priority rule
    #[error(
        "Breach of priority rule: Adviser failed to give priority to client's interests over own interests. \
         Details: {details}. See Corporations Act 2001 s.961J."
    )]
    BreachPriorityRule { details: String },

    // ============================================================================
    // Disclosure Errors (Part 7.9)
    // ============================================================================
    /// FSG not provided
    #[error(
        "Financial Services Guide not provided to retail client before providing financial service. \
         See Corporations Act 2001 s.941A."
    )]
    FsgNotProvided,

    /// FSG deficient
    #[error(
        "Financial Services Guide deficient: {deficiency}. See Corporations Act 2001 s.942B-C."
    )]
    FsgDeficient { deficiency: String },

    /// PDS not provided
    #[error(
        "Product Disclosure Statement not provided to retail client before acquiring financial product. \
         See Corporations Act 2001 s.1012A."
    )]
    PdsNotProvided,

    /// PDS deficient
    #[error(
        "Product Disclosure Statement deficient: {deficiency}. See Corporations Act 2001 s.1013C-E."
    )]
    PdsDeficient { deficiency: String },

    /// SOA not provided
    #[error(
        "Statement of Advice not provided after giving personal advice to retail client. \
         See Corporations Act 2001 s.946A."
    )]
    SoaNotProvided,

    /// SOA deficient
    #[error("Statement of Advice deficient: {deficiency}. See Corporations Act 2001 s.947B-C.")]
    SoaDeficient { deficiency: String },

    // ============================================================================
    // Conflicted Remuneration Errors (Part 7.7A)
    // ============================================================================
    /// Conflicted remuneration received
    #[error(
        "Conflicted remuneration received: {description}. Value: ${amount}. \
         Conflicted remuneration is banned under Corporations Act 2001 s.963E."
    )]
    ConflictedRemuneration { description: String, amount: f64 },

    /// Soft dollar benefits received
    #[error(
        "Soft dollar benefit received: {description}. Soft dollar benefits are generally prohibited \
         under Corporations Act 2001 s.963E unless exempt."
    )]
    SoftDollarBenefit { description: String },

    // ============================================================================
    // Dispute Resolution Errors (s.912A(1)(g))
    // ============================================================================
    /// No IDR system
    #[error(
        "No internal dispute resolution system in place. AFSL holders must have an IDR system \
         that complies with ASIC RG 271. See Corporations Act 2001 s.912A(1)(g)."
    )]
    NoIdrSystem,

    /// IDR non-compliant
    #[error("Internal dispute resolution system does not comply with ASIC RG 271: {deficiency}.")]
    IdrNonCompliant { deficiency: String },

    /// No EDR membership
    #[error(
        "Not a member of Australian Financial Complaints Authority (AFCA). \
         Retail client service providers must be AFCA members. See Corporations Act 2001 s.912A(1)(g)."
    )]
    NoEdrMembership,

    // ============================================================================
    // Compensation Arrangement Errors (s.912B)
    // ============================================================================
    /// Inadequate compensation arrangements
    #[error(
        "Inadequate compensation arrangements: {reason}. \
         See Corporations Act 2001 s.912B."
    )]
    InadequateCompensationArrangements { reason: String },

    /// Insufficient PI insurance
    #[error(
        "Professional indemnity insurance coverage ${coverage_aud} is insufficient. \
         Required coverage: ${required_aud}. See Corporations Act 2001 s.912B and ASIC RG 126."
    )]
    InsufficientPiInsurance {
        coverage_aud: f64,
        required_aud: f64,
    },

    /// PI insurance expired
    #[error(
        "Professional indemnity insurance expired on {expiry_date}. \
         Must maintain adequate compensation arrangements at all times. See Corporations Act 2001 s.912B."
    )]
    PiInsuranceExpired { expiry_date: String },

    // ============================================================================
    // Breach Reporting Errors (s.912D-E)
    // ============================================================================
    /// Significant breach not reported
    #[error(
        "Significant breach not reported to ASIC within required timeframe. \
         Breach identified {days_ago} days ago. Must report within 30 days. \
         See Corporations Act 2001 s.912D."
    )]
    SignificantBreachNotReported { days_ago: u32 },

    /// Late breach notification
    #[error(
        "Breach notification to ASIC was late. Breach identified on {identification_date}, \
         reported on {notification_date}. Required within 30 days. See Corporations Act 2001 s.912D."
    )]
    LateBreachNotification {
        identification_date: String,
        notification_date: String,
    },

    // ============================================================================
    // Representative Errors
    // ============================================================================
    /// Unauthorised representative
    #[error(
        "'{representative_name}' is not an authorised representative of AFSL {license_number}. \
         See Corporations Act 2001 s.916A."
    )]
    UnauthorisedRepresentative {
        representative_name: String,
        license_number: String,
    },

    /// Representative not competent
    #[error(
        "Authorised representative '{representative_name}' does not meet competency requirements \
         for {service}. See Corporations Act 2001 s.912A(1)(f) and ASIC RG 146."
    )]
    RepresentativeNotCompetent {
        representative_name: String,
        service: String,
    },

    // ============================================================================
    // General Errors
    // ============================================================================
    /// Validation error
    #[error("Financial services validation error: {message}")]
    ValidationError { message: String },

    /// Multiple errors
    #[error("Multiple financial services errors: {errors:?}")]
    MultipleErrors { errors: Vec<String> },
}

/// Result type for financial services operations
pub type Result<T> = std::result::Result<T, FinancialServicesError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages_contain_references() {
        let error = FinancialServicesError::NoAfsl {
            entity_name: "Test Firm".to_string(),
        };
        assert!(error.to_string().contains("Corporations Act 2001"));
        assert!(error.to_string().contains("s.911A"));

        let error = FinancialServicesError::BreachBestInterestsDuty {
            details: "Failed to investigate".to_string(),
        };
        assert!(error.to_string().contains("s.961B"));
    }

    #[test]
    fn test_general_obligation_errors() {
        let errors = vec![
            FinancialServicesError::BreachEfficientHonestFair {
                details: "test".to_string(),
            },
            FinancialServicesError::InadequateConflictsManagement {
                details: "test".to_string(),
            },
            FinancialServicesError::InadequateResources {
                details: "test".to_string(),
            },
        ];

        for error in errors {
            assert!(error.to_string().contains("s.912A"));
        }
    }

    #[test]
    fn test_disclosure_errors() {
        let fsg_error = FinancialServicesError::FsgNotProvided;
        assert!(fsg_error.to_string().contains("Financial Services Guide"));
        assert!(fsg_error.to_string().contains("s.941A"));

        let pds_error = FinancialServicesError::PdsNotProvided;
        assert!(
            pds_error
                .to_string()
                .contains("Product Disclosure Statement")
        );
        assert!(pds_error.to_string().contains("s.1012A"));

        let soa_error = FinancialServicesError::SoaNotProvided;
        assert!(soa_error.to_string().contains("Statement of Advice"));
        assert!(soa_error.to_string().contains("s.946A"));
    }

    #[test]
    fn test_client_classification_errors() {
        let error = FinancialServicesError::InvalidWholesaleClassification {
            client_name: "John Smith".to_string(),
            basis: "Assets test".to_string(),
            reason: "Net assets below $2.5M".to_string(),
        };
        assert!(error.to_string().contains("s.761G"));

        let error = FinancialServicesError::MissingAccountantCertificate {
            client_name: "Jane Doe".to_string(),
            test_type: "assets test".to_string(),
        };
        assert!(error.to_string().contains("accountant certificate"));
    }
}
