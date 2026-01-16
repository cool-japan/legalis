//! Canada Corporate Law - Error Types
//!
//! Error handling for Canadian corporate law operations.

use std::fmt;

/// Corporate law error
#[derive(Debug, Clone)]
pub enum CorporateError {
    /// Invalid corporate type
    InvalidCorporateType(String),
    /// Invalid incorporation jurisdiction
    InvalidJurisdiction(String),
    /// Director duty analysis error
    DirectorDutyError(String),
    /// Oppression analysis error
    OppressionError(String),
    /// Share structure error
    ShareStructureError(String),
    /// Fundamental change error
    FundamentalChangeError(String),
    /// Complainant status error
    ComplainantStatusError(String),
    /// Province not supported
    ProvinceNotSupported(String),
    /// Missing required information
    MissingInformation(String),
    /// General corporate error
    General(String),
}

impl fmt::Display for CorporateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCorporateType(msg) => write!(f, "Invalid corporate type: {}", msg),
            Self::InvalidJurisdiction(msg) => write!(f, "Invalid jurisdiction: {}", msg),
            Self::DirectorDutyError(msg) => write!(f, "Director duty error: {}", msg),
            Self::OppressionError(msg) => write!(f, "Oppression error: {}", msg),
            Self::ShareStructureError(msg) => write!(f, "Share structure error: {}", msg),
            Self::FundamentalChangeError(msg) => write!(f, "Fundamental change error: {}", msg),
            Self::ComplainantStatusError(msg) => write!(f, "Complainant status error: {}", msg),
            Self::ProvinceNotSupported(msg) => write!(f, "Province not supported: {}", msg),
            Self::MissingInformation(msg) => write!(f, "Missing information: {}", msg),
            Self::General(msg) => write!(f, "Corporate error: {}", msg),
        }
    }
}

impl std::error::Error for CorporateError {}

/// Result type for corporate operations
pub type CorporateResult<T> = Result<T, CorporateError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corporate_error_display() {
        let error = CorporateError::InvalidCorporateType("unknown type".to_string());
        assert!(error.to_string().contains("Invalid corporate type"));
    }

    #[test]
    fn test_director_duty_error() {
        let error = CorporateError::DirectorDutyError("breach identified".to_string());
        assert!(error.to_string().contains("Director duty"));
    }

    #[test]
    fn test_oppression_error() {
        let error = CorporateError::OppressionError("no reasonable expectations".to_string());
        assert!(error.to_string().contains("Oppression error"));
    }
}
