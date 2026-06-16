//! Integration tests for BGB special tort sections (Besondere Deliktstatbestände).
//!
//! Covers §§ 824, 825, 832, 833-838 and 839 BGB through the crate's public
//! `legalis_de::bgb::unerlaubte_handlungen` API, validating both the legal logic
//! and the bilingual error / citation system.

use chrono::Utc;
use legalis_de::bgb::unerlaubte_handlungen::*;
use legalis_de::gmbhg::Capital;

// ===== § 824 Kreditgefährdung =====

#[test]
fn test_824_valid_claim_via_public_api() {
    let claim = CreditEndangermentClaimBuilder::new()
        .asserting_party("Competitor GmbH", "Berlin")
        .affected_party("Mustermann AG", "Munich")
        .statement(
            "Das Unternehmen ist zahlungsunfähig",
            StatementForm::Dissemination,
            StatementNature::FactualAssertion,
        )
        .untrue(true)
        .suitable_to_endanger_credit(true)
        .knew_or_should_have_known(true)
        .incident_date(Utc::now())
        .damages_lost_income(Capital::from_euros(50_000))
        .causation_established(true)
        .build()
        .expect("claim should build");

    assert!(validate_credit_endangerment_claim(&claim).is_ok());
}

#[test]
fn test_824_value_judgment_rejected() {
    let claim = CreditEndangermentClaimBuilder::new()
        .asserting_party("Critic", "Berlin")
        .affected_party("Firm", "Munich")
        .statement(
            "Schlechte Firma",
            StatementForm::Assertion,
            StatementNature::ValueJudgment,
        )
        .untrue(true)
        .suitable_to_endanger_credit(true)
        .knew_or_should_have_known(true)
        .incident_date(Utc::now())
        .damages_lost_income(Capital::from_euros(10_000))
        .causation_established(true)
        .build()
        .expect("claim should build");

    let err = validate_credit_endangerment_claim(&claim).unwrap_err();
    assert_eq!(err, TortError::NotFactualAssertion);
    assert_eq!(err.article_reference(), "§824 Abs. 1 BGB");
}

#[test]
fn test_824_legitimate_interest_privilege_citation() {
    let err = TortError::LegitimateInterestPrivilege;
    assert_eq!(err.article_reference(), "§824 Abs. 2 BGB");
    let msg = err.to_string();
    assert!(msg.contains("berechtigten Interesses"));
    assert!(msg.contains("legitimate interest"));
}

// ===== § 825 Bestimmung zu sexuellen Handlungen =====

#[test]
fn test_825_valid_claim() {
    let claim = SexualSelfDeterminationClaimBuilder::new()
        .tortfeasor("A", "Berlin")
        .injured_party("B", "Berlin")
        .inducement(true)
        .means(InducementMeans::AbuseOfDependence)
        .act_involvement(SexualActInvolvement::Toleration)
        .incident_date(Utc::now())
        .damages_pain_suffering(Capital::from_euros(20_000))
        .causation_established(true)
        .build()
        .expect("claim should build");

    assert!(validate_sexual_self_determination_claim(&claim).is_ok());
}

#[test]
fn test_825_requires_qualifying_means() {
    let claim = SexualSelfDeterminationClaimBuilder::new()
        .tortfeasor("A", "Berlin")
        .injured_party("B", "Berlin")
        .inducement(true)
        .act_involvement(SexualActInvolvement::Undertaking)
        .incident_date(Utc::now())
        .damages_pain_suffering(Capital::from_euros(5_000))
        .causation_established(true)
        .build()
        .expect("claim should build");

    let err = validate_sexual_self_determination_claim(&claim).unwrap_err();
    assert_eq!(err, TortError::NoQualifyingInducementMeans);
    assert_eq!(err.article_reference(), "§825 BGB");
}

// ===== § 832 Aufsichtspflicht =====

#[test]
fn test_832_parent_liable_for_child() {
    let claim = SupervisionLiabilityClaimBuilder::new()
        .supervisor("Parent", "Berlin")
        .supervised_person("Child", "Berlin")
        .injured_third_party("Neighbour", "Berlin")
        .supervision_basis(SupervisionBasis::Statutory)
        .supervision_reason(SupervisionReason::Minority)
        .unlawful_damage_caused(true)
        .incident_date(Utc::now())
        .damages_property(Capital::from_euros(3_000))
        .causation_established(true)
        .build()
        .expect("claim should build");

    assert!(validate_supervision_liability(&claim).is_ok());
}

#[test]
fn test_832_exculpation_citation() {
    let claim = SupervisionLiabilityClaimBuilder::new()
        .supervisor("Care home", "Berlin")
        .supervised_person("Resident", "Berlin")
        .injured_third_party("Visitor", "Berlin")
        .supervision_basis(SupervisionBasis::Contractual)
        .supervision_reason(SupervisionReason::MentalOrPhysicalCondition)
        .unlawful_damage_caused(true)
        .supervision_duty_fulfilled(true)
        .incident_date(Utc::now())
        .damages_property(Capital::from_euros(3_000))
        .causation_established(true)
        .build()
        .expect("claim should build");

    let err = validate_supervision_liability(&claim).unwrap_err();
    assert!(matches!(err, TortError::SupervisorExculpated { .. }));
    assert_eq!(err.article_reference(), "§832 Abs. 1 S. 2 BGB");
}

// ===== §§ 833-835 Tierhalterhaftung =====

#[test]
fn test_833_luxury_animal_strict_liability() {
    let claim = AnimalLiabilityClaimBuilder::new()
        .liable_party("Owner", "Hamburg")
        .injured_party("Victim", "Hamburg")
        .animal("Hobby horse")
        .basis(AnimalLiabilityBasis::Keeper)
        .category(AnimalCategory::LuxuryAnimal)
        .harm_type(PhysicalHarmType::BodilyInjury)
        .caused_by_animal(true)
        .required_care_observed(true) // no effect: strict liability
        .incident_date(Utc::now())
        .damages_medical(Capital::from_euros(7_000))
        .causation_established(true)
        .build()
        .expect("claim should build");

    assert!(claim.is_strict_liability());
    assert!(validate_animal_liability(&claim).is_ok());
}

#[test]
fn test_833_utility_animal_exculpation() {
    let claim = AnimalLiabilityClaimBuilder::new()
        .liable_party("Farmer", "Bavaria")
        .injured_party("Visitor", "Bavaria")
        .animal("Working farm dog")
        .basis(AnimalLiabilityBasis::Keeper)
        .category(AnimalCategory::DomesticUtilityAnimal)
        .harm_type(PhysicalHarmType::HealthInjury)
        .caused_by_animal(true)
        .required_care_observed(true)
        .incident_date(Utc::now())
        .damages_medical(Capital::from_euros(4_000))
        .causation_established(true)
        .build()
        .expect("claim should build");

    assert!(!claim.is_strict_liability());
    let err = validate_animal_liability(&claim).unwrap_err();
    assert!(matches!(err, TortError::AnimalKeeperExculpated { .. }));
    assert_eq!(err.article_reference(), "§833 S. 2 / §834 S. 2 BGB");
}

#[test]
fn test_834_supervisor_exculpation() {
    let claim = AnimalLiabilityClaimBuilder::new()
        .liable_party("Dog sitter", "Berlin")
        .injured_party("Jogger", "Berlin")
        .animal("Boarded dog")
        .basis(AnimalLiabilityBasis::Supervisor)
        .category(AnimalCategory::LuxuryAnimal)
        .harm_type(PhysicalHarmType::BodilyInjury)
        .caused_by_animal(true)
        .damage_would_have_occurred_anyway(true)
        .incident_date(Utc::now())
        .damages_medical(Capital::from_euros(2_000))
        .causation_established(true)
        .build()
        .expect("claim should build");

    assert!(matches!(
        validate_animal_liability(&claim),
        Err(TortError::AnimalKeeperExculpated { .. })
    ));
}

#[test]
fn test_835_repealed_marker_exposed() {
    assert!(SECTION_835_REPEALED.contains("BJagdG"));
}

// ===== §§ 836-838 Gebäudehaftung =====

#[test]
fn test_836_land_possessor_liable() {
    let claim = BuildingLiabilityClaimBuilder::new()
        .liable_party("Owner", "Cologne", BuildingLiableParty::LandPossessor)
        .injured_party("Pedestrian", "Cologne")
        .structure("Crumbling wall")
        .failure_type(StructuralFailureType::Collapse)
        .defect_cause(StructuralDefectCause::DefectiveMaintenance)
        .harm_type(PhysicalHarmType::HealthInjury)
        .incident_date(Utc::now())
        .damages_medical(Capital::from_euros(8_000))
        .causation_established(true)
        .build()
        .expect("claim should build");

    assert!(validate_building_liability(&claim).is_ok());
}

#[test]
fn test_836_force_majeure_no_liability() {
    let claim = BuildingLiabilityClaimBuilder::new()
        .liable_party("Owner", "Cologne", BuildingLiableParty::LandPossessor)
        .injured_party("Pedestrian", "Cologne")
        .structure("Wall")
        .failure_type(StructuralFailureType::Collapse)
        .defect_cause(StructuralDefectCause::Other)
        .harm_type(PhysicalHarmType::PropertyDamage)
        .incident_date(Utc::now())
        .damages_property(Capital::from_euros(8_000))
        .causation_established(true)
        .build()
        .expect("claim should build");

    let err = validate_building_liability(&claim).unwrap_err();
    assert_eq!(err, TortError::NoStructuralDefectCausation);
    assert_eq!(err.article_reference(), "§836 BGB");
}

#[test]
fn test_838_maintenance_obligor_exculpation() {
    let claim = BuildingLiabilityClaimBuilder::new()
        .liable_party(
            "Maintenance firm",
            "Cologne",
            BuildingLiableParty::MaintenanceObligor,
        )
        .injured_party("Pedestrian", "Cologne")
        .structure("Balcony")
        .failure_type(StructuralFailureType::DetachmentOfParts)
        .defect_cause(StructuralDefectCause::FaultyConstruction)
        .harm_type(PhysicalHarmType::BodilyInjury)
        .required_care_observed(true)
        .incident_date(Utc::now())
        .damages_medical(Capital::from_euros(8_000))
        .causation_established(true)
        .build()
        .expect("claim should build");

    assert!(matches!(
        validate_building_liability(&claim),
        Err(TortError::BuildingPossessorExculpated { .. })
    ));
}

// ===== § 839 Amtshaftung =====

#[test]
fn test_839_intentional_breach_liable() {
    let claim = OfficialLiabilityClaimBuilder::new()
        .official("Official", "Dresden")
        .injured_party("Citizen", "Dresden")
        .is_official(true)
        .official_duty_breached(true)
        .duty_owed_to_third_party(true)
        .fault(OfficialFault::Intent)
        .incident_date(Utc::now())
        .damages_property(Capital::from_euros(30_000))
        .causation_established(true)
        .build()
        .expect("claim should build");

    assert!(validate_official_liability(&claim).is_ok());
}

#[test]
fn test_839_subsidiarity_for_negligence() {
    let claim = OfficialLiabilityClaimBuilder::new()
        .official("Official", "Dresden")
        .injured_party("Citizen", "Dresden")
        .is_official(true)
        .official_duty_breached(true)
        .duty_owed_to_third_party(true)
        .fault(OfficialFault::Negligence)
        .alternative_compensation_available(true)
        .incident_date(Utc::now())
        .damages_property(Capital::from_euros(30_000))
        .causation_established(true)
        .build()
        .expect("claim should build");

    let err = validate_official_liability(&claim).unwrap_err();
    assert_eq!(err, TortError::OfficialLiabilitySubsidiary);
    assert_eq!(err.article_reference(), "§839 Abs. 1 S. 2 BGB");
}

#[test]
fn test_839_judicial_privilege() {
    let claim = OfficialLiabilityClaimBuilder::new()
        .official("Judge", "Karlsruhe")
        .injured_party("Litigant", "Karlsruhe")
        .is_official(true)
        .official_duty_breached(true)
        .duty_owed_to_third_party(true)
        .fault(OfficialFault::Negligence)
        .judicial_context(true, false, false)
        .incident_date(Utc::now())
        .damages_property(Capital::from_euros(30_000))
        .causation_established(true)
        .build()
        .expect("claim should build");

    let err = validate_official_liability(&claim).unwrap_err();
    assert_eq!(err, TortError::JudicialPrivilege);
    assert_eq!(err.article_reference(), "§839 Abs. 2 BGB");
}

#[test]
fn test_839_failure_to_use_remedy() {
    let claim = OfficialLiabilityClaimBuilder::new()
        .official("Official", "Dresden")
        .injured_party("Citizen", "Dresden")
        .is_official(true)
        .official_duty_breached(true)
        .duty_owed_to_third_party(true)
        .fault(OfficialFault::Intent)
        .failed_to_use_legal_remedy(true)
        .incident_date(Utc::now())
        .damages_property(Capital::from_euros(30_000))
        .causation_established(true)
        .build()
        .expect("claim should build");

    let err = validate_official_liability(&claim).unwrap_err();
    assert_eq!(err, TortError::FailureToUseLegalRemedy);
    assert_eq!(err.article_reference(), "§839 Abs. 3 BGB");
}

// ===== Cross-cutting: bilingual citation coverage =====

#[test]
fn test_all_new_errors_have_section_citation() {
    let errors = [
        TortError::NotFactualAssertion,
        TortError::StatementNotUntrue,
        TortError::NotSuitableToEndangerCredit,
        TortError::NoKnowledgeOrNegligenceOfUntruth,
        TortError::LegitimateInterestPrivilege,
        TortError::NoSexualActInducement,
        TortError::NoQualifyingInducementMeans,
        TortError::NoUnlawfulDamageBySupervised,
        TortError::SupervisorExculpated {
            ground: "x".to_string(),
        },
        TortError::NotCausedByAnimal,
        TortError::AnimalKeeperExculpated {
            ground: "x".to_string(),
        },
        TortError::NoStructuralDefectCausation,
        TortError::BuildingPossessorExculpated {
            ground: "x".to_string(),
        },
        TortError::NotAnOfficial,
        TortError::NoOfficialDutyBreach,
        TortError::NoDutyToThirdParty,
        TortError::NoOfficialFault,
        TortError::OfficialLiabilitySubsidiary,
        TortError::JudicialPrivilege,
        TortError::FailureToUseLegalRemedy,
    ];

    for err in errors {
        // Every new error must cite a § and carry a bilingual message.
        assert!(err.article_reference().contains('§'));
        let msg = err.to_string();
        assert!(msg.contains('\n'), "expected bilingual message: {msg}");
    }
}
