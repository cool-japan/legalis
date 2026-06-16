//! Cross-domain integration tests spanning contract law and tort law.
//!
//! A single commercial fact pattern frequently engages both bodies of law: a
//! defective product can simultaneously be a breach of a contractual condition
//! (sounding in expectation damages) and an act of negligence causing personal
//! injury (sounding in tort damages, reduced for any contributory negligence).
//! These tests confirm the two `legalis_sg` modules compose coherently.

use legalis_sg::contract::{
    Acceptance, AcceptanceMode, AgreementContext, Consideration, Contract, ContractTerm,
    DamagesMeasure, HeadOfLoss, Offer, TermClassification, assess_damages, classify_breach,
    validate_formation,
};
use legalis_sg::tort::{
    BreachAnalysis, CausationAnalysis, DutyOfCareAnalysis, HarmCategory, NegligenceClaim,
    StandardOfCare, TortError, apportion_for_contributory_negligence, assess_negligence,
};

/// A defective machine: breach of a contractual condition AND negligence causing
/// injury, analysed under both regimes from one set of facts.
#[test]
fn defective_product_engages_contract_and_tort() {
    // --- Contract side: formation + breach of condition + damages -----------
    let offer = Offer::new(
        "x-1",
        "Forge Equipment Pte Ltd",
        "Jurong Workshop Pte Ltd",
        "supply of an industrial press",
    );
    let mut contract = Contract::new("kx-1", offer, AgreementContext::Commercial).with_acceptance(
        Acceptance::new("x-1", "Jurong Workshop Pte Ltd", AcceptanceMode::Electronic),
    );
    contract.add_consideration(Consideration::promise(
        "Forge Equipment Pte Ltd",
        "supply a press conforming to specification",
    ));
    contract.add_consideration(Consideration::promise(
        "Jurong Workshop Pte Ltd",
        "pay SGD 90,000",
    ));
    assert!(validate_formation(&contract).is_ok());

    // The safety guard requirement is a condition; its breach permits
    // termination and sounds in expectation damages.
    let safety_condition = ContractTerm::new(
        "t-safety",
        "press to be fitted with a compliant safety guard",
        TermClassification::Condition,
    );
    assert!(classify_breach(&safety_condition, false).may_terminate);

    let contract_heads = vec![
        HeadOfLoss::ordinary("cost of fitting a compliant guard", 1_200_000),
        HeadOfLoss::ordinary("lost production during stoppage", 800_000),
    ];
    let contract_award =
        assess_damages(DamagesMeasure::Expectation, &contract_heads).expect("award");
    assert_eq!(contract_award.recoverable_cents, 2_000_000); // SGD 20,000

    // --- Tort side: negligence causing the operator's injury ----------------
    let negligence = NegligenceClaim::new(
        "nx-1",
        "Injured Operator",
        "Forge Equipment Pte Ltd",
        DutyOfCareAnalysis::established(HarmCategory::PersonalInjury),
        BreachAnalysis::new(StandardOfCare::ReasonablePerson, true).with_risk_factors(80, 95, 10),
        CausationAnalysis::established(),
        6_000_000, // SGD 60,000 personal-injury damages
    );
    match assess_negligence(&negligence) {
        Err(TortError::NegligenceEstablished { .. }) => {}
        other => panic!("expected negligence established, got {other:?}"),
    }

    // The operator bypassed an interlock: 20% contributory negligence.
    let tort_recoverable = apportion_for_contributory_negligence(6_000_000, 20).expect("apportion");
    assert_eq!(tort_recoverable, 4_800_000); // SGD 48,000

    // The two awards are independent heads of recovery arising from one event.
    assert_ne!(contract_award.recoverable_cents, tort_recoverable as i64);
}

/// Where the contractual term breached is a mere warranty, the buyer cannot
/// terminate, yet a parallel negligence claim may still lie for resulting
/// personal injury.
#[test]
fn warranty_breach_no_termination_but_tort_may_lie() {
    let warranty = ContractTerm::new(
        "t-manual",
        "supplier to provide an operating manual in English",
        TermClassification::Warranty,
    );
    // Breach of warranty: damages only, no termination.
    assert!(!classify_breach(&warranty, false).may_terminate);

    // But if the missing instructions caused foreseeable injury, negligence may
    // still be established.
    let negligence = NegligenceClaim::new(
        "nw-1",
        "Operator",
        "Supplier",
        DutyOfCareAnalysis::established(HarmCategory::PersonalInjury),
        BreachAnalysis::new(StandardOfCare::ReasonablePerson, true),
        CausationAnalysis::established(),
        1_500_000,
    );
    assert!(matches!(
        assess_negligence(&negligence),
        Err(TortError::NegligenceEstablished { .. })
    ));
}
