//! Error types for legal reasoning (Fehlertypen für rechtliche Analyse).

use thiserror::Error;

/// Errors that can occur during legal reasoning
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ReasoningError {
    #[error(
        "Keine anwendbaren Gesetze gefunden / No applicable statutes found for entity type: {entity_type}"
    )]
    NoApplicableStatutes { entity_type: String },

    #[error("Gesetz nicht gefunden / Statute not found: {statute_id}")]
    StatuteNotFound { statute_id: String },

    #[error("Bedingungsbewertung fehlgeschlagen / Condition evaluation failed: {reason}")]
    ConditionEvaluationFailed { reason: String },

    #[error("Fehlende Kontextdaten / Missing context data: {description}")]
    MissingContextData { description: String },

    #[error("Ungültige Entitätsdaten / Invalid entity data: {reason}")]
    InvalidEntityData { reason: String },

    #[error("Gesetzesregisterfehler / Statute registry error: {message}")]
    RegistryError { message: String },

    #[error(
        "Analysevertrauen zu niedrig / Analysis confidence too low: {confidence:.2} < {threshold:.2}"
    )]
    LowConfidence { confidence: f64, threshold: f64 },

    #[error("Mehrere Fehler aufgetreten / Multiple errors occurred: {0:?}")]
    MultipleErrors(Vec<ReasoningError>),

    #[error("{message_de} / {message_en}")]
    Custom {
        message_de: String,
        message_en: String,
    },
}

impl ReasoningError {
    #[must_use]
    pub fn custom(message_de: impl Into<String>, message_en: impl Into<String>) -> Self {
        Self::Custom {
            message_de: message_de.into(),
            message_en: message_en.into(),
        }
    }

    #[must_use]
    pub const fn is_critical(&self) -> bool {
        matches!(
            self,
            Self::NoApplicableStatutes { .. }
                | Self::RegistryError { .. }
                | Self::InvalidEntityData { .. }
        )
    }

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

pub type ReasoningResult<T> = Result<T, ReasoningError>;
