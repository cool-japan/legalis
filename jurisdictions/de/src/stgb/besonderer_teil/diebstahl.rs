//! StGB §§ 242-248c - Theft Offences (Diebstahl)
//!
//! ## § 242 StGB - Diebstahl (basic theft)
//!
//! > (1) Wer eine fremde bewegliche Sache einem anderen in der Absicht wegnimmt,
//! > die Sache sich oder einem Dritten rechtswidrig zuzueignen, wird mit
//! > Freiheitsstrafe bis zu fünf Jahren oder mit Geldstrafe bestraft.
//! > (2) Der Versuch ist strafbar.
//!
//! **English**: Whoever takes movable property belonging to another away from that
//! person with the intent of unlawfully appropriating the property for himself or a
//! third party is punished with imprisonment of **up to five years or a fine**. The
//! **attempt is punishable** (§ 242 Abs. 2 StGB).
//!
//! The objective elements (objektiver Tatbestand) are:
//! - **fremde bewegliche Sache** - movable property belonging to another (not the
//!   offender's own, not ownerless, and corporeal/movable);
//! - **Wegnahme** - the taking, defined as the *breaking* of another's custody
//!   (Bruch fremden Gewahrsams) and the *establishment* of new custody
//!   (Begründung neuen Gewahrsams).
//!
//! The subjective elements (subjektiver Tatbestand) are:
//! - **Vorsatz** - intent as to all objective elements (§ 15 StGB);
//! - **Absicht rechtswidriger Zueignung** - the intent of *unlawful* appropriation
//!   (Zueignungsabsicht) including awareness of the unlawfulness of the intended
//!   appropriation (Rechtswidrigkeit der Zueignung).
//!
//! ## § 243 StGB - Besonders schwerer Fall des Diebstahls (Regelbeispiele)
//!
//! § 243 is **not a qualification** but a sentencing rule containing
//! *Regelbeispiele* (standard examples) that *indicate* an especially serious case.
//! Where one applies, the range increases to imprisonment of **three months to ten
//! years** (§ 243 Abs. 1 StGB). Examples include burglary by breaking in
//! (Einbruchdiebstahl, Nr. 1), use of a false key (Nr. 1), professional commission
//! (gewerbsmäßig, Nr. 3), and theft from a place of worship (Nr. 4).
//!
//! Crucially, **§ 243 Abs. 2 StGB excludes** the rule where the theft relates to a
//! thing of **low value** (geringwertige Sache): the Regelbeispiel does not lead to
//! the elevated range.
//!
//! ## § 244 StGB - Diebstahl mit Waffen; Bandendiebstahl; Wohnungseinbruchdiebstahl
//!
//! These are genuine **qualifications** (Qualifikationen). The range is imprisonment
//! of **six months to ten years** (§ 244 Abs. 1 StGB) for:
//! - carrying a weapon or dangerous tool (Nr. 1a);
//! - theft as a member of a gang (Bandendiebstahl, Nr. 2);
//! - burglary of a dwelling (Wohnungseinbruchdiebstahl, Nr. 3).
//!
//! § 244 Abs. 4 StGB raises burglary of a **permanently used private dwelling**
//! (dauerhaft genutzte Privatwohnung) to imprisonment of **one to ten years**.
//!
//! ## § 244a StGB - Schwerer Bandendiebstahl (aggravated gang theft)
//!
//! Gang theft combined with a § 243 Regelbeispiel or a § 244 qualification:
//! imprisonment of **one to ten years**.
//!
//! ## § 248a StGB - Diebstahl geringwertiger Sachen (theft of low-value things)
//!
//! Theft (and unlawful appropriation) of a thing of low value is prosecuted **only
//! on application** (Strafantrag) unless the prosecuting authority affirms a
//! **special public interest** in prosecution (besonderes öffentliches Interesse).
//! It is therefore an *Antragsdelikt*; the statutory range is unchanged.
//!
//! ## § 248b StGB - Unbefugter Gebrauch eines Fahrzeugs (furtum usus)
//!
//! Using a motor vehicle or bicycle against the will of the entitled person:
//! imprisonment of **up to three years or a fine** (subsidiary to graver offences).
//!
//! ## § 248c StGB - Entziehung elektrischer Energie (abstraction of electricity)
//!
//! Abstracting electrical energy from an installation by means of a conductor not
//! intended for the proper withdrawal with the intent of unlawful appropriation:
//! imprisonment of **up to five years or a fine**.

use serde::{Deserialize, Serialize};

use crate::stgb::error::{Result, StgbError};
use crate::stgb::strafe::Strafrahmen;

/// A standard example (Regelbeispiel) of an especially serious case of theft,
/// § 243 Abs. 1 S. 2 StGB.
///
/// Each variant indicates - but does not conclusively establish - an especially
/// serious case (besonders schwerer Fall). The indication can be rebutted, and per
/// § 243 Abs. 2 StGB it is excluded for things of low value (geringwertige Sachen).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Regelbeispiel243 {
    /// Breaking, climbing or intruding into a building, dwelling, business or
    /// other enclosed space (§ 243 Abs. 1 S. 2 Nr. 1 StGB) - burglary.
    Einbruchdiebstahl,
    /// Climbing in (Einsteigen) into the enclosed space (§ 243 Abs. 1 S. 2 Nr. 1
    /// StGB, alternative).
    Einsteigen,
    /// Use of a false key or other tool not intended for proper opening
    /// (falscher Schlüssel, § 243 Abs. 1 S. 2 Nr. 1 StGB, alternative).
    FalscherSchluessel,
    /// Theft of a thing secured against removal by a protective device
    /// (§ 243 Abs. 1 S. 2 Nr. 2 StGB).
    GeschuetzteSache,
    /// Professional commission of the theft (gewerbsmäßig, § 243 Abs. 1 S. 2 Nr. 3
    /// StGB).
    Gewerbsmaessig,
    /// Theft from a church or other building/room dedicated to religious worship
    /// (§ 243 Abs. 1 S. 2 Nr. 4 StGB).
    AusKirche,
    /// Theft of an object of significance for science, art, history or technical
    /// development (§ 243 Abs. 1 S. 2 Nr. 5 StGB).
    KulturGut,
    /// Exploiting the helplessness, an accident or a common danger of another
    /// (§ 243 Abs. 1 S. 2 Nr. 6 StGB).
    AusnutzungHilflosigkeit,
    /// Theft of a firearm or war weapon (§ 243 Abs. 1 S. 2 Nr. 7 StGB).
    Schusswaffe,
}

impl Regelbeispiel243 {
    /// The § citation of the standard example.
    #[must_use]
    pub fn paragraph(&self) -> &'static str {
        match self {
            Regelbeispiel243::Einbruchdiebstahl
            | Regelbeispiel243::Einsteigen
            | Regelbeispiel243::FalscherSchluessel => "§ 243 Abs. 1 S. 2 Nr. 1 StGB",
            Regelbeispiel243::GeschuetzteSache => "§ 243 Abs. 1 S. 2 Nr. 2 StGB",
            Regelbeispiel243::Gewerbsmaessig => "§ 243 Abs. 1 S. 2 Nr. 3 StGB",
            Regelbeispiel243::AusKirche => "§ 243 Abs. 1 S. 2 Nr. 4 StGB",
            Regelbeispiel243::KulturGut => "§ 243 Abs. 1 S. 2 Nr. 5 StGB",
            Regelbeispiel243::AusnutzungHilflosigkeit => "§ 243 Abs. 1 S. 2 Nr. 6 StGB",
            Regelbeispiel243::Schusswaffe => "§ 243 Abs. 1 S. 2 Nr. 7 StGB",
        }
    }
}

/// A qualification of theft under § 244 StGB.
///
/// Unlike the Regelbeispiele of § 243 (which are mere sentencing rules), these are
/// genuine qualifications: their fulfilment is part of the offence definition and is
/// not excluded for things of low value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Qualifikation244 {
    /// Carrying a weapon or other dangerous tool (§ 244 Abs. 1 Nr. 1a StGB).
    WaffenOderWerkzeug,
    /// Carrying a tool or means to overcome resistance by force or threat
    /// (§ 244 Abs. 1 Nr. 1b StGB).
    MittelGegenWiderstand,
    /// Theft as a member of a gang formed for the continued commission of theft or
    /// robbery (Bandendiebstahl, § 244 Abs. 1 Nr. 2 StGB).
    Bandendiebstahl,
    /// Burglary of a dwelling (Wohnungseinbruchdiebstahl, § 244 Abs. 1 Nr. 3 StGB).
    Wohnungseinbruchdiebstahl,
    /// Burglary of a permanently used private dwelling
    /// (§ 244 Abs. 4 StGB) - higher minimum sentence.
    PrivatwohnungEinbruch,
}

impl Qualifikation244 {
    /// The § citation of the qualification.
    #[must_use]
    pub fn paragraph(&self) -> &'static str {
        match self {
            Qualifikation244::WaffenOderWerkzeug => "§ 244 Abs. 1 Nr. 1a StGB",
            Qualifikation244::MittelGegenWiderstand => "§ 244 Abs. 1 Nr. 1b StGB",
            Qualifikation244::Bandendiebstahl => "§ 244 Abs. 1 Nr. 2 StGB",
            Qualifikation244::Wohnungseinbruchdiebstahl => "§ 244 Abs. 1 Nr. 3 StGB",
            Qualifikation244::PrivatwohnungEinbruch => "§ 244 Abs. 4 StGB",
        }
    }
}

/// The specific theft offence applicable to a case (§§ 242-248c StGB).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TheftOffence {
    /// Basic theft (Grunddiebstahl, § 242 StGB).
    Grunddiebstahl {
        /// Whether the theft relates to a thing of low value (geringwertige Sache).
        /// If `true`, § 248a StGB makes the offence an Antragsdelikt (prosecution
        /// only on application or upon a special public interest).
        geringwertig: bool,
    },
    /// Especially serious case of theft (besonders schwerer Fall, § 243 StGB),
    /// indicated by one or more Regelbeispiele.
    BesondersSchwererFall {
        /// The standard examples present (at least one is required).
        regelbeispiele: Vec<Regelbeispiel243>,
        /// Whether the theft relates to a thing of low value (geringwertige Sache).
        /// Per § 243 Abs. 2 StGB this excludes the elevated range.
        geringwertig: bool,
    },
    /// Qualified theft (§ 244 StGB) - weapons, gang or burglary of a dwelling.
    QualifizierterDiebstahl244 {
        /// The qualification fulfilled.
        qualifikation: Qualifikation244,
    },
    /// Aggravated gang theft (schwerer Bandendiebstahl, § 244a StGB).
    SchwererBandendiebstahl244a,
    /// Unauthorised use of a vehicle (unbefugter Fahrzeuggebrauch, § 248b StGB).
    UnbefugterFahrzeuggebrauch248b,
    /// Abstraction of electrical energy (Entziehung elektrischer Energie,
    /// § 248c StGB).
    EntziehungElektrischerEnergie248c,
}

impl TheftOffence {
    /// The § citation of the offence.
    #[must_use]
    pub fn paragraph(&self) -> &'static str {
        match self {
            TheftOffence::Grunddiebstahl { .. } => "§ 242 StGB",
            TheftOffence::BesondersSchwererFall { .. } => "§ 243 StGB",
            TheftOffence::QualifizierterDiebstahl244 { qualifikation } => qualifikation.paragraph(),
            TheftOffence::SchwererBandendiebstahl244a => "§ 244a StGB",
            TheftOffence::UnbefugterFahrzeuggebrauch248b => "§ 248b StGB",
            TheftOffence::EntziehungElektrischerEnergie248c => "§ 248c StGB",
        }
    }

    /// The statutory sentencing range (Strafrahmen) of the offence.
    ///
    /// Note that for § 243 the elevated range only applies where a Regelbeispiel is
    /// established and not excluded by § 243 Abs. 2 StGB; where the theft relates to
    /// a thing of low value the offence falls back to the basic range of § 242 StGB.
    #[must_use]
    pub fn strafrahmen(&self) -> Strafrahmen {
        match self {
            // § 242 Abs. 1 StGB - up to five years (60 months) or a fine. The range
            // is the same for low-value theft; § 248a StGB only affects prosecution.
            TheftOffence::Grunddiebstahl { .. } => Strafrahmen::up_to_months_or_fine(60),
            // § 243 Abs. 1 StGB - three months to ten years; but § 243 Abs. 2 StGB
            // falls back to the basic range for things of low value.
            TheftOffence::BesondersSchwererFall { geringwertig, .. } => {
                if *geringwertig {
                    Strafrahmen::up_to_months_or_fine(60)
                } else {
                    Strafrahmen::imprisonment(3, 120)
                }
            }
            // § 244 StGB - six months to ten years; § 244 Abs. 4 StGB (private
            // dwelling burglary) one to ten years.
            TheftOffence::QualifizierterDiebstahl244 { qualifikation } => match qualifikation {
                Qualifikation244::PrivatwohnungEinbruch => Strafrahmen::imprisonment(12, 120),
                _ => Strafrahmen::imprisonment(6, 120),
            },
            // § 244a Abs. 1 StGB - one to ten years.
            TheftOffence::SchwererBandendiebstahl244a => Strafrahmen::imprisonment(12, 120),
            // § 248b Abs. 1 StGB - up to three years (36 months) or a fine.
            TheftOffence::UnbefugterFahrzeuggebrauch248b => Strafrahmen::up_to_months_or_fine(36),
            // § 248c Abs. 1 StGB - up to five years (60 months) or a fine.
            TheftOffence::EntziehungElektrischerEnergie248c => {
                Strafrahmen::up_to_months_or_fine(60)
            }
        }
    }

    /// Whether the offence is a *Zueignungsdelikt* requiring intent of unlawful
    /// appropriation (Zueignungsabsicht).
    ///
    /// § 248b StGB (unauthorised use) requires only the intent to *use*, not to
    /// appropriate (it is a furtum usus); all the other theft offences here require
    /// Zueignungsabsicht.
    #[must_use]
    pub fn requires_zueignungsabsicht(&self) -> bool {
        !matches!(self, TheftOffence::UnbefugterFahrzeuggebrauch248b)
    }
}

/// A theft case (Diebstahl), §§ 242-248c StGB, with its objective and subjective
/// elements (Tatbestandsmerkmale).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheftCase {
    /// Description of the object of the offence (Tatobjekt).
    pub tatobjekt: String,
    /// Whether the object is a movable thing belonging to another (fremde
    /// bewegliche Sache); the offender's own or ownerless things do not qualify.
    pub fremde_bewegliche_sache: bool,
    /// Wegnahme, element 1: breaking of another's custody (Bruch fremden
    /// Gewahrsams).
    pub bruch_fremden_gewahrsams: bool,
    /// Wegnahme, element 2: establishment of new custody (Begründung neuen
    /// Gewahrsams).
    pub begruendung_neuen_gewahrsams: bool,
    /// Intent as to all objective elements (Vorsatz, § 15 StGB).
    pub vorsatz: bool,
    /// Intent of appropriation (Zueignungsabsicht); not required for § 248b StGB.
    pub zueignungsabsicht: bool,
    /// Awareness/intent that the appropriation is unlawful (Rechtswidrigkeit der
    /// Zueignung); a claim of right (fälliger Anspruch) excludes it.
    pub rechtswidrigkeit_der_zueignung: bool,
    /// Whether a criminal complaint has been filed (Strafantrag gestellt); relevant
    /// for § 248a StGB (theft of low-value things).
    pub strafantrag_gestellt: bool,
    /// Whether the prosecuting authority affirms a special public interest in
    /// prosecution (besonderes öffentliches Interesse), § 248a StGB.
    pub oeffentliches_interesse: bool,
    /// The offence the case is charged under.
    pub offence: TheftOffence,
}

/// Validate a theft case under §§ 242-248c StGB.
///
/// The elements are checked in the doctrinal order: first the object (fremde
/// bewegliche Sache), then the taking (Wegnahme = Bruch + Begründung neuen
/// Gewahrsams), then intent (Vorsatz) and - for the Zueignungsdelikte - the intent
/// of unlawful appropriation (Absicht rechtswidriger Zueignung). Offence-specific
/// requirements (Regelbeispiele under § 243, the Antragserfordernis under § 248a)
/// are validated last.
///
/// # Errors
/// - [`StgbError::InvalidTatobjekt`] if the object is missing or not a movable
///   thing belonging to another (fremde bewegliche Sache).
/// - [`StgbError::TatbestandNotFulfilled`] if the Wegnahme is incomplete, or if a
///   § 243 Regelbeispiel is excluded for a thing of low value (§ 243 Abs. 2 StGB),
///   or if no Regelbeispiel is present for § 243.
/// - [`StgbError::FahrlaessigkeitNichtStrafbar`] if intent (Vorsatz) is missing
///   (theft is an intent-only offence, § 15 StGB).
/// - [`StgbError::AbsichtMissing`] if the intent of unlawful appropriation
///   (Zueignungsabsicht / Rechtswidrigkeit der Zueignung) is missing on a
///   Zueignungsdelikt.
/// - [`StgbError::InvalidField`] if § 248a StGB applies and neither a Strafantrag
///   nor a special public interest exists.
pub fn validate_theft(case: &TheftCase) -> Result<()> {
    // (1) Objektiver Tatbestand: fremde bewegliche Sache.
    if case.tatobjekt.trim().is_empty() || !case.fremde_bewegliche_sache {
        return Err(StgbError::InvalidTatobjekt {
            detail: "Tatobjekt muss eine fremde bewegliche Sache sein (§ 242 Abs. 1 StGB)"
                .to_string(),
        });
    }

    // (2) Objektiver Tatbestand: Wegnahme = Bruch fremden + Begründung neuen
    // Gewahrsams (§ 242 Abs. 1 StGB).
    if !case.bruch_fremden_gewahrsams {
        return Err(StgbError::TatbestandNotFulfilled {
            element: "Wegnahme: Bruch fremden Gewahrsams (§ 242 Abs. 1 StGB)".to_string(),
        });
    }
    if !case.begruendung_neuen_gewahrsams {
        return Err(StgbError::TatbestandNotFulfilled {
            element: "Wegnahme: Begründung neuen Gewahrsams (§ 242 Abs. 1 StGB)".to_string(),
        });
    }

    // (3) Subjektiver Tatbestand: Vorsatz (§ 15 StGB). Theft is punishable only when
    // committed intentionally; negligence is not penalised.
    if !case.vorsatz {
        return Err(StgbError::FahrlaessigkeitNichtStrafbar);
    }

    // (4) Subjektiver Tatbestand: Absicht rechtswidriger Zueignung
    // (§ 242 Abs. 1 StGB) - except for § 248b StGB (furtum usus), which requires
    // only the intent to use.
    if case.offence.requires_zueignungsabsicht() {
        if !case.zueignungsabsicht {
            return Err(StgbError::AbsichtMissing {
                detail: "Absicht der Zueignung fehlt (§ 242 Abs. 1 StGB)".to_string(),
            });
        }
        if !case.rechtswidrigkeit_der_zueignung {
            return Err(StgbError::AbsichtMissing {
                detail: "Rechtswidrigkeit der Zueignung fehlt; bei fälligem Anspruch ist die \
                         Zueignung nicht rechtswidrig (§ 242 Abs. 1 StGB)"
                    .to_string(),
            });
        }
    }

    // (5) Offence-specific checks for § 243 StGB (Regelbeispiele).
    if let TheftOffence::BesondersSchwererFall {
        regelbeispiele,
        geringwertig,
    } = &case.offence
    {
        if regelbeispiele.is_empty() {
            return Err(StgbError::TatbestandNotFulfilled {
                element: "§ 243 StGB setzt mindestens ein Regelbeispiel voraus".to_string(),
            });
        }
        // § 243 Abs. 2 StGB: the elevated range is excluded for things of low value;
        // the Regelbeispiel does not lead to an especially serious case.
        if *geringwertig {
            return Err(StgbError::TatbestandNotFulfilled {
                element: "§ 243 Abs. 2 StGB: geringwertige Sache schließt Regelbeispiel aus"
                    .to_string(),
            });
        }
    }

    // § 248a StGB - theft of a thing of low value is an Antragsdelikt: it requires
    // either a criminal complaint (Strafantrag) or a special public interest.
    if is_geringwertig(&case.offence) && !case.strafantrag_gestellt && !case.oeffentliches_interesse
    {
        return Err(StgbError::InvalidField {
            field: "Strafantrag (§ 248a StGB): Diebstahl geringwertiger Sachen wird nur auf \
                    Antrag oder bei besonderem öffentlichem Interesse verfolgt"
                .to_string(),
        });
    }

    Ok(())
}

/// Whether the offence relates to a thing of low value (geringwertige Sache),
/// triggering the Antragserfordernis of § 248a StGB.
fn is_geringwertig(offence: &TheftOffence) -> bool {
    matches!(
        offence,
        TheftOffence::Grunddiebstahl { geringwertig: true }
            | TheftOffence::BesondersSchwererFall {
                geringwertig: true,
                ..
            }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_case(offence: TheftOffence) -> TheftCase {
        TheftCase {
            tatobjekt: "fremdes Fahrrad".to_string(),
            fremde_bewegliche_sache: true,
            bruch_fremden_gewahrsams: true,
            begruendung_neuen_gewahrsams: true,
            vorsatz: true,
            zueignungsabsicht: true,
            rechtswidrigkeit_der_zueignung: true,
            strafantrag_gestellt: false,
            oeffentliches_interesse: false,
            offence,
        }
    }

    #[test]
    fn grunddiebstahl_valid_and_range() {
        let o = TheftOffence::Grunddiebstahl {
            geringwertig: false,
        };
        assert_eq!(o.paragraph(), "§ 242 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.max_months, Some(60));
        assert!(r.fine_alternative);
        assert!(!r.allows_life());
        assert!(validate_theft(&base_case(o)).is_ok());
    }

    #[test]
    fn missing_fremde_bewegliche_sache_is_rejected() {
        let mut c = base_case(TheftOffence::Grunddiebstahl {
            geringwertig: false,
        });
        c.fremde_bewegliche_sache = false;
        assert!(matches!(
            validate_theft(&c),
            Err(StgbError::InvalidTatobjekt { .. })
        ));
    }

    #[test]
    fn empty_tatobjekt_is_rejected() {
        let mut c = base_case(TheftOffence::Grunddiebstahl {
            geringwertig: false,
        });
        c.tatobjekt = "   ".to_string();
        assert!(matches!(
            validate_theft(&c),
            Err(StgbError::InvalidTatobjekt { .. })
        ));
    }

    #[test]
    fn incomplete_wegnahme_is_rejected() {
        let mut c1 = base_case(TheftOffence::Grunddiebstahl {
            geringwertig: false,
        });
        c1.bruch_fremden_gewahrsams = false;
        assert!(matches!(
            validate_theft(&c1),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));

        let mut c2 = base_case(TheftOffence::Grunddiebstahl {
            geringwertig: false,
        });
        c2.begruendung_neuen_gewahrsams = false;
        assert!(matches!(
            validate_theft(&c2),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }

    #[test]
    fn missing_zueignungsabsicht_is_rejected() {
        let mut c = base_case(TheftOffence::Grunddiebstahl {
            geringwertig: false,
        });
        c.zueignungsabsicht = false;
        assert!(matches!(
            validate_theft(&c),
            Err(StgbError::AbsichtMissing { .. })
        ));
    }

    #[test]
    fn missing_rechtswidrigkeit_der_zueignung_is_rejected() {
        // A claim of right (fälliger Anspruch) excludes the unlawfulness of the
        // intended appropriation.
        let mut c = base_case(TheftOffence::Grunddiebstahl {
            geringwertig: false,
        });
        c.rechtswidrigkeit_der_zueignung = false;
        assert!(matches!(
            validate_theft(&c),
            Err(StgbError::AbsichtMissing { .. })
        ));
    }

    #[test]
    fn missing_vorsatz_is_not_punishable() {
        let mut c = base_case(TheftOffence::Grunddiebstahl {
            geringwertig: false,
        });
        c.vorsatz = false;
        assert!(matches!(
            validate_theft(&c),
            Err(StgbError::FahrlaessigkeitNichtStrafbar)
        ));
    }

    #[test]
    fn besonders_schwerer_fall_range_and_regelbeispiel() {
        let o = TheftOffence::BesondersSchwererFall {
            regelbeispiele: vec![Regelbeispiel243::Einbruchdiebstahl],
            geringwertig: false,
        };
        assert_eq!(o.paragraph(), "§ 243 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 3);
        assert_eq!(r.max_months, Some(120));
        assert!(!r.fine_alternative);
        assert!(validate_theft(&base_case(o)).is_ok());

        assert_eq!(
            Regelbeispiel243::Einbruchdiebstahl.paragraph(),
            "§ 243 Abs. 1 S. 2 Nr. 1 StGB"
        );
        assert_eq!(
            Regelbeispiel243::Gewerbsmaessig.paragraph(),
            "§ 243 Abs. 1 S. 2 Nr. 3 StGB"
        );
    }

    #[test]
    fn besonders_schwerer_fall_requires_regelbeispiel() {
        let o = TheftOffence::BesondersSchwererFall {
            regelbeispiele: vec![],
            geringwertig: false,
        };
        assert!(matches!(
            validate_theft(&base_case(o)),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }

    #[test]
    fn geringwertig_excludes_regelbeispiel() {
        // § 243 Abs. 2 StGB: low-value things exclude the Regelbeispiel.
        let o = TheftOffence::BesondersSchwererFall {
            regelbeispiele: vec![Regelbeispiel243::Einbruchdiebstahl],
            geringwertig: true,
        };
        // The range falls back to the basic § 242 range.
        let r = o.strafrahmen();
        assert_eq!(r.max_months, Some(60));
        assert!(r.fine_alternative);
        // Validation rejects the especially-serious-case charge.
        assert!(matches!(
            validate_theft(&base_case(o)),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }

    #[test]
    fn qualifizierter_diebstahl_244_range() {
        let o = TheftOffence::QualifizierterDiebstahl244 {
            qualifikation: Qualifikation244::WaffenOderWerkzeug,
        };
        assert_eq!(o.paragraph(), "§ 244 Abs. 1 Nr. 1a StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 6);
        assert_eq!(r.max_months, Some(120));
        assert!(!r.fine_alternative);
        assert!(validate_theft(&base_case(o)).is_ok());

        let bande = TheftOffence::QualifizierterDiebstahl244 {
            qualifikation: Qualifikation244::Bandendiebstahl,
        };
        assert_eq!(bande.paragraph(), "§ 244 Abs. 1 Nr. 2 StGB");
        assert_eq!(bande.strafrahmen().effective_min_months(), 6);
    }

    #[test]
    fn privatwohnung_einbruch_and_bandendiebstahl_244a_range() {
        // § 244 Abs. 4 StGB: one to ten years.
        let priv_wohnung = TheftOffence::QualifizierterDiebstahl244 {
            qualifikation: Qualifikation244::PrivatwohnungEinbruch,
        };
        assert_eq!(priv_wohnung.paragraph(), "§ 244 Abs. 4 StGB");
        let r1 = priv_wohnung.strafrahmen();
        assert_eq!(r1.effective_min_months(), 12);
        assert_eq!(r1.max_months, Some(120));

        // § 244a StGB: one to ten years.
        let o = TheftOffence::SchwererBandendiebstahl244a;
        assert_eq!(o.paragraph(), "§ 244a StGB");
        let r2 = o.strafrahmen();
        assert_eq!(r2.effective_min_months(), 12);
        assert_eq!(r2.max_months, Some(120));
        assert!(!r2.fine_alternative);
        assert!(validate_theft(&base_case(o)).is_ok());
    }

    #[test]
    fn unbefugter_fahrzeuggebrauch_248b_no_zueignungsabsicht_needed() {
        let o = TheftOffence::UnbefugterFahrzeuggebrauch248b;
        assert_eq!(o.paragraph(), "§ 248b StGB");
        let r = o.strafrahmen();
        assert_eq!(r.max_months, Some(36));
        assert!(r.fine_alternative);
        assert!(!o.requires_zueignungsabsicht());

        // No Zueignungsabsicht is required (furtum usus): a case lacking it is valid.
        let mut c = base_case(o);
        c.zueignungsabsicht = false;
        c.rechtswidrigkeit_der_zueignung = false;
        assert!(validate_theft(&c).is_ok());
    }

    #[test]
    fn entziehung_elektrischer_energie_248c_range() {
        let o = TheftOffence::EntziehungElektrischerEnergie248c;
        assert_eq!(o.paragraph(), "§ 248c StGB");
        let r = o.strafrahmen();
        assert_eq!(r.max_months, Some(60));
        assert!(r.fine_alternative);
        assert!(o.requires_zueignungsabsicht());
        assert!(validate_theft(&base_case(o)).is_ok());
    }

    #[test]
    fn geringwertiger_diebstahl_requires_strafantrag_248a() {
        // § 248a StGB: theft of a low-value thing is an Antragsdelikt. The typical
        // case is ordinary theft (§ 242 StGB) of a thing of low value.
        let offence = TheftOffence::Grunddiebstahl { geringwertig: true };
        assert!(is_geringwertig(&offence));

        // Without a Strafantrag and without a special public interest, prosecution
        // is barred.
        let mut c = base_case(offence);
        c.strafantrag_gestellt = false;
        c.oeffentliches_interesse = false;
        assert!(matches!(
            validate_theft(&c),
            Err(StgbError::InvalidField { .. })
        ));

        // A criminal complaint (Strafantrag) clears the hurdle.
        let mut c_antrag = c.clone();
        c_antrag.strafantrag_gestellt = true;
        assert!(validate_theft(&c_antrag).is_ok());

        // A special public interest (besonderes öffentliches Interesse) also clears
        // the hurdle.
        let mut c_interesse = c.clone();
        c_interesse.oeffentliches_interesse = true;
        assert!(validate_theft(&c_interesse).is_ok());
    }

    #[test]
    fn geringwertig_excludes_regelbeispiel_before_strafantrag() {
        // § 243 Abs. 2 StGB: for a low-value especially-serious case, the
        // Regelbeispiel exclusion fires; this is the legally correct outcome
        // (the case is not an especially serious one in the first place).
        let offence = TheftOffence::BesondersSchwererFall {
            regelbeispiele: vec![Regelbeispiel243::Einbruchdiebstahl],
            geringwertig: true,
        };
        assert!(is_geringwertig(&offence));
        assert!(matches!(
            validate_theft(&base_case(offence)),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }
}
