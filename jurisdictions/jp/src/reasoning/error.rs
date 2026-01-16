//! Error types for legal reasoning (法的推論エラー型).

use thiserror::Error;

/// Errors that can occur during legal reasoning
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ReasoningError {
    #[error(
        "該当する法令が見つかりません / No applicable statutes found for entity type: {entity_type}"
    )]
    NoApplicableStatutes { entity_type: String },

    #[error("法令が見つかりません / Statute not found: {statute_id}")]
    StatuteNotFound { statute_id: String },

    #[error("条件評価に失敗 / Condition evaluation failed: {reason}")]
    ConditionEvaluationFailed { reason: String },

    #[error("コンテキストデータが不足 / Missing context data: {description}")]
    MissingContextData { description: String },

    #[error("無効なエンティティデータ / Invalid entity data: {reason}")]
    InvalidEntityData { reason: String },

    #[error("法令レジストリエラー / Statute registry error: {message}")]
    RegistryError { message: String },

    #[error("分析信頼度が低い / Analysis confidence too low: {confidence:.2} < {threshold:.2}")]
    LowConfidence { confidence: f64, threshold: f64 },

    #[error("複数のエラーが発生 / Multiple errors occurred: {0:?}")]
    MultipleErrors(Vec<ReasoningError>),

    #[error("{message_jp} / {message_en}")]
    Custom {
        message_jp: String,
        message_en: String,
    },
}

impl ReasoningError {
    #[must_use]
    pub fn custom(message_jp: impl Into<String>, message_en: impl Into<String>) -> Self {
        Self::Custom {
            message_jp: message_jp.into(),
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
