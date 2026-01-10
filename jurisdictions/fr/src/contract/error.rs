//! Contract law error types (Types d'erreurs de droit des contrats)
//!
//! Error types for French contract law validation and operations.

use thiserror::Error;

use super::types::ValidityDefect;

/// Contract law error (Erreur de droit des contrats)
///
/// Errors that can occur when validating or processing contracts under French law.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ContractError {
    /// No consent given (Absence de consentement)
    ///
    /// Article 1128 requires consent of the parties.
    #[error("Absence de consentement / No consent given (Article 1128)")]
    NoConsent,

    /// Insufficient parties (Nombre de parties insuffisant)
    ///
    /// A contract requires at least 2 parties.
    #[error("Le contrat doit avoir au moins 2 parties / Contract must have at least 2 parties")]
    InsufficientParties,

    /// Validity defect (Vice du consentement)
    ///
    /// Consent was vitiated by error, fraud, or duress (Articles 1130-1171).
    #[error("Vice du consentement détecté / Validity defect detected: {0:?} (Articles 1130-1171)")]
    ValidityDefect(Vec<ValidityDefect>),

    /// No contract type specified
    #[error("Type de contrat non spécifié / No contract type specified")]
    NoContractType,

    /// Unlawful content (Contenu illicite)
    ///
    /// Article 1128 requires lawful and certain content.
    #[error("Contenu illicite ou incertain / Unlawful or uncertain content (Article 1128)")]
    UnlawfulContent { description: String },

    /// Lack of capacity (Incapacité)
    ///
    /// Article 1128 requires capacity to contract.
    #[error("Incapacité de contracter / Lack of capacity to contract (Article 1128): {party}")]
    LackOfCapacity { party: String },

    /// No breach specified
    #[error("Aucune inexécution spécifiée / No breach specified")]
    NoBreach,

    /// Cannot calculate damages without breach
    #[error(
        "Impossible de calculer les dommages sans inexécution / Cannot calculate damages without breach"
    )]
    CannotCalculateDamages,

    /// Insufficient information for damages calculation
    #[error(
        "Informations insuffisantes pour calculer les dommages / Insufficient information for damages calculation: {missing}"
    )]
    InsufficientDamageInfo { missing: String },

    /// Good faith violation (Violation de la bonne foi)
    ///
    /// Article 1104 requires contracts to be negotiated, formed, and performed in good faith.
    #[error("Violation du principe de bonne foi / Good faith violation (Article 1104)")]
    BadFaith,

    /// Multiple validation errors
    #[error("Erreurs multiples de validation / Multiple validation errors: {0:?}")]
    MultipleErrors(Vec<ContractError>),
}

/// Validation result type for contract operations
pub type ValidationResult<T> = Result<T, ContractError>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::types::{DuressLevel, ValidityDefect};

    #[test]
    fn test_error_display() {
        let error = ContractError::NoConsent;
        let display = format!("{}", error);
        assert!(display.contains("consentement"));
        assert!(display.contains("Article 1128"));
    }

    #[test]
    fn test_validity_defect_error() {
        let defects = vec![ValidityDefect::Duress {
            severity: DuressLevel::Severe,
            description: "Menaces graves".to_string(),
        }];

        let error = ContractError::ValidityDefect(defects.clone());
        assert!(matches!(error, ContractError::ValidityDefect(_)));
    }

    #[test]
    fn test_multiple_errors() {
        let errors = vec![ContractError::NoConsent, ContractError::InsufficientParties];

        let error = ContractError::MultipleErrors(errors);
        assert!(matches!(error, ContractError::MultipleErrors(_)));
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(ContractError::NoConsent, ContractError::NoConsent);
        assert_ne!(ContractError::NoConsent, ContractError::NoBreach);
    }
}
