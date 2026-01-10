//! Error types for Competition Law compliance

use thiserror::Error;

/// Errors for Competition Law (Articles 101-102 TFEU) compliance validation
#[derive(Error, Debug, Clone, PartialEq)]
pub enum CompetitionError {
    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid market share value
    #[error("Invalid market share: {reason}")]
    InvalidMarketShare { reason: String },

    /// Invalid relevant market definition
    #[error("Invalid relevant market definition: {reason}")]
    InvalidRelevantMarket { reason: String },

    /// No appreciable effect on competition (de minimis)
    #[error("Agreement does not have appreciable effect on competition (de minimis rule applies)")]
    DeMinimis,

    /// Article 101(3) exemption criteria not met
    #[error("Article 101(3) exemption criteria not met: {reason}")]
    ExemptionNotMet { reason: String },

    /// No dominant position found
    #[error(
        "Undertaking does not hold dominant position: market share {market_share}% is below threshold"
    )]
    NoDominantPosition { market_share: f64 },

    /// Abuse not established
    #[error("Abuse of dominant position not established: {reason}")]
    AbuseNotEstablished { reason: String },

    /// Insufficient cross-border effect
    #[error("Insufficient effect on trade between Member States")]
    InsufficientCrossBorderEffect,

    /// Invalid value
    #[error("Invalid value for field '{field}': {reason}")]
    InvalidValue { field: String, reason: String },

    /// Multiple violations detected
    #[error("Multiple competition law violations: {0:?}")]
    MultipleViolations(Vec<String>),
}

impl CompetitionError {
    /// Create error for missing field
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField(field.into())
    }

    /// Create error for invalid market share
    pub fn invalid_market_share(reason: impl Into<String>) -> Self {
        Self::InvalidMarketShare {
            reason: reason.into(),
        }
    }

    /// Create error for invalid relevant market
    pub fn invalid_relevant_market(reason: impl Into<String>) -> Self {
        Self::InvalidRelevantMarket {
            reason: reason.into(),
        }
    }

    /// Create error for exemption not met
    pub fn exemption_not_met(reason: impl Into<String>) -> Self {
        Self::ExemptionNotMet {
            reason: reason.into(),
        }
    }

    /// Create error for no dominant position
    pub fn no_dominant_position(market_share: f64) -> Self {
        Self::NoDominantPosition { market_share }
    }

    /// Create error for abuse not established
    pub fn abuse_not_established(reason: impl Into<String>) -> Self {
        Self::AbuseNotEstablished {
            reason: reason.into(),
        }
    }

    /// Create error for invalid value
    pub fn invalid_value(field: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidValue {
            field: field.into(),
            reason: reason.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = CompetitionError::missing_field("undertaking");
        assert_eq!(err.to_string(), "Missing required field: undertaking");

        let err2 = CompetitionError::invalid_market_share("Must be between 0 and 1");
        assert!(err2.to_string().contains("Invalid market share"));
    }

    #[test]
    fn test_error_construction() {
        let err = CompetitionError::no_dominant_position(0.35);
        assert!(err.to_string().contains("35"));

        let err2 = CompetitionError::exemption_not_met("Does not benefit consumers");
        assert!(err2.to_string().contains("Does not benefit consumers"));
    }
}
