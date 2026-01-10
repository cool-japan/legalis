//! Error types for legal reasoning.

use thiserror::Error;

/// Errors that can occur during legal reasoning
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ReasoningError {
    /// No applicable statutes found for the entity
    #[error(
        "Aucune loi applicable trouvée / No applicable statutes found for entity type: {entity_type}"
    )]
    NoApplicableStatutes { entity_type: String },

    /// Statute not found in registry
    #[error("Loi non trouvée / Statute not found: {statute_id}")]
    StatuteNotFound { statute_id: String },

    /// Failed to evaluate condition
    #[error("Échec de l'évaluation de la condition / Condition evaluation failed: {reason}")]
    ConditionEvaluationFailed { reason: String },

    /// Missing required context data
    #[error("Données de contexte manquantes / Missing context data: {description}")]
    MissingContextData { description: String },

    /// Invalid entity data
    #[error("Données d'entité invalides / Invalid entity data: {reason}")]
    InvalidEntityData { reason: String },

    /// Statute registry error
    #[error("Erreur du registre des lois / Statute registry error: {message}")]
    RegistryError { message: String },

    /// JIT compilation error
    #[error("Erreur de compilation JIT / JIT compilation error: {message}")]
    JitCompilationError { message: String },

    /// Analysis confidence too low
    #[error(
        "Confiance d'analyse trop faible / Analysis confidence too low: {confidence:.2} < {threshold:.2}"
    )]
    LowConfidence { confidence: f64, threshold: f64 },

    /// Multiple errors occurred
    #[error("Erreurs multiples / Multiple errors occurred: {0:?}")]
    MultipleErrors(Vec<ReasoningError>),

    /// Custom error
    #[error("{message_fr} / {message_en}")]
    Custom {
        message_fr: String,
        message_en: String,
    },
}

impl ReasoningError {
    /// Create a custom error
    #[must_use]
    pub fn custom(message_fr: impl Into<String>, message_en: impl Into<String>) -> Self {
        Self::Custom {
            message_fr: message_fr.into(),
            message_en: message_en.into(),
        }
    }

    /// Check if this is a critical error that should halt analysis
    #[must_use]
    pub const fn is_critical(&self) -> bool {
        matches!(
            self,
            Self::NoApplicableStatutes { .. }
                | Self::RegistryError { .. }
                | Self::InvalidEntityData { .. }
        )
    }

    /// Check if this error is recoverable
    #[must_use]
    pub const fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::ConditionEvaluationFailed { .. }
                | Self::MissingContextData { .. }
                | Self::LowConfidence { .. }
        )
    }
}

/// Result type for reasoning operations
pub type ReasoningResult<T> = Result<T, ReasoningError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ReasoningError::StatuteNotFound {
            statute_id: "code-civil-1128".to_string(),
        };

        let display = format!("{}", err);
        assert!(display.contains("code-civil-1128"));
        assert!(display.contains("not found"));
    }

    #[test]
    fn test_no_applicable_statutes() {
        let err = ReasoningError::NoApplicableStatutes {
            entity_type: "Contract".to_string(),
        };

        assert!(err.is_critical());
        assert!(!err.is_recoverable());
    }

    #[test]
    fn test_condition_evaluation_failed() {
        let err = ReasoningError::ConditionEvaluationFailed {
            reason: "Missing attribute".to_string(),
        };

        assert!(!err.is_critical());
        assert!(err.is_recoverable());
    }

    #[test]
    fn test_custom_error() {
        let err = ReasoningError::custom("Erreur personnalisée", "Custom error");

        let display = format!("{}", err);
        assert!(display.contains("Erreur personnalisée"));
        assert!(display.contains("Custom error"));
    }

    #[test]
    fn test_low_confidence() {
        let err = ReasoningError::LowConfidence {
            confidence: 0.5,
            threshold: 0.7,
        };

        let display = format!("{}", err);
        assert!(display.contains("0.50"));
        assert!(display.contains("0.70"));
    }

    #[test]
    fn test_multiple_errors() {
        let errors = vec![
            ReasoningError::StatuteNotFound {
                statute_id: "art1".to_string(),
            },
            ReasoningError::StatuteNotFound {
                statute_id: "art2".to_string(),
            },
        ];

        let err = ReasoningError::MultipleErrors(errors.clone());

        match err {
            ReasoningError::MultipleErrors(errs) => {
                assert_eq!(errs.len(), 2);
            }
            _ => panic!("Expected MultipleErrors"),
        }
    }

    #[test]
    fn test_error_equality() {
        let err1 = ReasoningError::StatuteNotFound {
            statute_id: "code-civil-1128".to_string(),
        };
        let err2 = ReasoningError::StatuteNotFound {
            statute_id: "code-civil-1128".to_string(),
        };
        let err3 = ReasoningError::StatuteNotFound {
            statute_id: "code-civil-1231".to_string(),
        };

        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_registry_error() {
        let err = ReasoningError::RegistryError {
            message: "Failed to initialize".to_string(),
        };

        assert!(err.is_critical());
    }

    #[test]
    fn test_jit_compilation_error() {
        let err = ReasoningError::JitCompilationError {
            message: "Backend not available".to_string(),
        };

        let display = format!("{}", err);
        assert!(display.contains("Backend not available"));
    }
}
