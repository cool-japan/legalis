//! Error types for German Tax Law (Steuerrecht)
//!
//! Bilingual error messages (German/English) for tax-law validation and
//! computation across the Income Tax Act (EStG), the VAT Act (UStG) and the
//! Fiscal Code (Abgabenordnung, AO), with § citations.
//!
//! Fehlertypen für das deutsche Steuerrecht. Die Meldungen sind zweisprachig
//! (Deutsch/Englisch) und enthalten Paragraphenverweise auf EStG, UStG und AO.

use thiserror::Error;

/// Result type for tax-law operations.
///
/// Ergebnistyp für steuerrechtliche Operationen.
pub type Result<T> = std::result::Result<T, SteuerError>;

/// Errors for German tax-law validation and computation.
///
/// Fehler bei der Prüfung und Berechnung im deutschen Steuerrecht.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum SteuerError {
    /// An invalid monetary amount was supplied (e.g. an amount that cannot be
    /// represented or violates a domain constraint).
    #[error(
        "Ungültiger Betrag: {detail}\n\
         Invalid amount: {detail}"
    )]
    InvalidAmount {
        /// Human-readable detail describing why the amount is invalid.
        detail: String,
    },

    /// An income type (Einkunftsart) was expected but could not be determined.
    #[error(
        "Ungültige Einkunftsart (§ 2 Abs. 1 EStG)\n\
         Invalid income type (§ 2 Para. 1 EStG)"
    )]
    InvalidIncomeType,

    /// A tax base (Bemessungsgrundlage) was negative where a non-negative value
    /// is required.
    #[error(
        "Negative Bemessungsgrundlage unzulässig\n\
         Negative tax base not permitted"
    )]
    NegativeTaxBase,

    /// The requested tax year is not supported by the encoded tariff/rules.
    #[error(
        "Ungültiges bzw. nicht unterstütztes Steuerjahr: {year} (§ 32a EStG)\n\
         Invalid or unsupported tax year: {year} (§ 32a EStG)"
    )]
    InvalidTaxYear {
        /// The offending tax year.
        year: i32,
    },

    /// An unknown / unclassifiable type of income (Einkunftsart).
    #[error(
        "Unbekannte Einkunftsart (§ 2 Abs. 1 EStG)\n\
         Unknown type of income (§ 2 Para. 1 EStG)"
    )]
    UnknownEinkunftsart,

    /// An invalid VAT/income tax rate (Steuersatz) was supplied.
    #[error(
        "Ungültiger Steuersatz (§ 12 UStG / § 32a EStG)\n\
         Invalid tax rate (§ 12 UStG / § 32a EStG)"
    )]
    InvalidSteuersatz,

    /// The period for assessment (Festsetzungsfrist) has expired, § 169 AO.
    #[error(
        "Festsetzungsfrist abgelaufen: {years} Jahre (§ 169 AO)\n\
         Period for assessment expired: {years} years (§ 169 AO)"
    )]
    FestsetzungsfristAbgelaufen {
        /// The applicable assessment period in years that has elapsed.
        years: u32,
    },

    /// A tax assessment notice (Steuerbescheid) is not effective because it has
    /// not been disclosed (bekanntgegeben), § 124 AO.
    #[error(
        "Steuerbescheid nicht wirksam: noch nicht bekanntgegeben (§ 124 AO)\n\
         Tax assessment not effective: not yet disclosed (§ 124 AO)"
    )]
    BescheidNichtWirksam,

    /// An objection (Einspruch) was filed too late, § 355 AO.
    #[error(
        "Einspruch verfristet: {days} Tage nach Bekanntgabe (§ 355 AO)\n\
         Objection out of time: {days} days after disclosure (§ 355 AO)"
    )]
    EinspruchVerfristet {
        /// Number of days between disclosure and the (late) objection.
        days: i64,
    },

    /// A required field was empty.
    #[error(
        "Leeres Pflichtfeld: {field}\n\
         Empty required field: {field}"
    )]
    EmptyField {
        /// Name of the empty field.
        field: String,
    },
}
