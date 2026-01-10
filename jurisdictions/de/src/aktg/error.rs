//! Error types for German stock corporation validation (AktG)
//!
//! Comprehensive error definitions with bilingual (German/English) messages
//! for AG validation.

use thiserror::Error;

/// Result type alias for AktG operations
pub type Result<T> = std::result::Result<T, AktGError>;

/// Comprehensive error types for AG validation
///
/// All errors include bilingual messages (German primary, English secondary)
/// with relevant AktG article references.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum AktGError {
    // ========================================================================
    // Share Capital Errors (Grundkapital-Fehler)
    // ========================================================================
    /// Share capital below minimum €50,000
    #[error(
        "Grundkapital €{actual:.2} unterschreitet Mindestbetrag €50,000 (§7 AktG)\n\
         Share capital €{actual:.2} is below minimum €50,000 (§7 AktG)"
    )]
    CapitalBelowMinimum { actual: f64 },

    /// Share capital is zero
    #[error(
        "Grundkapital darf nicht null sein (§7 AktG)\n\
         Share capital cannot be zero (§7 AktG)"
    )]
    ZeroCapital,

    /// Shares do not sum to share capital
    #[error(
        "Summe der Aktiennennbeträge €{shares_total:.2} stimmt nicht mit Grundkapital €{capital:.2} überein\n\
         Sum of share par values €{shares_total:.2} does not match share capital €{capital:.2}"
    )]
    SharesMismatchCapital { shares_total: f64, capital: f64 },

    // ========================================================================
    // Share Errors (Aktien-Fehler)
    // ========================================================================
    /// No shares issued
    #[error(
        "Keine Aktien ausgegeben\n\
         No shares issued"
    )]
    NoShares,

    /// Par value share below minimum €1
    #[error(
        "Nennbetrag €{par_value:.2} unterschreitet Mindestbetrag €1 (§8 Abs. 2 AktG)\n\
         Par value €{par_value:.2} is below minimum €1 (§8 para. 2 AktG)"
    )]
    ParValueTooLow { par_value: f64 },

    /// No-par share notional value below minimum €1
    #[error(
        "Rechnerischer Wert €{notional_value:.2} der Stückaktie unterschreitet Mindestbetrag €1 (§8 Abs. 3 AktG)\n\
         Notional value €{notional_value:.2} of no-par share is below minimum €1 (§8 para. 3 AktG)"
    )]
    NotionalValueTooLow { notional_value: f64 },

    /// Initial payment insufficient (must be 25% of par value + full premium)
    #[error(
        "Eingezahlter Betrag €{paid:.2} unterschreitet Mindesteinlage (25% Nennbetrag + volles Agio = €{required:.2}) (§36a AktG)\n\
         Amount paid €{paid:.2} is below minimum payment (25% par value + full premium = €{required:.2}) (§36a AktG)"
    )]
    InsufficientInitialPayment { paid: f64, required: f64 },

    /// Amount paid exceeds issue price
    #[error(
        "Eingezahlter Betrag €{paid:.2} übersteigt Ausgabepreis €{issue_price:.2}\n\
         Amount paid €{paid:.2} exceeds issue price €{issue_price:.2}"
    )]
    PaidExceedsIssuePrice { paid: f64, issue_price: f64 },

    // ========================================================================
    // Company Name Errors (Firma-Fehler)
    // ========================================================================
    /// Company name missing required legal form suffix
    #[error(
        "Firma '{name}' fehlt erforderlicher Rechtsformzusatz 'AG' oder 'Aktiengesellschaft' (§4 AktG)\n\
         Company name '{name}' missing required legal form suffix 'AG' or 'Aktiengesellschaft' (§4 AktG)"
    )]
    MissingLegalFormSuffix { name: String },

    /// Empty company name
    #[error(
        "Firma darf nicht leer sein\n\
         Company name cannot be empty"
    )]
    EmptyCompanyName,

    /// Company name too short
    #[error(
        "Firma '{name}' ist zu kurz (mindestens 3 Zeichen erforderlich)\n\
         Company name '{name}' is too short (minimum 3 characters required)"
    )]
    CompanyNameTooShort { name: String },

    // ========================================================================
    // Management Board Errors (Vorstand-Fehler)
    // ========================================================================
    /// No management board members
    #[error(
        "Vorstand muss mindestens 1 Mitglied haben (§76 Abs. 2 AktG)\n\
         Management board must have at least 1 member (§76 para. 2 AktG)"
    )]
    NoManagementBoardMembers,

    /// Management board member lacks legal capacity
    #[error(
        "Vorstandsmitglied '{name}' fehlt Geschäftsfähigkeit (§76 Abs. 3 AktG)\n\
         Management board member '{name}' lacks legal capacity (§76 para. 3 AktG)"
    )]
    BoardMemberLacksCapacity { name: String },

    /// Management board member name empty
    #[error(
        "Vorstandsmitglied: Name darf nicht leer sein\n\
         Management board member: Name cannot be empty"
    )]
    EmptyBoardMemberName,

    /// Management board member address empty
    #[error(
        "Vorstandsmitglied '{name}': Anschrift darf nicht leer sein\n\
         Management board member '{name}': Address cannot be empty"
    )]
    EmptyBoardMemberAddress { name: String },

    /// Board member term exceeds maximum 5 years
    #[error(
        "Vorstandsmitglied '{name}': Bestellungsdauer übersteigt Maximum von 5 Jahren (§84 Abs. 1 AktG)\n\
         Management board member '{name}': Term exceeds maximum of 5 years (§84 para. 1 AktG)"
    )]
    BoardMemberTermTooLong { name: String },

    // ========================================================================
    // Supervisory Board Errors (Aufsichtsrat-Fehler)
    // ========================================================================
    /// Insufficient supervisory board members (minimum 3)
    #[error(
        "Aufsichtsrat muss mindestens 3 Mitglieder haben (§95 AktG)\n\
         Supervisory board must have at least 3 members (§95 AktG)"
    )]
    InsufficientSupervisoryBoardMembers,

    /// Supervisory board size not divisible by 3
    #[error(
        "Aufsichtsrat-Größe {size} ist nicht durch 3 teilbar (§95, §101 AktG)\n\
         Supervisory board size {size} is not divisible by 3 (§95, §101 AktG)"
    )]
    SupervisoryBoardSizeNotDivisibleByThree { size: usize },

    /// Supervisory board member name empty
    #[error(
        "Aufsichtsratsmitglied: Name darf nicht leer sein\n\
         Supervisory board member: Name cannot be empty"
    )]
    EmptySupervisoryBoardMemberName,

    /// Chairman not found in supervisory board
    #[error(
        "Vorsitzender '{chairman}' ist kein Mitglied des Aufsichtsrats\n\
         Chairman '{chairman}' is not a member of the supervisory board"
    )]
    ChairmanNotMember { chairman: String },

    /// Deputy chairman not found in supervisory board
    #[error(
        "Stellvertretender Vorsitzender '{deputy}' ist kein Mitglied des Aufsichtsrats\n\
         Deputy chairman '{deputy}' is not a member of the supervisory board"
    )]
    DeputyChairmanNotMember { deputy: String },

    /// Supervisory board member term exceeds maximum 4 years
    #[error(
        "Aufsichtsratsmitglied '{name}': Amtszeit übersteigt Maximum von 4 Jahren (§102 AktG)\n\
         Supervisory board member '{name}': Term exceeds maximum of 4 years (§102 AktG)"
    )]
    SupervisoryBoardMemberTermTooLong { name: String },

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

impl AktGError {
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
    pub fn multiple(errors: Vec<AktGError>) -> Self {
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
    fn test_capital_below_minimum() {
        let err = AktGError::CapitalBelowMinimum { actual: 45_000.0 };
        let msg = err.to_string();
        assert!(msg.contains("45000"));
        assert!(msg.contains("50,000"));
        assert!(msg.contains("§7 AktG"));
    }

    #[test]
    fn test_zero_capital() {
        let err = AktGError::ZeroCapital;
        let msg = err.to_string();
        assert!(msg.contains("§7 AktG"));
        assert!(msg.contains("null"));
    }

    #[test]
    fn test_shares_mismatch_capital() {
        let err = AktGError::SharesMismatchCapital {
            shares_total: 45_000.0,
            capital: 50_000.0,
        };
        let msg = err.to_string();
        assert!(msg.contains("45000"));
        assert!(msg.contains("50000"));
    }

    #[test]
    fn test_no_shares() {
        let err = AktGError::NoShares;
        let msg = err.to_string();
        assert!(msg.contains("Keine Aktien"));
        assert!(msg.contains("No shares"));
    }

    #[test]
    fn test_par_value_too_low() {
        let err = AktGError::ParValueTooLow { par_value: 0.50 };
        let msg = err.to_string();
        assert!(msg.contains("0.50"));
        assert!(msg.contains("§8 Abs. 2 AktG"));
    }

    #[test]
    fn test_insufficient_initial_payment() {
        let err = AktGError::InsufficientInitialPayment {
            paid: 10_000.0,
            required: 12_500.0,
        };
        let msg = err.to_string();
        assert!(msg.contains("10000"));
        assert!(msg.contains("12500"));
        assert!(msg.contains("§36a AktG"));
    }

    #[test]
    fn test_missing_legal_form_suffix() {
        let err = AktGError::MissingLegalFormSuffix {
            name: "Tech Solutions".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("Tech Solutions"));
        assert!(msg.contains("§4 AktG"));
    }

    #[test]
    fn test_no_management_board_members() {
        let err = AktGError::NoManagementBoardMembers;
        let msg = err.to_string();
        assert!(msg.contains("§76 Abs. 2 AktG"));
        assert!(msg.contains("mindestens 1"));
    }

    #[test]
    fn test_insufficient_supervisory_board_members() {
        let err = AktGError::InsufficientSupervisoryBoardMembers;
        let msg = err.to_string();
        assert!(msg.contains("§95 AktG"));
        assert!(msg.contains("mindestens 3"));
    }

    #[test]
    fn test_supervisory_board_size_not_divisible_by_three() {
        let err = AktGError::SupervisoryBoardSizeNotDivisibleByThree { size: 4 };
        let msg = err.to_string();
        assert!(msg.contains('4'));
        assert!(msg.contains("durch 3 teilbar"));
    }

    #[test]
    fn test_chairman_not_member() {
        let err = AktGError::ChairmanNotMember {
            chairman: "John Doe".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("John Doe"));
        assert!(msg.contains("Vorsitzender"));
    }

    #[test]
    fn test_board_member_term_too_long() {
        let err = AktGError::BoardMemberTermTooLong {
            name: "CEO".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("CEO"));
        assert!(msg.contains("5 Jahren"));
        assert!(msg.contains("§84 Abs. 1 AktG"));
    }

    #[test]
    fn test_error_clone() {
        let err1 = AktGError::ZeroCapital;
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_multiple_errors() {
        let errors = vec![
            AktGError::ZeroCapital,
            AktGError::NoShares,
            AktGError::NoManagementBoardMembers,
        ];

        let err = AktGError::multiple(errors);
        let msg = err.to_string();
        assert!(msg.contains("1."));
        assert!(msg.contains("2."));
        assert!(msg.contains("3."));
        assert!(msg.contains("Mehrere Validierungsfehler"));
    }
}
