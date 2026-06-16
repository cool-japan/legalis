//! VwGO §§ 42, 68 ff. - Legal Remedies (Rechtsbehelfe)
//!
//! ## Widerspruchsverfahren (objection procedure) - §§ 68 ff. VwGO
//!
//! Before an action for annulment (Anfechtungsklage) or for the issue of an act
//! (Verpflichtungsklage), the lawfulness and expediency of the administrative act
//! must, as a rule, first be reviewed in preliminary proceedings (Vorverfahren,
//! § 68 Abs. 1 VwGO). The objection (Widerspruch) must be lodged within **one month**
//! after notification of the administrative act (§ 70 Abs. 1 VwGO).
//!
//! **English**: The objection is a precondition (Sachurteilsvoraussetzung) for the
//! annulment and the mandatory-injunction actions; it must be lodged within one month
//! of notification.
//!
//! ## Klagearten (types of action) - § 42 VwGO
//!
//! - **Anfechtungsklage** (§ 42 Abs. 1 Alt. 1 VwGO): action to annul an administrative act.
//! - **Verpflichtungsklage** (§ 42 Abs. 1 Alt. 2 VwGO): action to compel the issue of an act.
//! - **Fortsetzungsfeststellungsklage** (§ 113 Abs. 1 S. 4 VwGO): continuation declaratory action
//!   where the act has been settled (Erledigung) but a legitimate interest remains.
//!
//! ## Klagebefugnis (standing) - § 42 Abs. 2 VwGO
//!
//! Unless otherwise provided, the action is admissible only if the claimant asserts
//! that they are **possibly violated in their own rights** (möglicherweise in eigenen
//! Rechten verletzt) by the administrative act or its refusal.

use serde::{Deserialize, Serialize};

use crate::verwaltungsrecht::error::{Result, VwVfGError};

use chrono::NaiveDate;

/// A legal remedy in administrative law (Rechtsbehelf).
///
/// Rechtsbehelf im Verwaltungsrecht.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rechtsbehelf {
    /// Objection (Widerspruch, §§ 68 ff. VwGO).
    Widerspruch,
    /// Action for annulment (Anfechtungsklage, § 42 Abs. 1 Alt. 1 VwGO).
    Anfechtungsklage,
    /// Action to compel issue of an act (Verpflichtungsklage, § 42 Abs. 1 Alt. 2 VwGO).
    Verpflichtungsklage,
    /// Continuation declaratory action (Fortsetzungsfeststellungsklage, § 113 Abs. 1 S. 4 VwGO).
    Fortsetzungsfeststellungsklage,
}

impl Rechtsbehelf {
    /// The leading § citation of the legal remedy.
    ///
    /// Das maßgebliche §-Zitat des Rechtsbehelfs.
    #[must_use]
    pub fn paragraph(&self) -> &'static str {
        match self {
            Rechtsbehelf::Widerspruch => "§§ 68 ff. VwGO",
            Rechtsbehelf::Anfechtungsklage => "§ 42 Abs. 1 Alt. 1 VwGO",
            Rechtsbehelf::Verpflichtungsklage => "§ 42 Abs. 1 Alt. 2 VwGO",
            Rechtsbehelf::Fortsetzungsfeststellungsklage => "§ 113 Abs. 1 S. 4 VwGO",
        }
    }

    /// Whether the remedy is an action before the administrative court (Klage)
    /// rather than the preliminary objection (Widerspruch).
    ///
    /// Ob es sich um eine Klage (und nicht den Widerspruch) handelt.
    #[must_use]
    pub fn ist_klage(&self) -> bool {
        !matches!(self, Rechtsbehelf::Widerspruch)
    }
}

/// The type of administrative court action (Klageart), § 42 VwGO.
///
/// Klageart im verwaltungsgerichtlichen Verfahren (§ 42 VwGO).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Klageart {
    /// Action for annulment (Anfechtungsklage).
    Anfechtungsklage,
    /// Action to compel issue of an act (Verpflichtungsklage).
    Verpflichtungsklage,
    /// Continuation declaratory action (Fortsetzungsfeststellungsklage).
    Fortsetzungsfeststellungsklage,
}

impl Klageart {
    /// Whether the action is, as a rule, directed against an administrative act and
    /// therefore requires preliminary proceedings (Vorverfahren, § 68 VwGO).
    ///
    /// The continuation declaratory action presupposes that the act has already been
    /// settled (Erledigung), so a preliminary procedure is no longer required.
    ///
    /// Ob die Klageart regelmäßig ein Vorverfahren (§ 68 VwGO) voraussetzt.
    #[must_use]
    pub fn benoetigt_vorverfahren(&self) -> bool {
        matches!(
            self,
            Klageart::Anfechtungsklage | Klageart::Verpflichtungsklage
        )
    }
}

/// Compute the day on which the one-month objection period expires.
///
/// The objection period of § 70 Abs. 1 VwGO runs for **one calendar month** after
/// notification. Following the deadline rules of §§ 187, 188 BGB (applicable via
/// § 57 Abs. 2 VwGO, § 222 ZPO), the period ends on the day of the following month
/// that bears the same number as the day of notification; where that month is shorter
/// (e.g. notification on the 31st, target month February), it ends on the last day of
/// that month. This calendar-month arithmetic is performed by chrono's
/// [`chrono::Months`] addition, which clamps an overlong day to the month's end -
/// exactly the § 188 Abs. 3 BGB rule. The number of elapsed days is exposed
/// separately, see [`pruefe_widerspruch`].
fn fristende_ein_monat(start: NaiveDate) -> NaiveDate {
    // `checked_add_months` clamps overlong days (e.g. 31 Jan -> 28/29 Feb) and only
    // returns `None` on out-of-range overflow (year > ~262_000), which cannot occur
    // for realistic legal dates; the total fallback returns the start date unchanged.
    start
        .checked_add_months(chrono::Months::new(1))
        .unwrap_or(start)
}

/// An objection against an administrative act, §§ 68 ff. VwGO.
///
/// Widerspruch gegen einen Verwaltungsakt (§§ 68 ff. VwGO).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Widerspruch {
    /// Date on which the objection was lodged (Tag der Einlegung).
    pub eingelegt_am: NaiveDate,
    /// Date on which the administrative act was notified (Bekanntgabe, § 41 VwVfG).
    pub bekanntgabe_am: NaiveDate,
    /// Whether the objection is the admissible remedy (statthaft) at all, i.e. an
    /// administrative act exists against which the objection lies.
    pub statthaft: bool,
}

impl Widerspruch {
    /// The number of days elapsed between notification and lodging of the objection.
    ///
    /// May be negative if the objection is dated before notification (which is itself
    /// inadmissible). Exposed so callers can inspect the timing.
    ///
    /// Anzahl der zwischen Bekanntgabe und Einlegung verstrichenen Tage.
    #[must_use]
    pub fn verstrichene_tage(&self) -> i64 {
        (self.eingelegt_am - self.bekanntgabe_am).num_days()
    }
}

/// Validate an objection under §§ 68, 70 VwGO.
///
/// The objection must be admissible (statthaft) and must have been lodged within one
/// month of notification (§ 70 Abs. 1 VwGO). The one-month period is computed by
/// calendar arithmetic (see [`fristende_ein_monat`]); an objection lodged on or
/// before the expiry day is in time. The elapsed-day count is reported in the error.
///
/// Prüft einen Widerspruch nach §§ 68, 70 VwGO.
///
/// # Errors
/// - [`VwVfGError::EmptyField`] if the objection is dated before notification.
/// - [`VwVfGError::Formfehler`] if the objection is not admissible (nicht statthaft).
/// - [`VwVfGError::WiderspruchVerfristet`] if it was lodged after the one-month period
///   (§ 70 Abs. 1 VwGO); the field `days` reports the elapsed days.
pub fn pruefe_widerspruch(w: &Widerspruch) -> Result<()> {
    let tage = w.verstrichene_tage();
    if tage < 0 {
        return Err(VwVfGError::EmptyField {
            field: "eingelegt_am (liegt vor der Bekanntgabe / before notification)".to_string(),
        });
    }
    if !w.statthaft {
        return Err(VwVfGError::Formfehler {
            detail: "Widerspruch nicht statthaft (kein tauglicher Verwaltungsakt) / objection not \
                     the admissible remedy"
                .to_string(),
        });
    }
    let fristende = fristende_ein_monat(w.bekanntgabe_am);
    if w.eingelegt_am > fristende {
        return Err(VwVfGError::WiderspruchVerfristet { days: tage });
    }
    Ok(())
}

/// An action for annulment before the administrative court, § 42 VwGO.
///
/// Anfechtungsklage vor dem Verwaltungsgericht (§ 42 VwGO).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnfechtungsklageCase {
    /// Whether the preliminary proceedings have been conducted
    /// (Vorverfahren durchgeführt, § 68 VwGO).
    pub vorverfahren_durchgefuehrt: bool,
    /// Whether the one-month time limit for the action has been observed
    /// (Klagefrist gewahrt, § 74 Abs. 1 VwGO).
    pub klagefrist_gewahrt: bool,
    /// Whether the claimant has standing, i.e. asserts a possible violation of their
    /// own rights (Klagebefugnis, § 42 Abs. 2 VwGO).
    pub klagebefugnis: bool,
}

/// Validate the admissibility of an action for annulment, § 42 VwGO.
///
/// The action is admissible only if the claimant has standing (§ 42 Abs. 2 VwGO),
/// the preliminary proceedings have been conducted (§ 68 VwGO) and the time limit has
/// been observed (§ 74 Abs. 1 VwGO).
///
/// Prüft die Zulässigkeit einer Anfechtungsklage (§ 42 VwGO).
///
/// # Errors
/// - [`VwVfGError::Formfehler`] if the standing (§ 42 Abs. 2 VwGO) or the preliminary
///   proceedings (§ 68 VwGO) are missing.
/// - [`VwVfGError::WiderspruchVerfristet`] (here with `days == 0` as a marker) if the
///   action's time limit (§ 74 Abs. 1 VwGO) has not been observed.
pub fn pruefe_anfechtungsklage(c: &AnfechtungsklageCase) -> Result<()> {
    if !c.klagebefugnis {
        return Err(VwVfGError::Formfehler {
            detail: "Klagebefugnis fehlt (§ 42 Abs. 2 VwGO) / no standing (§ 42 para. 2 VwGO)"
                .to_string(),
        });
    }
    if !c.vorverfahren_durchgefuehrt {
        return Err(VwVfGError::Formfehler {
            detail: "Vorverfahren nicht durchgeführt (§ 68 VwGO) / preliminary proceedings not \
                     conducted (§ 68 VwGO)"
                .to_string(),
        });
    }
    if !c.klagefrist_gewahrt {
        return Err(VwVfGError::WiderspruchVerfristet { days: 0 });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ymd(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).expect("valid test date")
    }

    #[test]
    fn rechtsbehelf_helpers() {
        assert!(Rechtsbehelf::Anfechtungsklage.ist_klage());
        assert!(!Rechtsbehelf::Widerspruch.ist_klage());
        assert_eq!(Rechtsbehelf::Widerspruch.paragraph(), "§§ 68 ff. VwGO");
        assert_eq!(
            Rechtsbehelf::Verpflichtungsklage.paragraph(),
            "§ 42 Abs. 1 Alt. 2 VwGO"
        );
    }

    #[test]
    fn klageart_vorverfahren() {
        assert!(Klageart::Anfechtungsklage.benoetigt_vorverfahren());
        assert!(Klageart::Verpflichtungsklage.benoetigt_vorverfahren());
        assert!(!Klageart::Fortsetzungsfeststellungsklage.benoetigt_vorverfahren());
    }

    #[test]
    fn fristende_normal_month() {
        // 15 Jan -> 15 Feb.
        assert_eq!(fristende_ein_monat(ymd(2026, 1, 15)), ymd(2026, 2, 15));
        // 10 Jun -> 10 Jul.
        assert_eq!(fristende_ein_monat(ymd(2026, 6, 10)), ymd(2026, 7, 10));
    }

    #[test]
    fn fristende_month_rollover_and_clamp() {
        // 31 Jan -> 28 Feb (non-leap year 2026).
        assert_eq!(fristende_ein_monat(ymd(2026, 1, 31)), ymd(2026, 2, 28));
        // 31 Dec -> 31 Jan next year.
        assert_eq!(fristende_ein_monat(ymd(2025, 12, 31)), ymd(2026, 1, 31));
        // 31 Jan 2024 -> 29 Feb 2024 (leap year).
        assert_eq!(fristende_ein_monat(ymd(2024, 1, 31)), ymd(2024, 2, 29));
    }

    #[test]
    fn widerspruch_in_time_same_day() {
        let w = Widerspruch {
            eingelegt_am: ymd(2026, 1, 20),
            bekanntgabe_am: ymd(2026, 1, 20),
            statthaft: true,
        };
        assert_eq!(w.verstrichene_tage(), 0);
        assert!(pruefe_widerspruch(&w).is_ok());
    }

    #[test]
    fn widerspruch_in_time_at_deadline() {
        // Notified 15 Jan, lodged 15 Feb - exactly on the one-month deadline.
        let w = Widerspruch {
            eingelegt_am: ymd(2026, 2, 15),
            bekanntgabe_am: ymd(2026, 1, 15),
            statthaft: true,
        };
        assert!(pruefe_widerspruch(&w).is_ok());
    }

    #[test]
    fn widerspruch_verfristet() {
        // Notified 15 Jan, lodged 16 Feb - one day after the deadline.
        let w = Widerspruch {
            eingelegt_am: ymd(2026, 2, 16),
            bekanntgabe_am: ymd(2026, 1, 15),
            statthaft: true,
        };
        match pruefe_widerspruch(&w) {
            Err(VwVfGError::WiderspruchVerfristet { days }) => {
                assert_eq!(days, 32);
            }
            other => panic!("expected WiderspruchVerfristet, got {other:?}"),
        }
    }

    #[test]
    fn widerspruch_clearly_late() {
        let w = Widerspruch {
            eingelegt_am: ymd(2026, 5, 1),
            bekanntgabe_am: ymd(2026, 1, 1),
            statthaft: true,
        };
        assert!(matches!(
            pruefe_widerspruch(&w),
            Err(VwVfGError::WiderspruchVerfristet { .. })
        ));
    }

    #[test]
    fn widerspruch_nicht_statthaft() {
        let w = Widerspruch {
            eingelegt_am: ymd(2026, 1, 20),
            bekanntgabe_am: ymd(2026, 1, 20),
            statthaft: false,
        };
        assert!(matches!(
            pruefe_widerspruch(&w),
            Err(VwVfGError::Formfehler { .. })
        ));
    }

    #[test]
    fn widerspruch_before_notification_rejected() {
        let w = Widerspruch {
            eingelegt_am: ymd(2026, 1, 10),
            bekanntgabe_am: ymd(2026, 1, 20),
            statthaft: true,
        };
        assert!(matches!(
            pruefe_widerspruch(&w),
            Err(VwVfGError::EmptyField { .. })
        ));
    }

    #[test]
    fn anfechtungsklage_admissible() {
        let c = AnfechtungsklageCase {
            vorverfahren_durchgefuehrt: true,
            klagefrist_gewahrt: true,
            klagebefugnis: true,
        };
        assert!(pruefe_anfechtungsklage(&c).is_ok());
    }

    #[test]
    fn anfechtungsklage_missing_vorverfahren() {
        let c = AnfechtungsklageCase {
            vorverfahren_durchgefuehrt: false,
            klagefrist_gewahrt: true,
            klagebefugnis: true,
        };
        assert!(matches!(
            pruefe_anfechtungsklage(&c),
            Err(VwVfGError::Formfehler { .. })
        ));
    }

    #[test]
    fn anfechtungsklage_missing_klagebefugnis() {
        let c = AnfechtungsklageCase {
            vorverfahren_durchgefuehrt: true,
            klagefrist_gewahrt: true,
            klagebefugnis: false,
        };
        assert!(matches!(
            pruefe_anfechtungsklage(&c),
            Err(VwVfGError::Formfehler { .. })
        ));
    }

    #[test]
    fn anfechtungsklage_missing_klagefrist() {
        let c = AnfechtungsklageCase {
            vorverfahren_durchgefuehrt: true,
            klagefrist_gewahrt: false,
            klagebefugnis: true,
        };
        assert!(matches!(
            pruefe_anfechtungsklage(&c),
            Err(VwVfGError::WiderspruchVerfristet { .. })
        ));
    }
}
