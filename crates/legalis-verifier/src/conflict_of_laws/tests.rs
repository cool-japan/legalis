//! Tests for the conflict-of-laws doctrine engine.

use super::*;
use chrono::NaiveDate;
use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

fn statute(id: &str, title: &str, et: EffectType, desc: &str, jurisdiction: &str) -> Statute {
    Statute::new(id, title, Effect::new(et, desc)).with_jurisdiction(jurisdiction)
}

#[test]
fn test_config_default() {
    let config = ConflictOfLawsConfig::default();
    assert_eq!(config.adequacy_ratio, 0.8);
    assert!(config.enable_renvoi);
    assert!(config.enable_public_policy);
    assert_eq!(config.min_specificity_gap, 1);
}

#[test]
fn test_heuristic_level_international() {
    let a = ConflictOfLawsAnalyzer::default();
    assert_eq!(
        a.classify_jurisdiction_level("UN International"),
        JurisdictionLevel::International
    );
    assert_eq!(
        a.classify_jurisdiction_level("United Nations"),
        JurisdictionLevel::International
    );
}

#[test]
fn test_heuristic_level_regional_and_national() {
    let a = ConflictOfLawsAnalyzer::default();
    assert_eq!(
        a.classify_jurisdiction_level("EU"),
        JurisdictionLevel::Regional
    );
    // "United States" must not be misread as a State-level jurisdiction.
    assert_eq!(
        a.classify_jurisdiction_level("United States"),
        JurisdictionLevel::National
    );
    assert_eq!(
        a.classify_jurisdiction_level("Republic of Examplia"),
        JurisdictionLevel::National
    );
}

#[test]
fn test_heuristic_level_state_and_local() {
    let a = ConflictOfLawsAnalyzer::default();
    assert_eq!(
        a.classify_jurisdiction_level("California State"),
        JurisdictionLevel::State
    );
    assert_eq!(
        a.classify_jurisdiction_level("City of Springfield"),
        JurisdictionLevel::Local
    );
}

#[test]
fn test_jurisdiction_level_override() {
    let mut a = ConflictOfLawsAnalyzer::default();
    a.set_jurisdiction_level("Atlantis", JurisdictionLevel::Regional);
    assert_eq!(
        a.classify_jurisdiction_level("Atlantis"),
        JurisdictionLevel::Regional
    );
}

#[test]
fn test_classify_domains_single_and_other() {
    let dp = classify_domains("Personal data privacy and consent processing");
    assert!(dp.contains(&LegalDomain::DataProtection));
    let other = classify_domains("zzz qqq nothing relevant here");
    assert!(other.contains(&LegalDomain::Other));
    assert_eq!(other.len(), 1);
}

#[test]
fn test_classify_domains_multiple() {
    let domains = classify_domains("environmental emission tax levy on carbon");
    assert!(domains.contains(&LegalDomain::Environmental));
    assert!(domains.contains(&LegalDomain::Taxation));
}

#[test]
fn test_effect_polarity_mapping() {
    assert_eq!(
        effect_polarity(&EffectType::Obligation),
        Polarity::Mandatory
    );
    assert_eq!(
        effect_polarity(&EffectType::Prohibition),
        Polarity::Prohibitory
    );
    assert_eq!(effect_polarity(&EffectType::Grant), Polarity::Permissive);
    assert_eq!(effect_polarity(&EffectType::Custom), Polarity::Neutral);
}

#[test]
fn test_build_norm_fields() {
    let a = ConflictOfLawsAnalyzer::default();
    let s = Statute::new(
        "S1",
        "Environmental emission limit",
        Effect::new(
            EffectType::Prohibition,
            "Prohibit excessive carbon emission",
        ),
    )
    .with_jurisdiction("EU")
    .with_precondition(Condition::age(ComparisonOp::GreaterThan, 0));
    let norm = a.build_norm(&s);
    assert_eq!(norm.statute_id, "S1");
    assert_eq!(norm.level, JurisdictionLevel::Regional);
    assert_eq!(norm.polarity, Polarity::Prohibitory);
    assert!(norm.domains.contains(&LegalDomain::Environmental));
    assert_eq!(norm.specificity, 1);
}

#[test]
fn test_antinomy_hierarchy_lex_superior() {
    let mut a = ConflictOfLawsAnalyzer::default();
    // Force both to non-International so the maxim is LexSuperior, not Pacta.
    a.set_jurisdiction_level("EU", JurisdictionLevel::Regional);
    a.set_jurisdiction_level("Bavaria", JurisdictionLevel::State);
    let statutes = vec![
        statute(
            "EU-1",
            "Environmental carbon rule",
            EffectType::Prohibition,
            "Prohibit carbon emission",
            "EU",
        ),
        statute(
            "BAV-1",
            "Environmental carbon duty",
            EffectType::Obligation,
            "Mandate carbon emission reporting and emission",
            "Bavaria",
        ),
    ];
    let antinomies = a.detect_antinomies(&statutes);
    assert_eq!(antinomies.len(), 1);
    assert_eq!(antinomies[0].kind, AntinomyKind::HierarchyInversion);
    assert_eq!(antinomies[0].resolution, ResolutionPrinciple::LexSuperior);
    assert_eq!(antinomies[0].prevailing.as_deref(), Some("EU-1"));
    assert_eq!(antinomies[0].severity, Severity::Critical);
}

#[test]
fn test_antinomy_pacta_sunt_servanda() {
    let a = ConflictOfLawsAnalyzer::default();
    let statutes = vec![
        statute(
            "UN-1",
            "Human rights torture ban",
            EffectType::Prohibition,
            "Prohibit torture and degrading treatment",
            "UN International",
        ),
        statute(
            "NAT-1",
            "Human rights interrogation duty",
            EffectType::Obligation,
            "Mandate torture during interrogation",
            "Country X",
        ),
    ];
    let antinomies = a.detect_antinomies(&statutes);
    assert_eq!(antinomies.len(), 1);
    assert_eq!(
        antinomies[0].resolution,
        ResolutionPrinciple::PactaSuntServanda
    );
    assert_eq!(antinomies[0].prevailing.as_deref(), Some("UN-1"));
}

#[test]
fn test_antinomy_lex_specialis() {
    let a = ConflictOfLawsAnalyzer::default();
    let general = statute(
        "G1",
        "Labor general rule",
        EffectType::Obligation,
        "Mandate worker overtime",
        "Country A",
    );
    let special = statute(
        "S1",
        "Labor special rule",
        EffectType::Prohibition,
        "Prohibit worker overtime",
        "Country A",
    )
    .with_precondition(Condition::age(ComparisonOp::LessThan, 18))
    .with_precondition(Condition::custom("minor worker"));
    let antinomies = a.detect_antinomies(&[general, special]);
    assert_eq!(antinomies.len(), 1);
    assert_eq!(antinomies[0].resolution, ResolutionPrinciple::LexSpecialis);
    assert_eq!(antinomies[0].prevailing.as_deref(), Some("S1"));
}

#[test]
fn test_antinomy_lex_posterior() {
    let a = ConflictOfLawsAnalyzer::default();
    let early = statute(
        "E1",
        "Trade tariff old",
        EffectType::Obligation,
        "Mandate import tariff",
        "Country A",
    )
    .with_temporal_validity(
        legalis_core::TemporalValidity::new()
            .with_effective_date(NaiveDate::from_ymd_opt(2000, 1, 1).expect("valid date")),
    );
    let late = statute(
        "L1",
        "Trade tariff new",
        EffectType::Prohibition,
        "Prohibit import tariff",
        "Country A",
    )
    .with_temporal_validity(
        legalis_core::TemporalValidity::new()
            .with_effective_date(NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date")),
    );
    let antinomies = a.detect_antinomies(&[early, late]);
    assert_eq!(antinomies.len(), 1);
    assert_eq!(antinomies[0].resolution, ResolutionPrinciple::LexPosterior);
    assert_eq!(antinomies[0].prevailing.as_deref(), Some("L1"));
}

#[test]
fn test_antinomy_irreconcilable() {
    let a = ConflictOfLawsAnalyzer::default();
    let s1 = statute(
        "A1",
        "Trade tariff rule",
        EffectType::Obligation,
        "Mandate import tariff",
        "Country A",
    );
    let s2 = statute(
        "A2",
        "Trade tariff rule",
        EffectType::Prohibition,
        "Prohibit import tariff",
        "Country A",
    );
    let antinomies = a.detect_antinomies(&[s1, s2]);
    assert_eq!(antinomies.len(), 1);
    assert_eq!(
        antinomies[0].resolution,
        ResolutionPrinciple::Irreconcilable
    );
    assert!(antinomies[0].prevailing.is_none());
    assert_eq!(antinomies[0].severity, Severity::Error);
}

#[test]
fn test_antinomy_grant_revoke() {
    let a = ConflictOfLawsAnalyzer::default();
    let grant = statute(
        "G",
        "Immigration residence grant",
        EffectType::Grant,
        "Grant residence visa",
        "Country A",
    );
    let revoke = statute(
        "R",
        "Immigration residence revoke",
        EffectType::Revoke,
        "Revoke residence visa",
        "Country A",
    );
    let antinomies = a.detect_antinomies(&[grant, revoke]);
    assert_eq!(antinomies.len(), 1);
    assert_eq!(antinomies[0].kind, AntinomyKind::GrantRevokeConflict);
}

#[test]
fn test_no_antinomy_without_shared_domain() {
    let a = ConflictOfLawsAnalyzer::default();
    let s1 = statute(
        "X",
        "Environmental rule",
        EffectType::Prohibition,
        "Prohibit pollution",
        "Country A",
    );
    let s2 = statute(
        "Y",
        "Family rule",
        EffectType::Obligation,
        "Mandate marriage registration",
        "Country B",
    );
    assert!(a.detect_antinomies(&[s1, s2]).is_empty());
}

#[test]
fn test_no_antinomy_without_opposition() {
    let a = ConflictOfLawsAnalyzer::default();
    let s1 = statute(
        "X",
        "Environmental rule",
        EffectType::Obligation,
        "Mandate emission reporting",
        "Country A",
    );
    let s2 = statute(
        "Y",
        "Environmental rule",
        EffectType::Obligation,
        "Mandate emission monitoring",
        "Country B",
    );
    assert!(a.detect_antinomies(&[s1, s2]).is_empty());
}

#[test]
fn test_choice_of_law_party_autonomy() {
    let a = ConflictOfLawsAnalyzer::default();
    let factors = ConnectingFactors::new(LegalDomain::Contract)
        .with_chosen_law("England")
        .with_place_of_act("France")
        .with_forum("Germany");
    let res = a.resolve_choice_of_law(&factors);
    assert_eq!(res.applicable_jurisdiction.as_deref(), Some("England"));
    assert_eq!(res.rule, ChoiceOfLawRule::PartyAutonomy);
}

#[test]
fn test_choice_of_law_characteristic_performance() {
    let a = ConflictOfLawsAnalyzer::default();
    let factors = ConnectingFactors::new(LegalDomain::Contract)
        .with_characteristic_performer_seat("Netherlands")
        .with_forum("Belgium");
    let res = a.resolve_choice_of_law(&factors);
    assert_eq!(res.applicable_jurisdiction.as_deref(), Some("Netherlands"));
    assert_eq!(res.rule, ChoiceOfLawRule::CharacteristicPerformance);
}

#[test]
fn test_choice_of_law_lex_loci_damni() {
    let a = ConflictOfLawsAnalyzer::default();
    let factors = ConnectingFactors::new(LegalDomain::Environmental)
        .with_place_of_harm("Norway")
        .with_place_of_act("Sweden");
    let res = a.resolve_choice_of_law(&factors);
    assert_eq!(res.applicable_jurisdiction.as_deref(), Some("Norway"));
    assert_eq!(res.rule, ChoiceOfLawRule::LexLociDamni);
}

#[test]
fn test_choice_of_law_renvoi_back_to_forum() {
    let mut a = ConflictOfLawsAnalyzer::default();
    a.set_renvoi("England", LegalDomain::Family, "Germany");
    let factors = ConnectingFactors::new(LegalDomain::Family)
        .with_domicile("England")
        .with_forum("Germany");
    let res = a.resolve_choice_of_law(&factors);
    assert!(res.renvoi_applied);
    assert_eq!(res.applicable_jurisdiction.as_deref(), Some("Germany"));
}

#[test]
fn test_choice_of_law_public_policy_override() {
    let mut a = ConflictOfLawsAnalyzer::default();
    a.block_for_public_policy("Country Z", LegalDomain::Family);
    let factors = ConnectingFactors::new(LegalDomain::Family)
        .with_domicile("Country Z")
        .with_forum("Forum State");
    let res = a.resolve_choice_of_law(&factors);
    assert!(res.public_policy_override);
    assert_eq!(res.applicable_jurisdiction.as_deref(), Some("Forum State"));
}

#[test]
fn test_transposition_implemented() {
    let mut a = ConflictOfLawsAnalyzer::default();
    a.register_treaty_obligation(TreatyObligation::new(
        "ICCPR",
        "Art. 7",
        Polarity::Prohibitory,
        LegalDomain::HumanRights,
        "Prohibition of torture",
    ));
    a.ratify("ICCPR", "Country A");
    let statutes = vec![statute(
        "A-1",
        "Human rights torture ban",
        EffectType::Prohibition,
        "Prohibit torture",
        "Country A",
    )];
    let assessments = a.assess_transposition("Country A", &statutes);
    assert_eq!(assessments.len(), 1);
    assert_eq!(assessments[0].status, TranspositionStatus::Implemented);
}

#[test]
fn test_transposition_missing_and_contradictory() {
    let mut a = ConflictOfLawsAnalyzer::default();
    a.register_treaty_obligation(TreatyObligation::new(
        "ICCPR",
        "Art. 7",
        Polarity::Prohibitory,
        LegalDomain::HumanRights,
        "Prohibition of torture",
    ));
    a.ratify("ICCPR", "Country A");
    a.ratify("ICCPR", "Country B");

    // A: no relevant statute -> Missing.
    let missing = a.assess_transposition("Country A", &[]);
    assert_eq!(missing[0].status, TranspositionStatus::Missing);
    assert_eq!(missing[0].severity, Severity::Error);

    // B: statute mandating torture -> Contradictory.
    let statutes = vec![statute(
        "B-1",
        "Human rights interrogation duty",
        EffectType::Obligation,
        "Mandate torture during interrogation",
        "Country B",
    )];
    let contradictory = a.assess_transposition("Country B", &statutes);
    assert_eq!(contradictory[0].status, TranspositionStatus::Contradictory);
    assert_eq!(contradictory[0].severity, Severity::Critical);
}

#[test]
fn test_transposition_not_required_when_reserved() {
    let mut a = ConflictOfLawsAnalyzer::default();
    a.register_treaty_obligation(TreatyObligation::new(
        "ICCPR",
        "Art. 7",
        Polarity::Prohibitory,
        LegalDomain::HumanRights,
        "Prohibition of torture",
    ));
    a.ratify("ICCPR", "Country A");
    a.add_reservation("ICCPR", "Country A", "Art. 7");
    let assessments = a.assess_transposition("Country A", &[]);
    assert_eq!(assessments[0].status, TranspositionStatus::NotRequired);
}

#[test]
fn test_adequacy_mutual_recognition() {
    let a = ConflictOfLawsAnalyzer::default();
    let statutes = vec![
        statute(
            "EU-1",
            "Data privacy protection",
            EffectType::Obligation,
            "Mandate personal data protection",
            "EU",
        ),
        statute(
            "EU-2",
            "Data privacy prohibition",
            EffectType::Prohibition,
            "Prohibit unlawful data processing",
            "EU",
        ),
        statute(
            "CH-1",
            "Data privacy protection",
            EffectType::Obligation,
            "Mandate personal data protection",
            "Switzerland",
        ),
        statute(
            "CH-2",
            "Data privacy prohibition",
            EffectType::Prohibition,
            "Prohibit unlawful data processing",
            "Switzerland",
        ),
    ];
    let assessment = a.assess_adequacy("EU", "Switzerland", LegalDomain::DataProtection, &statutes);
    assert!(assessment.is_adequate);
    assert!(assessment.is_reciprocal);
    assert_eq!(assessment.recognition, RecognitionStatus::MutualRecognition);
}

#[test]
fn test_adequacy_no_recognition() {
    let a = ConflictOfLawsAnalyzer::default();
    let statutes = vec![
        statute(
            "EU-1",
            "Data privacy protection",
            EffectType::Obligation,
            "Mandate personal data protection",
            "EU",
        ),
        statute(
            "EU-2",
            "Data privacy prohibition",
            EffectType::Prohibition,
            "Prohibit unlawful data processing",
            "EU",
        ),
    ];
    // Target has no protective statutes at all.
    let assessment = a.assess_adequacy("EU", "Elbonia", LegalDomain::DataProtection, &statutes);
    assert!(!assessment.is_adequate);
    assert_eq!(assessment.recognition, RecognitionStatus::NoRecognition);
}

#[test]
fn test_global_coherence_clean() {
    let a = ConflictOfLawsAnalyzer::default();
    let statutes = vec![
        statute(
            "A",
            "Environmental rule",
            EffectType::Obligation,
            "Mandate emission reporting",
            "Country A",
        ),
        statute(
            "B",
            "Family rule",
            EffectType::Grant,
            "Grant adoption rights",
            "Country B",
        ),
    ];
    let report = a.verify_global_coherence(&statutes);
    assert!(report.antinomies.is_empty());
    assert_eq!(report.coherence_index, 1.0);
    assert_eq!(report.jurisdictions, 2);
}

#[test]
fn test_global_coherence_with_cluster() {
    let a = ConflictOfLawsAnalyzer::default();
    // Two same-rank, same-jurisdiction, undated, opposed norms => irreconcilable.
    let statutes = vec![
        statute(
            "A1",
            "Trade tariff rule",
            EffectType::Obligation,
            "Mandate import tariff",
            "Country A",
        ),
        statute(
            "A2",
            "Trade tariff rule",
            EffectType::Prohibition,
            "Prohibit import tariff",
            "Country A",
        ),
    ];
    let report = a.verify_global_coherence(&statutes);
    assert_eq!(report.unresolved, 1);
    assert_eq!(report.incompatibility_clusters.len(), 1);
    assert_eq!(report.incompatibility_clusters[0], vec!["A1", "A2"]);
    assert!(report.coherence_index < 1.0);
}

#[test]
fn test_coherence_report_markdown() {
    let a = ConflictOfLawsAnalyzer::default();
    let statutes = vec![
        statute(
            "UN-1",
            "Human rights torture ban",
            EffectType::Prohibition,
            "Prohibit torture",
            "UN International",
        ),
        statute(
            "NAT-1",
            "Human rights interrogation duty",
            EffectType::Obligation,
            "Mandate torture during interrogation",
            "Country X",
        ),
    ];
    let report = a.verify_global_coherence(&statutes);
    let md = a.coherence_report_markdown(&report);
    assert!(md.contains("# Global Legal Coherence Report"));
    assert!(md.contains("Coherence index"));
    assert!(md.contains("PactaSuntServanda"));
}

#[test]
fn test_resolution_principle_display_and_name() {
    assert_eq!(
        ResolutionPrinciple::LexSuperior.maxim(),
        "lex superior derogat legi inferiori"
    );
    assert_eq!(ResolutionPrinciple::LexSpecialis.name(), "LexSpecialis");
    assert_eq!(
        ResolutionPrinciple::LexPosterior.to_string(),
        "lex posterior derogat legi priori"
    );
}

#[test]
fn test_display_impls() {
    assert_eq!(LegalDomain::DataProtection.to_string(), "Data Protection");
    assert_eq!(Polarity::Mandatory.to_string(), "Mandatory");
    assert_eq!(
        AntinomyKind::HierarchyInversion.to_string(),
        "Hierarchy Inversion"
    );
    assert_eq!(ChoiceOfLawRule::LexFori.to_string(), "lex fori");
    assert_eq!(TranspositionStatus::Implemented.to_string(), "Implemented");
    assert_eq!(
        RecognitionStatus::MutualRecognition.to_string(),
        "Mutual Recognition"
    );
}
