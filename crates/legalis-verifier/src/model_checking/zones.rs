//! Timed-automaton verification via Difference Bound Matrices (DBMs).
//!
//! This module provides a genuine *symbolic* analysis of timed automata using
//! zone graphs, replacing the integer-step enumeration of
//! [`crate::verify_timed_reachability`]. A zone (a convex set of clock
//! valuations) is represented as a canonical [`DifferenceBoundMatrix`] with the
//! standard operations:
//!
//! * `canonicalize` — tighten all bounds via shortest paths (Floyd–Warshall);
//! * `and_constraint` (intersect) — conjoin a guard/invariant;
//! * `reset` — set a clock back to zero;
//! * `up` (delay) — let time elapse, removing upper bounds;
//! * `extrapolate` — `k`-normalization, guaranteeing a finite zone graph;
//! * `includes` — zone inclusion, used to subsume already-explored zones.
//!
//! On top of these, [`reachable_zone_states`] builds the zone graph and
//! [`check_deadline_satisfaction`] decides whether a deadline (a clock bound at a
//! target location) is *reachable*, and whether it is *guaranteed* on every path.

use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

use crate::{Clock, ClockConstraint, DeadlineViolation, TimedAutomaton, VerificationError};

/// A bound entry in a DBM: either `< ∞` or a finite value with a strictness flag.
///
/// The constraint encoded at matrix cell `(i, j)` is `x_i - x_j ≺ value`, where
/// `≺` is `<` when `strict` and `≤` otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DbmBound {
    /// No upper bound (positive infinity).
    Inf,
    /// A finite bound `value`, strict (`<`) or non-strict (`≤`).
    Finite {
        /// The numeric bound.
        value: i64,
        /// Whether the comparison is strict (`<`).
        strict: bool,
    },
}

impl DbmBound {
    /// The bound `(0, ≤)`.
    pub fn zero() -> Self {
        DbmBound::Finite {
            value: 0,
            strict: false,
        }
    }

    /// A finite bound constructor.
    pub fn finite(value: i64, strict: bool) -> Self {
        DbmBound::Finite { value, strict }
    }

    /// The numeric value of the bound (`i64::MAX` for [`DbmBound::Inf`]).
    pub fn value(&self) -> i64 {
        match self {
            DbmBound::Inf => i64::MAX,
            DbmBound::Finite { value, .. } => *value,
        }
    }

    /// Adds two bounds (saturating at infinity).
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (DbmBound::Inf, _) | (_, DbmBound::Inf) => DbmBound::Inf,
            (
                DbmBound::Finite {
                    value: a,
                    strict: sa,
                },
                DbmBound::Finite {
                    value: b,
                    strict: sb,
                },
            ) => DbmBound::Finite {
                value: a.saturating_add(b),
                strict: sa || sb,
            },
        }
    }
}

impl PartialOrd for DbmBound {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DbmBound {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (DbmBound::Inf, DbmBound::Inf) => Ordering::Equal,
            (DbmBound::Inf, _) => Ordering::Greater,
            (_, DbmBound::Inf) => Ordering::Less,
            (
                DbmBound::Finite {
                    value: a,
                    strict: sa,
                },
                DbmBound::Finite {
                    value: b,
                    strict: sb,
                },
            ) => a.cmp(b).then_with(|| match (sa, sb) {
                // At equal value, `<` (strict) is the tighter/smaller bound.
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => Ordering::Equal,
            }),
        }
    }
}

/// A canonical Difference Bound Matrix over a fixed set of clocks.
///
/// Index `0` is the implicit reference clock that is always `0`; clock `clocks[k]`
/// occupies matrix index `k + 1`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DifferenceBoundMatrix {
    dim: usize,
    matrix: Vec<DbmBound>,
    clocks: Vec<String>,
}

impl DifferenceBoundMatrix {
    /// Creates the zone in which every clock equals zero.
    pub fn zero(clocks: &[String]) -> Self {
        let dim = clocks.len() + 1;
        Self {
            dim,
            matrix: vec![DbmBound::zero(); dim * dim],
            clocks: clocks.to_vec(),
        }
    }

    /// The number of clocks plus the reference clock.
    pub fn dimension(&self) -> usize {
        self.dim
    }

    /// Reads the bound at cell `(i, j)`.
    pub fn bound(&self, i: usize, j: usize) -> DbmBound {
        self.matrix[i * self.dim + j]
    }

    fn set(&mut self, i: usize, j: usize, bound: DbmBound) {
        self.matrix[i * self.dim + j] = bound;
    }

    fn index_of(&self, clock: &str) -> Result<usize, VerificationError> {
        self.clocks
            .iter()
            .position(|c| c == clock)
            .map(|p| p + 1)
            .ok_or_else(|| VerificationError::LogicalContradiction {
                message: format!("unknown clock '{}' in zone", clock),
            })
    }

    /// Tightens the matrix via shortest paths (Floyd–Warshall).
    pub fn canonicalize(&mut self) {
        let n = self.dim;
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    let through = self.bound(i, k).add(self.bound(k, j));
                    if through < self.bound(i, j) {
                        self.set(i, j, through);
                    }
                }
            }
        }
    }

    /// Whether the zone is empty (contains a negative cycle).
    pub fn is_empty(&self) -> bool {
        (0..self.dim).any(|i| self.bound(i, i) < DbmBound::zero())
    }

    fn tighten_cell(&mut self, i: usize, j: usize, bound: DbmBound) {
        if bound < self.bound(i, j) {
            self.set(i, j, bound);
        }
    }

    fn tighten(&mut self, constraint: &ClockConstraint) -> Result<(), VerificationError> {
        match constraint {
            ClockConstraint::Less(clock, v) => {
                let i = self.index_of(&clock.name)?;
                self.tighten_cell(i, 0, DbmBound::finite(*v as i64, true));
            }
            ClockConstraint::LessOrEqual(clock, v) => {
                let i = self.index_of(&clock.name)?;
                self.tighten_cell(i, 0, DbmBound::finite(*v as i64, false));
            }
            ClockConstraint::Greater(clock, v) => {
                let i = self.index_of(&clock.name)?;
                self.tighten_cell(0, i, DbmBound::finite(-(*v as i64), true));
            }
            ClockConstraint::GreaterOrEqual(clock, v) => {
                let i = self.index_of(&clock.name)?;
                self.tighten_cell(0, i, DbmBound::finite(-(*v as i64), false));
            }
            ClockConstraint::Equal(clock, v) => {
                let i = self.index_of(&clock.name)?;
                self.tighten_cell(i, 0, DbmBound::finite(*v as i64, false));
                self.tighten_cell(0, i, DbmBound::finite(-(*v as i64), false));
            }
            ClockConstraint::And(a, b) => {
                self.tighten(a)?;
                self.tighten(b)?;
            }
        }
        Ok(())
    }

    /// Intersects the zone with a clock constraint, re-canonicalizing afterwards.
    pub fn and_constraint(
        &mut self,
        constraint: &ClockConstraint,
    ) -> Result<(), VerificationError> {
        self.tighten(constraint)?;
        self.canonicalize();
        Ok(())
    }

    /// Lets time elapse: removes upper bounds on every clock (the `up` operator).
    pub fn up(&mut self) {
        for i in 1..self.dim {
            self.set(i, 0, DbmBound::Inf);
        }
        self.canonicalize();
    }

    /// Resets `clock` to zero.
    pub fn reset(&mut self, clock: &str) -> Result<(), VerificationError> {
        let r = self.index_of(clock)?;
        let row0: Vec<DbmBound> = (0..self.dim).map(|j| self.bound(0, j)).collect();
        let col0: Vec<DbmBound> = (0..self.dim).map(|i| self.bound(i, 0)).collect();
        for j in 0..self.dim {
            self.set(r, j, row0[j]);
        }
        for i in 0..self.dim {
            self.set(i, r, col0[i]);
        }
        self.set(r, r, DbmBound::zero());
        self.set(r, 0, DbmBound::zero());
        self.set(0, r, DbmBound::zero());
        self.canonicalize();
        Ok(())
    }

    /// Classic maximal-constant (`k`) extrapolation, ensuring a finite zone graph.
    ///
    /// `k_bounds[i]` is the maximal constant relevant to matrix index `i`
    /// (`k_bounds[0]` is `0` for the reference clock).
    pub fn extrapolate(&mut self, k_bounds: &[i64]) {
        let n = self.dim;
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let cur = self.bound(i, j);
                if cur == DbmBound::Inf {
                    continue;
                }
                let ki = *k_bounds.get(i).unwrap_or(&0);
                let kj = *k_bounds.get(j).unwrap_or(&0);
                if cur.value() > ki {
                    self.set(i, j, DbmBound::Inf);
                } else if cur < DbmBound::finite(-kj, true) {
                    self.set(i, j, DbmBound::finite(-kj, true));
                }
            }
        }
        self.canonicalize();
    }

    /// Whether `self` contains `other` (i.e. `other ⊆ self`).
    pub fn includes(&self, other: &DifferenceBoundMatrix) -> bool {
        if self.dim != other.dim {
            return false;
        }
        (0..self.dim).all(|i| (0..self.dim).all(|j| other.bound(i, j) <= self.bound(i, j)))
    }
}

/// A reachable symbolic state of a timed automaton: a location plus a zone.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ZoneState {
    /// The automaton location.
    pub location: String,
    /// The reachable zone at that location.
    pub zone: DifferenceBoundMatrix,
}

/// Internal zone-graph node tracking the predecessor for path reconstruction.
struct ZoneNode {
    location: String,
    zone: DifferenceBoundMatrix,
    parent: Option<usize>,
}

/// Collects, per matrix index, the maximal constant appearing in any constraint.
fn collect_max_constants(automaton: &TimedAutomaton, extra: Option<&ClockConstraint>) -> Vec<i64> {
    let dim = automaton.clocks.len() + 1;
    let mut k = vec![0i64; dim];
    let mut name_idx: HashMap<&str, usize> = HashMap::new();
    for (i, clock) in automaton.clocks.iter().enumerate() {
        name_idx.insert(clock.name.as_str(), i + 1);
    }
    let mut consts: Vec<(String, i64)> = Vec::new();
    for location in automaton.locations.values() {
        if let Some(inv) = &location.invariant {
            constraint_constants(inv, &mut consts);
        }
    }
    for transition in &automaton.transitions {
        if let Some(guard) = &transition.guard {
            constraint_constants(guard, &mut consts);
        }
    }
    if let Some(extra) = extra {
        constraint_constants(extra, &mut consts);
    }
    for (name, value) in consts {
        if let Some(&i) = name_idx.get(name.as_str())
            && value > k[i]
        {
            k[i] = value;
        }
    }
    k
}

/// Extracts `(clock, value)` constants from a constraint tree.
fn constraint_constants(constraint: &ClockConstraint, out: &mut Vec<(String, i64)>) {
    match constraint {
        ClockConstraint::Less(c, v)
        | ClockConstraint::LessOrEqual(c, v)
        | ClockConstraint::Greater(c, v)
        | ClockConstraint::GreaterOrEqual(c, v)
        | ClockConstraint::Equal(c, v) => out.push((c.name.clone(), *v as i64)),
        ClockConstraint::And(a, b) => {
            constraint_constants(a, out);
            constraint_constants(b, out);
        }
    }
}

fn clock_names(automaton: &TimedAutomaton) -> Vec<String> {
    automaton.clocks.iter().map(|c| c.name.clone()).collect()
}

/// Builds the (extrapolated, subsumption-reduced) zone graph of `automaton`.
fn explore(
    automaton: &TimedAutomaton,
    k_bounds: &[i64],
) -> Result<Vec<ZoneNode>, VerificationError> {
    let clocks = clock_names(automaton);
    let mut nodes: Vec<ZoneNode> = Vec::new();
    let mut visited: HashMap<String, Vec<DifferenceBoundMatrix>> = HashMap::new();
    let mut work: VecDeque<usize> = VecDeque::new();

    let mut z0 = DifferenceBoundMatrix::zero(&clocks);
    z0.up();
    if let Some(loc) = automaton.locations.get(&automaton.initial)
        && let Some(inv) = &loc.invariant
    {
        z0.and_constraint(inv)?;
    }
    z0.extrapolate(k_bounds);
    if z0.is_empty() {
        return Ok(nodes);
    }
    visited
        .entry(automaton.initial.clone())
        .or_default()
        .push(z0.clone());
    nodes.push(ZoneNode {
        location: automaton.initial.clone(),
        zone: z0,
        parent: None,
    });
    work.push_back(0);

    while let Some(idx) = work.pop_front() {
        let loc = nodes[idx].location.clone();
        let zone = nodes[idx].zone.clone();
        for transition in &automaton.transitions {
            if transition.from != loc {
                continue;
            }
            let mut z = zone.clone();
            if let Some(guard) = &transition.guard {
                z.and_constraint(guard)?;
            }
            if z.is_empty() {
                continue;
            }
            for clock in &transition.resets {
                z.reset(&clock.name)?;
            }
            if let Some(target) = automaton.locations.get(&transition.to) {
                if let Some(inv) = &target.invariant {
                    z.and_constraint(inv)?;
                    if z.is_empty() {
                        continue;
                    }
                }
                z.up();
                if let Some(inv) = &target.invariant {
                    z.and_constraint(inv)?;
                    if z.is_empty() {
                        continue;
                    }
                }
            } else {
                z.up();
            }
            z.extrapolate(k_bounds);
            if z.is_empty() {
                continue;
            }
            let entry = visited.entry(transition.to.clone()).or_default();
            if entry.iter().any(|existing| existing.includes(&z)) {
                continue;
            }
            entry.push(z.clone());
            nodes.push(ZoneNode {
                location: transition.to.clone(),
                zone: z,
                parent: Some(idx),
            });
            work.push_back(nodes.len() - 1);
        }
    }
    Ok(nodes)
}

/// Reconstructs the location path leading to node `idx`.
fn path_locations(nodes: &[ZoneNode], idx: usize) -> Vec<String> {
    let mut path = vec![nodes[idx].location.clone()];
    let mut cur = idx;
    while let Some(parent) = nodes[cur].parent {
        path.push(nodes[parent].location.clone());
        cur = parent;
    }
    path.reverse();
    path
}

/// Returns the reachable symbolic (location, zone) states of `automaton`.
pub fn reachable_zone_states(
    automaton: &TimedAutomaton,
) -> Result<Vec<ZoneState>, VerificationError> {
    let k = collect_max_constants(automaton, None);
    let nodes = explore(automaton, &k)?;
    Ok(nodes
        .into_iter()
        .map(|n| ZoneState {
            location: n.location,
            zone: n.zone,
        })
        .collect())
}

/// Zone-graph based reachability of any accepting location.
pub fn accepting_reachable_zone(automaton: &TimedAutomaton) -> Result<bool, VerificationError> {
    let k = collect_max_constants(automaton, None);
    let nodes = explore(automaton, &k)?;
    Ok(nodes.iter().any(|n| {
        automaton
            .locations
            .get(&n.location)
            .is_some_and(|l| l.accepting)
    }))
}

/// What location(s) a deadline applies to.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DeadlineTarget {
    /// All accepting locations of the automaton.
    AcceptingLocations,
    /// A single named location.
    Location(String),
}

/// A deadline: a clock bound that should hold when a target location is reached.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Deadline {
    /// The clock constraint encoding the deadline (e.g. `x ≤ 10`).
    pub constraint: ClockConstraint,
    /// The location(s) the deadline applies to.
    pub target: DeadlineTarget,
}

impl Deadline {
    /// Creates a deadline from an explicit constraint and target.
    pub fn new(constraint: ClockConstraint, target: DeadlineTarget) -> Self {
        Self { constraint, target }
    }

    /// Convenience: clock `clock_name` must be `≤ bound` at any accepting location.
    pub fn before(clock_name: impl Into<String>, bound: u64) -> Self {
        Self {
            constraint: ClockConstraint::LessOrEqual(Clock::new(clock_name), bound),
            target: DeadlineTarget::AcceptingLocations,
        }
    }
}

/// The outcome of a deadline-satisfaction analysis.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DeadlineOutcome {
    /// Whether some path reaches the target while meeting the deadline.
    pub satisfiable: bool,
    /// Whether every path reaching the target meets the deadline.
    pub guaranteed: bool,
    /// A location path witnessing satisfiability (empty if unsatisfiable).
    pub witness: Vec<String>,
    /// A concrete deadline violation, present when not guaranteed.
    pub violation: Option<DeadlineViolation>,
}

fn is_target(automaton: &TimedAutomaton, location: &str, target: &DeadlineTarget) -> bool {
    match target {
        DeadlineTarget::AcceptingLocations => automaton
            .locations
            .get(location)
            .is_some_and(|l| l.accepting),
        DeadlineTarget::Location(name) => name == location,
    }
}

/// Negates a *simple* (single-comparison) clock constraint.
fn negate_simple(constraint: &ClockConstraint) -> Result<ClockConstraint, VerificationError> {
    match constraint {
        ClockConstraint::Less(c, v) => Ok(ClockConstraint::GreaterOrEqual(c.clone(), *v)),
        ClockConstraint::LessOrEqual(c, v) => Ok(ClockConstraint::Greater(c.clone(), *v)),
        ClockConstraint::Greater(c, v) => Ok(ClockConstraint::LessOrEqual(c.clone(), *v)),
        ClockConstraint::GreaterOrEqual(c, v) => Ok(ClockConstraint::Less(c.clone(), *v)),
        ClockConstraint::Equal(_, _) | ClockConstraint::And(_, _) => {
            Err(VerificationError::LogicalContradiction {
                message: "deadline guarantee requires a single non-equality clock comparison"
                    .to_string(),
            })
        }
    }
}

fn deadline_label(constraint: &ClockConstraint) -> String {
    match constraint {
        ClockConstraint::Less(c, v) => format!("{} < {}", c.name, v),
        ClockConstraint::LessOrEqual(c, v) => format!("{} <= {}", c.name, v),
        ClockConstraint::Greater(c, v) => format!("{} > {}", c.name, v),
        ClockConstraint::GreaterOrEqual(c, v) => format!("{} >= {}", c.name, v),
        ClockConstraint::Equal(c, v) => format!("{} == {}", c.name, v),
        ClockConstraint::And(_, _) => "composite deadline".to_string(),
    }
}

fn deadline_bound(constraint: &ClockConstraint) -> usize {
    match constraint {
        ClockConstraint::Less(_, v)
        | ClockConstraint::LessOrEqual(_, v)
        | ClockConstraint::Greater(_, v)
        | ClockConstraint::GreaterOrEqual(_, v)
        | ClockConstraint::Equal(_, v) => *v as usize,
        ClockConstraint::And(_, _) => 0,
    }
}

/// Whether the deadline is reachable: some path reaches a target location with a
/// clock valuation satisfying the deadline constraint.
pub fn verify_deadline_reachable(
    automaton: &TimedAutomaton,
    deadline: &Deadline,
) -> Result<bool, VerificationError> {
    let k = collect_max_constants(automaton, Some(&deadline.constraint));
    let nodes = explore(automaton, &k)?;
    for node in &nodes {
        if is_target(automaton, &node.location, &deadline.target) {
            let mut zone = node.zone.clone();
            zone.and_constraint(&deadline.constraint)?;
            if !zone.is_empty() {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// Full deadline-satisfaction analysis over the zone graph.
///
/// Computes whether the deadline can be met (`satisfiable`) and whether it is met
/// on every path reaching the target (`guaranteed`), producing a witness path and,
/// when the deadline can be missed, a concrete [`DeadlineViolation`].
pub fn check_deadline_satisfaction(
    automaton: &TimedAutomaton,
    deadline: &Deadline,
) -> Result<DeadlineOutcome, VerificationError> {
    let k = collect_max_constants(automaton, Some(&deadline.constraint));
    let nodes = explore(automaton, &k)?;

    let target_indices: Vec<usize> = nodes
        .iter()
        .enumerate()
        .filter(|(_, n)| is_target(automaton, &n.location, &deadline.target))
        .map(|(i, _)| i)
        .collect();

    let mut satisfiable = false;
    let mut witness = Vec::new();
    for &i in &target_indices {
        let mut zone = nodes[i].zone.clone();
        zone.and_constraint(&deadline.constraint)?;
        if !zone.is_empty() {
            satisfiable = true;
            witness = path_locations(&nodes, i);
            break;
        }
    }

    let negation = negate_simple(&deadline.constraint)?;
    let mut guaranteed = true;
    let mut violation = None;
    for &i in &target_indices {
        let mut zone = nodes[i].zone.clone();
        zone.and_constraint(&negation)?;
        if !zone.is_empty() {
            guaranteed = false;
            let path = path_locations(&nodes, i);
            violation = Some(DeadlineViolation {
                deadline_id: deadline_label(&deadline.constraint),
                actual_steps: path.len().saturating_sub(1),
                max_steps: deadline_bound(&deadline.constraint),
                description: format!(
                    "deadline '{}' can be violated when reaching location '{}'",
                    deadline_label(&deadline.constraint),
                    nodes[i].location
                ),
            });
            break;
        }
    }

    Ok(DeadlineOutcome {
        satisfiable,
        guaranteed,
        witness,
        violation,
    })
}
