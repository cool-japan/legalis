//! Cross-metaverse legal harmonization.
//!
//! Different metaverse jurisdictions enact overlapping rules on the same topics
//! (minimum age to trade, maximum resale royalty, whether soulbound tokens are
//! allowed, data-retention windows). When an avatar, asset or contract spans
//! several worlds, those rule sets must be *reconciled*. A [`MetaverseRule`] is a
//! single normative statement keyed by `topic`; a [`HarmonizationEngine`] holds
//! the rule sets of several jurisdictions, detects where they conflict
//! ([`RuleConflict`] / [`ConflictKind`]), and applies a
//! [`HarmonizationStrategy`] to fold them into one harmonized rule set
//! ([`HarmonizedRule`]) plus a report of conflicts that could not be resolved
//! automatically.
//!
//! The engine is deterministic: the same jurisdictions and strategy always
//! produce the same harmonized output, so a harmonization can be audited and
//! committed to the [`crate::blockchain`] ledger.

use super::sha256_parts;
use crate::PortingError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

type HarmonizationResult<T> = Result<T, PortingError>;

/// A single normative rule within a metaverse jurisdiction.
///
/// A rule constrains one `topic`. Numeric rules (age floors, royalty caps) carry
/// a `value` and a [`Bound`] saying whether the value is a floor or a ceiling;
/// categorical rules (e.g. "soulbound tokens permitted") carry a `flag`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetaverseRule {
    /// The topic the rule governs (the harmonization key).
    pub topic: String,
    /// The kind of constraint.
    pub bound: Bound,
    /// Numeric value for floor/ceiling rules (ignored for categorical).
    pub value: i64,
    /// Boolean value for categorical rules (ignored for numeric).
    pub flag: bool,
    /// Human-readable description.
    pub description: String,
}

/// The kind of constraint a numeric or categorical [`MetaverseRule`] expresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Bound {
    /// The value is a *minimum* (e.g. minimum age >= value).
    Floor,
    /// The value is a *maximum* (e.g. maximum royalty <= value).
    Ceiling,
    /// The rule is categorical; only [`MetaverseRule::flag`] is meaningful.
    Categorical,
}

impl MetaverseRule {
    /// Creates a floor rule (a minimum value).
    pub fn floor(topic: impl Into<String>, value: i64, description: impl Into<String>) -> Self {
        Self {
            topic: topic.into(),
            bound: Bound::Floor,
            value,
            flag: false,
            description: description.into(),
        }
    }

    /// Creates a ceiling rule (a maximum value).
    pub fn ceiling(topic: impl Into<String>, value: i64, description: impl Into<String>) -> Self {
        Self {
            topic: topic.into(),
            bound: Bound::Ceiling,
            value,
            flag: false,
            description: description.into(),
        }
    }

    /// Creates a categorical (boolean) rule.
    pub fn categorical(
        topic: impl Into<String>,
        flag: bool,
        description: impl Into<String>,
    ) -> Self {
        Self {
            topic: topic.into(),
            bound: Bound::Categorical,
            value: 0,
            flag,
            description: description.into(),
        }
    }
}

/// The nature of a conflict between two jurisdictions' rules on one topic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictKind {
    /// Numeric rules with different values (e.g. age floor 13 vs. 18).
    ValueMismatch,
    /// Categorical rules disagree (permitted vs. prohibited).
    PolarityClash,
    /// One jurisdiction expresses a topic as a floor, another as a ceiling.
    BoundMismatch,
}

/// A detected conflict on a single topic across jurisdictions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleConflict {
    /// The topic in conflict.
    pub topic: String,
    /// The nature of the conflict.
    pub kind: ConflictKind,
    /// The conflicting `(jurisdiction, rule)` pairs, sorted by jurisdiction.
    pub positions: Vec<(String, MetaverseRule)>,
}

/// The strategy used to fold conflicting rules into one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarmonizationStrategy {
    /// Adopt the strictest rule (highest floor, lowest ceiling, prohibition
    /// beats permission). The default for protective regimes.
    MostRestrictive,
    /// Adopt the most permissive rule (lowest floor, highest ceiling, permission
    /// beats prohibition).
    LeastRestrictive,
    /// Numeric conflicts are unresolvable and reported; categorical conflicts
    /// default to the most restrictive. Used when divergence must surface to a
    /// human (a true legal negotiation).
    Strict,
}

/// A harmonized rule for one topic, with provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HarmonizedRule {
    /// The resolved rule.
    pub rule: MetaverseRule,
    /// Jurisdictions whose rule on this topic the resolution matches.
    pub adopted_from: Vec<String>,
    /// Whether this topic had a conflict that the strategy resolved.
    pub was_conflicting: bool,
}

/// The full result of a harmonization run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HarmonizationReport {
    /// The strategy that was applied.
    pub strategy: HarmonizationStrategy,
    /// The harmonized rule set, one entry per topic, sorted by topic.
    pub harmonized: Vec<HarmonizedRule>,
    /// Conflicts the strategy could not resolve automatically.
    pub residual_conflicts: Vec<RuleConflict>,
    /// A content hash binding the report's inputs and outputs.
    pub digest: String,
}

impl HarmonizationReport {
    /// Whether harmonization fully succeeded (no residual conflicts).
    pub fn is_fully_harmonized(&self) -> bool {
        self.residual_conflicts.is_empty()
    }

    /// Looks up the harmonized rule for a topic.
    pub fn rule_for(&self, topic: &str) -> Option<&HarmonizedRule> {
        self.harmonized.iter().find(|h| h.rule.topic == topic)
    }
}

/// Reconciles rule sets across multiple metaverse jurisdictions.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HarmonizationEngine {
    /// Rule sets keyed by jurisdiction id.
    jurisdictions: BTreeMap<String, Vec<MetaverseRule>>,
}

impl HarmonizationEngine {
    /// Creates an empty engine.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers (or replaces) a jurisdiction's rule set.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if the rule set contains two rules
    /// on the same topic (a jurisdiction must speak with one voice per topic).
    pub fn add_jurisdiction(
        &mut self,
        id: impl Into<String>,
        rules: Vec<MetaverseRule>,
    ) -> HarmonizationResult<()> {
        let id = id.into();
        let mut seen = BTreeMap::new();
        for rule in &rules {
            if seen.insert(rule.topic.clone(), ()).is_some() {
                return Err(PortingError::InvalidInput(format!(
                    "harmonization: jurisdiction '{id}' has duplicate rule for topic '{}'",
                    rule.topic
                )));
            }
        }
        self.jurisdictions.insert(id, rules);
        Ok(())
    }

    /// Number of registered jurisdictions.
    pub fn jurisdiction_count(&self) -> usize {
        self.jurisdictions.len()
    }

    /// All distinct topics across all jurisdictions, sorted.
    pub fn topics(&self) -> Vec<String> {
        let mut topics: Vec<String> = self
            .jurisdictions
            .values()
            .flat_map(|rules| rules.iter().map(|r| r.topic.clone()))
            .collect();
        topics.sort();
        topics.dedup();
        topics
    }

    /// The `(jurisdiction, rule)` positions on a topic, sorted by jurisdiction.
    fn positions_on(&self, topic: &str) -> Vec<(String, MetaverseRule)> {
        let mut positions: Vec<(String, MetaverseRule)> = self
            .jurisdictions
            .iter()
            .filter_map(|(id, rules)| {
                rules
                    .iter()
                    .find(|r| r.topic == topic)
                    .map(|r| (id.clone(), r.clone()))
            })
            .collect();
        positions.sort_by(|a, b| a.0.cmp(&b.0));
        positions
    }

    /// Classifies a conflict among positions on one topic, or `None` if they
    /// already agree.
    fn classify(positions: &[(String, MetaverseRule)]) -> Option<ConflictKind> {
        if positions.len() < 2 {
            return None;
        }
        let first = &positions[0].1;
        let mixed_bounds = positions.iter().any(|(_, r)| r.bound != first.bound);
        if mixed_bounds {
            return Some(ConflictKind::BoundMismatch);
        }
        match first.bound {
            Bound::Categorical => {
                if positions.iter().any(|(_, r)| r.flag != first.flag) {
                    Some(ConflictKind::PolarityClash)
                } else {
                    None
                }
            }
            Bound::Floor | Bound::Ceiling => {
                if positions.iter().any(|(_, r)| r.value != first.value) {
                    Some(ConflictKind::ValueMismatch)
                } else {
                    None
                }
            }
        }
    }

    /// Detects every conflict across registered jurisdictions, sorted by topic.
    pub fn detect_conflicts(&self) -> Vec<RuleConflict> {
        let mut conflicts = Vec::new();
        for topic in self.topics() {
            let positions = self.positions_on(&topic);
            if let Some(kind) = Self::classify(&positions) {
                conflicts.push(RuleConflict {
                    topic,
                    kind,
                    positions,
                });
            }
        }
        conflicts
    }

    /// Resolves the positions on a single topic under a strategy.
    ///
    /// Returns the chosen rule and the jurisdictions it was adopted from, or
    /// `None` if the strategy declines to resolve (left as a residual conflict).
    fn resolve(
        positions: &[(String, MetaverseRule)],
        strategy: HarmonizationStrategy,
    ) -> Option<(MetaverseRule, Vec<String>)> {
        if positions.is_empty() {
            return None;
        }
        // Mixed floor/ceiling on the same topic is never auto-resolvable.
        let first_bound = positions[0].1.bound;
        if positions.iter().any(|(_, r)| r.bound != first_bound) {
            return None;
        }

        let chosen = match first_bound {
            Bound::Floor => match strategy {
                HarmonizationStrategy::MostRestrictive | HarmonizationStrategy::Strict => {
                    positions.iter().max_by_key(|(_, r)| r.value)
                }
                HarmonizationStrategy::LeastRestrictive => {
                    positions.iter().min_by_key(|(_, r)| r.value)
                }
            },
            Bound::Ceiling => match strategy {
                HarmonizationStrategy::MostRestrictive | HarmonizationStrategy::Strict => {
                    positions.iter().min_by_key(|(_, r)| r.value)
                }
                HarmonizationStrategy::LeastRestrictive => {
                    positions.iter().max_by_key(|(_, r)| r.value)
                }
            },
            Bound::Categorical => {
                // false == prohibited (more restrictive), true == permitted.
                match strategy {
                    HarmonizationStrategy::MostRestrictive | HarmonizationStrategy::Strict => {
                        positions.iter().min_by_key(|(_, r)| r.flag)
                    }
                    HarmonizationStrategy::LeastRestrictive => {
                        positions.iter().max_by_key(|(_, r)| r.flag)
                    }
                }
            }
        }?;

        // Under Strict, a genuine numeric divergence is left unresolved.
        if strategy == HarmonizationStrategy::Strict
            && matches!(first_bound, Bound::Floor | Bound::Ceiling)
            && positions.iter().any(|(_, r)| r.value != chosen.1.value)
        {
            return None;
        }

        let adopted_from: Vec<String> = positions
            .iter()
            .filter(|(_, r)| {
                r.bound == chosen.1.bound && r.value == chosen.1.value && r.flag == chosen.1.flag
            })
            .map(|(id, _)| id.clone())
            .collect();
        Some((chosen.1.clone(), adopted_from))
    }

    /// Harmonizes all registered jurisdictions under `strategy`.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if fewer than two jurisdictions are
    /// registered (nothing to harmonize).
    pub fn harmonize(
        &self,
        strategy: HarmonizationStrategy,
    ) -> HarmonizationResult<HarmonizationReport> {
        if self.jurisdictions.len() < 2 {
            return Err(PortingError::InvalidInput(
                "harmonization: need at least two jurisdictions to harmonize".to_string(),
            ));
        }

        let mut harmonized = Vec::new();
        let mut residual = Vec::new();

        for topic in self.topics() {
            let positions = self.positions_on(&topic);
            let conflict_kind = Self::classify(&positions);
            match Self::resolve(&positions, strategy) {
                Some((rule, adopted_from)) => harmonized.push(HarmonizedRule {
                    rule,
                    adopted_from,
                    was_conflicting: conflict_kind.is_some(),
                }),
                None => {
                    if let Some(kind) = conflict_kind {
                        residual.push(RuleConflict {
                            topic,
                            kind,
                            positions,
                        });
                    }
                }
            }
        }

        harmonized.sort_by(|a, b| a.rule.topic.cmp(&b.rule.topic));
        residual.sort_by(|a, b| a.topic.cmp(&b.topic));

        let digest = Self::report_digest(strategy, &harmonized, &residual);
        Ok(HarmonizationReport {
            strategy,
            harmonized,
            residual_conflicts: residual,
            digest,
        })
    }

    /// A deterministic digest binding the strategy and resolved/residual output.
    fn report_digest(
        strategy: HarmonizationStrategy,
        harmonized: &[HarmonizedRule],
        residual: &[RuleConflict],
    ) -> String {
        let harmonized_repr = harmonized
            .iter()
            .map(|h| {
                format!(
                    "{}={:?}:{}:{}|{}",
                    h.rule.topic,
                    h.rule.bound,
                    h.rule.value,
                    h.rule.flag,
                    h.adopted_from.join(",")
                )
            })
            .collect::<Vec<_>>()
            .join(";");
        let residual_repr = residual
            .iter()
            .map(|c| format!("{}:{:?}", c.topic, c.kind))
            .collect::<Vec<_>>()
            .join(";");
        sha256_parts(&[
            format!("{strategy:?}").as_bytes(),
            harmonized_repr.as_bytes(),
            residual_repr.as_bytes(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine_two() -> HarmonizationEngine {
        let mut engine = HarmonizationEngine::new();
        engine
            .add_jurisdiction(
                "mv-a",
                vec![
                    MetaverseRule::floor("min_trade_age", 13, "min age to trade"),
                    MetaverseRule::ceiling("max_royalty_bps", 1000, "max resale royalty"),
                    MetaverseRule::categorical("soulbound_allowed", true, "soulbound permitted"),
                ],
            )
            .expect("a");
        engine
            .add_jurisdiction(
                "mv-b",
                vec![
                    MetaverseRule::floor("min_trade_age", 18, "min age to trade"),
                    MetaverseRule::ceiling("max_royalty_bps", 2500, "max resale royalty"),
                    MetaverseRule::categorical("soulbound_allowed", false, "soulbound prohibited"),
                ],
            )
            .expect("b");
        engine
    }

    #[test]
    fn test_add_jurisdiction_rejects_duplicate_topic() {
        let mut engine = HarmonizationEngine::new();
        let result = engine.add_jurisdiction(
            "x",
            vec![
                MetaverseRule::floor("t", 1, "a"),
                MetaverseRule::floor("t", 2, "b"),
            ],
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_topics_are_sorted_and_deduped() {
        let engine = engine_two();
        let topics = engine.topics();
        assert_eq!(
            topics,
            vec!["max_royalty_bps", "min_trade_age", "soulbound_allowed"]
        );
    }

    #[test]
    fn test_detect_conflicts_classifies() {
        let engine = engine_two();
        let conflicts = engine.detect_conflicts();
        assert_eq!(conflicts.len(), 3);
        let by_topic: BTreeMap<&str, ConflictKind> = conflicts
            .iter()
            .map(|c| (c.topic.as_str(), c.kind))
            .collect();
        assert_eq!(
            by_topic.get("min_trade_age"),
            Some(&ConflictKind::ValueMismatch)
        );
        assert_eq!(
            by_topic.get("max_royalty_bps"),
            Some(&ConflictKind::ValueMismatch)
        );
        assert_eq!(
            by_topic.get("soulbound_allowed"),
            Some(&ConflictKind::PolarityClash)
        );
    }

    #[test]
    fn test_no_conflict_when_agreed() {
        let mut engine = HarmonizationEngine::new();
        engine
            .add_jurisdiction("a", vec![MetaverseRule::floor("age", 18, "x")])
            .expect("a");
        engine
            .add_jurisdiction("b", vec![MetaverseRule::floor("age", 18, "y")])
            .expect("b");
        assert!(engine.detect_conflicts().is_empty());
    }

    #[test]
    fn test_harmonize_most_restrictive() {
        let engine = engine_two();
        let report = engine
            .harmonize(HarmonizationStrategy::MostRestrictive)
            .expect("h");
        assert!(report.is_fully_harmonized());
        // Highest floor wins: age 18.
        assert_eq!(
            report.rule_for("min_trade_age").expect("age").rule.value,
            18
        );
        // Lowest ceiling wins: royalty 1000.
        assert_eq!(
            report.rule_for("max_royalty_bps").expect("roy").rule.value,
            1000
        );
        // Prohibition beats permission: soulbound = false.
        assert!(!report.rule_for("soulbound_allowed").expect("sb").rule.flag);
        assert!(
            report
                .rule_for("min_trade_age")
                .expect("age")
                .was_conflicting
        );
    }

    #[test]
    fn test_harmonize_least_restrictive() {
        let engine = engine_two();
        let report = engine
            .harmonize(HarmonizationStrategy::LeastRestrictive)
            .expect("h");
        // Lowest floor: age 13; highest ceiling: royalty 2500; permission wins.
        assert_eq!(
            report.rule_for("min_trade_age").expect("age").rule.value,
            13
        );
        assert_eq!(
            report.rule_for("max_royalty_bps").expect("roy").rule.value,
            2500
        );
        assert!(report.rule_for("soulbound_allowed").expect("sb").rule.flag);
    }

    #[test]
    fn test_harmonize_strict_leaves_numeric_residual() {
        let engine = engine_two();
        let report = engine.harmonize(HarmonizationStrategy::Strict).expect("h");
        // Numeric conflicts remain unresolved; categorical resolves restrictively.
        assert!(!report.is_fully_harmonized());
        let residual_topics: Vec<&str> = report
            .residual_conflicts
            .iter()
            .map(|c| c.topic.as_str())
            .collect();
        assert!(residual_topics.contains(&"min_trade_age"));
        assert!(residual_topics.contains(&"max_royalty_bps"));
        // Categorical still harmonized to the restrictive option.
        assert!(!report.rule_for("soulbound_allowed").expect("sb").rule.flag);
    }

    #[test]
    fn test_bound_mismatch_is_residual() {
        let mut engine = HarmonizationEngine::new();
        engine
            .add_jurisdiction("a", vec![MetaverseRule::floor("fee", 10, "min fee")])
            .expect("a");
        engine
            .add_jurisdiction("b", vec![MetaverseRule::ceiling("fee", 10, "max fee")])
            .expect("b");
        let conflicts = engine.detect_conflicts();
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].kind, ConflictKind::BoundMismatch);
        let report = engine
            .harmonize(HarmonizationStrategy::MostRestrictive)
            .expect("h");
        // A floor/ceiling mismatch cannot be auto-resolved under any strategy.
        assert!(!report.is_fully_harmonized());
    }

    #[test]
    fn test_harmonize_requires_two_jurisdictions() {
        let mut engine = HarmonizationEngine::new();
        engine
            .add_jurisdiction("only", vec![MetaverseRule::floor("age", 18, "x")])
            .expect("only");
        assert!(
            engine
                .harmonize(HarmonizationStrategy::MostRestrictive)
                .is_err()
        );
    }

    #[test]
    fn test_adopted_from_records_provenance() {
        let mut engine = HarmonizationEngine::new();
        engine
            .add_jurisdiction("a", vec![MetaverseRule::floor("age", 18, "x")])
            .expect("a");
        engine
            .add_jurisdiction("b", vec![MetaverseRule::floor("age", 18, "y")])
            .expect("b");
        engine
            .add_jurisdiction("c", vec![MetaverseRule::floor("age", 16, "z")])
            .expect("c");
        let report = engine
            .harmonize(HarmonizationStrategy::MostRestrictive)
            .expect("h");
        let age = report.rule_for("age").expect("age");
        assert_eq!(age.rule.value, 18);
        let mut adopted = age.adopted_from.clone();
        adopted.sort();
        assert_eq!(adopted, vec!["a", "b"]);
    }

    #[test]
    fn test_digest_is_deterministic_and_strategy_sensitive() {
        let engine = engine_two();
        let r1 = engine
            .harmonize(HarmonizationStrategy::MostRestrictive)
            .expect("h1");
        let r2 = engine
            .harmonize(HarmonizationStrategy::MostRestrictive)
            .expect("h2");
        let r3 = engine
            .harmonize(HarmonizationStrategy::LeastRestrictive)
            .expect("h3");
        assert_eq!(r1.digest, r2.digest);
        assert_ne!(r1.digest, r3.digest);
    }

    #[test]
    fn test_report_serde_roundtrip() {
        let engine = engine_two();
        let report = engine
            .harmonize(HarmonizationStrategy::MostRestrictive)
            .expect("h");
        let json = serde_json::to_string(&report).expect("ser");
        let back: HarmonizationReport = serde_json::from_str(&json).expect("de");
        assert_eq!(report, back);
    }

    #[test]
    fn test_engine_serde_roundtrip() {
        let engine = engine_two();
        let json = serde_json::to_string(&engine).expect("ser");
        let back: HarmonizationEngine = serde_json::from_str(&json).expect("de");
        assert_eq!(engine, back);
    }
}
