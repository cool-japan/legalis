//! Error types for US legal reasoning.

use std::error::Error;
use std::fmt;

/// Errors that can occur during legal reasoning
#[derive(Debug, Clone)]
pub enum ReasoningError {
    /// No applicable statute found for the context
    NoApplicableStatutes {
        /// Description of the entity being evaluated
        entity_type: String,
    },
    /// Statute not found in registry
    StatuteNotFound {
        /// The ID of the statute that was not found
        statute_id: String,
    },
    /// Condition evaluation failed
    ConditionEvaluationFailed {
        /// The condition that failed
        condition: String,
        /// Reason for the failure
        reason: String,
    },
    /// Missing context data required for evaluation
    MissingContextData {
        /// Description of the missing data
        description: String,
    },
    /// Invalid entity data provided
    InvalidEntityData {
        /// The reason for the invalid data
        reason: String,
    },
    /// Error in the statute registry
    RegistryError {
        /// The error message
        message: String,
    },
}

impl fmt::Display for ReasoningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoApplicableStatutes { entity_type } => {
                write!(f, "no applicable statutes found for entity: {entity_type}")
            }
            Self::StatuteNotFound { statute_id } => {
                write!(f, "statute not found: {statute_id}")
            }
            Self::ConditionEvaluationFailed { condition, reason } => {
                write!(f, "failed to evaluate condition '{condition}': {reason}")
            }
            Self::MissingContextData { description } => {
                write!(f, "missing context data: {description}")
            }
            Self::InvalidEntityData { reason } => {
                write!(f, "invalid entity data: {reason}")
            }
            Self::RegistryError { message } => {
                write!(f, "statute registry error: {message}")
            }
        }
    }
}

impl Error for ReasoningError {}

/// Result type for reasoning operations
pub type ReasoningResult<T> = Result<T, ReasoningError>;
