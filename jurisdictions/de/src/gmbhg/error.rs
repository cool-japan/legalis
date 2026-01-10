//! Error types for German company law validation
//!
//! Comprehensive error definitions with bilingual (German/English) messages
//! following the thiserror pattern.

use thiserror::Error;

/// Result type alias for GmbHG operations
pub type Result<T> = std::result::Result<T, GmbHError>;

/// Comprehensive error types for GmbH/UG formation and validation
///
/// All errors include bilingual messages (German primary, English secondary)
/// with relevant GmbHG article references.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum GmbHError {
    // ========================================================================
    // Capital Errors (Kapitalfehler)
    // ========================================================================
    /// Share capital below minimum requirement
    #[error(
        "Stammkapital €{actual_euros:.2} unterschreitet Mindestbetrag €{minimum_euros:.2} (§5 GmbHG)\n\
         Share capital €{actual_euros:.2} is below minimum €{minimum_euros:.2} (§5 GmbHG)"
    )]
    CapitalBelowMinimum {
        actual_euros: f64,
        minimum_euros: f64,
    },

    /// UG capital exceeds UG limit (must be GmbH at €25,000)
    #[error(
        "UG-Kapital €{actual_euros:.2} überschreitet UG-Grenze €24,999.99. Für €25,000+ ist eine reguläre GmbH erforderlich (§5a GmbHG).\n\
         UG capital €{actual_euros:.2} exceeds UG limit €24,999.99. For €25,000+ a regular GmbH is required (§5a GmbHG)."
    )]
    UGCapitalExceedsLimit { actual_euros: f64 },

    /// Initial contribution below minimum requirement (§7 Abs. 2 GmbHG)
    #[error(
        "Einlage €{paid_euros:.2} unterschreitet gesetzliche Mindesteinlage €{required_euros:.2} (§7 Abs. 2 GmbHG - mindestens 50% oder €12,500)\n\
         Contribution €{paid_euros:.2} is below required initial contribution €{required_euros:.2} (§7 para. 2 GmbHG - at least 50% or €12,500)"
    )]
    InitialContributionTooLow {
        paid_euros: f64,
        required_euros: f64,
    },

    /// Sum of share allocations does not match capital
    #[error(
        "Summe der Geschäftsanteile (€{total_shares:.2}) stimmt nicht mit Stammkapital (€{capital:.2}) überein\n\
         Sum of share allocations (€{total_shares:.2}) does not match share capital (€{capital:.2})"
    )]
    ShareAllocationMismatch { total_shares: f64, capital: f64 },

    /// Zero capital (invalid)
    #[error(
        "Stammkapital kann nicht null sein (§5 GmbHG)\n\
         Share capital cannot be zero (§5 GmbHG)"
    )]
    ZeroCapital,

    // ========================================================================
    // Articles of Association Errors (Gesellschaftsvertragsfehler)
    // ========================================================================
    /// Company name missing required legal form suffix
    #[error(
        "Firma '{name}' fehlt erforderlicher Rechtsformzusatz '{required_suffix}' (§4 GmbHG)\n\
         Company name '{name}' missing required legal form suffix '{required_suffix}' (§4 GmbHG)"
    )]
    MissingLegalFormSuffix {
        name: String,
        required_suffix: String,
    },

    /// Business purpose missing or too short
    #[error(
        "Unternehmensgegenstand fehlt oder ist zu kurz (§3 Abs. 1 Nr. 2 GmbHG)\n\
         Business purpose missing or too short (§3 para. 1 no. 2 GmbHG)"
    )]
    InvalidBusinessPurpose,

    /// Registered office is not a valid German city
    #[error(
        "Sitz '{city}' ist keine gültige deutsche Stadt (§4a GmbHG erfordert deutschen Sitz)\n\
         Registered office '{city}' is not a valid German city (§4a GmbHG requires German office)"
    )]
    InvalidRegisteredOffice { city: String },

    /// No shareholders specified
    #[error(
        "Keine Gesellschafter angegeben (mindestens 1 erforderlich für Gründung)\n\
         No shareholders specified (at least 1 required for formation)"
    )]
    NoShareholders,

    /// Fiscal year end date is invalid
    #[error(
        "Geschäftsjahresende ungültig: Monat {month}, Tag {day}\n\
         Fiscal year end invalid: month {month}, day {day}"
    )]
    InvalidFiscalYearEnd { month: u8, day: u8 },

    /// Empty company name
    #[error(
        "Firma darf nicht leer sein (§4 GmbHG)\n\
         Company name cannot be empty (§4 GmbHG)"
    )]
    EmptyCompanyName,

    /// Empty registered office city
    #[error(
        "Sitz der Gesellschaft darf nicht leer sein (§4a GmbHG)\n\
         Registered office city cannot be empty (§4a GmbHG)"
    )]
    EmptyRegisteredOffice,

    // ========================================================================
    // Managing Director Errors (Geschäftsführerfehler)
    // ========================================================================
    /// No managing directors appointed
    #[error(
        "Keine Geschäftsführer bestellt (§6 Abs. 3 GmbHG - mindestens 1 erforderlich)\n\
         No managing directors appointed (§6 para. 3 GmbHG - at least 1 required)"
    )]
    NoManagingDirectors,

    /// Managing director lacks legal capacity
    #[error(
        "Geschäftsführer '{name}' fehlt Geschäftsfähigkeit (§6 Abs. 2 S. 2 GmbHG - nur natürliche, voll geschäftsfähige Personen)\n\
         Managing director '{name}' lacks legal capacity (§6 para. 2 sent. 2 GmbHG - only natural persons with full capacity)"
    )]
    DirectorLacksCapacity { name: String },

    /// Managing director data incomplete
    #[error(
        "Geschäftsführerdaten unvollständig für '{name}': {reason}\n\
         Managing director data incomplete for '{name}': {reason}"
    )]
    IncompleteDirectorData { name: String, reason: String },

    /// Empty managing director name
    #[error(
        "Geschäftsführername darf nicht leer sein\n\
         Managing director name cannot be empty"
    )]
    EmptyDirectorName,

    /// Empty managing director address
    #[error(
        "Geschäftsführeranschrift darf nicht leer sein\n\
         Managing director address cannot be empty"
    )]
    EmptyDirectorAddress,

    // ========================================================================
    // Shareholder Errors (Gesellschafterfehler)
    // ========================================================================
    /// Contribution paid exceeds nominal amount
    #[error(
        "Gesellschafter '{name}': Einlage €{paid:.2} übersteigt Nennbetrag €{nominal:.2}\n\
         Shareholder '{name}': contribution €{paid:.2} exceeds nominal amount €{nominal:.2}"
    )]
    ContributionExceedsNominal {
        name: String,
        paid: f64,
        nominal: f64,
    },

    /// Nominal amount too low (must be at least €1)
    #[error(
        "Gesellschafter '{name}': Nennbetrag muss mindestens €1 sein\n\
         Shareholder '{name}': nominal amount must be at least €1"
    )]
    NominalAmountTooLow { name: String },

    /// Empty shareholder name
    #[error(
        "Gesellschaftername darf nicht leer sein\n\
         Shareholder name cannot be empty"
    )]
    EmptyShareholderName,

    /// Empty shareholder address
    #[error(
        "Gesellschafteranschrift darf nicht leer sein\n\
         Shareholder address cannot be empty"
    )]
    EmptyShareholderAddress,

    /// Share allocation has zero nominal amount
    #[error(
        "Geschäftsanteil mit Nennbetrag null ist ungültig\n\
         Share allocation with zero nominal amount is invalid"
    )]
    ZeroNominalAmount,

    // ========================================================================
    // General Validation Errors
    // ========================================================================
    /// Required field is missing
    #[error(
        "Pflichtfeld fehlt: {field_name}\n\
         Required field missing: {field_name}"
    )]
    MissingRequiredField { field_name: String },

    /// Generic validation error
    #[error(
        "Validierungsfehler: {message}\n\
         Validation error: {message}"
    )]
    ValidationError { message: String },

    /// Multiple validation errors occurred
    #[error(
        "Mehrere Validierungsfehler aufgetreten:\n{errors}\n\
         Multiple validation errors occurred:\n{errors}"
    )]
    MultipleErrors { errors: String },
}

impl GmbHError {
    /// Create a MissingRequiredField error
    pub fn missing_field(field_name: impl Into<String>) -> Self {
        Self::MissingRequiredField {
            field_name: field_name.into(),
        }
    }

    /// Create a ValidationError
    pub fn validation(message: impl Into<String>) -> Self {
        Self::ValidationError {
            message: message.into(),
        }
    }

    /// Create a MultipleErrors from a list of errors
    pub fn multiple(errors: Vec<GmbHError>) -> Self {
        let error_messages = errors
            .iter()
            .enumerate()
            .map(|(i, e)| format!("  {}. {}", i + 1, e))
            .collect::<Vec<_>>()
            .join("\n");

        Self::MultipleErrors {
            errors: error_messages,
        }
    }
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capital_below_minimum_error() {
        let err = GmbHError::CapitalBelowMinimum {
            actual_euros: 24_999.0,
            minimum_euros: 25_000.0,
        };

        let msg = err.to_string();
        assert!(msg.contains("24999"));
        assert!(msg.contains("25000"));
        assert!(msg.contains("§5 GmbHG"));
        assert!(msg.contains("Stammkapital")); // German
        assert!(msg.contains("Share capital")); // English
    }

    #[test]
    fn test_ug_capital_exceeds_limit_error() {
        let err = GmbHError::UGCapitalExceedsLimit {
            actual_euros: 25_000.0,
        };

        let msg = err.to_string();
        assert!(msg.contains("25000"));
        assert!(msg.contains("§5a GmbHG"));
        assert!(msg.contains("UG-Kapital")); // German
        assert!(msg.contains("UG capital")); // English
    }

    #[test]
    fn test_initial_contribution_too_low_error() {
        let err = GmbHError::InitialContributionTooLow {
            paid_euros: 10_000.0,
            required_euros: 12_500.0,
        };

        let msg = err.to_string();
        assert!(msg.contains("10000"));
        assert!(msg.contains("12500"));
        assert!(msg.contains("§7 Abs. 2 GmbHG"));
        assert!(msg.contains("Einlage")); // German
        assert!(msg.contains("Contribution")); // English
    }

    #[test]
    fn test_missing_legal_form_suffix_error() {
        let err = GmbHError::MissingLegalFormSuffix {
            name: "Tech Solutions".to_string(),
            required_suffix: "GmbH".to_string(),
        };

        let msg = err.to_string();
        assert!(msg.contains("Tech Solutions"));
        assert!(msg.contains("GmbH"));
        assert!(msg.contains("§4 GmbHG"));
        assert!(msg.contains("Rechtsformzusatz")); // German
        assert!(msg.contains("legal form suffix")); // English
    }

    #[test]
    fn test_no_managing_directors_error() {
        let err = GmbHError::NoManagingDirectors;

        let msg = err.to_string();
        assert!(msg.contains("§6 Abs. 3 GmbHG"));
        assert!(msg.contains("Geschäftsführer")); // German
        assert!(msg.contains("managing directors")); // English
    }

    #[test]
    fn test_director_lacks_capacity_error() {
        let err = GmbHError::DirectorLacksCapacity {
            name: "Max Mustermann".to_string(),
        };

        let msg = err.to_string();
        assert!(msg.contains("Max Mustermann"));
        assert!(msg.contains("§6 Abs. 2 S. 2 GmbHG"));
        assert!(msg.contains("Geschäftsfähigkeit")); // German
        assert!(msg.contains("legal capacity")); // English
    }

    #[test]
    fn test_contribution_exceeds_nominal_error() {
        let err = GmbHError::ContributionExceedsNominal {
            name: "Anna Schmidt".to_string(),
            paid: 30_000.0,
            nominal: 25_000.0,
        };

        let msg = err.to_string();
        assert!(msg.contains("Anna Schmidt"));
        assert!(msg.contains("30000"));
        assert!(msg.contains("25000"));
        assert!(msg.contains("Einlage")); // German
        assert!(msg.contains("contribution")); // English
    }

    #[test]
    fn test_share_allocation_mismatch_error() {
        let err = GmbHError::ShareAllocationMismatch {
            total_shares: 30_000.0,
            capital: 50_000.0,
        };

        let msg = err.to_string();
        assert!(msg.contains("30000"));
        assert!(msg.contains("50000"));
        assert!(msg.contains("Geschäftsanteile")); // German
        assert!(msg.contains("share allocations")); // English
    }

    #[test]
    fn test_invalid_business_purpose_error() {
        let err = GmbHError::InvalidBusinessPurpose;

        let msg = err.to_string();
        assert!(msg.contains("§3 Abs. 1 Nr. 2 GmbHG"));
        assert!(msg.contains("Unternehmensgegenstand")); // German
        assert!(msg.contains("Business purpose")); // English
    }

    #[test]
    fn test_no_shareholders_error() {
        let err = GmbHError::NoShareholders;

        let msg = err.to_string();
        assert!(msg.contains("Gesellschafter")); // German
        assert!(msg.contains("shareholders")); // English
    }

    #[test]
    fn test_missing_required_field_error() {
        let err = GmbHError::missing_field("company_name");

        let msg = err.to_string();
        assert!(msg.contains("company_name"));
        assert!(msg.contains("Pflichtfeld")); // German
        assert!(msg.contains("Required field")); // English
    }

    #[test]
    fn test_validation_error() {
        let err = GmbHError::validation("Custom validation message");

        let msg = err.to_string();
        assert!(msg.contains("Custom validation message"));
        assert!(msg.contains("Validierungsfehler")); // German
        assert!(msg.contains("Validation error")); // English
    }

    #[test]
    fn test_multiple_errors() {
        let errors = vec![
            GmbHError::EmptyCompanyName,
            GmbHError::NoShareholders,
            GmbHError::NoManagingDirectors,
        ];

        let err = GmbHError::multiple(errors);

        let msg = err.to_string();
        assert!(msg.contains("1."));
        assert!(msg.contains("2."));
        assert!(msg.contains("3."));
        assert!(msg.contains("Mehrere Validierungsfehler")); // German
        assert!(msg.contains("Multiple validation errors")); // English
    }

    #[test]
    fn test_error_equality() {
        let err1 = GmbHError::EmptyCompanyName;
        let err2 = GmbHError::EmptyCompanyName;
        let err3 = GmbHError::NoShareholders;

        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_error_clone() {
        let err1 = GmbHError::CapitalBelowMinimum {
            actual_euros: 1000.0,
            minimum_euros: 25000.0,
        };

        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }
}
