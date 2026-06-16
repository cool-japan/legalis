//! Tests for the advanced model-checking module.

use std::collections::HashSet;

use super::bdd::{
    BDD_FALSE, BDD_TRUE, Bdd, SymbolicCtlChecker, check_ctl_star_symbolic, check_ctl_symbolic,
    ctl_star_to_ctl,
};
use super::buchi::{check_ltl, ltl_to_gba};
use super::synthesis::TemporalPropertySynthesizer;
use super::zones::{
    DbmBound, Deadline, DeadlineTarget, DifferenceBoundMatrix, accepting_reachable_zone,
    check_deadline_satisfaction, reachable_zone_states, verify_deadline_reachable,
};
use crate::{
    Clock, ClockConstraint, CtlFormula, CtlStarFormula, CtlStarPathFormula, LtlFormula,
    TemporalState, TimedAutomaton, TimedLocation, TimedTransition, TransitionSystem, verify_ctl,
};

fn props(items: &[&str]) -> HashSet<String> {
    items.iter().map(|s| s.to_string()).collect()
}

// --------------------------------------------------------------------------
// LTL model checking via Büchi automata
// --------------------------------------------------------------------------

#[test]
fn test_ltl_to_gba_eventually_single_acceptance() {
    let formula = LtlFormula::eventually(LtlFormula::atom("p"));
    let gba = ltl_to_gba(&formula);
    assert!(gba.num_states > 0);
    assert!(!gba.initial.is_empty());
    // A single `Until` (F p ≡ true U p) gives a single generalized acceptance set.
    assert_eq!(gba.num_accepting_sets(), 1);
    assert!(gba.num_edges() > 0);
}

#[test]
fn test_ltl_to_gba_two_untils_two_acceptance() {
    let formula = LtlFormula::and(
        LtlFormula::eventually(LtlFormula::atom("a")),
        LtlFormula::eventually(LtlFormula::atom("b")),
    );
    let gba = ltl_to_gba(&formula);
    // Two distinct `Until` sub-formulas => two generalized acceptance sets.
    assert_eq!(gba.num_accepting_sets(), 2);
}

#[test]
fn test_check_ltl_globally_holds() {
    let mut system = TransitionSystem::new();
    system.add_state(TemporalState::new("s0").with_proposition("safe"));
    system.add_transition("s0", "s0");
    system.add_initial_state("s0");

    let result = check_ltl(&system, &LtlFormula::always(LtlFormula::atom("safe")));
    assert!(result.holds);
    assert!(result.counterexample.is_none());
}

#[test]
fn test_check_ltl_globally_fails_with_counterexample() {
    let mut system = TransitionSystem::new();
    system.add_state(TemporalState::new("s0").with_proposition("safe"));
    system.add_state(TemporalState::new("s1")); // no `safe`
    system.add_transition("s0", "s1");
    system.add_transition("s1", "s1");
    system.add_initial_state("s0");

    let result = check_ltl(&system, &LtlFormula::always(LtlFormula::atom("safe")));
    assert!(!result.holds);
    let lasso = result.counterexample.expect("counterexample expected");
    assert!(!lasso.loop_segment.is_empty());
    // The loop must eventually settle in the unsafe state.
    assert!(lasso.loop_segment.contains(&"s1".to_string()));
}

#[test]
fn test_check_ltl_eventually_holds_and_fails() {
    // Holds: every path reaches `goal`.
    let mut reaching = TransitionSystem::new();
    reaching.add_state(TemporalState::new("a"));
    reaching.add_state(TemporalState::new("b").with_proposition("goal"));
    reaching.add_transition("a", "b");
    reaching.add_transition("b", "b");
    reaching.add_initial_state("a");
    let ok = check_ltl(&reaching, &LtlFormula::eventually(LtlFormula::atom("goal")));
    assert!(ok.holds);

    // Fails: a self-loop that never reaches `goal`.
    let mut stuck = TransitionSystem::new();
    stuck.add_state(TemporalState::new("a"));
    stuck.add_transition("a", "a");
    stuck.add_initial_state("a");
    let bad = check_ltl(&stuck, &LtlFormula::eventually(LtlFormula::atom("goal")));
    assert!(!bad.holds);
    assert!(bad.counterexample.is_some());
}

#[test]
fn test_check_ltl_until() {
    let mut system = TransitionSystem::new();
    system.add_state(TemporalState::new("s0").with_proposition("p"));
    system.add_state(TemporalState::new("s1").with_proposition("q"));
    system.add_transition("s0", "s1");
    system.add_transition("s1", "s1");
    system.add_initial_state("s0");

    let formula = LtlFormula::until(LtlFormula::atom("p"), LtlFormula::atom("q"));
    let result = check_ltl(&system, &formula);
    assert!(result.holds);
}

#[test]
fn test_check_ltl_infinitely_often() {
    // a -> b -> a, with `p` true only in b: G F p holds.
    let mut system = TransitionSystem::new();
    system.add_state(TemporalState::new("a"));
    system.add_state(TemporalState::new("b").with_proposition("p"));
    system.add_transition("a", "b");
    system.add_transition("b", "a");
    system.add_initial_state("a");

    let gfp = LtlFormula::always(LtlFormula::eventually(LtlFormula::atom("p")));
    assert!(check_ltl(&system, &gfp).holds);

    // Self-loop with no `p`: G F p fails.
    let mut stuck = TransitionSystem::new();
    stuck.add_state(TemporalState::new("a"));
    stuck.add_transition("a", "a");
    stuck.add_initial_state("a");
    assert!(!check_ltl(&stuck, &gfp).holds);
}

// --------------------------------------------------------------------------
// Binary decision diagrams
// --------------------------------------------------------------------------

#[test]
fn test_bdd_boolean_identities() {
    let mut bdd = Bdd::new();
    let x = bdd.ithvar(0);
    let not_x = bdd.not(x);
    let contradiction = bdd.and(x, not_x);
    let tautology = bdd.or(x, not_x);
    assert_eq!(contradiction, BDD_FALSE);
    assert_eq!(tautology, BDD_TRUE);
    // Double negation.
    let nn_x = bdd.not(not_x);
    assert_eq!(nn_x, x);
}

#[test]
fn test_bdd_reduction_and_sharing() {
    let mut bdd = Bdd::new();
    let x1 = bdd.ithvar(3);
    let x2 = bdd.ithvar(3);
    // Structural sharing: identical variables map to the same node.
    assert_eq!(x1, x2);
    // Reduction: ite(f, g, g) collapses to g (redundant test elimination).
    let y = bdd.ithvar(5);
    let collapsed = bdd.ite(x1, y, y);
    assert_eq!(collapsed, y);
    // Idempotence.
    let and_xx = bdd.and(x1, x1);
    assert_eq!(and_xx, x1);
}

#[test]
fn test_bdd_restrict() {
    let mut bdd = Bdd::new();
    let x0 = bdd.ithvar(0);
    let x1 = bdd.ithvar(1);
    let f = bdd.and(x0, x1);
    // f|x0=true == x1 ; f|x0=false == false.
    let r_true = bdd.restrict(f, 0, true);
    let r_false = bdd.restrict(f, 0, false);
    assert_eq!(r_true, x1);
    assert_eq!(r_false, BDD_FALSE);
}

#[test]
fn test_bdd_exists() {
    let mut bdd = Bdd::new();
    let x0 = bdd.ithvar(0);
    let x1 = bdd.ithvar(1);
    let f = bdd.and(x0, x1);
    // ∃x0. (x0 ∧ x1) == x1.
    let exists_x0 = bdd.exists(f, 0);
    assert_eq!(exists_x0, x1);
}

#[test]
fn test_bdd_rename() {
    let mut bdd = Bdd::new();
    let x0 = bdd.ithvar(0);
    let x2 = bdd.ithvar(2);
    let mut mapping = std::collections::HashMap::new();
    mapping.insert(0usize, 2usize);
    let renamed = bdd.rename(x0, &mapping);
    assert_eq!(renamed, x2);
}

// --------------------------------------------------------------------------
// Symbolic CTL model checking
// --------------------------------------------------------------------------

fn goal_system() -> TransitionSystem {
    let mut system = TransitionSystem::new();
    system.add_state(TemporalState::new("s0"));
    system.add_state(TemporalState::new("s1").with_proposition("goal"));
    system.add_transition("s0", "s1");
    system.add_transition("s1", "s1");
    system.add_initial_state("s0");
    system
}

#[test]
fn test_symbolic_ef_and_ag() {
    let system = goal_system();
    let ef = CtlFormula::exists_eventually(CtlFormula::atom("goal"));
    let ef_result = check_ctl_symbolic(&system, &ef).expect("ctl check");
    assert!(ef_result.holds);
    assert!(ef_result.satisfying_states.contains(&"s0".to_string()));
    assert!(ef_result.satisfying_states.contains(&"s1".to_string()));

    let ag = CtlFormula::all_always(CtlFormula::atom("goal"));
    let ag_result = check_ctl_symbolic(&system, &ag).expect("ctl check");
    assert!(!ag_result.holds);
}

#[test]
fn test_symbolic_af() {
    let system = goal_system();
    let af = CtlFormula::all_eventually(CtlFormula::atom("goal"));
    let result = check_ctl_symbolic(&system, &af).expect("ctl check");
    assert!(result.holds);
}

#[test]
fn test_symbolic_eg() {
    let system = goal_system();
    let mut checker = SymbolicCtlChecker::from_system(&system).expect("build checker");
    let eg = CtlFormula::exists_always(CtlFormula::atom("goal"));
    let states = checker.satisfying_states(&eg);
    assert!(states.contains(&"s1".to_string()));
    assert!(!states.contains(&"s0".to_string()));
    // The initial state s0 does not satisfy EG goal.
    assert!(!checker.check(&eg));
}

#[test]
fn test_symbolic_matches_explicit() {
    let system = goal_system();
    let ef = CtlFormula::exists_eventually(CtlFormula::atom("goal"));
    let ag = CtlFormula::all_always(CtlFormula::atom("goal"));
    assert_eq!(
        check_ctl_symbolic(&system, &ef).expect("ctl").holds,
        verify_ctl(&system, &ef)
    );
    assert_eq!(
        check_ctl_symbolic(&system, &ag).expect("ctl").holds,
        verify_ctl(&system, &ag)
    );
}

#[test]
fn test_ctl_star_fragment_and_nonfragment() {
    // AG safe is in the CTL fragment.
    let safe = CtlStarFormula::atom("safe");
    let ag_path = CtlStarPathFormula::Always(Box::new(CtlStarPathFormula::State(Box::new(safe))));
    let ag = CtlStarFormula::all(ag_path);
    let translated = ctl_star_to_ctl(&ag).expect("translatable to CTL");
    assert!(matches!(translated, CtlFormula::AllAlways(_)));

    let mut system = TransitionSystem::new();
    system.add_state(TemporalState::new("s0").with_proposition("safe"));
    system.add_transition("s0", "s0");
    system.add_initial_state("s0");
    let result = check_ctl_star_symbolic(&system, &ag).expect("symbolic ctl*");
    assert!(result.holds);

    // A[F G p] genuinely nests temporal operators: outside the CTL fragment.
    let inner = CtlStarFormula::atom("p");
    let nested = CtlStarPathFormula::Eventually(Box::new(CtlStarPathFormula::Always(Box::new(
        CtlStarPathFormula::State(Box::new(inner)),
    ))));
    let ctl_star = CtlStarFormula::all(nested);
    assert!(ctl_star_to_ctl(&ctl_star).is_none());
    assert!(check_ctl_star_symbolic(&system, &ctl_star).is_err());
}

// --------------------------------------------------------------------------
// Difference Bound Matrices and zone graphs
// --------------------------------------------------------------------------

#[test]
fn test_dbm_bound_order() {
    // Strict (<) is tighter than non-strict (<=) at equal value.
    assert!(DbmBound::finite(5, true) < DbmBound::finite(5, false));
    // Infinity dominates every finite bound.
    assert!(DbmBound::Inf > DbmBound::finite(1_000, false));
    assert!(DbmBound::finite(3, false) < DbmBound::finite(4, true));
}

#[test]
fn test_dbm_zero_and_emptiness() {
    let clocks = vec!["x".to_string()];
    let mut dbm = DifferenceBoundMatrix::zero(&clocks);
    assert!(!dbm.is_empty());
    // x = 0 means both x <= 0 and x >= 0.
    assert_eq!(dbm.bound(1, 0), DbmBound::zero());
    assert_eq!(dbm.bound(0, 1), DbmBound::zero());

    // Contradictory constraints yield an empty zone.
    dbm.up();
    dbm.and_constraint(&ClockConstraint::LessOrEqual(Clock::new("x"), 2))
        .expect("intersect");
    dbm.and_constraint(&ClockConstraint::GreaterOrEqual(Clock::new("x"), 5))
        .expect("intersect");
    assert!(dbm.is_empty());
}

#[test]
fn test_dbm_up_reset() {
    let clocks = vec!["x".to_string()];
    let mut dbm = DifferenceBoundMatrix::zero(&clocks);
    dbm.up();
    // After delay the upper bound on x is removed.
    assert_eq!(dbm.bound(1, 0), DbmBound::Inf);
    // Lower bound x >= 0 is preserved.
    assert_eq!(dbm.bound(0, 1), DbmBound::zero());

    // Constrain x == 3, then reset it back to 0.
    dbm.and_constraint(&ClockConstraint::Equal(Clock::new("x"), 3))
        .expect("intersect");
    dbm.reset("x").expect("reset");
    assert_eq!(dbm.bound(1, 0), DbmBound::zero());
    assert_eq!(dbm.bound(0, 1), DbmBound::zero());
}

#[test]
fn test_dbm_canonicalize_propagation() {
    // From the origin, delay couples clocks (x = y); constraining x propagates to y.
    let clocks = vec!["x".to_string(), "y".to_string()];
    let mut dbm = DifferenceBoundMatrix::zero(&clocks);
    dbm.up();
    dbm.and_constraint(&ClockConstraint::LessOrEqual(Clock::new("x"), 5))
        .expect("intersect");
    // Canonicalization derives y <= 5 (index of y is 2).
    assert_eq!(dbm.bound(2, 0), DbmBound::finite(5, false));
}

#[test]
fn test_dbm_includes() {
    let clocks = vec!["x".to_string()];
    let mut wide = DifferenceBoundMatrix::zero(&clocks);
    wide.up();
    wide.and_constraint(&ClockConstraint::LessOrEqual(Clock::new("x"), 10))
        .expect("intersect");

    let mut narrow = DifferenceBoundMatrix::zero(&clocks);
    narrow.up();
    narrow
        .and_constraint(&ClockConstraint::GreaterOrEqual(Clock::new("x"), 2))
        .expect("intersect");
    narrow
        .and_constraint(&ClockConstraint::LessOrEqual(Clock::new("x"), 5))
        .expect("intersect");

    assert!(wide.includes(&narrow));
    assert!(!narrow.includes(&wide));
}

#[test]
fn test_zone_graph_terminates() {
    // A clock-resetting self-loop would diverge without extrapolation.
    let mut automaton = TimedAutomaton::new("idle");
    automaton.add_clock(Clock::new("x"));
    automaton.add_location(TimedLocation::new("idle"));
    automaton.add_transition(
        TimedTransition::new("idle", "idle", "tick")
            .with_guard(ClockConstraint::GreaterOrEqual(Clock::new("x"), 1))
            .with_reset(Clock::new("x")),
    );
    let zones = reachable_zone_states(&automaton).expect("zone graph");
    // Extrapolation + subsumption keep the zone graph finite.
    assert_eq!(zones.len(), 1);
    assert!(!accepting_reachable_zone(&automaton).expect("reachability"));
}

#[test]
fn test_deadline_reachable_and_guaranteed() {
    // idle --[x <= 5]--> served (accepting, invariant x <= 5).
    let mut automaton = TimedAutomaton::new("idle");
    automaton.add_clock(Clock::new("x"));
    automaton.add_location(TimedLocation::new("idle"));
    automaton.add_location(
        TimedLocation::new("served")
            .with_invariant(ClockConstraint::LessOrEqual(Clock::new("x"), 5))
            .accepting(),
    );
    automaton.add_transition(
        TimedTransition::new("idle", "served", "serve")
            .with_guard(ClockConstraint::LessOrEqual(Clock::new("x"), 5)),
    );

    assert!(accepting_reachable_zone(&automaton).expect("reachability"));
    let deadline = Deadline::before("x", 5);
    assert!(verify_deadline_reachable(&automaton, &deadline).expect("reachable"));

    let outcome = check_deadline_satisfaction(&automaton, &deadline).expect("deadline");
    assert!(outcome.satisfiable);
    assert!(outcome.guaranteed);
    assert!(outcome.violation.is_none());
    assert!(!outcome.witness.is_empty());
}

#[test]
fn test_deadline_not_guaranteed() {
    // idle --> served (accepting, NO invariant): time can drift past the deadline.
    let mut automaton = TimedAutomaton::new("idle");
    automaton.add_clock(Clock::new("x"));
    automaton.add_location(TimedLocation::new("idle"));
    automaton.add_location(TimedLocation::new("served").accepting());
    automaton.add_transition(TimedTransition::new("idle", "served", "serve"));

    let deadline = Deadline::before("x", 5);
    let outcome = check_deadline_satisfaction(&automaton, &deadline).expect("deadline");
    assert!(outcome.satisfiable);
    assert!(!outcome.guaranteed);
    let violation = outcome.violation.expect("violation expected");
    assert_eq!(violation.max_steps, 5);
}

#[test]
fn test_deadline_unreachable() {
    // The only way to `served` requires x >= 10, so the x <= 5 deadline is unmet.
    let mut automaton = TimedAutomaton::new("idle");
    automaton.add_clock(Clock::new("x"));
    automaton.add_location(TimedLocation::new("idle"));
    automaton.add_location(TimedLocation::new("served").accepting());
    automaton.add_transition(
        TimedTransition::new("idle", "served", "serve")
            .with_guard(ClockConstraint::GreaterOrEqual(Clock::new("x"), 10)),
    );

    let deadline = Deadline::new(
        ClockConstraint::LessOrEqual(Clock::new("x"), 5),
        DeadlineTarget::AcceptingLocations,
    );
    assert!(!verify_deadline_reachable(&automaton, &deadline).expect("reachable"));
}

// --------------------------------------------------------------------------
// Temporal property synthesis
// --------------------------------------------------------------------------

#[test]
fn test_synthesize_existence() {
    let synth = TemporalPropertySynthesizer::new();
    let positive = vec![
        vec![props(&["start"]), props(&["grant"])],
        vec![props(&[]), props(&["grant"]), props(&["done"])],
    ];
    let negative = vec![vec![props(&["start"]), props(&["deny"])], vec![props(&[])]];

    let outcome = synth.synthesize(&positive, &negative);
    assert!(outcome.separated);
    let best = outcome.best.expect("a separating formula");
    // The simplest separator should be "eventually grant".
    assert_eq!(best, LtlFormula::eventually(LtlFormula::atom("grant")));
    assert!(!outcome.candidates.is_empty());
}

#[test]
fn test_synthesize_response() {
    let synth = TemporalPropertySynthesizer::new();
    // Positives: every `request` is eventually followed by `grant`.
    let positive = vec![
        vec![props(&["request"]), props(&["grant"])],
        vec![props(&["idle"]), props(&["request"]), props(&["grant"])],
    ];
    // Negatives: a `request` never answered by `grant`.
    let negative = vec![vec![props(&["request"]), props(&["idle"])]];

    let outcome = synth.synthesize(&positive, &negative);
    assert!(outcome.separated);
    let best = outcome.best.expect("a separating formula");
    // The response candidate should satisfy the positives and reject the negatives.
    let response = LtlFormula::always(LtlFormula::implies(
        LtlFormula::atom("request"),
        LtlFormula::eventually(LtlFormula::atom("grant")),
    ));
    let matches_response = outcome
        .candidates
        .iter()
        .any(|c| c.formula == response && c.separates);
    assert!(matches_response || best == response);
}

#[test]
fn test_synthesize_partial_when_inseparable() {
    let synth = TemporalPropertySynthesizer::new();
    // Identical positive and negative examples: no perfect separator exists.
    let positive = vec![vec![props(&["p"]), props(&["q"])]];
    let negative = vec![vec![props(&["p"]), props(&["q"])]];

    let outcome = synth.synthesize(&positive, &negative);
    assert!(!outcome.separated);
    // A best-effort partial candidate is still returned.
    assert!(outcome.best.is_some());
    assert!(!outcome.candidates.is_empty());
}
