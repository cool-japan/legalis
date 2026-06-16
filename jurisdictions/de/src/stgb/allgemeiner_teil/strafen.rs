//! StGB §§ 38-43 - Penalties (Strafen) - General Part view
//!
//! This module provides the General-Part perspective on the sentencing framework
//! defined in [`crate::stgb::strafe`], covering §§ 38-43 StGB:
//!
//! - **§ 38** - Dauer der Freiheitsstrafe (duration of imprisonment).
//! - **§ 39** - Bemessung der Freiheitsstrafe (calculation in weeks/months/years).
//! - **§ 40** - Verhängung der Geldstrafe in Tagessätzen (day-fine system).
//! - **§ 41** - Geldstrafe neben Freiheitsstrafe (fine in addition to prison).
//! - **§ 43** - Ersatzfreiheitsstrafe (default imprisonment).
//!
//! See [`crate::stgb::strafe`] for the underlying [`Freiheitsstrafe`],
//! [`Geldstrafe`], [`Strafe`] and [`Strafrahmen`] types and their validation.

use serde::{Deserialize, Serialize};

use crate::stgb::error::{Result, StgbError};
use crate::stgb::strafe::{Freiheitsstrafe, Geldstrafe};

pub use crate::stgb::strafe::{
    FREIHEITSSTRAFE_MAX_MONTHS, FREIHEITSSTRAFE_MIN_MONTHS, Freiheitsstrafe as Imprisonment,
    Geldstrafe as DayFine, Strafe, Strafrahmen, TAGESSAETZE_MAX, TAGESSAETZE_MIN,
    TAGESSATZ_MAX_CENTS, TAGESSATZ_MIN_CENTS,
};

/// Unit in which a custodial sentence is measured under § 39 StGB.
///
/// Imprisonment of less than one year is measured in **full weeks and months**;
/// of one year or more in **full months and years** (§ 39 StGB).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SentenceMeasureUnit {
    /// Full weeks and months (Wochen und Monate) - terms under one year.
    WeeksAndMonths,
    /// Full months and years (Monate und Jahre) - terms of one year or more.
    MonthsAndYears,
}

/// Determine how a custodial term is to be measured (§ 39 StGB).
#[must_use]
pub fn measure_unit(strafe: &Freiheitsstrafe) -> SentenceMeasureUnit {
    match strafe.months() {
        Some(m) if m < 12 => SentenceMeasureUnit::WeeksAndMonths,
        _ => SentenceMeasureUnit::MonthsAndYears,
    }
}

/// A combined penalty: imprisonment optionally accompanied by a fine (§ 41 StGB).
///
/// A fine may be imposed **in addition** to imprisonment where the offender has
/// enriched himself, or tried to, through the offence, and a fine is appropriate
/// having regard to his personal and economic circumstances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombinedPenalty {
    /// The principal custodial sentence.
    pub imprisonment: Freiheitsstrafe,
    /// An additional fine under § 41 StGB, if imposed.
    pub additional_fine: Option<Geldstrafe>,
}

impl CombinedPenalty {
    /// Construct a combined penalty, validating the § 41 StGB precondition.
    ///
    /// # Errors
    /// [`StgbError::FineNotAvailable`] if an additional fine is requested although
    /// the offender did not enrich himself (or try to) through the offence, which
    /// is the statutory precondition of § 41 StGB.
    pub fn new(
        imprisonment: Freiheitsstrafe,
        additional_fine: Option<Geldstrafe>,
        offender_enriched: bool,
    ) -> Result<Self> {
        if additional_fine.is_some() && !offender_enriched {
            return Err(StgbError::FineNotAvailable);
        }
        Ok(Self {
            imprisonment,
            additional_fine,
        })
    }
}

/// Compute the default imprisonment term (Ersatzfreiheitsstrafe) for an
/// unrecoverable fine: one day per daily unit (§ 43 S. 2 StGB).
#[must_use]
pub fn ersatzfreiheitsstrafe_days(fine: &Geldstrafe) -> u32 {
    fine.default_imprisonment_days()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_term_measured_in_weeks_and_months() {
        let fs = Freiheitsstrafe::from_months(6).expect("ok");
        assert_eq!(measure_unit(&fs), SentenceMeasureUnit::WeeksAndMonths);
    }

    #[test]
    fn long_term_measured_in_months_and_years() {
        let fs = Freiheitsstrafe::from_months(24).expect("ok");
        assert_eq!(measure_unit(&fs), SentenceMeasureUnit::MonthsAndYears);
        assert_eq!(
            measure_unit(&Freiheitsstrafe::Lebenslang),
            SentenceMeasureUnit::MonthsAndYears
        );
    }

    #[test]
    fn additional_fine_requires_enrichment() {
        let imprisonment = Freiheitsstrafe::from_months(12).expect("ok");
        let fine = Geldstrafe::new(90, 5_000).expect("ok");
        // § 41 precondition not met.
        assert!(matches!(
            CombinedPenalty::new(imprisonment, Some(fine), false),
            Err(StgbError::FineNotAvailable)
        ));
        // § 41 precondition met.
        let cp = CombinedPenalty::new(imprisonment, Some(fine), true).expect("ok");
        assert!(cp.additional_fine.is_some());
    }

    #[test]
    fn imprisonment_without_fine_is_always_ok() {
        let imprisonment = Freiheitsstrafe::from_months(12).expect("ok");
        let cp = CombinedPenalty::new(imprisonment, None, false).expect("ok");
        assert!(cp.additional_fine.is_none());
    }

    #[test]
    fn ersatzfreiheitsstrafe_is_one_day_per_unit() {
        let fine = Geldstrafe::new(120, 3_000).expect("ok");
        assert_eq!(ersatzfreiheitsstrafe_days(&fine), 120);
    }
}
