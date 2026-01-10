//! Labor law error types (Types d'erreurs de droit du travail)
//!
//! Error types for French labor law validation and operations.

use thiserror::Error;

use super::types::{CDDReason, TrialPeriodCategory};

/// Labor law error (Erreur de droit du travail)
///
/// Errors that can occur when validating or processing employment matters under French law.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum LaborLawError {
    /// CDD duration exceeded (Durée du CDD dépassée)
    ///
    /// CDD cannot exceed 18 months (Article L1242-8).
    #[error(
        "Durée du CDD dépassée / CDD duration exceeded: max 18 months, got {months} months (Article L1242-8)"
    )]
    CDDDurationExceeded { months: u8 },

    /// Invalid CDD reason (Motif de recours au CDD invalide)
    ///
    /// CDD can only be used for specific authorized reasons (Article L1242-2).
    #[error(
        "Motif de recours au CDD invalide / Invalid CDD reason: {reason:?} not authorized (Article L1242-2)"
    )]
    InvalidCDDReason { reason: CDDReason },

    /// CDD must be written (CDD doit être écrit)
    ///
    /// CDD requires a written contract (Article L1242-12).
    #[error("CDD doit être écrit / CDD must be in writing (Article L1242-12)")]
    CDDNotWritten,

    /// Trial period too long (Période d'essai trop longue)
    ///
    /// Trial period exceeds maximum for category (Article L1221-19).
    #[error(
        "Période d'essai trop longue / Trial period too long: max {max} months for {category:?}, got {actual} months (Article L1221-19)"
    )]
    TrialPeriodTooLong {
        category: TrialPeriodCategory,
        max: u8,
        actual: u8,
    },

    /// Weekly hours exceeded (Durée maximale dépassée)
    ///
    /// Weekly hours exceed legal maximum (Article L3121-20).
    #[error(
        "Durée maximale hebdomadaire dépassée / Weekly hours exceeded: max {max} hours, got {actual} hours (Article L3121-20)"
    )]
    WeeklyHoursExceeded { max: f32, actual: f32 },

    /// Daily hours exceeded (Durée quotidienne maximale dépassée)
    ///
    /// Daily hours exceed legal maximum (Article L3121-18).
    #[error(
        "Durée quotidienne maximale dépassée / Daily hours exceeded: max {max} hours, got {actual} hours (Article L3121-18)"
    )]
    DailyHoursExceeded { max: f32, actual: f32 },

    /// Missing real and serious cause (Absence de cause réelle et sérieuse)
    ///
    /// Dismissal lacks real and serious cause (Article L1232-1).
    #[error(
        "Absence de cause réelle et sérieuse / Missing real and serious cause for dismissal (Article L1232-1)"
    )]
    MissingRealSeriousCause,

    /// Economic dismissal invalid (Licenciement économique invalide)
    ///
    /// Economic dismissal does not meet requirements (Article L1233-3).
    #[error(
        "Licenciement économique invalide / Invalid economic dismissal: must have economic difficulties AND job elimination (Article L1233-3)"
    )]
    InvalidEconomicDismissal,

    /// Missing dismissal interview (Absence d'entretien préalable)
    ///
    /// Employer must hold pre-dismissal interview (Article L1232-2).
    #[error("Absence d'entretien préalable / Missing pre-dismissal interview (Article L1232-2)")]
    MissingDismissalInterview,

    /// Insufficient notice period (Préavis insuffisant)
    ///
    /// Notice period does not meet minimum requirements (Article L1234-1).
    #[error(
        "Préavis insuffisant / Insufficient notice period: required {required} months, got {actual} months (Article L1234-1)"
    )]
    InsufficientNotice { required: u8, actual: u8 },

    /// Below minimum wage (Salaire inférieur au SMIC)
    ///
    /// Hourly rate below minimum wage (SMIC).
    #[error("Salaire inférieur au SMIC / Below minimum wage: SMIC €{smic}, got €{actual}")]
    BelowMinimumWage { smic: f32, actual: f32 },

    /// Multiple validation errors
    #[error("Erreurs multiples de validation / Multiple validation errors: {0:?}")]
    MultipleErrors(Vec<LaborLawError>),
}

/// Validation result type for labor operations
pub type ValidationResult<T> = Result<T, LaborLawError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdd_duration_exceeded_error() {
        let error = LaborLawError::CDDDurationExceeded { months: 24 };
        let display = format!("{}", error);
        assert!(display.contains("18 months"));
        assert!(display.contains("24 months"));
        assert!(display.contains("L1242-8"));
    }

    #[test]
    fn test_trial_period_too_long_error() {
        let error = LaborLawError::TrialPeriodTooLong {
            category: TrialPeriodCategory::WorkersEmployees,
            max: 2,
            actual: 4,
        };
        let display = format!("{}", error);
        assert!(display.contains("2 months"));
        assert!(display.contains("4 months"));
    }

    #[test]
    fn test_weekly_hours_exceeded_error() {
        let error = LaborLawError::WeeklyHoursExceeded {
            max: 48.0,
            actual: 55.0,
        };
        assert!(format!("{}", error).contains("48"));
    }

    #[test]
    fn test_error_equality() {
        let err1 = LaborLawError::CDDNotWritten;
        let err2 = LaborLawError::CDDNotWritten;
        assert_eq!(err1, err2);

        let err3 = LaborLawError::MissingDismissalInterview;
        assert_ne!(err1, err3);
    }
}
