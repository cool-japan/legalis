//! Error types for French property law
//!
//! This module provides comprehensive error handling for property law
//! violations with bilingual (French/English) support.

use thiserror::Error;

/// Bilingual string for error messages
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BilingualString {
    pub fr: String,
    pub en: String,
}

impl BilingualString {
    /// Creates a new bilingual string
    pub fn new(fr: impl Into<String>, en: impl Into<String>) -> Self {
        Self {
            fr: fr.into(),
            en: en.into(),
        }
    }
}

/// Result type for property law operations
pub type PropertyLawResult<T> = Result<T, PropertyLawError>;

/// Errors that can occur in property law operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum PropertyLawError {
    /// Invalid property type
    #[error("Invalid property type: {property_type}")]
    InvalidPropertyType { property_type: String },

    /// Property ownership violation (Article 544)
    #[error("Ownership rights violated: {reason}")]
    OwnershipViolation { reason: String },

    /// Easement not legally established
    #[error("Invalid easement: {reason}")]
    InvalidEasement { reason: String },

    /// Landlocked property without legal access (Article 682)
    #[error("Landlocked property requires legal right of way")]
    LandlockedPropertyWithoutAccess,

    /// Easement conflicts with property rights
    #[error("Easement conflict: {conflict}")]
    EasementConflict { conflict: String },

    /// Invalid encumbrance
    #[error("Invalid encumbrance: {reason}")]
    InvalidEncumbrance { reason: String },

    /// Property value error
    #[error("Invalid property value: {value}")]
    InvalidPropertyValue { value: u64 },

    /// Missing required formality (Article 1873-1878)
    #[error("Missing required formality: {formality}")]
    MissingFormality { formality: String },

    /// Invalid real estate transaction
    #[error("Invalid transaction: {reason}")]
    InvalidTransaction { reason: String },

    /// Property classification error (Article 490)
    #[error("Property classification error: {reason}")]
    ClassificationError { reason: String },

    /// Multiple errors occurred
    #[error("Multiple errors: {0:?}")]
    MultipleErrors(Vec<PropertyLawError>),
}

impl PropertyLawError {
    /// Returns a bilingual description of the error
    pub fn description(&self) -> BilingualString {
        match self {
            Self::InvalidPropertyType { property_type } => BilingualString::new(
                format!("Type de bien invalide : {}", property_type),
                format!("Invalid property type: {}", property_type),
            ),

            Self::OwnershipViolation { reason } => BilingualString::new(
                format!("Violation du droit de propriété (Article 544) : {}", reason),
                format!("Ownership rights violated (Article 544): {}", reason),
            ),

            Self::InvalidEasement { reason } => BilingualString::new(
                format!("Servitude invalide : {}", reason),
                format!("Invalid easement: {}", reason),
            ),

            Self::LandlockedPropertyWithoutAccess => BilingualString::new(
                "Le fonds enclavé nécessite une servitude de passage forcé (Article 682)",
                "Landlocked property requires legal right of way (Article 682)",
            ),

            Self::EasementConflict { conflict } => BilingualString::new(
                format!("Conflit de servitudes : {}", conflict),
                format!("Easement conflict: {}", conflict),
            ),

            Self::InvalidEncumbrance { reason } => BilingualString::new(
                format!("Charge invalide : {}", reason),
                format!("Invalid encumbrance: {}", reason),
            ),

            Self::InvalidPropertyValue { value } => BilingualString::new(
                format!("Valeur de bien invalide : {} (doit être > 0)", value),
                format!("Invalid property value: {} (must be > 0)", value),
            ),

            Self::MissingFormality { formality } => BilingualString::new(
                format!(
                    "Formalité requise manquante (Article 1873-1878) : {}",
                    formality
                ),
                format!(
                    "Missing required formality (Article 1873-1878): {}",
                    formality
                ),
            ),

            Self::InvalidTransaction { reason } => BilingualString::new(
                format!("Transaction immobilière invalide : {}", reason),
                format!("Invalid real estate transaction: {}", reason),
            ),

            Self::ClassificationError { reason } => BilingualString::new(
                format!("Erreur de classification (Article 490) : {}", reason),
                format!("Property classification error (Article 490): {}", reason),
            ),

            Self::MultipleErrors(errors) => {
                let fr_msgs: Vec<String> = errors.iter().map(|e| e.description().fr).collect();
                let en_msgs: Vec<String> = errors.iter().map(|e| e.description().en).collect();

                BilingualString::new(
                    format!("Erreurs multiples : {}", fr_msgs.join("; ")),
                    format!("Multiple errors: {}", en_msgs.join("; ")),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_property_type_error() {
        let error = PropertyLawError::InvalidPropertyType {
            property_type: "Unknown".to_string(),
        };
        let desc = error.description();
        assert!(desc.fr.contains("invalide"));
        assert!(desc.en.contains("Invalid"));
    }

    #[test]
    fn test_ownership_violation_error() {
        let error = PropertyLawError::OwnershipViolation {
            reason: "Unauthorized use".to_string(),
        };
        let desc = error.description();
        assert!(desc.fr.contains("Article 544"));
        assert!(desc.en.contains("Article 544"));
        assert!(desc.fr.contains("propriété"));
        assert!(desc.en.contains("Ownership"));
    }

    #[test]
    fn test_landlocked_property_error() {
        let error = PropertyLawError::LandlockedPropertyWithoutAccess;
        let desc = error.description();
        assert!(desc.fr.contains("enclavé"));
        assert!(desc.en.contains("Landlocked"));
        assert!(desc.fr.contains("Article 682"));
    }

    #[test]
    fn test_invalid_easement_error() {
        let error = PropertyLawError::InvalidEasement {
            reason: "No legal basis".to_string(),
        };
        let desc = error.description();
        assert!(desc.fr.contains("Servitude"));
        assert!(desc.en.contains("easement"));
    }

    #[test]
    fn test_missing_formality_error() {
        let error = PropertyLawError::MissingFormality {
            formality: "Notarial deed".to_string(),
        };
        let desc = error.description();
        assert!(desc.fr.contains("Formalité"));
        assert!(desc.en.contains("formality"));
        assert!(desc.fr.contains("Article 1873-1878"));
    }

    #[test]
    fn test_multiple_errors() {
        let errors = vec![
            PropertyLawError::InvalidPropertyValue { value: 0 },
            PropertyLawError::LandlockedPropertyWithoutAccess,
        ];
        let error = PropertyLawError::MultipleErrors(errors);
        let desc = error.description();
        assert!(desc.fr.contains("multiples"));
        assert!(desc.en.contains("Multiple"));
    }
}
