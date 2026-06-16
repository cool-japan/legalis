//! Error types for German Administrative Law (Verwaltungsrecht - VwVfG)
//!
//! Provides bilingual error messages (German/English) for administrative procedure
//! validation with paragraph (§) references from the Verwaltungsverfahrensgesetz
//! (VwVfG) and, for legal remedies, from the Verwaltungsgerichtsordnung (VwGO).
//!
//! Fehlertypen für das deutsche Verwaltungsrecht (VwVfG) mit zweisprachigen
//! Fehlermeldungen (Deutsch/Englisch) und §-Zitaten.

use thiserror::Error;

/// Result type for administrative law operations.
///
/// Ergebnistyp für verwaltungsrechtliche Operationen.
pub type Result<T> = std::result::Result<T, VwVfGError>;

/// Validation errors for German administrative procedure (VwVfG / VwGO).
///
/// Validierungsfehler für das deutsche Verwaltungsverfahren (VwVfG / VwGO).
#[derive(Error, Debug, Clone, PartialEq)]
pub enum VwVfGError {
    // === Verwaltungsakt (§ 35 VwVfG) ===
    #[error(
        "Kein Verwaltungsakt: kein Tatbestandsmerkmal des § 35 S. 1 VwVfG erfüllt\n\
         Not an administrative act: no element of § 35 sent. 1 VwVfG fulfilled"
    )]
    NotAVerwaltungsakt,

    #[error(
        "Keine hoheitliche Maßnahme einer Behörde (§ 35 S. 1 VwVfG)\n\
         No sovereign measure of an authority (§ 35 sent. 1 VwVfG)"
    )]
    MissingHoheitlicheMassnahme,

    #[error(
        "Keine Regelung: Maßnahme ist nicht auf eine Rechtsfolge gerichtet (§ 35 S. 1 VwVfG)\n\
         No regulation: the measure is not directed at a legal consequence (§ 35 sent. 1 VwVfG)"
    )]
    NoRegelung,

    #[error(
        "Keine unmittelbare Rechtswirkung nach außen (§ 35 S. 1 VwVfG)\n\
         No direct external legal effect (§ 35 sent. 1 VwVfG)"
    )]
    NoAussenwirkung,

    #[error(
        "Kein Einzelfall: keine konkret-individuelle Regelung (§ 35 S. 1 VwVfG)\n\
         Not an individual case: no concrete-individual regulation (§ 35 sent. 1 VwVfG)"
    )]
    NotEinzelfall,

    // === Bekanntgabe / Wirksamkeit (§§ 41, 43 VwVfG) ===
    #[error(
        "Verwaltungsakt nicht bekanntgegeben und damit unwirksam (§ 41, § 43 Abs. 1 VwVfG)\n\
         Administrative act not notified and thus ineffective (§ 41, § 43 para. 1 VwVfG)"
    )]
    NotBekanntgegeben,

    // === Nichtigkeit (§ 44 VwVfG) ===
    #[error(
        "Verwaltungsakt nichtig (§ 44 VwVfG): {grund}\n\
         Administrative act void (§ 44 VwVfG): {grund}"
    )]
    Nichtig {
        /// Reason for nullity (Nichtigkeitsgrund).
        grund: String,
    },

    // === Nebenbestimmungen (§ 36 VwVfG) ===
    #[error(
        "Unzulässige Nebenbestimmung zum gebundenen Verwaltungsakt (§ 36 Abs. 1 VwVfG)\n\
         Inadmissible ancillary provision for a bound administrative act (§ 36 para. 1 VwVfG)"
    )]
    UnzulaessigeNebenbestimmung,

    // === Rücknahme / Widerruf (§§ 48, 49 VwVfG) ===
    #[error(
        "Rücknahme unzulässig: Vertrauensschutz beim begünstigenden Verwaltungsakt (§ 48 Abs. 2 VwVfG)\n\
         Retraction inadmissible: protection of legitimate expectations for a favourable act (§ 48 para. 2 VwVfG)"
    )]
    RuecknahmeUnzulaessig,

    #[error(
        "Widerruf unzulässig: kein Widerrufsgrund nach § 49 Abs. 2/3 VwVfG\n\
         Revocation inadmissible: no ground for revocation under § 49 para. 2/3 VwVfG"
    )]
    WiderrufUnzulaessig,

    // === Rechtsbehelfe (VwGO) ===
    #[error(
        "Widerspruch verfristet: {days} Tage nach Bekanntgabe (Frist: ein Monat, § 70 Abs. 1 VwGO)\n\
         Objection out of time: {days} days after notification (deadline: one month, § 70 para. 1 VwGO)"
    )]
    WiderspruchVerfristet {
        /// Days elapsed between Bekanntgabe and Widerspruchseinlegung.
        days: i64,
    },

    // === Verfahrensfehler (§§ 28, 37, 39 VwVfG) ===
    #[error(
        "Anhörung unterblieben (§ 28 Abs. 1 VwVfG)\n\
         Hearing of the affected party omitted (§ 28 para. 1 VwVfG)"
    )]
    AnhoerungUnterblieben,

    #[error(
        "Begründung fehlt (§ 39 Abs. 1 VwVfG)\n\
         Statement of reasons missing (§ 39 para. 1 VwVfG)"
    )]
    BegruendungFehlt,

    #[error(
        "Formfehler: {detail} (§ 37 VwVfG)\n\
         Formal defect: {detail} (§ 37 VwVfG)"
    )]
    Formfehler {
        /// Details of the formal defect (Beschreibung des Formfehlers).
        detail: String,
    },

    // === General ===
    #[error(
        "Leeres Pflichtfeld: {field}\n\
         Empty required field: {field}"
    )]
    EmptyField {
        /// Name of the empty field (Name des leeren Feldes).
        field: String,
    },
}
