//! Tests for the domain-specific language variants.

use super::*;
use crate::ast::{
    ConditionNode, ConditionValue, DefaultNode, EffectNode, LegalDocument, StatuteNode,
};
use crate::module_system::Visibility;

// ---------------------------------------------------------------------------
// Helpers.
// ---------------------------------------------------------------------------

fn statute_with(
    id: &str,
    conditions: Vec<ConditionNode>,
    defaults: Vec<DefaultNode>,
) -> StatuteNode {
    StatuteNode {
        id: id.to_string(),
        title: format!("Statute {id}"),
        visibility: Visibility::Private,
        conditions,
        effects: vec![EffectNode {
            effect_type: "grant".to_string(),
            description: "x".to_string(),
            parameters: vec![],
        }],
        discretion: None,
        exceptions: vec![],
        amendments: vec![],
        supersedes: vec![],
        defaults,
        requires: vec![],
        delegates: vec![],
        scope: None,
        constraints: vec![],
        priority: None,
    }
}

fn errors(diags: &[DomainDiagnostic]) -> Vec<&DomainDiagnostic> {
    diags.iter().filter(|d| d.is_error()).collect()
}

fn has_code(diags: &[DomainDiagnostic], code: &str) -> bool {
    diags.iter().any(|d| d.code == code)
}

// ---------------------------------------------------------------------------
// Tax law.
// ---------------------------------------------------------------------------

#[test]
fn test_tax_parse_bracket_with_rate() {
    let domain = TaxLawDomain;
    let cond = domain
        .parse_condition("BRACKET income FROM 0 TO 9875 RATE 10")
        .expect("parse bracket");
    match cond {
        ConditionNode::And(between, rate) => {
            assert!(matches!(
                between.as_ref(),
                ConditionNode::Between { field, .. } if field == "income"
            ));
            assert!(matches!(
                rate.as_ref(),
                ConditionNode::Comparison { field, value, .. }
                    if field == "rate" && *value == ConditionValue::Number(10)
            ));
        }
        other => panic!("expected AND(between, rate), got {other:?}"),
    }
}

#[test]
fn test_tax_parse_rate_with_operator() {
    let domain = TaxLawDomain;
    let cond = domain.parse_condition("RATE >= 22").expect("parse rate");
    assert_eq!(
        cond,
        ConditionNode::Comparison {
            field: "rate".to_string(),
            operator: ">=".to_string(),
            value: ConditionValue::Number(22),
        }
    );
}

#[test]
fn test_tax_parse_fractional_rate() {
    let domain = TaxLawDomain;
    let cond = domain.parse_condition("RATE 12.5").expect("parse rate");
    match cond {
        ConditionNode::Comparison { value, .. } => {
            assert_eq!(value, ConditionValue::String("12.5".to_string()));
        }
        other => panic!("expected comparison, got {other:?}"),
    }
}

#[test]
fn test_tax_validate_rate_out_of_range() {
    let domain = TaxLawDomain;
    let s = statute_with(
        "t",
        vec![ConditionNode::Comparison {
            field: "rate".to_string(),
            operator: "==".to_string(),
            value: ConditionValue::Number(150),
        }],
        vec![],
    );
    let diags = domain.validate_statute(&s);
    assert!(has_code(&diags, "tax.rate-out-of-range"));
    assert_eq!(errors(&diags).len(), 1);
}

#[test]
fn test_tax_validate_inverted_bracket() {
    let domain = TaxLawDomain;
    let s = statute_with(
        "t",
        vec![ConditionNode::Between {
            field: "income".to_string(),
            min: ConditionValue::Number(100),
            max: ConditionValue::Number(10),
        }],
        vec![],
    );
    let diags = domain.validate_statute(&s);
    assert!(has_code(&diags, "tax.bracket-inverted"));
}

#[test]
fn test_tax_validate_overlapping_brackets() {
    let domain = TaxLawDomain;
    let s = statute_with(
        "t",
        vec![
            ConditionNode::Between {
                field: "income".to_string(),
                min: ConditionValue::Number(0),
                max: ConditionValue::Number(100),
            },
            ConditionNode::Between {
                field: "income".to_string(),
                min: ConditionValue::Number(50),
                max: ConditionValue::Number(200),
            },
        ],
        vec![],
    );
    let diags = domain.validate_statute(&s);
    assert!(has_code(&diags, "tax.brackets-overlap"));
}

// ---------------------------------------------------------------------------
// Criminal law.
// ---------------------------------------------------------------------------

#[test]
fn test_criminal_parse_mens_rea() {
    let domain = CriminalLawDomain;
    let cond = domain.parse_condition("MENS_REA intent").expect("parse");
    assert_eq!(
        cond,
        ConditionNode::Comparison {
            field: "mens_rea".to_string(),
            operator: "==".to_string(),
            value: ConditionValue::String("intent".to_string()),
        }
    );
}

#[test]
fn test_criminal_parse_penalty_range() {
    let domain = CriminalLawDomain;
    let cond = domain
        .parse_condition("PENALTY_RANGE 12 TO 24 months")
        .expect("parse");
    assert_eq!(
        cond,
        ConditionNode::Between {
            field: "penalty_months".to_string(),
            min: ConditionValue::Number(12),
            max: ConditionValue::Number(24),
        }
    );
}

#[test]
fn test_criminal_validate_complete_offence() {
    let domain = CriminalLawDomain;
    let s = statute_with(
        "c",
        vec![
            domain
                .parse_condition("ACTUS_REUS \"took property\"")
                .unwrap(),
            domain.parse_condition("MENS_REA intent").unwrap(),
        ],
        vec![],
    );
    let diags = domain.validate_statute(&s);
    assert!(errors(&diags).is_empty(), "diags: {diags:?}");
    assert!(!has_code(&diags, "criminal.missing-actus-reus"));
    assert!(!has_code(&diags, "criminal.missing-mens-rea"));
}

#[test]
fn test_criminal_validate_missing_elements() {
    let domain = CriminalLawDomain;
    let s = statute_with(
        "c",
        vec![ConditionNode::HasAttribute { key: "x".into() }],
        vec![],
    );
    let diags = domain.validate_statute(&s);
    assert!(has_code(&diags, "criminal.missing-actus-reus"));
    assert!(has_code(&diags, "criminal.missing-mens-rea"));
}

#[test]
fn test_criminal_validate_unknown_mens_rea() {
    let domain = CriminalLawDomain;
    let s = statute_with(
        "c",
        vec![ConditionNode::Comparison {
            field: "mens_rea".to_string(),
            operator: "==".to_string(),
            value: ConditionValue::String("malice".to_string()),
        }],
        vec![],
    );
    let diags = domain.validate_statute(&s);
    assert!(has_code(&diags, "criminal.unknown-mens-rea"));
}

#[test]
fn test_criminal_validate_inverted_penalty() {
    let domain = CriminalLawDomain;
    let s = statute_with(
        "c",
        vec![ConditionNode::Between {
            field: "penalty_months".to_string(),
            min: ConditionValue::Number(24),
            max: ConditionValue::Number(12),
        }],
        vec![],
    );
    let diags = domain.validate_statute(&s);
    assert!(has_code(&diags, "criminal.penalty-range-inverted"));
}

// ---------------------------------------------------------------------------
// Environmental.
// ---------------------------------------------------------------------------

#[test]
fn test_environmental_parse_emission_limit() {
    let domain = EnvironmentalDomain;
    let cond = domain
        .parse_condition("EMISSION_LIMIT co2 <= 100 mgm3")
        .expect("parse");
    assert_eq!(
        cond,
        ConditionNode::Comparison {
            field: "emission_co2".to_string(),
            operator: "<=".to_string(),
            value: ConditionValue::Number(100),
        }
    );
}

#[test]
fn test_environmental_parse_reporting_period() {
    let domain = EnvironmentalDomain;
    let cond = domain
        .parse_condition("REPORTING_PERIOD 3 months")
        .expect("parse");
    assert_eq!(
        cond,
        ConditionNode::Comparison {
            field: "reporting_period_days".to_string(),
            operator: "==".to_string(),
            value: ConditionValue::Number(90),
        }
    );
}

#[test]
fn test_environmental_validate_negative_limit() {
    let domain = EnvironmentalDomain;
    let s = statute_with(
        "e",
        vec![ConditionNode::Comparison {
            field: "emission_nox".to_string(),
            operator: "<=".to_string(),
            value: ConditionValue::Number(-5),
        }],
        vec![],
    );
    let diags = domain.validate_statute(&s);
    assert!(has_code(&diags, "environmental.negative-limit"));
}

#[test]
fn test_environmental_validate_bad_reporting_period() {
    let domain = EnvironmentalDomain;
    let s = statute_with(
        "e",
        vec![ConditionNode::Comparison {
            field: "reporting_period_days".to_string(),
            operator: "==".to_string(),
            value: ConditionValue::Number(0),
        }],
        vec![],
    );
    let diags = domain.validate_statute(&s);
    assert!(has_code(&diags, "environmental.invalid-reporting-period"));
}

// ---------------------------------------------------------------------------
// Financial.
// ---------------------------------------------------------------------------

#[test]
fn test_financial_parse_capital_ratio() {
    let domain = FinancialServicesDomain;
    let cond = domain.parse_condition("CAPITAL_RATIO >= 8").expect("parse");
    assert_eq!(
        cond,
        ConditionNode::Comparison {
            field: "ratio_capital".to_string(),
            operator: ">=".to_string(),
            value: ConditionValue::Number(8),
        }
    );
}

#[test]
fn test_financial_parse_named_ratio() {
    let domain = FinancialServicesDomain;
    let cond = domain.parse_condition("RATIO lcr >= 100").expect("parse");
    assert!(matches!(
        cond,
        ConditionNode::Comparison { ref field, .. } if field == "ratio_lcr"
    ));
}

#[test]
fn test_financial_validate_negative_ratio() {
    let domain = FinancialServicesDomain;
    let s = statute_with(
        "f",
        vec![ConditionNode::Comparison {
            field: "ratio_capital".to_string(),
            operator: ">=".to_string(),
            value: ConditionValue::Number(-3),
        }],
        vec![],
    );
    let diags = domain.validate_statute(&s);
    assert!(has_code(&diags, "financial.ratio-negative"));
}

#[test]
fn test_financial_validate_unknown_ratio_info() {
    let domain = FinancialServicesDomain;
    let s = statute_with(
        "f",
        vec![ConditionNode::Comparison {
            field: "ratio_madeup".to_string(),
            operator: ">=".to_string(),
            value: ConditionValue::Number(10),
        }],
        vec![],
    );
    let diags = domain.validate_statute(&s);
    assert!(has_code(&diags, "financial.unknown-ratio"));
    assert!(errors(&diags).is_empty());
}

// ---------------------------------------------------------------------------
// Healthcare.
// ---------------------------------------------------------------------------

#[test]
fn test_healthcare_parse_consent_and_retention() {
    let domain = HealthcareDomain;
    let consent = domain.parse_condition("CONSENT explicit").expect("parse");
    assert_eq!(
        consent,
        ConditionNode::Comparison {
            field: "consent".to_string(),
            operator: "==".to_string(),
            value: ConditionValue::String("explicit".to_string()),
        }
    );
    let retention = domain.parse_condition("RETENTION 7 years").expect("parse");
    assert_eq!(
        retention,
        ConditionNode::Comparison {
            field: "retention_days".to_string(),
            operator: "==".to_string(),
            value: ConditionValue::Number(2555),
        }
    );
}

#[test]
fn test_healthcare_validate_unknown_consent() {
    let domain = HealthcareDomain;
    let s = statute_with(
        "h",
        vec![ConditionNode::Comparison {
            field: "consent".to_string(),
            operator: "==".to_string(),
            value: ConditionValue::String("maybe".to_string()),
        }],
        vec![],
    );
    let diags = domain.validate_statute(&s);
    assert!(has_code(&diags, "healthcare.unknown-consent"));
}

#[test]
fn test_healthcare_validate_weak_consent_for_phi() {
    let domain = HealthcareDomain;
    let s = statute_with(
        "h",
        vec![
            domain.parse_condition("DATA_CATEGORY phi").unwrap(),
            domain.parse_condition("CONSENT implied").unwrap(),
        ],
        vec![],
    );
    let diags = domain.validate_statute(&s);
    assert!(has_code(
        &diags,
        "healthcare.weak-consent-for-protected-data"
    ));
}

#[test]
fn test_healthcare_validate_retention_zero() {
    let domain = HealthcareDomain;
    let s = statute_with(
        "h",
        vec![ConditionNode::Comparison {
            field: "retention_days".to_string(),
            operator: "==".to_string(),
            value: ConditionValue::Number(0),
        }],
        vec![],
    );
    let diags = domain.validate_statute(&s);
    assert!(has_code(&diags, "healthcare.invalid-retention"));
}

// ---------------------------------------------------------------------------
// Registry + tagging.
// ---------------------------------------------------------------------------

#[test]
fn test_builtin_registry_has_all_domains() {
    let registry = builtin_registry();
    let names = registry.names();
    for expected in [
        "criminal",
        "environmental",
        "financial",
        "healthcare",
        "tax",
    ] {
        assert!(names.contains(&expected.to_string()), "missing {expected}");
    }
}

#[test]
fn test_tag_statute_and_read_tag() {
    let s = statute_with("s", vec![], vec![]);
    assert_eq!(domain_tag(&s), None);
    let tagged = tag_statute(&s, "tax");
    assert_eq!(domain_tag(&tagged), Some("tax".to_string()));
    assert!(is_tagged_with(&tagged, "TAX"));
    // Re-tagging updates in place (no duplicate DEFAULT).
    let retagged = tag_statute(&tagged, "financial");
    assert_eq!(domain_tag(&retagged), Some("financial".to_string()));
    assert_eq!(
        retagged
            .defaults
            .iter()
            .filter(|d| d.field == "domain")
            .count(),
        1
    );
}

#[test]
fn test_registry_validates_only_tagged_statutes() {
    let registry = builtin_registry();
    // Untagged statute with an out-of-range rate -> no domain diagnostics.
    let untagged = statute_with(
        "u",
        vec![ConditionNode::Comparison {
            field: "rate".to_string(),
            operator: "==".to_string(),
            value: ConditionValue::Number(150),
        }],
        vec![],
    );
    assert!(registry.validate_statute(&untagged).is_empty());

    // Tag it as tax -> the rate violation surfaces, attributed to the statute.
    let tagged = tag_statute(&untagged, "tax");
    let diags = registry.validate_statute(&tagged);
    assert!(has_code(&diags, "tax.rate-out-of-range"));
    assert_eq!(diags[0].statute_id.as_deref(), Some("u"));
}

#[test]
fn test_registry_validate_document() {
    let registry = builtin_registry();
    let doc = LegalDocument {
        namespace: None,
        imports: vec![],
        exports: vec![],
        statutes: vec![
            tag_statute(
                &statute_with(
                    "good",
                    vec![ConditionNode::Comparison {
                        field: "rate".to_string(),
                        operator: "==".to_string(),
                        value: ConditionValue::Number(20),
                    }],
                    vec![],
                ),
                "tax",
            ),
            tag_statute(
                &statute_with(
                    "bad",
                    vec![ConditionNode::Comparison {
                        field: "rate".to_string(),
                        operator: "==".to_string(),
                        value: ConditionValue::Number(250),
                    }],
                    vec![],
                ),
                "tax",
            ),
        ],
    };
    let diags = registry.validate_document(&doc);
    assert_eq!(errors(&diags).len(), 1);
    assert_eq!(diags[0].statute_id.as_deref(), Some("bad"));
}

#[test]
fn test_registry_unknown_domain_parse_errors() {
    let registry = builtin_registry();
    assert!(registry.parse_condition("nonexistent", "X").is_err());
}

#[test]
fn test_vocabulary_exposed() {
    let domain = TaxLawDomain;
    let vocab = domain.vocabulary();
    assert_eq!(vocab.domain, "tax");
    assert!(vocab.keywords.iter().any(|k| k.keyword == "BRACKET"));
    assert!(!vocab.operators.is_empty());
}

// ---------------------------------------------------------------------------
// Integration: base grammar untouched + domain condition round-trips.
// ---------------------------------------------------------------------------

#[test]
fn test_domain_tag_parses_as_plain_default() {
    // A `DEFAULT domain "tax"` tag is ordinary base syntax and must parse.
    let source = r#"
        STATUTE s: "Tagged" {
            DEFAULT domain "tax"
            WHEN rate == 20
            THEN GRANT "ok"
        }
    "#;
    let parser = crate::LegalDslParser::new();
    let doc = parser.parse_document(source).expect("parse tagged statute");
    assert_eq!(domain_tag(&doc.statutes[0]), Some("tax".to_string()));
}

#[test]
fn test_domain_condition_roundtrips_through_printer() {
    let domain = TaxLawDomain;
    let cond = domain
        .parse_condition("BRACKET income FROM 0 TO 9875 RATE 10")
        .expect("parse");

    let s = StatuteNode {
        id: "bracket_rule".to_string(),
        title: "Bracket".to_string(),
        visibility: Visibility::Private,
        conditions: vec![cond.clone()],
        effects: vec![EffectNode {
            effect_type: "grant".to_string(),
            description: "tax".to_string(),
            parameters: vec![],
        }],
        discretion: None,
        exceptions: vec![],
        amendments: vec![],
        supersedes: vec![],
        defaults: vec![DefaultNode {
            field: "domain".to_string(),
            value: ConditionValue::String("tax".to_string()),
        }],
        requires: vec![],
        delegates: vec![],
        scope: None,
        constraints: vec![],
        priority: None,
    };
    let doc = LegalDocument {
        namespace: None,
        imports: vec![],
        exports: vec![],
        statutes: vec![s],
    };

    let printed = crate::printer::format_document(&doc);
    let parser = crate::LegalDslParser::new();
    let reparsed = parser.parse_document(&printed).expect("reparse");
    assert_eq!(domain_tag(&reparsed.statutes[0]), Some("tax".to_string()));
    // The tax domain still validates the reparsed statute cleanly.
    let registry = builtin_registry();
    assert!(errors(&registry.validate_statute(&reparsed.statutes[0])).is_empty());
}
