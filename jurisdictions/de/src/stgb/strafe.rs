//! StGB Penalties and Sentencing Framework (Strafen und Strafrahmen)
//!
//! Shared types for representing German criminal sanctions under the
//! Strafgesetzbuch (StGB), §§ 38-43, used by both the General Part
//! (Allgemeiner Teil) and the Special Part (Besonderer Teil).
//!
//! # Legal Context (Rechtlicher Kontext)
//!
//! German criminal law distinguishes two principal sanctions (Hauptstrafen):
//!
//! - **Freiheitsstrafe** (imprisonment) - §§ 38-39 StGB
//! - **Geldstrafe** (fine, expressed in daily units / Tagessätze) - §§ 40-43 StGB
//!
//! ## § 38 StGB - Dauer der Freiheitsstrafe (Duration of imprisonment)
//!
//! - Freiheitsstrafe is either **time-limited** (zeitig) or **for life** (lebenslang).
//! - The general statutory range for time-limited imprisonment is **one month to
//!   fifteen years** (§ 38 Abs. 2 StGB), unless a specific provision orders life.
//!
//! ## § 39 StGB - Bemessung der Freiheitsstrafe (Calculation)
//!
//! - Imprisonment of **less than one year** is measured in full weeks and months.
//! - Imprisonment of **a longer duration** is measured in full months and years.
//!
//! ## § 40 StGB - Verhängung der Geldstrafe in Tagessätzen (Day-fine system)
//!
//! - A fine is imposed in **daily units (Tagessätze)**: minimum **5**, maximum
//!   **360** full daily units (§ 40 Abs. 1 StGB).
//! - The amount of a single daily unit is **at least 1 EUR and at most 30,000 EUR**
//!   (§ 40 Abs. 2 S. 3 StGB), set according to the offender's net daily income.
//!
//! ## § 41 StGB - Geldstrafe neben Freiheitsstrafe
//!
//! A fine may be imposed **in addition** to imprisonment where the offender
//! enriched (or tried to enrich) himself through the offence.
//!
//! ## § 43 StGB - Ersatzfreiheitsstrafe (Default imprisonment)
//!
//! Where a fine cannot be recovered, it is converted into imprisonment: **one day
//! of imprisonment corresponds to one daily unit** (§ 43 S. 2 StGB).

use serde::{Deserialize, Serialize};

use super::error::{Result, StgbError};

/// Minimum number of daily units of a fine (§ 40 Abs. 1 S. 2 StGB).
pub const TAGESSAETZE_MIN: u32 = 5;
/// Maximum number of daily units of a single fine (§ 40 Abs. 1 S. 2 StGB).
pub const TAGESSAETZE_MAX: u32 = 360;
/// Minimum value of a single daily unit in EUR cents (§ 40 Abs. 2 S. 3 StGB): 1 EUR.
pub const TAGESSATZ_MIN_CENTS: u64 = 100;
/// Maximum value of a single daily unit in EUR cents (§ 40 Abs. 2 S. 3 StGB): 30 000 EUR.
pub const TAGESSATZ_MAX_CENTS: u64 = 3_000_000;
/// General minimum of time-limited imprisonment in months (§ 38 Abs. 2 StGB): one month.
pub const FREIHEITSSTRAFE_MIN_MONTHS: u32 = 1;
/// General maximum of time-limited imprisonment in months (§ 38 Abs. 2 StGB): 15 years.
pub const FREIHEITSSTRAFE_MAX_MONTHS: u32 = 15 * 12;

/// A custodial sentence (Freiheitsstrafe), §§ 38-39 StGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Freiheitsstrafe {
    /// Time-limited imprisonment (zeitige Freiheitsstrafe), measured in months.
    ///
    /// Per § 38 Abs. 2 StGB the general range is 1 month to 180 months (15 years).
    Zeitig {
        /// Duration in full months.
        months: u32,
    },
    /// Imprisonment for life (lebenslange Freiheitsstrafe), e.g. § 211 StGB (Mord).
    Lebenslang,
}

impl Freiheitsstrafe {
    /// Construct a time-limited custodial sentence from a number of months.
    ///
    /// # Errors
    /// Returns [`StgbError::InvalidFreiheitsstrafe`] if `months` is outside the
    /// general range of § 38 Abs. 2 StGB (1..=180), unless a specific provision
    /// permits a different range (which is enforced at the offence level).
    pub fn from_months(months: u32) -> Result<Self> {
        if !(FREIHEITSSTRAFE_MIN_MONTHS..=FREIHEITSSTRAFE_MAX_MONTHS).contains(&months) {
            return Err(StgbError::InvalidFreiheitsstrafe { months });
        }
        Ok(Self::Zeitig { months })
    }

    /// Whether this is a life sentence.
    #[must_use]
    pub fn is_lifelong(&self) -> bool {
        matches!(self, Self::Lebenslang)
    }

    /// Duration in months for a time-limited sentence, or `None` for life.
    #[must_use]
    pub fn months(&self) -> Option<u32> {
        match self {
            Self::Zeitig { months } => Some(*months),
            Self::Lebenslang => None,
        }
    }
}

/// A day-fine (Geldstrafe in Tagessätzen), §§ 40-43 StGB.
///
/// Stored as the number of daily units and the amount of a single unit in EUR
/// cents, so the total fine and the default-imprisonment term can be derived.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Geldstrafe {
    /// Number of daily units (Anzahl der Tagessätze), 5..=360 (§ 40 Abs. 1 StGB).
    pub daily_units: u32,
    /// Amount of a single daily unit in EUR cents (§ 40 Abs. 2 StGB).
    pub unit_amount_cents: u64,
}

impl Geldstrafe {
    /// Construct and validate a day-fine.
    ///
    /// # Errors
    /// Returns [`StgbError::InvalidTagessaetze`] if the number of daily units is
    /// outside 5..=360, or [`StgbError::InvalidTagessatzHoehe`] if a single daily
    /// unit is outside 1..=30 000 EUR (§ 40 Abs. 1 and Abs. 2 S. 3 StGB).
    pub fn new(daily_units: u32, unit_amount_cents: u64) -> Result<Self> {
        if !(TAGESSAETZE_MIN..=TAGESSAETZE_MAX).contains(&daily_units) {
            return Err(StgbError::InvalidTagessaetze { count: daily_units });
        }
        if !(TAGESSATZ_MIN_CENTS..=TAGESSATZ_MAX_CENTS).contains(&unit_amount_cents) {
            return Err(StgbError::InvalidTagessatzHoehe {
                cents: unit_amount_cents,
            });
        }
        Ok(Self {
            daily_units,
            unit_amount_cents,
        })
    }

    /// Total amount of the fine in EUR cents (daily units × unit amount).
    #[must_use]
    pub fn total_cents(&self) -> u64 {
        u64::from(self.daily_units) * self.unit_amount_cents
    }

    /// Total amount of the fine in EUR.
    #[must_use]
    pub fn total_euros(&self) -> f64 {
        self.total_cents() as f64 / 100.0
    }

    /// Default imprisonment term (Ersatzfreiheitsstrafe) in days: one day per
    /// daily unit (§ 43 S. 2 StGB).
    #[must_use]
    pub fn default_imprisonment_days(&self) -> u32 {
        self.daily_units
    }
}

/// A concrete sanction imposed in an individual case (verhängte Strafe).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Strafe {
    /// Custodial sentence (Freiheitsstrafe), §§ 38-39 StGB.
    Freiheitsstrafe(Freiheitsstrafe),
    /// Day-fine (Geldstrafe), §§ 40-43 StGB.
    Geldstrafe(Geldstrafe),
}

/// A statutory sentencing range (Strafrahmen) for an offence.
///
/// German offences specify their penalty in the abstract, e.g. "imprisonment of
/// up to five years or a fine" (§ 242 StGB) or "imprisonment of not less than one
/// year" (§ 249 StGB). This type encodes that abstract range so that a concrete
/// [`Strafe`] can be checked against it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Strafrahmen {
    /// Lower limit of custodial sentence in months (Mindestmaß der Freiheitsstrafe).
    ///
    /// `None` means the general statutory minimum of one month (§ 38 Abs. 2 StGB)
    /// applies (i.e. the provision states only an upper limit).
    pub min_months: Option<u32>,
    /// Upper limit of custodial sentence in months (Höchstmaß der Freiheitsstrafe).
    ///
    /// `None` means the offence is punishable by **life imprisonment**.
    pub max_months: Option<u32>,
    /// Whether a fine (Geldstrafe) is available as an alternative to imprisonment.
    pub fine_alternative: bool,
}

impl Strafrahmen {
    /// Range "imprisonment of up to `max` months, or a fine" (the typical
    /// formulation of less serious offences, e.g. § 242 Abs. 1 StGB).
    #[must_use]
    pub fn up_to_months_or_fine(max: u32) -> Self {
        Self {
            min_months: None,
            max_months: Some(max),
            fine_alternative: true,
        }
    }

    /// Range "imprisonment from `min` to `max` months" (no fine alternative),
    /// e.g. the qualified offences such as § 250 StGB (schwerer Raub).
    #[must_use]
    pub fn imprisonment(min: u32, max: u32) -> Self {
        Self {
            min_months: Some(min),
            max_months: Some(max),
            fine_alternative: false,
        }
    }

    /// Range "imprisonment of not less than `min` months" up to the general
    /// maximum of 15 years (§ 38 Abs. 2 StGB), e.g. § 249 Abs. 1 StGB (Raub).
    #[must_use]
    pub fn at_least_months(min: u32) -> Self {
        Self {
            min_months: Some(min),
            max_months: Some(FREIHEITSSTRAFE_MAX_MONTHS),
            fine_alternative: false,
        }
    }

    /// Range "life imprisonment" (lebenslange Freiheitsstrafe), e.g. § 211 StGB.
    #[must_use]
    pub fn life() -> Self {
        Self {
            min_months: None,
            max_months: None,
            fine_alternative: false,
        }
    }

    /// Whether life imprisonment is within this range.
    #[must_use]
    pub fn allows_life(&self) -> bool {
        self.max_months.is_none()
    }

    /// Effective lower bound of imprisonment in months (defaults to the general
    /// statutory minimum of one month per § 38 Abs. 2 StGB when unspecified).
    #[must_use]
    pub fn effective_min_months(&self) -> u32 {
        self.min_months.unwrap_or(FREIHEITSSTRAFE_MIN_MONTHS)
    }

    /// Check whether a concrete sanction falls within this statutory range.
    ///
    /// # Errors
    /// - [`StgbError::SentenceOutsideRange`] if a custodial term is below the
    ///   minimum or above the maximum of the range.
    /// - [`StgbError::FineNotAvailable`] if a fine is imposed where the offence
    ///   does not permit a fine alternative.
    /// - [`StgbError::LifeNotAvailable`] if a life sentence is imposed where the
    ///   offence does not permit life imprisonment.
    pub fn check(&self, strafe: &Strafe) -> Result<()> {
        match strafe {
            Strafe::Freiheitsstrafe(Freiheitsstrafe::Lebenslang) => {
                if self.allows_life() {
                    Ok(())
                } else {
                    Err(StgbError::LifeNotAvailable)
                }
            }
            Strafe::Freiheitsstrafe(Freiheitsstrafe::Zeitig { months }) => {
                let min = self.effective_min_months();
                let max = self.max_months.unwrap_or(FREIHEITSSTRAFE_MAX_MONTHS);
                if *months < min || *months > max {
                    return Err(StgbError::SentenceOutsideRange {
                        months: *months,
                        min,
                        max,
                    });
                }
                Ok(())
            }
            Strafe::Geldstrafe(_) => {
                if self.fine_alternative {
                    Ok(())
                } else {
                    Err(StgbError::FineNotAvailable)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freiheitsstrafe_within_general_range() {
        let fs = Freiheitsstrafe::from_months(12).expect("12 months valid");
        assert_eq!(fs.months(), Some(12));
        assert!(!fs.is_lifelong());
    }

    #[test]
    fn freiheitsstrafe_rejects_zero_and_overlong() {
        assert!(Freiheitsstrafe::from_months(0).is_err());
        assert!(Freiheitsstrafe::from_months(FREIHEITSSTRAFE_MAX_MONTHS + 1).is_err());
    }

    #[test]
    fn lebenslang_has_no_month_count() {
        let fs = Freiheitsstrafe::Lebenslang;
        assert!(fs.is_lifelong());
        assert_eq!(fs.months(), None);
    }

    #[test]
    fn geldstrafe_total_and_default_imprisonment() {
        // 90 daily units of 50 EUR each.
        let gs = Geldstrafe::new(90, 5_000).expect("valid fine");
        assert_eq!(gs.total_cents(), 90 * 5_000);
        assert_eq!(gs.total_euros(), 4_500.0);
        // § 43 S. 2 StGB: one day per daily unit.
        assert_eq!(gs.default_imprisonment_days(), 90);
    }

    #[test]
    fn geldstrafe_rejects_units_out_of_range() {
        assert!(Geldstrafe::new(TAGESSAETZE_MIN - 1, 5_000).is_err());
        assert!(Geldstrafe::new(TAGESSAETZE_MAX + 1, 5_000).is_err());
    }

    #[test]
    fn geldstrafe_rejects_unit_amount_out_of_range() {
        assert!(Geldstrafe::new(30, TAGESSATZ_MIN_CENTS - 1).is_err());
        assert!(Geldstrafe::new(30, TAGESSATZ_MAX_CENTS + 1).is_err());
    }

    #[test]
    fn strafrahmen_up_to_or_fine_accepts_fine_and_imprisonment() {
        // § 242 StGB style: up to 5 years or a fine.
        let r = Strafrahmen::up_to_months_or_fine(60);
        assert!(!r.allows_life());
        assert!(
            r.check(&Strafe::Freiheitsstrafe(
                Freiheitsstrafe::from_months(60).expect("ok")
            ))
            .is_ok()
        );
        assert!(
            r.check(&Strafe::Geldstrafe(Geldstrafe::new(30, 3_000).expect("ok")))
                .is_ok()
        );
    }

    #[test]
    fn strafrahmen_rejects_sentence_above_max() {
        let r = Strafrahmen::up_to_months_or_fine(60);
        let res = r.check(&Strafe::Freiheitsstrafe(
            Freiheitsstrafe::from_months(72).expect("ok"),
        ));
        assert!(matches!(res, Err(StgbError::SentenceOutsideRange { .. })));
    }

    #[test]
    fn strafrahmen_at_least_enforces_minimum() {
        // § 249 StGB style: not less than one year.
        let r = Strafrahmen::at_least_months(12);
        assert_eq!(r.effective_min_months(), 12);
        let too_low = r.check(&Strafe::Freiheitsstrafe(
            Freiheitsstrafe::from_months(6).expect("ok"),
        ));
        assert!(matches!(
            too_low,
            Err(StgbError::SentenceOutsideRange { .. })
        ));
        assert!(
            r.check(&Strafe::Freiheitsstrafe(
                Freiheitsstrafe::from_months(24).expect("ok")
            ))
            .is_ok()
        );
    }

    #[test]
    fn strafrahmen_imprisonment_only_rejects_fine() {
        let r = Strafrahmen::imprisonment(12, 180);
        let res = r.check(&Strafe::Geldstrafe(Geldstrafe::new(30, 3_000).expect("ok")));
        assert!(matches!(res, Err(StgbError::FineNotAvailable)));
    }

    #[test]
    fn strafrahmen_life_accepts_life_and_rejects_elsewhere() {
        // § 211 StGB: life imprisonment.
        let life = Strafrahmen::life();
        assert!(life.allows_life());
        assert!(
            life.check(&Strafe::Freiheitsstrafe(Freiheitsstrafe::Lebenslang))
                .is_ok()
        );
        let no_life = Strafrahmen::up_to_months_or_fine(60);
        assert!(matches!(
            no_life.check(&Strafe::Freiheitsstrafe(Freiheitsstrafe::Lebenslang)),
            Err(StgbError::LifeNotAvailable)
        ));
    }
}
