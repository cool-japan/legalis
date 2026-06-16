//! StGB §§ 32-35 - Justification and Excuse (Rechtfertigung und Entschuldigung)
//!
//! ## § 32 StGB - Notwehr (Self-defence) - justification
//!
//! > (1) Wer eine Tat begeht, die durch Notwehr geboten ist, handelt nicht
//! > rechtswidrig.
//! > (2) Notwehr ist die Verteidigung, die erforderlich ist, um einen
//! > gegenwärtigen rechtswidrigen Angriff von sich oder einem anderen abzuwenden.
//!
//! **Requirements** (Notwehrlage + Notwehrhandlung):
//! 1. A **present** (gegenwärtig), **unlawful** (rechtswidrig) **attack**
//!    (Angriff) on a legally protected interest of the defender or a third party
//!    (Nothilfe).
//! 2. Defence that is **necessary** (erforderlich): suitable and the mildest
//!    equally effective means.
//! 3. Defence that is **called for** (geboten): no abuse, no gross
//!    disproportion (sozialethische Einschränkungen).
//!
//! ## § 34 StGB - Rechtfertigender Notstand (Justifying necessity)
//!
//! > Wer in einer gegenwärtigen, nicht anders abwendbaren Gefahr für ein
//! > Rechtsgut eine Tat begeht, um die Gefahr abzuwenden, handelt nicht
//! > rechtswidrig, wenn bei Abwägung der widerstreitenden Interessen … das
//! > geschützte Interesse das beeinträchtigte wesentlich überwiegt. …
//!
//! **Requirements**: a present danger not otherwise avertable; the defensive act
//! averts it; the protected interest **substantially outweighs** (wesentlich
//! überwiegt) the interest impaired (Interessenabwägung); the act is an
//! appropriate means (angemessenes Mittel).
//!
//! ## § 35 StGB - Entschuldigender Notstand (Excusing necessity) - excuse
//!
//! > (1) Wer in einer gegenwärtigen, nicht anders abwendbaren Gefahr für Leben,
//! > Leib oder Freiheit eine rechtswidrige Tat begeht, um die Gefahr von sich,
//! > einem Angehörigen oder einer anderen ihm nahestehenden Person abzuwenden,
//! > handelt ohne Schuld. …
//!
//! Unlike § 34, § 35 is an **excuse** (Entschuldigungsgrund): the act remains
//! unlawful but the offender's culpability is excluded. It protects only **life,
//! limb or freedom** of the offender or persons close to him.

use serde::{Deserialize, Serialize};

use crate::stgb::error::{Result, StgbError};

/// Whether a defence is a justification (negates unlawfulness) or an excuse
/// (negates culpability).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefenceKind {
    /// Justification ground (Rechtfertigungsgrund): no unlawfulness.
    Rechtfertigung,
    /// Excuse (Entschuldigungsgrund): no culpability, act remains unlawful.
    Entschuldigung,
}

/// A self-defence situation (Notwehr, § 32 StGB).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Notwehr {
    /// Whether a present attack exists (gegenwärtiger Angriff).
    pub angriff_gegenwaertig: bool,
    /// Whether the attack is unlawful (rechtswidriger Angriff).
    pub angriff_rechtswidrig: bool,
    /// Whether the defensive measure is necessary (erforderlich): suitable and the
    /// mildest equally effective means.
    pub verteidigung_erforderlich: bool,
    /// Whether the defence is called for (geboten): no gross abuse / no
    /// socio-ethical restriction defeats it.
    pub verteidigung_geboten: bool,
}

/// Evaluate self-defence under § 32 StGB.
///
/// On success the act is justified (no unlawfulness). This is reported via the
/// dedicated error variant so callers in a liability chain treat justification as
/// barring liability.
///
/// # Errors
/// - [`StgbError::NotwehrlageFehlt`] if there is no present unlawful attack.
/// - [`StgbError::VerteidigungNichtErforderlich`] if the defence was not
///   necessary, or if it was not called for (geboten).
/// - [`StgbError::GerechtfertigtNotwehr`] if all requirements are met and the act
///   is justified.
pub fn evaluate_notwehr(n: &Notwehr) -> Result<()> {
    if !n.angriff_gegenwaertig || !n.angriff_rechtswidrig {
        return Err(StgbError::NotwehrlageFehlt);
    }
    if !n.verteidigung_erforderlich || !n.verteidigung_geboten {
        return Err(StgbError::VerteidigungNichtErforderlich);
    }
    Err(StgbError::GerechtfertigtNotwehr)
}

/// A justifying-necessity situation (rechtfertigender Notstand, § 34 StGB).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RechtfertigenderNotstand {
    /// Whether there is a present danger to a legal interest (gegenwärtige Gefahr).
    pub gefahr_gegenwaertig: bool,
    /// Whether the danger is not otherwise avertable (nicht anders abwendbar).
    pub nicht_anders_abwendbar: bool,
    /// Whether the protected interest substantially outweighs the impaired one
    /// (wesentliches Überwiegen, § 34 S. 1).
    pub geschuetztes_interesse_ueberwiegt_wesentlich: bool,
    /// Whether the act is an appropriate means to avert the danger (angemessenes
    /// Mittel, § 34 S. 2).
    pub angemessenes_mittel: bool,
}

/// Evaluate justifying necessity under § 34 StGB.
///
/// # Errors
/// - [`StgbError::InteresseUeberwiegtNicht`] if the present, not-otherwise-
///   avertable danger is missing, the balancing of interests is not clearly in
///   favour of the protected interest, or the means is not appropriate.
/// - [`StgbError::GerechtfertigtNotstand`] if all requirements are met and the
///   act is justified.
pub fn evaluate_rechtfertigender_notstand(n: &RechtfertigenderNotstand) -> Result<()> {
    if !n.gefahr_gegenwaertig
        || !n.nicht_anders_abwendbar
        || !n.geschuetztes_interesse_ueberwiegt_wesentlich
        || !n.angemessenes_mittel
    {
        return Err(StgbError::InteresseUeberwiegtNicht);
    }
    Err(StgbError::GerechtfertigtNotstand)
}

/// The class of legal interest endangered, decisive for § 35 StGB (which protects
/// only life, limb or freedom).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotstandsRechtsgut {
    /// Life (Leben).
    Leben,
    /// Limb / bodily integrity (Leib).
    Leib,
    /// Freedom (Freiheit).
    Freiheit,
    /// Any other interest (not covered by § 35).
    Sonstiges,
}

impl NotstandsRechtsgut {
    /// Whether this interest is protected by § 35 StGB.
    #[must_use]
    pub fn covered_by_35(&self) -> bool {
        matches!(
            self,
            NotstandsRechtsgut::Leben | NotstandsRechtsgut::Leib | NotstandsRechtsgut::Freiheit
        )
    }
}

/// An excusing-necessity situation (entschuldigender Notstand, § 35 StGB).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntschuldigenderNotstand {
    /// The endangered legal interest (must be life, limb or freedom).
    pub rechtsgut: NotstandsRechtsgut,
    /// Whether there is a present danger (gegenwärtige Gefahr).
    pub gefahr_gegenwaertig: bool,
    /// Whether the danger is not otherwise avertable (nicht anders abwendbar).
    pub nicht_anders_abwendbar: bool,
    /// Whether the endangered person is the offender, a relative or a person close
    /// to him (Angehöriger / nahestehende Person).
    pub nahestehende_person: bool,
    /// Whether the offender could be expected to tolerate the danger
    /// (Zumutbarkeit), which would bar the excuse (§ 35 Abs. 1 S. 2).
    pub gefahr_hinzunehmen_zumutbar: bool,
}

/// Evaluate excusing necessity under § 35 StGB.
///
/// # Errors
/// - [`StgbError::InteresseUeberwiegtNicht`] if § 35's requirements are not met
///   (interest not covered, danger absent/avertable, no qualifying person, or the
///   offender could be expected to tolerate the danger).
/// - [`StgbError::EntschuldigtNotstand`] if all requirements are met and the act
///   is excused (culpability excluded).
pub fn evaluate_entschuldigender_notstand(n: &EntschuldigenderNotstand) -> Result<()> {
    if !n.rechtsgut.covered_by_35()
        || !n.gefahr_gegenwaertig
        || !n.nicht_anders_abwendbar
        || !n.nahestehende_person
        || n.gefahr_hinzunehmen_zumutbar
    {
        return Err(StgbError::InteresseUeberwiegtNicht);
    }
    Err(StgbError::EntschuldigtNotstand)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_notwehr_justifies() {
        let n = Notwehr {
            angriff_gegenwaertig: true,
            angriff_rechtswidrig: true,
            verteidigung_erforderlich: true,
            verteidigung_geboten: true,
        };
        assert!(matches!(
            evaluate_notwehr(&n),
            Err(StgbError::GerechtfertigtNotwehr)
        ));
    }

    #[test]
    fn notwehr_without_present_attack_fails() {
        let n = Notwehr {
            angriff_gegenwaertig: false,
            angriff_rechtswidrig: true,
            verteidigung_erforderlich: true,
            verteidigung_geboten: true,
        };
        assert!(matches!(
            evaluate_notwehr(&n),
            Err(StgbError::NotwehrlageFehlt)
        ));
    }

    #[test]
    fn notwehr_excessive_defence_fails() {
        let n = Notwehr {
            angriff_gegenwaertig: true,
            angriff_rechtswidrig: true,
            verteidigung_erforderlich: false,
            verteidigung_geboten: true,
        };
        assert!(matches!(
            evaluate_notwehr(&n),
            Err(StgbError::VerteidigungNichtErforderlich)
        ));
    }

    #[test]
    fn valid_rechtfertigender_notstand_justifies() {
        let n = RechtfertigenderNotstand {
            gefahr_gegenwaertig: true,
            nicht_anders_abwendbar: true,
            geschuetztes_interesse_ueberwiegt_wesentlich: true,
            angemessenes_mittel: true,
        };
        assert!(matches!(
            evaluate_rechtfertigender_notstand(&n),
            Err(StgbError::GerechtfertigtNotstand)
        ));
    }

    #[test]
    fn notstand_without_overweight_fails() {
        let n = RechtfertigenderNotstand {
            gefahr_gegenwaertig: true,
            nicht_anders_abwendbar: true,
            geschuetztes_interesse_ueberwiegt_wesentlich: false,
            angemessenes_mittel: true,
        };
        assert!(matches!(
            evaluate_rechtfertigender_notstand(&n),
            Err(StgbError::InteresseUeberwiegtNicht)
        ));
    }

    #[test]
    fn rechtsgut_coverage_for_35() {
        assert!(NotstandsRechtsgut::Leben.covered_by_35());
        assert!(NotstandsRechtsgut::Freiheit.covered_by_35());
        assert!(!NotstandsRechtsgut::Sonstiges.covered_by_35());
    }

    #[test]
    fn valid_entschuldigender_notstand_excuses() {
        let n = EntschuldigenderNotstand {
            rechtsgut: NotstandsRechtsgut::Leben,
            gefahr_gegenwaertig: true,
            nicht_anders_abwendbar: true,
            nahestehende_person: true,
            gefahr_hinzunehmen_zumutbar: false,
        };
        assert!(matches!(
            evaluate_entschuldigender_notstand(&n),
            Err(StgbError::EntschuldigtNotstand)
        ));
    }

    #[test]
    fn entschuldigender_notstand_property_not_covered() {
        let n = EntschuldigenderNotstand {
            rechtsgut: NotstandsRechtsgut::Sonstiges,
            gefahr_gegenwaertig: true,
            nicht_anders_abwendbar: true,
            nahestehende_person: true,
            gefahr_hinzunehmen_zumutbar: false,
        };
        assert!(matches!(
            evaluate_entschuldigender_notstand(&n),
            Err(StgbError::InteresseUeberwiegtNicht)
        ));
    }

    #[test]
    fn entschuldigender_notstand_zumutbar_bars_excuse() {
        let n = EntschuldigenderNotstand {
            rechtsgut: NotstandsRechtsgut::Leib,
            gefahr_gegenwaertig: true,
            nicht_anders_abwendbar: true,
            nahestehende_person: true,
            gefahr_hinzunehmen_zumutbar: true,
        };
        assert!(matches!(
            evaluate_entschuldigender_notstand(&n),
            Err(StgbError::InteresseUeberwiegtNicht)
        ));
    }

    #[test]
    fn defence_kind_distinguishes() {
        assert_ne!(DefenceKind::Rechtfertigung, DefenceKind::Entschuldigung);
    }
}
