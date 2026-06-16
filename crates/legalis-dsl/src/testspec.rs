//! Test-specification grammar for the Legalis DSL (v0.2.7).
//!
//! This module extends the inline `@test` runner in [`crate::contract`] with the
//! remaining *Test DSL* constructs, all introduced by a leading `@` directive:
//!
//! ```text
//! @mock adult  { age = 30, citizen = true }
//! @mock minor  { age = 12, citizen = true }
//!
//! @test "adult votes" FOR voting {
//!     USING adult
//!     EXPECT GRANT
//! }
//!
//! @property "everyone of age is eligible" FOR voting {
//!     FORALL age IN 18 TO 120
//!     GIVEN citizen = true
//!     EXPECT SATISFIED
//! }
//!
//! @coverage REQUIRE statutes >= 100%
//! @coverage REQUIRE outcomes >= 50% FOR voting
//!
//! @snapshot "voting baseline" FOR voting EXPECT "GRANT#abcdef0123456789"
//! ```
//!
//! Every construct round-trips through
//! [`crate::printer::format_test_spec_document`] and
//! [`crate::LegalDslParser::parse_test_spec_document`]. The aggregate
//! [`TestSpecDocument::run`] evaluates everything against parsed
//! [`legalis_core::Statute`]s:
//!
//! - **mock entities** are reusable attribute fixtures pulled into a `@test` or
//!   `@property` with `USING <id>` (an explicit `GIVEN` overrides a mock value);
//! - **property specifications** quantify an expectation over generated entity
//!   values (exhaustive when the domain product is small, otherwise
//!   deterministically sampled) and report a *shrunk* counterexample on failure;
//! - **coverage requirements** assert a floor (or exact value) on statute /
//!   branch-outcome coverage of the surrounding suite;
//! - **snapshot assertions** pin a statute's structural signature (effect kind +
//!   a stable FNV-1a digest of its canonical pretty-print).
//!
//! The engine is pure Rust with no external randomness or fuzzing dependency:
//! sampling uses an in-module SplitMix64 seeded from the property name, so runs
//! are fully reproducible.

use legalis_core::{AttributeBasedContext, Statute};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::contract::{
    TestBinding, TestCaseNode, TestCaseResult, TestExpectation, TestRunReport, TestValue,
    effect_type_keyword, evaluate_case_outcome, statute_satisfied,
};

/// Default ceiling on the number of generated cases per property when the
/// domain cross-product is larger than can be exhaustively enumerated.
pub const DEFAULT_PROPERTY_CASES: usize = 256;

/// Upper bound on how far the shrinker scans a single integer/value domain when
/// minimising a counterexample (keeps shrinking bounded for huge ranges).
const SHRINK_SCAN_CAP: u64 = 4096;

// ---------------------------------------------------------------------------
// Mock entities
// ---------------------------------------------------------------------------

/// A reusable, named bag of attribute bindings (`@mock <id> { k = v, ... }`).
///
/// Mocks are referenced from a `@test` or `@property` with `USING <id>`; their
/// bindings seed the evaluation context and are overridden by any explicit
/// `GIVEN` (or, in a property, by the quantified `FORALL` variable).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MockEntityNode {
    /// Stable mock identifier referenced by `USING`.
    pub id: String,
    /// Attribute bindings the mock contributes to the context.
    pub bindings: Vec<TestBinding>,
}

impl MockEntityNode {
    /// Creates an empty mock entity with the given identifier.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            bindings: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Property specifications
// ---------------------------------------------------------------------------

/// The generation domain of a quantified `FORALL` variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyDomain {
    /// An inclusive integer range `lo TO hi`.
    IntRange {
        /// Inclusive lower bound.
        lo: i64,
        /// Inclusive upper bound.
        hi: i64,
    },
    /// An explicit, non-empty list of literal values `( v1, v2, ... )`.
    Values(Vec<TestValue>),
}

impl PropertyDomain {
    /// Returns the number of distinct values in this domain.
    pub fn size(&self) -> u64 {
        match self {
            Self::IntRange { lo, hi } => {
                if hi >= lo {
                    // (hi - lo) cannot overflow i64 for sane bounds; widen first.
                    (*hi as i128 - *lo as i128 + 1).max(0) as u64
                } else {
                    0
                }
            }
            Self::Values(values) => values.len() as u64,
        }
    }

    /// Returns the value at logical index `idx` (`0` is the smallest), saturating
    /// rather than panicking on an out-of-range index.
    pub fn value_at(&self, idx: u64) -> TestValue {
        match self {
            Self::IntRange { lo, .. } => TestValue::Number(lo.saturating_add(idx as i64)),
            Self::Values(values) => {
                let bounded = (idx as usize).min(values.len().saturating_sub(1));
                values
                    .get(bounded)
                    .cloned()
                    .unwrap_or(TestValue::Boolean(false))
            }
        }
    }
}

/// A quantified variable of a property: `FORALL <name> IN <domain>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyVar {
    /// Attribute name bound across the generated domain.
    pub name: String,
    /// The values the variable ranges over.
    pub domain: PropertyDomain,
}

/// A property-based test specification (`@property "..." FOR <statute> { ... }`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertySpecNode {
    /// Human-readable property name (also seeds the deterministic sampler).
    pub name: String,
    /// Identifier of the statute the property is checked against.
    pub target_statute: String,
    /// Quantified variables (the generation space).
    pub vars: Vec<PropertyVar>,
    /// Fixed `GIVEN` bindings applied to every generated case.
    pub fixed_bindings: Vec<TestBinding>,
    /// Mock fixtures pulled in with `USING`.
    pub uses: Vec<String>,
    /// The expectation that must hold for every generated case.
    pub expectation: TestExpectation,
    /// Optional override of [`DEFAULT_PROPERTY_CASES`] for the sampling budget.
    pub max_cases: Option<usize>,
}

impl PropertySpecNode {
    /// Returns `true` when the domain cross-product fits within the case budget,
    /// so the property is checked exhaustively rather than sampled.
    pub fn is_exhaustive(&self) -> bool {
        if self.vars.is_empty() {
            return true;
        }
        let sizes: Vec<u64> = self.vars.iter().map(|v| v.domain.size()).collect();
        if sizes.contains(&0) {
            return true;
        }
        let total: u128 = sizes.iter().map(|&s| s as u128).product();
        total <= self.budget() as u128
    }

    /// The effective per-property sampling budget.
    pub fn budget(&self) -> usize {
        self.max_cases.unwrap_or(DEFAULT_PROPERTY_CASES)
    }
}

// ---------------------------------------------------------------------------
// Coverage requirements
// ---------------------------------------------------------------------------

/// A coverage metric a [`CoverageRequirementNode`] constrains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoverageMetric {
    /// Fraction of statutes targeted by at least one `@test`/`@property`.
    Statutes,
    /// Fraction of statutes whose suite exercises *both* a satisfied and an
    /// unsatisfied evaluation (true/false branch coverage of the predicate).
    Outcomes,
}

impl CoverageMetric {
    /// Maps a source keyword to a metric, if recognised.
    pub fn from_keyword(word: &str) -> Option<Self> {
        match word.to_ascii_lowercase().as_str() {
            "statutes" | "statute" => Some(Self::Statutes),
            "outcomes" | "outcome" | "branches" | "branch" => Some(Self::Outcomes),
            _ => None,
        }
    }

    /// Returns the canonical source keyword for this metric.
    pub fn keyword(&self) -> &'static str {
        match self {
            Self::Statutes => "statutes",
            Self::Outcomes => "outcomes",
        }
    }
}

/// The comparison applied between actual coverage and the required threshold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoverageComparator {
    /// `>=` — actual must be at least the threshold.
    AtLeast,
    /// `>` — actual must strictly exceed the threshold.
    GreaterThan,
    /// `==` — actual must equal the threshold.
    Exactly,
}

impl CoverageComparator {
    /// Maps an operator lexeme to a comparator, if recognised.
    pub fn from_operator(op: &str) -> Option<Self> {
        match op {
            ">=" => Some(Self::AtLeast),
            ">" => Some(Self::GreaterThan),
            "=" | "==" => Some(Self::Exactly),
            _ => None,
        }
    }

    /// Returns the canonical operator lexeme for this comparator.
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::AtLeast => ">=",
            Self::GreaterThan => ">",
            Self::Exactly => "==",
        }
    }

    /// Returns `true` when `actual` satisfies this comparator against `threshold`
    /// (with a small epsilon to absorb floating-point rounding).
    pub fn satisfied_by(&self, actual: f64, threshold: f64) -> bool {
        const EPS: f64 = 1e-9;
        match self {
            Self::AtLeast => actual + EPS >= threshold,
            Self::GreaterThan => actual > threshold + EPS,
            Self::Exactly => (actual - threshold).abs() < 1e-6,
        }
    }
}

/// A coverage requirement (`@coverage REQUIRE <metric> <op> <pct>% [FOR <id>]`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoverageRequirementNode {
    /// The metric being constrained.
    pub metric: CoverageMetric,
    /// How `threshold` is compared against the measured coverage.
    pub comparator: CoverageComparator,
    /// The required percentage (0.0 – 100.0).
    pub threshold: f64,
    /// Optional statute the requirement is scoped to (otherwise suite-wide).
    pub target: Option<String>,
}

// ---------------------------------------------------------------------------
// Snapshot assertions
// ---------------------------------------------------------------------------

/// Whether a snapshot assertion verifies against a pinned signature or records
/// the current one.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SnapshotMode {
    /// Verify the statute's signature equals this pinned value.
    Match(String),
    /// Record (bless) the current signature; always reported as passing.
    Record,
}

/// A snapshot assertion (`@snapshot "<name>" FOR <statute> EXPECT "<sig>"`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotAssertionNode {
    /// Human-readable snapshot name.
    pub name: String,
    /// Identifier of the statute whose signature is asserted.
    pub target_statute: String,
    /// Whether to match a pinned signature or record the current one.
    pub mode: SnapshotMode,
}

// ---------------------------------------------------------------------------
// Aggregate document
// ---------------------------------------------------------------------------

/// A parsed test-specification document: the full set of `@`-directives that
/// make up a DSL test suite.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TestSpecDocument {
    /// Mock entity fixtures, in declaration order.
    pub mocks: Vec<MockEntityNode>,
    /// Inline `@test` cases, in declaration order.
    pub tests: Vec<TestCaseNode>,
    /// Property specifications, in declaration order.
    pub properties: Vec<PropertySpecNode>,
    /// Coverage requirements, in declaration order.
    pub coverage: Vec<CoverageRequirementNode>,
    /// Snapshot assertions, in declaration order.
    pub snapshots: Vec<SnapshotAssertionNode>,
}

impl TestSpecDocument {
    /// Returns `true` when the document declares no directives at all.
    pub fn is_empty(&self) -> bool {
        self.mocks.is_empty()
            && self.tests.is_empty()
            && self.properties.is_empty()
            && self.coverage.is_empty()
            && self.snapshots.is_empty()
    }

    /// Returns the mock entity with the given id, if present.
    pub fn mock(&self, id: &str) -> Option<&MockEntityNode> {
        self.mocks.iter().find(|m| m.id == id)
    }

    /// Runs every directive against `statutes`, producing an aggregate report.
    pub fn run(&self, statutes: &[Statute]) -> TestSpecReport {
        let tests = run_test_cases_with_mocks(statutes, &self.mocks, &self.tests);
        let properties = run_property_cases(statutes, &self.mocks, &self.properties);
        let stats = compute_coverage(statutes, &self.tests, &self.properties, &self.mocks);
        let coverage = check_coverage(&stats, &self.coverage);
        let snapshots = run_snapshots(statutes, &self.snapshots);
        TestSpecReport {
            tests,
            properties,
            coverage,
            snapshots,
        }
    }
}

// ---------------------------------------------------------------------------
// Run reports
// ---------------------------------------------------------------------------

/// The aggregate outcome of running a [`TestSpecDocument`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestSpecReport {
    /// Result of the inline `@test` cases.
    pub tests: TestRunReport,
    /// Result of the `@property` specifications.
    pub properties: PropertyRunReport,
    /// Result of the `@coverage` requirements.
    pub coverage: CoverageReport,
    /// Result of the `@snapshot` assertions.
    pub snapshots: SnapshotRunReport,
}

impl TestSpecReport {
    /// Returns `true` when every directive passed.
    pub fn all_passed(&self) -> bool {
        self.tests.all_passed()
            && self.properties.all_passed()
            && self.coverage.all_passed()
            && self.snapshots.all_passed()
    }

    /// Total number of failing directives across every category.
    pub fn total_failures(&self) -> usize {
        self.tests.failed()
            + self.properties.failed()
            + self.coverage.failed()
            + self.snapshots.failed()
    }
}

/// The outcome of checking a single [`PropertySpecNode`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyResult {
    /// Name of the property that was checked.
    pub name: String,
    /// Targeted statute id.
    pub target_statute: String,
    /// Number of generated cases evaluated before stopping.
    pub checked_cases: usize,
    /// Whether the domain was checked exhaustively (vs. sampled).
    pub exhaustive: bool,
    /// Whether the property held for every checked case.
    pub passed: bool,
    /// The (shrunk) counterexample assignment when the property failed.
    pub counterexample: Option<Vec<(String, TestValue)>>,
    /// Diagnostic detail.
    pub message: String,
}

/// The aggregate outcome of running a set of property specifications.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PropertyRunReport {
    /// Per-property results, in input order.
    pub results: Vec<PropertyResult>,
}

impl PropertyRunReport {
    /// Number of properties that passed.
    pub fn passed(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    /// Number of properties that failed.
    pub fn failed(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    /// Total number of properties run.
    pub fn total(&self) -> usize {
        self.results.len()
    }

    /// Returns `true` when every property passed.
    pub fn all_passed(&self) -> bool {
        self.results.iter().all(|r| r.passed)
    }
}

/// The outcome of checking a single coverage requirement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoverageResult {
    /// The requirement that was checked.
    pub requirement: CoverageRequirementNode,
    /// The measured coverage percentage.
    pub actual: f64,
    /// Whether the requirement was met.
    pub passed: bool,
    /// Diagnostic detail.
    pub message: String,
}

/// The aggregate outcome of checking coverage requirements.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CoverageReport {
    /// Per-requirement results, in input order.
    pub results: Vec<CoverageResult>,
}

impl CoverageReport {
    /// Number of requirements satisfied.
    pub fn passed(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    /// Number of requirements violated.
    pub fn failed(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    /// Total number of requirements checked.
    pub fn total(&self) -> usize {
        self.results.len()
    }

    /// Returns `true` when every requirement was met.
    pub fn all_passed(&self) -> bool {
        self.results.iter().all(|r| r.passed)
    }
}

/// The outcome of checking a single snapshot assertion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotResult {
    /// Name of the snapshot assertion.
    pub name: String,
    /// Targeted statute id.
    pub target_statute: String,
    /// The pinned signature (absent in record mode).
    pub expected: Option<String>,
    /// The signature computed from the current statute.
    pub actual: String,
    /// Whether the assertion passed.
    pub passed: bool,
    /// Diagnostic detail.
    pub message: String,
}

/// The aggregate outcome of running snapshot assertions.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SnapshotRunReport {
    /// Per-snapshot results, in input order.
    pub results: Vec<SnapshotResult>,
}

impl SnapshotRunReport {
    /// Number of snapshots that matched.
    pub fn passed(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    /// Number of snapshots that mismatched.
    pub fn failed(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    /// Total number of snapshots checked.
    pub fn total(&self) -> usize {
        self.results.len()
    }

    /// Returns `true` when every snapshot matched.
    pub fn all_passed(&self) -> bool {
        self.results.iter().all(|r| r.passed)
    }
}

// ---------------------------------------------------------------------------
// Deterministic helpers (no external randomness)
// ---------------------------------------------------------------------------

/// 64-bit FNV-1a hash of `bytes` — a stable, platform-independent digest used
/// for snapshot signatures and as the property sampler seed.
pub fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for &byte in bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// One step of the SplitMix64 generator (used for reproducible sampling).
fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

// ---------------------------------------------------------------------------
// Context construction & shared evaluation
// ---------------------------------------------------------------------------

/// Builds the evaluation context for a case, applying values in increasing
/// precedence: mock fixtures, then fixed `GIVEN` bindings, then generated values.
fn build_context(
    mocks: &[MockEntityNode],
    uses: &[String],
    fixed: &[TestBinding],
    generated: &[(String, TestValue)],
) -> Result<AttributeBasedContext, String> {
    let mut attributes: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    for id in uses {
        match mocks.iter().find(|m| &m.id == id) {
            Some(mock) => {
                for binding in &mock.bindings {
                    attributes.insert(binding.key.clone(), binding.value.as_attribute_string());
                }
            }
            None => return Err(format!("unknown mock entity '{id}'")),
        }
    }
    for binding in fixed {
        attributes.insert(binding.key.clone(), binding.value.as_attribute_string());
    }
    for (key, value) in generated {
        attributes.insert(key.clone(), value.as_attribute_string());
    }
    Ok(AttributeBasedContext::new(attributes))
}

/// Returns `true` when `satisfied`/effect agrees with `expectation`.
fn expectation_holds(statute: &Statute, satisfied: bool, expectation: &TestExpectation) -> bool {
    match expectation {
        TestExpectation::Satisfied => satisfied,
        TestExpectation::Unsatisfied => !satisfied,
        TestExpectation::Effect(effect) => satisfied && effect.matches(&statute.effect.effect_type),
    }
}

// ---------------------------------------------------------------------------
// Mock-aware @test runner
// ---------------------------------------------------------------------------

/// Runs each `@test` case against the matching statute, resolving any `USING`
/// mock fixtures from `mocks`. With an empty `mocks` slice this behaves exactly
/// like [`crate::contract::run_test_cases`].
pub fn run_test_cases_with_mocks(
    statutes: &[Statute],
    mocks: &[MockEntityNode],
    cases: &[TestCaseNode],
) -> TestRunReport {
    let mut results = Vec::with_capacity(cases.len());
    for case in cases {
        results.push(run_single_case(statutes, mocks, case));
    }
    TestRunReport { results }
}

/// Evaluates one `@test` case.
fn run_single_case(
    statutes: &[Statute],
    mocks: &[MockEntityNode],
    case: &TestCaseNode,
) -> TestCaseResult {
    let make = |passed: bool, message: String| TestCaseResult {
        name: case.name.clone(),
        target_statute: case.target_statute.clone(),
        passed,
        message,
    };

    let Some(statute) = statutes.iter().find(|s| s.id == case.target_statute) else {
        return make(
            false,
            format!("no statute with id '{}'", case.target_statute),
        );
    };
    let context = match build_context(mocks, &case.uses, &case.bindings, &[]) {
        Ok(context) => context,
        Err(message) => return make(false, message),
    };
    let satisfied = match statute_satisfied(statute, &context) {
        Ok(value) => value,
        Err(message) => return make(false, message),
    };
    let (passed, message) = evaluate_case_outcome(statute, satisfied, &case.expectation);
    make(passed, message)
}

// ---------------------------------------------------------------------------
// Property runner
// ---------------------------------------------------------------------------

/// Enumerates (exhaustively or by deterministic sampling) the assignments a
/// property is checked against.
pub fn enumerate_assignments(prop: &PropertySpecNode) -> Vec<Vec<(String, TestValue)>> {
    if prop.vars.is_empty() {
        return vec![Vec::new()];
    }
    let sizes: Vec<u64> = prop.vars.iter().map(|v| v.domain.size()).collect();
    if sizes.contains(&0) {
        return Vec::new();
    }
    let total: u128 = sizes.iter().map(|&s| s as u128).product();
    let budget = prop.budget() as u128;
    let mut out = Vec::new();

    if total <= budget {
        // Exhaustive, mixed-radix enumeration over the cross-product.
        for n in 0..total {
            let mut rem = n;
            let mut assignment = Vec::with_capacity(prop.vars.len());
            for (var, &size) in prop.vars.iter().zip(&sizes) {
                let size = size as u128;
                let idx = (rem % size) as u64;
                rem /= size;
                assignment.push((var.name.clone(), var.domain.value_at(idx)));
            }
            out.push(assignment);
        }
    } else {
        // Deterministic sampling seeded by the property name.
        let mut state = fnv1a_64(prop.name.as_bytes()) ^ 0xD1B5_4A32_D192_ED03;
        for _ in 0..budget {
            let mut assignment = Vec::with_capacity(prop.vars.len());
            for (var, &size) in prop.vars.iter().zip(&sizes) {
                let idx = splitmix64(&mut state) % size;
                assignment.push((var.name.clone(), var.domain.value_at(idx)));
            }
            out.push(assignment);
        }
    }
    out
}

/// Runs each property specification against the matching statute.
pub fn run_property_cases(
    statutes: &[Statute],
    mocks: &[MockEntityNode],
    properties: &[PropertySpecNode],
) -> PropertyRunReport {
    let mut results = Vec::with_capacity(properties.len());
    for prop in properties {
        results.push(run_single_property(statutes, mocks, prop));
    }
    PropertyRunReport { results }
}

/// Evaluates one property specification, returning a shrunk counterexample on
/// failure.
fn run_single_property(
    statutes: &[Statute],
    mocks: &[MockEntityNode],
    prop: &PropertySpecNode,
) -> PropertyResult {
    let make = |checked: usize,
                passed: bool,
                counterexample: Option<Vec<(String, TestValue)>>,
                message: String| PropertyResult {
        name: prop.name.clone(),
        target_statute: prop.target_statute.clone(),
        checked_cases: checked,
        exhaustive: prop.is_exhaustive(),
        passed,
        counterexample,
        message,
    };

    let Some(statute) = statutes.iter().find(|s| s.id == prop.target_statute) else {
        return make(
            0,
            false,
            None,
            format!("no statute with id '{}'", prop.target_statute),
        );
    };
    for id in &prop.uses {
        if !mocks.iter().any(|m| &m.id == id) {
            return make(0, false, None, format!("unknown mock entity '{id}'"));
        }
    }

    let mut checked = 0usize;
    for assignment in enumerate_assignments(prop) {
        match property_violated(statute, mocks, prop, &assignment) {
            Ok(true) => {
                let shrunk = shrink_counterexample(statute, mocks, prop, &assignment);
                let message = format!("property failed for {}", render_assignment(&shrunk));
                return make(checked + 1, false, Some(shrunk), message);
            }
            Ok(false) => checked += 1,
            Err(message) => return make(checked, false, None, message),
        }
    }

    let message = if prop.is_exhaustive() {
        format!("property holds over all {checked} case(s)")
    } else {
        format!("property holds over {checked} sampled case(s)")
    };
    make(checked, true, None, message)
}

/// Returns `Ok(true)` when an assignment violates the property's expectation.
fn property_violated(
    statute: &Statute,
    mocks: &[MockEntityNode],
    prop: &PropertySpecNode,
    assignment: &[(String, TestValue)],
) -> Result<bool, String> {
    let context = build_context(mocks, &prop.uses, &prop.fixed_bindings, assignment)?;
    let satisfied = statute_satisfied(statute, &context)?;
    Ok(!expectation_holds(statute, satisfied, &prop.expectation))
}

/// Minimises a failing assignment by reducing each variable toward the smallest
/// value (range low bound / first listed value) that still triggers a failure.
fn shrink_counterexample(
    statute: &Statute,
    mocks: &[MockEntityNode],
    prop: &PropertySpecNode,
    failing: &[(String, TestValue)],
) -> Vec<(String, TestValue)> {
    let mut current = failing.to_vec();
    for (i, var) in prop.vars.iter().enumerate() {
        if i >= current.len() {
            break;
        }
        let scan = var.domain.size().min(SHRINK_SCAN_CAP);
        for idx in 0..scan {
            let candidate = var.domain.value_at(idx);
            if candidate == current[i].1 {
                // Reached the current value; anything beyond is not "smaller".
                break;
            }
            let mut trial = current.clone();
            trial[i] = (var.name.clone(), candidate.clone());
            if matches!(property_violated(statute, mocks, prop, &trial), Ok(true)) {
                current[i] = (var.name.clone(), candidate);
                break;
            }
        }
    }
    current
}

/// Renders an assignment as `k = v, ...` for diagnostics.
fn render_assignment(assignment: &[(String, TestValue)]) -> String {
    if assignment.is_empty() {
        return "the empty assignment".to_string();
    }
    assignment
        .iter()
        .map(|(key, value)| format!("{key} = {}", render_value(value)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Renders a [`TestValue`] for a diagnostic message.
fn render_value(value: &TestValue) -> String {
    match value {
        TestValue::Number(n) => n.to_string(),
        TestValue::String(s) => format!("\"{s}\""),
        TestValue::Boolean(b) => b.to_string(),
    }
}

// ---------------------------------------------------------------------------
// Coverage
// ---------------------------------------------------------------------------

/// Measured coverage of a statute suite by an inline test specification.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CoverageStats {
    /// Total number of statutes in scope.
    pub total_statutes: usize,
    /// Statute ids targeted by at least one `@test`/`@property`.
    pub targeted: BTreeSet<String>,
    /// Per-statute `(saw_satisfied, saw_unsatisfied)` evaluation outcomes.
    pub outcomes: BTreeMap<String, (bool, bool)>,
}

impl CoverageStats {
    /// Percentage of statutes targeted by the suite.
    pub fn statute_percent(&self) -> f64 {
        percent(self.targeted.len(), self.total_statutes)
    }

    /// Percentage of statutes with both true- and false-branch outcomes.
    pub fn outcome_percent(&self) -> f64 {
        let covered = self
            .outcomes
            .values()
            .filter(|(satisfied, unsatisfied)| *satisfied && *unsatisfied)
            .count();
        percent(covered, self.total_statutes)
    }

    /// Computes the coverage percentage for a metric, optionally scoped to a
    /// single statute id.
    pub fn percent_for(&self, metric: &CoverageMetric, target: Option<&str>) -> f64 {
        match (metric, target) {
            (CoverageMetric::Statutes, None) => self.statute_percent(),
            (CoverageMetric::Statutes, Some(id)) => {
                if self.targeted.contains(id) {
                    100.0
                } else {
                    0.0
                }
            }
            (CoverageMetric::Outcomes, None) => self.outcome_percent(),
            (CoverageMetric::Outcomes, Some(id)) => match self.outcomes.get(id) {
                Some((true, true)) => 100.0,
                Some((true, false)) | Some((false, true)) => 50.0,
                _ => 0.0,
            },
        }
    }
}

/// Divides `num` by `den` as a percentage; an empty universe is vacuously 100%.
fn percent(num: usize, den: usize) -> f64 {
    if den == 0 {
        100.0
    } else {
        (num as f64) * 100.0 / (den as f64)
    }
}

/// Measures the coverage of `statutes` achieved by `tests` and `properties`.
pub fn compute_coverage(
    statutes: &[Statute],
    tests: &[TestCaseNode],
    properties: &[PropertySpecNode],
    mocks: &[MockEntityNode],
) -> CoverageStats {
    let mut stats = CoverageStats {
        total_statutes: statutes.len(),
        targeted: BTreeSet::new(),
        outcomes: BTreeMap::new(),
    };

    for case in tests {
        stats.targeted.insert(case.target_statute.clone());
        if let Some(statute) = statutes.iter().find(|s| s.id == case.target_statute)
            && let Ok(context) = build_context(mocks, &case.uses, &case.bindings, &[])
            && let Ok(satisfied) = statute_satisfied(statute, &context)
        {
            record_outcome(&mut stats, &case.target_statute, satisfied);
        }
    }

    for prop in properties {
        stats.targeted.insert(prop.target_statute.clone());
        let Some(statute) = statutes.iter().find(|s| s.id == prop.target_statute) else {
            continue;
        };
        for assignment in enumerate_assignments(prop) {
            if let Ok(context) = build_context(mocks, &prop.uses, &prop.fixed_bindings, &assignment)
                && let Ok(satisfied) = statute_satisfied(statute, &context)
            {
                record_outcome(&mut stats, &prop.target_statute, satisfied);
            }
        }
    }

    stats
}

/// Folds a single evaluation outcome into the per-statute branch record.
fn record_outcome(stats: &mut CoverageStats, statute_id: &str, satisfied: bool) {
    let entry = stats
        .outcomes
        .entry(statute_id.to_string())
        .or_insert((false, false));
    entry.0 |= satisfied;
    entry.1 |= !satisfied;
}

/// Checks coverage `requirements` against measured `stats`.
pub fn check_coverage(
    stats: &CoverageStats,
    requirements: &[CoverageRequirementNode],
) -> CoverageReport {
    let mut results = Vec::with_capacity(requirements.len());
    for req in requirements {
        let actual = stats.percent_for(&req.metric, req.target.as_deref());
        let passed = req.comparator.satisfied_by(actual, req.threshold);
        let scope = match &req.target {
            Some(id) => format!(" for '{id}'"),
            None => String::new(),
        };
        let message = format!(
            "{} coverage{scope} is {actual:.1}% (required {} {:.1}%)",
            req.metric.keyword(),
            req.comparator.symbol(),
            req.threshold,
        );
        results.push(CoverageResult {
            requirement: req.clone(),
            actual,
            passed,
            message,
        });
    }
    CoverageReport { results }
}

// ---------------------------------------------------------------------------
// Snapshots
// ---------------------------------------------------------------------------

/// Computes a statute's structural snapshot signature: its effect keyword plus a
/// stable FNV-1a digest of its canonical pretty-print. Any structural change to
/// the statute flips the digest.
pub fn statute_signature(statute: &Statute) -> String {
    let canonical = crate::printer::format_statute(statute);
    format!(
        "{}#{:016x}",
        effect_type_keyword(&statute.effect.effect_type),
        fnv1a_64(canonical.as_bytes())
    )
}

/// Runs each snapshot assertion against the matching statute.
pub fn run_snapshots(
    statutes: &[Statute],
    snapshots: &[SnapshotAssertionNode],
) -> SnapshotRunReport {
    let mut results = Vec::with_capacity(snapshots.len());
    for snap in snapshots {
        let expected = match &snap.mode {
            SnapshotMode::Match(sig) => Some(sig.clone()),
            SnapshotMode::Record => None,
        };
        let Some(statute) = statutes.iter().find(|s| s.id == snap.target_statute) else {
            results.push(SnapshotResult {
                name: snap.name.clone(),
                target_statute: snap.target_statute.clone(),
                expected,
                actual: String::new(),
                passed: false,
                message: format!("no statute with id '{}'", snap.target_statute),
            });
            continue;
        };
        let actual = statute_signature(statute);
        let (passed, message) = match &snap.mode {
            SnapshotMode::Match(pinned) => {
                if pinned == &actual {
                    (true, "snapshot matches".to_string())
                } else {
                    (
                        false,
                        format!("snapshot mismatch: expected '{pinned}', got '{actual}'"),
                    )
                }
            }
            SnapshotMode::Record => (true, format!("recorded snapshot '{actual}'")),
        };
        results.push(SnapshotResult {
            name: snap.name.clone(),
            target_statute: snap.target_statute.clone(),
            expected,
            actual,
            passed,
            message,
        });
    }
    SnapshotRunReport { results }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::ExpectedEffect;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn voting_statute() -> Statute {
        Statute::new(
            "voting",
            "Voting Rights",
            Effect::new(EffectType::Grant, "Right to vote"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    fn int_var(name: &str, lo: i64, hi: i64) -> PropertyVar {
        PropertyVar {
            name: name.to_string(),
            domain: PropertyDomain::IntRange { lo, hi },
        }
    }

    #[test]
    fn test_domain_size_and_value_at() {
        let range = PropertyDomain::IntRange { lo: 5, hi: 9 };
        assert_eq!(range.size(), 5);
        assert_eq!(range.value_at(0), TestValue::Number(5));
        assert_eq!(range.value_at(4), TestValue::Number(9));

        let values = PropertyDomain::Values(vec![TestValue::Number(1), TestValue::Boolean(true)]);
        assert_eq!(values.size(), 2);
        assert_eq!(values.value_at(1), TestValue::Boolean(true));
        // Out-of-range index saturates instead of panicking.
        assert_eq!(values.value_at(99), TestValue::Boolean(true));
    }

    #[test]
    fn test_empty_range_has_no_values() {
        let empty = PropertyDomain::IntRange { lo: 10, hi: 1 };
        assert_eq!(empty.size(), 0);
    }

    #[test]
    fn test_fnv_is_deterministic_and_distinct() {
        assert_eq!(fnv1a_64(b"voting"), fnv1a_64(b"voting"));
        assert_ne!(fnv1a_64(b"voting"), fnv1a_64(b"Voting"));
    }

    #[test]
    fn test_sampling_is_reproducible() {
        let prop = PropertySpecNode {
            name: "big".to_string(),
            target_statute: "voting".to_string(),
            vars: vec![int_var("age", 0, 1_000_000)],
            fixed_bindings: vec![],
            uses: vec![],
            expectation: TestExpectation::Satisfied,
            max_cases: Some(32),
        };
        assert!(!prop.is_exhaustive());
        let first = enumerate_assignments(&prop);
        let second = enumerate_assignments(&prop);
        assert_eq!(first.len(), 32);
        assert_eq!(first, second, "sampling must be deterministic");
    }

    #[test]
    fn test_exhaustive_enumeration_cross_product() {
        let prop = PropertySpecNode {
            name: "grid".to_string(),
            target_statute: "voting".to_string(),
            vars: vec![int_var("a", 0, 2), int_var("b", 0, 1)],
            fixed_bindings: vec![],
            uses: vec![],
            expectation: TestExpectation::Satisfied,
            max_cases: None,
        };
        assert!(prop.is_exhaustive());
        // 3 * 2 = 6 distinct assignments.
        assert_eq!(enumerate_assignments(&prop).len(), 6);
    }

    #[test]
    fn test_property_holds_exhaustively() {
        let prop = PropertySpecNode {
            name: "adults eligible".to_string(),
            target_statute: "voting".to_string(),
            vars: vec![int_var("age", 18, 40)],
            fixed_bindings: vec![],
            uses: vec![],
            expectation: TestExpectation::Satisfied,
            max_cases: None,
        };
        let report = run_property_cases(&[voting_statute()], &[], &[prop]);
        assert!(report.all_passed(), "{:?}", report.results);
        assert!(report.results[0].exhaustive);
        assert_eq!(report.results[0].checked_cases, 23);
    }

    #[test]
    fn test_property_fails_with_shrunk_counterexample() {
        let prop = PropertySpecNode {
            name: "all ages eligible".to_string(),
            target_statute: "voting".to_string(),
            vars: vec![int_var("age", 0, 40)],
            fixed_bindings: vec![],
            uses: vec![],
            expectation: TestExpectation::Satisfied,
            max_cases: None,
        };
        let report = run_property_cases(&[voting_statute()], &[], &[prop]);
        assert!(!report.all_passed());
        let counter = report.results[0]
            .counterexample
            .as_ref()
            .expect("counterexample present");
        // The minimal failing age is 0 (smallest value below the 18 threshold).
        assert_eq!(counter, &vec![("age".to_string(), TestValue::Number(0))]);
    }

    #[test]
    fn test_build_context_precedence() {
        let mocks = vec![MockEntityNode {
            id: "adult".to_string(),
            bindings: vec![TestBinding {
                key: "age".to_string(),
                value: TestValue::Number(30),
            }],
        }];
        // Generated value overrides the mock.
        let context = build_context(
            &mocks,
            &["adult".to_string()],
            &[],
            &[("age".to_string(), TestValue::Number(15))],
        )
        .expect("context builds");
        let statute = voting_statute();
        let satisfied = statute_satisfied(&statute, &context).expect("evaluates");
        assert!(!satisfied, "generated age 15 should override mock age 30");
    }

    #[test]
    fn test_unknown_mock_is_reported() {
        let err =
            build_context(&[], &["ghost".to_string()], &[], &[]).expect_err("missing mock errors");
        assert!(err.contains("unknown mock entity 'ghost'"));
    }

    #[test]
    fn test_coverage_statutes_and_outcomes() {
        let statute = voting_statute();
        let tests = vec![
            TestCaseNode {
                name: "adult".to_string(),
                target_statute: "voting".to_string(),
                uses: vec![],
                bindings: vec![TestBinding {
                    key: "age".to_string(),
                    value: TestValue::Number(40),
                }],
                expectation: TestExpectation::Satisfied,
            },
            TestCaseNode {
                name: "minor".to_string(),
                target_statute: "voting".to_string(),
                uses: vec![],
                bindings: vec![TestBinding {
                    key: "age".to_string(),
                    value: TestValue::Number(10),
                }],
                expectation: TestExpectation::Unsatisfied,
            },
        ];
        let stats = compute_coverage(&[statute], &tests, &[], &[]);
        assert_eq!(stats.statute_percent(), 100.0);
        // Both branches exercised → full outcome coverage.
        assert_eq!(stats.outcome_percent(), 100.0);

        let reqs = vec![
            CoverageRequirementNode {
                metric: CoverageMetric::Statutes,
                comparator: CoverageComparator::AtLeast,
                threshold: 100.0,
                target: None,
            },
            CoverageRequirementNode {
                metric: CoverageMetric::Outcomes,
                comparator: CoverageComparator::AtLeast,
                threshold: 50.0,
                target: None,
            },
        ];
        let report = check_coverage(&stats, &reqs);
        assert!(report.all_passed(), "{:?}", report.results);
    }

    #[test]
    fn test_coverage_outcome_requirement_can_fail() {
        let statute = voting_statute();
        // Only a satisfied case → false branch never exercised.
        let tests = vec![TestCaseNode {
            name: "adult".to_string(),
            target_statute: "voting".to_string(),
            uses: vec![],
            bindings: vec![TestBinding {
                key: "age".to_string(),
                value: TestValue::Number(40),
            }],
            expectation: TestExpectation::Satisfied,
        }];
        let stats = compute_coverage(&[statute], &tests, &[], &[]);
        assert_eq!(stats.outcome_percent(), 0.0);
        let reqs = vec![CoverageRequirementNode {
            metric: CoverageMetric::Outcomes,
            comparator: CoverageComparator::AtLeast,
            threshold: 100.0,
            target: None,
        }];
        let report = check_coverage(&stats, &reqs);
        assert_eq!(report.failed(), 1);
    }

    #[test]
    fn test_comparator_semantics() {
        assert!(CoverageComparator::AtLeast.satisfied_by(80.0, 80.0));
        assert!(!CoverageComparator::GreaterThan.satisfied_by(80.0, 80.0));
        assert!(CoverageComparator::Exactly.satisfied_by(50.0, 50.0));
        assert_eq!(
            CoverageComparator::from_operator(">="),
            Some(CoverageComparator::AtLeast)
        );
        assert_eq!(
            CoverageComparator::from_operator("=="),
            Some(CoverageComparator::Exactly)
        );
    }

    #[test]
    fn test_snapshot_signature_stable_and_matches() {
        let statute = voting_statute();
        let sig = statute_signature(&statute);
        assert_eq!(sig, statute_signature(&statute), "signature must be stable");
        assert!(sig.starts_with("GRANT#"));

        let snap = SnapshotAssertionNode {
            name: "baseline".to_string(),
            target_statute: "voting".to_string(),
            mode: SnapshotMode::Match(sig.clone()),
        };
        let report = run_snapshots(&[statute], &[snap]);
        assert!(report.all_passed());
    }

    #[test]
    fn test_snapshot_mismatch_and_record() {
        let statute = voting_statute();
        let mismatch = SnapshotAssertionNode {
            name: "stale".to_string(),
            target_statute: "voting".to_string(),
            mode: SnapshotMode::Match("GRANT#0000000000000000".to_string()),
        };
        let record = SnapshotAssertionNode {
            name: "bless".to_string(),
            target_statute: "voting".to_string(),
            mode: SnapshotMode::Record,
        };
        let report = run_snapshots(&[statute], &[mismatch, record]);
        assert_eq!(report.failed(), 1);
        assert!(report.results[0].message.contains("snapshot mismatch"));
        assert!(report.results[1].passed, "record mode always passes");
    }

    #[test]
    fn test_run_test_cases_with_mock_override() {
        let mocks = vec![MockEntityNode {
            id: "adult".to_string(),
            bindings: vec![TestBinding {
                key: "age".to_string(),
                value: TestValue::Number(30),
            }],
        }];
        let case = TestCaseNode {
            name: "uses mock".to_string(),
            target_statute: "voting".to_string(),
            uses: vec!["adult".to_string()],
            bindings: vec![],
            expectation: TestExpectation::Effect(ExpectedEffect::Grant),
        };
        let report = run_test_cases_with_mocks(&[voting_statute()], &mocks, &[case]);
        assert!(report.all_passed(), "{:?}", report.results);
    }

    #[test]
    fn test_spec_document_run_aggregates() {
        let mut doc = TestSpecDocument::default();
        doc.properties.push(PropertySpecNode {
            name: "adults".to_string(),
            target_statute: "voting".to_string(),
            vars: vec![int_var("age", 18, 25)],
            fixed_bindings: vec![],
            uses: vec![],
            expectation: TestExpectation::Satisfied,
            max_cases: None,
        });
        doc.snapshots.push(SnapshotAssertionNode {
            name: "rec".to_string(),
            target_statute: "voting".to_string(),
            mode: SnapshotMode::Record,
        });
        let report = doc.run(&[voting_statute()]);
        assert!(report.all_passed());
        assert_eq!(report.total_failures(), 0);
    }
}
