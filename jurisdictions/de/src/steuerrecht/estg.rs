//! EStG - Income Tax Act (Einkommensteuergesetz)
//!
//! Type-safe representations of the seven types of income (Einkunftsarten,
//! § 2 Abs. 1 EStG) and the income-tax tariff (Einkommensteuertarif, § 32a EStG).
//!
//! Typsichere Abbildung der sieben Einkunftsarten (§ 2 Abs. 1 EStG) und des
//! Einkommensteuertarifs (§ 32a EStG).
//!
//! ## § 2 EStG - Umfang der Besteuerung
//!
//! > (1) Der Einkommensteuer unterliegen [...] Einkünfte aus
//! > 1. Land- und Forstwirtschaft, 2. Gewerbebetrieb, 3. selbständiger Arbeit,
//! > 4. nichtselbständiger Arbeit, 5. Kapitalvermögen,
//! > 6. Vermietung und Verpachtung, 7. sonstige Einkünfte [...].
//!
//! **English**: Income tax is levied on the seven types of income enumerated in
//! § 2 Abs. 1 EStG. The first three (agriculture/forestry, trade, self-employment)
//! are **profit income** (Gewinneinkünfte); the remaining four are **surplus
//! income** (Überschusseinkünfte), § 2 Abs. 2 EStG.
//!
//! ## § 32a EStG - Einkommensteuertarif
//!
//! The tariff is a zone-based piecewise function of the taxable income (zu
//! versteuerndes Einkommen, zvE). This module encodes the **2023** tariff
//! (see [`STEUERJAHR`]); other years are rejected by [`einkommensteuer`].

use serde::{Deserialize, Serialize};

use crate::steuerrecht::error::{Result, SteuerError};

/// The tax year whose tariff is encoded in this module (§ 32a EStG 2023).
///
/// Das Steuerjahr, dessen Tarif hier abgebildet ist.
pub const STEUERJAHR: i32 = 2023;

/// Basic tax-free allowance for 2023 (Grundfreibetrag), § 32a Abs. 1 Nr. 1 EStG.
///
/// Grundfreibetrag 2023 in vollen Euro.
pub const GRUNDFREIBETRAG_2023_EUR: u64 = 10_908;

// =============================================================================
// § 2 Abs. 1 EStG - The seven types of income (Einkunftsarten)
// =============================================================================

/// The seven types of income under § 2 Abs. 1 EStG (Einkunftsarten).
///
/// Die sieben Einkunftsarten des § 2 Abs. 1 EStG.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Einkunftsart {
    /// Income from agriculture and forestry (§ 13 EStG).
    ///
    /// Einkünfte aus Land- und Forstwirtschaft.
    LandUndForstwirtschaft,
    /// Income from a trade or business (§ 15 EStG).
    ///
    /// Einkünfte aus Gewerbebetrieb.
    Gewerbebetrieb,
    /// Income from self-employment (§ 18 EStG).
    ///
    /// Einkünfte aus selbständiger Arbeit.
    SelbstaendigeArbeit,
    /// Income from employment (§ 19 EStG).
    ///
    /// Einkünfte aus nichtselbständiger Arbeit.
    NichtselbstaendigeArbeit,
    /// Income from capital assets (§ 20 EStG).
    ///
    /// Einkünfte aus Kapitalvermögen.
    Kapitalvermoegen,
    /// Income from letting and leasing (§ 21 EStG).
    ///
    /// Einkünfte aus Vermietung und Verpachtung.
    VermietungUndVerpachtung,
    /// Other income (§ 22 EStG).
    ///
    /// Sonstige Einkünfte.
    Sonstige,
}

impl Einkunftsart {
    /// The § citation of the income type.
    ///
    /// Der Paragraphenverweis der Einkunftsart.
    #[must_use]
    pub fn paragraph(&self) -> &'static str {
        match self {
            Einkunftsart::LandUndForstwirtschaft => "§ 13 EStG",
            Einkunftsart::Gewerbebetrieb => "§ 15 EStG",
            Einkunftsart::SelbstaendigeArbeit => "§ 18 EStG",
            Einkunftsart::NichtselbstaendigeArbeit => "§ 19 EStG",
            Einkunftsart::Kapitalvermoegen => "§ 20 EStG",
            Einkunftsart::VermietungUndVerpachtung => "§ 21 EStG",
            Einkunftsart::Sonstige => "§ 22 EStG",
        }
    }

    /// Whether this is profit income (Gewinneinkunftsart), § 2 Abs. 2 Nr. 1 EStG.
    ///
    /// The first three types (agriculture/forestry, trade, self-employment) are
    /// profit income (Gewinneinkünfte); the remaining four are surplus income
    /// (Überschusseinkünfte).
    ///
    /// Gibt an, ob es sich um eine Gewinneinkunftsart handelt (§ 2 Abs. 2 EStG).
    #[must_use]
    pub fn is_gewinneinkunftsart(&self) -> bool {
        matches!(
            self,
            Einkunftsart::LandUndForstwirtschaft
                | Einkunftsart::Gewerbebetrieb
                | Einkunftsart::SelbstaendigeArbeit
        )
    }
}

// =============================================================================
// § 2 Abs. 3 EStG - Income (Einkünfte) and their aggregation
// =============================================================================

/// Income of a single type (Einkünfte einer Einkunftsart), § 2 Abs. 2 EStG.
///
/// The amount is stored in EUR cents and may be **negative** (a loss, Verlust).
///
/// Einkünfte einer einzelnen Einkunftsart; der Betrag wird in Euro-Cent
/// gespeichert und kann negativ sein (Verlust).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Einkuenfte {
    /// The type of income (Einkunftsart).
    pub art: Einkunftsart,
    /// The amount in EUR cents; negative values denote a loss (Verlust).
    pub betrag_cents: i64,
}

impl Einkuenfte {
    /// Create income of a given type from an amount in EUR cents.
    ///
    /// Erzeugt Einkünfte einer Einkunftsart aus einem Betrag in Euro-Cent.
    #[must_use]
    pub fn new(art: Einkunftsart, betrag_cents: i64) -> Self {
        Self { art, betrag_cents }
    }

    /// Create income of a given type from an amount in whole EUR.
    ///
    /// Erzeugt Einkünfte aus einem Betrag in vollen Euro.
    #[must_use]
    pub fn from_euros(art: Einkunftsart, euros: i64) -> Self {
        Self {
            art,
            betrag_cents: euros.saturating_mul(100),
        }
    }

    /// Convert the amount to EUR as `f64` (for display only).
    ///
    /// Wandelt den Betrag zur Anzeige in Euro (`f64`) um.
    #[must_use]
    pub fn to_euros(&self) -> f64 {
        // Display-only helper; the stored value remains integer cents.
        f64::from(i32::try_from(self.betrag_cents).unwrap_or(i32::MAX)) / 100.0
    }
}

/// Aggregated income across all income types (Summe der Einkünfte), § 2 EStG.
///
/// Gesamtheit der Einkünfte über alle Einkunftsarten hinweg.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Einkommen {
    /// The individual income items per type (Einkünfte je Einkunftsart).
    pub einkuenfte: Vec<Einkuenfte>,
}

impl Einkommen {
    /// Create an empty income aggregation.
    ///
    /// Erzeugt eine leere Einkünftezusammenstellung.
    #[must_use]
    pub fn new() -> Self {
        Self {
            einkuenfte: Vec::new(),
        }
    }

    /// Create from a list of income items.
    ///
    /// Erzeugt aus einer Liste von Einkünften.
    #[must_use]
    pub fn with_einkuenfte(einkuenfte: Vec<Einkuenfte>) -> Self {
        Self { einkuenfte }
    }

    /// Add an income item.
    ///
    /// Fügt Einkünfte hinzu.
    pub fn push(&mut self, e: Einkuenfte) {
        self.einkuenfte.push(e);
    }

    /// Sum of all income (Summe der Einkünfte) in EUR cents, § 2 Abs. 3 EStG.
    ///
    /// Losses (negative amounts) reduce the sum (horizontaler/vertikaler
    /// Verlustausgleich, simplified). The result may be negative.
    ///
    /// Summe der Einkünfte in Euro-Cent (§ 2 Abs. 3 EStG); Verluste mindern die
    /// Summe. Das Ergebnis kann negativ sein.
    #[must_use]
    pub fn summe_der_einkuenfte(&self) -> i64 {
        self.einkuenfte
            .iter()
            .fold(0_i64, |acc, e| acc.saturating_add(e.betrag_cents))
    }
}

/// Compute the taxable income (zu versteuerndes Einkommen, zvE), § 2 Abs. 5 EStG.
///
/// Simplified model: the deductions (Sonderausgaben, Freibeträge, …) are
/// subtracted from the sum of income, with a floor at 0.
///
/// Both values are in the same unit (EUR cents recommended).
///
/// Vereinfachte Ermittlung des zu versteuernden Einkommens: Abzüge werden von
/// der Summe der Einkünfte abgezogen, jedoch nicht unter 0.
#[must_use]
pub fn zu_versteuerndes_einkommen(summe: i64, abzuege: i64) -> i64 {
    (summe.saturating_sub(abzuege)).max(0)
}

// =============================================================================
// § 32a EStG - Income tax tariff (Einkommensteuertarif), 2023
// =============================================================================

/// Compute the 2023 income tax (Einkommensteuer) under § 32a Abs. 1 EStG.
///
/// `zve_euros` is the taxable income (zu versteuerndes Einkommen) in whole EUR.
/// The result is the income tax in whole EUR, **rounded down** to full euros as
/// required by § 32a (the tariff is applied to the euro amount and the resulting
/// tax is truncated).
///
/// The official 2023 zones (§ 32a Abs. 1 EStG):
/// - `x <= 10908`: 0 (Grundfreibetrag)
/// - `10909..=15999`: `(979.18*y + 1400)*y` with `y = (x-10908)/10000`
/// - `16000..=62809`: `(192.59*z + 2397)*z + 966.53` with `z = (x-15999)/10000`
/// - `62810..=277825`: `0.42*x - 9972.98`
/// - `x >= 277826`: `0.45*x - 18307.73`
///
/// The `f64` arithmetic is purely internal; the public signature is integer.
///
/// Berechnet die Einkommensteuer 2023 nach § 32a Abs. 1 EStG (Eingabe und
/// Ausgabe in vollen Euro, Abrundung der Steuer auf volle Euro).
#[must_use]
pub fn einkommensteuer_2023(zve_euros: u64) -> u64 {
    // Grundfreibetrag: no tax up to and including 10 908 EUR.
    if zve_euros <= GRUNDFREIBETRAG_2023_EUR {
        return 0;
    }

    // Internal floating-point computation of the tariff; the value of x is a
    // whole-euro income that fits comfortably into f64 (< 2^53), so the
    // conversion is exact for any realistic income.
    #[allow(
        clippy::cast_precision_loss,
        reason = "zve_euros is a whole-euro income well below 2^53, so the f64 conversion is exact"
    )]
    let x = zve_euros as f64;

    let tax = if zve_euros <= 15_999 {
        // Progressionszone I.
        let y = (x - 10_908.0) / 10_000.0;
        (979.18 * y + 1_400.0) * y
    } else if zve_euros <= 62_809 {
        // Progressionszone II.
        let z = (x - 15_999.0) / 10_000.0;
        (192.59 * z + 2_397.0) * z + 966.53
    } else if zve_euros <= 277_825 {
        // Proportionalzone I (Spitzensteuersatz 42 %).
        0.42 * x - 9_972.98
    } else {
        // Proportionalzone II (Reichensteuer 45 %).
        0.45 * x - 18_307.73
    };

    // § 32a: round the resulting tax down to full euros. `tax` is non-negative
    // in every zone for x > Grundfreibetrag and is bounded by 0.45*x, far below
    // u64::MAX for any realistic income, so the truncating cast is safe.
    let tax_floor = tax.floor().max(0.0);
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "tax_floor is a non-negative, integral, finite euro amount well below u64::MAX"
    )]
    let result = tax_floor as u64;
    result
}

/// Compute the income tax (Einkommensteuer) for a given year, § 32a EStG.
///
/// Only the **2023** tariff is encoded (see [`STEUERJAHR`]). For any other year
/// this returns [`SteuerError::InvalidTaxYear`]. This is a deliberate
/// limitation: the per-year tariff coefficients change annually.
///
/// Berechnet die Einkommensteuer für ein Jahr. Nur der Tarif **2023** ist
/// hinterlegt; andere Jahre liefern [`SteuerError::InvalidTaxYear`].
///
/// # Errors
/// - [`SteuerError::InvalidTaxYear`] if `jahr != STEUERJAHR`.
pub fn einkommensteuer(zve_euros: u64, jahr: i32) -> Result<u64> {
    if jahr != STEUERJAHR {
        return Err(SteuerError::InvalidTaxYear { year: jahr });
    }
    Ok(einkommensteuer_2023(zve_euros))
}

/// Name of the tariff zone for a taxable income, § 32a Abs. 1 EStG (2023).
///
/// Returns one of `"Grundfreibetrag"`, `"Progressionszone I"`,
/// `"Progressionszone II"`, `"Proportionalzone I"`, `"Spitzensteuersatz"`.
///
/// Liefert den Namen der Tarifzone für ein zu versteuerndes Einkommen.
#[must_use]
pub fn grenzsteuersatz_zone(zve_euros: u64) -> &'static str {
    if zve_euros <= GRUNDFREIBETRAG_2023_EUR {
        "Grundfreibetrag"
    } else if zve_euros <= 15_999 {
        "Progressionszone I"
    } else if zve_euros <= 62_809 {
        "Progressionszone II"
    } else if zve_euros <= 277_825 {
        "Proportionalzone I"
    } else {
        "Spitzensteuersatz"
    }
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seven_einkunftsarten_paragraphs() {
        assert_eq!(
            Einkunftsart::LandUndForstwirtschaft.paragraph(),
            "§ 13 EStG"
        );
        assert_eq!(Einkunftsart::Gewerbebetrieb.paragraph(), "§ 15 EStG");
        assert_eq!(Einkunftsart::SelbstaendigeArbeit.paragraph(), "§ 18 EStG");
        assert_eq!(
            Einkunftsart::NichtselbstaendigeArbeit.paragraph(),
            "§ 19 EStG"
        );
        assert_eq!(Einkunftsart::Kapitalvermoegen.paragraph(), "§ 20 EStG");
        assert_eq!(
            Einkunftsart::VermietungUndVerpachtung.paragraph(),
            "§ 21 EStG"
        );
        assert_eq!(Einkunftsart::Sonstige.paragraph(), "§ 22 EStG");
    }

    #[test]
    fn gewinneinkunftsart_classification() {
        // The first three are profit income (Gewinneinkünfte).
        assert!(Einkunftsart::LandUndForstwirtschaft.is_gewinneinkunftsart());
        assert!(Einkunftsart::Gewerbebetrieb.is_gewinneinkunftsart());
        assert!(Einkunftsart::SelbstaendigeArbeit.is_gewinneinkunftsart());
        // The remaining four are surplus income (Überschusseinkünfte).
        assert!(!Einkunftsart::NichtselbstaendigeArbeit.is_gewinneinkunftsart());
        assert!(!Einkunftsart::Kapitalvermoegen.is_gewinneinkunftsart());
        assert!(!Einkunftsart::VermietungUndVerpachtung.is_gewinneinkunftsart());
        assert!(!Einkunftsart::Sonstige.is_gewinneinkunftsart());
    }

    #[test]
    fn einkunftsart_serde_roundtrip() {
        let art = Einkunftsart::Gewerbebetrieb;
        let json = serde_json::to_string(&art).expect("serialize Einkunftsart");
        let back: Einkunftsart = serde_json::from_str(&json).expect("deserialize Einkunftsart");
        assert_eq!(art, back);
    }

    #[test]
    fn einkuenfte_to_euros() {
        let e = Einkuenfte::from_euros(Einkunftsart::NichtselbstaendigeArbeit, 50_000);
        assert_eq!(e.betrag_cents, 5_000_000);
        assert!((e.to_euros() - 50_000.0).abs() < 1e-6);
    }

    #[test]
    fn summe_der_einkuenfte_positive() {
        let income = Einkommen::with_einkuenfte(vec![
            Einkuenfte::from_euros(Einkunftsart::NichtselbstaendigeArbeit, 40_000),
            Einkuenfte::from_euros(Einkunftsart::Kapitalvermoegen, 5_000),
        ]);
        assert_eq!(income.summe_der_einkuenfte(), 4_500_000);
    }

    #[test]
    fn summe_der_einkuenfte_with_loss() {
        // A loss (negative income) reduces the sum (§ 2 Abs. 3 EStG).
        let mut income = Einkommen::new();
        income.push(Einkuenfte::from_euros(
            Einkunftsart::NichtselbstaendigeArbeit,
            40_000,
        ));
        income.push(Einkuenfte::from_euros(
            Einkunftsart::Gewerbebetrieb,
            -10_000,
        ));
        assert_eq!(income.summe_der_einkuenfte(), 3_000_000);
    }

    #[test]
    fn empty_einkommen_is_zero() {
        let income = Einkommen::default();
        assert_eq!(income.summe_der_einkuenfte(), 0);
    }

    #[test]
    fn zve_normal_subtraction() {
        // 30 000 EUR income minus 6 000 EUR deductions = 24 000 EUR (in cents).
        assert_eq!(zu_versteuerndes_einkommen(3_000_000, 600_000), 2_400_000);
    }

    #[test]
    fn zve_floor_at_zero() {
        // Deductions exceeding income floor the zvE at 0.
        assert_eq!(zu_versteuerndes_einkommen(500_000, 800_000), 0);
        assert_eq!(zu_versteuerndes_einkommen(-100, 0), 0);
    }

    #[test]
    fn tax_zero_in_grundfreibetrag() {
        assert_eq!(einkommensteuer_2023(0), 0);
        assert_eq!(einkommensteuer_2023(5_000), 0);
        // Boundary: exactly the Grundfreibetrag is still tax-free.
        assert_eq!(einkommensteuer_2023(GRUNDFREIBETRAG_2023_EUR), 0);
        assert_eq!(einkommensteuer_2023(10_908), 0);
    }

    #[test]
    fn tax_progression_positive_and_monotonic() {
        // Just above the Grundfreibetrag the tax becomes positive.
        let t1 = einkommensteuer_2023(10_909);
        // A small positive (or zero after flooring just past the threshold) value
        // followed by clearly increasing tax across the progression zone.
        let t20 = einkommensteuer_2023(20_000);
        let t40 = einkommensteuer_2023(40_000);
        assert!(t20 > 0);
        assert!(t1 <= t20);
        // Strict monotonicity across the progression zone.
        assert!(t20 < t40);
    }

    #[test]
    fn tax_proportionalzone_exact() {
        // 100 000 EUR: floor(0.42*100000 - 9972.98) = floor(32027.02) = 32027.
        assert_eq!(einkommensteuer_2023(100_000), 32_027);
    }

    #[test]
    fn tax_spitzensteuersatz_exact() {
        // 300 000 EUR: floor(0.45*300000 - 18307.73) = floor(116692.27) = 116692.
        assert_eq!(einkommensteuer_2023(300_000), 116_692);
    }

    #[test]
    fn einkommensteuer_wrapper_year_2023_ok() {
        let t = einkommensteuer(100_000, 2023).expect("2023 tariff is encoded");
        assert_eq!(t, 32_027);
    }

    #[test]
    fn einkommensteuer_wrapper_rejects_2022() {
        let r = einkommensteuer(100_000, 2022);
        assert!(matches!(r, Err(SteuerError::InvalidTaxYear { year: 2022 })));
    }

    #[test]
    fn grenzsteuersatz_zone_names() {
        assert_eq!(grenzsteuersatz_zone(5_000), "Grundfreibetrag");
        assert_eq!(grenzsteuersatz_zone(10_908), "Grundfreibetrag");
        assert_eq!(grenzsteuersatz_zone(12_000), "Progressionszone I");
        assert_eq!(grenzsteuersatz_zone(30_000), "Progressionszone II");
        assert_eq!(grenzsteuersatz_zone(100_000), "Proportionalzone I");
        assert_eq!(grenzsteuersatz_zone(300_000), "Spitzensteuersatz");
    }
}
