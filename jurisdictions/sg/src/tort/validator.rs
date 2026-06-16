//! Tort Law - Validation and Analysis Logic
//!
//! Functions applying Singapore tort law to the typed models:
//!
//! - [`assess_negligence`] — works through duty, breach, causation and damage
//!   in order, surfacing the specific [`TortError`] for the first element that
//!   fails, or [`TortError::NegligenceEstablished`] when all are made out.
//! - [`assess_defamation`] — tests the core ingredients, per-se actionability /
//!   special damage, and the pleaded defences.
//! - [`assess_private_nuisance`], [`assess_public_nuisance`],
//!   [`assess_occupiers_liability`] — the other torts.
//! - [`apportion_for_contributory_negligence`] — reduces an award under the
//!   Contributory Negligence and Personal Injuries Act 1953.

use super::error::{Result, TortError};
use super::nuisance::{
    OccupiersLiabilityClaim, PrivateNuisanceClaim, PublicNuisanceClaim, TortDefence,
};
use super::types::*;
use serde::{Deserialize, Serialize};

/// Assesses a negligence claim element by element.
///
/// # Errors
///
/// Returns [`TortError::NoDutyOfCare`], [`TortError::NoBreach`],
/// [`TortError::NoFactualCausation`], [`TortError::NovusActusInterveniens`],
/// [`TortError::RemoteDamage`] or [`TortError::ValidationError`] (no damage) for
/// the first element that fails. When every element is satisfied it returns
/// [`TortError::NegligenceEstablished`], so callers should treat the `Ok`/`Err`
/// branch deliberately; see [`negligence_succeeds`] for a boolean helper.
pub fn assess_negligence(claim: &NegligenceClaim) -> Result<()> {
    // 1. Duty of care (Spandeck two-stage test).
    if !claim.duty.factual_foreseeability {
        return Err(TortError::NoDutyOfCare {
            reason: "harm to the claimant was not reasonably foreseeable (threshold not met)"
                .to_string(),
        });
    }
    if !claim.duty.legal_proximity {
        return Err(TortError::NoDutyOfCare {
            reason: "insufficient legal proximity between the parties (Spandeck stage 1)"
                .to_string(),
        });
    }
    if claim.duty.policy_negates_duty {
        return Err(TortError::NoDutyOfCare {
            reason: "policy considerations negate the duty (Spandeck stage 2)".to_string(),
        });
    }

    // 2. Breach of the standard of care.
    if !claim.breach.is_breach() {
        return Err(TortError::NoBreach {
            reason: format!(
                "the defendant met the applicable standard ({})",
                claim.breach.standard.authority()
            ),
        });
    }

    // 3. Causation: factual, intervening act, then legal remoteness.
    if !claim.causation.but_for_satisfied {
        return Err(TortError::NoFactualCausation {
            reason: "the loss would have occurred even without the breach".to_string(),
        });
    }
    if claim.causation.novus_actus {
        return Err(TortError::NovusActusInterveniens {
            detail: "an intervening act broke the chain of causation".to_string(),
        });
    }
    if !claim.causation.damage_kind_foreseeable {
        return Err(TortError::RemoteDamage {
            reason: "the kind of damage was not reasonably foreseeable".to_string(),
        });
    }

    // 4. Actionable damage.
    if claim.damage_cents == 0 {
        return Err(TortError::ValidationError {
            message: "negligence is not actionable without proof of damage".to_string(),
        });
    }

    Err(TortError::NegligenceEstablished {
        detail: format!(
            "duty, breach, causation and damage of SGD {:.2} all established",
            claim.damage_cents as f64 / 100.0
        ),
    })
}

/// Returns whether a negligence claim is made out (a convenience over
/// [`assess_negligence`]).
pub fn negligence_succeeds(claim: &NegligenceClaim) -> bool {
    matches!(
        assess_negligence(claim),
        Err(TortError::NegligenceEstablished { .. })
    )
}

/// Assesses a defamation claim: core ingredients, actionability, and defences.
///
/// # Errors
///
/// Returns [`TortError::ValidationError`] where a core ingredient is missing or
/// (for slander) special damage is required but not proved;
/// [`TortError::DefenceSucceeds`] where a complete defence is made out; and
/// [`TortError::Libel`] / [`TortError::Slander`] where the claim is established.
pub fn assess_defamation(claim: &DefamationClaim) -> Result<()> {
    // 1. Core ingredients: defamatory meaning, reference, publication.
    if !claim.defamatory_meaning {
        return Err(TortError::ValidationError {
            message: "the statement does not bear a defamatory meaning".to_string(),
        });
    }
    if !claim.refers_to_claimant {
        return Err(TortError::ValidationError {
            message: "the statement does not refer to the claimant".to_string(),
        });
    }
    if !claim.published_to_third_party {
        return Err(TortError::ValidationError {
            message: "the statement was not published to a third party".to_string(),
        });
    }

    // 2. Actionability: libel per se; slander needs an exception or special
    //    damage.
    if matches!(claim.form, DefamationForm::Slander)
        && !claim.actionable_per_se()
        && !claim.special_damage_proved
    {
        return Err(TortError::ValidationError {
            message: "slander requires proof of special damage outside the ss. 5–6 exceptions"
                .to_string(),
        });
    }

    // 3. Defences. Qualified privilege and fair comment are defeated by malice;
    //    the others (justification, absolute privilege, offer of amends) stand.
    for defence in &claim.defences {
        let defeated_by_malice = matches!(
            defence,
            DefamationDefence::QualifiedPrivilege | DefamationDefence::FairComment
        ) && claim.malice_proved;
        if defence.is_complete_defence() && !defeated_by_malice {
            return Err(TortError::DefenceSucceeds {
                defence: format!("{defence:?}"),
                detail: defence.authority().to_string(),
            });
        }
    }

    // 4. Established.
    match claim.form {
        DefamationForm::Libel => Err(TortError::Libel {
            statement: claim.statement.clone(),
        }),
        DefamationForm::Slander => {
            let basis = claim
                .slander_exception
                .map(|e| e.basis().to_string())
                .unwrap_or_else(|| "special damage proved".to_string());
            Err(TortError::Slander {
                statement: claim.statement.clone(),
                basis,
            })
        }
    }
}

/// Returns whether a defamation claim succeeds (no successful complete defence
/// and the ingredients/actionability are met).
pub fn defamation_succeeds(claim: &DefamationClaim) -> bool {
    matches!(
        assess_defamation(claim),
        Err(TortError::Libel { .. }) | Err(TortError::Slander { .. })
    )
}

/// Assesses a private-nuisance claim.
///
/// # Errors
///
/// Returns [`TortError::ValidationError`] where standing or the substantial/
/// unreasonable requirement fails, otherwise [`TortError::PrivateNuisance`].
pub fn assess_private_nuisance(claim: &PrivateNuisanceClaim) -> Result<()> {
    if !claim.has_proprietary_interest {
        return Err(TortError::ValidationError {
            message: "the claimant lacks a proprietary interest in the affected land (Hunter v Canary Wharf [1997] AC 655)".to_string(),
        });
    }
    if claim.is_actionable() {
        return Err(TortError::PrivateNuisance {
            detail: format!("{:?} interference", claim.interference),
        });
    }
    Err(TortError::ValidationError {
        message: "the interference is not both substantial and unreasonable".to_string(),
    })
}

/// Assesses a public-nuisance claim brought by a private claimant.
///
/// # Errors
///
/// Returns [`TortError::ValidationError`] where a class of the public is not
/// affected or special damage is not proved, otherwise
/// [`TortError::PublicNuisance`].
pub fn assess_public_nuisance(claim: &PublicNuisanceClaim) -> Result<()> {
    if !claim.affects_class_of_public {
        return Err(TortError::ValidationError {
            message: "the act does not affect a class of the public".to_string(),
        });
    }
    if !claim.special_damage {
        return Err(TortError::ValidationError {
            message: "a private claimant must prove special damage beyond that suffered by the public generally (Tate & Lyle v GLC [1983] 2 AC 509)".to_string(),
        });
    }
    Err(TortError::PublicNuisance {
        detail: claim.description.clone(),
    })
}

/// Assesses an occupiers'-liability claim.
///
/// # Errors
///
/// Returns [`TortError::ValidationError`] where the duty was discharged (e.g. by
/// an adequate warning) or not breached, otherwise
/// [`TortError::OccupiersLiability`].
pub fn assess_occupiers_liability(claim: &OccupiersLiabilityClaim) -> Result<()> {
    if claim.is_liable() {
        return Err(TortError::OccupiersLiability {
            visitor_kind: claim.status.label().to_string(),
            detail: claim.danger.clone(),
        });
    }
    let reason = if claim.adequate_warning_given {
        "the occupier gave an adequate warning that enabled the entrant to be reasonably safe"
    } else if claim.independent_contractor_defence {
        "the danger was due to an independent contractor and the occupier acted reasonably"
    } else {
        "the occupier took the care required for that entrant"
    };
    Err(TortError::ValidationError {
        message: reason.to_string(),
    })
}

/// Applies contributory negligence to an award, reducing it by the claimant's
/// share of fault under the Contributory Negligence and Personal Injuries Act
/// 1953, s. 3.
///
/// # Errors
///
/// Returns [`TortError::ValidationError`] if the fault percentage exceeds 100.
pub fn apportion_for_contributory_negligence(
    full_award_cents: u64,
    claimant_fault_percent: u8,
) -> Result<u64> {
    if claimant_fault_percent > 100 {
        return Err(TortError::ValidationError {
            message: "claimant fault percentage cannot exceed 100".to_string(),
        });
    }
    let defendant_share = 100u64 - claimant_fault_percent as u64;
    // Integer arithmetic on cents; rounds down.
    Ok(full_award_cents * defendant_share / 100)
}

/// The category of tort a report concerns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TortCategory {
    /// Negligence.
    Negligence,
    /// Defamation.
    Defamation,
    /// Private nuisance.
    PrivateNuisance,
    /// Public nuisance.
    PublicNuisance,
    /// Occupiers' liability.
    OccupiersLiability,
}

/// A consolidated report on a tort claim.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TortAssessmentReport {
    /// Identifier of the claim assessed.
    pub claim_id: String,
    /// The tort category.
    pub category: TortCategory,
    /// Whether liability is established.
    pub liability_established: bool,
    /// The outcome as a display string (the established-liability message, the
    /// failed element, or the successful defence).
    pub outcome: String,
    /// Any general defences that, if proved, would defeat or reduce the claim.
    pub applicable_defences: Vec<String>,
}

impl TortAssessmentReport {
    /// Builds a report for a negligence claim, taking account of any general
    /// defences (a complete defence overrides an otherwise-established claim;
    /// contributory negligence is recorded but does not defeat liability).
    pub fn for_negligence(claim: &NegligenceClaim, defences: &[TortDefence]) -> Self {
        let established = negligence_succeeds(claim);
        let complete_defence = defences.iter().find(|d| d.is_complete_defence());

        let (liability_established, outcome) = match (established, complete_defence) {
            (true, Some(defence)) => (
                false,
                format!("liability would be established but is defeated by {defence:?}"),
            ),
            (true, None) => (
                true,
                assess_negligence(claim)
                    .err()
                    .map(|e| e.to_string())
                    .unwrap_or_default(),
            ),
            (false, _) => (
                false,
                assess_negligence(claim)
                    .err()
                    .map(|e| e.to_string())
                    .unwrap_or_default(),
            ),
        };

        TortAssessmentReport {
            claim_id: claim.id.clone(),
            category: TortCategory::Negligence,
            liability_established,
            outcome,
            applicable_defences: defences.iter().map(|d| format!("{d:?}")).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tort::nuisance::{EntrantStatus, InterferenceKind};

    fn established_negligence() -> NegligenceClaim {
        NegligenceClaim::new(
            "n1",
            "Plaintiff",
            "Defendant",
            DutyOfCareAnalysis::established(HarmCategory::PersonalInjury),
            BreachAnalysis::new(StandardOfCare::ReasonablePerson, true),
            CausationAnalysis::established(),
            1_000_000,
        )
    }

    #[test]
    fn negligence_made_out() {
        let claim = established_negligence();
        assert!(negligence_succeeds(&claim));
        match assess_negligence(&claim) {
            Err(TortError::NegligenceEstablished { .. }) => {}
            other => panic!("expected established, got {other:?}"),
        }
    }

    #[test]
    fn no_duty_when_policy_negates() {
        let mut claim = established_negligence();
        claim.duty = claim.duty.with_policy_negation();
        match assess_negligence(&claim) {
            Err(TortError::NoDutyOfCare { .. }) => {}
            other => panic!("expected NoDutyOfCare, got {other:?}"),
        }
    }

    #[test]
    fn no_breach_surfaces_breach_error() {
        let mut claim = established_negligence();
        claim.breach = BreachAnalysis::new(StandardOfCare::Professional, false);
        match assess_negligence(&claim) {
            Err(TortError::NoBreach { .. }) => {}
            other => panic!("expected NoBreach, got {other:?}"),
        }
    }

    #[test]
    fn but_for_failure_surfaces_factual_causation_error() {
        let mut claim = established_negligence();
        claim.causation = CausationAnalysis::established().with_but_for(false);
        match assess_negligence(&claim) {
            Err(TortError::NoFactualCausation { .. }) => {}
            other => panic!("expected NoFactualCausation, got {other:?}"),
        }
    }

    #[test]
    fn novus_actus_surfaces_its_error() {
        let mut claim = established_negligence();
        claim.causation = CausationAnalysis::established().with_novus_actus();
        match assess_negligence(&claim) {
            Err(TortError::NovusActusInterveniens { .. }) => {}
            other => panic!("expected novus actus, got {other:?}"),
        }
    }

    #[test]
    fn libel_succeeds_without_defence() {
        let claim = DefamationClaim::new("d1", "P", "D", "P is corrupt", DefamationForm::Libel);
        assert!(defamation_succeeds(&claim));
    }

    #[test]
    fn justification_defeats_defamation() {
        let mut claim = DefamationClaim::new("d2", "P", "D", "P is corrupt", DefamationForm::Libel);
        claim.add_defence(DefamationDefence::Justification);
        match assess_defamation(&claim) {
            Err(TortError::DefenceSucceeds { .. }) => {}
            other => panic!("expected defence, got {other:?}"),
        }
        assert!(!defamation_succeeds(&claim));
    }

    #[test]
    fn qualified_privilege_defeated_by_malice() {
        let mut claim = DefamationClaim::new("d3", "P", "D", "P is corrupt", DefamationForm::Libel)
            .with_malice();
        claim.add_defence(DefamationDefence::QualifiedPrivilege);
        // Malice defeats qualified privilege, so the claim succeeds.
        assert!(defamation_succeeds(&claim));
    }

    #[test]
    fn slander_without_exception_or_damage_is_not_actionable() {
        let claim =
            DefamationClaim::new("d4", "P", "D", "P is unpleasant", DefamationForm::Slander);
        match assess_defamation(&claim) {
            Err(TortError::ValidationError { .. }) => {}
            other => panic!("expected validation error, got {other:?}"),
        }
    }

    #[test]
    fn private_nuisance_established() {
        let claim = PrivateNuisanceClaim::new("p1", "Owner", "Factory", InterferenceKind::Noise);
        match assess_private_nuisance(&claim) {
            Err(TortError::PrivateNuisance { .. }) => {}
            other => panic!("expected nuisance, got {other:?}"),
        }
    }

    #[test]
    fn public_nuisance_needs_special_damage() {
        let claim = PublicNuisanceClaim::new("pn1", "Trader", "D", "obstruction");
        assert!(matches!(
            assess_public_nuisance(&claim),
            Err(TortError::ValidationError { .. })
        ));
        let with_damage = claim.with_special_damage();
        assert!(matches!(
            assess_public_nuisance(&with_damage),
            Err(TortError::PublicNuisance { .. })
        ));
    }

    #[test]
    fn occupier_liability_and_warning_defence() {
        let claim = OccupiersLiabilityClaim::new(
            "o1",
            "Visitor",
            "Shop",
            EntrantStatus::LawfulVisitor,
            "wet floor",
        );
        assert!(matches!(
            assess_occupiers_liability(&claim),
            Err(TortError::OccupiersLiability { .. })
        ));
        let warned = claim.with_adequate_warning();
        assert!(matches!(
            assess_occupiers_liability(&warned),
            Err(TortError::ValidationError { .. })
        ));
    }

    #[test]
    fn contributory_negligence_reduces_award() {
        let reduced = apportion_for_contributory_negligence(1_000_000, 25).expect("apportion");
        assert_eq!(reduced, 750_000);
    }

    #[test]
    fn contributory_negligence_rejects_over_100() {
        assert!(apportion_for_contributory_negligence(1_000_000, 101).is_err());
    }

    #[test]
    fn report_records_complete_defence() {
        let claim = established_negligence();
        let report =
            TortAssessmentReport::for_negligence(&claim, &[TortDefence::VolentiNonFitInjuria]);
        assert!(!report.liability_established);
        assert!(report.outcome.contains("defeated"));
    }

    #[test]
    fn report_records_established_with_contributory_negligence() {
        let claim = established_negligence();
        let report = TortAssessmentReport::for_negligence(
            &claim,
            &[TortDefence::ContributoryNegligence {
                claimant_fault_percent: 30,
            }],
        );
        // Contributory negligence apportions but does not defeat liability.
        assert!(report.liability_established);
    }
}
