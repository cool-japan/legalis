//! Error types for legal reasoning.

use thiserror::Error;

/// Errors that can occur during legal reasoning
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ReasoningError {
    /// No applicable statutes found for the given entity type
    #[error("No applicable statutes found for entity type: {entity_type}")]
    NoApplicableStatutes {
        /// The entity type that has no applicable statutes
        entity_type: String,
    },

    /// Statute not found in the registry
    #[error("Statute not found: {statute_id}")]
    StatuteNotFound {
        /// The ID of the statute that was not found
        statute_id: String,
    },

    /// Condition evaluation failed
    #[error("Condition evaluation failed: {reason}")]
    ConditionEvaluationFailed {
        /// The reason for the failure
        reason: String,
    },

    /// Missing context data required for evaluation
    #[error("Missing context data: {description}")]
    MissingContextData {
        /// Description of the missing data
        description: String,
    },

    /// Invalid entity data provided
    #[error("Invalid entity data: {reason}")]
    InvalidEntityData {
        /// The reason for the invalid data
        reason: String,
    },

    /// Error in the statute registry
    #[error("Statute registry error: {message}")]
    RegistryError {
        /// The error message
        message: String,
    },

    /// Analysis confidence is too low
    #[error("Analysis confidence too low: {confidence:.2} < {threshold:.2}")]
    LowConfidence {
        /// The actual confidence level
        confidence: f64,
        /// The required threshold
        threshold: f64,
    },

    /// Multiple errors occurred during analysis
    #[error("Multiple errors occurred: {0:?}")]
    MultipleErrors(Vec<ReasoningError>),

    /// Custom error with a message
    #[error("{message_en}")]
    Custom {
        /// The error message in English
        message_en: String,
    },
}

impl ReasoningError {
    /// Creates a custom error with the given message
    #[must_use]
    pub fn custom(message_en: impl Into<String>) -> Self {
        Self::Custom {
            message_en: message_en.into(),
        }
    }

    /// Returns true if this is a critical error that should stop processing
    #[must_use]
    pub const fn is_critical(&self) -> bool {
        matches!(
            self,
            Self::NoApplicableStatutes { .. }
                | Self::RegistryError { .. }
                | Self::InvalidEntityData { .. }
        )
    }

    /// Returns true if this error is recoverable
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
