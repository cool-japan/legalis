//! Tests for the automated refactoring suite.

use super::*;
use crate::ast::{
    ConditionNode, ConditionValue, EffectNode, ExceptionNode, LegalDocument, StatuteNode,
};
use crate::module_system::Visibility;

// ---------------------------------------------------------------------------
// Test helpers.
// ---------------------------------------------------------------------------

fn cmp(field: &str, op: &str, value: i64) -> ConditionNode {
    ConditionNode::Comparison {
        field: field.to_string(),
        operator: op.to_string(),
        value: ConditionValue::Number(value),
    }
}

fn has(key: &str) -> ConditionNode {
    ConditionNode::HasAttribute {
        key: key.to_string(),
    }
}

fn and(a: ConditionNode, b: ConditionNode) -> ConditionNode {
    ConditionNode::And(Box::new(a), Box::new(b))
}

fn or(a: ConditionNode, b: ConditionNode) -> ConditionNode {
    ConditionNode::Or(Box::new(a), Box::new(b))
}

fn not(a: ConditionNode) -> ConditionNode {
    ConditionNode::Not(Box::new(a))
}

fn grant(desc: &str) -> EffectNode {
    EffectNode {
        effect_type: "grant".to_string(),
        description: desc.to_string(),
        parameters: vec![],
    }
}

fn statute(id: &str, conditions: Vec<ConditionNode>, effects: Vec<EffectNode>) -> StatuteNode {
    StatuteNode {
        id: id.to_string(),
        title: format!("Statute {id}"),
        visibility: Visibility::Private,
        conditions,
        effects,
        discretion: None,
        exceptions: vec![],
        amendments: vec![],
        supersedes: vec![],
        defaults: vec![],
        requires: vec![],
        delegates: vec![],
        scope: None,
        constraints: vec![],
        priority: None,
    }
}

fn document(statutes: Vec<StatuteNode>) -> LegalDocument {
    LegalDocument {
        namespace: None,
        imports: vec![],
        exports: vec![],
        statutes,
    }
}

// ---------------------------------------------------------------------------
// normalize_condition_structure.
// ---------------------------------------------------------------------------

#[test]
fn test_normalize_double_negation() {
    let cond = not(not(has("active")));
    let (normalized, report) = normalize_condition_structure(&cond);
    assert_eq!(normalized, has("active"));
    assert!(report.negations_pushed >= 1);
}

#[test]
fn test_normalize_de_morgan_and() {
    // NOT (a AND b) => (NOT a) OR (NOT b)
    let cond = not(and(has("a"), has("b")));
    let (normalized, _) = normalize_condition_structure(&cond);
    match &normalized {
        ConditionNode::Or(l, r) => {
            assert!(matches!(l.as_ref(), ConditionNode::Not(_)));
            assert!(matches!(r.as_ref(), ConditionNode::Not(_)));
        }
        other => panic!("expected OR of negations, got {other:?}"),
    }
}

#[test]
fn test_normalize_de_morgan_or() {
    // NOT (a OR b) => (NOT a) AND (NOT b)
    let cond = not(or(has("a"), has("b")));
    let (normalized, _) = normalize_condition_structure(&cond);
    assert!(matches!(normalized, ConditionNode::And(_, _)));
}

#[test]
fn test_normalize_flatten_and_dedup() {
    // (a AND a) AND b => a AND b  (one duplicate removed, two atoms left)
    let cond = and(and(has("a"), has("a")), has("b"));
    let (normalized, report) = normalize_condition_structure(&cond);
    let conjuncts = flatten_conjuncts(&normalized);
    assert_eq!(conjuncts.len(), 2, "got {normalized:?}");
    assert!(report.duplicates_removed >= 1);
}

#[test]
fn test_normalize_is_order_independent() {
    // (a AND b) and (b AND a) must normalize to the same canonical form.
    let c1 = and(has("alpha"), has("beta"));
    let c2 = and(has("beta"), has("alpha"));
    let (n1, _) = normalize_condition_structure(&c1);
    let (n2, _) = normalize_condition_structure(&c2);
    assert_eq!(n1, n2);
}

#[test]
fn test_normalize_idempotent() {
    let cond = not(and(or(has("a"), has("b")), not(has("c"))));
    let (once, _) = normalize_condition_structure(&cond);
    let (twice, report) = normalize_condition_structure(&once);
    assert_eq!(once, twice);
    assert!(
        report.is_noop(),
        "second pass should be a no-op: {report:?}"
    );
}

#[test]
fn test_normalize_not_inrange_folds() {
    let cond = not(ConditionNode::InRange {
        field: "age".to_string(),
        min: ConditionValue::Number(18),
        max: ConditionValue::Number(65),
        inclusive_min: true,
        inclusive_max: false,
    });
    let (normalized, _) = normalize_condition_structure(&cond);
    assert!(matches!(normalized, ConditionNode::NotInRange { .. }));
}

#[test]
fn test_normalize_statute_conditions() {
    let s = statute("s", vec![not(not(has("ok")))], vec![grant("x")]);
    let (rewritten, report) = normalize_statute_conditions(&s);
    assert_eq!(rewritten.conditions[0], has("ok"));
    assert!(!report.is_empty());
}

// ---------------------------------------------------------------------------
// extract + inline round-trip.
// ---------------------------------------------------------------------------

fn shared_condition() -> ConditionNode {
    and(cmp("age", ">=", 18), has("citizen"))
}

#[test]
fn test_extract_auto_then_inline_roundtrip() {
    let doc = document(vec![
        statute(
            "s1",
            vec![and(shared_condition(), has("extra1"))],
            vec![grant("a")],
        ),
        statute(
            "s2",
            vec![and(shared_condition(), has("extra2"))],
            vec![grant("b")],
        ),
    ]);

    let result = extract_condition(&doc, &ExtractOptions::default());
    assert!(
        !result.is_noop(),
        "should have extracted the shared condition"
    );
    assert_eq!(result.extracted.len(), 1);
    assert!(result.extracted[0].occurrences >= 2);

    // The placeholder key must appear in the rewritten document.
    let ref_key = result.extracted[0].ref_key.clone();
    let placeholder = has(&ref_key);
    let mut subs = Vec::new();
    for s in &result.document.statutes {
        for c in &s.conditions {
            collect_subconditions(c, &mut subs);
        }
    }
    assert!(subs.contains(&placeholder), "placeholder not found");

    // Inlining reproduces the original document exactly.
    let inlined = inline_condition(&result.document, &result.extracted);
    assert_eq!(inlined.document, doc, "inline(extract(doc)) must equal doc");
    assert!(inlined.substitutions >= 2);
}

#[test]
fn test_extract_targeted_named() {
    let doc = document(vec![
        statute("s1", vec![shared_condition()], vec![grant("a")]),
        statute("s2", vec![shared_condition()], vec![grant("b")]),
    ]);
    let opts = ExtractOptions::default()
        .target(shared_condition())
        .named("Adult Citizen");
    let result = extract_condition(&doc, &opts);
    assert_eq!(result.extracted.len(), 1);
    assert_eq!(result.extracted[0].name, "Adult Citizen");
    assert_eq!(result.extracted[0].ref_key, "cond_adult_citizen");

    let inlined = inline_condition(&result.document, &result.extracted);
    assert_eq!(inlined.document, doc);
}

#[test]
fn test_extract_avoids_existing_key_collision() {
    // A document that already uses `cond_adult_citizen` as a real attribute must
    // not have the extraction collide with it.
    let doc = document(vec![
        statute(
            "s1",
            vec![and(shared_condition(), has("cond_adult_citizen"))],
            vec![grant("a")],
        ),
        statute(
            "s2",
            vec![and(shared_condition(), has("cond_adult_citizen"))],
            vec![grant("b")],
        ),
    ]);
    let opts = ExtractOptions::default()
        .target(shared_condition())
        .named("Adult Citizen");
    let result = extract_condition(&doc, &opts);
    assert_ne!(result.extracted[0].ref_key, "cond_adult_citizen");

    // Inlining is still exact despite the existing same-prefixed attribute.
    let inlined = inline_condition(&result.document, &result.extracted);
    assert_eq!(inlined.document, doc);
}

#[test]
fn test_extract_noop_when_no_repetition() {
    let doc = document(vec![statute("s1", vec![has("solo")], vec![grant("a")])]);
    let result = extract_condition(&doc, &ExtractOptions::default());
    assert!(result.is_noop());
    assert_eq!(result.document, doc);
}

#[test]
fn test_inline_named_conditions_direct() {
    let doc = document(vec![statute(
        "s",
        vec![and(has("cond_ref"), has("other"))],
        vec![grant("a")],
    )]);
    let mut map = std::collections::HashMap::new();
    map.insert("cond_ref".to_string(), shared_condition());
    let result = inline_named_conditions(&doc, &map);
    assert_eq!(result.substitutions, 1);
    assert_eq!(
        result.document.statutes[0].conditions[0],
        and(shared_condition(), has("other"))
    );
}

// ---------------------------------------------------------------------------
// merge_similar_statutes.
// ---------------------------------------------------------------------------

#[test]
fn test_merge_factors_common_condition() {
    let doc = document(vec![
        statute(
            "rule_a",
            vec![and(cmp("age", ">=", 18), has("citizen"))],
            vec![grant("vote")],
        ),
        statute(
            "rule_b",
            vec![and(cmp("age", ">=", 18), has("resident"))],
            vec![grant("vote")],
        ),
    ]);
    let result = merge_similar_statutes(&doc, &MergeOptions::default());
    assert_eq!(result.merged_groups, 1);
    assert_eq!(result.document.statutes.len(), 1);

    let merged = &result.document.statutes[0];
    let conjuncts = flatten_conjuncts(
        &fold_and(merged.conditions.clone()).expect("merged should have conditions"),
    );
    // Common conjunct age>=18 is factored, the rest (HAS citizen / HAS resident)
    // become an OR.
    assert!(conjuncts.contains(&cmp("age", ">=", 18)));
    assert!(
        conjuncts
            .iter()
            .any(|c| matches!(c, ConditionNode::Or(_, _)))
    );
}

#[test]
fn test_merge_skips_different_effects() {
    let doc = document(vec![
        statute("a", vec![has("x")], vec![grant("one")]),
        statute(
            "b",
            vec![has("y")],
            vec![EffectNode {
                effect_type: "revoke".to_string(),
                description: "two".to_string(),
                parameters: vec![],
            }],
        ),
    ]);
    let result = merge_similar_statutes(&doc, &MergeOptions::default());
    assert_eq!(result.merged_groups, 0);
    assert_eq!(result.document.statutes.len(), 2);
    assert!(result.is_noop());
}

#[test]
fn test_merge_empty_rest_drops_disjunction() {
    // One member's conditions are a strict subset (empty remainder), so the merged
    // statute keeps only the common part.
    let doc = document(vec![
        statute("a", vec![cmp("age", ">=", 18)], vec![grant("vote")]),
        statute(
            "b",
            vec![and(cmp("age", ">=", 18), has("extra"))],
            vec![grant("vote")],
        ),
    ]);
    let result = merge_similar_statutes(&doc, &MergeOptions::default());
    assert_eq!(result.merged_groups, 1);
    let merged = &result.document.statutes[0];
    let conjuncts: Vec<ConditionNode> = merged
        .conditions
        .iter()
        .flat_map(flatten_conjuncts)
        .collect();
    assert_eq!(conjuncts, vec![cmp("age", ">=", 18)]);
}

// ---------------------------------------------------------------------------
// split_complex_statute.
// ---------------------------------------------------------------------------

#[test]
fn test_split_by_effect() {
    let s = statute(
        "multi",
        vec![cmp("age", ">=", 18)],
        vec![grant("vote"), grant("drive")],
    );
    let result = split_complex_statute(&s, &SplitOptions::default());
    assert_eq!(result.statutes.len(), 2);
    // Each split keeps the same single condition and exactly one effect.
    for split in &result.statutes {
        assert_eq!(split.effects.len(), 1);
        assert_eq!(split.conditions, vec![cmp("age", ">=", 18)]);
    }
    // Ids are unique and the first keeps the base id.
    assert_eq!(result.statutes[0].id, "multi");
    let ids: std::collections::BTreeSet<_> = result.statutes.iter().map(|s| s.id.clone()).collect();
    assert_eq!(ids.len(), 2);
}

#[test]
fn test_split_by_disjunction() {
    let s = statute(
        "disj",
        vec![or(has("a"), or(has("b"), has("c")))],
        vec![grant("benefit")],
    );
    let result = split_complex_statute(&s, &SplitOptions::default());
    assert_eq!(result.statutes.len(), 3);
    // Each branch carries one disjunct.
    let cond_set: std::collections::BTreeSet<String> = result
        .statutes
        .iter()
        .map(|st| format!("{:?}", st.conditions))
        .collect();
    assert_eq!(cond_set.len(), 3);
}

#[test]
fn test_split_disjunction_keeps_other_conjuncts() {
    // conditions = [age>=18, (a OR b)] -> two statutes [age>=18, a], [age>=18, b].
    let s = statute(
        "guarded",
        vec![cmp("age", ">=", 18), or(has("a"), has("b"))],
        vec![grant("x")],
    );
    let result = split_complex_statute(&s, &SplitOptions::default());
    assert_eq!(result.statutes.len(), 2);
    for split in &result.statutes {
        assert_eq!(split.conditions.len(), 2);
        assert_eq!(split.conditions[0], cmp("age", ">=", 18));
        assert!(matches!(
            split.conditions[1],
            ConditionNode::HasAttribute { .. }
        ));
    }
}

#[test]
fn test_split_noop_for_simple_statute() {
    let s = statute("simple", vec![has("x")], vec![grant("y")]);
    let result = split_complex_statute(&s, &SplitOptions::default());
    assert!(result.is_noop());
    assert_eq!(result.statutes.len(), 1);
    assert_eq!(result.statutes[0], s);
}

// ---------------------------------------------------------------------------
// Printer round-trip: a refactored document must still parse + print.
// ---------------------------------------------------------------------------

#[test]
fn test_refactor_roundtrips_through_printer() {
    let source = r#"
        STATUTE adult_a: "Adult A" {
            WHEN AGE >= 18 AND HAS citizen
            THEN GRANT "right one"
        }
        STATUTE adult_b: "Adult B" {
            WHEN AGE >= 18 AND HAS citizen
            THEN GRANT "right two"
        }
    "#;
    let parser = crate::LegalDslParser::new();
    let doc = parser.parse_document(source).expect("parse");

    // Extract the shared condition, then print + reparse the result.
    let result = extract_condition(&doc, &ExtractOptions::default());
    assert!(!result.is_noop());
    let printed = crate::printer::format_document(&result.document);
    let reparsed = parser.parse_document(&printed).expect("reparse refactored");
    assert_eq!(reparsed.statutes.len(), 2);

    // Inlining the reparsed document restores the original conditions.
    let inlined = inline_condition(&reparsed, &result.extracted);
    let normalized_original = parser.parse_document(source).expect("reparse original");
    assert_eq!(
        inlined.document.statutes.len(),
        normalized_original.statutes.len()
    );
}

#[test]
fn test_normalize_roundtrips_through_printer() {
    let source = r#"
        STATUTE s: "Normalize me" {
            WHEN NOT (HAS a AND HAS b)
            THEN GRANT "ok"
        }
    "#;
    let parser = crate::LegalDslParser::new();
    let doc = parser.parse_document(source).expect("parse");
    let (normalized, report) = normalize_document_conditions(&doc);
    assert!(!report.is_empty());
    let printed = crate::printer::format_document(&normalized);
    let reparsed = parser.parse_document(&printed).expect("reparse normalized");
    // The reparsed normalized form is itself a fixed point.
    let (renormalized, _) = normalize_document_conditions(&reparsed);
    assert_eq!(
        renormalized.statutes[0].conditions,
        normalized.statutes[0].conditions
    );
}

#[test]
fn test_report_serialization() {
    let s = statute("s", vec![not(not(has("ok")))], vec![grant("x")]);
    let (_, report) = normalize_statute_conditions(&s);
    let json = serde_json::to_string(&report).expect("serialize report");
    let back: RefactorReport = serde_json::from_str(&json).expect("deserialize report");
    assert_eq!(report, back);
}

#[test]
fn test_exception_conditions_are_refactored() {
    // Conditions inside EXCEPTION carve-outs are normalized too.
    let mut s = statute("s", vec![has("base")], vec![grant("x")]);
    s.exceptions = vec![ExceptionNode {
        conditions: vec![not(not(has("minor")))],
        description: "carve out".to_string(),
    }];
    let (rewritten, _) = normalize_statute_conditions(&s);
    assert_eq!(rewritten.exceptions[0].conditions[0], has("minor"));
}
