//! AFS Licensing Errors

use thiserror::Error;

/// Errors related to AFS licensing
#[derive(Debug, Clone, Error, PartialEq)]
pub enum AfsLicensingError {
    /// No AFSL held
    #[error(
        "'{entity_name}' does not hold an Australian Financial Services License. \
         See Corporations Act 2001 s.911A."
    )]
    NoLicense { entity_name: String },

    /// License not current
    #[error(
        "AFSL {license_number} is {status}. Cannot conduct regulated activities. \
         See Corporations Act 2001 s.915A."
    )]
    LicenseNotCurrent {
        license_number: String,
        status: String,
    },

    /// Service not authorized
    #[error(
        "Service '{service}' is not authorized under AFSL {license_number}. \
         Authorized services: {authorized}. See Corporations Act 2001 s.911A(2)."
    )]
    ServiceNotAuthorized {
        service: String,
        license_number: String,
        authorized: String,
    },

    /// Condition breach
    #[error(
        "Breach of AFSL condition '{condition_id}': {description}. \
         See Corporations Act 2001 s.912A(1)(b)."
    )]
    ConditionBreach {
        condition_id: String,
        description: String,
    },

    /// No responsible manager
    #[error(
        "No responsible manager nominated for AFSL {license_number}. \
         See ASIC RG 105."
    )]
    NoResponsibleManager { license_number: String },

    /// Responsible manager not fit and proper
    #[error(
        "Responsible manager '{name}' does not meet fit and proper requirements. \
         Details: {reason}. See ASIC RG 105."
    )]
    ResponsibleManagerNotFitProper { name: String, reason: String },

    /// AR not authorized
    #[error(
        "Authorised representative '{ar_name}' (AR {ar_number}) is not authorized to provide service. \
         See Corporations Act 2001 s.916A."
    )]
    ArNotAuthorized { ar_name: String, ar_number: String },

    /// AR not compliant
    #[error(
        "Authorised representative '{ar_name}' does not meet training requirements. \
         {reason}. See ASIC RG 146."
    )]
    ArNotCompliant { ar_name: String, reason: String },

    /// Validation error
    #[error("AFS licensing validation error: {message}")]
    ValidationError { message: String },
}

/// Result type
pub type Result<T> = std::result::Result<T, AfsLicensingError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages() {
        let error = AfsLicensingError::NoLicense {
            entity_name: "Test Co".to_string(),
        };
        assert!(error.to_string().contains("s.911A"));

        let error = AfsLicensingError::ConditionBreach {
            condition_id: "COND-1".to_string(),
            description: "Training not completed".to_string(),
        };
        assert!(error.to_string().contains("s.912A"));
    }
}
