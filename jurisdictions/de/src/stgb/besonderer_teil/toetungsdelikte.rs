//! StGB §§ 211-222 - Homicide Offences (Tötungsdelikte)
//!
//! ## § 212 StGB - Totschlag (Manslaughter) - the basic offence
//!
//! > (1) Wer einen Menschen tötet, ohne Mörder zu sein, wird als Totschläger mit
//! > Freiheitsstrafe nicht unter fünf Jahren bestraft.
//! > (2) In besonders schweren Fällen ist auf lebenslange Freiheitsstrafe zu
//! > erkennen.
//!
//! **English**: Whoever kills a person without being a murderer is punished as a
//! manslaughterer with imprisonment of **not less than five years**; in especially
//! serious cases, **life imprisonment**.
//!
//! ## § 211 StGB - Mord (Murder)
//!
//! > (1) Der Mörder wird mit lebenslanger Freiheitsstrafe bestraft.
//! > (2) Mörder ist, wer aus Mordlust, zur Befriedigung des Geschlechtstriebs, aus
//! > Habgier oder sonst aus niedrigen Beweggründen, heimtückisch oder grausam oder
//! > mit gemeingefährlichen Mitteln oder um eine andere Straftat zu ermöglichen
//! > oder zu verdecken, einen Menschen tötet.
//!
//! Murder is manslaughter aggravated by at least one **murder characteristic**
//! (Mordmerkmal). The three groups are:
//! - **1. Gruppe (täterbezogen)**: Mordlust, Befriedigung des Geschlechtstriebs,
//!   Habgier, sonstige niedrige Beweggründe.
//! - **2. Gruppe (tatbezogen)**: Heimtücke, Grausamkeit, gemeingefährliche Mittel.
//! - **3. Gruppe (täterbezogen)**: Ermöglichungs- oder Verdeckungsabsicht.
//!
//! Punishment is **mandatory life imprisonment**.
//!
//! ## § 213 StGB - Minder schwerer Fall des Totschlags
//!
//! A less serious case of manslaughter (e.g. provocation) carries imprisonment
//! from **one to ten years**.
//!
//! ## § 216 StGB - Tötung auf Verlangen (Killing on request)
//!
//! Killing at the express and earnest request of the victim: imprisonment from
//! **six months to five years**.
//!
//! ## § 222 StGB - Fahrlässige Tötung (Negligent homicide)
//!
//! > Wer durch Fahrlässigkeit den Tod eines Menschen verursacht, wird mit
//! > Freiheitsstrafe bis zu fünf Jahren oder mit Geldstrafe bestraft.
//!
//! Causing a person's death by negligence: imprisonment of **up to five years or
//! a fine**.

use serde::{Deserialize, Serialize};

use crate::stgb::error::{Result, StgbError};
use crate::stgb::strafe::Strafrahmen;

/// A murder characteristic (Mordmerkmal), § 211 Abs. 2 StGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mordmerkmal {
    /// Lust for killing (Mordlust) - 1st group.
    Mordlust,
    /// Satisfaction of the sexual drive (Befriedigung des Geschlechtstriebs).
    Geschlechtstrieb,
    /// Greed (Habgier) - 1st group.
    Habgier,
    /// Other base motives (sonstige niedrige Beweggründe) - 1st group.
    NiedrigeBeweggruende,
    /// Treachery / by stealth (Heimtücke) - 2nd group.
    Heimtuecke,
    /// Cruelty (Grausamkeit) - 2nd group.
    Grausamkeit,
    /// Means dangerous to the public (gemeingefährliche Mittel) - 2nd group.
    GemeingefaehrlicheMittel,
    /// Intent to enable another offence (Ermöglichungsabsicht) - 3rd group.
    Ermoeglichungsabsicht,
    /// Intent to cover up another offence (Verdeckungsabsicht) - 3rd group.
    Verdeckungsabsicht,
}

impl Mordmerkmal {
    /// The doctrinal group (1-3) to which the murder characteristic belongs.
    #[must_use]
    pub fn gruppe(&self) -> u8 {
        match self {
            Mordmerkmal::Mordlust
            | Mordmerkmal::Geschlechtstrieb
            | Mordmerkmal::Habgier
            | Mordmerkmal::NiedrigeBeweggruende => 1,
            Mordmerkmal::Heimtuecke
            | Mordmerkmal::Grausamkeit
            | Mordmerkmal::GemeingefaehrlicheMittel => 2,
            Mordmerkmal::Ermoeglichungsabsicht | Mordmerkmal::Verdeckungsabsicht => 3,
        }
    }
}

/// The specific homicide offence applicable to a case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HomicideOffence {
    /// Murder (Mord, § 211 StGB) with the murder characteristics found.
    Mord {
        /// The murder characteristics present (at least one).
        mordmerkmale: Vec<Mordmerkmal>,
    },
    /// Manslaughter (Totschlag, § 212 StGB).
    Totschlag {
        /// Whether an especially serious case applies (§ 212 Abs. 2 → life).
        besonders_schwer: bool,
    },
    /// Less serious manslaughter (minder schwerer Fall, § 213 StGB).
    MinderSchwererTotschlag,
    /// Killing on request (Tötung auf Verlangen, § 216 StGB).
    ToetungAufVerlangen,
    /// Negligent homicide (fahrlässige Tötung, § 222 StGB).
    FahrlaessigeToetung,
}

impl HomicideOffence {
    /// The § citation of the offence.
    #[must_use]
    pub fn paragraph(&self) -> &'static str {
        match self {
            HomicideOffence::Mord { .. } => "§ 211 StGB",
            HomicideOffence::Totschlag { .. } => "§ 212 StGB",
            HomicideOffence::MinderSchwererTotschlag => "§ 213 StGB",
            HomicideOffence::ToetungAufVerlangen => "§ 216 StGB",
            HomicideOffence::FahrlaessigeToetung => "§ 222 StGB",
        }
    }

    /// The statutory sentencing range (Strafrahmen) of the offence.
    #[must_use]
    pub fn strafrahmen(&self) -> Strafrahmen {
        match self {
            // § 211 StGB - mandatory life imprisonment.
            HomicideOffence::Mord { .. } => Strafrahmen::life(),
            // § 212 StGB - not less than five years (60 months); life in especially
            // serious cases (§ 212 Abs. 2).
            HomicideOffence::Totschlag { besonders_schwer } => {
                if *besonders_schwer {
                    Strafrahmen::life()
                } else {
                    Strafrahmen::at_least_months(60)
                }
            }
            // § 213 StGB - one to ten years.
            HomicideOffence::MinderSchwererTotschlag => Strafrahmen::imprisonment(12, 120),
            // § 216 StGB - six months to five years.
            HomicideOffence::ToetungAufVerlangen => Strafrahmen::imprisonment(6, 60),
            // § 222 StGB - up to five years or a fine.
            HomicideOffence::FahrlaessigeToetung => Strafrahmen::up_to_months_or_fine(60),
        }
    }
}

/// A homicide case (Tötungsdelikt), §§ 211-222 StGB.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HomicideCase {
    /// Description of the victim (Tatopfer) - must be "another human being".
    pub opfer: String,
    /// Whether the victim was a living human being (anderer Mensch) at the time of
    /// the act; killing the perpetrator himself is not a homicide offence, nor is
    /// acting on a non-living object.
    pub opfer_ist_anderer_mensch: bool,
    /// Whether death was caused (Tod verursacht / Erfolg eingetreten).
    pub tod_verursacht: bool,
    /// Whether causation is established (Kausalität, conditio sine qua non).
    pub kausalitaet: bool,
    /// Whether the offender acted intentionally (Tötungsvorsatz). Required for all
    /// intentional homicide offences; `false` only for § 222 (negligence).
    pub vorsatz: bool,
    /// For § 216: whether there was an express and earnest request to be killed.
    pub ausdrueckliches_verlangen: bool,
    /// The offence the case is charged under.
    pub offence: HomicideOffence,
}

/// Validate a homicide case under §§ 211-222 StGB.
///
/// Common elements: the act must be directed at another living human being, must
/// have caused death, and causation must be established. Murder additionally
/// requires at least one murder characteristic and intent; § 216 requires an
/// express and earnest request; § 222 requires (only) negligence.
///
/// # Errors
/// - [`StgbError::InvalidTatobjekt`] if the object is missing or not another
///   living human being.
/// - [`StgbError::TatbestandNotFulfilled`] if death did not occur.
/// - [`StgbError::NoKausalitaet`] if causation is not established.
/// - [`StgbError::FahrlaessigkeitNichtStrafbar`] if an intentional homicide
///   offence is charged but intent is missing.
/// - [`StgbError::AbsichtMissing`] if § 211 is charged without any murder
///   characteristic, or § 216 without an express request.
pub fn validate_homicide(case: &HomicideCase) -> Result<()> {
    if case.opfer.trim().is_empty() || !case.opfer_ist_anderer_mensch {
        return Err(StgbError::InvalidTatobjekt {
            detail: "Tatobjekt muss ein anderer lebender Mensch sein (§§ 211 ff. StGB)".to_string(),
        });
    }
    if !case.tod_verursacht {
        return Err(StgbError::TatbestandNotFulfilled {
            element: "Tod des Opfers (Taterfolg)".to_string(),
        });
    }
    if !case.kausalitaet {
        return Err(StgbError::NoKausalitaet);
    }

    match &case.offence {
        HomicideOffence::FahrlaessigeToetung => {
            // § 222 StGB does not require intent; negligence suffices and is
            // assumed by the choice of offence.
        }
        HomicideOffence::Mord { mordmerkmale } => {
            if !case.vorsatz {
                return Err(StgbError::FahrlaessigkeitNichtStrafbar);
            }
            if mordmerkmale.is_empty() {
                return Err(StgbError::AbsichtMissing {
                    detail: "Mord setzt mindestens ein Mordmerkmal voraus (§ 211 Abs. 2 StGB)"
                        .to_string(),
                });
            }
        }
        HomicideOffence::ToetungAufVerlangen => {
            if !case.vorsatz {
                return Err(StgbError::FahrlaessigkeitNichtStrafbar);
            }
            if !case.ausdrueckliches_verlangen {
                return Err(StgbError::AbsichtMissing {
                    detail: "§ 216 StGB setzt ausdrückliches und ernstliches Verlangen voraus"
                        .to_string(),
                });
            }
        }
        _ => {
            if !case.vorsatz {
                return Err(StgbError::FahrlaessigkeitNichtStrafbar);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_case(offence: HomicideOffence) -> HomicideCase {
        HomicideCase {
            opfer: "Opfer".to_string(),
            opfer_ist_anderer_mensch: true,
            tod_verursacht: true,
            kausalitaet: true,
            vorsatz: true,
            ausdrueckliches_verlangen: false,
            offence,
        }
    }

    #[test]
    fn totschlag_minimum_is_five_years() {
        let o = HomicideOffence::Totschlag {
            besonders_schwer: false,
        };
        assert_eq!(o.paragraph(), "§ 212 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 60);
        assert!(!r.allows_life());
        assert!(validate_homicide(&base_case(o)).is_ok());
    }

    #[test]
    fn totschlag_besonders_schwer_allows_life() {
        let o = HomicideOffence::Totschlag {
            besonders_schwer: true,
        };
        assert!(o.strafrahmen().allows_life());
    }

    #[test]
    fn mord_requires_mordmerkmal_and_is_life() {
        let o = HomicideOffence::Mord {
            mordmerkmale: vec![Mordmerkmal::Heimtuecke],
        };
        assert!(o.strafrahmen().allows_life());
        assert!(validate_homicide(&base_case(o)).is_ok());

        let o_empty = HomicideOffence::Mord {
            mordmerkmale: vec![],
        };
        assert!(matches!(
            validate_homicide(&base_case(o_empty)),
            Err(StgbError::AbsichtMissing { .. })
        ));
    }

    #[test]
    fn mordmerkmal_groups() {
        assert_eq!(Mordmerkmal::Habgier.gruppe(), 1);
        assert_eq!(Mordmerkmal::Heimtuecke.gruppe(), 2);
        assert_eq!(Mordmerkmal::Verdeckungsabsicht.gruppe(), 3);
    }

    #[test]
    fn toetung_auf_verlangen_requires_request() {
        let o = HomicideOffence::ToetungAufVerlangen;
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 6);
        let mut c = base_case(o.clone());
        assert!(matches!(
            validate_homicide(&c),
            Err(StgbError::AbsichtMissing { .. })
        ));
        c.ausdrueckliches_verlangen = true;
        assert!(validate_homicide(&c).is_ok());
    }

    #[test]
    fn fahrlaessige_toetung_needs_no_intent() {
        let o = HomicideOffence::FahrlaessigeToetung;
        assert!(o.strafrahmen().fine_alternative);
        let mut c = base_case(o);
        c.vorsatz = false;
        assert!(validate_homicide(&c).is_ok());
    }

    #[test]
    fn intentional_offence_requires_intent() {
        let mut c = base_case(HomicideOffence::Totschlag {
            besonders_schwer: false,
        });
        c.vorsatz = false;
        assert!(matches!(
            validate_homicide(&c),
            Err(StgbError::FahrlaessigkeitNichtStrafbar)
        ));
    }

    #[test]
    fn object_must_be_another_human() {
        let mut c = base_case(HomicideOffence::Totschlag {
            besonders_schwer: false,
        });
        c.opfer_ist_anderer_mensch = false;
        assert!(matches!(
            validate_homicide(&c),
            Err(StgbError::InvalidTatobjekt { .. })
        ));
    }

    #[test]
    fn death_and_causation_required() {
        let mut c = base_case(HomicideOffence::Totschlag {
            besonders_schwer: false,
        });
        c.tod_verursacht = false;
        assert!(matches!(
            validate_homicide(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
        let mut c2 = base_case(HomicideOffence::Totschlag {
            besonders_schwer: false,
        });
        c2.kausalitaet = false;
        assert!(matches!(
            validate_homicide(&c2),
            Err(StgbError::NoKausalitaet)
        ));
    }

    #[test]
    fn minder_schwerer_totschlag_range() {
        let o = HomicideOffence::MinderSchwererTotschlag;
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 12);
        assert_eq!(r.max_months, Some(120));
    }
}
