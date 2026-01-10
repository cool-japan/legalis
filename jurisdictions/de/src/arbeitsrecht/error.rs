//! Error types for German Labor Law (Arbeitsrecht)
//!
//! Provides bilingual error messages (German/English) for labor law validation
//! with references to relevant German labor statutes.

use thiserror::Error;

/// Result type for labor law operations
pub type Result<T> = std::result::Result<T, LaborLawError>;

/// Errors for German labor law validation
#[derive(Error, Debug, Clone, PartialEq)]
pub enum LaborLawError {
    // Employment Contract Errors
    #[error(
        "Arbeitsvertrag nicht schriftlich dokumentiert (§2 NachwG)\n\
         Employment contract not documented in writing (§2 NachwG)"
    )]
    ContractNotWritten,

    #[error(
        "Probezeit überschreitet Maximum von 6 Monaten (§622 Abs. 3 BGB)\n\
         Probation period exceeds maximum of 6 months (§622 Para. 3 BGB)"
    )]
    ProbationTooLong,

    #[error(
        "Befristung ohne sachlichen Grund länger als 2 Jahre (§14 Abs. 2 TzBfG)\n\
         Fixed-term without reason longer than 2 years (§14 Para. 2 TzBfG)"
    )]
    FixedTermTooLong,

    // Working Hours Errors (ArbZG)
    #[error(
        "Arbeitszeit überschreitet Maximum: {hours} Stunden/Tag (§3 ArbZG: max 10h)\n\
         Working hours exceed maximum: {hours} hours/day (§3 ArbZG: max 10h)"
    )]
    WorkingHoursExceeded { hours: f32 },

    #[error(
        "Mindestruhezeit nicht eingehalten: {hours} Stunden (§5 ArbZG: min 11h)\n\
         Minimum rest period not observed: {hours} hours (§5 ArbZG: min 11h)"
    )]
    InsufficientRestPeriod { hours: u8 },

    // Leave Errors (BUrlG)
    #[error(
        "Urlaubsanspruch unterschreitet Minimum: {actual} Tage (§3 BUrlG: min {required} Tage)\n\
         Leave entitlement below minimum: {actual} days (§3 BUrlG: min {required} days)"
    )]
    LeaveBelowMinimum { actual: u8, required: u8 },

    #[error(
        "Urlaubsübertrag nicht rechtzeitig: Urlaub verfällt (§7 Abs. 3 BUrlG)\n\
         Leave carryover not timely: Leave expires (§7 Para. 3 BUrlG)"
    )]
    LeaveCarryoverExpired,

    // Dismissal Errors (KSchG)
    #[error(
        "Kündigung nicht schriftlich (§623 BGB)\n\
         Dismissal not in writing (§623 BGB)"
    )]
    DismissalNotWritten,

    #[error(
        "Betriebsrat nicht angehört (§102 BetrVG)\n\
         Works council not consulted (§102 BetrVG)"
    )]
    WorksCouncilNotConsulted,

    #[error(
        "Kündigungsschutz gilt: Kein sachlicher Grund (§1 KSchG)\n\
         Dismissal protection applies: No justified reason (§1 KSchG)"
    )]
    NoJustifiedDismissalReason,

    #[error(
        "Kündigungsfrist nicht eingehalten: {actual} Wochen (§622 BGB: min {required} Wochen)\n\
         Notice period not observed: {actual} weeks (§622 BGB: min {required} weeks)"
    )]
    InsufficientNoticePeriod { actual: u8, required: u8 },

    #[error(
        "Außerordentliche Kündigung ohne wichtigen Grund (§626 BGB)\n\
         Extraordinary dismissal without good cause (§626 BGB)"
    )]
    NoGoodCauseForExtraordinaryDismissal,

    #[error(
        "Kündigungsschutzklage verspätet: {weeks} Wochen (§4 KSchG: max 3 Wochen)\n\
         Dismissal protection action late: {weeks} weeks (§4 KSchG: max 3 weeks)"
    )]
    DismissalActionTooLate { weeks: u8 },

    // Sick Leave Errors (EFZG)
    #[error(
        "Arbeitsunfähigkeitsbescheinigung fehlt nach 3 Tagen (§5 EFZG)\n\
         Medical certificate missing after 3 days (§5 EFZG)"
    )]
    MedicalCertificateMissing,

    #[error(
        "Arbeitgeber nicht rechtzeitig informiert\n\
         Employer not notified timely"
    )]
    EmployerNotNotified,

    #[error(
        "Entgeltfortzahlung überschreitet 6 Wochen (§3 EFZG)\n\
         Continued remuneration exceeds 6 weeks (§3 EFZG)"
    )]
    ContinuedPayExceedsSixWeeks,

    // Maternity Protection Errors (MuSchG)
    #[error(
        "Mutterschutzfrist nicht eingehalten: {weeks} Wochen (§3 MuSchG: 6 Wochen vor, 8 Wochen nach Geburt)\n\
         Maternity leave period not observed: {weeks} weeks (§3 MuSchG: 6 before, 8 after birth)"
    )]
    MaternityLeaveIncorrect { weeks: u8 },

    #[error(
        "Kündigung während Mutterschutz (§17 MuSchG)\n\
         Dismissal during maternity protection (§17 MuSchG)"
    )]
    DismissalDuringMaternityProtection,

    // Parental Leave Errors (BEEG)
    #[error(
        "Elternzeit überschreitet Maximum: {years} Jahre (§15 BEEG: max 3 Jahre)\n\
         Parental leave exceeds maximum: {years} years (§15 BEEG: max 3 years)"
    )]
    ParentalLeaveTooLong { years: f32 },

    #[error(
        "Elternzeit-Antrag zu spät: {weeks} Wochen (§16 BEEG: min 7 Wochen vor Beginn)\n\
         Parental leave request too late: {weeks} weeks (§16 BEEG: min 7 weeks before start)"
    )]
    ParentalLeaveNoticeTooLate { weeks: u8 },

    #[error(
        "Kündigung während Elternzeit (§18 BEEG)\n\
         Dismissal during parental leave (§18 BEEG)"
    )]
    DismissalDuringParentalLeave,

    // Works Council Errors (BetrVG)
    #[error(
        "Betriebsrat erforderlich: {employee_count} Arbeitnehmer (§1 BetrVG: ab 5 Arbeitnehmern)\n\
         Works council required: {employee_count} employees (§1 BetrVG: from 5 employees)"
    )]
    WorksCouncilRequired { employee_count: u32 },

    #[error(
        "Betriebsratsgröße falsch: {actual} Mitglieder für {employee_count} Arbeitnehmer (§9 BetrVG: {required} erforderlich)\n\
         Works council size incorrect: {actual} members for {employee_count} employees (§9 BetrVG: {required} required)"
    )]
    WorksCouncilSizeIncorrect {
        actual: u8,
        required: u8,
        employee_count: u32,
    },

    #[error(
        "Mitbestimmungsrecht verletzt (§87 BetrVG)\n\
         Co-determination right violated (§87 BetrVG)"
    )]
    CodeterminationRightViolated,

    // General Errors
    #[error(
        "Ungültiges Datum: {date}\n\
         Invalid date: {date}"
    )]
    InvalidDate { date: String },

    #[error(
        "Leerer Name\n\
         Empty name"
    )]
    EmptyName,

    #[error(
        "Fehlende Pflichtangabe: {field}\n\
         Missing required field: {field}"
    )]
    MissingRequiredField { field: String },

    #[error(
        "Ungültiger Wert: {reason}\n\
         Invalid value: {reason}"
    )]
    InvalidValue { reason: String },
}
