//! StGB §§ 177-184b - Offences against sexual self-determination
//! (Straftaten gegen die sexuelle Selbstbestimmung)
//!
//! This module models the offence structure (Tatbestandsmerkmale) and the
//! statutory sentencing ranges (Strafrahmen) of §§ 177-184b StGB at a sober,
//! clinical legal abstraction. It is centred on § 177 StGB in its current
//! form after the 2016 reform ("Nein heißt Nein"), under which a sexual act
//! against the recognisable contrary will of another person is punishable.
//!
//! Only neutral legal element-modelling (booleans / enums) is encoded; the
//! module contains no narrative content.
//!
//! ## § 177 Abs. 1 StGB - Sexueller Übergriff (basic offence)
//!
//! > Wer gegen den erkennbaren Willen einer anderen Person sexuelle Handlungen
//! > an dieser Person vornimmt oder von ihr vornehmen lässt [...], wird mit
//! > Freiheitsstrafe von sechs Monaten bis zu fünf Jahren bestraft.
//!
//! **English**: Whoever performs sexual acts on another person, or has them
//! performed, against the **recognisable will** of that person is punished with
//! imprisonment of **six months to five years** → `imprisonment(6, 60)`.
//!
//! ## § 177 Abs. 2 StGB - Equated situations (gleichgestellte Tatsituationen)
//!
//! Abs. 2 extends Abs. 1 to situations such as: the victim being unable to form
//! or express a contrary will; exploitation of a moment of surprise
//! (Überraschungsmoment); exploitation of a situation in which the victim is at
//! the offender's mercy; or where the offender has compelled the act by threat.
//! The Abs. 1 frame applies.
//!
//! ## § 177 Abs. 5 StGB - Sexuelle Nötigung (qualified by coercion)
//!
//! The offence is committed with **Gewalt** (force), by **Drohung mit
//! gegenwärtiger Gefahr für Leib oder Leben** (threat of present danger to life
//! or limb), or by exploiting a **schutzlose Lage** (a situation in which the
//! victim is exposed to the offender's influence). Punishment: **not less than
//! one year** → `at_least_months(12)`.
//!
//! ## § 177 Abs. 6 StGB - Vergewaltigung (especially serious case, Regelbeispiel)
//!
//! An especially serious case is, as a rule, present where the act involves
//! **Beischlaf** or similar **penetrative acts** (penetration), or is committed
//! **by several persons jointly** (gemeinschaftlich). Punishment: **not less
//! than two years** → `at_least_months(24)`.
//!
//! ## § 177 Abs. 7 / Abs. 8 StGB - Further qualifications
//!
//! - **Abs. 7**: the offender carries a weapon or other dangerous tool, or
//!   otherwise carries a tool to overcome resistance → **not less than three
//!   years** → `at_least_months(36)`.
//! - **Abs. 8**: the offender uses a weapon or dangerous tool, severely
//!   physically maltreats the victim, or places the victim in danger of death →
//!   **not less than five years** → `at_least_months(60)`.
//!
//! ## § 178 StGB - Sexueller Übergriff / Vergewaltigung mit Todesfolge
//!
//! Where the offender causes the death of the victim **at least through gross
//! negligence (wenigstens leichtfertig)**, the punishment is **life
//! imprisonment or imprisonment of not less than ten years**. This is encoded
//! literally as `Strafrahmen { min_months: Some(120), max_months: None,
//! fine_alternative: false }`: `max_months = None` permits life imprisonment,
//! while `min_months = Some(120)` sets the ten-year floor.
//!
//! ## § 184 StGB - Verbreitung pornographischer Inhalte
//!
//! Framework offence of disseminating pornographic content (to minors, in
//! public, etc.): imprisonment of **up to one year or a fine** →
//! `up_to_months_or_fine(12)`.
//!
//! ## § 184b StGB - Verbreitung kinderpornographischer Inhalte
//!
//! Framework offence of disseminating child pornographic content: **not less
//! than one year** → `at_least_months(12)`.
//!
//! All § 177 offences are intentional offences (Vorsatzdelikte); negligence is
//! not punishable (§ 15 StGB). The Erheblichkeitsschwelle of § 184h Nr. 1 StGB
//! applies throughout: only sexual acts of some significance are relevant.

use serde::{Deserialize, Serialize};

use crate::stgb::error::{Result, StgbError};
use crate::stgb::strafe::Strafrahmen;

/// The situation (Tatsituation) establishing a sexual assault under
/// § 177 Abs. 1 / Abs. 2 StGB.
///
/// Abs. 1 covers the act done against the recognisable contrary will; Abs. 2
/// equates further situations in which protection is owed even where no contrary
/// will is (or can be) recognisably expressed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tatsituation177 {
    /// Act against the recognisable contrary will (gegen den erkennbaren
    /// entgegenstehenden Willen), § 177 Abs. 1 StGB.
    ErkennbarerEntgegenstehenderWille,
    /// The victim is unable to form or express a contrary will
    /// (Willensunfähigkeit), § 177 Abs. 2 Nr. 1 StGB.
    OpferWillensunfaehig,
    /// Exploitation of a moment of surprise (Ausnutzung eines
    /// Überraschungsmoments), § 177 Abs. 2 Nr. 3 StGB.
    AusnutzungUeberraschung,
    /// Exploitation of a situation in which the victim is at the offender's
    /// mercy (Ausnutzung einer schutzlosen Lage), § 177 Abs. 2 Nr. 4 StGB.
    AusnutzungSchutzloseLage,
    /// The act is compelled by threat of a perceptible harm (Drohung mit einem
    /// empfindlichen Übel), § 177 Abs. 2 Nr. 5 StGB.
    Drohung,
}

/// A means of coercion (Nötigungsmittel) for sexual coercion under
/// § 177 Abs. 5 StGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Noetigungsmittel177 {
    /// Force (Gewalt), § 177 Abs. 5 Nr. 1 StGB.
    Gewalt,
    /// Threat of present danger to life or limb (Drohung mit gegenwärtiger
    /// Gefahr für Leib oder Leben), § 177 Abs. 5 Nr. 2 StGB.
    DrohungLeibLeben,
    /// Exploitation of a situation in which the victim is exposed to the
    /// offender's influence (Ausnutzung einer schutzlosen Lage),
    /// § 177 Abs. 5 Nr. 3 StGB.
    AusnutzungSchutzloseLage,
}

/// A qualifying circumstance (Qualifikation) raising the sentencing range under
/// § 177 Abs. 6 (mehrere), Abs. 7 and Abs. 8 StGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Qualifikation177 {
    /// The offender carries a weapon or other dangerous tool with them
    /// (Waffe oder gefährliches Werkzeug beisichgeführt), § 177 Abs. 7 Nr. 1 StGB.
    WaffeBeisichGefuehrt,
    /// The offender uses a weapon or dangerous tool (Waffe oder gefährliches
    /// Werkzeug verwendet), § 177 Abs. 8 Nr. 1 StGB.
    WaffeVerwendet,
    /// The offender severely physically maltreats the victim (schwere körperliche
    /// Misshandlung), § 177 Abs. 8 Nr. 2 lit. a StGB.
    SchwereMisshandlung,
    /// The offender places the victim in danger of death (Gefahr des Todes),
    /// § 177 Abs. 8 Nr. 2 lit. b StGB.
    Lebensgefahr,
    /// Commission by several persons jointly (gemeinschaftliche Begehung),
    /// § 177 Abs. 6 S. 2 Nr. 2 StGB.
    GemeinschaftlicheBegehung,
}

impl Qualifikation177 {
    /// Whether this circumstance belongs to the higher qualification of
    /// § 177 Abs. 8 StGB (use of a weapon, severe maltreatment, danger of death),
    /// which carries the five-year floor.
    #[must_use]
    pub fn ist_abs8(&self) -> bool {
        matches!(
            self,
            Qualifikation177::WaffeVerwendet
                | Qualifikation177::SchwereMisshandlung
                | Qualifikation177::Lebensgefahr
        )
    }
}

/// The specific offence against sexual self-determination applicable to a case,
/// §§ 177-184b StGB.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SexualOffence {
    /// Sexual assault (sexueller Übergriff), § 177 Abs. 1 / Abs. 2 StGB.
    SexuellerUebergriff {
        /// The situation establishing the offence.
        situation: Tatsituation177,
    },
    /// Sexual coercion (sexuelle Nötigung), § 177 Abs. 5 StGB.
    SexuelleNoetigung {
        /// The means of coercion employed.
        mittel: Noetigungsmittel177,
    },
    /// Rape (Vergewaltigung) as an especially serious case, § 177 Abs. 6 StGB.
    Vergewaltigung {
        /// Whether the act involved Beischlaf or similar penetrative acts.
        penetration: bool,
        /// Whether the act was committed by several persons jointly.
        gemeinschaftlich: bool,
    },
    /// Qualified sexual assault, § 177 Abs. 7 / Abs. 8 StGB.
    QualifizierterUebergriff {
        /// The qualifying circumstances present (at least one).
        qualifikation: Vec<Qualifikation177>,
        /// Whether the higher five-year floor of Abs. 8 is engaged.
        abs8: bool,
    },
    /// Sexual assault / rape resulting in death (mit Todesfolge), § 178 StGB.
    MitTodesfolge,
    /// Dissemination of pornographic content (Verbreitung pornographischer
    /// Inhalte), § 184 StGB.
    VerbreitungPornographie,
    /// Dissemination of child pornographic content (Verbreitung
    /// kinderpornographischer Inhalte), § 184b StGB.
    VerbreitungKinderpornographie,
}

impl SexualOffence {
    /// The § citation of the offence.
    #[must_use]
    pub fn paragraph(&self) -> &'static str {
        match self {
            SexualOffence::SexuellerUebergriff { .. } => "§ 177 Abs. 1 StGB",
            SexualOffence::SexuelleNoetigung { .. } => "§ 177 Abs. 5 StGB",
            SexualOffence::Vergewaltigung { .. } => "§ 177 Abs. 6 StGB",
            SexualOffence::QualifizierterUebergriff { abs8, .. } => {
                if *abs8 {
                    "§ 177 Abs. 8 StGB"
                } else {
                    "§ 177 Abs. 7 StGB"
                }
            }
            SexualOffence::MitTodesfolge => "§ 178 StGB",
            SexualOffence::VerbreitungPornographie => "§ 184 StGB",
            SexualOffence::VerbreitungKinderpornographie => "§ 184b StGB",
        }
    }

    /// The statutory sentencing range (Strafrahmen) of the offence.
    #[must_use]
    pub fn strafrahmen(&self) -> Strafrahmen {
        match self {
            // § 177 Abs. 1 StGB - six months to five years.
            SexualOffence::SexuellerUebergriff { .. } => Strafrahmen::imprisonment(6, 60),
            // § 177 Abs. 5 StGB - not less than one year.
            SexualOffence::SexuelleNoetigung { .. } => Strafrahmen::at_least_months(12),
            // § 177 Abs. 6 StGB - not less than two years.
            SexualOffence::Vergewaltigung { .. } => Strafrahmen::at_least_months(24),
            // § 177 Abs. 7 StGB - not less than three years;
            // § 177 Abs. 8 StGB - not less than five years.
            SexualOffence::QualifizierterUebergriff { abs8, .. } => {
                if *abs8 {
                    Strafrahmen::at_least_months(60)
                } else {
                    Strafrahmen::at_least_months(36)
                }
            }
            // § 178 StGB - life imprisonment or not less than ten years.
            // Constructed literally: max_months None permits life, min_months
            // Some(120) sets a ten-year floor.
            SexualOffence::MitTodesfolge => Strafrahmen {
                min_months: Some(120),
                max_months: None,
                fine_alternative: false,
            },
            // § 184 StGB - up to one year or a fine.
            SexualOffence::VerbreitungPornographie => Strafrahmen::up_to_months_or_fine(12),
            // § 184b StGB - not less than one year.
            SexualOffence::VerbreitungKinderpornographie => Strafrahmen::at_least_months(12),
        }
    }
}

/// A case of an offence against sexual self-determination, §§ 177-184b StGB.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SexualOffenceCase {
    /// Description of the victim (Tatopfer). Must not be empty.
    pub opfer: String,
    /// Whether a sexual act (sexuelle Handlung) was performed, § 184h StGB.
    pub sexuelle_handlung: bool,
    /// Whether the act crosses the threshold of significance
    /// (Erheblichkeitsschwelle, § 184h Nr. 1 StGB): only acts of some
    /// significance with respect to the protected interest are relevant.
    pub erheblichkeit: bool,
    /// Whether the offender acted intentionally (Vorsatz). Required for all
    /// § 177 offences; negligence is not punishable (§ 15 StGB).
    pub vorsatz: bool,
    /// For § 178: whether the death of the victim occurred (Todesfolge
    /// eingetreten).
    pub todesfolge_eingetreten: bool,
    /// For § 178: whether the death was caused at least through gross negligence
    /// (wenigstens leichtfertig).
    pub todesfolge_wenigstens_leichtfertig: bool,
    /// The offence the case is charged under.
    pub offence: SexualOffence,
}

/// Validate a case of an offence against sexual self-determination under
/// §§ 177-184b StGB.
///
/// Common elements: a sexual act of some significance must be present
/// (sexuelle Handlung above the Erheblichkeitsschwelle, § 184h Nr. 1 StGB), and
/// the offender must have acted intentionally (§§ 177, 15 StGB). Specific
/// offences add further requirements:
///
/// - **Vergewaltigung (§ 177 Abs. 6 StGB)** requires the Regelbeispiel to be
///   present, i.e. either penetration or joint commission.
/// - **Qualifizierter Übergriff (§ 177 Abs. 7 / Abs. 8 StGB)** requires at least
///   one qualifying circumstance.
/// - **Mit Todesfolge (§ 178 StGB)** requires that death occurred and was caused
///   at least through gross negligence.
///
/// The framework offences § 184 and § 184b StGB are validated only for the
/// common elements at this abstraction level.
///
/// # Errors
/// - [`StgbError::InvalidField`] if the victim description is empty.
/// - [`StgbError::TatbestandNotFulfilled`] if no sexual act is present, if it
///   does not cross the threshold of significance, or if the Vergewaltigung
///   Regelbeispiel (penetration or joint commission) is absent.
/// - [`StgbError::FahrlaessigkeitNichtStrafbar`] if intent is missing.
/// - [`StgbError::AbsichtMissing`] if a qualified offence is charged without any
///   qualifying circumstance.
/// - [`StgbError::NoSchuldform`] if § 178 is charged but the death did not occur
///   or was not caused at least through gross negligence.
pub fn validate_sexual_offence(case: &SexualOffenceCase) -> Result<()> {
    if case.opfer.trim().is_empty() {
        return Err(StgbError::InvalidField {
            field: "opfer (Tatopfer) darf nicht leer sein (§§ 177 ff. StGB)".to_string(),
        });
    }
    if !case.sexuelle_handlung {
        return Err(StgbError::TatbestandNotFulfilled {
            element: "sexuelle Handlung (§ 184h StGB)".to_string(),
        });
    }
    if !case.erheblichkeit {
        return Err(StgbError::TatbestandNotFulfilled {
            element: "Erheblichkeit der sexuellen Handlung (§ 184h Nr. 1 StGB)".to_string(),
        });
    }
    // All § 177 offences are intentional offences (Vorsatzdelikte). The framework
    // offences § 184 / § 184b StGB are likewise intentional at this abstraction.
    if !case.vorsatz {
        return Err(StgbError::FahrlaessigkeitNichtStrafbar);
    }

    match &case.offence {
        // § 177 Abs. 6 StGB Regelbeispiel: penetrative act or joint
        // commission. Absent either, the especially serious case is not made
        // out and the act remains a basic sexual assault.
        SexualOffence::Vergewaltigung {
            penetration,
            gemeinschaftlich,
        } if !*penetration && !*gemeinschaftlich => {
            return Err(StgbError::TatbestandNotFulfilled {
                element: "Regelbeispiel der Vergewaltigung: Penetration oder \
                          gemeinschaftliche Begehung (§ 177 Abs. 6 StGB)"
                    .to_string(),
            });
        }
        SexualOffence::Vergewaltigung { .. } => {}
        SexualOffence::QualifizierterUebergriff { qualifikation, .. }
            if qualifikation.is_empty() =>
        {
            return Err(StgbError::AbsichtMissing {
                detail: "qualifizierter Übergriff setzt mindestens eine \
                         Qualifikation voraus (§ 177 Abs. 7/Abs. 8 StGB)"
                    .to_string(),
            });
        }
        SexualOffence::QualifizierterUebergriff { .. } => {}
        // § 178 StGB: erfolgsqualifiziertes Delikt requiring the death to have
        // occurred and to have been caused at least leichtfertig.
        SexualOffence::MitTodesfolge
            if !case.todesfolge_eingetreten || !case.todesfolge_wenigstens_leichtfertig =>
        {
            return Err(StgbError::NoSchuldform);
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_case(offence: SexualOffence) -> SexualOffenceCase {
        SexualOffenceCase {
            opfer: "Opfer".to_string(),
            sexuelle_handlung: true,
            erheblichkeit: true,
            vorsatz: true,
            todesfolge_eingetreten: false,
            todesfolge_wenigstens_leichtfertig: false,
            offence,
        }
    }

    #[test]
    fn sexueller_uebergriff_abs1_valid_and_range() {
        let o = SexualOffence::SexuellerUebergriff {
            situation: Tatsituation177::ErkennbarerEntgegenstehenderWille,
        };
        assert_eq!(o.paragraph(), "§ 177 Abs. 1 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 6);
        assert_eq!(r.max_months, Some(60));
        assert!(!r.allows_life());
        assert!(validate_sexual_offence(&base_case(o)).is_ok());
    }

    #[test]
    fn missing_sexuelle_handlung_is_tatbestand_not_fulfilled() {
        let mut c = base_case(SexualOffence::SexuellerUebergriff {
            situation: Tatsituation177::OpferWillensunfaehig,
        });
        c.sexuelle_handlung = false;
        assert!(matches!(
            validate_sexual_offence(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }

    #[test]
    fn below_erheblichkeitsschwelle_is_tatbestand_not_fulfilled() {
        let mut c = base_case(SexualOffence::SexuellerUebergriff {
            situation: Tatsituation177::AusnutzungUeberraschung,
        });
        c.erheblichkeit = false;
        assert!(matches!(
            validate_sexual_offence(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }

    #[test]
    fn missing_intent_is_fahrlaessigkeit_nicht_strafbar() {
        let mut c = base_case(SexualOffence::SexuellerUebergriff {
            situation: Tatsituation177::Drohung,
        });
        c.vorsatz = false;
        assert!(matches!(
            validate_sexual_offence(&c),
            Err(StgbError::FahrlaessigkeitNichtStrafbar)
        ));
    }

    #[test]
    fn sexuelle_noetigung_abs5_range_at_least_one_year() {
        let o = SexualOffence::SexuelleNoetigung {
            mittel: Noetigungsmittel177::Gewalt,
        };
        assert_eq!(o.paragraph(), "§ 177 Abs. 5 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 12);
        assert!(!r.allows_life());
        assert!(validate_sexual_offence(&base_case(o)).is_ok());
    }

    #[test]
    fn sexuelle_noetigung_alle_mittel() {
        for mittel in [
            Noetigungsmittel177::Gewalt,
            Noetigungsmittel177::DrohungLeibLeben,
            Noetigungsmittel177::AusnutzungSchutzloseLage,
        ] {
            let o = SexualOffence::SexuelleNoetigung { mittel };
            assert_eq!(o.strafrahmen().effective_min_months(), 12);
            assert!(validate_sexual_offence(&base_case(o)).is_ok());
        }
    }

    #[test]
    fn vergewaltigung_abs6_range_at_least_two_years() {
        let o = SexualOffence::Vergewaltigung {
            penetration: true,
            gemeinschaftlich: false,
        };
        assert_eq!(o.paragraph(), "§ 177 Abs. 6 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 24);
        assert!(!r.allows_life());
        assert!(validate_sexual_offence(&base_case(o)).is_ok());
    }

    #[test]
    fn vergewaltigung_requires_penetration_or_gemeinschaftlich() {
        let o_neither = SexualOffence::Vergewaltigung {
            penetration: false,
            gemeinschaftlich: false,
        };
        assert!(matches!(
            validate_sexual_offence(&base_case(o_neither)),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
        // Joint commission alone fulfils the Regelbeispiel.
        let o_joint = SexualOffence::Vergewaltigung {
            penetration: false,
            gemeinschaftlich: true,
        };
        assert!(validate_sexual_offence(&base_case(o_joint)).is_ok());
    }

    #[test]
    fn qualifizierter_uebergriff_abs7_range_at_least_three_years() {
        let o = SexualOffence::QualifizierterUebergriff {
            qualifikation: vec![Qualifikation177::WaffeBeisichGefuehrt],
            abs8: false,
        };
        assert_eq!(o.paragraph(), "§ 177 Abs. 7 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 36);
        assert!(!r.allows_life());
        assert!(validate_sexual_offence(&base_case(o)).is_ok());
    }

    #[test]
    fn qualifizierter_uebergriff_abs8_range_at_least_five_years() {
        let o = SexualOffence::QualifizierterUebergriff {
            qualifikation: vec![
                Qualifikation177::WaffeVerwendet,
                Qualifikation177::Lebensgefahr,
            ],
            abs8: true,
        };
        assert_eq!(o.paragraph(), "§ 177 Abs. 8 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 60);
        assert!(!r.allows_life());
        assert!(validate_sexual_offence(&base_case(o)).is_ok());
    }

    #[test]
    fn qualifizierter_uebergriff_requires_nonempty_qualifikation() {
        let o = SexualOffence::QualifizierterUebergriff {
            qualifikation: vec![],
            abs8: false,
        };
        assert!(matches!(
            validate_sexual_offence(&base_case(o)),
            Err(StgbError::AbsichtMissing { .. })
        ));
    }

    #[test]
    fn qualifikation_abs8_classification() {
        assert!(Qualifikation177::WaffeVerwendet.ist_abs8());
        assert!(Qualifikation177::SchwereMisshandlung.ist_abs8());
        assert!(Qualifikation177::Lebensgefahr.ist_abs8());
        assert!(!Qualifikation177::WaffeBeisichGefuehrt.ist_abs8());
        assert!(!Qualifikation177::GemeinschaftlicheBegehung.ist_abs8());
    }

    #[test]
    fn mit_todesfolge_allows_life_and_has_ten_year_floor() {
        let o = SexualOffence::MitTodesfolge;
        assert_eq!(o.paragraph(), "§ 178 StGB");
        let r = o.strafrahmen();
        // § 178 StGB: life imprisonment or not less than ten years.
        assert!(r.allows_life());
        assert_eq!(r.effective_min_months(), 120);
        assert_eq!(r.min_months, Some(120));
        assert_eq!(r.max_months, None);
        assert!(!r.fine_alternative);
    }

    #[test]
    fn mit_todesfolge_requires_death_at_least_leichtfertig() {
        // Death not yet established.
        let mut c = base_case(SexualOffence::MitTodesfolge);
        assert!(matches!(
            validate_sexual_offence(&c),
            Err(StgbError::NoSchuldform)
        ));
        // Death occurred but not at least leichtfertig.
        c.todesfolge_eingetreten = true;
        assert!(matches!(
            validate_sexual_offence(&c),
            Err(StgbError::NoSchuldform)
        ));
        // Death occurred and was at least leichtfertig.
        c.todesfolge_wenigstens_leichtfertig = true;
        assert!(validate_sexual_offence(&c).is_ok());
    }

    #[test]
    fn verbreitung_pornographie_range_with_fine() {
        let o = SexualOffence::VerbreitungPornographie;
        assert_eq!(o.paragraph(), "§ 184 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.max_months, Some(12));
        assert!(r.fine_alternative);
        assert!(!r.allows_life());
        assert!(validate_sexual_offence(&base_case(o)).is_ok());
    }

    #[test]
    fn verbreitung_kinderpornographie_range_at_least_one_year() {
        let o = SexualOffence::VerbreitungKinderpornographie;
        assert_eq!(o.paragraph(), "§ 184b StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 12);
        assert!(!r.fine_alternative);
        assert!(!r.allows_life());
        assert!(validate_sexual_offence(&base_case(o)).is_ok());
    }

    #[test]
    fn empty_victim_is_invalid_field() {
        let mut c = base_case(SexualOffence::SexuellerUebergriff {
            situation: Tatsituation177::AusnutzungSchutzloseLage,
        });
        c.opfer = "   ".to_string();
        assert!(matches!(
            validate_sexual_offence(&c),
            Err(StgbError::InvalidField { .. })
        ));
    }
}
