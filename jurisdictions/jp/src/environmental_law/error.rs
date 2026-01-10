//! Environmental Law Error Types
//!
//! Error types for Air Pollution Control Act (大気汚染防止法),
//! Water Pollution Prevention Act (水質汚濁防止法),
//! and Waste Management Act (廃棄物処理法) operations.

use thiserror::Error;

/// Environmental law errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum EnvironmentalError {
    /// Emission exceeds legal limit
    #[error("Emission exceeds legal limit: {pollutant} {actual} > {limit} {unit} ({legal_basis})")]
    EmissionExceedsLimit {
        pollutant: String,
        actual: f64,
        limit: f64,
        unit: String,
        legal_basis: String,
    },

    /// Missing pollution control equipment
    #[error("Missing pollution control equipment (大気汚染防止法第3条)")]
    MissingPollutionControl,

    /// Facility standards not met
    #[error("Facility standards not met (廃棄物処理法第8条)")]
    FacilityStandardsNotMet,

    /// Invalid waste manifest
    #[error("Invalid waste manifest: {reason}")]
    InvalidManifest { reason: String },

    /// Permit expired
    #[error("Permit expired: {expiration_date}")]
    PermitExpired { expiration_date: String },

    /// Unauthorized waste type
    #[error("Unauthorized waste type: {0}")]
    UnauthorizedWasteType(String),

    /// Missing required field
    #[error("Missing required field: {field}")]
    MissingRequiredField { field: String },

    /// Monitoring requirements not met
    #[error("Monitoring requirements not met: {0}")]
    MonitoringNotMet(String),

    /// Prior notification not filed
    #[error("Prior notification not filed ({days} days required)")]
    PriorNotificationNotFiled { days: u32 },

    /// Processing capacity exceeded
    #[error("Processing capacity exceeded: {actual} > {limit} tons/day")]
    ProcessingCapacityExceeded { actual: f64, limit: f64 },

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Filing error
    #[error("Filing error: {0}")]
    FilingError(String),

    /// Other error
    #[error("{0}")]
    Other(String),
}

/// Result type for environmental law operations
pub type Result<T> = std::result::Result<T, EnvironmentalError>;

impl EnvironmentalError {
    /// Check if error is related to pollution control
    pub fn is_pollution_error(&self) -> bool {
        matches!(
            self,
            Self::EmissionExceedsLimit { .. }
                | Self::MissingPollutionControl
                | Self::MonitoringNotMet(_)
                | Self::PriorNotificationNotFiled { .. }
        )
    }

    /// Check if error is related to waste management
    pub fn is_waste_error(&self) -> bool {
        matches!(
            self,
            Self::FacilityStandardsNotMet
                | Self::InvalidManifest { .. }
                | Self::UnauthorizedWasteType(_)
                | Self::ProcessingCapacityExceeded { .. }
        )
    }

    /// Check if error is related to validation
    pub fn is_validation_error(&self) -> bool {
        matches!(
            self,
            Self::MissingRequiredField { .. } | Self::Validation(_)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = EnvironmentalError::EmissionExceedsLimit {
            pollutant: "SOx".to_string(),
            actual: 150.0,
            limit: 100.0,
            unit: "ppm".to_string(),
            legal_basis: "大気汚染防止法第3条".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("SOx"));
        assert!(msg.contains("150"));
        assert!(msg.contains("大気汚染防止法"));
    }

    #[test]
    fn test_pollution_error_check() {
        let err = EnvironmentalError::MissingPollutionControl;
        assert!(err.is_pollution_error());
        assert!(!err.is_waste_error());
    }

    #[test]
    fn test_waste_error_check() {
        let err = EnvironmentalError::FacilityStandardsNotMet;
        assert!(err.is_waste_error());
        assert!(!err.is_pollution_error());
    }

    #[test]
    fn test_validation_error_check() {
        let err = EnvironmentalError::MissingRequiredField {
            field: "facility_name".to_string(),
        };
        assert!(err.is_validation_error());
    }

    #[test]
    fn test_manifest_error() {
        let err = EnvironmentalError::InvalidManifest {
            reason: "Missing generator information".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("manifest"));
        assert!(msg.contains("Missing generator"));
    }
}
