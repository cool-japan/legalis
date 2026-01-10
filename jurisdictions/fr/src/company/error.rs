//! Company law error types (Types d'erreurs de droit des sociétés)
//!
//! Error types for French company law validation and operations.

use thiserror::Error;

use super::types::CompanyType;

/// Company law error (Erreur de droit des sociétés)
///
/// Errors that can occur when validating or processing companies under French law.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum CompanyLawError {
    /// Insufficient capital (Capital insuffisant)
    ///
    /// The capital does not meet the minimum requirements for the company type.
    #[error(
        "Capital insuffisant / Insufficient capital: required €{required}, got €{actual} for {company_type:?} (Article L{article})"
    )]
    InsufficientCapital {
        company_type: CompanyType,
        required: u64,
        actual: u64,
        article: String,
    },

    /// Invalid company name (Dénomination sociale invalide)
    ///
    /// Company name must include the company type (SA, SARL, or SAS).
    #[error(
        "Dénomination sociale invalide / Invalid company name: must include '{suffix}' for {company_type:?}"
    )]
    InvalidCompanyName {
        company_type: CompanyType,
        suffix: String,
    },

    /// Too many partners (Trop d'associés)
    ///
    /// SARL cannot have more than 100 partners (Article L223-3).
    #[error(
        "Trop d'associés pour une SARL / Too many partners for SARL: max 100, got {count} (Article L223-3)"
    )]
    TooManyPartners { count: usize },

    /// No shareholders (Aucun associé)
    ///
    /// A company must have at least one shareholder.
    #[error("Aucun associé / No shareholders: company must have at least one shareholder")]
    NoShareholders,

    /// Missing business purpose (Objet social manquant)
    ///
    /// Articles of incorporation must specify the business purpose.
    #[error("Objet social manquant / Missing business purpose")]
    MissingBusinessPurpose,

    /// Missing head office (Siège social manquant)
    ///
    /// Articles of incorporation must specify the registered office.
    #[error("Siège social manquant / Missing head office")]
    MissingHeadOffice,

    /// Invalid board size (Conseil d'administration invalide)
    ///
    /// SA board must have 3-18 directors (Article L225-17).
    #[error(
        "Taille du conseil invalide / Invalid board size: required 3-18 directors, got {size} (Article L225-17)"
    )]
    InvalidBoardSize { size: usize },

    /// Director term too long (Mandat trop long)
    ///
    /// SA directors can serve max 6 years per term (Article L225-18).
    #[error(
        "Mandat trop long / Director term too long: max 6 years, got {years} years (Article L225-18)"
    )]
    DirectorTermTooLong { years: u8 },

    /// Invalid fiscal year end (Clôture d'exercice invalide)
    ///
    /// Fiscal year end must be 1-12 (January-December).
    #[error("Clôture d'exercice invalide / Invalid fiscal year end: must be 1-12, got {month}")]
    InvalidFiscalYearEnd { month: u8 },

    /// Duration too long (Durée trop longue)
    ///
    /// Company duration cannot exceed 99 years.
    #[error("Durée trop longue / Duration too long: max 99 years, got {years} years")]
    DurationTooLong { years: u8 },

    /// Quorum not met (Quorum non atteint)
    ///
    /// Shareholders meeting did not meet quorum requirements.
    #[error("Quorum non atteint / Quorum not met: required {required}%, got {actual}%")]
    QuorumNotMet { required: f64, actual: f64 },

    /// Resolution not approved (Résolution non adoptée)
    ///
    /// Resolution did not receive sufficient votes.
    #[error(
        "Résolution non adoptée / Resolution not approved: required {required}%, got {actual}%"
    )]
    ResolutionNotApproved { required: f64, actual: f64 },

    /// Multiple validation errors
    #[error("Erreurs multiples de validation / Multiple validation errors: {0:?}")]
    MultipleErrors(Vec<CompanyLawError>),
}

/// Validation result type for company operations
pub type ValidationResult<T> = Result<T, CompanyLawError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insufficient_capital_error() {
        let error = CompanyLawError::InsufficientCapital {
            company_type: CompanyType::SA,
            required: 37_000,
            actual: 30_000,
            article: "225-1".to_string(),
        };

        let display = format!("{}", error);
        assert!(display.contains("37000"));
        assert!(display.contains("30000"));
        assert!(display.contains("L225-1"));
    }

    #[test]
    fn test_invalid_company_name_error() {
        let error = CompanyLawError::InvalidCompanyName {
            company_type: CompanyType::SARL,
            suffix: "SARL".to_string(),
        };

        assert!(format!("{}", error).contains("SARL"));
    }

    #[test]
    fn test_too_many_partners_error() {
        let error = CompanyLawError::TooManyPartners { count: 150 };
        let display = format!("{}", error);
        assert!(display.contains("100"));
        assert!(display.contains("150"));
        assert!(display.contains("L223-3"));
    }

    #[test]
    fn test_invalid_board_size_error() {
        let error = CompanyLawError::InvalidBoardSize { size: 2 };
        assert!(format!("{}", error).contains("3-18"));
    }

    #[test]
    fn test_error_equality() {
        let err1 = CompanyLawError::NoShareholders;
        let err2 = CompanyLawError::NoShareholders;
        assert_eq!(err1, err2);

        let err3 = CompanyLawError::MissingBusinessPurpose;
        assert_ne!(err1, err3);
    }
}
