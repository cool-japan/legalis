//! Error types for German commercial law validation (HGB)
//!
//! Comprehensive error definitions with bilingual (German/English) messages
//! for partnership validation.

use thiserror::Error;

/// Result type alias for HGB operations
pub type Result<T> = std::result::Result<T, HGBError>;

/// Comprehensive error types for HGB partnership validation
///
/// All errors include bilingual messages (German primary, English secondary)
/// with relevant HGB article references.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum HGBError {
    // ========================================================================
    // Partnership Name Errors (Firma-Fehler)
    // ========================================================================
    /// Partnership name missing required legal form suffix
    #[error(
        "Firma '{name}' fehlt erforderlicher Rechtsformzusatz '{required_suffix}' (§19 HGB)\n\
         Partnership name '{name}' missing required legal form suffix '{required_suffix}' (§19 HGB)"
    )]
    MissingLegalFormSuffix {
        name: String,
        required_suffix: String,
    },

    /// Empty partnership name
    #[error(
        "Firma darf nicht leer sein (§19 HGB)\n\
         Partnership name cannot be empty (§19 HGB)"
    )]
    EmptyPartnershipName,

    /// Partnership name too short
    #[error(
        "Firma '{name}' ist zu kurz (mindestens 3 Zeichen erforderlich)\n\
         Partnership name '{name}' is too short (minimum 3 characters required)"
    )]
    PartnershipNameTooShort { name: String },

    // ========================================================================
    // Partner Errors (Gesellschafter-Fehler)
    // ========================================================================
    /// Insufficient number of partners
    #[error(
        "Nicht genügend Gesellschafter: {actual} vorhanden, {required} erforderlich ({partnership_type})\n\
         Insufficient partners: {actual} present, {required} required ({partnership_type})"
    )]
    InsufficientPartners {
        actual: usize,
        required: usize,
        partnership_type: String,
    },

    /// Empty partner name
    #[error(
        "Gesellschaftername darf nicht leer sein\n\
         Partner name cannot be empty"
    )]
    EmptyPartnerName,

    /// Empty partner address
    #[error(
        "Gesellschafteranschrift darf nicht leer sein\n\
         Partner address cannot be empty"
    )]
    EmptyPartnerAddress,

    /// Contribution paid exceeds contribution amount
    #[error(
        "Gesellschafter '{name}': Geleistete Einlage €{paid:.2} übersteigt Einlageverpflichtung €{contribution:.2}\n\
         Partner '{name}': Paid contribution €{paid:.2} exceeds contribution obligation €{contribution:.2}"
    )]
    ContributionPaidExceedsObligation {
        name: String,
        paid: f64,
        contribution: f64,
    },

    // ========================================================================
    // OHG-Specific Errors
    // ========================================================================
    /// OHG requires minimum 2 partners
    #[error(
        "OHG benötigt mindestens 2 Gesellschafter (§105 Abs. 1 HGB)\n\
         OHG requires at least 2 partners (§105 para. 1 HGB)"
    )]
    OHGInsufficientPartners,

    /// All OHG partners must have unlimited liability
    #[error(
        "OHG: Alle Gesellschafter müssen unbeschränkt haften (§128 HGB)\n\
         OHG: All partners must have unlimited liability (§128 HGB)"
    )]
    OHGLimitedLiabilityNotAllowed,

    // ========================================================================
    // KG-Specific Errors
    // ========================================================================
    /// KG requires at least one general partner
    #[error(
        "KG benötigt mindestens 1 Komplementär (persönlich haftender Gesellschafter) (§161 Abs. 1 HGB)\n\
         KG requires at least 1 general partner (unlimited liability) (§161 para. 1 HGB)"
    )]
    KGNoGeneralPartner,

    /// KG requires at least one limited partner
    #[error(
        "KG benötigt mindestens 1 Kommanditisten (§161 Abs. 1 HGB)\n\
         KG requires at least 1 limited partner (§161 para. 1 HGB)"
    )]
    KGNoLimitedPartner,

    /// Limited partner liability limit too low
    #[error(
        "Kommanditist '{name}': Haftsumme €{liability:.2} ist zu niedrig (mindestens €1 erforderlich)\n\
         Limited partner '{name}': Liability limit €{liability:.2} is too low (minimum €1 required)"
    )]
    LiabilityLimitTooLow { name: String, liability: f64 },

    /// Limited partner contribution exceeds liability limit
    #[error(
        "Kommanditist '{name}': Geleistete Einlage €{paid:.2} übersteigt Haftsumme €{limit:.2}\n\
         Limited partner '{name}': Paid contribution €{paid:.2} exceeds liability limit €{limit:.2}"
    )]
    ContributionExceedsLiabilityLimit { name: String, paid: f64, limit: f64 },

    /// Zero liability limit for limited partner
    #[error(
        "Kommanditist '{name}': Haftsumme kann nicht null sein (§171 HGB)\n\
         Limited partner '{name}': Liability limit cannot be zero (§171 HGB)"
    )]
    ZeroLiabilityLimit { name: String },

    // ========================================================================
    // GmbH & Co. KG Specific Errors
    // ========================================================================
    /// GmbH & Co. KG requires GmbH as general partner
    #[error(
        "GmbH & Co. KG benötigt eine GmbH als Komplementär\n\
         GmbH & Co. KG requires a GmbH as general partner"
    )]
    GmbHCoKGNoGmbHPartner,

    /// GmbH partner has invalid structure
    #[error(
        "GmbH-Komplementär '{name}' ist ungültig: {reason}\n\
         GmbH general partner '{name}' is invalid: {reason}"
    )]
    InvalidGmbHPartner { name: String, reason: String },

    /// GmbH partner has insufficient capital
    #[error(
        "GmbH-Komplementär '{name}': Stammkapital €{actual:.2} unterschreitet Mindestbetrag €25,000\n\
         GmbH partner '{name}': Share capital €{actual:.2} is below minimum €25,000"
    )]
    GmbHPartnerInsufficientCapital { name: String, actual: f64 },

    /// GmbH partner has no managing directors
    #[error(
        "GmbH-Komplementär '{name}' hat keine Geschäftsführer\n\
         GmbH partner '{name}' has no managing directors"
    )]
    GmbHPartnerNoDirectors { name: String },

    // ========================================================================
    // Business Purpose Errors
    // ========================================================================
    /// Invalid business purpose
    #[error(
        "Unternehmensgegenstand fehlt oder ist zu kurz (mindestens 10 Zeichen erforderlich)\n\
         Business purpose missing or too short (minimum 10 characters required)"
    )]
    InvalidBusinessPurpose,

    /// Empty business purpose
    #[error(
        "Unternehmensgegenstand darf nicht leer sein\n\
         Business purpose cannot be empty"
    )]
    EmptyBusinessPurpose,

    // ========================================================================
    // Registered Office Errors
    // ========================================================================
    /// Invalid registered office
    #[error(
        "Sitz '{city}' ist keine gültige deutsche Stadt\n\
         Registered office '{city}' is not a valid German city"
    )]
    InvalidRegisteredOffice { city: String },

    /// Empty registered office
    #[error(
        "Sitz der Gesellschaft darf nicht leer sein\n\
         Registered office cannot be empty"
    )]
    EmptyRegisteredOffice,

    // ========================================================================
    // Fiscal Year End Errors
    // ========================================================================
    /// Invalid fiscal year end date
    #[error(
        "Geschäftsjahresende ungültig: Monat {month}, Tag {day}\n\
         Fiscal year end invalid: month {month}, day {day}"
    )]
    InvalidFiscalYearEnd { month: u8, day: u8 },

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

impl HGBError {
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
    pub fn multiple(errors: Vec<HGBError>) -> Self {
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
    fn test_missing_legal_form_suffix() {
        let err = HGBError::MissingLegalFormSuffix {
            name: "Mustermann & Schmidt".to_string(),
            required_suffix: "OHG".to_string(),
        };

        let msg = err.to_string();
        assert!(msg.contains("Mustermann & Schmidt"));
        assert!(msg.contains("OHG"));
        assert!(msg.contains("§19 HGB"));
        assert!(msg.contains("Firma")); // German
        assert!(msg.contains("Partnership name")); // English
    }

    #[test]
    fn test_insufficient_partners() {
        let err = HGBError::InsufficientPartners {
            actual: 1,
            required: 2,
            partnership_type: "OHG".to_string(),
        };

        let msg = err.to_string();
        assert!(msg.contains('1'));
        assert!(msg.contains('2'));
        assert!(msg.contains("OHG"));
        assert!(msg.contains("Gesellschafter")); // German
        assert!(msg.contains("partners")); // English
    }

    #[test]
    fn test_ohg_insufficient_partners() {
        let err = HGBError::OHGInsufficientPartners;

        let msg = err.to_string();
        assert!(msg.contains("§105 Abs. 1 HGB"));
        assert!(msg.contains("mindestens 2")); // German
        assert!(msg.contains("at least 2")); // English
    }

    #[test]
    fn test_kg_no_general_partner() {
        let err = HGBError::KGNoGeneralPartner;

        let msg = err.to_string();
        assert!(msg.contains("§161"));
        assert!(msg.contains("Komplementär")); // German
        assert!(msg.contains("general partner")); // English
    }

    #[test]
    fn test_kg_no_limited_partner() {
        let err = HGBError::KGNoLimitedPartner;

        let msg = err.to_string();
        assert!(msg.contains("§161"));
        assert!(msg.contains("Kommanditisten")); // German
        assert!(msg.contains("limited partner")); // English
    }

    #[test]
    fn test_liability_limit_too_low() {
        let err = HGBError::LiabilityLimitTooLow {
            name: "Anna Schmidt".to_string(),
            liability: 0.5,
        };

        let msg = err.to_string();
        assert!(msg.contains("Anna Schmidt"));
        assert!(msg.contains("0.5"));
        assert!(msg.contains("Haftsumme")); // German
        assert!(msg.contains("Liability limit")); // English
    }

    #[test]
    fn test_contribution_exceeds_liability_limit() {
        let err = HGBError::ContributionExceedsLiabilityLimit {
            name: "Max Mustermann".to_string(),
            paid: 60_000.0,
            limit: 50_000.0,
        };

        let msg = err.to_string();
        assert!(msg.contains("Max Mustermann"));
        assert!(msg.contains("60000"));
        assert!(msg.contains("50000"));
    }

    #[test]
    fn test_gmbh_partner_insufficient_capital() {
        let err = HGBError::GmbHPartnerInsufficientCapital {
            name: "Verwaltungs GmbH".to_string(),
            actual: 20_000.0,
        };

        let msg = err.to_string();
        assert!(msg.contains("Verwaltungs GmbH"));
        assert!(msg.contains("20000.00"));
        assert!(msg.contains("25,000")); // Hardcoded in error message with comma
    }

    #[test]
    fn test_error_equality() {
        let err1 = HGBError::EmptyPartnershipName;
        let err2 = HGBError::EmptyPartnershipName;
        let err3 = HGBError::EmptyBusinessPurpose;

        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_error_clone() {
        let err1 = HGBError::InvalidBusinessPurpose;
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_multiple_errors() {
        let errors = vec![
            HGBError::EmptyPartnershipName,
            HGBError::OHGInsufficientPartners,
            HGBError::InvalidBusinessPurpose,
        ];

        let err = HGBError::multiple(errors);

        let msg = err.to_string();
        assert!(msg.contains("1."));
        assert!(msg.contains("2."));
        assert!(msg.contains("3."));
        assert!(msg.contains("Mehrere Validierungsfehler")); // German
        assert!(msg.contains("Multiple validation errors")); // English
    }
}
