//! Root-cause analysis over the hash-chained audit log.
//!
//! When a finding implicates a particular record, the natural next question is
//! *why* — what earlier events plausibly contributed to it? This module answers
//! that by combining two complementary techniques:
//!
//! 1. **Causal-chain backtracking.** Audit records form a hash chain
//!    (`previous_hash` -> `record_hash`). We walk that chain backward from the
//!    target to recover its literal predecessors.
//! 2. **Event correlation.** Among the records preceding the target we score
//!    candidate antecedents by the correlation signals they share with it
//!    (same subject, same statute, same actor, temporal proximity, a preceding
//!    override or void, a shared context attribute), weighted and discounted by
//!    recency.
//!
//! The result is a ranked list of [`CandidateCause`]s, each explaining the
//! evidence behind its score.

use crate::insights::finding::AuditFinding;
use crate::{Actor, AuditError, AuditRecord, AuditResult, DecisionResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A single correlation signal between an antecedent and the target record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CorrelationKind {
    /// The antecedent is the target's immediate hash-chain predecessor.
    ChainPredecessor,
    /// Both records concern the same subject.
    SameSubject,
    /// Both records apply the same statute.
    SameStatute,
    /// Both records were triggered by the same actor.
    SameActor,
    /// The antecedent occurred shortly before the target.
    TemporalProximity,
    /// The antecedent was a human override (a prior intervention).
    PrecedingOverride,
    /// The antecedent was voided due to a logical error.
    PrecedingVoid,
    /// The records share a context attribute key/value.
    SharedAttribute,
}

impl CorrelationKind {
    /// The evidential weight of this signal.
    pub fn weight(self) -> f64 {
        match self {
            CorrelationKind::SameSubject => 0.9,
            CorrelationKind::PrecedingOverride => 0.7,
            CorrelationKind::PrecedingVoid => 0.7,
            CorrelationKind::ChainPredecessor => 0.5,
            CorrelationKind::SameStatute => 0.5,
            CorrelationKind::SharedAttribute => 0.5,
            CorrelationKind::SameActor => 0.4,
            CorrelationKind::TemporalProximity => 0.4,
        }
    }

    /// A stable lower-case label.
    pub fn label(self) -> &'static str {
        match self {
            CorrelationKind::ChainPredecessor => "chain_predecessor",
            CorrelationKind::SameSubject => "same_subject",
            CorrelationKind::SameStatute => "same_statute",
            CorrelationKind::SameActor => "same_actor",
            CorrelationKind::TemporalProximity => "temporal_proximity",
            CorrelationKind::PrecedingOverride => "preceding_override",
            CorrelationKind::PrecedingVoid => "preceding_void",
            CorrelationKind::SharedAttribute => "shared_attribute",
        }
    }
}

/// A scored link from an antecedent record to the target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLink {
    /// The antecedent record's identifier.
    pub antecedent_id: Uuid,
    /// When the antecedent occurred.
    pub antecedent_timestamp: DateTime<Utc>,
    /// The correlation signals matched.
    pub kinds: Vec<CorrelationKind>,
    /// Seconds the antecedent preceded the target.
    pub lag_seconds: i64,
    /// Combined strength in `[0, 1]`.
    pub strength: f64,
}

/// A ranked candidate root cause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateCause {
    /// The candidate record's identifier.
    pub record_id: Uuid,
    /// Combined strength in `[0, 1]`.
    pub strength: f64,
    /// The correlation signals that justify the candidate.
    pub correlations: Vec<CorrelationKind>,
    /// A human-readable explanation.
    pub summary: String,
}

/// The full result of analysing one target record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootCauseAnalysis {
    /// The record under investigation.
    pub target_id: Uuid,
    /// When the target occurred.
    pub target_timestamp: DateTime<Utc>,
    /// Literal hash-chain predecessors, nearest first.
    pub chain_predecessors: Vec<Uuid>,
    /// Every scored antecedent above the strength floor.
    pub causal_links: Vec<CausalLink>,
    /// The top-ranked candidate causes.
    pub ranked_causes: Vec<CandidateCause>,
    /// A narrative summary of the analysis.
    pub summary: String,
}

/// Configuration for [`RootCauseAnalyzer`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootCauseConfig {
    /// Window (seconds) within which temporal proximity is credited.
    pub temporal_window_secs: i64,
    /// Maximum hash-chain backtracking depth.
    pub max_chain_depth: usize,
    /// Maximum number of ranked candidate causes to retain.
    pub max_candidates: usize,
    /// Minimum strength for an antecedent to be retained.
    pub min_strength: f64,
}

impl Default for RootCauseConfig {
    fn default() -> Self {
        Self {
            temporal_window_secs: 60 * 60 * 24, // 24 hours
            max_chain_depth: 16,
            max_candidates: 8,
            min_strength: 0.15,
        }
    }
}

/// Performs root-cause analysis over a set of audit records.
#[derive(Debug, Clone)]
pub struct RootCauseAnalyzer {
    config: RootCauseConfig,
}

impl RootCauseAnalyzer {
    /// Creates an analyzer with the default configuration.
    pub fn new() -> Self {
        Self::with_config(RootCauseConfig::default())
    }

    /// Creates an analyzer with a custom configuration.
    pub fn with_config(config: RootCauseConfig) -> Self {
        Self { config }
    }

    /// Returns the active configuration.
    pub fn config(&self) -> &RootCauseConfig {
        &self.config
    }

    /// Analyses the antecedents of `target_id` within `records`.
    pub fn analyze(
        &self,
        records: &[AuditRecord],
        target_id: Uuid,
    ) -> AuditResult<RootCauseAnalysis> {
        let target = records
            .iter()
            .find(|r| r.id == target_id)
            .ok_or(AuditError::RecordNotFound(target_id))?;

        let chain_predecessors = self.backtrack_chain(records, target);
        let chain_set: std::collections::HashSet<Uuid> =
            chain_predecessors.iter().copied().collect();

        let mut links: Vec<CausalLink> = Vec::new();
        for candidate in records {
            if candidate.id == target.id || candidate.timestamp > target.timestamp {
                continue;
            }
            let kinds = self.correlations(target, candidate, &chain_set);
            if kinds.is_empty() {
                continue;
            }
            let lag_seconds = (target.timestamp - candidate.timestamp)
                .num_seconds()
                .max(0);
            let strength = self.strength(&kinds, lag_seconds);
            if strength < self.config.min_strength {
                continue;
            }
            links.push(CausalLink {
                antecedent_id: candidate.id,
                antecedent_timestamp: candidate.timestamp,
                kinds,
                lag_seconds,
                strength,
            });
        }

        links.sort_by(|a, b| {
            b.strength
                .total_cmp(&a.strength)
                .then(a.lag_seconds.cmp(&b.lag_seconds))
        });

        let ranked_causes: Vec<CandidateCause> = links
            .iter()
            .take(self.config.max_candidates)
            .map(|link| CandidateCause {
                record_id: link.antecedent_id,
                strength: link.strength,
                correlations: link.kinds.clone(),
                summary: format!(
                    "Record {} (lag {}s) correlates via [{}] with strength {:.2}.",
                    link.antecedent_id,
                    link.lag_seconds,
                    link.kinds
                        .iter()
                        .map(|k| k.label())
                        .collect::<Vec<_>>()
                        .join(", "),
                    link.strength
                ),
            })
            .collect();

        let summary = if let Some(top) = ranked_causes.first() {
            format!(
                "Identified {} candidate antecedent(s) for record {}; the strongest is {} via [{}] (strength {:.2}).",
                ranked_causes.len(),
                target.id,
                top.record_id,
                top.correlations
                    .iter()
                    .map(|k| k.label())
                    .collect::<Vec<_>>()
                    .join(", "),
                top.strength
            )
        } else {
            format!(
                "No correlated antecedents found for record {} above the strength floor.",
                target.id
            )
        };

        Ok(RootCauseAnalysis {
            target_id: target.id,
            target_timestamp: target.timestamp,
            chain_predecessors,
            causal_links: links,
            ranked_causes,
            summary,
        })
    }

    /// Analyses the representative evidence records of a finding (up to
    /// `max_targets` of them), returning one analysis per target.
    pub fn analyze_finding(
        &self,
        records: &[AuditRecord],
        finding: &AuditFinding,
        max_targets: usize,
    ) -> AuditResult<Vec<RootCauseAnalysis>> {
        let mut analyses = Vec::new();
        for &target_id in finding.evidence.iter().take(max_targets.max(1)) {
            // Evidence ids may be capped/missing from the working set; skip
            // gracefully rather than failing the whole analysis.
            match self.analyze(records, target_id) {
                Ok(analysis) => analyses.push(analysis),
                Err(AuditError::RecordNotFound(_)) => continue,
                Err(other) => return Err(other),
            }
        }
        Ok(analyses)
    }

    /// Walks the hash chain backward from `target`.
    fn backtrack_chain(&self, records: &[AuditRecord], target: &AuditRecord) -> Vec<Uuid> {
        let mut by_hash: HashMap<&str, &AuditRecord> = HashMap::new();
        for record in records {
            if !record.record_hash.is_empty() {
                by_hash.insert(record.record_hash.as_str(), record);
            }
        }

        let mut predecessors = Vec::new();
        let mut current = target;
        for _ in 0..self.config.max_chain_depth {
            let Some(prev_hash) = current.previous_hash.as_deref() else {
                break;
            };
            let Some(prev) = by_hash.get(prev_hash) else {
                break;
            };
            if prev.id == current.id {
                break; // defensive: never loop on self-referential hashes
            }
            predecessors.push(prev.id);
            current = prev;
        }
        predecessors
    }

    /// Computes the set of correlation signals between target and candidate.
    fn correlations(
        &self,
        target: &AuditRecord,
        candidate: &AuditRecord,
        chain_set: &std::collections::HashSet<Uuid>,
    ) -> Vec<CorrelationKind> {
        let mut kinds = Vec::new();

        if chain_set.contains(&candidate.id) {
            kinds.push(CorrelationKind::ChainPredecessor);
        }
        if candidate.subject_id == target.subject_id {
            kinds.push(CorrelationKind::SameSubject);
        }
        if candidate.statute_id == target.statute_id {
            kinds.push(CorrelationKind::SameStatute);
        }
        if actor_identity(&candidate.actor) == actor_identity(&target.actor) {
            kinds.push(CorrelationKind::SameActor);
        }

        let lag = (target.timestamp - candidate.timestamp).num_seconds();
        if (0..=self.config.temporal_window_secs).contains(&lag) {
            kinds.push(CorrelationKind::TemporalProximity);
        }

        let related =
            candidate.subject_id == target.subject_id || candidate.statute_id == target.statute_id;
        if related {
            match &candidate.result {
                DecisionResult::Overridden { .. } => kinds.push(CorrelationKind::PrecedingOverride),
                DecisionResult::Void { .. } => kinds.push(CorrelationKind::PrecedingVoid),
                _ => {}
            }
        }

        if shares_attribute(target, candidate) {
            kinds.push(CorrelationKind::SharedAttribute);
        }

        kinds
    }

    /// Folds matched signals and recency into a `[0, 1]` strength.
    fn strength(&self, kinds: &[CorrelationKind], lag_seconds: i64) -> f64 {
        let raw: f64 = kinds.iter().map(|k| k.weight()).sum();
        // Saturating transform: diminishing returns as signals accumulate.
        let signal = 1.0 - (-raw).exp();
        // Recency discount over the temporal window (exponential decay).
        let window = self.config.temporal_window_secs.max(1) as f64;
        let recency = (-(lag_seconds as f64) / window).exp().clamp(0.0, 1.0);
        (signal * (0.4 + 0.6 * recency)).clamp(0.0, 1.0)
    }
}

impl Default for RootCauseAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns a stable identity string for an actor.
fn actor_identity(actor: &Actor) -> String {
    match actor {
        Actor::System { component } => format!("system:{component}"),
        Actor::User { user_id, .. } => format!("user:{user_id}"),
        Actor::External { system_id } => format!("external:{system_id}"),
    }
}

/// Returns `true` if the two records share at least one identical context
/// attribute key/value pair.
fn shares_attribute(a: &AuditRecord, b: &AuditRecord) -> bool {
    if a.context.attributes.is_empty() || b.context.attributes.is_empty() {
        return false;
    }
    a.context.attributes.iter().any(|(key, value)| {
        b.context
            .attributes
            .get(key)
            .map(|other| other == value)
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DecisionContext, EventType};
    use chrono::Duration;
    use std::collections::HashMap as StdHashMap;

    fn base_record(
        subject: Uuid,
        statute: &str,
        offset_secs: i64,
        result: DecisionResult,
    ) -> AuditRecord {
        AuditRecord {
            id: Uuid::new_v4(),
            timestamp: Utc::now() + Duration::seconds(offset_secs),
            event_type: EventType::AutomaticDecision,
            actor: Actor::System {
                component: "engine".to_string(),
            },
            statute_id: statute.to_string(),
            subject_id: subject,
            context: DecisionContext::default(),
            result,
            previous_hash: None,
            record_hash: String::new(),
        }
    }

    fn deterministic(effect: &str) -> DecisionResult {
        DecisionResult::Deterministic {
            effect_applied: effect.to_string(),
            parameters: StdHashMap::new(),
        }
    }

    #[test]
    fn test_analyze_same_subject_correlation() {
        let subject = Uuid::new_v4();
        let antecedent = base_record(subject, "stat-1", 0, deterministic("approved"));
        let target = base_record(subject, "stat-1", 60, deterministic("denied"));
        let target_id = target.id;
        let records = vec![antecedent.clone(), target];

        let analysis = RootCauseAnalyzer::new()
            .analyze(&records, target_id)
            .expect("analysis ok");
        assert!(!analysis.ranked_causes.is_empty());
        let top = &analysis.ranked_causes[0];
        assert_eq!(top.record_id, antecedent.id);
        assert!(top.correlations.contains(&CorrelationKind::SameSubject));
        assert!(top.correlations.contains(&CorrelationKind::SameStatute));
    }

    #[test]
    fn test_preceding_override_signal() {
        let subject = Uuid::new_v4();
        let override_result = DecisionResult::Overridden {
            original_result: Box::new(deterministic("approved")),
            new_result: Box::new(deterministic("denied")),
            justification: "manual".to_string(),
        };
        let antecedent = base_record(subject, "stat-1", 0, override_result);
        let target = base_record(subject, "stat-1", 120, deterministic("denied"));
        let target_id = target.id;
        let records = vec![antecedent, target];

        let analysis = RootCauseAnalyzer::new()
            .analyze(&records, target_id)
            .expect("ok");
        assert!(
            analysis.causal_links[0]
                .kinds
                .contains(&CorrelationKind::PrecedingOverride)
        );
    }

    #[test]
    fn test_chain_backtracking() {
        // Manually wire a 3-record hash chain.
        let subject = Uuid::new_v4();
        let mut r1 = base_record(subject, "s", 0, deterministic("approved"));
        r1.record_hash = "hash1".to_string();
        let mut r2 = base_record(subject, "s", 10, deterministic("approved"));
        r2.previous_hash = Some("hash1".to_string());
        r2.record_hash = "hash2".to_string();
        let mut r3 = base_record(subject, "s", 20, deterministic("denied"));
        r3.previous_hash = Some("hash2".to_string());
        r3.record_hash = "hash3".to_string();
        let target_id = r3.id;
        let (id1, id2) = (r1.id, r2.id);
        let records = vec![r1, r2, r3];

        let analysis = RootCauseAnalyzer::new()
            .analyze(&records, target_id)
            .expect("ok");
        assert_eq!(analysis.chain_predecessors, vec![id2, id1]);
        assert!(
            analysis
                .ranked_causes
                .iter()
                .any(|c| c.correlations.contains(&CorrelationKind::ChainPredecessor))
        );
    }

    #[test]
    fn test_shared_attribute_signal() {
        let mut a = base_record(Uuid::new_v4(), "s1", 0, deterministic("approved"));
        a.context
            .attributes
            .insert("region".to_string(), "EU".to_string());
        let mut target = base_record(Uuid::new_v4(), "s2", 30, deterministic("denied"));
        target
            .context
            .attributes
            .insert("region".to_string(), "EU".to_string());
        let target_id = target.id;
        let records = vec![a, target];

        let analysis = RootCauseAnalyzer::new()
            .analyze(&records, target_id)
            .expect("ok");
        assert!(
            analysis
                .causal_links
                .iter()
                .any(|l| l.kinds.contains(&CorrelationKind::SharedAttribute))
        );
    }

    #[test]
    fn test_missing_target_errors() {
        let records = vec![base_record(Uuid::new_v4(), "s", 0, deterministic("ok"))];
        let result = RootCauseAnalyzer::new().analyze(&records, Uuid::new_v4());
        assert!(matches!(result, Err(AuditError::RecordNotFound(_))));
    }

    #[test]
    fn test_no_antecedents() {
        // A lone record with no predecessors yields an empty ranking.
        let r = base_record(Uuid::new_v4(), "s", 0, deterministic("ok"));
        let id = r.id;
        let analysis = RootCauseAnalyzer::new().analyze(&[r], id).expect("ok");
        assert!(analysis.ranked_causes.is_empty());
        assert!(analysis.summary.contains("No correlated antecedents"));
    }
}
