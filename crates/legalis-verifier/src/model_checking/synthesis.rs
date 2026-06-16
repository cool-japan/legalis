//! Temporal property synthesis from labelled example traces.
//!
//! Given a set of *positive* traces (which should satisfy the sought property)
//! and *negative* traces (which should not), this module infers a candidate LTL
//! formula that separates them. Rather than the four hard-coded shapes of
//! [`crate::synthesize_ltl_property`], it instantiates a library of Dwyer-style
//! *specification patterns* (absence, existence, universality, response,
//! precedence, ...) over the atomic propositions observed in the traces, scores
//! every candidate by how well it separates the examples, and returns a ranked
//! list together with the best separating (or, failing that, best partial)
//! formula.
//!
//! The finite-trace LTL semantics are reused from
//! [`crate::functions_3::check_formula_on_trace`] so behaviour stays consistent
//! with the rest of the crate.

use std::collections::{HashSet, VecDeque};

use crate::LtlFormula;
use crate::functions_3::check_formula_on_trace;

/// A trace is a finite sequence of states, each labelled with the propositions
/// that hold in it.
pub type Trace = Vec<HashSet<String>>;

/// The family of specification patterns the synthesizer can instantiate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SpecificationPattern {
    /// `G ¬p` — `p` never holds.
    Absence,
    /// `F p` — `p` eventually holds.
    Existence,
    /// `G p` — `p` always holds.
    Universality,
    /// `F G p` — `p` eventually holds forever.
    EventuallyStable,
    /// `G F p` — `p` holds infinitely often.
    InfinitelyOften,
    /// `G (p → F q)` — every `p` is eventually followed by `q`.
    Response,
    /// `G (p → q)` — every `p` is accompanied by `q`.
    ImmediateResponse,
    /// `G ¬q ∨ (¬q U p)` — `p` precedes `q`.
    Precedence,
    /// `p U q` — `p` holds until `q`.
    Until,
}

impl SpecificationPattern {
    /// A short human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            SpecificationPattern::Absence => "absence",
            SpecificationPattern::Existence => "existence",
            SpecificationPattern::Universality => "universality",
            SpecificationPattern::EventuallyStable => "eventually-stable",
            SpecificationPattern::InfinitelyOften => "infinitely-often",
            SpecificationPattern::Response => "response",
            SpecificationPattern::ImmediateResponse => "immediate-response",
            SpecificationPattern::Precedence => "precedence",
            SpecificationPattern::Until => "until",
        }
    }

    /// Whether the pattern operates over a pair of propositions.
    pub fn is_binary(&self) -> bool {
        matches!(
            self,
            SpecificationPattern::Response
                | SpecificationPattern::ImmediateResponse
                | SpecificationPattern::Precedence
                | SpecificationPattern::Until
        )
    }
}

/// A scored candidate formula.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ScoredCandidate {
    /// The candidate LTL formula.
    pub formula: LtlFormula,
    /// The pattern the candidate was instantiated from.
    pub pattern: SpecificationPattern,
    /// The propositions plugged into the pattern.
    pub propositions: Vec<String>,
    /// Number of positive traces the formula satisfies.
    pub positives_matched: usize,
    /// Number of negative traces the formula rejects.
    pub negatives_rejected: usize,
    /// Separation score in `[0, 1]` (1.0 means a perfect separator).
    pub score: f64,
    /// Whether the candidate perfectly separates positives from negatives.
    pub separates: bool,
}

/// The result of a synthesis run.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SynthesisOutcome {
    /// The best formula found: a perfect separator if one exists, otherwise the
    /// highest-scoring partial candidate.
    pub best: Option<LtlFormula>,
    /// Whether `best` perfectly separates the examples.
    pub separated: bool,
    /// All candidates, ranked best-first.
    pub candidates: Vec<ScoredCandidate>,
}

/// Synthesizes temporal properties from labelled example traces.
#[derive(Debug, Clone)]
pub struct TemporalPropertySynthesizer {
    /// Maximum number of ranked candidates to retain in the outcome.
    pub max_candidates: usize,
}

impl Default for TemporalPropertySynthesizer {
    fn default() -> Self {
        Self { max_candidates: 32 }
    }
}

impl TemporalPropertySynthesizer {
    /// Creates a synthesizer with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum number of ranked candidates kept in the outcome.
    pub fn with_max_candidates(mut self, max_candidates: usize) -> Self {
        self.max_candidates = max_candidates;
        self
    }

    /// Synthesizes a separating temporal property from the given examples.
    pub fn synthesize(&self, positive: &[Trace], negative: &[Trace]) -> SynthesisOutcome {
        let props = collect_propositions(positive, negative);
        if props.is_empty() {
            return SynthesisOutcome {
                best: None,
                separated: false,
                candidates: Vec::new(),
            };
        }

        let mut candidates: Vec<ScoredCandidate> = Vec::new();
        for prop in &props {
            for pattern in [
                SpecificationPattern::Absence,
                SpecificationPattern::Existence,
                SpecificationPattern::Universality,
                SpecificationPattern::EventuallyStable,
                SpecificationPattern::InfinitelyOften,
            ] {
                let formula = instantiate(pattern, std::slice::from_ref(prop));
                candidates.push(score(
                    formula,
                    pattern,
                    vec![prop.clone()],
                    positive,
                    negative,
                ));
            }
        }
        for p in &props {
            for q in &props {
                if p == q {
                    continue;
                }
                for pattern in [
                    SpecificationPattern::Response,
                    SpecificationPattern::ImmediateResponse,
                    SpecificationPattern::Precedence,
                    SpecificationPattern::Until,
                ] {
                    let formula = instantiate(pattern, &[p.clone(), q.clone()]);
                    candidates.push(score(
                        formula,
                        pattern,
                        vec![p.clone(), q.clone()],
                        positive,
                        negative,
                    ));
                }
            }
        }

        // Rank: perfect separators first, then by score, then by simplicity.
        candidates.sort_by(|a, b| {
            b.separates
                .cmp(&a.separates)
                .then(
                    b.score
                        .partial_cmp(&a.score)
                        .unwrap_or(std::cmp::Ordering::Equal),
                )
                .then(formula_size(&a.formula).cmp(&formula_size(&b.formula)))
        });

        let best_separating = candidates
            .iter()
            .filter(|c| c.separates)
            .min_by_key(|c| formula_size(&c.formula))
            .map(|c| c.formula.clone());
        let separated = best_separating.is_some();
        let best = best_separating.or_else(|| candidates.first().map(|c| c.formula.clone()));

        candidates.truncate(self.max_candidates);

        SynthesisOutcome {
            best,
            separated,
            candidates,
        }
    }
}

/// Collects every proposition appearing anywhere in the traces.
fn collect_propositions(positive: &[Trace], negative: &[Trace]) -> Vec<String> {
    let mut set: HashSet<String> = HashSet::new();
    for trace in positive.iter().chain(negative.iter()) {
        for state in trace {
            for prop in state {
                set.insert(prop.clone());
            }
        }
    }
    let mut props: Vec<String> = set.into_iter().collect();
    props.sort();
    props
}

/// Instantiates a pattern over the supplied propositions.
fn instantiate(pattern: SpecificationPattern, props: &[String]) -> LtlFormula {
    let p = || LtlFormula::atom(props[0].clone());
    let q = || LtlFormula::atom(props[1].clone());
    match pattern {
        SpecificationPattern::Absence => LtlFormula::always(LtlFormula::not(p())),
        SpecificationPattern::Existence => LtlFormula::eventually(p()),
        SpecificationPattern::Universality => LtlFormula::always(p()),
        SpecificationPattern::EventuallyStable => LtlFormula::eventually(LtlFormula::always(p())),
        SpecificationPattern::InfinitelyOften => LtlFormula::always(LtlFormula::eventually(p())),
        SpecificationPattern::Response => {
            LtlFormula::always(LtlFormula::implies(p(), LtlFormula::eventually(q())))
        }
        SpecificationPattern::ImmediateResponse => {
            LtlFormula::always(LtlFormula::implies(p(), q()))
        }
        SpecificationPattern::Precedence => LtlFormula::or(
            LtlFormula::always(LtlFormula::not(q())),
            LtlFormula::until(LtlFormula::not(q()), p()),
        ),
        SpecificationPattern::Until => LtlFormula::until(p(), q()),
    }
}

/// Scores a candidate by how well it separates the labelled examples.
fn score(
    formula: LtlFormula,
    pattern: SpecificationPattern,
    propositions: Vec<String>,
    positive: &[Trace],
    negative: &[Trace],
) -> ScoredCandidate {
    let positives_matched = positive
        .iter()
        .filter(|t| check_formula_on_trace(&formula, t))
        .count();
    let negatives_rejected = negative
        .iter()
        .filter(|t| !check_formula_on_trace(&formula, t))
        .count();

    let pos_ratio = if positive.is_empty() {
        1.0
    } else {
        positives_matched as f64 / positive.len() as f64
    };
    let neg_ratio = if negative.is_empty() {
        1.0
    } else {
        negatives_rejected as f64 / negative.len() as f64
    };
    let score = (pos_ratio + neg_ratio) / 2.0;
    let separates = positives_matched == positive.len()
        && negatives_rejected == negative.len()
        && !(positive.is_empty() && negative.is_empty());

    ScoredCandidate {
        formula,
        pattern,
        propositions,
        positives_matched,
        negatives_rejected,
        score,
        separates,
    }
}

/// Counts the number of nodes in an LTL formula (its syntactic size).
pub fn formula_size(formula: &LtlFormula) -> usize {
    let mut count = 0usize;
    let mut stack: VecDeque<&LtlFormula> = VecDeque::new();
    stack.push_back(formula);
    while let Some(f) = stack.pop_back() {
        count += 1;
        match f {
            LtlFormula::Atom(_) => {}
            LtlFormula::Not(a)
            | LtlFormula::Next(a)
            | LtlFormula::Eventually(a)
            | LtlFormula::Always(a) => stack.push_back(a),
            LtlFormula::And(a, b)
            | LtlFormula::Or(a, b)
            | LtlFormula::Implies(a, b)
            | LtlFormula::Until(a, b)
            | LtlFormula::Release(a, b) => {
                stack.push_back(a);
                stack.push_back(b);
            }
        }
    }
    count
}
