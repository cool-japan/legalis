//! StGB §§ 19-21 - Criminal Capacity (Schuldfähigkeit)
//!
//! ## § 19 StGB - Schuldunfähigkeit des Kindes (Incapacity of children)
//!
//! > Schuldunfähig ist, wer bei Begehung der Tat noch nicht vierzehn Jahre alt ist.
//!
//! **English**: A person who has not yet reached the age of fourteen at the time
//! of the offence is incapable of guilt (absolute incapacity).
//!
//! ## § 20 StGB - Schuldunfähigkeit wegen seelischer Störungen
//!
//! > Ohne Schuld handelt, wer bei Begehung der Tat wegen einer krankhaften
//! > seelischen Störung, wegen einer tiefgreifenden Bewusstseinsstörung oder wegen
//! > Intelligenzminderung oder einer schweren anderen seelischen Störung unfähig
//! > ist, das Unrecht der Tat einzusehen oder nach dieser Einsicht zu handeln.
//!
//! **English**: A person acts without guilt if, at the time of the offence, due
//! to a pathological mental disorder, a profound disturbance of consciousness,
//! intellectual disability or a severe other mental disorder, he is **incapable
//! of appreciating the wrongfulness** of the act (Einsichtsfähigkeit) or of
//! **acting in accordance with that appreciation** (Steuerungsfähigkeit).
//!
//! ## § 21 StGB - Verminderte Schuldfähigkeit (Diminished capacity)
//!
//! > Ist die Fähigkeit des Täters, das Unrecht der Tat einzusehen oder nach dieser
//! > Einsicht zu handeln, aus einem der in § 20 bezeichneten Gründe bei Begehung
//! > der Tat erheblich vermindert, so kann die Strafe nach § 49 Abs. 1 gemildert
//! > werden.
//!
//! **English**: If the offender's capacity is **substantially diminished**
//! (erheblich vermindert) for one of the reasons in § 20, the sentence **may** be
//! mitigated (§ 49 Abs. 1 StGB).
//!
//! Note: criminal responsibility of 14-17 year-olds (and, under conditions,
//! 18-20 year-olds) is governed by the separate Jugendgerichtsgesetz (JGG); this
//! module models only the StGB capacity rules.

use serde::{Deserialize, Serialize};

use crate::stgb::error::{Result, StgbError};

/// Minimum age of criminal capacity (Strafmündigkeit), § 19 StGB: 14 years.
pub const STRAFMUENDIGKEIT_ALTER: u32 = 14;

/// The biological/psychological basis under § 20 StGB (Eingangsmerkmale).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeelischeStoerung {
    /// Pathological mental disorder (krankhafte seelische Störung).
    KrankhafteStoerung,
    /// Profound disturbance of consciousness (tiefgreifende Bewusstseinsstörung).
    Bewusstseinsstoerung,
    /// Intellectual disability (Intelligenzminderung).
    Intelligenzminderung,
    /// Severe other mental disorder (schwere andere seelische Störung).
    SchwereAndereStoerung,
}

/// Degree to which capacity is affected at the time of the offence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapacityImpact {
    /// Capacity fully present (uneingeschränkt schuldfähig).
    Voll,
    /// Capacity substantially diminished (erheblich vermindert, § 21 StGB).
    ErheblichVermindert,
    /// Capacity excluded (aufgehoben, § 20 StGB).
    Aufgehoben,
}

/// The legal consequence for an offender's culpability (Schuld).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapacityResult {
    /// Fully culpable (voll schuldfähig).
    SchuldfaehigVoll,
    /// Culpable, but the sentence may be mitigated (§ 21 / § 49 Abs. 1 StGB).
    VermindertSchuldfaehig,
    /// Not culpable (schuldunfähig); the act is not punishable.
    Schuldunfaehig,
}

/// Assessment of an offender's capacity (§§ 19-21 StGB).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapacityAssessment {
    /// Age of the offender (in completed years) at the time of the offence.
    pub age_years: u32,
    /// Underlying mental disorder, if any (§ 20 Eingangsmerkmal).
    pub stoerung: Option<SeelischeStoerung>,
    /// Whether the offender lacked the capacity to appreciate wrongfulness or to
    /// act accordingly, and to what degree.
    pub impact: CapacityImpact,
}

impl CapacityAssessment {
    /// Whether the offender is a child below the age of criminal capacity
    /// (§ 19 StGB).
    #[must_use]
    pub fn is_child(&self) -> bool {
        self.age_years < STRAFMUENDIGKEIT_ALTER
    }
}

/// Determine criminal capacity under §§ 19-21 StGB.
///
/// Order of analysis:
/// 1. **§ 19 StGB**: a child under 14 is always incapable of guilt.
/// 2. **§ 20 StGB**: capacity excluded due to a qualifying mental disorder.
/// 3. **§ 21 StGB**: capacity substantially diminished (sentence may be mitigated).
/// 4. Otherwise fully culpable.
///
/// # Errors
/// - [`StgbError::Schuldunfaehig19Kind`] if the offender is under 14 (§ 19 StGB).
/// - [`StgbError::Schuldunfaehig20`] if capacity is excluded under § 20 StGB.
///
/// Diminished capacity (§ 21 StGB) and full capacity both return `Ok` with the
/// corresponding [`CapacityResult`].
pub fn assess_capacity(a: &CapacityAssessment) -> Result<CapacityResult> {
    // § 19 StGB - absolute incapacity of children.
    if a.is_child() {
        return Err(StgbError::Schuldunfaehig19Kind);
    }

    match (a.impact, a.stoerung) {
        // § 20 StGB - capacity excluded; requires a qualifying disorder.
        (CapacityImpact::Aufgehoben, Some(_)) => Err(StgbError::Schuldunfaehig20),
        // § 21 StGB - substantially diminished capacity (sentence may be mitigated).
        (CapacityImpact::ErheblichVermindert, Some(_)) => {
            Ok(CapacityResult::VermindertSchuldfaehig)
        }
        // Without a qualifying § 20 disorder, capacity is unaffected even if the
        // offender claims impairment.
        _ => Ok(CapacityResult::SchuldfaehigVoll),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn child_under_14_is_incapable() {
        let a = CapacityAssessment {
            age_years: 13,
            stoerung: None,
            impact: CapacityImpact::Voll,
        };
        assert!(a.is_child());
        assert!(matches!(
            assess_capacity(&a),
            Err(StgbError::Schuldunfaehig19Kind)
        ));
    }

    #[test]
    fn exactly_14_is_capable() {
        let a = CapacityAssessment {
            age_years: 14,
            stoerung: None,
            impact: CapacityImpact::Voll,
        };
        assert!(!a.is_child());
        assert_eq!(
            assess_capacity(&a).expect("capable"),
            CapacityResult::SchuldfaehigVoll
        );
    }

    #[test]
    fn section_20_excludes_capacity() {
        let a = CapacityAssessment {
            age_years: 30,
            stoerung: Some(SeelischeStoerung::KrankhafteStoerung),
            impact: CapacityImpact::Aufgehoben,
        };
        assert!(matches!(
            assess_capacity(&a),
            Err(StgbError::Schuldunfaehig20)
        ));
    }

    #[test]
    fn section_21_diminishes_capacity() {
        let a = CapacityAssessment {
            age_years: 30,
            stoerung: Some(SeelischeStoerung::Bewusstseinsstoerung),
            impact: CapacityImpact::ErheblichVermindert,
        };
        assert_eq!(
            assess_capacity(&a).expect("diminished"),
            CapacityResult::VermindertSchuldfaehig
        );
    }

    #[test]
    fn impairment_without_disorder_keeps_full_capacity() {
        let a = CapacityAssessment {
            age_years: 40,
            stoerung: None,
            impact: CapacityImpact::Aufgehoben,
        };
        assert_eq!(
            assess_capacity(&a).expect("capable"),
            CapacityResult::SchuldfaehigVoll
        );
    }
}
