//! Error types for EU legal reasoning.

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
    /// Failed to evaluate a condition
    ConditionEvaluationFailed {
        /// The condition that failed
        condition: String,
        /// Reason for the failure
        reason: String,
    },
    /// Invalid GDPR configuration
    InvalidGdprConfiguration {
        /// Description of the invalid configuration
        description: String,
    },
    /// Competition law analysis error
    CompetitionAnalysisError {
        /// Description of the error
        description: String,
    },
    /// Consumer rights analysis error
    ConsumerRightsError {
        /// Description of the error
        description: String,
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
            Self::InvalidGdprConfiguration { description } => {
                write!(f, "invalid GDPR configuration: {description}")
            }
            Self::CompetitionAnalysisError { description } => {
                write!(f, "competition law analysis error: {description}")
            }
            Self::ConsumerRightsError { description } => {
                write!(f, "consumer rights error: {description}")
            }
        }
    }
}

impl Error for ReasoningError {}

/// Result type for reasoning operations
pub type ReasoningResult<T> = Result<T, ReasoningError>;
