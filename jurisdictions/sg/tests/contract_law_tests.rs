//! Integration tests for the Singapore contract-law module.
//!
//! These exercise the public API of [`legalis_sg::contract`] across the four
//! doctrinal areas — formation, terms, vitiating factors, discharge and
//! remedies — through realistic Singapore commercial scenarios.

use legalis_sg::contract::*;

/// A complete, well-formed commercial sale contract should pass formation.
#[test]
fn commercial_sale_contract_is_well_formed() {
    let offer = Offer::new(
        "o-2026-001",
        "Marina Supplies Pte Ltd",
        "Harbourfront Logistics Pte Ltd",
        "supply of 100 pallet racks",
    );
    let mut contract = Contract::new("k-2026-001", offer, AgreementContext::Commercial)
        .with_acceptance(Acceptance::new(
            "o-2026-001",
            "Harbourfront Logistics Pte Ltd",
            AcceptanceMode::Electronic,
        ));
    contract.add_consideration(Consideration::promise(
        "Marina Supplies Pte Ltd",
        "deliver 100 pallet racks",
    ));
    contract.add_consideration(Consideration::promise(
        "Harbourfront Logistics Pte Ltd",
        "pay SGD 120,000",
    ));

    assert!(validate_formation(&contract).is_ok());
    assert!(is_formed(&contract));
}

/// A pre-existing-duty promise with no practical benefit is not good
/// consideration (cf *Williams v Roffey Bros* \[1991\] 1 QB 1).
#[test]
fn existing_duty_without_practical_benefit_fails_formation() {
    let offer = Offer::new("o2", "Builder", "Owner", "extra payment to finish on time");
    let mut contract = Contract::new("k2", offer, AgreementContext::Commercial).with_acceptance(
        Acceptance::new("o2", "Owner", AcceptanceMode::Instantaneous),
    );
    contract.add_consideration(
        Consideration::promise("Builder", "complete the work already contracted for")
            .with_kind(ConsiderationKind::ExistingDuty),
    );

    match validate_formation(&contract) {
        Err(ContractError::ExistingDutyConsideration { .. }) => {}
        other => panic!("expected existing-duty error, got {other:?}"),
    }
}

/// A condition entitles termination on any breach; a warranty does not.
#[test]
fn term_classification_drives_termination_right() {
    let condition = ContractTerm::new(
        "t-cond",
        "goods to be delivered by the agreed date (time of the essence)",
        TermClassification::Condition,
    );
    assert!(classify_breach(&condition, false).may_terminate);

    let warranty = ContractTerm::new(
        "t-warr",
        "supplier to provide a courtesy product manual",
        TermClassification::Warranty,
    );
    assert!(!classify_breach(&warranty, false).may_terminate);
}

/// An innominate term: termination only where the breach deprives the innocent
/// party of substantially the whole benefit (*Hongkong Fir*).
#[test]
fn innominate_term_follows_hongkong_fir() {
    let term = ContractTerm::new(
        "t-innom",
        "vessel to be seaworthy",
        TermClassification::Innominate,
    );
    assert!(!classify_breach(&term, false).may_terminate);
    assert!(classify_breach(&term, true).may_terminate);
}

/// A negligent misrepresentation under s. 2(1) of the Misrepresentation Act 1967
/// is actionable.
#[test]
fn negligent_misrepresentation_is_actionable() {
    let misrep = Misrepresentation::new(
        "the warehouse had never flooded",
        MisrepresentationCategory::Negligent,
    );
    match assess_misrepresentation(&misrep) {
        Err(ContractError::Misrepresentation { authority, .. }) => {
            assert_eq!(authority, "Misrepresentation Act 1967 s. 2(1)");
        }
        other => panic!("expected misrepresentation, got {other:?}"),
    }
}

/// A unilateral mistake known to the other party is operative (*Chwee Kin Keong v
/// Digilandmall.com* \[2005\] SGCA 2).
#[test]
fn known_unilateral_mistake_is_operative() {
    let mistake = OperativeMistake::new(
        MistakeKind::Unilateral,
        "printer priced at SGD 66 instead of SGD 3,854",
    )
    .with_actual_knowledge();
    assert!(assess_mistake(&mistake).is_err());
}

/// Economic duress requires illegitimate pressure that is a significant cause and
/// the absence of a realistic practical alternative.
#[test]
fn economic_duress_made_out_then_negatived() {
    let claim = DuressClaim::new(
        DuressKind::Economic,
        "threat to halt critical supply unless price doubled",
    );
    assert!(assess_duress(&claim).is_err());

    let mut with_alternative = claim;
    with_alternative.practical_alternative = true;
    assert!(assess_duress(&with_alternative).is_ok());
}

/// Frustration succeeds for a supervening illegality but not where the risk was
/// allocated by a force-majeure term.
#[test]
fn frustration_and_its_limits() {
    let event = FrustratingEvent::new("export of the goods became illegal by new sanctions");
    assert_eq!(
        assess_frustration(&event).expect("frustrates"),
        DischargeMode::Frustration
    );

    let allocated = event.risk_allocated();
    assert!(assess_frustration(&allocated).is_err());
}

/// Damages: a remote second-limb loss is excluded, an avoidable loss is excluded,
/// and the ordinary loss is recovered (*Hadley v Baxendale*; *British
/// Westinghouse*).
#[test]
fn damages_apply_remoteness_and_mitigation() {
    let heads = vec![
        HeadOfLoss::ordinary("cost of sourcing replacement racks", 1_500_000),
        HeadOfLoss::special("profit on an undisclosed sub-contract", 8_000_000, false),
        HeadOfLoss::ordinary("storage that could have been avoided", 300_000).avoidable(),
    ];
    let award = assess_damages(DamagesMeasure::Expectation, &heads).expect("award");
    assert_eq!(award.claimed_cents, 9_800_000);
    assert_eq!(award.recoverable_cents, 1_500_000);
    assert_eq!(award.remote_heads.len(), 1);
    assert_eq!(award.unmitigated_heads.len(), 1);
}

/// Specific performance is available for the sale of land (unique subject
/// matter) but not where damages are adequate.
#[test]
fn specific_performance_for_land_only() {
    let land = SpecificPerformanceFactors::new().unique_subject_matter();
    assert!(assess_specific_performance(&land).is_ok());

    let ordinary_goods = SpecificPerformanceFactors::new();
    assert!(assess_specific_performance(&ordinary_goods).is_err());
}

/// The consolidated report flags an otherwise well-formed contract as
/// unenforceable when a vitiating factor is present.
#[test]
fn analysis_report_detects_vitiated_contract() {
    let offer = Offer::new("o-r", "Vendor", "Purchaser", "sale of a painting");
    let mut contract = Contract::new("k-r", offer, AgreementContext::Commercial).with_acceptance(
        Acceptance::new("o-r", "Purchaser", AcceptanceMode::Electronic),
    );
    contract.add_consideration(Consideration::promise("Vendor", "deliver the painting"));
    contract.add_consideration(Consideration::promise("Purchaser", "pay SGD 250,000"));

    let misrep = Misrepresentation::new(
        "the painting is an original by a named master",
        MisrepresentationCategory::Fraudulent,
    );
    let report = analyse_contract(&contract, Some(&misrep), None, None, None);
    assert!(report.formed);
    assert!(!report.is_enforceable());
    assert_eq!(report.vitiating_factors.len(), 1);
}

/// Serialization roundtrips for the aggregate contract type.
#[test]
fn contract_serde_roundtrip() {
    let offer = Offer::new("o-s", "A", "B", "services agreement");
    let mut contract = Contract::new("k-s", offer, AgreementContext::Commercial)
        .with_acceptance(Acceptance::new("o-s", "B", AcceptanceMode::Postal));
    contract.add_consideration(Consideration::promise("A", "render services"));
    contract.add_consideration(Consideration::promise("B", "pay the fee"));
    contract.add_term(ContractTerm::new(
        "t1",
        "services to a professional standard",
        TermClassification::Innominate,
    ));

    let json = serde_json::to_string(&contract).expect("serialize");
    let restored: Contract = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(contract, restored);
}

/// A typical formation validation completes well under 1 ms — run a tight loop
/// to demonstrate it is cheap.
#[test]
fn formation_validation_is_fast() {
    use std::time::Instant;

    let offer = Offer::new("o-p", "A", "B", "supply agreement");
    let mut contract = Contract::new("k-p", offer, AgreementContext::Commercial)
        .with_acceptance(Acceptance::new("o-p", "B", AcceptanceMode::Electronic));
    contract.add_consideration(Consideration::promise("A", "supply goods"));
    contract.add_consideration(Consideration::promise("B", "pay price"));

    let iterations = 10_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = validate_formation(&contract);
    }
    let elapsed = start.elapsed();
    let per_call = elapsed / iterations;
    // A single validation should be far under 1 millisecond.
    assert!(
        per_call.as_micros() < 1_000,
        "validation too slow: {per_call:?} per call"
    );
}
