//! UStG - VAT Act (Umsatzsteuergesetz)
//!
//! Type-safe representations of taxability (Steuerbarkeit, § 1 Abs. 1 Nr. 1 UStG)
//! and the standard / reduced VAT rates (Steuersätze, § 12 UStG).
//!
//! Typsichere Abbildung der Steuerbarkeit (§ 1 Abs. 1 Nr. 1 UStG) und der
//! Steuersätze (§ 12 UStG).
//!
//! ## § 1 Abs. 1 Nr. 1 UStG - Steuerbare Umsätze
//!
//! > (1) Der Umsatzsteuer unterliegen die folgenden Umsätze:
//! > 1. die Lieferungen und sonstigen Leistungen, die ein Unternehmer im Inland
//! >    gegen Entgelt im Rahmen seines Unternehmens ausführt.
//!
//! **English**: A supply (Lieferung or sonstige Leistung) is **taxable**
//! (steuerbar) if it is made by an entrepreneur (Unternehmer, § 2 UStG), within
//! the country (im Inland), for consideration (gegen Entgelt), in the course of
//! the entrepreneur's business (im Rahmen seines Unternehmens). All five elements
//! must be present.
//!
//! ## § 12 UStG - Steuersätze
//!
//! - Standard rate (Regelsteuersatz): **19 %**, § 12 Abs. 1 UStG.
//! - Reduced rate (ermäßigter Steuersatz): **7 %**, § 12 Abs. 2 UStG (e.g.
//!   foodstuffs, books).
//! - Exempt (steuerfrei): **0 %**, § 4 UStG.

use serde::{Deserialize, Serialize};

use crate::steuerrecht::error::Result;

/// Standard VAT rate in percent (Regelsteuersatz), § 12 Abs. 1 UStG.
///
/// Regelsteuersatz in Prozent.
pub const REGELSTEUERSATZ_PROZENT: u32 = 19;

/// Reduced VAT rate in percent (ermäßigter Steuersatz), § 12 Abs. 2 UStG.
///
/// Ermäßigter Steuersatz in Prozent.
pub const ERMAESSIGTER_SATZ_PROZENT: u32 = 7;

// =============================================================================
// § 1 Abs. 1 Nr. 1 UStG - Taxability (Steuerbarkeit)
// =============================================================================

/// A turnover / supply (Umsatz) examined for taxability, § 1 Abs. 1 Nr. 1 UStG.
///
/// The five statutory elements are modelled as booleans; the net consideration
/// (Nettoentgelt) is stored in EUR cents.
///
/// Ein Umsatz, der auf Steuerbarkeit geprüft wird; die fünf Tatbestandsmerkmale
/// sind als Wahrheitswerte abgebildet, das Nettoentgelt in Euro-Cent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Umsatz {
    /// Whether there is a supply of goods or services (Lieferung oder sonstige
    /// Leistung).
    pub lieferung_oder_leistung: bool,
    /// Whether the supplier is an entrepreneur (Unternehmer, § 2 UStG).
    pub unternehmer: bool,
    /// Whether the supply is made within the country (im Inland).
    pub im_inland: bool,
    /// Whether the supply is made for consideration (gegen Entgelt).
    pub gegen_entgelt: bool,
    /// Whether the supply is made in the course of the business (im Rahmen
    /// seines Unternehmens).
    pub im_rahmen_des_unternehmens: bool,
    /// The net consideration in EUR cents (Nettoentgelt).
    pub netto_entgelt_cents: u64,
}

impl Umsatz {
    /// Create a turnover with all five taxability elements present and a given
    /// net consideration in EUR cents.
    ///
    /// Erzeugt einen Umsatz mit allen fünf Tatbestandsmerkmalen und einem
    /// Nettoentgelt in Euro-Cent.
    #[must_use]
    pub fn steuerbarer_umsatz(netto_entgelt_cents: u64) -> Self {
        Self {
            lieferung_oder_leistung: true,
            unternehmer: true,
            im_inland: true,
            gegen_entgelt: true,
            im_rahmen_des_unternehmens: true,
            netto_entgelt_cents,
        }
    }
}

/// Determine whether a turnover is taxable (steuerbar), § 1 Abs. 1 Nr. 1 UStG.
///
/// Returns `true` if and only if all five statutory elements are present.
///
/// Bestimmt, ob ein Umsatz steuerbar ist; `true` genau dann, wenn alle fünf
/// Tatbestandsmerkmale vorliegen.
///
/// # Errors
/// This function currently performs no fallible validation and always returns
/// `Ok`; the [`Result`] return type keeps the API uniform with the rest of the
/// tax module and allows future element-level error reporting.
pub fn ist_steuerbar(u: &Umsatz) -> Result<bool> {
    Ok(u.lieferung_oder_leistung
        && u.unternehmer
        && u.im_inland
        && u.gegen_entgelt
        && u.im_rahmen_des_unternehmens)
}

// =============================================================================
// § 12 UStG - VAT rates (Steuersätze)
// =============================================================================

/// A VAT rate under § 12 UStG (Steuersatz).
///
/// Ein Umsatzsteuersatz nach § 12 UStG.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Steuersatz {
    /// Standard rate of 19 % (Regelsteuersatz), § 12 Abs. 1 UStG.
    Regelsteuersatz,
    /// Reduced rate of 7 % (ermäßigter Steuersatz), § 12 Abs. 2 UStG.
    ErmaessigterSatz,
    /// Exempt, 0 % (steuerfrei), § 4 UStG.
    Steuerfrei,
}

impl Steuersatz {
    /// The rate in percent, § 12 UStG.
    ///
    /// `19` for the standard rate (§ 12 Abs. 1), `7` for the reduced rate
    /// (§ 12 Abs. 2), `0` for exempt supplies (§ 4).
    ///
    /// Der Steuersatz in Prozent.
    #[must_use]
    pub fn prozent(&self) -> u32 {
        match self {
            Steuersatz::Regelsteuersatz => REGELSTEUERSATZ_PROZENT,
            Steuersatz::ErmaessigterSatz => ERMAESSIGTER_SATZ_PROZENT,
            Steuersatz::Steuerfrei => 0,
        }
    }
}

/// Compute the VAT amount (Umsatzsteuer) in EUR cents from a net amount.
///
/// Integer arithmetic: `netto_cents * prozent / 100`. The intermediate product
/// is computed in `u128` to avoid overflow; the final integer division
/// truncates (rounds **down**) any fractional cent.
///
/// Berechnet die Umsatzsteuer in Euro-Cent aus einem Nettobetrag
/// (`netto_cents * prozent / 100`, abgerundet).
#[must_use]
pub fn umsatzsteuer_cents(netto_cents: u64, satz: Steuersatz) -> u64 {
    let prozent = u128::from(satz.prozent());
    let produkt = u128::from(netto_cents) * prozent / 100;
    u64::try_from(produkt).unwrap_or(u64::MAX)
}

/// Compute the gross amount (Bruttobetrag) in EUR cents from a net amount.
///
/// `brutto = netto + umsatzsteuer`. Rounding follows [`umsatzsteuer_cents`].
///
/// Berechnet den Bruttobetrag in Euro-Cent (`netto + Umsatzsteuer`).
#[must_use]
pub fn brutto_cents(netto_cents: u64, satz: Steuersatz) -> u64 {
    netto_cents.saturating_add(umsatzsteuer_cents(netto_cents, satz))
}

/// Compute the net amount (Nettobetrag) in EUR cents from a gross amount.
///
/// `netto = brutto * 100 / (100 + prozent)`. Integer arithmetic in `u128`;
/// the division truncates (rounds **down**).
///
/// Berechnet den Nettobetrag in Euro-Cent aus dem Bruttobetrag
/// (`brutto * 100 / (100 + prozent)`, abgerundet).
#[must_use]
pub fn netto_aus_brutto_cents(brutto_cents: u64, satz: Steuersatz) -> u64 {
    let nenner = u128::from(100 + satz.prozent());
    let zaehler = u128::from(brutto_cents) * 100;
    u64::try_from(zaehler / nenner).unwrap_or(u64::MAX)
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steuerbar_full_success() {
        let u = Umsatz::steuerbarer_umsatz(10_000);
        assert!(ist_steuerbar(&u).expect("no fallible validation"));
    }

    #[test]
    fn steuerbar_missing_lieferung() {
        let mut u = Umsatz::steuerbarer_umsatz(10_000);
        u.lieferung_oder_leistung = false;
        assert!(!ist_steuerbar(&u).expect("no fallible validation"));
    }

    #[test]
    fn steuerbar_missing_unternehmer() {
        let mut u = Umsatz::steuerbarer_umsatz(10_000);
        u.unternehmer = false;
        assert!(!ist_steuerbar(&u).expect("no fallible validation"));
    }

    #[test]
    fn steuerbar_missing_inland() {
        let mut u = Umsatz::steuerbarer_umsatz(10_000);
        u.im_inland = false;
        assert!(!ist_steuerbar(&u).expect("no fallible validation"));
    }

    #[test]
    fn steuerbar_missing_entgelt() {
        let mut u = Umsatz::steuerbarer_umsatz(10_000);
        u.gegen_entgelt = false;
        assert!(!ist_steuerbar(&u).expect("no fallible validation"));
    }

    #[test]
    fn steuerbar_missing_rahmen() {
        let mut u = Umsatz::steuerbarer_umsatz(10_000);
        u.im_rahmen_des_unternehmens = false;
        assert!(!ist_steuerbar(&u).expect("no fallible validation"));
    }

    #[test]
    fn satz_prozent_values() {
        assert_eq!(Steuersatz::Regelsteuersatz.prozent(), 19);
        assert_eq!(Steuersatz::ErmaessigterSatz.prozent(), 7);
        assert_eq!(Steuersatz::Steuerfrei.prozent(), 0);
    }

    #[test]
    fn umsatzsteuer_regelsatz_on_100_euro() {
        // 100,00 EUR = 10 000 cents at 19 % = 19,00 EUR = 1 900 cents.
        assert_eq!(
            umsatzsteuer_cents(10_000, Steuersatz::Regelsteuersatz),
            1_900
        );
    }

    #[test]
    fn umsatzsteuer_ermaessigt_on_100_euro() {
        // 100,00 EUR at 7 % = 7,00 EUR = 700 cents.
        assert_eq!(
            umsatzsteuer_cents(10_000, Steuersatz::ErmaessigterSatz),
            700
        );
    }

    #[test]
    fn umsatzsteuer_steuerfrei_is_zero() {
        assert_eq!(umsatzsteuer_cents(10_000, Steuersatz::Steuerfrei), 0);
    }

    #[test]
    fn brutto_regelsatz_on_100_euro() {
        // 100,00 EUR net + 19,00 EUR VAT = 119,00 EUR = 11 900 cents.
        assert_eq!(brutto_cents(10_000, Steuersatz::Regelsteuersatz), 11_900);
    }

    #[test]
    fn netto_aus_brutto_regelsatz() {
        // 119,00 EUR gross at 19 % -> 100,00 EUR net = 10 000 cents.
        assert_eq!(
            netto_aus_brutto_cents(11_900, Steuersatz::Regelsteuersatz),
            10_000
        );
    }

    #[test]
    fn netto_aus_brutto_ermaessigt() {
        // 107,00 EUR gross at 7 % -> 100,00 EUR net = 10 000 cents.
        assert_eq!(
            netto_aus_brutto_cents(10_700, Steuersatz::ErmaessigterSatz),
            10_000
        );
    }

    #[test]
    fn steuersatz_serde_roundtrip() {
        let s = Steuersatz::Regelsteuersatz;
        let json = serde_json::to_string(&s).expect("serialize Steuersatz");
        let back: Steuersatz = serde_json::from_str(&json).expect("deserialize Steuersatz");
        assert_eq!(s, back);
    }
}
