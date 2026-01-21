//! Banking Errors

use thiserror::Error;

/// Errors related to banking regulation
#[derive(Debug, Clone, Error, PartialEq)]
pub enum BankingError {
    /// Not authorized as ADI
    #[error(
        "'{entity_name}' is not authorized as an ADI. \
         See Banking Act 1959 s.9."
    )]
    NotAuthorized { entity_name: String },

    /// ADI authorization not current
    #[error(
        "ADI authorization for '{adi_name}' is {status}. \
         See Banking Act 1959 s.9A."
    )]
    AuthorizationNotCurrent { adi_name: String, status: String },

    /// Capital inadequacy
    #[error(
        "Capital inadequacy: {ratio_type} ratio {actual}% is below minimum {required}%. \
         See APRA Prudential Standard APS 110."
    )]
    CapitalInadequacy {
        ratio_type: String,
        actual: f64,
        required: f64,
    },

    /// Liquidity inadequacy
    #[error(
        "Liquidity inadequacy: {ratio_type} {actual}% is below minimum 100%. \
         See APRA Prudential Standard APS 210."
    )]
    LiquidityInadequacy { ratio_type: String, actual: f64 },

    /// Prudential standard breach
    #[error("Breach of {standard}: {details}.")]
    PrudentialStandardBreach { standard: String, details: String },

    /// Validation error
    #[error("Banking validation error: {message}")]
    ValidationError { message: String },
}

/// Result type
pub type Result<T> = std::result::Result<T, BankingError>;
