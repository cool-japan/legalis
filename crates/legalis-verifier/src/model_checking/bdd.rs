//! Ordered binary decision diagrams (OBDDs) and symbolic CTL/CTL* checking.
//!
//! The [`Bdd`] type is a fully reduced, ordered BDD manager implementing
//! Bryant's classic `mk`/`ite` algorithms with a unique-node table (enforcing
//! the *reduced* property) and an `ite` memoization cache (enforcing efficient
//! `apply`). On top of it, [`SymbolicCtlChecker`] performs *symbolic* CTL model
//! checking: states are encoded as Boolean vectors, the transition relation and
//! every set of states become BDDs, and the temporal operators are evaluated as
//! least/greatest fixpoints (`EX`, `EU`, `EG`, ...).
//!
//! Pure CTL* model checking with BDDs is supported for the CTL-expressible
//! fragment via [`check_ctl_star_symbolic`]; formulas that nest linear-time
//! operators (genuine CTL*) fall outside this fragment and should be checked
//! with the explicit [`crate::verify_ctl_star`] evaluator instead.

use std::collections::HashMap;

use crate::{CtlFormula, CtlStarFormula, CtlStarPathFormula, TransitionSystem, VerificationError};

/// A reference to a node in a [`Bdd`] manager.
pub type BddRef = usize;

/// The `false` terminal node.
pub const BDD_FALSE: BddRef = 0;
/// The `true` terminal node.
pub const BDD_TRUE: BddRef = 1;
/// Sentinel "variable" of the terminal nodes (ordered after every real variable).
const TERMINAL_VAR: usize = usize::MAX;

/// An internal BDD node (a Shannon decomposition on `var`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BddNode {
    /// Decision variable.
    var: usize,
    /// Sub-diagram taken when `var = 0`.
    low: BddRef,
    /// Sub-diagram taken when `var = 1`.
    high: BddRef,
}

/// A reduced, ordered binary decision diagram manager.
#[derive(Debug, Clone)]
pub struct Bdd {
    nodes: Vec<BddNode>,
    unique: HashMap<(usize, BddRef, BddRef), BddRef>,
    ite_cache: HashMap<(BddRef, BddRef, BddRef), BddRef>,
}

impl Default for Bdd {
    fn default() -> Self {
        Self::new()
    }
}

impl Bdd {
    /// Creates an empty manager containing only the two terminal nodes.
    pub fn new() -> Self {
        let terminal = BddNode {
            var: TERMINAL_VAR,
            low: 0,
            high: 0,
        };
        Self {
            nodes: vec![terminal, terminal],
            unique: HashMap::new(),
            ite_cache: HashMap::new(),
        }
    }

    /// Number of live nodes (including the two terminals).
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Always `false`; present to satisfy the clippy `len`/`is_empty` pairing.
    pub fn is_empty(&self) -> bool {
        false
    }

    fn var_of(&self, f: BddRef) -> usize {
        self.nodes[f].var
    }

    fn low(&self, f: BddRef) -> BddRef {
        self.nodes[f].low
    }

    fn high(&self, f: BddRef) -> BddRef {
        self.nodes[f].high
    }

    /// The reduction operator: returns an existing node or allocates a fresh one,
    /// eliminating redundant tests where `low == high`.
    fn mk(&mut self, var: usize, low: BddRef, high: BddRef) -> BddRef {
        if low == high {
            return low;
        }
        let key = (var, low, high);
        if let Some(&existing) = self.unique.get(&key) {
            return existing;
        }
        let id = self.nodes.len();
        self.nodes.push(BddNode { var, low, high });
        self.unique.insert(key, id);
        id
    }

    fn cofactor(&self, f: BddRef, var: usize) -> (BddRef, BddRef) {
        if self.var_of(f) == var {
            (self.low(f), self.high(f))
        } else {
            (f, f)
        }
    }

    /// The if-then-else `apply` operator: `ite(f, g, h) = (f ∧ g) ∨ (¬f ∧ h)`.
    pub fn ite(&mut self, f: BddRef, g: BddRef, h: BddRef) -> BddRef {
        if f == BDD_TRUE {
            return g;
        }
        if f == BDD_FALSE {
            return h;
        }
        if g == h {
            return g;
        }
        if g == BDD_TRUE && h == BDD_FALSE {
            return f;
        }
        let key = (f, g, h);
        if let Some(&cached) = self.ite_cache.get(&key) {
            return cached;
        }
        let var = self.var_of(f).min(self.var_of(g)).min(self.var_of(h));
        let (f0, f1) = self.cofactor(f, var);
        let (g0, g1) = self.cofactor(g, var);
        let (h0, h1) = self.cofactor(h, var);
        let low = self.ite(f0, g0, h0);
        let high = self.ite(f1, g1, h1);
        let result = self.mk(var, low, high);
        self.ite_cache.insert(key, result);
        result
    }

    /// The BDD for the positive literal of variable `index`.
    pub fn ithvar(&mut self, index: usize) -> BddRef {
        self.mk(index, BDD_FALSE, BDD_TRUE)
    }

    /// Logical negation.
    pub fn not(&mut self, f: BddRef) -> BddRef {
        self.ite(f, BDD_FALSE, BDD_TRUE)
    }

    /// Logical conjunction.
    pub fn and(&mut self, f: BddRef, g: BddRef) -> BddRef {
        self.ite(f, g, BDD_FALSE)
    }

    /// Logical disjunction.
    pub fn or(&mut self, f: BddRef, g: BddRef) -> BddRef {
        self.ite(f, BDD_TRUE, g)
    }

    /// Logical exclusive-or.
    pub fn xor(&mut self, f: BddRef, g: BddRef) -> BddRef {
        let not_g = self.not(g);
        self.ite(f, not_g, g)
    }

    /// Logical implication `f → g`.
    pub fn implies(&mut self, f: BddRef, g: BddRef) -> BddRef {
        self.ite(f, g, BDD_TRUE)
    }

    /// Restricts (cofactors) `f` by fixing `var` to `value`.
    pub fn restrict(&mut self, f: BddRef, var: usize, value: bool) -> BddRef {
        let mut memo = HashMap::new();
        self.restrict_rec(f, var, value, &mut memo)
    }

    fn restrict_rec(
        &mut self,
        f: BddRef,
        var: usize,
        value: bool,
        memo: &mut HashMap<BddRef, BddRef>,
    ) -> BddRef {
        let vf = self.var_of(f);
        if vf > var {
            return f;
        }
        if let Some(&cached) = memo.get(&f) {
            return cached;
        }
        let result = if vf == var {
            if value { self.high(f) } else { self.low(f) }
        } else {
            let lo = self.low(f);
            let hi = self.high(f);
            let low = self.restrict_rec(lo, var, value, memo);
            let high = self.restrict_rec(hi, var, value, memo);
            self.mk(vf, low, high)
        };
        memo.insert(f, result);
        result
    }

    /// Existentially quantifies a single variable: `∃var. f`.
    pub fn exists(&mut self, f: BddRef, var: usize) -> BddRef {
        let lo = self.restrict(f, var, false);
        let hi = self.restrict(f, var, true);
        self.or(lo, hi)
    }

    /// Existentially quantifies a set of variables.
    pub fn exists_set(&mut self, f: BddRef, vars: &[usize]) -> BddRef {
        let mut result = f;
        for &var in vars {
            result = self.exists(result, var);
        }
        result
    }

    /// Renames variables according to an order-preserving `mapping`.
    pub fn rename(&mut self, f: BddRef, mapping: &HashMap<usize, usize>) -> BddRef {
        let mut memo = HashMap::new();
        self.rename_rec(f, mapping, &mut memo)
    }

    fn rename_rec(
        &mut self,
        f: BddRef,
        mapping: &HashMap<usize, usize>,
        memo: &mut HashMap<BddRef, BddRef>,
    ) -> BddRef {
        if f == BDD_FALSE || f == BDD_TRUE {
            return f;
        }
        if let Some(&cached) = memo.get(&f) {
            return cached;
        }
        let vf = self.var_of(f);
        let nv = *mapping.get(&vf).unwrap_or(&vf);
        let lo = self.low(f);
        let hi = self.high(f);
        let low = self.rename_rec(lo, mapping, memo);
        let high = self.rename_rec(hi, mapping, memo);
        let result = self.mk(nv, low, high);
        memo.insert(f, result);
        result
    }
}

/// Maps original variable indices to renamed ones.
type RenameMap = HashMap<usize, usize>;

/// Result of a symbolic CTL model-checking query.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CtlModelCheckResult {
    /// Whether all initial states satisfy the formula.
    pub holds: bool,
    /// The (sorted) identifiers of every state that satisfies the formula.
    pub satisfying_states: Vec<String>,
}

/// Symbolic CTL model checker over a [`TransitionSystem`], backed by a [`Bdd`].
pub struct SymbolicCtlChecker {
    bdd: Bdd,
    index_state: Vec<String>,
    bits: usize,
    present_vars: Vec<usize>,
    next_vars: Vec<usize>,
    trans: BddRef,
    states_bdd: BddRef,
    initial_bdd: BddRef,
    prop_to_states: HashMap<String, Vec<usize>>,
    prop_cache: HashMap<String, BddRef>,
    p2n: RenameMap,
}

impl SymbolicCtlChecker {
    /// Builds a symbolic checker by encoding `system` into BDDs.
    ///
    /// Deadlock states (those with no listed successors) are given an implicit
    /// self-loop so the transition relation is total, matching the standard
    /// convention for CTL fixpoint evaluation.
    pub fn from_system(system: &TransitionSystem) -> Result<Self, VerificationError> {
        let mut index_state: Vec<String> = system.states.keys().cloned().collect();
        index_state.sort();
        let n = index_state.len();
        if n == 0 {
            return Err(VerificationError::LogicalContradiction {
                message: "cannot model-check an empty transition system".to_string(),
            });
        }
        let mut bits = 1usize;
        while (1usize << bits) < n {
            bits += 1;
            if bits > 31 {
                return Err(VerificationError::LogicalContradiction {
                    message: "transition system too large for symbolic encoding".to_string(),
                });
            }
        }

        let mut state_index: HashMap<String, usize> = HashMap::new();
        for (idx, id) in index_state.iter().enumerate() {
            state_index.insert(id.clone(), idx);
        }

        let present_vars: Vec<usize> = (0..bits).map(|b| 2 * b).collect();
        let next_vars: Vec<usize> = (0..bits).map(|b| 2 * b + 1).collect();
        let mut p2n = HashMap::new();
        for b in 0..bits {
            p2n.insert(2 * b, 2 * b + 1);
        }

        let mut bdd = Bdd::new();

        // Characteristic BDD of all valid states.
        let mut states_bdd = BDD_FALSE;
        for idx in 0..n {
            let enc = encode_index(&mut bdd, idx, bits, false);
            states_bdd = bdd.or(states_bdd, enc);
        }

        // Transition relation, totalized with self-loops on deadlocks.
        let mut trans = BDD_FALSE;
        for (idx, id) in index_state.iter().enumerate() {
            let succ_ids = system.transitions.get(id);
            let valid_succ: Vec<usize> = succ_ids
                .map(|ids| {
                    ids.iter()
                        .filter_map(|s| state_index.get(s).copied())
                        .collect()
                })
                .unwrap_or_default();
            let targets = if valid_succ.is_empty() {
                vec![idx]
            } else {
                valid_succ
            };
            let from = encode_index(&mut bdd, idx, bits, false);
            for target in targets {
                let to = encode_index(&mut bdd, target, bits, true);
                let edge = bdd.and(from, to);
                trans = bdd.or(trans, edge);
            }
        }

        // Initial states.
        let mut initial_bdd = BDD_FALSE;
        for id in &system.initial_states {
            if let Some(&idx) = state_index.get(id) {
                let enc = encode_index(&mut bdd, idx, bits, false);
                initial_bdd = bdd.or(initial_bdd, enc);
            }
        }

        // Proposition -> states map.
        let mut prop_to_states: HashMap<String, Vec<usize>> = HashMap::new();
        for (id, state) in &system.states {
            let Some(&idx) = state_index.get(id) else {
                continue;
            };
            for prop in &state.propositions {
                prop_to_states.entry(prop.clone()).or_default().push(idx);
            }
        }

        Ok(Self {
            bdd,
            index_state,
            bits,
            present_vars,
            next_vars,
            trans,
            states_bdd,
            initial_bdd,
            prop_to_states,
            prop_cache: HashMap::new(),
            p2n,
        })
    }

    /// Number of present-state Boolean variables in the encoding.
    pub fn num_vars(&self) -> usize {
        self.present_vars.len()
    }

    fn prop_bdd(&mut self, prop: &str) -> BddRef {
        if let Some(&cached) = self.prop_cache.get(prop) {
            return cached;
        }
        let indices = self.prop_to_states.get(prop).cloned().unwrap_or_default();
        let mut result = BDD_FALSE;
        for idx in indices {
            let enc = encode_index(&mut self.bdd, idx, self.bits, false);
            result = self.bdd.or(result, enc);
        }
        self.prop_cache.insert(prop.to_string(), result);
        result
    }

    /// Complement restricted to the valid-state universe.
    fn complement(&mut self, f: BddRef) -> BddRef {
        let nf = self.bdd.not(f);
        self.bdd.and(self.states_bdd, nf)
    }

    /// Existential pre-image: states with a successor in `target`.
    fn ex(&mut self, target: BddRef) -> BddRef {
        let next = self.bdd.rename(target, &self.p2n);
        let conj = self.bdd.and(self.trans, next);
        let next_vars = self.next_vars.clone();
        let projected = self.bdd.exists_set(conj, &next_vars);
        self.bdd.and(projected, self.states_bdd)
    }

    fn ax(&mut self, target: BddRef) -> BddRef {
        let neg = self.complement(target);
        let ex = self.ex(neg);
        self.complement(ex)
    }

    /// Least fixpoint `E[f U g]`.
    fn eu(&mut self, f: BddRef, g: BddRef) -> BddRef {
        let mut z = g;
        loop {
            let ex_z = self.ex(z);
            let pre = self.bdd.and(f, ex_z);
            let new_z = self.bdd.or(g, pre);
            if new_z == z {
                return z;
            }
            z = new_z;
        }
    }

    /// Greatest fixpoint `EG f`.
    fn eg(&mut self, f: BddRef) -> BddRef {
        let mut z = f;
        loop {
            let ex_z = self.ex(z);
            let new_z = self.bdd.and(f, ex_z);
            if new_z == z {
                return z;
            }
            z = new_z;
        }
    }

    fn ef(&mut self, f: BddRef) -> BddRef {
        let states = self.states_bdd;
        self.eu(states, f)
    }

    fn af(&mut self, f: BddRef) -> BddRef {
        let neg = self.complement(f);
        let eg = self.eg(neg);
        self.complement(eg)
    }

    fn ag(&mut self, f: BddRef) -> BddRef {
        let neg = self.complement(f);
        let ef = self.ef(neg);
        self.complement(ef)
    }

    /// `A[f U g] ≡ ¬(E[¬g U (¬f ∧ ¬g)] ∨ EG ¬g)`.
    fn au(&mut self, f: BddRef, g: BddRef) -> BddRef {
        let not_g = self.complement(g);
        let not_f = self.complement(f);
        let not_f_and_not_g = self.bdd.and(not_f, not_g);
        let eu = self.eu(not_g, not_f_and_not_g);
        let eg = self.eg(not_g);
        let disj = self.bdd.or(eu, eg);
        self.complement(disj)
    }

    /// Evaluates a CTL formula, returning the BDD of its satisfying state set.
    pub fn eval(&mut self, formula: &CtlFormula) -> BddRef {
        match formula {
            CtlFormula::Atom(p) => self.prop_bdd(p),
            CtlFormula::Not(f) => {
                let inner = self.eval(f);
                self.complement(inner)
            }
            CtlFormula::And(a, b) => {
                let l = self.eval(a);
                let r = self.eval(b);
                self.bdd.and(l, r)
            }
            CtlFormula::Or(a, b) => {
                let l = self.eval(a);
                let r = self.eval(b);
                self.bdd.or(l, r)
            }
            CtlFormula::Implies(a, b) => {
                let l = self.eval(a);
                let r = self.eval(b);
                let not_l = self.complement(l);
                self.bdd.or(not_l, r)
            }
            CtlFormula::ExistsNext(f) => {
                let inner = self.eval(f);
                self.ex(inner)
            }
            CtlFormula::AllNext(f) => {
                let inner = self.eval(f);
                self.ax(inner)
            }
            CtlFormula::ExistsEventually(f) => {
                let inner = self.eval(f);
                self.ef(inner)
            }
            CtlFormula::AllEventually(f) => {
                let inner = self.eval(f);
                self.af(inner)
            }
            CtlFormula::ExistsAlways(f) => {
                let inner = self.eval(f);
                self.eg(inner)
            }
            CtlFormula::AllAlways(f) => {
                let inner = self.eval(f);
                self.ag(inner)
            }
            CtlFormula::ExistsUntil(a, b) => {
                let l = self.eval(a);
                let r = self.eval(b);
                self.eu(l, r)
            }
            CtlFormula::AllUntil(a, b) => {
                let l = self.eval(a);
                let r = self.eval(b);
                self.au(l, r)
            }
        }
    }

    /// Whether every initial state satisfies `formula`.
    pub fn check(&mut self, formula: &CtlFormula) -> bool {
        let phi = self.eval(formula);
        let not_phi = self.bdd.not(phi);
        let bad = self.bdd.and(self.initial_bdd, not_phi);
        bad == BDD_FALSE
    }

    /// The identifiers of all states satisfying `formula`.
    pub fn satisfying_states(&mut self, formula: &CtlFormula) -> Vec<String> {
        let phi = self.eval(formula);
        let mut result = Vec::new();
        for idx in 0..self.index_state.len() {
            let enc = encode_index(&mut self.bdd, idx, self.bits, false);
            let conj = self.bdd.and(enc, phi);
            if conj == enc {
                result.push(self.index_state[idx].clone());
            }
        }
        result
    }
}

/// Encodes a state index as a conjunction of (possibly negated) bit literals.
fn encode_index(bdd: &mut Bdd, index: usize, bits: usize, use_next: bool) -> BddRef {
    let mut result = BDD_TRUE;
    for b in 0..bits {
        let var = if use_next { 2 * b + 1 } else { 2 * b };
        let lit = bdd.ithvar(var);
        let bit = (index >> b) & 1 == 1;
        let factor = if bit { lit } else { bdd.not(lit) };
        result = bdd.and(result, factor);
    }
    result
}

/// Symbolic CTL model checking with BDD-based fixpoint evaluation.
pub fn check_ctl_symbolic(
    system: &TransitionSystem,
    formula: &CtlFormula,
) -> Result<CtlModelCheckResult, VerificationError> {
    let mut checker = SymbolicCtlChecker::from_system(system)?;
    let satisfying_states = checker.satisfying_states(formula);
    let holds = checker.check(formula);
    Ok(CtlModelCheckResult {
        holds,
        satisfying_states,
    })
}

/// Translates a CTL\* formula into CTL when it lies in the CTL fragment.
///
/// Returns `None` when the formula nests linear-time operators (genuine CTL\*),
/// which cannot be expressed as CTL and therefore cannot be checked by the
/// fixpoint engine.
pub fn ctl_star_to_ctl(formula: &CtlStarFormula) -> Option<CtlFormula> {
    match formula {
        CtlStarFormula::Atom(p) => Some(CtlFormula::atom(p.clone())),
        CtlStarFormula::Not(f) => Some(CtlFormula::not(ctl_star_to_ctl(f)?)),
        CtlStarFormula::And(a, b) => {
            Some(CtlFormula::and(ctl_star_to_ctl(a)?, ctl_star_to_ctl(b)?))
        }
        CtlStarFormula::Or(a, b) => Some(CtlFormula::or(ctl_star_to_ctl(a)?, ctl_star_to_ctl(b)?)),
        CtlStarFormula::Implies(a, b) => Some(CtlFormula::implies(
            ctl_star_to_ctl(a)?,
            ctl_star_to_ctl(b)?,
        )),
        CtlStarFormula::Exists(p) => path_to_ctl(p, true),
        CtlStarFormula::All(p) => path_to_ctl(p, false),
    }
}

/// Extracts a CTL state formula from a path formula that is just a state formula.
fn path_state(path: &CtlStarPathFormula) -> Option<CtlFormula> {
    match path {
        CtlStarPathFormula::State(state) => ctl_star_to_ctl(state),
        _ => None,
    }
}

/// Translates `E path` / `A path` to CTL when `path` is a single temporal
/// operator applied to state formulas.
fn path_to_ctl(path: &CtlStarPathFormula, exists: bool) -> Option<CtlFormula> {
    match path {
        CtlStarPathFormula::State(state) => ctl_star_to_ctl(state),
        CtlStarPathFormula::Next(inner) => {
            let f = path_state(inner)?;
            Some(if exists {
                CtlFormula::exists_next(f)
            } else {
                CtlFormula::all_next(f)
            })
        }
        CtlStarPathFormula::Eventually(inner) => {
            let f = path_state(inner)?;
            Some(if exists {
                CtlFormula::exists_eventually(f)
            } else {
                CtlFormula::all_eventually(f)
            })
        }
        CtlStarPathFormula::Always(inner) => {
            let f = path_state(inner)?;
            Some(if exists {
                CtlFormula::exists_always(f)
            } else {
                CtlFormula::all_always(f)
            })
        }
        CtlStarPathFormula::Until(left, right) => {
            let l = path_state(left)?;
            let r = path_state(right)?;
            Some(if exists {
                CtlFormula::exists_until(l, r)
            } else {
                CtlFormula::all_until(l, r)
            })
        }
        CtlStarPathFormula::Release(left, right) => {
            // a R b ≡ ¬(¬a U ¬b); E[a R b] ≡ ¬A[¬a U ¬b]; A[a R b] ≡ ¬E[¬a U ¬b].
            let l = path_state(left)?;
            let r = path_state(right)?;
            let not_l = CtlFormula::not(l);
            let not_r = CtlFormula::not(r);
            Some(if exists {
                CtlFormula::not(CtlFormula::all_until(not_l, not_r))
            } else {
                CtlFormula::not(CtlFormula::exists_until(not_l, not_r))
            })
        }
        CtlStarPathFormula::Not(_)
        | CtlStarPathFormula::And(_, _)
        | CtlStarPathFormula::Or(_, _) => None,
    }
}

/// Symbolic CTL\* model checking for the CTL-expressible fragment.
///
/// When `formula` lies outside CTL an error is returned advising use of the
/// explicit [`crate::verify_ctl_star`] evaluator.
pub fn check_ctl_star_symbolic(
    system: &TransitionSystem,
    formula: &CtlStarFormula,
) -> Result<CtlModelCheckResult, VerificationError> {
    match ctl_star_to_ctl(formula) {
        Some(ctl) => check_ctl_symbolic(system, &ctl),
        None => Err(VerificationError::LogicalContradiction {
            message: "CTL* formula nests path operators outside the CTL fragment; \
                      use verify_ctl_star for explicit evaluation"
                .to_string(),
        }),
    }
}
