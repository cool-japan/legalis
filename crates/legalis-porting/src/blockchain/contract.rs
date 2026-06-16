//! Smart-contract enforcement for ports.
//!
//! A [`PortingCovenant`] is a deterministic "smart contract": an ordered list of
//! [`Clause`]s, each pairing a [`Gate`] predicate over a [`PortingFacts`] with a
//! [`ClauseKind`] describing whether the clause *blocks* a port (mandatory) or
//! merely *warns* (advisory). Enforcement is performed by a gas-metered
//! [`ContractEngine`]: every gate node consumes gas, so a pathological covenant
//! fails cleanly rather than evaluating without bound.
//!
//! The engine *gates a port on conditions being met*: a port is allowed only
//! when every mandatory clause is satisfied. Gates reuse the crate-wide
//! [`legalis_core::Condition`] language — a [`Gate::CoreCondition`] is evaluated
//! against the ported statute's attribute facts via
//! [`legalis_core::Condition::evaluate_simple`] — alongside porting-specific
//! predicates such as a compatibility floor, a change budget, and requirements
//! that decentralized consensus and cross-border notarization have completed.

use crate::PortedStatute;
use crate::PortingError;
use legalis_core::{AttributeBasedContext, Condition};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

type ContractResult<T> = Result<T, PortingError>;

/// Default per-covenant gas budget.
pub const DEFAULT_GAS_LIMIT: u64 = 100_000;

/// Gas charged for evaluating a single gate node.
const GATE_GAS: u64 = 1;
/// Gas charged per sub-condition when evaluating a [`Gate::CoreCondition`].
const CONDITION_GAS: u64 = 2;

/// The facts a covenant is evaluated against.
///
/// Combines porting-specific metadata (compatibility, change count, the state of
/// consensus and notarization, the set of jurisdictions that have approved) with
/// a free-form attribute map used to evaluate [`Gate::CoreCondition`] gates.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PortingFacts {
    /// Source jurisdiction code.
    pub source_jurisdiction: String,
    /// Target jurisdiction code.
    pub target_jurisdiction: String,
    /// Compatibility score of the port (0.0 - 1.0).
    pub compatibility_score: f64,
    /// Number of adaptations the port made.
    pub change_count: usize,
    /// Whether decentralized approval consensus has committed this port.
    pub consensus_committed: bool,
    /// Whether cross-border notarization has completed.
    pub notarized: bool,
    /// Jurisdiction codes that have approved the port.
    pub approved_jurisdictions: BTreeSet<String>,
    /// Attribute facts evaluated by [`Gate::CoreCondition`].
    pub attributes: HashMap<String, String>,
}

impl PortingFacts {
    /// Builds facts from a ported statute, injecting standard derived attributes
    /// (`compatibility_score`, `change_count`, `source_jurisdiction`,
    /// `target_jurisdiction`) so they are also addressable from
    /// [`Gate::CoreCondition`] gates.
    pub fn from_ported(
        ported: &PortedStatute,
        source_jurisdiction: impl Into<String>,
        target_jurisdiction: impl Into<String>,
    ) -> Self {
        let source = source_jurisdiction.into();
        let target = target_jurisdiction.into();
        let mut attributes = HashMap::new();
        attributes.insert(
            "compatibility_score".to_string(),
            format!("{:.4}", ported.compatibility_score),
        );
        attributes.insert("change_count".to_string(), ported.changes.len().to_string());
        attributes.insert("source_jurisdiction".to_string(), source.clone());
        attributes.insert("target_jurisdiction".to_string(), target.clone());
        Self {
            source_jurisdiction: source,
            target_jurisdiction: target,
            compatibility_score: ported.compatibility_score,
            change_count: ported.changes.len(),
            consensus_committed: false,
            notarized: false,
            approved_jurisdictions: BTreeSet::new(),
            attributes,
        }
    }

    /// Sets a free-form attribute used by [`Gate::CoreCondition`].
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Marks decentralized consensus as committed (or not).
    pub fn with_consensus(mut self, committed: bool) -> Self {
        self.consensus_committed = committed;
        self
    }

    /// Marks cross-border notarization as complete (or not).
    pub fn with_notarization(mut self, notarized: bool) -> Self {
        self.notarized = notarized;
        self
    }

    /// Records that `jurisdiction` has approved the port.
    pub fn with_approval(mut self, jurisdiction: impl Into<String>) -> Self {
        self.approved_jurisdictions.insert(jurisdiction.into());
        self
    }

    /// The attribute-based evaluation context for [`legalis_core::Condition`].
    fn eval_context(&self) -> AttributeBasedContext {
        AttributeBasedContext::new(self.attributes.clone())
    }
}

/// A predicate over [`PortingFacts`].
///
/// Gates compose with [`Gate::and`], [`Gate::or`] and [`Gate::negate`] into an
/// evaluable AST. A [`Gate::CoreCondition`] embeds a [`legalis_core::Condition`]
/// so the full crate-wide condition language is reusable verbatim inside a
/// covenant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Gate {
    /// Always satisfied.
    Always,
    /// Satisfied if the compatibility score is at least the given floor.
    CompatibilityAtLeast(f64),
    /// Satisfied if the port made at most this many changes.
    MaxChanges(usize),
    /// Satisfied if decentralized consensus has committed the port.
    RequiresConsensus,
    /// Satisfied if cross-border notarization has completed.
    RequiresNotarization,
    /// Satisfied if the named jurisdiction has approved the port.
    JurisdictionApproved(String),
    /// Satisfied if the embedded core condition evaluates true against the
    /// fact attributes. Missing/unparseable attributes coerce to `false`.
    CoreCondition(Condition),
    /// Logical conjunction.
    And(Box<Gate>, Box<Gate>),
    /// Logical disjunction.
    Or(Box<Gate>, Box<Gate>),
    /// Logical negation.
    Not(Box<Gate>),
}

impl Gate {
    /// Conjunction combinator.
    pub fn and(self, other: Gate) -> Gate {
        Gate::And(Box::new(self), Box::new(other))
    }

    /// Disjunction combinator.
    pub fn or(self, other: Gate) -> Gate {
        Gate::Or(Box::new(self), Box::new(other))
    }

    /// Negation combinator.
    pub fn negate(self) -> Gate {
        Gate::Not(Box::new(self))
    }

    /// A short human-readable description of what this gate requires.
    pub fn describe(&self) -> String {
        match self {
            Gate::Always => "always".to_string(),
            Gate::CompatibilityAtLeast(v) => format!("compatibility >= {v:.4}"),
            Gate::MaxChanges(n) => format!("changes <= {n}"),
            Gate::RequiresConsensus => "consensus committed".to_string(),
            Gate::RequiresNotarization => "notarization complete".to_string(),
            Gate::JurisdictionApproved(j) => format!("jurisdiction '{j}' approved"),
            Gate::CoreCondition(_) => "core condition holds".to_string(),
            Gate::And(a, b) => format!("({} AND {})", a.describe(), b.describe()),
            Gate::Or(a, b) => format!("({} OR {})", a.describe(), b.describe()),
            Gate::Not(a) => format!("NOT ({})", a.describe()),
        }
    }

    /// Evaluates the gate against `facts`, charging gas as it descends.
    fn evaluate(&self, facts: &PortingFacts, gas: &mut u64, limit: u64) -> ContractResult<bool> {
        *gas = gas.saturating_add(GATE_GAS);
        if *gas > limit {
            return Err(PortingError::InvalidInput(format!(
                "contract: gas budget {limit} exhausted while evaluating covenant"
            )));
        }
        match self {
            Gate::Always => Ok(true),
            Gate::CompatibilityAtLeast(min) => Ok(facts.compatibility_score >= *min),
            Gate::MaxChanges(max) => Ok(facts.change_count <= *max),
            Gate::RequiresConsensus => Ok(facts.consensus_committed),
            Gate::RequiresNotarization => Ok(facts.notarized),
            Gate::JurisdictionApproved(j) => Ok(facts.approved_jurisdictions.contains(j)),
            Gate::CoreCondition(condition) => {
                let cost = (condition.count_conditions() as u64).saturating_mul(CONDITION_GAS);
                *gas = gas.saturating_add(cost);
                if *gas > limit {
                    return Err(PortingError::InvalidInput(format!(
                        "contract: gas budget {limit} exhausted evaluating core condition"
                    )));
                }
                // A condition that references missing/unparseable attributes is
                // treated as not demonstrably satisfied (false) so a mandatory
                // clause blocks the port rather than aborting enforcement.
                Ok(condition
                    .evaluate_simple(&facts.eval_context())
                    .unwrap_or(false))
            }
            Gate::And(a, b) => {
                if !a.evaluate(facts, gas, limit)? {
                    return Ok(false);
                }
                b.evaluate(facts, gas, limit)
            }
            Gate::Or(a, b) => {
                if a.evaluate(facts, gas, limit)? {
                    return Ok(true);
                }
                b.evaluate(facts, gas, limit)
            }
            Gate::Not(a) => Ok(!a.evaluate(facts, gas, limit)?),
        }
    }
}

/// Whether a clause blocks a port or merely warns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClauseKind {
    /// The clause is mandatory: if its gate is not satisfied the port is blocked.
    MustHold,
    /// The clause is advisory: an unsatisfied gate produces a warning only.
    ShouldHold,
}

/// A single covenant clause.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clause {
    /// Stable clause identifier.
    pub id: String,
    /// Human-readable description of the clause's intent.
    pub description: String,
    /// The condition under which the clause is satisfied.
    pub gate: Gate,
    /// Whether the clause blocks or merely warns.
    pub kind: ClauseKind,
}

impl Clause {
    /// Creates a mandatory clause whose gate must hold for a port to proceed.
    pub fn must_hold(id: impl Into<String>, description: impl Into<String>, gate: Gate) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            gate,
            kind: ClauseKind::MustHold,
        }
    }

    /// Creates an advisory clause whose unsatisfied gate produces a warning.
    pub fn should_hold(id: impl Into<String>, description: impl Into<String>, gate: Gate) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            gate,
            kind: ClauseKind::ShouldHold,
        }
    }
}

/// A named, ordered set of clauses enforced atomically over a port.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortingCovenant {
    /// Stable covenant identifier.
    pub id: String,
    /// The clauses, evaluated in order.
    pub clauses: Vec<Clause>,
}

impl PortingCovenant {
    /// Creates an empty covenant with the given id.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            clauses: Vec::new(),
        }
    }

    /// Appends a clause (builder style).
    pub fn with_clause(mut self, clause: Clause) -> Self {
        self.clauses.push(clause);
        self
    }

    /// Number of clauses.
    pub fn len(&self) -> usize {
        self.clauses.len()
    }

    /// Whether the covenant has no clauses.
    pub fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }
}

/// A mandatory clause whose gate was not satisfied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Violation {
    /// The clause that was violated.
    pub clause_id: String,
    /// Why the port was blocked.
    pub reason: String,
}

/// The outcome of enforcing a covenant over a set of facts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnforcementReceipt {
    /// The covenant that was enforced.
    pub covenant_id: String,
    /// Whether the port is allowed (no mandatory clause was violated).
    pub allowed: bool,
    /// Gas consumed while evaluating the covenant.
    pub gas_used: u64,
    /// Ids of clauses whose gate was satisfied.
    pub satisfied: Vec<String>,
    /// Mandatory clauses that blocked the port.
    pub violations: Vec<Violation>,
    /// Advisory clauses that were not satisfied.
    pub warnings: Vec<String>,
}

/// A deterministic, gas-metered engine that enforces covenants over ports.
#[derive(Debug, Clone)]
pub struct ContractEngine {
    gas_limit: u64,
}

impl Default for ContractEngine {
    fn default() -> Self {
        Self {
            gas_limit: DEFAULT_GAS_LIMIT,
        }
    }
}

impl ContractEngine {
    /// Creates an engine with the default gas budget.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the gas budget (a budget of zero is raised to one so the genesis
    /// gate charge can always be accounted).
    pub fn with_gas_limit(mut self, gas_limit: u64) -> Self {
        self.gas_limit = gas_limit.max(1);
        self
    }

    /// The configured gas budget.
    pub fn gas_limit(&self) -> u64 {
        self.gas_limit
    }

    /// Enforces `covenant` against `facts`, gating the port on every mandatory
    /// clause being satisfied.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if evaluation exceeds the gas
    /// budget.
    pub fn enforce(
        &self,
        covenant: &PortingCovenant,
        facts: &PortingFacts,
    ) -> ContractResult<EnforcementReceipt> {
        let mut gas: u64 = 0;
        let mut satisfied = Vec::new();
        let mut violations = Vec::new();
        let mut warnings = Vec::new();

        for clause in &covenant.clauses {
            let ok = clause.gate.evaluate(facts, &mut gas, self.gas_limit)?;
            if ok {
                satisfied.push(clause.id.clone());
            } else {
                match clause.kind {
                    ClauseKind::MustHold => violations.push(Violation {
                        clause_id: clause.id.clone(),
                        reason: format!(
                            "{}: required {} was not satisfied",
                            clause.description,
                            clause.gate.describe()
                        ),
                    }),
                    ClauseKind::ShouldHold => warnings.push(format!(
                        "{}: advisory {} was not satisfied",
                        clause.description,
                        clause.gate.describe()
                    )),
                }
            }
        }

        Ok(EnforcementReceipt {
            covenant_id: covenant.id.clone(),
            allowed: violations.is_empty(),
            gas_used: gas,
            satisfied,
            violations,
            warnings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ChangeType, PortingChange};
    use legalis_core::{ComparisonOp, Effect, EffectType, Statute};
    use legalis_i18n::Locale;

    fn ported(score: f64, changes: usize) -> PortedStatute {
        let mut change_vec = Vec::new();
        for i in 0..changes {
            change_vec.push(PortingChange {
                change_type: ChangeType::ValueAdaptation,
                description: format!("c{i}"),
                original: None,
                adapted: None,
                reason: "r".to_string(),
            });
        }
        PortedStatute {
            original_id: "src".to_string(),
            statute: Statute::new("dst", "Dst", Effect::new(EffectType::Grant, "B")),
            changes: change_vec,
            locale: Locale::new("en").with_country("US"),
            compatibility_score: score,
        }
    }

    fn standard_covenant() -> PortingCovenant {
        PortingCovenant::new("standard")
            .with_clause(Clause::must_hold(
                "compat",
                "Minimum compatibility",
                Gate::CompatibilityAtLeast(0.7),
            ))
            .with_clause(Clause::must_hold(
                "consensus",
                "Decentralized approval",
                Gate::RequiresConsensus,
            ))
            .with_clause(Clause::must_hold(
                "notary",
                "Cross-border notarization",
                Gate::RequiresNotarization,
            ))
            .with_clause(Clause::should_hold(
                "low-churn",
                "Few adaptations preferred",
                Gate::MaxChanges(3),
            ))
    }

    #[test]
    fn test_allows_when_all_mandatory_hold() {
        let facts = PortingFacts::from_ported(&ported(0.9, 1), "JP", "US")
            .with_consensus(true)
            .with_notarization(true);
        let receipt = ContractEngine::new()
            .enforce(&standard_covenant(), &facts)
            .expect("enforce");
        assert!(receipt.allowed);
        assert!(receipt.violations.is_empty());
        assert!(receipt.warnings.is_empty());
        assert_eq!(receipt.satisfied.len(), 4);
        assert!(receipt.gas_used > 0);
    }

    #[test]
    fn test_blocks_when_consensus_missing() {
        let facts = PortingFacts::from_ported(&ported(0.9, 1), "JP", "US").with_notarization(true);
        let receipt = ContractEngine::new()
            .enforce(&standard_covenant(), &facts)
            .expect("enforce");
        assert!(!receipt.allowed);
        assert_eq!(receipt.violations.len(), 1);
        assert_eq!(receipt.violations[0].clause_id, "consensus");
    }

    #[test]
    fn test_low_compat_blocks_and_high_churn_warns() {
        let facts = PortingFacts::from_ported(&ported(0.5, 9), "JP", "US")
            .with_consensus(true)
            .with_notarization(true);
        let receipt = ContractEngine::new()
            .enforce(&standard_covenant(), &facts)
            .expect("enforce");
        assert!(!receipt.allowed);
        assert!(receipt.violations.iter().any(|v| v.clause_id == "compat"));
        assert_eq!(receipt.warnings.len(), 1); // low-churn advisory tripped
    }

    #[test]
    fn test_core_condition_gate_reuses_legalis_core() {
        // Reuse the crate-wide Condition language: require age >= 18 from facts.
        let gate = Gate::CoreCondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });
        let covenant = PortingCovenant::new("age").with_clause(Clause::must_hold(
            "age",
            "Age of majority",
            gate,
        ));
        let engine = ContractEngine::new();

        let adult =
            PortingFacts::from_ported(&ported(1.0, 0), "JP", "US").with_attribute("age", "20");
        assert!(engine.enforce(&covenant, &adult).expect("e").allowed);

        let minor =
            PortingFacts::from_ported(&ported(1.0, 0), "JP", "US").with_attribute("age", "16");
        assert!(!engine.enforce(&covenant, &minor).expect("e").allowed);
    }

    #[test]
    fn test_core_condition_missing_attribute_is_false() {
        let gate = Gate::CoreCondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });
        let covenant =
            PortingCovenant::new("age").with_clause(Clause::must_hold("age", "Age", gate));
        // No "age" attribute -> condition coerces to false -> mandatory clause blocks.
        let facts = PortingFacts::from_ported(&ported(1.0, 0), "JP", "US");
        assert!(
            !ContractEngine::new()
                .enforce(&covenant, &facts)
                .expect("e")
                .allowed
        );
    }

    #[test]
    fn test_injected_attributes_addressable_by_condition() {
        // from_ported injects target_jurisdiction as an attribute.
        let gate = Gate::CoreCondition(Condition::AttributeEquals {
            key: "target_jurisdiction".to_string(),
            value: "US".to_string(),
        });
        let covenant =
            PortingCovenant::new("tgt").with_clause(Clause::must_hold("tgt", "Target US", gate));
        let facts = PortingFacts::from_ported(&ported(1.0, 0), "JP", "US").with_consensus(true);
        assert!(
            ContractEngine::new()
                .enforce(&covenant, &facts)
                .expect("e")
                .allowed
        );
    }

    #[test]
    fn test_gate_combinators() {
        let gate = Gate::CompatibilityAtLeast(0.8)
            .and(Gate::RequiresConsensus)
            .or(Gate::JurisdictionApproved("US".to_string()))
            .negate()
            .negate();
        let covenant =
            PortingCovenant::new("combo").with_clause(Clause::must_hold("c", "combo", gate));
        let engine = ContractEngine::new();

        let via_approval =
            PortingFacts::from_ported(&ported(0.1, 0), "JP", "US").with_approval("US");
        assert!(engine.enforce(&covenant, &via_approval).expect("e").allowed);

        let neither = PortingFacts::from_ported(&ported(0.1, 0), "JP", "US");
        assert!(!engine.enforce(&covenant, &neither).expect("e").allowed);
    }

    #[test]
    fn test_jurisdiction_approved_gate() {
        let gate = Gate::JurisdictionApproved("DE".to_string());
        let covenant =
            PortingCovenant::new("j").with_clause(Clause::must_hold("j", "DE approval", gate));
        let facts = PortingFacts::from_ported(&ported(1.0, 0), "FR", "DE").with_approval("DE");
        assert!(
            ContractEngine::new()
                .enforce(&covenant, &facts)
                .expect("e")
                .allowed
        );
    }

    #[test]
    fn test_gas_exhaustion_errors() {
        // A deeply nested gate exceeds a tiny gas budget.
        let mut gate = Gate::Always;
        for _ in 0..50 {
            gate = gate.and(Gate::Always);
        }
        let covenant =
            PortingCovenant::new("deep").with_clause(Clause::must_hold("deep", "deep", gate));
        let engine = ContractEngine::new().with_gas_limit(5);
        let facts = PortingFacts::default();
        assert!(engine.enforce(&covenant, &facts).is_err());
    }

    #[test]
    fn test_gas_limit_floor() {
        let engine = ContractEngine::new().with_gas_limit(0);
        assert_eq!(engine.gas_limit(), 1);
    }

    #[test]
    fn test_empty_covenant_allows() {
        let covenant = PortingCovenant::new("empty");
        assert!(covenant.is_empty());
        let receipt = ContractEngine::new()
            .enforce(&covenant, &PortingFacts::default())
            .expect("enforce");
        assert!(receipt.allowed);
        assert_eq!(receipt.gas_used, 0);
    }

    #[test]
    fn test_receipt_serde_roundtrip() {
        let facts = PortingFacts::from_ported(&ported(0.4, 0), "JP", "US");
        let receipt = ContractEngine::new()
            .enforce(&standard_covenant(), &facts)
            .expect("enforce");
        let json = serde_json::to_string(&receipt).expect("ser");
        let back: EnforcementReceipt = serde_json::from_str(&json).expect("de");
        assert_eq!(receipt, back);
    }

    #[test]
    fn test_describe_is_stable() {
        let gate = Gate::CompatibilityAtLeast(0.5).and(Gate::RequiresConsensus);
        assert_eq!(
            gate.describe(),
            "(compatibility >= 0.5000 AND consensus committed)"
        );
    }
}
