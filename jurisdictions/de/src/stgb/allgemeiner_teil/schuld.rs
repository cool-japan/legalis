//! StGB §§ 15-18 - Intent, Negligence, Mistake (Vorsatz, Fahrlässigkeit, Irrtum)
//!
//! ## § 15 StGB - Vorsätzliches und fahrlässiges Handeln
//!
//! > Strafbar ist nur vorsätzliches Handeln, wenn nicht das Gesetz fahrlässiges
//! > Handeln ausdrücklich mit Strafe bedroht.
//!
//! **English**: Only intentional conduct is punishable, unless the law expressly
//! penalises negligent conduct. (Vorsatz-principle / Schuldprinzip.)
//!
//! ## § 16 StGB - Irrtum über Tatumstände (Mistake of fact)
//!
//! > (1) Wer bei Begehung der Tat einen Umstand nicht kennt, der zum gesetzlichen
//! > Tatbestand gehört, handelt nicht vorsätzlich. …
//!
//! A mistake about a circumstance belonging to the offence definition excludes
//! intent (Tatbestandsirrtum). Liability for negligence remains possible.
//!
//! ## § 17 StGB - Verbotsirrtum (Mistake of law)
//!
//! If the offender lacks the awareness of doing wrong and the mistake was
//! **unavoidable**, he acts without culpability; if **avoidable**, the sentence
//! may be mitigated (§ 49 Abs. 1 StGB).
//!
//! ## § 18 StGB - Schwerere Strafe bei besonderen Tatfolgen
//!
//! For offences whose punishment is increased by a particular result
//! (erfolgsqualifizierte Delikte), the heavier penalty applies only if the
//! offender acted **at least negligently** with respect to that result.
//!
//! # Forms of intent (Vorsatzformen) - doctrine
//!
//! German doctrine distinguishes three degrees of intent:
//! - **dolus directus 1. Grades** (Absicht): the offender's aim.
//! - **dolus directus 2. Grades** (direkter Vorsatz / Wissentlichkeit): the
//!   offender knows the result will occur for certain.
//! - **dolus eventualis** (bedingter Vorsatz): the offender considers the result
//!   possible and reconciles himself with it (billigend in Kauf nehmen).
//!
//! And two degrees of negligence:
//! - **bewusste Fahrlässigkeit**: the offender recognises the risk but trusts the
//!   result will not occur.
//! - **unbewusste Fahrlässigkeit**: the offender fails to recognise a risk he
//!   could and should have recognised.

use serde::{Deserialize, Serialize};

use crate::stgb::error::{Result, StgbError};

/// Subjective offence element: the offender's form of guilt (Schuldform).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Schuldform {
    /// Intent of the first degree (Absicht / dolus directus 1. Grades).
    Absicht,
    /// Direct intent of the second degree (direkter Vorsatz / Wissentlichkeit).
    DirekterVorsatz,
    /// Conditional intent (bedingter Vorsatz / dolus eventualis).
    BedingterVorsatz,
    /// Conscious negligence (bewusste Fahrlässigkeit).
    BewussteFahrlaessigkeit,
    /// Unconscious negligence (unbewusste Fahrlässigkeit).
    UnbewussteFahrlaessigkeit,
}

impl Schuldform {
    /// Whether this form constitutes intent (Vorsatz) in any of its three degrees.
    #[must_use]
    pub fn is_vorsatz(&self) -> bool {
        matches!(
            self,
            Schuldform::Absicht | Schuldform::DirekterVorsatz | Schuldform::BedingterVorsatz
        )
    }

    /// Whether this form constitutes negligence (Fahrlässigkeit).
    #[must_use]
    pub fn is_fahrlaessigkeit(&self) -> bool {
        matches!(
            self,
            Schuldform::BewussteFahrlaessigkeit | Schuldform::UnbewussteFahrlaessigkeit
        )
    }

    /// Whether the form satisfies "at least negligence" (§ 18 StGB: wenigstens
    /// Fahrlässigkeit) - i.e. any intent or negligence.
    #[must_use]
    pub fn is_at_least_negligence(&self) -> bool {
        self.is_vorsatz() || self.is_fahrlaessigkeit()
    }
}

/// Whether an offence requires intent or also penalises negligence (§ 15 StGB).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OffenceMensRea {
    /// The offence is only punishable when committed intentionally (Vorsatzdelikt).
    VorsatzOnly,
    /// The law expressly penalises negligent commission (Fahrlässigkeitsdelikt),
    /// e.g. § 222 StGB (fahrlässige Tötung), § 229 StGB (fahrlässige
    /// Körperverletzung).
    FahrlaessigkeitPunishable,
}

/// Check the subjective requirement of § 15 StGB for an actor's [`Schuldform`].
///
/// Per § 15 StGB, only intentional conduct is punishable unless the offence
/// expressly penalises negligence.
///
/// # Errors
/// - [`StgbError::FahrlaessigkeitNichtStrafbar`] if the offence is a pure
///   intent-offence but the actor acted only negligently.
/// - [`StgbError::NoSchuldform`] if neither intent nor (punishable) negligence
///   is present.
pub fn check_mens_rea(offence: OffenceMensRea, actor: Schuldform) -> Result<()> {
    match offence {
        OffenceMensRea::VorsatzOnly => {
            if actor.is_vorsatz() {
                Ok(())
            } else {
                Err(StgbError::FahrlaessigkeitNichtStrafbar)
            }
        }
        OffenceMensRea::FahrlaessigkeitPunishable => {
            if actor.is_at_least_negligence() {
                Ok(())
            } else {
                Err(StgbError::NoSchuldform)
            }
        }
    }
}

/// A mistake (Irrtum) asserted by the offender (§§ 16-17 StGB).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Irrtum {
    /// No mistake.
    Keiner,
    /// Mistake of fact about an element of the offence (Tatbestandsirrtum, § 16
    /// Abs. 1 StGB): excludes intent.
    Tatbestandsirrtum,
    /// Unavoidable mistake of law (unvermeidbarer Verbotsirrtum, § 17 S. 1 StGB):
    /// excludes culpability.
    UnvermeidbarerVerbotsirrtum,
    /// Avoidable mistake of law (vermeidbarer Verbotsirrtum, § 17 S. 2 StGB):
    /// culpability remains, sentence may be mitigated.
    VermeidbarerVerbotsirrtum,
}

impl Irrtum {
    /// Whether this mistake excludes intent (§ 16 Abs. 1 StGB).
    #[must_use]
    pub fn excludes_intent(&self) -> bool {
        matches!(self, Irrtum::Tatbestandsirrtum)
    }

    /// Whether this mistake excludes culpability (§ 17 S. 1 StGB).
    #[must_use]
    pub fn excludes_culpability(&self) -> bool {
        matches!(self, Irrtum::UnvermeidbarerVerbotsirrtum)
    }

    /// Whether this mistake merely allows mitigation (§ 17 S. 2 StGB).
    #[must_use]
    pub fn allows_mitigation(&self) -> bool {
        matches!(self, Irrtum::VermeidbarerVerbotsirrtum)
    }
}

/// Evaluate the effect of an asserted mistake (§§ 16-17 StGB).
///
/// # Errors
/// - [`StgbError::Tatbestandsirrtum`] if a mistake of fact excludes intent.
/// - [`StgbError::UnvermeidbarerVerbotsirrtum`] if an unavoidable mistake of law
///   excludes culpability.
///
/// Returns `Ok(())` when no relevant mistake bars liability (including a merely
/// avoidable mistake of law, which leaves liability intact).
pub fn evaluate_mistake(irrtum: Irrtum) -> Result<()> {
    if irrtum.excludes_intent() {
        return Err(StgbError::Tatbestandsirrtum);
    }
    if irrtum.excludes_culpability() {
        return Err(StgbError::UnvermeidbarerVerbotsirrtum);
    }
    Ok(())
}

/// A result-qualified offence (erfolgsqualifiziertes Delikt, § 18 StGB).
///
/// The heavier penalty for the particular (graver) result applies only if the
/// offender acted **at least negligently** as to that result. Examples:
/// § 227 StGB (Körperverletzung mit Todesfolge), § 251 StGB (Raub mit
/// Todesfolge).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Erfolgsqualifikation {
    /// Name/description of the basic offence (Grunddelikt), e.g. "Körperverletzung".
    pub grunddelikt: String,
    /// Name/description of the aggravating result (besondere Tatfolge), e.g.
    /// "Tod des Opfers".
    pub schwere_folge: String,
    /// The offender's form of guilt with respect to the aggravating result.
    pub schuldform_folge: Schuldform,
}

/// Check whether the heavier penalty of a result-qualified offence applies (§ 18
/// StGB).
///
/// # Errors
/// [`StgbError::NoSchuldform`] if the offender acted with neither intent nor
/// negligence as to the aggravating result, so the qualification does not apply.
pub fn check_erfolgsqualifikation(eq: &Erfolgsqualifikation) -> Result<()> {
    if eq.schuldform_folge.is_at_least_negligence() {
        Ok(())
    } else {
        Err(StgbError::NoSchuldform)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intent_forms_classified() {
        assert!(Schuldform::Absicht.is_vorsatz());
        assert!(Schuldform::DirekterVorsatz.is_vorsatz());
        assert!(Schuldform::BedingterVorsatz.is_vorsatz());
        assert!(!Schuldform::Absicht.is_fahrlaessigkeit());
    }

    #[test]
    fn negligence_forms_classified() {
        assert!(Schuldform::BewussteFahrlaessigkeit.is_fahrlaessigkeit());
        assert!(Schuldform::UnbewussteFahrlaessigkeit.is_fahrlaessigkeit());
        assert!(!Schuldform::BewussteFahrlaessigkeit.is_vorsatz());
    }

    #[test]
    fn vorsatzdelikt_requires_intent() {
        assert!(check_mens_rea(OffenceMensRea::VorsatzOnly, Schuldform::BedingterVorsatz).is_ok());
        let res = check_mens_rea(
            OffenceMensRea::VorsatzOnly,
            Schuldform::BewussteFahrlaessigkeit,
        );
        assert!(matches!(res, Err(StgbError::FahrlaessigkeitNichtStrafbar)));
    }

    #[test]
    fn fahrlaessigkeitsdelikt_accepts_negligence_and_intent() {
        assert!(
            check_mens_rea(
                OffenceMensRea::FahrlaessigkeitPunishable,
                Schuldform::UnbewussteFahrlaessigkeit
            )
            .is_ok()
        );
        assert!(
            check_mens_rea(
                OffenceMensRea::FahrlaessigkeitPunishable,
                Schuldform::Absicht
            )
            .is_ok()
        );
    }

    #[test]
    fn tatbestandsirrtum_excludes_intent() {
        assert!(Irrtum::Tatbestandsirrtum.excludes_intent());
        assert!(matches!(
            evaluate_mistake(Irrtum::Tatbestandsirrtum),
            Err(StgbError::Tatbestandsirrtum)
        ));
    }

    #[test]
    fn unavoidable_verbotsirrtum_excludes_culpability() {
        assert!(Irrtum::UnvermeidbarerVerbotsirrtum.excludes_culpability());
        assert!(matches!(
            evaluate_mistake(Irrtum::UnvermeidbarerVerbotsirrtum),
            Err(StgbError::UnvermeidbarerVerbotsirrtum)
        ));
    }

    #[test]
    fn avoidable_verbotsirrtum_leaves_liability() {
        assert!(Irrtum::VermeidbarerVerbotsirrtum.allows_mitigation());
        assert!(evaluate_mistake(Irrtum::VermeidbarerVerbotsirrtum).is_ok());
        assert!(evaluate_mistake(Irrtum::Keiner).is_ok());
    }

    #[test]
    fn erfolgsqualifikation_requires_at_least_negligence() {
        let eq = Erfolgsqualifikation {
            grunddelikt: "Körperverletzung".to_string(),
            schwere_folge: "Tod des Opfers".to_string(),
            schuldform_folge: Schuldform::UnbewussteFahrlaessigkeit,
        };
        assert!(check_erfolgsqualifikation(&eq).is_ok());

        let eq_no = Erfolgsqualifikation {
            schuldform_folge: Schuldform::Absicht,
            ..eq.clone()
        };
        assert!(check_erfolgsqualifikation(&eq_no).is_ok());
    }
}
