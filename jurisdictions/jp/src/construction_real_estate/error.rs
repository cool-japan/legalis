//! Construction and Real Estate Error Types
//!
//! Error types for Construction Business Act (建設業法) and
//! Real Estate Transactions Act (宅地建物取引業法) operations.

use thiserror::Error;

/// Construction and Real Estate errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ConstructionRealEstateError {
    /// Insufficient capital for license type
    #[error("Insufficient capital: ¥{actual} < ¥{required} (建設業法第7条)")]
    InsufficientCapital { actual: u64, required: u64 },

    /// No qualified manager for construction type
    #[error("No qualified manager for construction type: {construction_type} (建設業法第8条)")]
    NoQualifiedManager { construction_type: String },

    /// License expired
    #[error("License expired: {expiration_date}")]
    LicenseExpired { expiration_date: String },

    /// Invalid license type for operation
    #[error("Invalid license type for construction type")]
    InvalidLicenseType,

    /// Important matters not explained (Real Estate Act Article 35)
    #[error("Important matters not explained (宅地建物取引業法第35条)")]
    ImportantMattersNotExplained,

    /// Commission exceeds legal limit (Real Estate Act Article 46)
    #[error("Commission exceeds legal limit: ¥{actual} > ¥{max} (宅地建物取引業法第46条)")]
    CommissionExceedsLimit { actual: u64, max: u64 },

    /// No licensed agent present
    #[error("No licensed agent present (宅地建物取引士が不在)")]
    NoLicensedAgent,

    /// Missing required field
    #[error("Missing required field: {field}")]
    MissingRequiredField { field: String },

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

/// Result type for construction and real estate operations
pub type Result<T> = std::result::Result<T, ConstructionRealEstateError>;

impl ConstructionRealEstateError {
    /// Check if error is related to construction business
    pub fn is_construction_error(&self) -> bool {
        matches!(
            self,
            Self::InsufficientCapital { .. }
                | Self::NoQualifiedManager { .. }
                | Self::InvalidLicenseType
        )
    }

    /// Check if error is related to real estate
    pub fn is_real_estate_error(&self) -> bool {
        matches!(
            self,
            Self::ImportantMattersNotExplained
                | Self::CommissionExceedsLimit { .. }
                | Self::NoLicensedAgent
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
        let err = ConstructionRealEstateError::InsufficientCapital {
            actual: 10_000_000,
            required: 20_000_000,
        };
        let msg = err.to_string();
        assert!(msg.contains("10000000"));
        assert!(msg.contains("建設業法"));
    }

    #[test]
    fn test_construction_error_check() {
        let err = ConstructionRealEstateError::NoQualifiedManager {
            construction_type: "Architecture".to_string(),
        };
        assert!(err.is_construction_error());
        assert!(!err.is_real_estate_error());
    }

    #[test]
    fn test_real_estate_error_check() {
        let err = ConstructionRealEstateError::ImportantMattersNotExplained;
        assert!(err.is_real_estate_error());
        assert!(!err.is_construction_error());
    }

    #[test]
    fn test_validation_error_check() {
        let err = ConstructionRealEstateError::MissingRequiredField {
            field: "business_name".to_string(),
        };
        assert!(err.is_validation_error());
    }
}
