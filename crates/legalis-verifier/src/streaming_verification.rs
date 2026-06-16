//! Streaming / Real-Time Incremental Verification
//!
//! This module provides an *incremental*, stream-oriented verification layer that
//! sits on top of the existing batch analyses ([`StatuteVerifier`],
//! [`detect_statute_conflicts`], [`analyze_change_impact`]) without duplicating
//! their logic. It is designed for live statute feeds where statutes are added,
//! modified, or removed one event at a time and where re-running the full
//! quadratic-cost analyses on every event would be wasteful.
//!
//! The four capabilities exposed here are:
//!
//! * [`StreamingVerifier`] — maintains an internal corpus and verification state,
//!   accepts a stream of [`StreamEvent`]s, and incrementally updates the
//!   verification result. Unchanged statutes reuse their previously computed
//!   per-statute results via [`IncrementalState`].
//! * [`ContinuousComplianceMonitor`] — tracks compliance of a changing rule set
//!   against any number of registered compliance evaluators and emits
//!   [`ComplianceAlert`]s when the compliance posture degrades.
//! * [`IncrementalConflictDetector`] — re-checks only the statute pairs *affected*
//!   by an event (the changed statute against the rest of the corpus) instead of
//!   every pair, reusing the existing whole-corpus conflict detection.
//! * [`IncrementalImpactAnalyzer`] — performs an instant impact assessment for a
//!   single change by delegating to [`analyze_change_impact`].
//!
//! All four are additive and backward compatible with the existing
//! [`crate::realtime_verification`] module.
//!
//! # Examples
//!
//! ```
//! use legalis_verifier::streaming_verification::*;
//! use legalis_core::{Statute, Effect, EffectType};
//!
//! let mut verifier = StreamingVerifier::new();
//! let statute = Statute::new(
//!     "TAX-1",
//!     "Tax",
//!     Effect::new(EffectType::Grant, "Grant a tax credit to qualifying residents"),
//! );
//! let outcome = verifier.apply(StreamEvent::add(statute));
//! assert_eq!(outcome.corpus_size, 1);
//! ```

use crate::{
    ChangeImpact, ConflictType, Severity, Statute, StatuteConflict, StatuteVerifier,
    VerificationResult, analyze_change_impact, detect_statute_conflicts,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// The kind of mutation carried by a [`StreamEvent`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamEventKind {
    /// A statute is introduced into the corpus.
    Add,
    /// An existing statute (matched by id) is replaced with a new revision.
    Modify,
    /// A statute is withdrawn from the corpus.
    Remove,
}

impl std::fmt::Display for StreamEventKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamEventKind::Add => write!(f, "Add"),
            StreamEventKind::Modify => write!(f, "Modify"),
            StreamEventKind::Remove => write!(f, "Remove"),
        }
    }
}

/// A single statute mutation in a verification stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent {
    /// Kind of mutation.
    pub kind: StreamEventKind,
    /// The statute payload. For [`StreamEventKind::Remove`] only the `id` field
    /// is significant, but a full statute may be supplied.
    pub statute: Statute,
    /// Monotonic sequence number assigned by the producer (optional).
    pub sequence: Option<u64>,
}

impl StreamEvent {
    /// Creates an add event.
    pub fn add(statute: Statute) -> Self {
        Self {
            kind: StreamEventKind::Add,
            statute,
            sequence: None,
        }
    }

    /// Creates a modify event.
    pub fn modify(statute: Statute) -> Self {
        Self {
            kind: StreamEventKind::Modify,
            statute,
            sequence: None,
        }
    }

    /// Creates a remove event. Only the statute id is required to be meaningful.
    pub fn remove(statute: Statute) -> Self {
        Self {
            kind: StreamEventKind::Remove,
            statute,
            sequence: None,
        }
    }

    /// Assigns a sequence number to the event.
    pub fn with_sequence(mut self, sequence: u64) -> Self {
        self.sequence = Some(sequence);
        self
    }

    /// Returns the id of the statute targeted by this event.
    pub fn statute_id(&self) -> &str {
        &self.statute.id
    }
}

/// Outcome of applying a single [`StreamEvent`] to a [`StreamingVerifier`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamOutcome {
    /// The id of the statute the event targeted.
    pub statute_id: String,
    /// The kind of event that was applied.
    pub kind: StreamEventKind,
    /// Whether the event actually changed the corpus (a no-op modify of an
    /// identical statute, or a remove of an absent id, reports `false`).
    pub corpus_changed: bool,
    /// Per-statute verification result for the targeted statute after the event.
    /// `None` for removals (the statute no longer exists).
    pub statute_result: Option<VerificationResult>,
    /// Conflicts that now involve the affected statute (incrementally computed).
    pub new_conflicts: Vec<StatuteConflict>,
    /// Impact assessment for a modification (delegates to change-impact analysis).
    /// `None` for pure additions and removals.
    pub impact: Option<ChangeImpact>,
    /// Total number of statutes in the corpus after the event.
    pub corpus_size: usize,
}

impl StreamOutcome {
    /// Returns `true` if the affected statute passed verification (or was removed).
    pub fn is_clean(&self) -> bool {
        self.statute_result
            .as_ref()
            .map(|r| r.passed)
            .unwrap_or(true)
            && self.new_conflicts.is_empty()
    }

    /// Returns the number of critical conflicts introduced by this event.
    pub fn critical_conflict_count(&self) -> usize {
        self.new_conflicts
            .iter()
            .filter(|c| c.severity == Severity::Critical)
            .count()
    }
}

/// Configuration knobs for a [`StreamingVerifier`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingConfig {
    /// Run incremental conflict detection on every event.
    pub detect_conflicts: bool,
    /// Run incremental impact analysis on modify events.
    pub analyze_impact: bool,
    /// Retain previous revisions so that modifications can be diffed for impact.
    pub track_history: bool,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            detect_conflicts: true,
            analyze_impact: true,
            track_history: true,
        }
    }
}

/// Incremental, stream-driven statute verifier.
///
/// Maintains an in-memory corpus keyed by statute id, the previous revision of
/// each statute (for diffing), and a cache of the last per-statute verification
/// result. Each [`StreamEvent`] updates only what is necessary.
///
/// Not `Clone`/`Debug` because it wraps a [`StatuteVerifier`] (which holds a
/// shared verification cache behind an `Arc<Mutex<..>>`).
pub struct StreamingVerifier {
    config: StreamingConfig,
    verifier: StatuteVerifier,
    /// Current corpus, keyed by statute id, preserving insertion order via `order`.
    corpus: HashMap<String, Statute>,
    /// Insertion order of statute ids (so reporting is deterministic).
    order: Vec<String>,
    /// Previous revision of each statute (for impact diffing of modifications).
    previous: HashMap<String, Statute>,
    /// Cache of the last per-statute verification result.
    statute_results: HashMap<String, VerificationResult>,
    /// Content hash of each statute, used to short-circuit no-op modifications.
    hashes: HashMap<String, u64>,
    /// All conflicts currently known in the corpus.
    conflicts: Vec<StatuteConflict>,
    /// Number of events processed.
    events_processed: u64,
}

impl Default for StreamingVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamingVerifier {
    /// Creates a streaming verifier with default configuration.
    pub fn new() -> Self {
        Self::with_config(StreamingConfig::default())
    }

    /// Creates a streaming verifier with the given configuration.
    pub fn with_config(config: StreamingConfig) -> Self {
        Self {
            config,
            verifier: StatuteVerifier::default(),
            corpus: HashMap::new(),
            order: Vec::new(),
            previous: HashMap::new(),
            statute_results: HashMap::new(),
            hashes: HashMap::new(),
            conflicts: Vec::new(),
            events_processed: 0,
        }
    }

    /// Number of statutes currently in the corpus.
    pub fn corpus_size(&self) -> usize {
        self.corpus.len()
    }

    /// Number of stream events processed so far.
    pub fn events_processed(&self) -> u64 {
        self.events_processed
    }

    /// Returns a snapshot of the current corpus in insertion order.
    pub fn corpus(&self) -> Vec<&Statute> {
        self.order
            .iter()
            .filter_map(|id| self.corpus.get(id))
            .collect()
    }

    /// Returns all conflicts currently known across the corpus.
    pub fn conflicts(&self) -> &[StatuteConflict] {
        &self.conflicts
    }

    /// Returns the most recent per-statute verification result for `id`.
    pub fn statute_result(&self, id: &str) -> Option<&VerificationResult> {
        self.statute_results.get(id)
    }

    /// Computes a content hash for a statute, capturing the fields that affect
    /// verification semantics (id, title, effect, preconditions, jurisdiction,
    /// discretion logic).
    fn content_hash(statute: &Statute) -> u64 {
        let mut hasher = DefaultHasher::new();
        statute.id.hash(&mut hasher);
        statute.title.hash(&mut hasher);
        statute.effect.effect_type.hash(&mut hasher);
        statute.effect.description.hash(&mut hasher);
        statute.preconditions.len().hash(&mut hasher);
        for cond in &statute.preconditions {
            format!("{:?}", cond).hash(&mut hasher);
        }
        statute.jurisdiction.hash(&mut hasher);
        statute.discretion_logic.hash(&mut hasher);
        hasher.finish()
    }

    /// Applies a single stream event, updating the verification state and
    /// returning the incremental outcome.
    pub fn apply(&mut self, event: StreamEvent) -> StreamOutcome {
        self.events_processed += 1;
        match event.kind {
            StreamEventKind::Add => self.apply_add(event.statute),
            StreamEventKind::Modify => self.apply_modify(event.statute),
            StreamEventKind::Remove => self.apply_remove(&event.statute.id),
        }
    }

    /// Applies a batch of events in order, returning one outcome per event.
    pub fn apply_batch(
        &mut self,
        events: impl IntoIterator<Item = StreamEvent>,
    ) -> Vec<StreamOutcome> {
        events.into_iter().map(|e| self.apply(e)).collect()
    }

    fn apply_add(&mut self, statute: Statute) -> StreamOutcome {
        let id = statute.id.clone();
        let is_new = !self.corpus.contains_key(&id);
        // Treat an "Add" of an existing id as a modify so we never silently drop
        // the previous revision's history.
        if !is_new {
            return self.apply_modify(statute);
        }

        let result = self.verifier.verify_single(&statute);
        self.statute_results.insert(id.clone(), result.clone());
        self.hashes.insert(id.clone(), Self::content_hash(&statute));
        self.corpus.insert(id.clone(), statute);
        self.order.push(id.clone());

        let new_conflicts = self.recompute_conflicts_for(&id);

        StreamOutcome {
            statute_id: id,
            kind: StreamEventKind::Add,
            corpus_changed: true,
            statute_result: Some(result),
            new_conflicts,
            impact: None,
            corpus_size: self.corpus.len(),
        }
    }

    fn apply_modify(&mut self, statute: Statute) -> StreamOutcome {
        let id = statute.id.clone();
        let new_hash = Self::content_hash(&statute);

        // No-op detection: identical content means no recomputation required.
        if self.hashes.get(&id) == Some(&new_hash) && self.corpus.contains_key(&id) {
            let result = self.statute_results.get(&id).cloned();
            return StreamOutcome {
                statute_id: id,
                kind: StreamEventKind::Modify,
                corpus_changed: false,
                statute_result: result,
                new_conflicts: self.conflicts_involving(&statute.id),
                impact: None,
                corpus_size: self.corpus.len(),
            };
        }

        let old_revision = self.corpus.get(&id).cloned();

        // Impact analysis reuses the existing change-impact engine, comparing the
        // new revision against the prior one over the whole corpus.
        let impact = if self.config.analyze_impact {
            old_revision.as_ref().map(|old| {
                let all: Vec<Statute> = self.snapshot_with(&statute);
                analyze_change_impact(&statute, old, &all)
            })
        } else {
            None
        };

        if self.config.track_history
            && let Some(old) = old_revision
        {
            self.previous.insert(id.clone(), old);
        }

        let result = self.verifier.verify_single(&statute);
        self.statute_results.insert(id.clone(), result.clone());
        self.hashes.insert(id.clone(), new_hash);

        let existed = self.corpus.insert(id.clone(), statute).is_some();
        if !existed {
            self.order.push(id.clone());
        }

        let new_conflicts = self.recompute_conflicts_for(&id);

        StreamOutcome {
            statute_id: id,
            kind: StreamEventKind::Modify,
            corpus_changed: true,
            statute_result: Some(result),
            new_conflicts,
            impact,
            corpus_size: self.corpus.len(),
        }
    }

    fn apply_remove(&mut self, id: &str) -> StreamOutcome {
        let existed = self.corpus.remove(id).is_some();
        self.statute_results.remove(id);
        self.hashes.remove(id);
        self.previous.remove(id);
        if existed {
            self.order.retain(|sid| sid != id);
        }
        // Drop conflicts that referenced the removed statute.
        self.conflicts
            .retain(|c| !c.statute_ids.iter().any(|sid| sid == id));

        StreamOutcome {
            statute_id: id.to_string(),
            kind: StreamEventKind::Remove,
            corpus_changed: existed,
            statute_result: None,
            new_conflicts: Vec::new(),
            impact: None,
            corpus_size: self.corpus.len(),
        }
    }

    /// Returns the current corpus as an owned `Vec`, substituting `replacement`
    /// for the entry with the same id (or appending it if absent).
    fn snapshot_with(&self, replacement: &Statute) -> Vec<Statute> {
        let mut out: Vec<Statute> = Vec::with_capacity(self.corpus.len() + 1);
        let mut replaced = false;
        for id in &self.order {
            if id == &replacement.id {
                out.push(replacement.clone());
                replaced = true;
            } else if let Some(s) = self.corpus.get(id) {
                out.push(s.clone());
            }
        }
        if !replaced {
            out.push(replacement.clone());
        }
        out
    }

    /// Recomputes conflicts that involve the statute `id`. Reuses the existing
    /// whole-corpus [`detect_statute_conflicts`] but restricts the input to the
    /// affected pairs: the changed statute against every other statute. This is
    /// O(n) per event instead of O(n^2).
    fn recompute_conflicts_for(&mut self, id: &str) -> Vec<StatuteConflict> {
        if !self.config.detect_conflicts {
            return Vec::new();
        }
        // Drop stale conflicts that referenced this statute; they will be
        // regenerated below if still present.
        self.conflicts
            .retain(|c| !c.statute_ids.iter().any(|sid| sid == id));

        let Some(changed) = self.corpus.get(id) else {
            return Vec::new();
        };

        let mut found = Vec::new();
        for other_id in &self.order {
            if other_id == id {
                continue;
            }
            let Some(other) = self.corpus.get(other_id) else {
                continue;
            };
            // Re-use the canonical pairwise detection by feeding exactly the two
            // statutes; this guarantees identical semantics to the batch path.
            let pair = [changed.clone(), other.clone()];
            for conflict in detect_statute_conflicts(&pair) {
                // Skip self-only conflicts (id collisions need distinct ids; the
                // pair has distinct ids by construction).
                found.push(conflict);
            }
        }
        self.conflicts.extend(found.clone());
        found
    }

    /// All currently known conflicts that mention `id`.
    fn conflicts_involving(&self, id: &str) -> Vec<StatuteConflict> {
        self.conflicts
            .iter()
            .filter(|c| c.statute_ids.iter().any(|sid| sid.contains(id)))
            .cloned()
            .collect()
    }

    /// Runs a full verification over the entire current corpus (escape hatch for
    /// callers that want the authoritative whole-corpus result, e.g. periodic
    /// reconciliation).
    pub fn verify_full(&self) -> VerificationResult {
        let statutes = self.snapshot();
        self.verifier.verify(&statutes)
    }

    /// Owned snapshot of the corpus in insertion order.
    pub fn snapshot(&self) -> Vec<Statute> {
        self.order
            .iter()
            .filter_map(|id| self.corpus.get(id).cloned())
            .collect()
    }

    /// Generates a human-readable status report for the streaming session.
    pub fn report(&self) -> String {
        let mut out = String::new();
        out.push_str("# Streaming Verification Report\n\n");
        out.push_str(&format!("- Events processed: {}\n", self.events_processed));
        out.push_str(&format!("- Corpus size: {}\n", self.corpus.len()));
        out.push_str(&format!("- Known conflicts: {}\n", self.conflicts.len()));
        let failing = self.statute_results.values().filter(|r| !r.passed).count();
        out.push_str(&format!("- Statutes failing verification: {}\n\n", failing));

        if !self.conflicts.is_empty() {
            out.push_str("## Active Conflicts\n\n");
            for c in &self.conflicts {
                out.push_str(&format!(
                    "- [{:?}] {} (statutes: {})\n",
                    c.conflict_type,
                    c.description,
                    c.statute_ids.join(", ")
                ));
            }
        }
        out
    }
}

/// A single compliance evaluation produced by an evaluator at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceSnapshot {
    /// Name of the framework / evaluator that produced this snapshot.
    pub framework: String,
    /// Compliance score in the range 0-100.
    pub score: f64,
    /// Whether the rule set is considered compliant.
    pub compliant: bool,
    /// Number of violations detected.
    pub violation_count: usize,
}

/// A change in compliance posture worth surfacing to an operator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAlert {
    /// Framework the alert concerns.
    pub framework: String,
    /// Severity of the alert.
    pub severity: Severity,
    /// Human-readable message.
    pub message: String,
    /// Score before the triggering event.
    pub previous_score: f64,
    /// Score after the triggering event.
    pub current_score: f64,
}

/// Trait implemented by anything that can score a corpus for compliance.
///
/// This lets the [`ContinuousComplianceMonitor`] reuse the structured framework
/// evaluators (HIPAA, PCI-DSS, FedRAMP, NIST, GDPR, ...) without depending on
/// any single one of them concretely.
pub trait ComplianceEvaluator {
    /// Short framework name (e.g. `"HIPAA"`).
    fn name(&self) -> String;
    /// Evaluates the corpus and returns a snapshot of its compliance posture.
    fn evaluate(&self, statutes: &[Statute]) -> ComplianceSnapshot;
}

/// Continuous compliance monitor over a changing rule set.
///
/// Holds a set of registered [`ComplianceEvaluator`]s and a mutable corpus.
/// Whenever the corpus changes, [`ContinuousComplianceMonitor::refresh`]
/// re-evaluates every framework and emits [`ComplianceAlert`]s describing any
/// regression (a drop in score, a transition from compliant to non-compliant, or
/// newly introduced violations).
pub struct ContinuousComplianceMonitor {
    corpus: HashMap<String, Statute>,
    order: Vec<String>,
    evaluators: Vec<Box<dyn ComplianceEvaluator + Send + Sync>>,
    last_snapshots: HashMap<String, ComplianceSnapshot>,
    /// Score drop (absolute, 0-100) that escalates an alert to `Error`.
    regression_error_threshold: f64,
}

impl ContinuousComplianceMonitor {
    /// Creates an empty monitor.
    pub fn new() -> Self {
        Self {
            corpus: HashMap::new(),
            order: Vec::new(),
            evaluators: Vec::new(),
            last_snapshots: HashMap::new(),
            regression_error_threshold: 15.0,
        }
    }

    /// Sets the score-drop threshold (0-100) above which a regression alert is
    /// escalated from `Warning` to `Error`.
    pub fn with_regression_error_threshold(mut self, threshold: f64) -> Self {
        self.regression_error_threshold = threshold.clamp(0.0, 100.0);
        self
    }

    /// Registers a compliance evaluator.
    pub fn register_evaluator(&mut self, evaluator: Box<dyn ComplianceEvaluator + Send + Sync>) {
        self.evaluators.push(evaluator);
    }

    /// Number of registered evaluators.
    pub fn evaluator_count(&self) -> usize {
        self.evaluators.len()
    }

    /// Number of statutes currently monitored.
    pub fn corpus_size(&self) -> usize {
        self.corpus.len()
    }

    /// Applies a stream event to the monitored corpus and re-evaluates all
    /// frameworks, returning any alerts produced by the change.
    pub fn apply(&mut self, event: StreamEvent) -> Vec<ComplianceAlert> {
        match event.kind {
            StreamEventKind::Add | StreamEventKind::Modify => {
                let id = event.statute.id.clone();
                if !self.corpus.contains_key(&id) {
                    self.order.push(id.clone());
                }
                self.corpus.insert(id, event.statute);
            }
            StreamEventKind::Remove => {
                let id = &event.statute.id;
                if self.corpus.remove(id).is_some() {
                    self.order.retain(|sid| sid != id);
                }
            }
        }
        self.refresh()
    }

    /// Re-evaluates every framework against the current corpus and emits alerts
    /// for any compliance regression relative to the previous evaluation.
    pub fn refresh(&mut self) -> Vec<ComplianceAlert> {
        let statutes = self.snapshot();
        let mut alerts = Vec::new();

        for evaluator in &self.evaluators {
            let name = evaluator.name();
            let snapshot = evaluator.evaluate(&statutes);

            if let Some(prev) = self.last_snapshots.get(&name) {
                // Compliant -> non-compliant transition.
                if prev.compliant && !snapshot.compliant {
                    alerts.push(ComplianceAlert {
                        framework: name.clone(),
                        severity: Severity::Critical,
                        message: format!(
                            "{} compliance lost (score {:.1} -> {:.1})",
                            name, prev.score, snapshot.score
                        ),
                        previous_score: prev.score,
                        current_score: snapshot.score,
                    });
                } else if snapshot.score + f64::EPSILON < prev.score {
                    // Score regression without crossing the compliance line.
                    let drop = prev.score - snapshot.score;
                    let severity = if drop >= self.regression_error_threshold {
                        Severity::Error
                    } else {
                        Severity::Warning
                    };
                    alerts.push(ComplianceAlert {
                        framework: name.clone(),
                        severity,
                        message: format!(
                            "{} compliance score dropped by {:.1} ({:.1} -> {:.1})",
                            name, drop, prev.score, snapshot.score
                        ),
                        previous_score: prev.score,
                        current_score: snapshot.score,
                    });
                } else if snapshot.violation_count > prev.violation_count {
                    // Same score band but more violations (e.g. spread across
                    // additional statutes).
                    alerts.push(ComplianceAlert {
                        framework: name.clone(),
                        severity: Severity::Warning,
                        message: format!(
                            "{} violations increased from {} to {}",
                            name, prev.violation_count, snapshot.violation_count
                        ),
                        previous_score: prev.score,
                        current_score: snapshot.score,
                    });
                }
            } else if !snapshot.compliant {
                // First evaluation already non-compliant.
                alerts.push(ComplianceAlert {
                    framework: name.clone(),
                    severity: Severity::Warning,
                    message: format!(
                        "{} initial evaluation is non-compliant (score {:.1})",
                        name, snapshot.score
                    ),
                    previous_score: snapshot.score,
                    current_score: snapshot.score,
                });
            }

            self.last_snapshots.insert(name, snapshot);
        }

        alerts
    }

    /// Returns the most recent snapshot for every framework.
    pub fn snapshots(&self) -> &HashMap<String, ComplianceSnapshot> {
        &self.last_snapshots
    }

    fn snapshot(&self) -> Vec<Statute> {
        self.order
            .iter()
            .filter_map(|id| self.corpus.get(id).cloned())
            .collect()
    }
}

impl Default for ContinuousComplianceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Real-time conflict detector that re-checks only affected pairs on each update.
///
/// This is a thin, stateful wrapper around the existing
/// [`detect_statute_conflicts`] function. It keeps the corpus and the set of
/// currently known conflicts, and on each update only re-evaluates the pairs that
/// include the changed statute.
#[derive(Debug, Clone, Default)]
pub struct IncrementalConflictDetector {
    corpus: HashMap<String, Statute>,
    order: Vec<String>,
    conflicts: Vec<StatuteConflict>,
}

impl IncrementalConflictDetector {
    /// Creates an empty detector.
    pub fn new() -> Self {
        Self::default()
    }

    /// Seeds the detector with an initial corpus, running a full detection once.
    pub fn seed(&mut self, statutes: &[Statute]) {
        for s in statutes {
            if !self.corpus.contains_key(&s.id) {
                self.order.push(s.id.clone());
            }
            self.corpus.insert(s.id.clone(), s.clone());
        }
        let snapshot = self.snapshot();
        self.conflicts = detect_statute_conflicts(&snapshot);
    }

    /// Registers (or replaces) a statute and re-checks only the pairs that
    /// involve it. Returns the conflicts now involving the statute.
    pub fn upsert(&mut self, statute: Statute) -> Vec<StatuteConflict> {
        let id = statute.id.clone();
        if !self.corpus.contains_key(&id) {
            self.order.push(id.clone());
        }
        self.corpus.insert(id.clone(), statute);
        self.recheck(&id)
    }

    /// Removes a statute and drops any conflicts that referenced it.
    pub fn remove(&mut self, id: &str) {
        if self.corpus.remove(id).is_some() {
            self.order.retain(|sid| sid != id);
        }
        self.conflicts
            .retain(|c| !c.statute_ids.iter().any(|sid| sid.contains(id)));
    }

    /// All conflicts currently known.
    pub fn conflicts(&self) -> &[StatuteConflict] {
        &self.conflicts
    }

    /// Number of statutes tracked.
    pub fn corpus_size(&self) -> usize {
        self.corpus.len()
    }

    /// Owned snapshot of the tracked corpus in insertion order.
    fn snapshot(&self) -> Vec<Statute> {
        self.order
            .iter()
            .filter_map(|id| self.corpus.get(id).cloned())
            .collect()
    }

    fn recheck(&mut self, id: &str) -> Vec<StatuteConflict> {
        // Remove stale conflicts mentioning this id.
        self.conflicts
            .retain(|c| !c.statute_ids.iter().any(|sid| sid.contains(id)));

        let Some(changed) = self.corpus.get(id).cloned() else {
            return Vec::new();
        };

        let mut new_conflicts = Vec::new();
        for other_id in self.order.clone() {
            if other_id == id {
                continue;
            }
            if let Some(other) = self.corpus.get(&other_id) {
                let pair = [changed.clone(), other.clone()];
                new_conflicts.extend(detect_statute_conflicts(&pair));
            }
        }
        self.conflicts.extend(new_conflicts.clone());
        new_conflicts
    }
}

/// Instant impact analyzer for a single change.
///
/// Wraps [`analyze_change_impact`] with a maintained corpus so callers can ask
/// "what is the impact of replacing statute X with this new revision?" in one
/// call without rebuilding the whole corpus each time.
#[derive(Debug, Clone, Default)]
pub struct IncrementalImpactAnalyzer {
    corpus: HashMap<String, Statute>,
    order: Vec<String>,
}

impl IncrementalImpactAnalyzer {
    /// Creates an empty analyzer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Seeds the analyzer with an initial corpus.
    pub fn seed(&mut self, statutes: &[Statute]) {
        for s in statutes {
            if !self.corpus.contains_key(&s.id) {
                self.order.push(s.id.clone());
            }
            self.corpus.insert(s.id.clone(), s.clone());
        }
    }

    /// Number of statutes tracked.
    pub fn corpus_size(&self) -> usize {
        self.corpus.len()
    }

    /// Assesses the impact of applying `new_revision` (a modification of an
    /// existing statute) and commits the change to the maintained corpus.
    ///
    /// Returns `None` if the statute id is not present (nothing to diff against).
    pub fn assess_modification(&mut self, new_revision: Statute) -> Option<ChangeImpact> {
        let old = self.corpus.get(&new_revision.id).cloned()?;
        let all = self.snapshot_with(&new_revision);
        let impact = analyze_change_impact(&new_revision, &old, &all);
        self.corpus.insert(new_revision.id.clone(), new_revision);
        Some(impact)
    }

    /// Assesses impact *without* committing the change (a dry run).
    pub fn preview_modification(&self, new_revision: &Statute) -> Option<ChangeImpact> {
        let old = self.corpus.get(&new_revision.id).cloned()?;
        let all = self.snapshot_with(new_revision);
        Some(analyze_change_impact(new_revision, &old, &all))
    }

    fn snapshot_with(&self, replacement: &Statute) -> Vec<Statute> {
        let mut out = Vec::with_capacity(self.corpus.len() + 1);
        let mut replaced = false;
        for id in &self.order {
            if id == &replacement.id {
                out.push(replacement.clone());
                replaced = true;
            } else if let Some(s) = self.corpus.get(id) {
                out.push(s.clone());
            }
        }
        if !replaced {
            out.push(replacement.clone());
        }
        out
    }
}

/// Convenience: classify an incremental conflict set into a single severity.
///
/// [`Severity`] implements `Ord` (Info < Warning < Error < Critical), so the
/// worst severity is simply the maximum.
pub fn worst_conflict_severity(conflicts: &[StatuteConflict]) -> Option<Severity> {
    conflicts.iter().map(|c| c.severity).max()
}

/// Maps a [`ConflictType`] to a short label used in streaming reports.
pub fn conflict_type_label(kind: &ConflictType) -> &'static str {
    match kind {
        ConflictType::EffectConflict => "effect",
        ConflictType::JurisdictionalOverlap => "jurisdiction",
        ConflictType::TemporalConflict => "temporal",
        ConflictType::HierarchyViolation => "hierarchy",
        ConflictType::IdCollision => "id-collision",
    }
}

#[cfg(test)]
mod tests;
