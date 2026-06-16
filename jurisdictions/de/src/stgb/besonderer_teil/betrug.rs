//! StGB §§ 263-266b - Fraud and Breach of Trust (Betrug und Untreue)
//!
//! This module models the central property-fraud offences of the German Criminal
//! Code (Strafgesetzbuch), each with its objective and subjective elements
//! (Tatbestandsmerkmale), a validator, and the statutory sentencing range
//! (Strafrahmen) drawn from [`crate::stgb::strafe`].
//!
//! ## § 263 StGB - Betrug (Fraud)
//!
//! > (1) Wer in der Absicht, sich oder einem Dritten einen rechtswidrigen
//! > Vermögensvorteil zu verschaffen, das Vermögen eines anderen dadurch
//! > beschädigt, dass er durch Vorspiegelung falscher oder durch Entstellung oder
//! > Unterdrückung wahrer Tatsachen einen Irrtum erregt oder unterhält, wird mit
//! > Freiheitsstrafe bis zu fünf Jahren oder mit Geldstrafe bestraft.
//!
//! **English**: Whoever, with the intent of obtaining an unlawful pecuniary
//! benefit for himself or a third party, damages the assets of another by causing
//! or maintaining an error by pretending false facts or by distorting or
//! suppressing true facts, is punished with imprisonment of **up to five years or
//! a fine**.
//!
//! Fraud (§ 263) is built on a closed chain of objective elements, each one
//! **causally** producing the next:
//!
//! 1. **Täuschung über Tatsachen** - deception about facts (express, implied, or
//!    by omission where a guarantor duty / Garantenstellung exists);
//! 2. **Erregung oder Unterhaltung eines Irrtums** - causing or maintaining an
//!    error in the mind of the deceived person;
//! 3. **Vermögensverfügung** - a pecuniary disposition by the deceived person
//!    (Verfügungsbewusstsein, freiwillige Selbstschädigung);
//! 4. **Vermögensschaden** - a pecuniary loss, determined by the principle of
//!    overall balancing (Prinzip der Gesamtsaldierung).
//!
//! Subjectively it requires (5) **Vorsatz** (intent as to all objective elements)
//! and (6) **Absicht rechtswidriger stoffgleicher Bereicherung** - the intent to
//! obtain an **unlawful** pecuniary benefit that is **"stoffgleich"** with the
//! victim's loss (the benefit is the direct flip side of the loss).
//!
//! Sentencing:
//! - § 263 Abs. 1 - up to five years (60 months) or a fine.
//! - § 263 Abs. 2 - the attempt (Versuch) is punishable.
//! - § 263 Abs. 3 - especially serious case (besonders schwerer Fall), with
//!   Regelbeispiele (gewerbsmäßig, Bande, Vermögensverlust großen Ausmaßes, ...):
//!   six months to ten years.
//! - § 263 Abs. 5 - gewerbsmäßige Bande (commercial gang fraud): one to ten years.
//!
//! ## § 263a StGB - Computerbetrug (Computer fraud)
//!
//! Computer fraud mirrors § 263 but replaces the human error and disposition with
//! the **Beeinflussung des Ergebnisses eines Datenverarbeitungsvorgangs**
//! (influencing the result of a data-processing operation) through one of four
//! modalities (see [`ComputerbetrugModalitaet`]): incorrect configuration of the
//! program, use of incorrect/incomplete data, unauthorised use of data, or other
//! unauthorised interference. It additionally requires a Vermögensschaden and the
//! same Bereicherungsabsicht. Sentence as for § 263: up to five years or a fine.
//!
//! ## § 265 StGB - Versicherungsmissbrauch (Insurance abuse)
//!
//! Damaging, destroying, impairing, disposing of, or handing over an insured thing
//! in order to obtain an insurance payout for oneself or a third party - a
//! subsidiary offence (subsidiär gegenüber § 263). Imprisonment of **up to three
//! years or a fine**.
//!
//! ## § 266 StGB - Untreue (Breach of trust)
//!
//! Two alternative variants protect another person's pecuniary interests:
//!
//! - **Missbrauchstatbestand** - abusing the power, granted by statute, public
//!   authority, or legal transaction, to dispose of another's assets or to obligate
//!   another;
//! - **Treubruchtatbestand** - breaching the duty to safeguard another's pecuniary
//!   interests (Vermögensbetreuungspflicht);
//!
//! in either case causing a **Vermögensnachteil** (pecuniary detriment) to the
//! person whose interests are to be safeguarded. Vorsatz is required.
//! - § 266 Abs. 1 - up to five years (60 months) or a fine.
//! - § 266 Abs. 2 - in especially serious cases, § 263 Abs. 3 applies accordingly:
//!   six months to ten years.
//!
//! ## § 266a StGB - Vorenthalten und Veruntreuen von Arbeitsentgelt
//!
//! Withholding an employee's social-security contributions from the collecting
//! agency. Imprisonment of **up to five years or a fine**.
//!
//! ## § 266b StGB - Missbrauch von Scheck- und Kreditkarten
//!
//! Abusing the possibility, granted by the handing-over of a cheque or credit card,
//! to induce the issuer to make a payment, thereby causing it a loss. Imprisonment
//! of **up to three years or a fine**.

use serde::{Deserialize, Serialize};

use crate::stgb::error::{Result, StgbError};
use crate::stgb::strafe::Strafrahmen;

/// A modality of computer fraud (Tatmodalität des Computerbetrugs), § 263a Abs. 1
/// StGB. The result of a data-processing operation may be influenced by one of
/// four mutually exclusive means.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComputerbetrugModalitaet {
    /// Incorrect configuration of the program (unrichtige Gestaltung des
    /// Programms), § 263a Abs. 1 Var. 1.
    ProgrammGestaltung,
    /// Use of incorrect or incomplete data (Verwendung unrichtiger oder
    /// unvollständiger Daten), § 263a Abs. 1 Var. 2.
    UnrichtigeDaten,
    /// Unauthorised use of data (unbefugte Verwendung von Daten), § 263a Abs. 1
    /// Var. 3 - the practically most important modality (e.g. misuse of card data).
    UnbefugteDatenverwendung,
    /// Other unauthorised interference with the operation (sonstige unbefugte
    /// Einwirkung auf den Ablauf), § 263a Abs. 1 Var. 4.
    SonstigeEinwirkung,
}

impl ComputerbetrugModalitaet {
    /// The variant number within § 263a Abs. 1 StGB (1-4).
    #[must_use]
    pub fn variante(&self) -> u8 {
        match self {
            ComputerbetrugModalitaet::ProgrammGestaltung => 1,
            ComputerbetrugModalitaet::UnrichtigeDaten => 2,
            ComputerbetrugModalitaet::UnbefugteDatenverwendung => 3,
            ComputerbetrugModalitaet::SonstigeEinwirkung => 4,
        }
    }

    /// Short German label of the modality.
    #[must_use]
    pub fn bezeichnung(&self) -> &'static str {
        match self {
            ComputerbetrugModalitaet::ProgrammGestaltung => "unrichtige Gestaltung des Programms",
            ComputerbetrugModalitaet::UnrichtigeDaten => {
                "Verwendung unrichtiger oder unvollständiger Daten"
            }
            ComputerbetrugModalitaet::UnbefugteDatenverwendung => "unbefugte Verwendung von Daten",
            ComputerbetrugModalitaet::SonstigeEinwirkung => "sonstige unbefugte Einwirkung",
        }
    }
}

/// The specific fraud or breach-of-trust offence applicable to a case
/// (§§ 263-266b StGB).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FraudOffence {
    /// Fraud (Betrug, § 263 StGB).
    Betrug {
        /// Whether an especially serious case applies (§ 263 Abs. 3 →
        /// 6 months to 10 years).
        besonders_schwer: bool,
        /// Whether commercial gang fraud applies (§ 263 Abs. 5 →
        /// 1 to 10 years). Takes precedence over `besonders_schwer`.
        bande_gewerbsmaessig: bool,
    },
    /// Computer fraud (Computerbetrug, § 263a StGB).
    Computerbetrug {
        /// The modality through which the data-processing result was influenced.
        modalitaet: ComputerbetrugModalitaet,
    },
    /// Insurance abuse (Versicherungsmissbrauch, § 265 StGB).
    Versicherungsmissbrauch,
    /// Breach of trust - abuse variant (Untreue, Missbrauchstatbestand,
    /// § 266 Abs. 1 Var. 1 StGB).
    UntreueMissbrauch {
        /// Whether an especially serious case applies (§ 266 Abs. 2 i.V.m.
        /// § 263 Abs. 3 → 6 months to 10 years).
        besonders_schwer: bool,
    },
    /// Breach of trust - fiduciary-breach variant (Untreue, Treubruchtatbestand,
    /// § 266 Abs. 1 Var. 2 StGB).
    UntreueTreubruch {
        /// Whether an especially serious case applies (§ 266 Abs. 2 i.V.m.
        /// § 263 Abs. 3 → 6 months to 10 years).
        besonders_schwer: bool,
    },
    /// Withholding and embezzling of employee remuneration (Vorenthalten und
    /// Veruntreuen von Arbeitsentgelt, § 266a StGB).
    VorenthaltenArbeitsentgelt,
    /// Abuse of cheque and credit cards (Missbrauch von Scheck- und Kreditkarten,
    /// § 266b StGB).
    MissbrauchKreditkarte,
}

impl FraudOffence {
    /// The § citation of the offence.
    #[must_use]
    pub fn paragraph(&self) -> &'static str {
        match self {
            FraudOffence::Betrug { .. } => "§ 263 StGB",
            FraudOffence::Computerbetrug { .. } => "§ 263a StGB",
            FraudOffence::Versicherungsmissbrauch => "§ 265 StGB",
            FraudOffence::UntreueMissbrauch { .. } | FraudOffence::UntreueTreubruch { .. } => {
                "§ 266 StGB"
            }
            FraudOffence::VorenthaltenArbeitsentgelt => "§ 266a StGB",
            FraudOffence::MissbrauchKreditkarte => "§ 266b StGB",
        }
    }

    /// The statutory sentencing range (Strafrahmen) of the offence.
    #[must_use]
    pub fn strafrahmen(&self) -> Strafrahmen {
        match self {
            // § 263 StGB - up to five years or a fine; § 263 Abs. 3 (besonders
            // schwerer Fall) 6 months to 10 years; § 263 Abs. 5 (gewerbsmäßige
            // Bande) 1 to 10 years.
            FraudOffence::Betrug {
                besonders_schwer,
                bande_gewerbsmaessig,
            } => {
                if *bande_gewerbsmaessig {
                    Strafrahmen::imprisonment(12, 120)
                } else if *besonders_schwer {
                    Strafrahmen::imprisonment(6, 120)
                } else {
                    Strafrahmen::up_to_months_or_fine(60)
                }
            }
            // § 263a StGB - same range as § 263 Abs. 1.
            FraudOffence::Computerbetrug { .. } => Strafrahmen::up_to_months_or_fine(60),
            // § 265 StGB - up to three years or a fine.
            FraudOffence::Versicherungsmissbrauch => Strafrahmen::up_to_months_or_fine(36),
            // § 266 Abs. 1 StGB - up to five years or a fine; § 266 Abs. 2 i.V.m.
            // § 263 Abs. 3 (besonders schwerer Fall) 6 months to 10 years.
            FraudOffence::UntreueMissbrauch { besonders_schwer }
            | FraudOffence::UntreueTreubruch { besonders_schwer } => {
                if *besonders_schwer {
                    Strafrahmen::imprisonment(6, 120)
                } else {
                    Strafrahmen::up_to_months_or_fine(60)
                }
            }
            // § 266a StGB - up to five years or a fine.
            FraudOffence::VorenthaltenArbeitsentgelt => Strafrahmen::up_to_months_or_fine(60),
            // § 266b StGB - up to three years or a fine.
            FraudOffence::MissbrauchKreditkarte => Strafrahmen::up_to_months_or_fine(36),
        }
    }

    /// Whether the attempt (Versuch) of the offence is punishable. Fraud
    /// (§ 263 Abs. 2) and computer fraud (§ 263a Abs. 2 i.V.m. § 263 Abs. 2) are
    /// expressly punishable as attempts.
    #[must_use]
    pub fn versuch_strafbar(&self) -> bool {
        matches!(
            self,
            FraudOffence::Betrug { .. } | FraudOffence::Computerbetrug { .. }
        )
    }
}

/// A fraud or breach-of-trust case (§§ 263-266b StGB).
///
/// The fields model the full objective and subjective elements
/// (Tatbestandsmerkmale). Which fields are decisive depends on the chosen
/// [`FraudOffence`]; [`validate_fraud`] applies the offence-specific checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FraudCase {
    // === § 263 chain (objective elements) ===
    /// (1) Deception about facts (Täuschung über Tatsachen). § 263 Abs. 1.
    pub taeuschung_ueber_tatsachen: bool,
    /// (2) An error was caused or maintained in the victim (Irrtum erregt oder
    /// unterhalten). § 263 Abs. 1.
    pub irrtum_erregt: bool,
    /// (3) The deceived person made a pecuniary disposition (Vermögensverfügung).
    pub vermoegensverfuegung: bool,
    /// (4) A pecuniary loss occurred (Vermögensschaden, Gesamtsaldierung).
    pub vermoegensschaden: bool,
    /// Amount of the pecuniary loss / detriment in EUR cents
    /// (Schadenshöhe / Nachteilshöhe).
    pub schaden_betrag_cents: u64,

    // === Subjective elements ===
    /// (5) Intent as to all objective elements (Vorsatz). § 15 StGB.
    pub vorsatz: bool,
    /// (6) Intent to obtain a pecuniary benefit (Bereicherungsabsicht). § 263.
    pub bereicherungsabsicht: bool,
    /// Whether the intended benefit is "stoffgleich" with the loss
    /// (Stoffgleichheit - the benefit is the direct flip side of the loss).
    pub stoffgleichheit: bool,
    /// Whether the intended benefit is unlawful (Rechtswidrigkeit des
    /// erstrebten Vermögensvorteils).
    pub rechtswidrigkeit_des_vorteils: bool,

    // === Causation ===
    /// Whether the causal chain Täuschung → Irrtum → Verfügung → Schaden is
    /// closed (kausale Verknüpfung der Tatbestandsmerkmale).
    pub kausalkette_geschlossen: bool,

    // === § 263a specific ===
    /// Whether the result of a data-processing operation was influenced
    /// (Beeinflussung des Ergebnisses eines Datenverarbeitungsvorgangs).
    /// § 263a Abs. 1.
    pub datenverarbeitung_beeinflusst: bool,

    // === § 266 specific ===
    /// Whether the offender had a duty to safeguard another's pecuniary interests
    /// (Vermögensbetreuungspflicht). Decisive for the Treubruchtatbestand.
    pub vermoegensbetreuungspflicht: bool,
    /// Whether a pecuniary detriment was caused to the protected person
    /// (Vermögensnachteil). § 266 Abs. 1.
    pub vermoegensnachteil: bool,

    /// The offence the case is charged under.
    pub offence: FraudOffence,
}

impl FraudCase {
    /// A neutral case with all elements set to `false`/`0`, to be customised for a
    /// concrete offence. The offence must be supplied explicitly.
    #[must_use]
    pub fn new(offence: FraudOffence) -> Self {
        Self {
            taeuschung_ueber_tatsachen: false,
            irrtum_erregt: false,
            vermoegensverfuegung: false,
            vermoegensschaden: false,
            schaden_betrag_cents: 0,
            vorsatz: false,
            bereicherungsabsicht: false,
            stoffgleichheit: false,
            rechtswidrigkeit_des_vorteils: false,
            kausalkette_geschlossen: false,
            datenverarbeitung_beeinflusst: false,
            vermoegensbetreuungspflicht: false,
            vermoegensnachteil: false,
            offence,
        }
    }
}

/// Validate that a stated pecuniary loss carries a positive amount.
///
/// # Errors
/// Returns [`StgbError::InvalidAmount`] if `vorhanden` is `true` but the amount is
/// zero - a loss / detriment without quantum is contradictory.
fn pruefe_schadensbetrag(vorhanden: bool, betrag_cents: u64) -> Result<()> {
    if vorhanden && betrag_cents == 0 {
        return Err(StgbError::InvalidAmount {
            detail: "Vermögensschaden bejaht, aber Schadensbetrag ist 0 (§§ 263, 266 StGB)"
                .to_string(),
        });
    }
    Ok(())
}

/// Validate the subjective intent of enrichment (Absicht rechtswidriger
/// stoffgleicher Bereicherung), § 263 / § 263a StGB.
///
/// # Errors
/// Returns [`StgbError::AbsichtMissing`] if the intent to enrich, its
/// "Stoffgleichheit", or the unlawfulness of the intended benefit is missing.
fn pruefe_bereicherungsabsicht(case: &FraudCase) -> Result<()> {
    if !case.bereicherungsabsicht {
        return Err(StgbError::AbsichtMissing {
            detail: "Absicht rechtswidriger Bereicherung fehlt (§ 263 Abs. 1 StGB)".to_string(),
        });
    }
    if !case.stoffgleichheit {
        return Err(StgbError::AbsichtMissing {
            detail: "Stoffgleichheit zwischen Schaden und erstrebtem Vorteil fehlt (§ 263 StGB)"
                .to_string(),
        });
    }
    if !case.rechtswidrigkeit_des_vorteils {
        return Err(StgbError::AbsichtMissing {
            detail: "Erstrebter Vermögensvorteil ist nicht rechtswidrig (§ 263 Abs. 1 StGB)"
                .to_string(),
        });
    }
    Ok(())
}

/// Validate the objective § 263 chain: Täuschung → Irrtum → Verfügung → Schaden.
///
/// # Errors
/// - [`StgbError::TatbestandNotFulfilled`] if any link of the chain is missing.
/// - [`StgbError::NoKausalitaet`] if the chain is not causally closed.
/// - [`StgbError::InvalidAmount`] if a loss is asserted without a positive amount.
fn pruefe_betrugskette(case: &FraudCase) -> Result<()> {
    if !case.taeuschung_ueber_tatsachen {
        return Err(StgbError::TatbestandNotFulfilled {
            element: "Täuschung über Tatsachen (§ 263 Abs. 1 StGB)".to_string(),
        });
    }
    if !case.irrtum_erregt {
        return Err(StgbError::TatbestandNotFulfilled {
            element: "Erregung oder Unterhaltung eines Irrtums (§ 263 Abs. 1 StGB)".to_string(),
        });
    }
    if !case.vermoegensverfuegung {
        return Err(StgbError::TatbestandNotFulfilled {
            element: "Vermögensverfügung des Getäuschten (§ 263 Abs. 1 StGB)".to_string(),
        });
    }
    if !case.vermoegensschaden {
        return Err(StgbError::TatbestandNotFulfilled {
            element: "Vermögensschaden (Gesamtsaldierung, § 263 Abs. 1 StGB)".to_string(),
        });
    }
    pruefe_schadensbetrag(case.vermoegensschaden, case.schaden_betrag_cents)?;
    if !case.kausalkette_geschlossen {
        return Err(StgbError::NoKausalitaet);
    }
    Ok(())
}

/// Validate a fraud or breach-of-trust case under §§ 263-266b StGB.
///
/// The check is offence-specific:
///
/// - **§ 263 (Betrug)**: the full objective chain (Täuschung → Irrtum →
///   Vermögensverfügung → Vermögensschaden) must be causally closed; the offender
///   must act with Vorsatz and with the intent of an unlawful, "stoffgleich"
///   enrichment.
/// - **§ 263a (Computerbetrug)**: the influence on the data-processing result, a
///   Vermögensschaden, Vorsatz, and the same Bereicherungsabsicht.
/// - **§ 266 (Untreue)**: the fiduciary-breach variant requires a
///   Vermögensbetreuungspflicht; both variants require a Vermögensnachteil and
///   Vorsatz.
/// - **§§ 265, 266a, 266b**: Vorsatz and (where a loss is asserted) a positive
///   amount.
///
/// # Errors
/// - [`StgbError::TatbestandNotFulfilled`] if a required objective element is
///   missing.
/// - [`StgbError::NoKausalitaet`] if the § 263 causal chain is not closed.
/// - [`StgbError::AbsichtMissing`] if the intent of unlawful stoffgleiche
///   enrichment is missing.
/// - [`StgbError::FahrlaessigkeitNichtStrafbar`] if Vorsatz is missing (none of
///   these offences is punishable on mere negligence).
/// - [`StgbError::InvalidAmount`] if a loss / detriment is asserted with a zero
///   amount.
pub fn validate_fraud(case: &FraudCase) -> Result<()> {
    match &case.offence {
        FraudOffence::Betrug { .. } => {
            pruefe_betrugskette(case)?;
            if !case.vorsatz {
                return Err(StgbError::FahrlaessigkeitNichtStrafbar);
            }
            pruefe_bereicherungsabsicht(case)?;
        }
        FraudOffence::Computerbetrug { .. } => {
            if !case.datenverarbeitung_beeinflusst {
                return Err(StgbError::TatbestandNotFulfilled {
                    element: "Beeinflussung des Ergebnisses eines Datenverarbeitungsvorgangs \
                              (§ 263a Abs. 1 StGB)"
                        .to_string(),
                });
            }
            if !case.vermoegensschaden {
                return Err(StgbError::TatbestandNotFulfilled {
                    element: "Vermögensschaden (§ 263a i.V.m. § 263 Abs. 1 StGB)".to_string(),
                });
            }
            pruefe_schadensbetrag(case.vermoegensschaden, case.schaden_betrag_cents)?;
            if !case.vorsatz {
                return Err(StgbError::FahrlaessigkeitNichtStrafbar);
            }
            pruefe_bereicherungsabsicht(case)?;
        }
        FraudOffence::UntreueMissbrauch { .. } => {
            if !case.vermoegensnachteil {
                return Err(StgbError::TatbestandNotFulfilled {
                    element: "Vermögensnachteil des Treugebers (§ 266 Abs. 1 StGB)".to_string(),
                });
            }
            pruefe_schadensbetrag(case.vermoegensnachteil, case.schaden_betrag_cents)?;
            if !case.vorsatz {
                return Err(StgbError::FahrlaessigkeitNichtStrafbar);
            }
        }
        FraudOffence::UntreueTreubruch { .. } => {
            if !case.vermoegensbetreuungspflicht {
                return Err(StgbError::TatbestandNotFulfilled {
                    element: "Vermögensbetreuungspflicht (Treubruchtatbestand, § 266 Abs. 1 StGB)"
                        .to_string(),
                });
            }
            if !case.vermoegensnachteil {
                return Err(StgbError::TatbestandNotFulfilled {
                    element: "Vermögensnachteil des Treugebers (§ 266 Abs. 1 StGB)".to_string(),
                });
            }
            pruefe_schadensbetrag(case.vermoegensnachteil, case.schaden_betrag_cents)?;
            if !case.vorsatz {
                return Err(StgbError::FahrlaessigkeitNichtStrafbar);
            }
        }
        FraudOffence::Versicherungsmissbrauch
        | FraudOffence::VorenthaltenArbeitsentgelt
        | FraudOffence::MissbrauchKreditkarte => {
            // These subsidiary offences are punishable only when committed
            // intentionally (§ 15 StGB); a stated loss must be quantified.
            pruefe_schadensbetrag(case.vermoegensschaden, case.schaden_betrag_cents)?;
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

    /// A fully satisfied § 263 fraud case (all elements present).
    fn valid_betrug() -> FraudCase {
        FraudCase {
            taeuschung_ueber_tatsachen: true,
            irrtum_erregt: true,
            vermoegensverfuegung: true,
            vermoegensschaden: true,
            schaden_betrag_cents: 50_000,
            vorsatz: true,
            bereicherungsabsicht: true,
            stoffgleichheit: true,
            rechtswidrigkeit_des_vorteils: true,
            kausalkette_geschlossen: true,
            datenverarbeitung_beeinflusst: false,
            vermoegensbetreuungspflicht: false,
            vermoegensnachteil: false,
            offence: FraudOffence::Betrug {
                besonders_schwer: false,
                bande_gewerbsmaessig: false,
            },
        }
    }

    #[test]
    fn betrug_full_chain_is_valid() {
        let c = valid_betrug();
        assert_eq!(c.offence.paragraph(), "§ 263 StGB");
        let r = c.offence.strafrahmen();
        assert_eq!(r.max_months, Some(60));
        assert!(r.fine_alternative);
        assert!(c.offence.versuch_strafbar());
        assert!(validate_fraud(&c).is_ok());
    }

    #[test]
    fn betrug_broken_causal_chain() {
        let mut c = valid_betrug();
        c.kausalkette_geschlossen = false;
        assert!(matches!(validate_fraud(&c), Err(StgbError::NoKausalitaet)));
    }

    #[test]
    fn betrug_missing_taeuschung_breaks_tatbestand() {
        let mut c = valid_betrug();
        c.taeuschung_ueber_tatsachen = false;
        assert!(matches!(
            validate_fraud(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }

    #[test]
    fn betrug_missing_bereicherungsabsicht() {
        let mut c = valid_betrug();
        c.bereicherungsabsicht = false;
        assert!(matches!(
            validate_fraud(&c),
            Err(StgbError::AbsichtMissing { .. })
        ));
    }

    #[test]
    fn betrug_missing_stoffgleichheit() {
        let mut c = valid_betrug();
        c.stoffgleichheit = false;
        assert!(matches!(
            validate_fraud(&c),
            Err(StgbError::AbsichtMissing { .. })
        ));
    }

    #[test]
    fn betrug_unlawful_benefit_required() {
        let mut c = valid_betrug();
        c.rechtswidrigkeit_des_vorteils = false;
        assert!(matches!(
            validate_fraud(&c),
            Err(StgbError::AbsichtMissing { .. })
        ));
    }

    #[test]
    fn betrug_missing_intent_is_negligence() {
        let mut c = valid_betrug();
        c.vorsatz = false;
        assert!(matches!(
            validate_fraud(&c),
            Err(StgbError::FahrlaessigkeitNichtStrafbar)
        ));
    }

    #[test]
    fn betrug_loss_without_amount_is_invalid() {
        let mut c = valid_betrug();
        c.schaden_betrag_cents = 0;
        assert!(matches!(
            validate_fraud(&c),
            Err(StgbError::InvalidAmount { .. })
        ));
    }

    #[test]
    fn betrug_besonders_schwer_range() {
        let o = FraudOffence::Betrug {
            besonders_schwer: true,
            bande_gewerbsmaessig: false,
        };
        let r = o.strafrahmen();
        // § 263 Abs. 3 - 6 months to 10 years, no fine alternative.
        assert_eq!(r.effective_min_months(), 6);
        assert_eq!(r.max_months, Some(120));
        assert!(!r.fine_alternative);
        assert!(!r.allows_life());
    }

    #[test]
    fn betrug_bande_gewerbsmaessig_range() {
        let o = FraudOffence::Betrug {
            besonders_schwer: false,
            bande_gewerbsmaessig: true,
        };
        let r = o.strafrahmen();
        // § 263 Abs. 5 - 1 to 10 years.
        assert_eq!(r.effective_min_months(), 12);
        assert_eq!(r.max_months, Some(120));
        assert!(!r.fine_alternative);
    }

    #[test]
    fn computerbetrug_valid_and_modalities() {
        for modalitaet in [
            ComputerbetrugModalitaet::ProgrammGestaltung,
            ComputerbetrugModalitaet::UnrichtigeDaten,
            ComputerbetrugModalitaet::UnbefugteDatenverwendung,
            ComputerbetrugModalitaet::SonstigeEinwirkung,
        ] {
            let mut c = valid_betrug();
            c.offence = FraudOffence::Computerbetrug { modalitaet };
            c.datenverarbeitung_beeinflusst = true;
            assert_eq!(c.offence.paragraph(), "§ 263a StGB");
            assert_eq!(c.offence.strafrahmen().max_months, Some(60));
            assert!(validate_fraud(&c).is_ok());
        }
        // Variant numbering and labels are stable.
        assert_eq!(ComputerbetrugModalitaet::ProgrammGestaltung.variante(), 1);
        assert_eq!(
            ComputerbetrugModalitaet::UnbefugteDatenverwendung.variante(),
            3
        );
        assert!(
            !ComputerbetrugModalitaet::SonstigeEinwirkung
                .bezeichnung()
                .is_empty()
        );
    }

    #[test]
    fn computerbetrug_requires_datenverarbeitung() {
        let mut c = valid_betrug();
        c.offence = FraudOffence::Computerbetrug {
            modalitaet: ComputerbetrugModalitaet::UnbefugteDatenverwendung,
        };
        c.datenverarbeitung_beeinflusst = false;
        assert!(matches!(
            validate_fraud(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }

    #[test]
    fn untreue_treubruch_valid_and_requires_betreuungspflicht() {
        let mut c = FraudCase::new(FraudOffence::UntreueTreubruch {
            besonders_schwer: false,
        });
        c.vermoegensbetreuungspflicht = true;
        c.vermoegensnachteil = true;
        c.schaden_betrag_cents = 100_000;
        c.vorsatz = true;
        assert_eq!(c.offence.paragraph(), "§ 266 StGB");
        assert!(validate_fraud(&c).is_ok());

        // Without the fiduciary duty the Treubruchtatbestand fails.
        c.vermoegensbetreuungspflicht = false;
        assert!(matches!(
            validate_fraud(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
    }

    #[test]
    fn untreue_missbrauch_requires_nachteil_and_intent() {
        let mut c = FraudCase::new(FraudOffence::UntreueMissbrauch {
            besonders_schwer: false,
        });
        // No Vermögensnachteil yet.
        assert!(matches!(
            validate_fraud(&c),
            Err(StgbError::TatbestandNotFulfilled { .. })
        ));
        c.vermoegensnachteil = true;
        c.schaden_betrag_cents = 25_000;
        c.vorsatz = false;
        assert!(matches!(
            validate_fraud(&c),
            Err(StgbError::FahrlaessigkeitNichtStrafbar)
        ));
        c.vorsatz = true;
        assert!(validate_fraud(&c).is_ok());
    }

    #[test]
    fn untreue_range_basic_and_besonders_schwer() {
        let basic = FraudOffence::UntreueTreubruch {
            besonders_schwer: false,
        };
        let r = basic.strafrahmen();
        assert_eq!(r.max_months, Some(60));
        assert!(r.fine_alternative);

        let schwer = FraudOffence::UntreueMissbrauch {
            besonders_schwer: true,
        };
        let rs = schwer.strafrahmen();
        // § 266 Abs. 2 i.V.m. § 263 Abs. 3 - 6 months to 10 years.
        assert_eq!(rs.effective_min_months(), 6);
        assert_eq!(rs.max_months, Some(120));
        assert!(!rs.fine_alternative);
    }

    #[test]
    fn subsidiary_offences_ranges_and_fine_alternative() {
        // § 265 - up to 3 years or fine.
        let vm = FraudOffence::Versicherungsmissbrauch;
        assert_eq!(vm.paragraph(), "§ 265 StGB");
        assert_eq!(vm.strafrahmen().max_months, Some(36));
        assert!(vm.strafrahmen().fine_alternative);

        // § 266a - up to 5 years or fine.
        let ae = FraudOffence::VorenthaltenArbeitsentgelt;
        assert_eq!(ae.paragraph(), "§ 266a StGB");
        assert_eq!(ae.strafrahmen().max_months, Some(60));
        assert!(ae.strafrahmen().fine_alternative);

        // § 266b - up to 3 years or fine.
        let kk = FraudOffence::MissbrauchKreditkarte;
        assert_eq!(kk.paragraph(), "§ 266b StGB");
        assert_eq!(kk.strafrahmen().max_months, Some(36));
        assert!(kk.strafrahmen().fine_alternative);

        // None of the subsidiary offences is an attempt-punishable felony here.
        assert!(!vm.versuch_strafbar());
        assert!(!ae.versuch_strafbar());
        assert!(!kk.versuch_strafbar());
    }

    #[test]
    fn subsidiary_offences_require_intent() {
        let mut c = FraudCase::new(FraudOffence::Versicherungsmissbrauch);
        c.vorsatz = false;
        assert!(matches!(
            validate_fraud(&c),
            Err(StgbError::FahrlaessigkeitNichtStrafbar)
        ));
        c.vorsatz = true;
        assert!(validate_fraud(&c).is_ok());
    }
}
