//! Integration tests for the Singapore tort-law module.
//!
//! These exercise the public API of [`legalis_sg::tort`] across negligence,
//! defamation, nuisance and occupiers' liability, applying the leading Singapore
//! and English authorities.

use legalis_sg::tort::*;

/// A road-traffic negligence claim with all four elements is made out under the
/// *Spandeck* framework.
#[test]
fn road_traffic_negligence_is_established() {
    let claim = NegligenceClaim::new(
        "rt-1",
        "Injured Cyclist",
        "Negligent Motorist",
        DutyOfCareAnalysis::established(HarmCategory::PersonalInjury),
        BreachAnalysis::new(StandardOfCare::ReasonablePerson, true).with_risk_factors(70, 90, 5),
        CausationAnalysis::established(),
        3_500_000, // SGD 35,000
    );
    assert!(negligence_succeeds(&claim));
}

/// Pure economic loss: where policy negates the duty, no claim arises even on
/// foreseeable loss.
#[test]
fn pure_economic_loss_can_be_negated_by_policy() {
    let claim = NegligenceClaim::new(
        "pel-1",
        "Disappointed Investor",
        "Auditor",
        DutyOfCareAnalysis::established(HarmCategory::PureEconomicLoss).with_policy_negation(),
        BreachAnalysis::new(StandardOfCare::Professional, true),
        CausationAnalysis::established(),
        10_000_000,
    );
    match assess_negligence(&claim) {
        Err(TortError::NoDutyOfCare { .. }) => {}
        other => panic!("expected no duty, got {other:?}"),
    }
}

/// "But for" causation: if the loss would have occurred anyway, the claim fails
/// (*Barnett v Chelsea & Kensington Hospital*).
#[test]
fn but_for_causation_defeats_claim() {
    let claim = NegligenceClaim::new(
        "bf-1",
        "Patient Estate",
        "Hospital",
        DutyOfCareAnalysis::established(HarmCategory::PersonalInjury),
        BreachAnalysis::new(StandardOfCare::Professional, true),
        CausationAnalysis::established().with_but_for(false),
        5_000_000,
    );
    match assess_negligence(&claim) {
        Err(TortError::NoFactualCausation { .. }) => {}
        other => panic!("expected no factual causation, got {other:?}"),
    }
}

/// Contributory negligence reduces, but does not defeat, an established claim.
#[test]
fn contributory_negligence_reduces_award() {
    let claim = NegligenceClaim::new(
        "cn-1",
        "Pedestrian",
        "Driver",
        DutyOfCareAnalysis::established(HarmCategory::PersonalInjury),
        BreachAnalysis::new(StandardOfCare::ReasonablePerson, true),
        CausationAnalysis::established(),
        4_000_000,
    );
    assert!(negligence_succeeds(&claim));

    // Pedestrian was 40% to blame for crossing against the signal.
    let reduced = apportion_for_contributory_negligence(4_000_000, 40).expect("apportion");
    assert_eq!(reduced, 2_400_000); // SGD 24,000

    let report = TortAssessmentReport::for_negligence(
        &claim,
        &[TortDefence::ContributoryNegligence {
            claimant_fault_percent: 40,
        }],
    );
    assert!(report.liability_established);
}

/// Volenti is a complete defence that defeats an otherwise-established claim.
#[test]
fn volenti_defeats_established_negligence() {
    let claim = NegligenceClaim::new(
        "v-1",
        "Willing Participant",
        "Organiser",
        DutyOfCareAnalysis::established(HarmCategory::PersonalInjury),
        BreachAnalysis::new(StandardOfCare::ReasonablePerson, true),
        CausationAnalysis::established(),
        2_000_000,
    );
    let report = TortAssessmentReport::for_negligence(&claim, &[TortDefence::VolentiNonFitInjuria]);
    assert!(!report.liability_established);
}

/// Libel is actionable per se; a successful plea of justification is a complete
/// defence.
#[test]
fn libel_and_justification() {
    let claim = DefamationClaim::new(
        "lib-1",
        "Public Figure",
        "Newspaper",
        "the claimant misappropriated charity funds",
        DefamationForm::Libel,
    );
    assert!(defamation_succeeds(&claim));

    let mut with_truth = claim;
    with_truth.add_defence(DefamationDefence::Justification);
    assert!(!defamation_succeeds(&with_truth));
}

/// Slander is not actionable per se outside the statutory exceptions; an
/// imputation of a crime (s. 5) is.
#[test]
fn slander_per_se_only_within_exception() {
    let bare = DefamationClaim::new(
        "sl-1",
        "Neighbour",
        "Gossip",
        "the claimant is an unpleasant person",
        DefamationForm::Slander,
    );
    match assess_defamation(&bare) {
        Err(TortError::ValidationError { .. }) => {}
        other => panic!("expected not actionable, got {other:?}"),
    }

    let criminal_imputation = DefamationClaim::new(
        "sl-2",
        "Shopkeeper",
        "Customer",
        "the claimant stole from the cash register",
        DefamationForm::Slander,
    )
    .with_slander_exception(SlanderPerSeException::CriminalOffence);
    assert!(defamation_succeeds(&criminal_imputation));
}

/// Qualified privilege is defeated by proof of malice.
#[test]
fn qualified_privilege_defeated_by_malice() {
    let mut claim = DefamationClaim::new(
        "qp-1",
        "Job Applicant",
        "Former Employer",
        "the applicant was dishonest",
        DefamationForm::Libel,
    )
    .with_malice();
    claim.add_defence(DefamationDefence::QualifiedPrivilege);
    // Malice destroys the privilege, so the claim succeeds.
    assert!(defamation_succeeds(&claim));
}

/// Private nuisance requires standing and a substantial, unreasonable
/// interference.
#[test]
fn private_nuisance_requires_standing_and_unreasonableness() {
    let claim = PrivateNuisanceClaim::new("pn-1", "Resident", "Nightclub", InterferenceKind::Noise);
    match assess_private_nuisance(&claim) {
        Err(TortError::PrivateNuisance { .. }) => {}
        other => panic!("expected private nuisance, got {other:?}"),
    }

    let no_standing = claim.with_proprietary_interest(false);
    assert!(matches!(
        assess_private_nuisance(&no_standing),
        Err(TortError::ValidationError { .. })
    ));
}

/// Public nuisance is actionable by a private claimant only on special damage.
#[test]
fn public_nuisance_needs_special_damage() {
    let claim = PublicNuisanceClaim::new(
        "pub-1",
        "Affected Trader",
        "Construction Firm",
        "obstruction of a public road for months",
    );
    assert!(matches!(
        assess_public_nuisance(&claim),
        Err(TortError::ValidationError { .. })
    ));

    let with_special_loss = claim.with_special_damage();
    assert!(matches!(
        assess_public_nuisance(&with_special_loss),
        Err(TortError::PublicNuisance { .. })
    ));
}

/// Occupiers' liability: a duty to a lawful visitor, discharged by an adequate
/// warning.
#[test]
fn occupiers_liability_and_warning_defence() {
    let claim = OccupiersLiabilityClaim::new(
        "ol-1",
        "Customer",
        "Shopping Mall",
        EntrantStatus::LawfulVisitor,
        "freshly mopped floor with no signage",
    );
    match assess_occupiers_liability(&claim) {
        Err(TortError::OccupiersLiability { visitor_kind, .. }) => {
            assert_eq!(visitor_kind, "lawful visitor");
        }
        other => panic!("expected occupiers liability, got {other:?}"),
    }

    let warned = claim.with_adequate_warning();
    assert!(matches!(
        assess_occupiers_liability(&warned),
        Err(TortError::ValidationError { .. })
    ));
}

/// Serialization roundtrips for the negligence claim.
#[test]
fn negligence_claim_serde_roundtrip() {
    let claim = NegligenceClaim::new(
        "ser-1",
        "P",
        "D",
        DutyOfCareAnalysis::established(HarmCategory::PropertyDamage),
        BreachAnalysis::new(StandardOfCare::ReasonablePerson, true),
        CausationAnalysis::established(),
        1_000_000,
    );
    let json = serde_json::to_string(&claim).expect("serialize");
    let restored: NegligenceClaim = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(claim, restored);
}

/// A typical negligence assessment completes well under 1 ms.
#[test]
fn negligence_assessment_is_fast() {
    use std::time::Instant;

    let claim = NegligenceClaim::new(
        "perf-1",
        "P",
        "D",
        DutyOfCareAnalysis::established(HarmCategory::PersonalInjury),
        BreachAnalysis::new(StandardOfCare::ReasonablePerson, true),
        CausationAnalysis::established(),
        1_000_000,
    );

    let iterations = 10_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = assess_negligence(&claim);
    }
    let per_call = start.elapsed() / iterations;
    assert!(
        per_call.as_micros() < 1_000,
        "assessment too slow: {per_call:?} per call"
    );
}
