//! StGB §§ 249-255 - Robbery and Extortion (Raub und Erpressung)
//!
//! This module models the offences of the 20th section of the Special Part of the
//! German Criminal Code (Zwanzigster Abschnitt - Raub und Erpressung), i.e. the
//! combination of a property taking/disposition with a qualified coercive means
//! (Gewalt oder Drohung). Each offence is modelled with its objective and
//! subjective elements (Tatbestandsmerkmale), a validator, and the statutory
//! sentencing range (Strafrahmen) drawn from [`crate::stgb::strafe`].
//!
//! ## § 249 StGB - Raub (Robbery)
//!
//! > (1) Wer mit Gewalt gegen eine Person oder unter Anwendung von Drohungen mit
//! > gegenwärtiger Gefahr für Leib oder Leben eine fremde bewegliche Sache einem
//! > anderen in der Absicht wegnimmt, die Sache sich oder einem Dritten
//! > rechtswidrig zuzueignen, wird mit Freiheitsstrafe nicht unter einem Jahr
//! > bestraft.
//! > (2) In minder schweren Fällen ist die Strafe Freiheitsstrafe von sechs
//! > Monaten bis zu fünf Jahren.
//!
//! **English**: Robbery is the **taking** (Wegnahme) of **another's movable
//! property** (fremde bewegliche Sache) by means of **force against a person**
//! (Gewalt gegen eine Person) **or** a **threat of present danger to life or
//! limb** (Drohung mit gegenwärtiger Gefahr für Leib oder Leben), with the
//! **intent of unlawful appropriation** (Zueignungsabsicht). The coercive means
//! must serve to **enable the taking** (Finalzusammenhang / Zweck-Mittel-
//! Relation). Punishment: imprisonment of **not less than one year**; in less
//! serious cases (Abs. 2) **six months to five years**.
//!
//! ## § 250 StGB - Schwerer Raub (Aggravated robbery)
//!
//! - **Abs. 1** (e.g. carrying a weapon or dangerous tool, gang robbery):
//!   imprisonment of **not less than three years**.
//! - **Abs. 2** (e.g. *using* a weapon/dangerous tool, serious physical
//!   maltreatment, danger of death by the act): imprisonment of **not less than
//!   five years**.
//! - **Abs. 3** (minder schwerer Fall): **one to ten years**.
//!
//! ## § 251 StGB - Raub mit Todesfolge (Robbery causing death)
//!
//! > Verursacht der Täter durch den Raub (§§ 249 und 250) wenigstens
//! > leichtfertig den Tod eines anderen Menschen, so ist die Strafe lebenslange
//! > Freiheitsstrafe oder Freiheitsstrafe nicht unter zehn Jahren.
//!
//! **English**: A result-aggravated offence (erfolgsqualifiziertes Delikt): where
//! the robbery causes another person's death at least **recklessly**
//! (leichtfertig), the punishment is **life imprisonment OR imprisonment of not
//! less than ten years**.
//!
//! ## § 252 StGB - Räuberischer Diebstahl (Robbery-like theft)
//!
//! > Wer bei einem Diebstahl auf frischer Tat betroffen, gegen eine Person Gewalt
//! > verübt oder Drohungen mit gegenwärtiger Gefahr für Leib oder Leben anwendet,
//! > um sich im Besitz des gestohlenen Gutes zu erhalten, wird gleich einem
//! > Räuber bestraft.
//!
//! **English**: Whoever, **caught in the act** of a theft (auf frischer Tat
//! betroffen), uses force or a threat of present danger to life or limb against a
//! person **in order to keep possession** of the stolen goods
//! (Beutesicherungsabsicht), is punished **like a robber** (→ § 249).
//!
//! ## § 253 StGB - Erpressung (Extortion)
//!
//! > (1) Wer einen Menschen rechtswidrig mit Gewalt oder durch Drohung mit einem
//! > empfindlichen Übel zu einer Handlung, Duldung oder Unterlassung nötigt und
//! > dadurch dem Vermögen des Genötigten oder eines anderen Nachteil zufügt, um
//! > sich oder einen Dritten zu Unrecht zu bereichern, wird mit Freiheitsstrafe
//! > bis zu fünf Jahren oder mit Geldstrafe bestraft.
//!
//! **English**: Extortion is **coercion** (Nötigung) of a person by **force** or
//! by **threat of a serious harm** (empfindliches Übel) to a conduct, sufferance
//! or omission causing a **detriment to property** (Vermögensnachteil) through a
//! **property disposition** (Vermögensverfügung), with the **intent of unlawful
//! enrichment** (Bereicherungsabsicht). The act must be **unlawful** (rechtswidrig
//! / verwerflich, Abs. 2). Punishment: imprisonment of **up to five years or a
//! fine**.
//!
//! ## § 255 StGB - Räuberische Erpressung (Robbery-like extortion)
//!
//! > Wird die Erpressung durch Gewalt gegen eine Person oder unter Anwendung von
//! > Drohungen mit gegenwärtiger Gefahr für Leib oder Leben begangen, so ist der
//! > Täter gleich einem Räuber zu bestrafen.
//!
//! **English**: Where the extortion is committed with **force against a person**
//! or a **threat of present danger to life or limb**, the offender is punished
//! **like a robber** (→ § 249, i.e. not less than one year).

use serde::{Deserialize, Serialize};

use crate::stgb::error::{Result, StgbError};
use crate::stgb::strafe::Strafrahmen;

/// A qualified coercive means (Nötigungsmittel) used in §§ 249-255 StGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Noetigungsmittel {
    /// Force against a person (Gewalt gegen eine Person), §§ 249, 252, 255 StGB.
    GewaltGegenPerson,
    /// Threat of present danger to life or limb (Drohung mit gegenwärtiger Gefahr
    /// für Leib oder Leben), §§ 249, 252, 255 StGB.
    DrohungLeibLeben,
    /// Threat of a serious harm (Drohung mit einem empfindlichen Übel), the
    /// lesser coercive means sufficient for plain extortion under § 253 StGB only.
    DrohungEmpfindlichesUebel,
}

impl Noetigungsmittel {
    /// Whether the coercive means is one of the *qualified* robbery means
    /// (Gewalt gegen eine Person oder Drohung mit gegenwärtiger Gefahr für Leib
    /// oder Leben) required by §§ 249, 250, 252, 255 StGB.
    ///
    /// The mere threat of a serious harm (empfindliches Übel) is **not**
    /// sufficient there; it only carries plain extortion under § 253 StGB.
    #[must_use]
    pub fn ist_raubqualifiziert(&self) -> bool {
        matches!(
            self,
            Noetigungsmittel::GewaltGegenPerson | Noetigungsmittel::DrohungLeibLeben
        )
    }
}

/// An aggravating circumstance of aggravated robbery (schwerer Raub),
/// § 250 StGB. The `Abs. 1` variants raise the floor to three years; the
/// `Abs. 2` variants raise it to five years.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchwererRaubMerkmal {
    /// § 250 Abs. 1 Nr. 1 lit. a: the offender or a participant *carries* a
    /// weapon or other dangerous tool (Waffe oder gefährliches Werkzeug bei sich
    /// geführt).
    WaffeBeisichGefuehrt250_1,
    /// § 250 Abs. 1 Nr. 1 lit. b: an other tool/means is carried to overcome
    /// resistance by force or threat of force (sonstiges Werkzeug/Mittel).
    SonstigesMittelBeisichGefuehrt250_1,
    /// § 250 Abs. 1 Nr. 2: gang robbery - commission as a member of a gang
    /// (Bandenraub).
    Bandenraub,
    /// § 250 Abs. 1 Nr. 1 lit. c: by the act another person is placed in danger
    /// of serious damage to health (Gefahr einer schweren Gesundheitsschädigung).
    GefahrSchwererGesundheitsschaedigung250_1,
    /// § 250 Abs. 2 Nr. 1: the offender or a participant *uses* a weapon or other
    /// dangerous tool during the act (Waffe oder gefährliches Werkzeug verwendet).
    WaffeVerwendet250_2,
    /// § 250 Abs. 2 Nr. 2: gang robbery while carrying a weapon (bewaffneter
    /// Bandenraub).
    BewaffneterBandenraub250_2,
    /// § 250 Abs. 2 Nr. 3 lit. a: another person is seriously physically
    /// maltreated by the act (schwere körperliche Misshandlung).
    SchwereKoerperlicheMisshandlung250_2,
    /// § 250 Abs. 2 Nr. 3 lit. b: another person is placed in danger of death by
    /// the act (Todesgefahr durch die Tat).
    TodesgefahrDurchTat250_2,
}

impl SchwererRaubMerkmal {
    /// Whether the circumstance belongs to the more serious paragraph
    /// (§ 250 Abs. 2 StGB), which raises the minimum to five years.
    #[must_use]
    pub fn ist_absatz_2(&self) -> bool {
        matches!(
            self,
            SchwererRaubMerkmal::WaffeVerwendet250_2
                | SchwererRaubMerkmal::BewaffneterBandenraub250_2
                | SchwererRaubMerkmal::SchwereKoerperlicheMisshandlung250_2
                | SchwererRaubMerkmal::TodesgefahrDurchTat250_2
        )
    }
}

/// The specific robbery/extortion offence applicable to a case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RobberyOffence {
    /// Robbery (Raub, § 249 StGB).
    Raub {
        /// Whether a less serious case applies (§ 249 Abs. 2 → 6 months to 5
        /// years).
        minder_schwer: bool,
    },
    /// Aggravated robbery (schwerer Raub, § 250 StGB).
    SchwererRaub {
        /// The aggravating circumstances present (at least one).
        merkmale: Vec<SchwererRaubMerkmal>,
        /// Whether the more serious paragraph (§ 250 Abs. 2) applies, i.e. an
        /// Abs. 2 circumstance is present.
        abs2: bool,
        /// Whether a less serious case applies (§ 250 Abs. 3 → 1 to 10 years).
        minder_schwer: bool,
    },
    /// Robbery causing death (Raub mit Todesfolge, § 251 StGB).
    RaubMitTodesfolge,
    /// Robbery-like theft (räuberischer Diebstahl, § 252 StGB).
    RaeuberischerDiebstahl,
    /// Extortion (Erpressung, § 253 StGB).
    Erpressung {
        /// Whether an especially serious case applies (§ 253 Abs. 4 → 1 to 15
        /// years).
        besonders_schwer: bool,
    },
    /// Robbery-like extortion (räuberische Erpressung, § 255 StGB).
    RaeuberischeErpressung,
}

impl RobberyOffence {
    /// The § citation of the offence.
    #[must_use]
    pub fn paragraph(&self) -> &'static str {
        match self {
            RobberyOffence::Raub { .. } => "§ 249 StGB",
            RobberyOffence::SchwererRaub { .. } => "§ 250 StGB",
            RobberyOffence::RaubMitTodesfolge => "§ 251 StGB",
            RobberyOffence::RaeuberischerDiebstahl => "§ 252 StGB",
            RobberyOffence::Erpressung { .. } => "§ 253 StGB",
            RobberyOffence::RaeuberischeErpressung => "§ 255 StGB",
        }
    }

    /// The statutory sentencing range (Strafrahmen) of the offence.
    #[must_use]
    pub fn strafrahmen(&self) -> Strafrahmen {
        match self {
            // § 249 StGB - not less than one year (12 months); minder schwerer
            // Fall (Abs. 2): six months to five years.
            RobberyOffence::Raub { minder_schwer } => {
                if *minder_schwer {
                    Strafrahmen::imprisonment(6, 60)
                } else {
                    Strafrahmen::at_least_months(12)
                }
            }
            // § 250 StGB - Abs. 1: not less than three years; Abs. 2: not less
            // than five years; minder schwerer Fall (Abs. 3): one to ten years.
            RobberyOffence::SchwererRaub {
                abs2,
                minder_schwer,
                ..
            } => {
                if *minder_schwer {
                    Strafrahmen::imprisonment(12, 120)
                } else if *abs2 {
                    Strafrahmen::at_least_months(60)
                } else {
                    Strafrahmen::at_least_months(36)
                }
            }
            // § 251 StGB - life imprisonment OR not less than ten years.
            //
            // NOTE: this range admits BOTH life imprisonment and a time-limited
            // sentence with a floor of ten years (120 months). [`Strafrahmen::
            // at_least_months`] cannot express this because it pins
            // `max_months` to 180 (and therefore forbids life). The struct is
            // therefore constructed literally: `max_months: None` permits life,
            // while `min_months: Some(120)` sets the ten-year floor for any
            // time-limited sentence.
            RobberyOffence::RaubMitTodesfolge => Strafrahmen {
                min_months: Some(120),
                max_months: None,
                fine_alternative: false,
            },
            // § 252 StGB - punished "like a robber" (gleich einem Räuber), i.e.
            // the § 249 Abs. 1 range: not less than one year.
            RobberyOffence::RaeuberischerDiebstahl => Strafrahmen::at_least_months(12),
            // § 253 StGB - up to five years or a fine; especially serious case
            // (Abs. 4): one to fifteen years.
            RobberyOffence::Erpressung { besonders_schwer } => {
                if *besonders_schwer {
                    Strafrahmen::imprisonment(12, 180)
                } else {
                    Strafrahmen::up_to_months_or_fine(60)
                }
            }
            // § 255 StGB - punished "like a robber" (gleich einem Räuber), i.e.
            // the § 249 Abs. 1 range: not less than one year.
            RobberyOffence::RaeuberischeErpressung => Strafrahmen::at_least_months(12),
        }
    }

    /// Whether this offence is a taking offence built on the theft model
    /// (§§ 249-252: fremde bewegliche Sache + Wegnahme + Zueignungsabsicht), as
    /// opposed to an extortion offence built on the disposition model
    /// (§§ 253, 255: Vermögensverfügung + Vermögensnachteil + Bereicherungs-
    /// absicht).
    #[must_use]
    pub fn ist_wegnahmedelikt(&self) -> bool {
        matches!(
            self,
            RobberyOffence::Raub { .. }
                | RobberyOffence::SchwererRaub { .. }
                | RobberyOffence::RaubMitTodesfolge
                | RobberyOffence::RaeuberischerDiebstahl
        )
    }
}

/// A robbery/extortion case (Raub- oder Erpressungsdelikt), §§ 249-255 StGB.
///
/// The taking offences (§§ 249-252) rely on the theft-model fields
/// ([`Self::fremde_bewegliche_sache`], [`Self::wegnahme_vollzogen`],
/// [`Self::zueignungs_oder_bereicherungsabsicht`]); the extortion offences
/// (§§ 253, 255) rely on the disposition-model fields
/// ([`Self::vermoegensverfuegung`], [`Self::vermoegensnachteil`],
/// [`Self::zueignungs_oder_bereicherungsabsicht`]). Each [`validate_robbery`]
/// branch checks only the fields relevant to the charged offence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RobberyCase {
    /// Description of the object of the offence (Tatobjekt): the movable thing
    /// (§§ 249-252) or the property/asset affected (§§ 253, 255).
    pub tatobjekt: String,
    /// Whether the object is another's movable property (fremde bewegliche
    /// Sache), required for the taking offences §§ 249-252 StGB.
    pub fremde_bewegliche_sache: bool,
    /// Whether the taking was carried out (Wegnahme vollzogen = breaking of
    /// existing custody and establishing new custody), §§ 249-252 StGB.
    pub wegnahme_vollzogen: bool,
    /// Whether a property disposition occurred (Vermögensverfügung), required for
    /// the extortion offences §§ 253, 255 StGB. (For the prevailing case law
    /// § 255 does not require a self-damaging disposition, but the disposition is
    /// modelled here to keep the element explicit.)
    pub vermoegensverfuegung: bool,
    /// Whether a detriment to property resulted (Vermögensnachteil), required for
    /// §§ 253, 255 StGB.
    pub vermoegensnachteil: bool,
    /// The qualified coercive means employed (Nötigungsmittel), if any.
    pub noetigungsmittel: Option<Noetigungsmittel>,
    /// Whether the coercive means served to enable the taking/disposition
    /// (Finalzusammenhang / Zweck-Mittel-Relation), required for §§ 249-251 and,
    /// in the form of the means→disposition link, for §§ 253, 255 StGB.
    pub finalzusammenhang: bool,
    /// Whether the offender acted intentionally (Vorsatz). Required for every
    /// offence of this section; none of §§ 249-255 StGB punishes negligence.
    pub vorsatz: bool,
    /// Whether the offender acted with the intent of unlawful appropriation
    /// (Zueignungsabsicht, §§ 249-252) or of unlawful enrichment
    /// (Bereicherungsabsicht, §§ 253, 255).
    pub zueignungs_oder_bereicherungsabsicht: bool,
    /// § 252 StGB: whether the offender was caught in the act of the theft (auf
    /// frischer Tat betroffen).
    pub auf_frischer_tat_betroffen: bool,
    /// § 252 StGB: whether the offender acted with the intent of keeping
    /// possession of the loot (Beutesicherungs-/Besitzerhaltungsabsicht).
    pub besitzerhaltungsabsicht: bool,
    /// The offence the case is charged under.
    pub offence: RobberyOffence,
}

impl RobberyCase {
    /// Construct a case with all flags cleared and the empty Tatobjekt, ready to
    /// be populated for the charged `offence`.
    #[must_use]
    pub fn new(offence: RobberyOffence) -> Self {
        Self {
            tatobjekt: String::new(),
            fremde_bewegliche_sache: false,
            wegnahme_vollzogen: false,
            vermoegensverfuegung: false,
            vermoegensnachteil: false,
            noetigungsmittel: None,
            finalzusammenhang: false,
            vorsatz: false,
            zueignungs_oder_bereicherungsabsicht: false,
            auf_frischer_tat_betroffen: false,
            besitzerhaltungsabsicht: false,
            offence,
        }
    }
}

/// Ensure the offender acted intentionally; none of §§ 249-255 StGB punishes
/// negligence (§ 15 StGB).
fn require_vorsatz(case: &RobberyCase) -> Result<()> {
    if case.vorsatz {
        Ok(())
    } else {
        Err(StgbError::FahrlaessigkeitNichtStrafbar)
    }
}

/// Ensure a qualified robbery coercive means (Gewalt gegen eine Person oder
/// Drohung mit gegenwärtiger Gefahr für Leib oder Leben) was employed, as
/// required by §§ 249, 250, 252, 255 StGB.
fn require_raubmittel(case: &RobberyCase, paragraph: &str) -> Result<()> {
    match case.noetigungsmittel {
        Some(mittel) if mittel.ist_raubqualifiziert() => Ok(()),
        _ => Err(StgbError::TatbestandNotFulfilled {
            element: format!(
                "qualifiziertes Nötigungsmittel (Gewalt gegen eine Person oder Drohung mit \
                 gegenwärtiger Gefahr für Leib oder Leben) ({paragraph})"
            ),
        }),
    }
}

/// Validate the theft-model elements common to §§ 249-252 StGB: another's
/// movable property, a completed taking, and the intent of unlawful
/// appropriation.
fn validate_wegnahme_elemente(case: &RobberyCase, paragraph: &str) -> Result<()> {
    if case.tatobjekt.trim().is_empty() || !case.fremde_bewegliche_sache {
        return Err(StgbError::InvalidTatobjekt {
            detail: format!("Tatobjekt muss eine fremde bewegliche Sache sein ({paragraph})"),
        });
    }
    if !case.wegnahme_vollzogen {
        return Err(StgbError::TatbestandNotFulfilled {
            element: format!("Wegnahme (Bruch und Begründung neuen Gewahrsams) ({paragraph})"),
        });
    }
    if !case.zueignungs_oder_bereicherungsabsicht {
        return Err(StgbError::AbsichtMissing {
            detail: format!("rechtswidrige Zueignungsabsicht ({paragraph})"),
        });
    }
    Ok(())
}

/// Validate the disposition-model elements common to §§ 253, 255 StGB: a
/// property disposition, a resulting detriment to property, and the intent of
/// unlawful enrichment.
fn validate_erpressung_elemente(case: &RobberyCase, paragraph: &str) -> Result<()> {
    if case.tatobjekt.trim().is_empty() {
        return Err(StgbError::InvalidTatobjekt {
            detail: format!("betroffenes Vermögen (Tatobjekt) fehlt ({paragraph})"),
        });
    }
    if !case.vermoegensverfuegung {
        return Err(StgbError::TatbestandNotFulfilled {
            element: format!("Vermögensverfügung des Genötigten ({paragraph})"),
        });
    }
    if !case.vermoegensnachteil {
        return Err(StgbError::TatbestandNotFulfilled {
            element: format!("Vermögensnachteil ({paragraph})"),
        });
    }
    if !case.zueignungs_oder_bereicherungsabsicht {
        return Err(StgbError::AbsichtMissing {
            detail: format!("Absicht rechtswidriger Bereicherung ({paragraph})"),
        });
    }
    Ok(())
}

/// Validate a robbery/extortion case under §§ 249-255 StGB.
///
/// The check is dispatched on [`RobberyCase::offence`]:
/// - **§ 249 / § 250 / § 251**: another's movable property, a completed taking,
///   a qualified coercive means, the final-connection (Finalzusammenhang) between
///   means and taking, intent and the intent of unlawful appropriation. § 250
///   additionally requires at least one aggravating circumstance (and that the
///   `abs2` flag matches an Abs. 2 circumstance). § 251 additionally requires
///   that the death of another was caused (modelled via [`RobberyCase::
///   vermoegensnachteil`] being irrelevant - the death element is the taking plus
///   a fatal result, here represented by requiring the robbery elements).
/// - **§ 252**: the offender caught in the act of a theft (auf frischer Tat
///   betroffen) employing a qualified coercive means with the intent of keeping
///   possession of the loot (Beutesicherungsabsicht).
/// - **§ 253 / § 255**: a property disposition causing a detriment to property
///   with the intent of unlawful enrichment. § 253 admits the lesser means
///   (Drohung mit einem empfindlichen Übel); § 255 requires the qualified
///   robbery means.
///
/// # Errors
/// - [`StgbError::InvalidTatobjekt`] if the object is missing or unsuitable.
/// - [`StgbError::TatbestandNotFulfilled`] if an objective element is missing
///   (taking, coercive means, final-connection, disposition, detriment, frische
///   Tat, an aggravating circumstance).
/// - [`StgbError::AbsichtMissing`] if the required intent (Zueignungs- bzw.
///   Bereicherungsabsicht, Beutesicherungsabsicht) is missing.
/// - [`StgbError::FahrlaessigkeitNichtStrafbar`] if intent (Vorsatz) is missing.
/// - [`StgbError::InvalidField`] if the offence's discriminating flags are
///   internally inconsistent (e.g. § 250 Abs. 2 charged without an Abs. 2
///   circumstance).
pub fn validate_robbery(case: &RobberyCase) -> Result<()> {
    require_vorsatz(case)?;

    match &case.offence {
        RobberyOffence::Raub { .. } => {
            validate_wegnahme_elemente(case, "§ 249 StGB")?;
            require_raubmittel(case, "§ 249 StGB")?;
            if !case.finalzusammenhang {
                return Err(StgbError::TatbestandNotFulfilled {
                    element: "Finalzusammenhang zwischen Nötigungsmittel und Wegnahme \
                              (Zweck-Mittel-Relation) (§ 249 StGB)"
                        .to_string(),
                });
            }
        }
        RobberyOffence::SchwererRaub {
            merkmale,
            abs2,
            minder_schwer: _,
        } => {
            validate_wegnahme_elemente(case, "§ 250 StGB")?;
            require_raubmittel(case, "§ 250 StGB")?;
            if !case.finalzusammenhang {
                return Err(StgbError::TatbestandNotFulfilled {
                    element: "Finalzusammenhang zwischen Nötigungsmittel und Wegnahme \
                              (Zweck-Mittel-Relation) (§ 250 StGB)"
                        .to_string(),
                });
            }
            if merkmale.is_empty() {
                return Err(StgbError::TatbestandNotFulfilled {
                    element: "Qualifikationsmerkmal des schweren Raubes (§ 250 StGB)".to_string(),
                });
            }
            // The `abs2` flag must be consistent with the circumstances present:
            // it is set iff at least one Abs. 2 circumstance was found.
            let hat_abs2_merkmal = merkmale.iter().any(SchwererRaubMerkmal::ist_absatz_2);
            if *abs2 != hat_abs2_merkmal {
                return Err(StgbError::InvalidField {
                    field: "schwerer Raub: abs2-Kennzeichen passt nicht zu den \
                            Qualifikationsmerkmalen (§ 250 Abs. 1/2 StGB)"
                        .to_string(),
                });
            }
        }
        RobberyOffence::RaubMitTodesfolge => {
            // The base robbery (§§ 249/250) must be fulfilled ...
            validate_wegnahme_elemente(case, "§ 251 StGB")?;
            require_raubmittel(case, "§ 251 StGB")?;
            if !case.finalzusammenhang {
                return Err(StgbError::TatbestandNotFulfilled {
                    element: "Finalzusammenhang zwischen Nötigungsmittel und Wegnahme \
                              (Zweck-Mittel-Relation) (§ 251 StGB)"
                        .to_string(),
                });
            }
            // ... and the qualifying fatal result must have been caused at least
            // recklessly (wenigstens leichtfertig den Tod verursacht). The fatal
            // result is modelled by the [`RobberyCase::vermoegensnachteil`] field
            // being repurposed would be misleading; instead the death element is
            // represented by the dedicated condition below.
            if !case.vermoegensnachteil {
                // `vermoegensnachteil` is reused here as the "fatal result
                // caused" flag for the result-aggravated offence; see the field
                // docs. Keeping a single struct avoids a parallel hierarchy.
                return Err(StgbError::TatbestandNotFulfilled {
                    element: "wenigstens leichtfertig verursachter Tod eines anderen Menschen \
                              (Todeserfolg) (§ 251 StGB)"
                        .to_string(),
                });
            }
        }
        RobberyOffence::RaeuberischerDiebstahl => {
            // The predicate theft object must be another's movable property and
            // the offender must intend to keep possession (Beutesicherung).
            if case.tatobjekt.trim().is_empty() || !case.fremde_bewegliche_sache {
                return Err(StgbError::InvalidTatobjekt {
                    detail: "Tatobjekt des Vortaten-Diebstahls muss eine fremde bewegliche Sache \
                             sein (§ 252 StGB)"
                        .to_string(),
                });
            }
            if !case.auf_frischer_tat_betroffen {
                return Err(StgbError::TatbestandNotFulfilled {
                    element: "auf frischer Tat betroffen (§ 252 StGB)".to_string(),
                });
            }
            require_raubmittel(case, "§ 252 StGB")?;
            if !case.besitzerhaltungsabsicht {
                return Err(StgbError::AbsichtMissing {
                    detail: "Beutesicherungsabsicht (sich im Besitz des gestohlenen Gutes zu \
                             erhalten) (§ 252 StGB)"
                        .to_string(),
                });
            }
        }
        RobberyOffence::Erpressung { .. } => {
            validate_erpressung_elemente(case, "§ 253 StGB")?;
            // Any coercive means (force, threat of present danger, or threat of a
            // serious harm) suffices for plain extortion. The act must, however,
            // be carried out by *some* coercive means.
            if case.noetigungsmittel.is_none() {
                return Err(StgbError::TatbestandNotFulfilled {
                    element: "Nötigungsmittel (Gewalt oder Drohung mit einem empfindlichen Übel) \
                              (§ 253 StGB)"
                        .to_string(),
                });
            }
            if !case.finalzusammenhang {
                return Err(StgbError::TatbestandNotFulfilled {
                    element: "Zusammenhang zwischen Nötigung und Vermögensverfügung (§ 253 StGB)"
                        .to_string(),
                });
            }
        }
        RobberyOffence::RaeuberischeErpressung => {
            validate_erpressung_elemente(case, "§ 255 StGB")?;
            // § 255 requires the *qualified* robbery means.
            require_raubmittel(case, "§ 255 StGB")?;
            if !case.finalzusammenhang {
                return Err(StgbError::TatbestandNotFulfilled {
                    element: "Zusammenhang zwischen qualifizierter Nötigung und \
                              Vermögensverfügung (§ 255 StGB)"
                        .to_string(),
                });
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fully-fulfilled taking-model case (§§ 249-252) for the given offence.
    fn wegnahme_case(offence: RobberyOffence) -> RobberyCase {
        RobberyCase {
            tatobjekt: "Geldbörse".to_string(),
            fremde_bewegliche_sache: true,
            wegnahme_vollzogen: true,
            vermoegensverfuegung: false,
            vermoegensnachteil: false,
            noetigungsmittel: Some(Noetigungsmittel::GewaltGegenPerson),
            finalzusammenhang: true,
            vorsatz: true,
            zueignungs_oder_bereicherungsabsicht: true,
            auf_frischer_tat_betroffen: false,
            besitzerhaltungsabsicht: false,
            offence,
        }
    }

    /// A fully-fulfilled disposition-model case (§§ 253, 255) for the offence.
    fn erpressung_case(offence: RobberyOffence) -> RobberyCase {
        RobberyCase {
            tatobjekt: "Bargeld".to_string(),
            fremde_bewegliche_sache: false,
            wegnahme_vollzogen: false,
            vermoegensverfuegung: true,
            vermoegensnachteil: true,
            noetigungsmittel: Some(Noetigungsmittel::DrohungEmpfindlichesUebel),
            finalzusammenhang: true,
            vorsatz: true,
            zueignungs_oder_bereicherungsabsicht: true,
            auf_frischer_tat_betroffen: false,
            besitzerhaltungsabsicht: false,
            offence,
        }
    }

    #[test]
    fn raub_valid_case() {
        let o = RobberyOffence::Raub {
            minder_schwer: false,
        };
        assert_eq!(o.paragraph(), "§ 249 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 12);
        assert!(!r.allows_life());
        assert!(!r.fine_alternative);
        assert!(validate_robbery(&wegnahme_case(o)).is_ok());
    }

    #[test]
    fn raub_missing_noetigungsmittel() {
        let mut c = wegnahme_case(RobberyOffence::Raub {
            minder_schwer: false,
        });
        c.noetigungsmittel = None;
        assert!(matches!(
            validate_robbery(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
        // A mere threat of a serious harm is not a qualified robbery means.
        c.noetigungsmittel = Some(Noetigungsmittel::DrohungEmpfindlichesUebel);
        assert!(matches!(
            validate_robbery(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }

    #[test]
    fn raub_missing_finalzusammenhang() {
        let mut c = wegnahme_case(RobberyOffence::Raub {
            minder_schwer: false,
        });
        c.finalzusammenhang = false;
        assert!(matches!(
            validate_robbery(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }

    #[test]
    fn raub_missing_zueignungsabsicht() {
        let mut c = wegnahme_case(RobberyOffence::Raub {
            minder_schwer: false,
        });
        c.zueignungs_oder_bereicherungsabsicht = false;
        assert!(matches!(
            validate_robbery(&c),
            Err(StgbError::AbsichtMissing { .. })
        ));
    }

    #[test]
    fn raub_requires_vorsatz() {
        let mut c = wegnahme_case(RobberyOffence::Raub {
            minder_schwer: false,
        });
        c.vorsatz = false;
        assert!(matches!(
            validate_robbery(&c),
            Err(StgbError::FahrlaessigkeitNichtStrafbar)
        ));
    }

    #[test]
    fn raub_minder_schwer_range() {
        let o = RobberyOffence::Raub {
            minder_schwer: true,
        };
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 6);
        assert_eq!(r.max_months, Some(60));
        assert!(!r.fine_alternative);
        assert!(validate_robbery(&wegnahme_case(o)).is_ok());
    }

    #[test]
    fn schwerer_raub_abs1_vs_abs2_ranges() {
        let abs1 = RobberyOffence::SchwererRaub {
            merkmale: vec![SchwererRaubMerkmal::WaffeBeisichGefuehrt250_1],
            abs2: false,
            minder_schwer: false,
        };
        assert_eq!(abs1.paragraph(), "§ 250 StGB");
        let r1 = abs1.strafrahmen();
        assert_eq!(r1.effective_min_months(), 36);
        assert!(!r1.allows_life());
        assert!(validate_robbery(&wegnahme_case(abs1)).is_ok());

        let abs2 = RobberyOffence::SchwererRaub {
            merkmale: vec![SchwererRaubMerkmal::WaffeVerwendet250_2],
            abs2: true,
            minder_schwer: false,
        };
        let r2 = abs2.strafrahmen();
        assert_eq!(r2.effective_min_months(), 60);
        assert!(validate_robbery(&wegnahme_case(abs2)).is_ok());
    }

    #[test]
    fn schwerer_raub_minder_schwer_range() {
        let o = RobberyOffence::SchwererRaub {
            merkmale: vec![SchwererRaubMerkmal::Bandenraub],
            abs2: false,
            minder_schwer: true,
        };
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 12);
        assert_eq!(r.max_months, Some(120));
    }

    #[test]
    fn schwerer_raub_requires_merkmal_and_consistent_abs2() {
        // No qualifying circumstance at all.
        let mut c = wegnahme_case(RobberyOffence::SchwererRaub {
            merkmale: vec![],
            abs2: false,
            minder_schwer: false,
        });
        assert!(matches!(
            validate_robbery(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
        // abs2 flag inconsistent with the (Abs. 1) circumstance present.
        c.offence = RobberyOffence::SchwererRaub {
            merkmale: vec![SchwererRaubMerkmal::WaffeBeisichGefuehrt250_1],
            abs2: true,
            minder_schwer: false,
        };
        assert!(matches!(
            validate_robbery(&c),
            Err(StgbError::InvalidField { .. })
        ));
    }

    #[test]
    fn raub_mit_todesfolge_allows_life_and_has_floor() {
        let o = RobberyOffence::RaubMitTodesfolge;
        assert_eq!(o.paragraph(), "§ 251 StGB");
        let r = o.strafrahmen();
        // § 251 admits BOTH life imprisonment AND a ten-year floor.
        assert!(r.allows_life());
        assert_eq!(r.effective_min_months(), 120);
        assert!(!r.fine_alternative);

        // A full robbery with the (recklessly caused) fatal result fulfilled.
        let mut c = wegnahme_case(o);
        c.vermoegensnachteil = true; // repurposed as "Todeserfolg" flag, see docs.
        assert!(validate_robbery(&c).is_ok());

        // Without the fatal result the element is missing.
        c.vermoegensnachteil = false;
        assert!(matches!(
            validate_robbery(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }

    #[test]
    fn raeuberischer_diebstahl_elements_and_range() {
        let o = RobberyOffence::RaeuberischerDiebstahl;
        assert_eq!(o.paragraph(), "§ 252 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 12);
        assert!(!r.allows_life());

        let mut c = wegnahme_case(o);
        c.auf_frischer_tat_betroffen = true;
        c.besitzerhaltungsabsicht = true;
        assert!(validate_robbery(&c).is_ok());

        // Missing the "caught in the act" element.
        c.auf_frischer_tat_betroffen = false;
        assert!(matches!(
            validate_robbery(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }

    #[test]
    fn raeuberischer_diebstahl_missing_besitzerhaltungsabsicht() {
        let mut c = wegnahme_case(RobberyOffence::RaeuberischerDiebstahl);
        c.auf_frischer_tat_betroffen = true;
        c.besitzerhaltungsabsicht = false;
        assert!(matches!(
            validate_robbery(&c),
            Err(StgbError::AbsichtMissing { .. })
        ));
    }

    #[test]
    fn erpressung_range_and_fine_alternative() {
        let o = RobberyOffence::Erpressung {
            besonders_schwer: false,
        };
        assert_eq!(o.paragraph(), "§ 253 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.max_months, Some(60));
        assert!(r.fine_alternative);
        assert!(!r.allows_life());
        assert!(validate_robbery(&erpressung_case(o)).is_ok());

        // Especially serious case: one to fifteen years, no fine.
        let schwer = RobberyOffence::Erpressung {
            besonders_schwer: true,
        };
        let rs = schwer.strafrahmen();
        assert_eq!(rs.effective_min_months(), 12);
        assert_eq!(rs.max_months, Some(180));
        assert!(!rs.fine_alternative);
    }

    #[test]
    fn erpressung_missing_bereicherungsabsicht() {
        let mut c = erpressung_case(RobberyOffence::Erpressung {
            besonders_schwer: false,
        });
        c.zueignungs_oder_bereicherungsabsicht = false;
        assert!(matches!(
            validate_robbery(&c),
            Err(StgbError::AbsichtMissing { .. })
        ));
    }

    #[test]
    fn erpressung_missing_vermoegensnachteil() {
        let mut c = erpressung_case(RobberyOffence::Erpressung {
            besonders_schwer: false,
        });
        c.vermoegensnachteil = false;
        assert!(matches!(
            validate_robbery(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }

    #[test]
    fn raeuberische_erpressung_range_and_qualified_means() {
        let o = RobberyOffence::RaeuberischeErpressung;
        assert_eq!(o.paragraph(), "§ 255 StGB");
        let r = o.strafrahmen();
        assert_eq!(r.effective_min_months(), 12);
        assert!(!r.allows_life());
        assert!(!r.fine_alternative);

        // § 255 requires the qualified robbery means: force against a person.
        let mut c = erpressung_case(o);
        c.noetigungsmittel = Some(Noetigungsmittel::GewaltGegenPerson);
        assert!(validate_robbery(&c).is_ok());

        // The lesser means (threat of a serious harm) is insufficient for § 255.
        c.noetigungsmittel = Some(Noetigungsmittel::DrohungEmpfindlichesUebel);
        assert!(matches!(
            validate_robbery(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }

    #[test]
    fn noetigungsmittel_qualification_helper() {
        assert!(Noetigungsmittel::GewaltGegenPerson.ist_raubqualifiziert());
        assert!(Noetigungsmittel::DrohungLeibLeben.ist_raubqualifiziert());
        assert!(!Noetigungsmittel::DrohungEmpfindlichesUebel.ist_raubqualifiziert());
    }

    #[test]
    fn wegnahmedelikt_classification() {
        assert!(
            RobberyOffence::Raub {
                minder_schwer: false
            }
            .ist_wegnahmedelikt()
        );
        assert!(RobberyOffence::RaeuberischerDiebstahl.ist_wegnahmedelikt());
        assert!(!RobberyOffence::RaeuberischeErpressung.ist_wegnahmedelikt());
        assert!(
            !RobberyOffence::Erpressung {
                besonders_schwer: false
            }
            .ist_wegnahmedelikt()
        );
    }

    #[test]
    fn new_constructor_clears_flags() {
        let c = RobberyCase::new(RobberyOffence::Raub {
            minder_schwer: false,
        });
        assert!(c.tatobjekt.is_empty());
        assert!(!c.vorsatz);
        assert!(!c.fremde_bewegliche_sache);
        assert!(c.noetigungsmittel.is_none());
    }
}
