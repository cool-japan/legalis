//! AO - Fiscal Code (Abgabenordnung)
//!
//! Type-safe representations of the tax assessment notice (Steuerbescheid,
//! §§ 155, 157, 124 AO), the limitation of assessment (Festsetzungsverjährung,
//! §§ 169-171 AO), the objection period (Einspruchsfrist, § 355 AO) and interest
//! on arrears (Nachzahlungszinsen, § 233a AO).
//!
//! Typsichere Abbildung des Steuerbescheids (§§ 155, 157, 124 AO), der
//! Festsetzungsverjährung (§§ 169-171 AO), der Einspruchsfrist (§ 355 AO) und der
//! Nachzahlungszinsen (§ 233a AO).
//!
//! ## § 124 AO - Wirksamkeit des Verwaltungsakts
//!
//! A tax assessment notice (Steuerbescheid) becomes **effective** (wirksam) only
//! upon disclosure (Bekanntgabe) to the addressee, § 124 Abs. 1 AO.
//!
//! ## §§ 169-171 AO - Festsetzungsverjährung
//!
//! The period for assessment (Festsetzungsfrist) is generally **4 years**
//! (§ 169 Abs. 2 Nr. 2 AO), **1 year** for excise duties (Verbrauchsteuern),
//! **10 years** for tax evasion (Steuerhinterziehung) and **5 years** for
//! reckless tax understatement (leichtfertige Steuerverkürzung), § 169 Abs. 2
//! S. 2 AO. It begins (Anlauf) with the end of the calendar year in which the
//! tax arose (simplified from § 170 AO).
//!
//! ## § 355 AO - Einspruchsfrist
//!
//! An objection (Einspruch) must be filed within **one month** of the
//! disclosure (Bekanntgabe) of the notice.
//!
//! ## § 233a AO - Verzinsung
//!
//! Interest on arrears (Nachzahlungszinsen) accrues at **0.5 % per full month**
//! (6 % p.a.) after a 15-month grace period (Karenzzeit).

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::steuerrecht::error::{Result, SteuerError};

// =============================================================================
// § 124, §§ 155, 157 AO - Tax assessment notice (Steuerbescheid)
// =============================================================================

/// A tax assessment notice (Steuerbescheid), §§ 155, 157 AO.
///
/// Its effectiveness (Wirksamkeit) depends on disclosure (Bekanntgabe), § 124 AO.
/// The assessed tax (festgesetzte Steuer) is stored in EUR cents.
///
/// Ein Steuerbescheid (§§ 155, 157 AO); die Wirksamkeit hängt von der Bekanntgabe
/// ab (§ 124 AO). Die festgesetzte Steuer wird in Euro-Cent gespeichert.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Steuerbescheid {
    /// The type of tax (Steuerart), e.g. "Einkommensteuer".
    pub steuerart: String,
    /// The assessed tax amount in EUR cents (festgesetzte Steuer).
    pub festgesetzte_steuer_cents: u64,
    /// Whether the notice has been disclosed (bekanntgegeben), § 124 Abs. 1 AO.
    pub bekanntgegeben: bool,
    /// The date of disclosure (Bekanntgabedatum), if known.
    pub bekanntgabe_datum: Option<NaiveDate>,
}

/// Check whether a tax assessment notice is effective (wirksam), § 124 Abs. 1 AO.
///
/// A notice is effective only once it has been disclosed (bekanntgegeben).
///
/// Prüft, ob ein Steuerbescheid wirksam ist (§ 124 Abs. 1 AO); wirksam erst nach
/// Bekanntgabe.
///
/// # Errors
/// - [`SteuerError::BescheidNichtWirksam`] if the notice has not been disclosed.
pub fn ist_wirksam(b: &Steuerbescheid) -> Result<()> {
    if !b.bekanntgegeben {
        return Err(SteuerError::BescheidNichtWirksam);
    }
    Ok(())
}

// =============================================================================
// §§ 169-171 AO - Limitation of assessment (Festsetzungsverjährung)
// =============================================================================

/// The category of tax for the purpose of the assessment period, § 169 AO.
///
/// Die Steuerart im Hinblick auf die Festsetzungsfrist (§ 169 AO).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Steuerart {
    /// Ordinary tax: 4-year period (§ 169 Abs. 2 Nr. 2 AO).
    ///
    /// Normalfall: Festsetzungsfrist 4 Jahre.
    Normalsteuer,
    /// Excise duties: 1-year period (§ 169 Abs. 2 Nr. 1 AO).
    ///
    /// Verbrauchsteuern: Festsetzungsfrist 1 Jahr.
    Verbrauchsteuer,
    /// Tax evasion: 10-year period (§ 169 Abs. 2 S. 2 AO).
    ///
    /// Steuerhinterziehung: Festsetzungsfrist 10 Jahre.
    Steuerhinterziehung,
    /// Reckless tax understatement: 5-year period (§ 169 Abs. 2 S. 2 AO).
    ///
    /// Leichtfertige Steuerverkürzung: Festsetzungsfrist 5 Jahre.
    LeichtfertigeVerkuerzung,
}

impl Steuerart {
    /// The assessment period (Festsetzungsfrist) in years, § 169 Abs. 2 AO.
    ///
    /// `4` ordinary, `1` excise duties, `10` tax evasion, `5` reckless
    /// understatement.
    ///
    /// Die Festsetzungsfrist in Jahren (§ 169 Abs. 2 AO).
    #[must_use]
    pub fn festsetzungsfrist_jahre(&self) -> u32 {
        match self {
            Steuerart::Normalsteuer => 4,
            Steuerart::Verbrauchsteuer => 1,
            Steuerart::Steuerhinterziehung => 10,
            Steuerart::LeichtfertigeVerkuerzung => 5,
        }
    }
}

/// The calendar year at the end of which the assessment period expires,
/// §§ 169-170 AO.
///
/// Since the period begins (Anlauf) with the end of `entstehungsjahr` (the year
/// in which the tax arose, simplified from § 170 AO), the period expires at the
/// end of `entstehungsjahr + festsetzungsfrist_jahre`.
///
/// Das Kalenderjahr, mit dessen Ablauf die Festsetzungsfrist endet
/// (`entstehungsjahr + Frist`).
#[must_use]
pub fn festsetzungsverjaehrung(entstehungsjahr: i32, art: Steuerart) -> i32 {
    let frist = i32::try_from(art.festsetzungsfrist_jahre()).unwrap_or(0);
    entstehungsjahr.saturating_add(frist)
}

/// Check whether assessment is time-barred (festsetzungsverjährt), §§ 169-171 AO.
///
/// Assessment is barred once the current year (`heutiges_jahr`) is **after** the
/// year at the end of which the period expires
/// ([`festsetzungsverjaehrung`]).
///
/// Prüft, ob Festsetzungsverjährung eingetreten ist (§§ 169-171 AO).
///
/// # Errors
/// - [`SteuerError::FestsetzungsfristAbgelaufen`] if the period has expired; the
///   error carries the applicable period in years.
pub fn ist_festsetzung_verjaehrt(
    entstehungsjahr: i32,
    art: Steuerart,
    heutiges_jahr: i32,
) -> Result<()> {
    let verjaehrungsjahr = festsetzungsverjaehrung(entstehungsjahr, art);
    if heutiges_jahr > verjaehrungsjahr {
        return Err(SteuerError::FestsetzungsfristAbgelaufen {
            years: art.festsetzungsfrist_jahre(),
        });
    }
    Ok(())
}

// =============================================================================
// § 355 AO - Objection period (Einspruchsfrist)
// =============================================================================

/// Approximate number of days for the one-month objection period, § 355 AO.
///
/// The statutory period is "one month"; this module approximates it as 30 days
/// (see [`pruefe_einspruch`]).
///
/// Näherung der Einmonatsfrist des § 355 AO in Tagen.
pub const EINSPRUCHSFRIST_TAGE: i64 = 30;

/// An objection (Einspruch) against a tax assessment notice, § 355 AO.
///
/// Ein Einspruch gegen einen Steuerbescheid (§ 355 AO).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Einspruch {
    /// The date the objection was filed (Tag der Einlegung).
    pub eingelegt_am: NaiveDate,
    /// The date the notice was disclosed (Tag der Bekanntgabe).
    pub bekanntgabe_am: NaiveDate,
}

/// Check whether an objection was filed in time, § 355 AO.
///
/// The statutory objection period is **one month** after disclosure. This
/// function approximates the month as **30 days**: if more than
/// [`EINSPRUCHSFRIST_TAGE`] days elapsed between disclosure and filing, the
/// objection is out of time (verfristet). The day count is exposed in the error.
///
/// Prüft, ob ein Einspruch fristgerecht eingelegt wurde (§ 355 AO); die
/// Einmonatsfrist wird mit 30 Tagen angenähert.
///
/// # Errors
/// - [`SteuerError::EinspruchVerfristet`] if more than [`EINSPRUCHSFRIST_TAGE`]
///   days elapsed between disclosure and filing; the error carries the day count.
pub fn pruefe_einspruch(e: &Einspruch) -> Result<()> {
    let tage = (e.eingelegt_am - e.bekanntgabe_am).num_days();
    if tage > EINSPRUCHSFRIST_TAGE {
        return Err(SteuerError::EinspruchVerfristet { days: tage });
    }
    Ok(())
}

// =============================================================================
// § 233a AO - Interest on arrears (Nachzahlungszinsen)
// =============================================================================

/// Compute interest on arrears (Nachzahlungszinsen) in EUR cents, § 233a AO.
///
/// Simplified model: **0.5 % per full month** of the assessed tax, i.e.
/// `steuer_cents * 5 * volle_monate / 1000`. The 15-month grace period
/// (Karenzzeit) is assumed already accounted for in `volle_monate` (the count of
/// full interest-bearing months). Integer arithmetic in `u128`; the division
/// truncates (rounds **down**).
///
/// Note: the reduced rate applicable to interest periods from 2019 onward
/// (following the Federal Constitutional Court's ruling) is **not** modelled.
///
/// Berechnet Nachzahlungszinsen in Euro-Cent (§ 233a AO): 0,5 % je vollem Monat.
/// Der ab 2019 geltende ermäßigte Zinssatz ist nicht abgebildet.
#[must_use]
pub fn nachzahlungszinsen_cents(steuer_cents: u64, volle_monate: u32) -> u64 {
    // 0.5 % per month = steuer * 5 / 1000 per month.
    let zinsen = u128::from(steuer_cents) * 5 * u128::from(volle_monate) / 1000;
    u64::try_from(zinsen).unwrap_or(u64::MAX)
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).expect("valid test date")
    }

    #[test]
    fn bescheid_wirksam_after_bekanntgabe() {
        let b = Steuerbescheid {
            steuerart: "Einkommensteuer".to_string(),
            festgesetzte_steuer_cents: 500_000,
            bekanntgegeben: true,
            bekanntgabe_datum: Some(date(2023, 5, 1)),
        };
        assert!(ist_wirksam(&b).is_ok());
    }

    #[test]
    fn bescheid_nicht_wirksam_ohne_bekanntgabe() {
        let b = Steuerbescheid {
            steuerart: "Einkommensteuer".to_string(),
            festgesetzte_steuer_cents: 500_000,
            bekanntgegeben: false,
            bekanntgabe_datum: None,
        };
        assert!(matches!(
            ist_wirksam(&b),
            Err(SteuerError::BescheidNichtWirksam)
        ));
    }

    #[test]
    fn festsetzungsfrist_jahre_per_steuerart() {
        assert_eq!(Steuerart::Normalsteuer.festsetzungsfrist_jahre(), 4);
        assert_eq!(Steuerart::Verbrauchsteuer.festsetzungsfrist_jahre(), 1);
        assert_eq!(Steuerart::Steuerhinterziehung.festsetzungsfrist_jahre(), 10);
        assert_eq!(
            Steuerart::LeichtfertigeVerkuerzung.festsetzungsfrist_jahre(),
            5
        );
    }

    #[test]
    fn festsetzungsverjaehrung_year_arithmetic() {
        // Tax arising in 2018, 4-year period -> expires end of 2022.
        assert_eq!(festsetzungsverjaehrung(2018, Steuerart::Normalsteuer), 2022);
        // Tax evasion: 10-year period -> expires end of 2028.
        assert_eq!(
            festsetzungsverjaehrung(2018, Steuerart::Steuerhinterziehung),
            2028
        );
    }

    #[test]
    fn festsetzung_not_yet_verjaehrt_at_boundary() {
        // Normalsteuer, arising 2018 -> verjährt end of 2022. In 2022 itself it
        // is NOT yet barred.
        assert!(ist_festsetzung_verjaehrt(2018, Steuerart::Normalsteuer, 2022).is_ok());
    }

    #[test]
    fn festsetzung_verjaehrt_after_boundary() {
        // In 2023 the 2018 ordinary tax IS barred.
        let r = ist_festsetzung_verjaehrt(2018, Steuerart::Normalsteuer, 2023);
        assert!(matches!(
            r,
            Err(SteuerError::FestsetzungsfristAbgelaufen { years: 4 })
        ));
    }

    #[test]
    fn festsetzung_steuerhinterziehung_long_period() {
        // 2018 evasion -> verjährt end of 2028; 2028 not barred, 2029 barred.
        assert!(ist_festsetzung_verjaehrt(2018, Steuerart::Steuerhinterziehung, 2028).is_ok());
        assert!(matches!(
            ist_festsetzung_verjaehrt(2018, Steuerart::Steuerhinterziehung, 2029),
            Err(SteuerError::FestsetzungsfristAbgelaufen { years: 10 })
        ));
    }

    #[test]
    fn einspruch_in_time() {
        // 20 days after disclosure: in time.
        let e = Einspruch {
            bekanntgabe_am: date(2023, 5, 1),
            eingelegt_am: date(2023, 5, 21),
        };
        assert!(pruefe_einspruch(&e).is_ok());
    }

    #[test]
    fn einspruch_exactly_30_days_in_time() {
        // Exactly 30 days: still in time (boundary of the approximation).
        let e = Einspruch {
            bekanntgabe_am: date(2023, 5, 1),
            eingelegt_am: date(2023, 5, 31),
        };
        assert!(pruefe_einspruch(&e).is_ok());
    }

    #[test]
    fn einspruch_verfristet() {
        // 45 days after disclosure: out of time, error carries the day count.
        let e = Einspruch {
            bekanntgabe_am: date(2023, 5, 1),
            eingelegt_am: date(2023, 6, 15),
        };
        let r = pruefe_einspruch(&e);
        assert!(matches!(
            r,
            Err(SteuerError::EinspruchVerfristet { days: 45 })
        ));
    }

    #[test]
    fn nachzahlungszinsen_zero_months() {
        assert_eq!(nachzahlungszinsen_cents(1_000_000, 0), 0);
    }

    #[test]
    fn nachzahlungszinsen_computation() {
        // 10 000,00 EUR (1 000 000 cents), 12 full months at 0.5 %/month = 6 %.
        // 1 000 000 * 5 * 12 / 1000 = 60 000 cents = 600,00 EUR.
        assert_eq!(nachzahlungszinsen_cents(1_000_000, 12), 60_000);
    }

    #[test]
    fn nachzahlungszinsen_single_month() {
        // 1 000,00 EUR (100 000 cents), 1 month at 0.5 % = 5,00 EUR = 500 cents.
        assert_eq!(nachzahlungszinsen_cents(100_000, 1), 500);
    }

    #[test]
    fn steuerart_serde_roundtrip() {
        let a = Steuerart::Steuerhinterziehung;
        let json = serde_json::to_string(&a).expect("serialize Steuerart");
        let back: Steuerart = serde_json::from_str(&json).expect("deserialize Steuerart");
        assert_eq!(a, back);
    }
}
