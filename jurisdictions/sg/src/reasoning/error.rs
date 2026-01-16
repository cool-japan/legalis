//! Error types for legal reasoning.

use thiserror::Error;

/// Errors that can occur during legal reasoning
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ReasoningError {
    /// No applicable statutes found for the entity
    #[error("No applicable statutes found for entity type: {entity_type}")]
    NoApplicableStatutes { entity_type: String },

    /// Statute not found in registry
    #[error("Statute not found: {statute_id}")]
    StatuteNotFound { statute_id: String },

    /// Failed to evaluate condition
    #[error("Condition evaluation failed: {reason}")]
    ConditionEvaluationFailed { reason: String },

    /// Missing required context data
    #[error("Missing context data: {description}")]
    MissingContextData { description: String },

    /// Invalid entity data
    #[error("Invalid entity data: {reason}")]
    InvalidEntityData { reason: String },

    /// Statute registry error
    #[error("Statute registry error: {message}")]
    RegistryError { message: String },

    /// Analysis confidence too low
    #[error("Analysis confidence too low: {confidence:.2} < {threshold:.2}")]
    LowConfidence { confidence: f64, threshold: f64 },

    /// Multiple errors occurred
    #[error("Multiple errors occurred: {0:?}")]
    MultipleErrors(Vec<ReasoningError>),

    /// Custom error
    #[error("{message_en}")]
    Custom { message_en: String },
}

impl ReasoningError {
    /// Create a custom error
    #[must_use]
    pub fn custom(message_en: impl Into<String>) -> Self {
        Self::Custom {
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
            statute_id: "EA_s38".to_string(),
        };

        let display = format!("{}", err);
        assert!(display.contains("EA_s38"));
        assert!(display.contains("not found"));
    }

    #[test]
    fn test_no_applicable_statutes() {
        let err = ReasoningError::NoApplicableStatutes {
            entity_type: "EmploymentContract".to_string(),
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
        let err = ReasoningError::custom("Custom error message");

        let display = format!("{}", err);
        assert!(display.contains("Custom error message"));
    }
}
