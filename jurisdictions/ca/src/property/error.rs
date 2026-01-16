//! Canada Property Law - Error Types
//!
//! Error handling for Canadian property law operations.

use std::fmt;

/// Property law error
#[derive(Debug, Clone)]
pub enum PropertyError {
    /// Invalid property type
    InvalidPropertyType(String),
    /// Invalid estate type
    InvalidEstateType(String),
    /// Title registration error
    TitleRegistrationError(String),
    /// Aboriginal title analysis error
    AboriginalTitleError(String),
    /// Consultation analysis error
    ConsultationError(String),
    /// Easement analysis error
    EasementError(String),
    /// Conveyancing error
    ConveyancingError(String),
    /// Province not supported
    ProvinceNotSupported(String),
    /// Missing required information
    MissingInformation(String),
    /// General property error
    General(String),
}

impl fmt::Display for PropertyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPropertyType(msg) => write!(f, "Invalid property type: {}", msg),
            Self::InvalidEstateType(msg) => write!(f, "Invalid estate type: {}", msg),
            Self::TitleRegistrationError(msg) => write!(f, "Title registration error: {}", msg),
            Self::AboriginalTitleError(msg) => write!(f, "Aboriginal title error: {}", msg),
            Self::ConsultationError(msg) => write!(f, "Consultation error: {}", msg),
            Self::EasementError(msg) => write!(f, "Easement error: {}", msg),
            Self::ConveyancingError(msg) => write!(f, "Conveyancing error: {}", msg),
            Self::ProvinceNotSupported(msg) => write!(f, "Province not supported: {}", msg),
            Self::MissingInformation(msg) => write!(f, "Missing information: {}", msg),
            Self::General(msg) => write!(f, "Property error: {}", msg),
        }
    }
}

impl std::error::Error for PropertyError {}

/// Result type for property operations
pub type PropertyResult<T> = Result<T, PropertyError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_error_display() {
        let error = PropertyError::InvalidPropertyType("unknown type".to_string());
        assert!(error.to_string().contains("Invalid property type"));
    }

    #[test]
    fn test_aboriginal_title_error() {
        let error = PropertyError::AboriginalTitleError("insufficient evidence".to_string());
        assert!(error.to_string().contains("Aboriginal title"));
    }

    #[test]
    fn test_consultation_error() {
        let error = PropertyError::ConsultationError("inadequate consultation".to_string());
        assert!(error.to_string().contains("Consultation error"));
    }
}
