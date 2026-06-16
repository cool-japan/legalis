//! Advanced model checking for temporal legal specifications.
//!
//! This module deepens the lightweight temporal-logic helpers already present in
//! the crate (`verify_ltl`, `verify_ctl`, `verify_timed_reachability`,
//! `synthesize_ltl_property`) with full, algorithmically faithful model-checking
//! engines. It is organised into four sub-modules, each reusing the crate's
//! existing temporal types ([`crate::LtlFormula`], [`crate::CtlFormula`],
//! [`crate::CtlStarFormula`], [`crate::TransitionSystem`],
//! [`crate::TimedAutomaton`], [`crate::ClockConstraint`], ...):
//!
//! * [`buchi`] — full LTL model checking via the GPVW generalized Büchi
//!   automaton construction, on-the-fly degeneralization, and emptiness checking
//!   with a nested depth-first search yielding lasso counterexamples.
//! * [`bdd`] — a reduced ordered binary decision diagram (`apply`/`ite`,
//!   reduction, restriction, quantification) used for symbolic CTL/CTL\* model
//!   checking via least/greatest fixpoints.
//! * [`zones`] — timed-automaton verification with Difference Bound Matrices and
//!   zone graphs, including deadline reachability and deadline-guarantee checks.
//! * [`synthesis`] — temporal-property synthesis that infers a separating LTL
//!   formula from labelled positive/negative example traces using a library of
//!   specification patterns.
//!
//! # Example
//!
//! ```
//! use legalis_verifier::model_checking::check_ltl;
//! use legalis_verifier::{LtlFormula, TemporalState, TransitionSystem};
//!
//! let mut system = TransitionSystem::new();
//! system.add_state(TemporalState::new("s0").with_proposition("safe"));
//! system.add_transition("s0", "s0");
//! system.add_initial_state("s0");
//!
//! // "Globally safe" holds on the single safe self-loop.
//! let result = check_ltl(&system, &LtlFormula::always(LtlFormula::atom("safe")));
//! assert!(result.holds);
//! assert!(result.counterexample.is_none());
//! ```

pub mod bdd;
pub mod buchi;
pub mod synthesis;
pub mod zones;

pub use bdd::{
    Bdd, BddRef, CtlModelCheckResult, SymbolicCtlChecker, check_ctl_star_symbolic,
    check_ctl_symbolic, ctl_star_to_ctl,
};
pub use buchi::{
    GeneralizedBuchiAutomaton, LassoTrace, Literal, LtlModelCheckResult, check_ltl, ltl_to_gba,
};
pub use synthesis::{
    ScoredCandidate, SpecificationPattern, SynthesisOutcome, TemporalPropertySynthesizer, Trace,
    formula_size,
};
pub use zones::{
    DbmBound, Deadline, DeadlineOutcome, DeadlineTarget, DifferenceBoundMatrix, ZoneState,
    accepting_reachable_zone, check_deadline_satisfaction, reachable_zone_states,
    verify_deadline_reachable,
};

#[cfg(test)]
mod tests;
