//! Tests for the streaming / incremental verification module.

use super::*;
use crate::compliance_frameworks::{ComplianceFrameworkEvaluator, ComplianceFrameworkKind};
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

fn statute(id: &str, title: &str, etype: EffectType, desc: &str) -> Statute {
    Statute::new(id, title, Effect::new(etype, desc))
}

fn statute_in(id: &str, title: &str, etype: EffectType, desc: &str, jurisdiction: &str) -> Statute {
    let mut s = statute(id, title, etype, desc);
    s.jurisdiction = Some(jurisdiction.to_string());
    s
}

/// Two statutes in the same jurisdiction with contradictory effects and an
/// overlapping age precondition — these should be detected as conflicting by the
/// existing `detect_statute_conflicts` engine.
fn conflicting_pair() -> (Statute, Statute) {
    let mut a = statute_in(
        "GUN-A",
        "Permit firearm carry for adults",
        EffectType::Grant,
        "Grant the right to carry a firearm to adult residents",
        "US-CA",
    );
    a.preconditions
        .push(Condition::age(ComparisonOp::GreaterOrEqual, 18));

    let mut b = statute_in(
        "GUN-B",
        "Revoke firearm carry for adults",
        EffectType::Revoke,
        "Revoke the right to carry a firearm from adult residents",
        "US-CA",
    );
    b.preconditions
        .push(Condition::age(ComparisonOp::GreaterOrEqual, 18));
    (a, b)
}

// ---------------------------------------------------------------------------
// StreamEvent
// ---------------------------------------------------------------------------

#[test]
fn test_stream_event_constructors() {
    let s = statute("A", "A", EffectType::Grant, "Grant access to the system");
    let add = StreamEvent::add(s.clone());
    assert_eq!(add.kind, StreamEventKind::Add);
    assert_eq!(add.statute_id(), "A");

    let modify = StreamEvent::modify(s.clone());
    assert_eq!(modify.kind, StreamEventKind::Modify);

    let remove = StreamEvent::remove(s).with_sequence(7);
    assert_eq!(remove.kind, StreamEventKind::Remove);
    assert_eq!(remove.sequence, Some(7));
}

#[test]
fn test_stream_event_kind_display() {
    assert_eq!(StreamEventKind::Add.to_string(), "Add");
    assert_eq!(StreamEventKind::Modify.to_string(), "Modify");
    assert_eq!(StreamEventKind::Remove.to_string(), "Remove");
}

// ---------------------------------------------------------------------------
// StreamingVerifier basic lifecycle
// ---------------------------------------------------------------------------

#[test]
fn test_streaming_verifier_add() {
    let mut v = StreamingVerifier::new();
    let s = statute(
        "TAX-1",
        "Tax",
        EffectType::Grant,
        "Grant a tax credit to qualifying residents",
    );
    let out = v.apply(StreamEvent::add(s));
    assert_eq!(out.kind, StreamEventKind::Add);
    assert!(out.corpus_changed);
    assert_eq!(out.corpus_size, 1);
    assert_eq!(v.corpus_size(), 1);
    assert_eq!(v.events_processed(), 1);
    assert!(out.statute_result.is_some());
}

#[test]
fn test_streaming_verifier_modify_changes_result() {
    let mut v = StreamingVerifier::new();
    let s = statute(
        "X",
        "Original",
        EffectType::Grant,
        "Grant a benefit to eligible persons",
    );
    v.apply(StreamEvent::add(s));

    let modified = statute(
        "X",
        "Updated title",
        EffectType::Obligation,
        "Require eligible persons to file an annual report",
    );
    let out = v.apply(StreamEvent::modify(modified));
    assert!(out.corpus_changed);
    assert_eq!(out.kind, StreamEventKind::Modify);
    assert_eq!(v.corpus_size(), 1);
}

#[test]
fn test_streaming_verifier_modify_noop_detected() {
    let mut v = StreamingVerifier::new();
    let s = statute(
        "X",
        "Title",
        EffectType::Grant,
        "Grant a benefit to eligible persons",
    );
    v.apply(StreamEvent::add(s.clone()));

    // Apply an identical modify - should be a no-op.
    let out = v.apply(StreamEvent::modify(s));
    assert!(!out.corpus_changed);
    assert!(out.impact.is_none());
}

#[test]
fn test_streaming_verifier_add_existing_treated_as_modify() {
    let mut v = StreamingVerifier::new();
    let s = statute(
        "X",
        "Title",
        EffectType::Grant,
        "Grant a benefit to eligible persons",
    );
    v.apply(StreamEvent::add(s));

    let s2 = statute(
        "X",
        "Title 2",
        EffectType::Grant,
        "Grant a different benefit to eligible persons",
    );
    let out = v.apply(StreamEvent::add(s2));
    // Reported as Modify because the id already existed.
    assert_eq!(out.kind, StreamEventKind::Modify);
    assert_eq!(v.corpus_size(), 1);
}

#[test]
fn test_streaming_verifier_remove() {
    let mut v = StreamingVerifier::new();
    let s = statute(
        "X",
        "Title",
        EffectType::Grant,
        "Grant a benefit to eligible persons",
    );
    v.apply(StreamEvent::add(s.clone()));
    assert_eq!(v.corpus_size(), 1);

    let out = v.apply(StreamEvent::remove(s));
    assert!(out.corpus_changed);
    assert_eq!(out.corpus_size, 0);
    assert!(out.statute_result.is_none());
    assert_eq!(v.corpus_size(), 0);
}

#[test]
fn test_streaming_verifier_remove_absent_is_noop() {
    let mut v = StreamingVerifier::new();
    let s = statute("GHOST", "Ghost", EffectType::Grant, "Grant nothing at all");
    let out = v.apply(StreamEvent::remove(s));
    assert!(!out.corpus_changed);
    assert_eq!(out.corpus_size, 0);
}

#[test]
fn test_streaming_verifier_batch() {
    let mut v = StreamingVerifier::new();
    let events = vec![
        StreamEvent::add(statute(
            "A",
            "A",
            EffectType::Grant,
            "Grant access to resource A for members",
        )),
        StreamEvent::add(statute(
            "B",
            "B",
            EffectType::Grant,
            "Grant access to resource B for members",
        )),
        StreamEvent::add(statute(
            "C",
            "C",
            EffectType::Grant,
            "Grant access to resource C for members",
        )),
    ];
    let outcomes = v.apply_batch(events);
    assert_eq!(outcomes.len(), 3);
    assert_eq!(v.corpus_size(), 3);
    assert_eq!(v.events_processed(), 3);
}

// ---------------------------------------------------------------------------
// Incremental conflict detection inside the streaming verifier
// ---------------------------------------------------------------------------

#[test]
fn test_streaming_verifier_detects_conflict_on_add() {
    let (a, b) = conflicting_pair();
    let mut v = StreamingVerifier::new();
    let out_a = v.apply(StreamEvent::add(a));
    assert!(
        out_a.new_conflicts.is_empty(),
        "single statute, no conflict yet"
    );

    let out_b = v.apply(StreamEvent::add(b));
    assert!(
        !out_b.new_conflicts.is_empty(),
        "adding contradictory statute should surface a conflict"
    );
    assert!(!out_b.is_clean());
}

#[test]
fn test_streaming_verifier_conflict_cleared_on_remove() {
    let (a, b) = conflicting_pair();
    let mut v = StreamingVerifier::new();
    v.apply(StreamEvent::add(a));
    v.apply(StreamEvent::add(b.clone()));
    assert!(!v.conflicts().is_empty());

    v.apply(StreamEvent::remove(b));
    assert!(
        v.conflicts().is_empty(),
        "removing a party to a conflict should clear it"
    );
}

#[test]
fn test_streaming_verifier_conflicts_disabled() {
    let (a, b) = conflicting_pair();
    let mut v = StreamingVerifier::with_config(StreamingConfig {
        detect_conflicts: false,
        ..Default::default()
    });
    v.apply(StreamEvent::add(a));
    let out = v.apply(StreamEvent::add(b));
    assert!(out.new_conflicts.is_empty());
    assert!(v.conflicts().is_empty());
}

// ---------------------------------------------------------------------------
// Incremental impact analysis inside the streaming verifier
// ---------------------------------------------------------------------------

#[test]
fn test_streaming_verifier_impact_on_modify() {
    let mut v = StreamingVerifier::new();

    // Base statute referenced by a dependent statute.
    let base = statute(
        "BASE",
        "Base rule",
        EffectType::Grant,
        "Grant a license to operate",
    );
    v.apply(StreamEvent::add(base));

    let mut dependent = statute(
        "DEP",
        "Dependent rule",
        EffectType::Obligation,
        "Require holders to comply with the base licensing rule",
    );
    // The reference extractor recognizes the `statute:<id>` custom-condition form.
    dependent.preconditions.push(Condition::Custom {
        description: "statute:BASE".to_string(),
    });
    v.apply(StreamEvent::add(dependent));

    // Modify BASE's effect - dependents make this a higher-impact change.
    let base_modified = statute(
        "BASE",
        "Base rule",
        EffectType::Prohibition,
        "Prohibit operating without a license",
    );
    let out = v.apply(StreamEvent::modify(base_modified));
    let impact = out.impact.expect("modify should produce impact");
    assert_eq!(impact.statute_id, "BASE");
    assert!(
        impact.affected_statutes.contains(&"DEP".to_string()),
        "DEP references BASE and should be flagged as affected"
    );
}

#[test]
fn test_streaming_verifier_no_impact_on_add() {
    let mut v = StreamingVerifier::new();
    let out = v.apply(StreamEvent::add(statute(
        "A",
        "A",
        EffectType::Grant,
        "Grant access to resource A",
    )));
    assert!(out.impact.is_none());
}

#[test]
fn test_streaming_verify_full_matches_corpus() {
    let mut v = StreamingVerifier::new();
    v.apply(StreamEvent::add(statute(
        "A",
        "A",
        EffectType::Grant,
        "Grant access to resource A for members",
    )));
    v.apply(StreamEvent::add(statute(
        "B",
        "B",
        EffectType::Grant,
        "Grant access to resource B for members",
    )));
    let full = v.verify_full();
    // No cross-statute issues among two independent grants.
    assert!(full.passed);
    assert_eq!(v.snapshot().len(), 2);
}

#[test]
fn test_streaming_verifier_report() {
    let (a, b) = conflicting_pair();
    let mut v = StreamingVerifier::new();
    v.apply(StreamEvent::add(a));
    v.apply(StreamEvent::add(b));
    let report = v.report();
    assert!(report.contains("Streaming Verification Report"));
    assert!(report.contains("Corpus size: 2"));
    assert!(report.contains("Active Conflicts"));
}

// ---------------------------------------------------------------------------
// IncrementalConflictDetector (standalone)
// ---------------------------------------------------------------------------

#[test]
fn test_incremental_conflict_detector_upsert() {
    let (a, b) = conflicting_pair();
    let mut d = IncrementalConflictDetector::new();
    let c1 = d.upsert(a);
    assert!(c1.is_empty());
    let c2 = d.upsert(b);
    assert!(!c2.is_empty());
    assert_eq!(d.corpus_size(), 2);
    assert!(!d.conflicts().is_empty());
}

#[test]
fn test_incremental_conflict_detector_seed_and_remove() {
    let (a, b) = conflicting_pair();
    let mut d = IncrementalConflictDetector::new();
    d.seed(&[a, b.clone()]);
    assert!(!d.conflicts().is_empty());

    d.remove(&b.id);
    assert!(d.conflicts().is_empty());
    assert_eq!(d.corpus_size(), 1);
}

#[test]
fn test_incremental_conflict_detector_matches_batch() {
    // The incremental result should agree with a full batch detection on the
    // *number* of conflicts for a small corpus.
    let (a, b) = conflicting_pair();
    let batch = detect_statute_conflicts(&[a.clone(), b.clone()]);

    let mut d = IncrementalConflictDetector::new();
    d.upsert(a);
    d.upsert(b);

    // Both should find at least the effect conflict; incremental may not
    // re-report id-collisions (distinct ids here), so compare effect conflicts.
    let batch_effect = batch
        .iter()
        .filter(|c| c.conflict_type == ConflictType::EffectConflict)
        .count();
    let inc_effect = d
        .conflicts()
        .iter()
        .filter(|c| c.conflict_type == ConflictType::EffectConflict)
        .count();
    assert_eq!(batch_effect, inc_effect);
    assert!(batch_effect >= 1);
}

// ---------------------------------------------------------------------------
// IncrementalImpactAnalyzer (standalone)
// ---------------------------------------------------------------------------

#[test]
fn test_incremental_impact_analyzer_assess() {
    let mut a = IncrementalImpactAnalyzer::new();
    let base = statute(
        "BASE",
        "Base",
        EffectType::Grant,
        "Grant a license to operate the facility",
    );
    let mut dep = statute(
        "DEP",
        "Dep",
        EffectType::Obligation,
        "Require compliance with the base licensing rule",
    );
    // The reference extractor recognizes the `statute:<id>` custom-condition form.
    dep.preconditions.push(Condition::Custom {
        description: "statute:BASE".to_string(),
    });
    a.seed(&[base, dep]);
    assert_eq!(a.corpus_size(), 2);

    let new_base = statute(
        "BASE",
        "Base",
        EffectType::Revoke,
        "Revoke the operating license",
    );
    let impact = a.assess_modification(new_base).expect("base exists");
    assert_eq!(impact.statute_id, "BASE");
    assert!(impact.affected_statutes.contains(&"DEP".to_string()));
}

#[test]
fn test_incremental_impact_analyzer_preview_does_not_commit() {
    let mut a = IncrementalImpactAnalyzer::new();
    let base = statute(
        "BASE",
        "Base",
        EffectType::Grant,
        "Grant a license to operate the facility",
    );
    a.seed(&[base]);

    let new_base = statute("BASE", "Base", EffectType::Revoke, "Revoke the license");
    let preview = a.preview_modification(&new_base);
    assert!(preview.is_some());
    // Preview must not mutate corpus content: assessing the same change again
    // should still detect the original (Grant) -> Revoke transition.
    let impact = a.assess_modification(new_base).expect("still present");
    assert!(
        impact
            .changes
            .iter()
            .any(|c| matches!(c, crate::StatuteChange::EffectChanged { .. })),
        "effect change should still be detected after a preview"
    );
}

#[test]
fn test_incremental_impact_analyzer_unknown_id() {
    let mut a = IncrementalImpactAnalyzer::new();
    let unknown = statute("NOPE", "Nope", EffectType::Grant, "Grant something new");
    assert!(a.assess_modification(unknown).is_none());
}

// ---------------------------------------------------------------------------
// ContinuousComplianceMonitor
// ---------------------------------------------------------------------------

/// HIPAA evaluator with the default (strict) threshold.
fn hipaa_evaluator() -> Box<dyn ComplianceEvaluator + Send + Sync> {
    Box::new(ComplianceFrameworkEvaluator::new(
        ComplianceFrameworkKind::Hipaa,
    ))
}

/// HIPAA evaluator with a permissive threshold so that a corpus with *some*
/// coverage is considered compliant. This lets tests deterministically drive
/// compliant -> non-compliant transitions.
fn lenient_hipaa_evaluator(threshold: f64) -> Box<dyn ComplianceEvaluator + Send + Sync> {
    Box::new(
        ComplianceFrameworkEvaluator::new(ComplianceFrameworkKind::Hipaa).with_threshold(threshold),
    )
}

/// A statute that provides broad HIPAA evidence (audit, access, encryption,
/// incident, retention, consent) so that, under a lenient threshold, the corpus
/// is compliant.
fn hipaa_strong_statute(id: &str) -> Statute {
    statute_in(
        id,
        "Comprehensive health data safeguard",
        EffectType::Obligation,
        "Require risk assessment, role based access authentication, encryption of \
         transmission, audit log and monitor, breach incident notification, data \
         retention period, and patient consent for disclosure of medical records",
        "US-CA",
    )
}

#[test]
fn test_continuous_monitor_registration() {
    let mut m = ContinuousComplianceMonitor::new();
    assert_eq!(m.evaluator_count(), 0);
    m.register_evaluator(hipaa_evaluator());
    assert_eq!(m.evaluator_count(), 1);
}

#[test]
fn test_continuous_monitor_initial_noncompliance_alerts() {
    // With the strict default threshold, a sparse corpus is non-compliant on its
    // first evaluation and the monitor surfaces a warning - this is the correct
    // contract for a framework with many unmet requirements.
    let mut m = ContinuousComplianceMonitor::new();
    m.register_evaluator(hipaa_evaluator());

    let s = statute_in(
        "ROAD-1",
        "Road maintenance",
        EffectType::Obligation,
        "Require the department to maintain public roads annually",
        "US-CA",
    );
    let alerts = m.apply(StreamEvent::add(s));
    assert!(
        alerts.iter().any(|a| a.framework == "HIPAA"),
        "initial non-compliance should be reported"
    );
}

#[test]
fn test_continuous_monitor_compliant_stays_quiet() {
    // Under a lenient threshold, a strong statute makes the corpus compliant;
    // adding another harmless (non-regressing) statute should not raise alerts.
    let mut m = ContinuousComplianceMonitor::new();
    m.register_evaluator(lenient_hipaa_evaluator(5.0));

    let first = m.apply(StreamEvent::add(hipaa_strong_statute("HIP-1")));
    assert!(
        first.is_empty(),
        "a compliant initial corpus should not alert, got {:?}",
        first
    );

    // Adding more coverage cannot reduce the score, so no regression alert.
    let more = m.apply(StreamEvent::add(statute_in(
        "HIP-2",
        "Encryption rule",
        EffectType::Obligation,
        "Require encryption of stored medical records and audit log access",
        "US-CA",
    )));
    assert!(
        more.is_empty(),
        "adding coverage must not alert, got {:?}",
        more
    );
}

#[test]
fn test_continuous_monitor_emits_alert_on_regression() {
    // Build a compliant corpus (lenient threshold), then remove the statute that
    // provided the coverage. Compliance is lost -> a critical alert must fire.
    let mut m = ContinuousComplianceMonitor::new();
    m.register_evaluator(lenient_hipaa_evaluator(5.0));

    let strong = hipaa_strong_statute("HIP-1");
    let init = m.apply(StreamEvent::add(strong.clone()));
    assert!(init.is_empty(), "initial compliant corpus should be quiet");

    let alerts = m.apply(StreamEvent::remove(strong));
    assert!(
        !alerts.is_empty(),
        "removing the sole compliant statute must raise an alert"
    );
    assert!(
        alerts
            .iter()
            .any(|a| a.framework == "HIPAA" && a.severity == Severity::Critical),
        "loss of compliance should be Critical, got {:?}",
        alerts
    );
}

#[test]
fn test_continuous_monitor_snapshots_recorded() {
    let mut m = ContinuousComplianceMonitor::new();
    m.register_evaluator(hipaa_evaluator());
    m.apply(StreamEvent::add(statute_in(
        "S",
        "S",
        EffectType::Grant,
        "Grant a generic permission",
        "US-CA",
    )));
    let snaps = m.snapshots();
    assert!(snaps.contains_key("HIPAA"));
    assert!(snaps["HIPAA"].score >= 0.0);
}

#[test]
fn test_continuous_monitor_remove_statute() {
    let mut m = ContinuousComplianceMonitor::new();
    m.register_evaluator(hipaa_evaluator());
    let s = statute_in(
        "S",
        "S",
        EffectType::Grant,
        "Grant a generic permission",
        "US-CA",
    );
    m.apply(StreamEvent::add(s.clone()));
    assert_eq!(m.corpus_size(), 1);
    m.apply(StreamEvent::remove(s));
    assert_eq!(m.corpus_size(), 0);
}

#[test]
fn test_continuous_monitor_regression_threshold_severity() {
    // A score drop that does not cross the compliance line should produce a
    // Warning (small drop) or Error (large drop) rather than Critical.
    let mut m = ContinuousComplianceMonitor::new().with_regression_error_threshold(5.0);
    m.register_evaluator(lenient_hipaa_evaluator(0.0)); // always "compliant"

    m.apply(StreamEvent::add(hipaa_strong_statute("HIP-1")));
    // Remove coverage: score drops to 0 but threshold 0.0 keeps it "compliant",
    // so this is a score-regression (not a compliance loss).
    let alerts = m.apply(StreamEvent::remove(hipaa_strong_statute("HIP-1")));
    assert!(!alerts.is_empty());
    assert!(
        alerts
            .iter()
            .all(|a| a.severity == Severity::Warning || a.severity == Severity::Error),
        "score regression without compliance loss must not be Critical, got {:?}",
        alerts
    );
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

#[test]
fn test_worst_conflict_severity() {
    let (a, b) = conflicting_pair();
    let conflicts = detect_statute_conflicts(&[a, b]);
    let worst = worst_conflict_severity(&conflicts);
    assert!(worst.is_some());
    // Effect conflicts are Critical.
    assert_eq!(worst, Some(Severity::Critical));
}

#[test]
fn test_worst_conflict_severity_empty() {
    assert!(worst_conflict_severity(&[]).is_none());
}

#[test]
fn test_conflict_type_label() {
    assert_eq!(conflict_type_label(&ConflictType::EffectConflict), "effect");
    assert_eq!(
        conflict_type_label(&ConflictType::JurisdictionalOverlap),
        "jurisdiction"
    );
    assert_eq!(
        conflict_type_label(&ConflictType::TemporalConflict),
        "temporal"
    );
    assert_eq!(
        conflict_type_label(&ConflictType::HierarchyViolation),
        "hierarchy"
    );
    assert_eq!(
        conflict_type_label(&ConflictType::IdCollision),
        "id-collision"
    );
}

#[test]
fn test_stream_outcome_critical_conflict_count() {
    let (a, b) = conflicting_pair();
    let mut v = StreamingVerifier::new();
    v.apply(StreamEvent::add(a));
    let out = v.apply(StreamEvent::add(b));
    assert!(out.critical_conflict_count() >= 1);
}

#[test]
fn test_streaming_config_default() {
    let cfg = StreamingConfig::default();
    assert!(cfg.detect_conflicts);
    assert!(cfg.analyze_impact);
    assert!(cfg.track_history);
}
