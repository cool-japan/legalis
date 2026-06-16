//! StGB §§ 267-282 - Forgery of Documents (Urkundenfälschung)
//!
//! ## Begriff der Urkunde (Concept of a document)
//!
//! A *Urkunde* (document) in the sense of §§ 267 ff. StGB is a **perpetuated
//! declaration of thought** (verkörperte Gedankenerklärung) that
//!
//! 1. is **suitable and intended to prove a legally relevant fact**
//!    (Beweisfunktion: Beweiseignung und Beweisbestimmung), and
//! 2. **lets its issuer be recognised** (Garantiefunktion: Erkennbarkeit des
//!    Ausstellers).
//!
//! A document is *echt* (genuine) when its **apparent issuer is its actual
//! issuer**. An *unechte Urkunde* misrepresents its issuer (Identitätstäuschung)
//! and is the object of § 267. By contrast, a merely **false-content** document
//! (schriftliche Lüge) — one whose issuer is correctly identified but whose
//! content is untrue — is generally **not** punishable under § 267.
//!
//! ## § 267 StGB - Urkundenfälschung
//!
//! > (1) Wer zur Täuschung im Rechtsverkehr eine unechte Urkunde herstellt, eine
//! > echte Urkunde verfälscht oder eine unechte oder verfälschte Urkunde
//! > gebraucht, wird mit Freiheitsstrafe bis zu fünf Jahren oder mit Geldstrafe
//! > bestraft.
//! > (2) Der Versuch ist strafbar.
//! > (3) In besonders schweren Fällen ist die Strafe Freiheitsstrafe von sechs
//! > Monaten bis zu zehn Jahren. [...]
//! > (4) [...] gewerbsmäßig als Mitglied einer Bande [...] Freiheitsstrafe von
//! > einem Jahr bis zu zehn Jahren [...].
//!
//! **English**: Whoever, **for the purpose of deception in legal relations**,
//! (1) **produces a forged** document, (2) **falsifies a genuine** document, or
//! (3) **uses** a forged or falsified document, is liable to imprisonment of up
//! to five years or a fine; the attempt is punishable (Abs. 2); especially
//! serious cases carry six months to ten years (Abs. 3); commission as a member
//! of a gang acting commercially carries one to ten years (Abs. 4).
//!
//! ## § 268 StGB - Fälschung technischer Aufzeichnungen
//!
//! Forgery of **technical records** (technische Aufzeichnung = an automated
//! record produced wholly or partly by a device): up to five years or a fine.
//!
//! ## § 269 StGB - Fälschung beweiserheblicher Daten
//!
//! Storing or altering **data relevant as evidence** such that, upon perception,
//! a forged or falsified document would exist, for the purpose of deception in
//! legal relations: up to five years or a fine.
//!
//! ## § 271 StGB - Mittelbare Falschbeurkundung
//!
//! Causing a false entry to be made in **public records, registers or
//! databases** (öffentliche Urkunden) about legally relevant facts: up to three
//! years or a fine.
//!
//! ## § 274 StGB - Urkundenunterdrückung
//!
//! **Suppressing, destroying or damaging** a document one is **not, or not
//! solely, entitled to**, with the **intent to cause disadvantage to another**
//! (Nachteilszufügungsabsicht): up to five years or a fine.
//!
//! ## § 277 StGB - Fälschung von Gesundheitszeugnissen
//!
//! Forgery of **health certificates** under a physician's name etc.: up to one
//! year or a fine.
//!
//! ## § 281 StGB - Missbrauch von Ausweispapieren
//!
//! **Misuse of identity papers** issued for another (or letting another use
//! one's own), for the purpose of deception in legal relations: up to one year or
//! a fine.

use serde::{Deserialize, Serialize};

use crate::stgb::error::{Result, StgbError};
use crate::stgb::strafe::Strafrahmen;

/// A document (Urkunde) in the sense of §§ 267 ff. StGB.
///
/// Modelled by the three doctrinal functions that, taken together, make a
/// perpetuated declaration of thought a *Urkunde*:
///
/// - [`perpetuierte_gedankenerklaerung`](Urkunde::perpetuierte_gedankenerklaerung)
///   — a thought is fixed in a (durable) physical form;
/// - [`beweisfunktion`](Urkunde::beweisfunktion) — it is suitable and intended to
///   prove a legally relevant fact (Beweiseignung und -bestimmung);
/// - [`garantiefunktion`](Urkunde::garantiefunktion) — its issuer is recognisable
///   (Erkennbarkeit des Ausstellers / aussteller_erkennbar).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Urkunde {
    /// Perpetuated declaration of thought (verkörperte Gedankenerklärung): the
    /// thought is fixed in a physical medium.
    pub perpetuierte_gedankenerklaerung: bool,
    /// Evidentiary function (Beweisfunktion): suitable and intended to prove a
    /// legally relevant fact (Beweiseignung und Beweisbestimmung).
    pub beweisfunktion: bool,
    /// Guarantee function (Garantiefunktion): the issuer is recognisable
    /// (Erkennbarkeit des Ausstellers).
    pub garantiefunktion: bool,
}

impl Urkunde {
    /// Whether the object qualifies as a *Urkunde*: all three functions must be
    /// present (perpetuated declaration of thought + Beweisfunktion +
    /// Garantiefunktion).
    #[must_use]
    pub fn is_urkunde(&self) -> bool {
        self.perpetuierte_gedankenerklaerung && self.beweisfunktion && self.garantiefunktion
    }

    /// Whether the issuer of the document is recognisable (Garantiefunktion /
    /// aussteller_erkennbar) — a convenience alias for the guarantee function.
    #[must_use]
    pub fn aussteller_erkennbar(&self) -> bool {
        self.garantiefunktion
    }
}

/// The three alternative acts of § 267 Abs. 1 StGB (Tathandlungen).
///
/// The offence is committed by **any one** of these alternatives, all directed at
/// an *unechte* (forged) or *verfälschte* (falsified) document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tathandlung267 {
    /// Producing a forged document (Herstellen einer unechten Urkunde): creating
    /// a document whose apparent issuer is not its actual issuer.
    HerstellenUnechterUrkunde,
    /// Falsifying a genuine document (Verfälschen einer echten Urkunde): altering
    /// a genuine document so that its content no longer reflects the original
    /// declaration of its issuer.
    VerfaelschenEchterUrkunde,
    /// Using a forged or falsified document (Gebrauchen einer unechten oder
    /// verfälschten Urkunde): making it accessible to the person to be deceived.
    GebrauchenUnechterUrkunde,
}

impl Tathandlung267 {
    /// A short German label for the act alternative.
    #[must_use]
    pub fn bezeichnung(&self) -> &'static str {
        match self {
            Tathandlung267::HerstellenUnechterUrkunde => "Herstellen einer unechten Urkunde",
            Tathandlung267::VerfaelschenEchterUrkunde => "Verfälschen einer echten Urkunde",
            Tathandlung267::GebrauchenUnechterUrkunde => {
                "Gebrauchen einer unechten oder verfälschten Urkunde"
            }
        }
    }
}

/// A forgery-related offence of §§ 267-282 StGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForgeryOffence {
    /// Forgery of documents (Urkundenfälschung, § 267 StGB).
    Urkundenfaelschung {
        /// Which of the three act alternatives is charged.
        handlung: Tathandlung267,
        /// Whether an especially serious case applies (§ 267 Abs. 3 StGB → six
        /// months to ten years).
        besonders_schwer: bool,
        /// Whether commission as a member of a gang acting commercially applies
        /// (§ 267 Abs. 4 StGB → one to ten years). Takes precedence over
        /// [`besonders_schwer`](ForgeryOffence::Urkundenfaelschung::besonders_schwer).
        bande_gewerbsmaessig: bool,
    },
    /// Forgery of technical records (Fälschung technischer Aufzeichnungen,
    /// § 268 StGB).
    FaelschungTechnischerAufzeichnungen,
    /// Forgery of data relevant as evidence (Fälschung beweiserheblicher Daten,
    /// § 269 StGB).
    FaelschungBeweiserheblicherDaten,
    /// Causing false entries in public records (mittelbare Falschbeurkundung,
    /// § 271 StGB).
    MittelbareFalschbeurkundung,
    /// Suppression of documents (Urkundenunterdrückung, § 274 StGB).
    Urkundenunterdrueckung,
    /// Forgery of health certificates (Fälschung von Gesundheitszeugnissen,
    /// § 277 StGB).
    FaelschungGesundheitszeugnisse,
    /// Misuse of identity papers (Missbrauch von Ausweispapieren, § 281 StGB).
    MissbrauchAusweispapiere,
}

impl ForgeryOffence {
    /// The § citation of the offence.
    #[must_use]
    pub fn paragraph(&self) -> &'static str {
        match self {
            ForgeryOffence::Urkundenfaelschung { .. } => "§ 267 StGB",
            ForgeryOffence::FaelschungTechnischerAufzeichnungen => "§ 268 StGB",
            ForgeryOffence::FaelschungBeweiserheblicherDaten => "§ 269 StGB",
            ForgeryOffence::MittelbareFalschbeurkundung => "§ 271 StGB",
            ForgeryOffence::Urkundenunterdrueckung => "§ 274 StGB",
            ForgeryOffence::FaelschungGesundheitszeugnisse => "§ 277 StGB",
            ForgeryOffence::MissbrauchAusweispapiere => "§ 281 StGB",
        }
    }

    /// The statutory sentencing range (Strafrahmen) of the offence.
    #[must_use]
    pub fn strafrahmen(&self) -> Strafrahmen {
        match self {
            // § 267 StGB - up to five years or a fine (Abs. 1); six months to ten
            // years in especially serious cases (Abs. 3); one to ten years for a
            // gang acting commercially (Abs. 4).
            ForgeryOffence::Urkundenfaelschung {
                besonders_schwer,
                bande_gewerbsmaessig,
                ..
            } => {
                if *bande_gewerbsmaessig {
                    Strafrahmen::imprisonment(12, 120)
                } else if *besonders_schwer {
                    Strafrahmen::imprisonment(6, 120)
                } else {
                    Strafrahmen::up_to_months_or_fine(60)
                }
            }
            // § 268 StGB - up to five years or a fine.
            ForgeryOffence::FaelschungTechnischerAufzeichnungen => {
                Strafrahmen::up_to_months_or_fine(60)
            }
            // § 269 StGB - up to five years or a fine.
            ForgeryOffence::FaelschungBeweiserheblicherDaten => {
                Strafrahmen::up_to_months_or_fine(60)
            }
            // § 271 StGB - up to three years or a fine.
            ForgeryOffence::MittelbareFalschbeurkundung => Strafrahmen::up_to_months_or_fine(36),
            // § 274 StGB - up to five years or a fine.
            ForgeryOffence::Urkundenunterdrueckung => Strafrahmen::up_to_months_or_fine(60),
            // § 277 StGB - up to one year or a fine.
            ForgeryOffence::FaelschungGesundheitszeugnisse => Strafrahmen::up_to_months_or_fine(12),
            // § 281 StGB - up to one year or a fine.
            ForgeryOffence::MissbrauchAusweispapiere => Strafrahmen::up_to_months_or_fine(12),
        }
    }

    /// Whether the offence is built on the *Urkunde* concept of § 267 and thus
    /// requires the object to be a genuine/forgeable document.
    ///
    /// § 268 (technical records), § 269 (data) and § 281 (identity papers) have
    /// their own objects and are not validated against [`Urkunde::is_urkunde`].
    #[must_use]
    fn requires_urkunde(&self) -> bool {
        matches!(
            self,
            ForgeryOffence::Urkundenfaelschung { .. }
                | ForgeryOffence::MittelbareFalschbeurkundung
                | ForgeryOffence::Urkundenunterdrueckung
                | ForgeryOffence::FaelschungGesundheitszeugnisse
        )
    }
}

/// A forgery case (Urkundsdelikt), §§ 267-282 StGB.
///
/// Bundles the object of the offence, its document properties, and the objective
/// and subjective elements required across the forgery offences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForgeryCase {
    /// Description of the object of the offence (Tatobjekt), e.g. "gefälschter
    /// Personalausweis". Must not be empty.
    pub tatobjekt_beschreibung: String,
    /// The document properties of the object (its three Urkunde functions).
    pub urkunde: Urkunde,
    /// Whether the document is *unecht* (forged: apparent issuer ≠ actual issuer)
    /// or *verfälscht* (a genuine document was altered). A merely false-content
    /// document (schriftliche Lüge) does not satisfy this and is not punishable
    /// under § 267.
    pub ist_unecht_oder_verfaelscht: bool,
    /// Whether the offender acted intentionally (Vorsatz). Forgery offences are
    /// punishable only when committed intentionally (§ 15 StGB).
    pub vorsatz: bool,
    /// Whether the offender acted for the purpose of deception in legal relations
    /// (zur Täuschung im Rechtsverkehr) — the special subjective element of
    /// §§ 267, 269, 281 StGB.
    pub taeuschungsabsicht_im_rechtsverkehr: bool,
    /// Whether the offender acted with the intent to cause disadvantage to
    /// another (Nachteilszufügungsabsicht) — required by § 274 StGB.
    pub nachteilszufuegungsabsicht: bool,
    /// The offence the case is charged under.
    pub offence: ForgeryOffence,
}

/// Validate a forgery case under §§ 267-282 StGB.
///
/// Common structure: the object must (where the offence builds on § 267) be a
/// *Urkunde*, the document must misrepresent its issuer or have been altered
/// (ist_unecht_oder_verfälscht), the offender must have acted intentionally and —
/// for §§ 267, 269, 281 — for the purpose of deception in legal relations; § 274
/// additionally requires the intent to cause disadvantage to another.
///
/// # Errors
/// - [`StgbError::InvalidField`] if the object description is empty.
/// - [`StgbError::InvalidTatobjekt`] if a § 267-based offence is charged but the
///   object is not a *Urkunde* (e.g. the Garantiefunktion is missing), or if
///   § 271 is charged without a document possessing the Beweisfunktion.
/// - [`StgbError::TatbestandNotFulfilled`] if a § 267 alternative is charged but
///   the document is neither *unecht* nor *verfälscht* (a mere schriftliche
///   Lüge).
/// - [`StgbError::FahrlaessigkeitNichtStrafbar`] if intent (Vorsatz) is missing.
/// - [`StgbError::AbsichtMissing`] if the special subjective element is missing:
///   Täuschungsabsicht im Rechtsverkehr (§§ 267, 269, 281) or
///   Nachteilszufügungsabsicht (§ 274).
pub fn validate_forgery(case: &ForgeryCase) -> Result<()> {
    if case.tatobjekt_beschreibung.trim().is_empty() {
        return Err(StgbError::InvalidField {
            field: "tatobjekt_beschreibung".to_string(),
        });
    }

    // Objective object requirements.
    if case.offence.requires_urkunde() && !case.urkunde.is_urkunde() {
        return Err(StgbError::InvalidTatobjekt {
            detail: "Tatobjekt ist keine Urkunde (es fehlt eine der drei Urkundenfunktionen: \
                     perpetuierte Gedankenerklärung, Beweisfunktion, Garantiefunktion)"
                .to_string(),
        });
    }

    // Intent (Vorsatz) is required for all forgery offences (§ 15 StGB).
    if !case.vorsatz {
        return Err(StgbError::FahrlaessigkeitNichtStrafbar);
    }

    match &case.offence {
        ForgeryOffence::Urkundenfaelschung { .. } => {
            // The document must misrepresent its issuer (unecht) or have been
            // altered (verfälscht); a mere false-content document is not enough.
            if !case.ist_unecht_oder_verfaelscht {
                return Err(StgbError::TatbestandNotFulfilled {
                    element: "unechte oder verfälschte Urkunde (eine bloße schriftliche Lüge \
                              genügt nicht, § 267 StGB)"
                        .to_string(),
                });
            }
            if !case.taeuschungsabsicht_im_rechtsverkehr {
                return Err(StgbError::AbsichtMissing {
                    detail: "Täuschungsabsicht im Rechtsverkehr (§ 267 Abs. 1 StGB)".to_string(),
                });
            }
        }
        ForgeryOffence::FaelschungBeweiserheblicherDaten => {
            // § 269 StGB - data that, if perceived, would form a forged document.
            if !case.taeuschungsabsicht_im_rechtsverkehr {
                return Err(StgbError::AbsichtMissing {
                    detail: "Täuschungsabsicht im Rechtsverkehr (§ 269 StGB)".to_string(),
                });
            }
        }
        ForgeryOffence::MissbrauchAusweispapiere => {
            // § 281 StGB - misuse of identity papers to deceive in legal relations.
            if !case.taeuschungsabsicht_im_rechtsverkehr {
                return Err(StgbError::AbsichtMissing {
                    detail: "Täuschungsabsicht im Rechtsverkehr (§ 281 StGB)".to_string(),
                });
            }
        }
        ForgeryOffence::MittelbareFalschbeurkundung => {
            // § 271 StGB - the object is a public record; its Beweisfunktion is
            // indispensable (already covered by requires_urkunde, but checked
            // explicitly so the diagnostic is precise).
            if !case.urkunde.beweisfunktion {
                return Err(StgbError::InvalidTatobjekt {
                    detail: "§ 271 StGB setzt eine öffentliche Urkunde mit Beweisfunktion voraus"
                        .to_string(),
                });
            }
        }
        ForgeryOffence::Urkundenunterdrueckung => {
            // § 274 StGB - requires the intent to cause disadvantage to another.
            if !case.nachteilszufuegungsabsicht {
                return Err(StgbError::AbsichtMissing {
                    detail: "Nachteilszufügungsabsicht (§ 274 Abs. 1 StGB)".to_string(),
                });
            }
        }
        ForgeryOffence::FaelschungTechnischerAufzeichnungen
        | ForgeryOffence::FaelschungGesundheitszeugnisse => {
            // §§ 268, 277 StGB - intentional forgery suffices for this model; the
            // specific objects are described in the case text.
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A document satisfying all three Urkunde functions.
    fn echte_urkunde() -> Urkunde {
        Urkunde {
            perpetuierte_gedankenerklaerung: true,
            beweisfunktion: true,
            garantiefunktion: true,
        }
    }

    /// A baseline § 267 forgery case (Herstellen einer unechten Urkunde) that is
    /// fully made out.
    fn base_case(offence: ForgeryOffence) -> ForgeryCase {
        ForgeryCase {
            tatobjekt_beschreibung: "gefälschtes Dokument".to_string(),
            urkunde: echte_urkunde(),
            ist_unecht_oder_verfaelscht: true,
            vorsatz: true,
            taeuschungsabsicht_im_rechtsverkehr: true,
            nachteilszufuegungsabsicht: false,
            offence,
        }
    }

    fn urkundenfaelschung(handlung: Tathandlung267) -> ForgeryOffence {
        ForgeryOffence::Urkundenfaelschung {
            handlung,
            besonders_schwer: false,
            bande_gewerbsmaessig: false,
        }
    }

    #[test]
    fn is_urkunde_requires_all_three_functions() {
        assert!(echte_urkunde().is_urkunde());
        let mut u = echte_urkunde();
        u.garantiefunktion = false;
        assert!(!u.is_urkunde());
        assert!(!u.aussteller_erkennbar());
    }

    #[test]
    fn valid_herstellen_unechter_urkunde() {
        let o = urkundenfaelschung(Tathandlung267::HerstellenUnechterUrkunde);
        assert_eq!(o.paragraph(), "§ 267 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.max_months, Some(60));
        assert!(r.fine_alternative);
        assert!(!r.allows_life());
        assert!(validate_forgery(&base_case(o)).is_ok());
    }

    #[test]
    fn object_not_an_urkunde_missing_garantiefunktion() {
        let mut c = base_case(urkundenfaelschung(
            Tathandlung267::HerstellenUnechterUrkunde,
        ));
        c.urkunde.garantiefunktion = false;
        assert!(matches!(
            validate_forgery(&c),
            Err(StgbError::InvalidTatobjekt { .. })
        ));
    }

    #[test]
    fn document_not_unecht_or_verfaelscht_is_no_offence() {
        // A genuine document with merely untrue content (schriftliche Lüge).
        let mut c = base_case(urkundenfaelschung(
            Tathandlung267::HerstellenUnechterUrkunde,
        ));
        c.ist_unecht_oder_verfaelscht = false;
        assert!(matches!(
            validate_forgery(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }

    #[test]
    fn missing_taeuschungsabsicht_im_rechtsverkehr() {
        let mut c = base_case(urkundenfaelschung(
            Tathandlung267::HerstellenUnechterUrkunde,
        ));
        c.taeuschungsabsicht_im_rechtsverkehr = false;
        assert!(matches!(
            validate_forgery(&c),
            Err(StgbError::AbsichtMissing { .. })
        ));
    }

    #[test]
    fn missing_vorsatz_is_not_punishable() {
        let mut c = base_case(urkundenfaelschung(
            Tathandlung267::HerstellenUnechterUrkunde,
        ));
        c.vorsatz = false;
        assert!(matches!(
            validate_forgery(&c),
            Err(StgbError::FahrlaessigkeitNichtStrafbar)
        ));
    }

    #[test]
    fn empty_description_is_invalid_field() {
        let mut c = base_case(urkundenfaelschung(
            Tathandlung267::HerstellenUnechterUrkunde,
        ));
        c.tatobjekt_beschreibung = "   ".to_string();
        assert!(matches!(
            validate_forgery(&c),
            Err(StgbError::InvalidField { .. })
        ));
    }

    #[test]
    fn alle_drei_tathandlungen_sind_strafbar() {
        for handlung in [
            Tathandlung267::HerstellenUnechterUrkunde,
            Tathandlung267::VerfaelschenEchterUrkunde,
            Tathandlung267::GebrauchenUnechterUrkunde,
        ] {
            let o = urkundenfaelschung(handlung);
            assert_eq!(o.strafrahmen().max_months, Some(60));
            assert!(!handlung.bezeichnung().is_empty());
            assert!(validate_forgery(&base_case(o)).is_ok());
        }
    }

    #[test]
    fn besonders_schwerer_fall_abs3_range() {
        let o = ForgeryOffence::Urkundenfaelschung {
            handlung: Tathandlung267::HerstellenUnechterUrkunde,
            besonders_schwer: true,
            bande_gewerbsmaessig: false,
        };
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 6);
        assert_eq!(r.max_months, Some(120));
        assert!(!r.fine_alternative);
    }

    #[test]
    fn bande_gewerbsmaessig_abs4_range() {
        let o = ForgeryOffence::Urkundenfaelschung {
            handlung: Tathandlung267::HerstellenUnechterUrkunde,
            besonders_schwer: false,
            bande_gewerbsmaessig: true,
        };
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 12);
        assert_eq!(r.max_months, Some(120));
        assert!(!r.fine_alternative);
    }

    #[test]
    fn bande_gewerbsmaessig_takes_precedence_over_abs3() {
        // Both flags set: § 267 Abs. 4 (one to ten years) prevails over Abs. 3.
        let o = ForgeryOffence::Urkundenfaelschung {
            handlung: Tathandlung267::HerstellenUnechterUrkunde,
            besonders_schwer: true,
            bande_gewerbsmaessig: true,
        };
        assert_eq!(o.strafrahmen().effective_min_months(), 12);
    }

    #[test]
    fn faelschung_technischer_aufzeichnungen_268() {
        let o = ForgeryOffence::FaelschungTechnischerAufzeichnungen;
        assert_eq!(o.paragraph(), "§ 268 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.max_months, Some(60));
        assert!(r.fine_alternative);
        assert!(validate_forgery(&base_case(o)).is_ok());
    }

    #[test]
    fn faelschung_beweiserheblicher_daten_269() {
        let o = ForgeryOffence::FaelschungBeweiserheblicherDaten;
        assert_eq!(o.paragraph(), "§ 269 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.max_months, Some(60));
        assert!(r.fine_alternative);
        // § 269 also requires Täuschungsabsicht im Rechtsverkehr.
        let mut c = base_case(o);
        assert!(validate_forgery(&c).is_ok());
        c.taeuschungsabsicht_im_rechtsverkehr = false;
        assert!(matches!(
            validate_forgery(&c),
            Err(StgbError::AbsichtMissing { .. })
        ));
    }

    #[test]
    fn mittelbare_falschbeurkundung_271() {
        let o = ForgeryOffence::MittelbareFalschbeurkundung;
        assert_eq!(o.paragraph(), "§ 271 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.max_months, Some(36));
        assert!(r.fine_alternative);
        assert!(validate_forgery(&base_case(o)).is_ok());
        // Object without Beweisfunktion fails (also fails the is_urkunde check).
        let mut c = base_case(o);
        c.urkunde.beweisfunktion = false;
        assert!(matches!(
            validate_forgery(&c),
            Err(StgbError::InvalidTatobjekt { .. })
        ));
    }

    #[test]
    fn urkundenunterdrueckung_274_needs_nachteilsabsicht() {
        let o = ForgeryOffence::Urkundenunterdrueckung;
        assert_eq!(o.paragraph(), "§ 274 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.max_months, Some(60));
        assert!(r.fine_alternative);
        // Without Nachteilszufügungsabsicht the offence is not made out.
        let mut c = base_case(o);
        assert!(matches!(
            validate_forgery(&c),
            Err(StgbError::AbsichtMissing { .. })
        ));
        c.nachteilszufuegungsabsicht = true;
        assert!(validate_forgery(&c).is_ok());
    }

    #[test]
    fn gesundheitszeugnisse_277_one_year() {
        let o = ForgeryOffence::FaelschungGesundheitszeugnisse;
        assert_eq!(o.paragraph(), "§ 277 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.max_months, Some(12));
        assert!(r.fine_alternative);
        assert!(validate_forgery(&base_case(o)).is_ok());
    }

    #[test]
    fn missbrauch_ausweispapiere_281_one_year() {
        let o = ForgeryOffence::MissbrauchAusweispapiere;
        assert_eq!(o.paragraph(), "§ 281 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.max_months, Some(12));
        assert!(r.fine_alternative);
        // § 281 requires Täuschungsabsicht im Rechtsverkehr.
        let mut c = base_case(o);
        assert!(validate_forgery(&c).is_ok());
        c.taeuschungsabsicht_im_rechtsverkehr = false;
        assert!(matches!(
            validate_forgery(&c),
            Err(StgbError::AbsichtMissing { .. })
        ));
    }
}
