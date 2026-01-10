//! Error types for German Constitutional Law (Grundgesetz)
//!
//! Provides bilingual error messages (German/English) for constitutional law validation
//! with article references from the Grundgesetz (GG).

use thiserror::Error;

/// Result type for constitutional law operations
pub type Result<T> = std::result::Result<T, ConstitutionalError>;

/// Errors for German constitutional law validation
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ConstitutionalError {
    // Basic Rights Errors (Grundrechte)
    #[error(
        "Grundrechtsträger fehlt oder ungültig\n\
         Basic right holder missing or invalid"
    )]
    InvalidRightHolder,

    #[error(
        "Grundrecht nur für deutsche Staatsangehörige (Art. {article} GG)\n\
         Basic right limited to German citizens (Art. {article} GG)"
    )]
    RightLimitedToGermanCitizens { article: String },

    #[error(
        "Öffentliche Gewalt kann keine Grundrechtsträger sein\n\
         Public authority cannot be a basic right holder"
    )]
    PublicAuthorityNotRightHolder,

    #[error(
        "Einschränkung ohne gesetzliche Grundlage\n\
         Restriction without legal basis"
    )]
    RestrictionWithoutLegalBasis,

    #[error(
        "Wesensgehalt des Grundrechts verletzt (Art. 19 Abs. 2 GG)\n\
         Essential content of basic right violated (Art. 19 Para. 2 GG)"
    )]
    EssentialContentViolated,

    // Proportionality Test Errors (Verhältnismäßigkeit)
    #[error(
        "Maßnahme nicht geeignet: {reason}\n\
         Measure not suitable: {reason}"
    )]
    NotSuitable { reason: String },

    #[error(
        "Maßnahme nicht erforderlich: Mildere Mittel verfügbar - {alternatives}\n\
         Measure not necessary: Less restrictive alternatives available - {alternatives}"
    )]
    NotNecessary { alternatives: String },

    #[error(
        "Maßnahme unangemessen: Öffentliches Interesse überwiegt nicht\n\
         Measure disproportionate: Public interest does not outweigh private interest"
    )]
    NotProportionate,

    #[error(
        "Verhältnismäßigkeitsprüfung fehlgeschlagen\n\
         Proportionality test failed"
    )]
    ProportionalityTestFailed,

    // Constitutional Complaint Errors (Verfassungsbeschwerde)
    #[error(
        "Subsidiarität nicht gewahrt: Rechtsweg nicht erschöpft (Art. 90 Abs. 2 BVerfGG)\n\
         Subsidiarity not met: Legal remedies not exhausted (Art. 90 Para. 2 BVerfGG)"
    )]
    SubsidiarityNotMet,

    #[error(
        "Beschwerdefrist abgelaufen: {days} Tage seit Zustellung\n\
         Complaint deadline expired: {days} days since service"
    )]
    ComplaintDeadlineExpired { days: u32 },

    #[error(
        "Nicht selbst betroffen: Beschwerde unzulässig\n\
         Not personally affected: Complaint inadmissible"
    )]
    NotPersonallyAffected,

    #[error(
        "Nicht gegenwärtig betroffen: Beschwerde unzulässig\n\
         Not currently affected: Complaint inadmissible"
    )]
    NotCurrentlyAffected,

    #[error(
        "Nicht unmittelbar betroffen: Beschwerde unzulässig\n\
         Not directly affected: Complaint inadmissible"
    )]
    NotDirectlyAffected,

    // Federal Structure Errors (Bundesstruktur)
    #[error(
        "Ungültige Bundesratsstimmen: {actual} Stimmen für {population} Einwohner (Art. 51 Abs. 2 GG)\n\
         Invalid Bundesrat votes: {actual} votes for {population} inhabitants (Art. 51 Para. 2 GG)"
    )]
    InvalidBundesratVotes { actual: u8, population: u64 },

    #[error(
        "Freies Mandat verletzt: Abgeordneter nicht nur dem Gewissen unterworfen (Art. 38 Abs. 1 S. 2 GG)\n\
         Free mandate violated: Representative not subject to conscience alone (Art. 38 Para. 1 Sent. 2 GG)"
    )]
    FreeMandateViolated,

    #[error(
        "Bundespräsident mehr als zwei Amtszeiten (Art. 54 Abs. 2 GG)\n\
         Federal President more than two terms (Art. 54 Para. 2 GG)"
    )]
    PresidentTooManyTerms,

    #[error(
        "Richtlinienkompetenz nicht beachtet: Bundeskanzler bestimmt Richtlinien (Art. 65 S. 1 GG)\n\
         Policy guidelines not respected: Federal Chancellor determines policy (Art. 65 Sent. 1 GG)"
    )]
    PolicyGuidelinesViolated,

    // Legislative Competence Errors (Gesetzgebungskompetenz)
    #[error(
        "Keine Gesetzgebungskompetenz: {level} hat keine Zuständigkeit für {subject}\n\
         No legislative competence: {level} has no competence for {subject}"
    )]
    NoLegislativeCompetence { level: String, subject: String },

    #[error(
        "Ausschließliche Bundeskompetenz: Länder dürfen nicht gesetzgebend tätig werden (Art. 71 GG)\n\
         Exclusive federal competence: States may not legislate (Art. 71 GG)"
    )]
    ExclusiveFederalCompetence,

    #[error(
        "Konkurrierende Gesetzgebung: Bundesbedürfnis nicht nachgewiesen (Art. 72 Abs. 2 GG)\n\
         Concurrent legislation: Federal necessity not demonstrated (Art. 72 Para. 2 GG)"
    )]
    FederalNecessityNotDemonstrated,

    // General Errors
    #[error(
        "Ungültiger Artikelverweis: {article}\n\
         Invalid article reference: {article}"
    )]
    InvalidArticleReference { article: String },

    #[error(
        "Leerer Name\n\
         Empty name"
    )]
    EmptyName,

    #[error(
        "Ungültiges Datum: {date}\n\
         Invalid date: {date}"
    )]
    InvalidDate { date: String },

    #[error(
        "Fehlende Begründung\n\
         Missing justification"
    )]
    MissingJustification,

    #[error(
        "Ungültige Zuständigkeit\n\
         Invalid authority"
    )]
    InvalidAuthority,
}
