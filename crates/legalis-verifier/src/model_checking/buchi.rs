//! Full LTL model checking via Büchi automata.
//!
//! This module implements *genuine* automata-theoretic LTL model checking over
//! infinite words, in contrast to the lightweight finite-path recursion offered
//! by [`crate::verify_ltl`]. The pipeline is the classic one:
//!
//! 1. The LTL formula is rewritten into negation normal form (NNF) over the
//!    minimal operator basis `{Atom, ¬Atom, ∧, ∨, X, U, R}`.
//! 2. A *generalized* Büchi automaton (GBA) is constructed using the on-the-fly
//!    tableau procedure of Gerth, Peled, Vardi and Wolper (the "GPVW"
//!    construction) based on `new`/`old`/`next` node expansion.
//! 3. To check `M ⊨ φ` we build the GBA of `¬φ`, take the synchronous product
//!    with the Kripke structure ([`TransitionSystem`]) while degeneralizing the
//!    acceptance condition on the fly, and test the product for emptiness with
//!    a *nested depth-first search* (Courcoubetis–Vardi–Wolper–Yannakakis).
//! 4. A non-empty product yields a lasso-shaped counterexample (a finite prefix
//!    followed by an infinitely repeated loop).

use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::rc::Rc;

use crate::{LtlFormula, TemporalState, TransitionSystem};

/// A literal label: a proposition name together with the required polarity.
///
/// `(p, true)` means `p` must hold; `(p, false)` means `p` must *not* hold.
pub type Literal = (String, bool);

/// Internal LTL representation in negation normal form.
///
/// Negation is pushed to the atomic level, leaving only the operators that the
/// GPVW tableau construction needs to decompose. `Rc` makes the recursive
/// structure cheap to clone while keeping structural `Eq`/`Hash`/`Ord`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Ltl {
    /// Boolean constant `true`.
    True,
    /// Boolean constant `false`.
    False,
    /// Positive literal.
    Atom(String),
    /// Negative literal.
    NotAtom(String),
    /// Conjunction.
    And(Rc<Ltl>, Rc<Ltl>),
    /// Disjunction.
    Or(Rc<Ltl>, Rc<Ltl>),
    /// Next.
    Next(Rc<Ltl>),
    /// Until (strong).
    Until(Rc<Ltl>, Rc<Ltl>),
    /// Release (dual of until).
    Release(Rc<Ltl>, Rc<Ltl>),
}

/// Rewrites an [`LtlFormula`] into NNF, optionally negating it first.
fn to_nnf(formula: &LtlFormula, negated: bool) -> Ltl {
    match formula {
        LtlFormula::Atom(p) => {
            if negated {
                Ltl::NotAtom(p.clone())
            } else {
                Ltl::Atom(p.clone())
            }
        }
        LtlFormula::Not(f) => to_nnf(f, !negated),
        LtlFormula::And(a, b) => {
            let (la, lb) = (to_nnf(a, negated), to_nnf(b, negated));
            if negated {
                Ltl::Or(Rc::new(la), Rc::new(lb))
            } else {
                Ltl::And(Rc::new(la), Rc::new(lb))
            }
        }
        LtlFormula::Or(a, b) => {
            let (la, lb) = (to_nnf(a, negated), to_nnf(b, negated));
            if negated {
                Ltl::And(Rc::new(la), Rc::new(lb))
            } else {
                Ltl::Or(Rc::new(la), Rc::new(lb))
            }
        }
        LtlFormula::Implies(a, b) => {
            // a -> b  ≡  ¬a ∨ b
            if negated {
                Ltl::And(Rc::new(to_nnf(a, false)), Rc::new(to_nnf(b, true)))
            } else {
                Ltl::Or(Rc::new(to_nnf(a, true)), Rc::new(to_nnf(b, false)))
            }
        }
        LtlFormula::Next(f) => Ltl::Next(Rc::new(to_nnf(f, negated))),
        LtlFormula::Eventually(f) => {
            // F f ≡ true U f ; ¬F f ≡ G ¬f ≡ false R ¬f
            if negated {
                Ltl::Release(Rc::new(Ltl::False), Rc::new(to_nnf(f, true)))
            } else {
                Ltl::Until(Rc::new(Ltl::True), Rc::new(to_nnf(f, false)))
            }
        }
        LtlFormula::Always(f) => {
            // G f ≡ false R f ; ¬G f ≡ F ¬f ≡ true U ¬f
            if negated {
                Ltl::Until(Rc::new(Ltl::True), Rc::new(to_nnf(f, true)))
            } else {
                Ltl::Release(Rc::new(Ltl::False), Rc::new(to_nnf(f, false)))
            }
        }
        LtlFormula::Until(a, b) => {
            // ¬(a U b) ≡ ¬a R ¬b
            if negated {
                Ltl::Release(Rc::new(to_nnf(a, true)), Rc::new(to_nnf(b, true)))
            } else {
                Ltl::Until(Rc::new(to_nnf(a, false)), Rc::new(to_nnf(b, false)))
            }
        }
        LtlFormula::Release(a, b) => {
            // ¬(a R b) ≡ ¬a U ¬b
            if negated {
                Ltl::Until(Rc::new(to_nnf(a, true)), Rc::new(to_nnf(b, true)))
            } else {
                Ltl::Release(Rc::new(to_nnf(a, false)), Rc::new(to_nnf(b, false)))
            }
        }
    }
}

/// Returns the literal dual of an atomic formula.
fn negate_literal(lit: &Ltl) -> Ltl {
    match lit {
        Ltl::Atom(p) => Ltl::NotAtom(p.clone()),
        Ltl::NotAtom(p) => Ltl::Atom(p.clone()),
        other => other.clone(),
    }
}

/// Collects every sub-formula (the syntactic closure) into `acc`.
fn collect_subformulas(f: &Ltl, acc: &mut BTreeSet<Ltl>) {
    if !acc.insert(f.clone()) {
        return;
    }
    match f {
        Ltl::And(a, b) | Ltl::Or(a, b) | Ltl::Until(a, b) | Ltl::Release(a, b) => {
            collect_subformulas(a, acc);
            collect_subformulas(b, acc);
        }
        Ltl::Next(a) => collect_subformulas(a, acc),
        _ => {}
    }
}

/// A node of the GPVW tableau.
#[derive(Clone)]
struct Node {
    /// Unique identifier for this node.
    id: usize,
    /// Identifiers of predecessor nodes (the sentinel [`INIT`] marks an initial edge).
    incoming: BTreeSet<usize>,
    /// Sub-formulas still to be processed in the current state.
    new: Vec<Ltl>,
    /// Sub-formulas already committed to hold in the current state.
    old: BTreeSet<Ltl>,
    /// Obligations deferred to the successor state.
    next: BTreeSet<Ltl>,
}

/// Sentinel predecessor id representing the automaton's initial edge.
const INIT: usize = 0;

/// Driver state for the GPVW expansion.
struct GpvwBuilder {
    /// Completed (fully expanded) nodes.
    nodes: Vec<Node>,
    /// Fresh-id counter (`INIT` is reserved as `0`).
    counter: usize,
}

impl GpvwBuilder {
    fn fresh_id(&mut self) -> usize {
        self.counter += 1;
        self.counter
    }

    /// Expands a single tableau node, possibly splitting it, recursing until the
    /// `new` work-list is exhausted.
    fn expand(&mut self, mut node: Node) {
        if node.new.is_empty() {
            if let Some(existing) = self
                .nodes
                .iter_mut()
                .find(|q| q.old == node.old && q.next == node.next)
            {
                existing.incoming.extend(node.incoming.iter().copied());
                return;
            }
            let succ_id = self.fresh_id();
            let succ = Node {
                id: succ_id,
                incoming: {
                    let mut set = BTreeSet::new();
                    set.insert(node.id);
                    set
                },
                new: node.next.iter().cloned().collect(),
                old: BTreeSet::new(),
                next: BTreeSet::new(),
            };
            self.nodes.push(node);
            self.expand(succ);
            return;
        }

        let eta = node.new.remove(0);
        if node.old.contains(&eta) {
            self.expand(node);
            return;
        }

        match eta.clone() {
            Ltl::True => {
                node.old.insert(eta);
                self.expand(node);
            }
            Ltl::False => {
                // Local contradiction: discard this branch.
            }
            Ltl::Atom(_) | Ltl::NotAtom(_) => {
                if node.old.contains(&negate_literal(&eta)) {
                    return; // contradictory literal: discard.
                }
                node.old.insert(eta);
                self.expand(node);
            }
            Ltl::And(a, b) => {
                node.old.insert(eta);
                let a = (*a).clone();
                let b = (*b).clone();
                if !node.old.contains(&a) {
                    node.new.push(a);
                }
                if !node.old.contains(&b) {
                    node.new.push(b);
                }
                self.expand(node);
            }
            Ltl::Next(a) => {
                node.old.insert(eta);
                node.next.insert((*a).clone());
                self.expand(node);
            }
            Ltl::Or(a, b) => {
                self.split(node, eta, vec![(*a).clone()], vec![], vec![(*b).clone()]);
            }
            Ltl::Until(a, b) => {
                // a U b: now-branch keeps a and defers (a U b); end-branch satisfies b.
                let deferred = eta.clone();
                self.split(
                    node,
                    eta,
                    vec![(*a).clone()],
                    vec![deferred],
                    vec![(*b).clone()],
                );
            }
            Ltl::Release(a, b) => {
                // a R b: now-branch keeps b and defers (a R b); end-branch satisfies a ∧ b.
                let deferred = eta.clone();
                self.split(
                    node,
                    eta,
                    vec![(*b).clone()],
                    vec![deferred],
                    vec![(*a).clone(), (*b).clone()],
                );
            }
        }
    }

    /// Splits a node into two successor branches (used for `∨`, `U` and `R`).
    fn split(&mut self, node: Node, eta: Ltl, new1: Vec<Ltl>, next1: Vec<Ltl>, new2: Vec<Ltl>) {
        let id1 = self.fresh_id();
        let mut n1 = node.clone();
        n1.id = id1;
        n1.old.insert(eta.clone());
        for f in new1 {
            if !n1.old.contains(&f) {
                n1.new.push(f);
            }
        }
        for f in next1 {
            n1.next.insert(f);
        }

        let id2 = self.fresh_id();
        let mut n2 = node;
        n2.id = id2;
        n2.old.insert(eta);
        for f in new2 {
            if !n2.old.contains(&f) {
                n2.new.push(f);
            }
        }

        self.expand(n1);
        self.expand(n2);
    }
}

/// A generalized Büchi automaton recognising the models of an LTL formula.
///
/// Acceptance is *generalized*: a run is accepting iff, for every set in
/// [`accepting_sets`](Self::accepting_sets), it visits some member of that set
/// infinitely often.
#[derive(Debug, Clone)]
pub struct GeneralizedBuchiAutomaton {
    /// Number of automaton states.
    pub num_states: usize,
    /// Indices of the initial states.
    pub initial: Vec<usize>,
    /// Outgoing edges per state (by index).
    pub edges: Vec<Vec<usize>>,
    /// Literal constraints that must hold in each state.
    pub labels: Vec<Vec<Literal>>,
    /// Generalized acceptance condition (one set per `U` sub-formula).
    pub accepting_sets: Vec<Vec<usize>>,
}

impl GeneralizedBuchiAutomaton {
    /// Number of generalized acceptance sets.
    pub fn num_accepting_sets(&self) -> usize {
        self.accepting_sets.len()
    }

    /// Total number of edges in the automaton.
    pub fn num_edges(&self) -> usize {
        self.edges.iter().map(Vec::len).sum()
    }
}

/// Constructs a generalized Büchi automaton for `formula` (the GPVW tableau).
pub fn ltl_to_gba(formula: &LtlFormula) -> GeneralizedBuchiAutomaton {
    build_gba(&to_nnf(formula, false))
}

/// Builds a GBA from an already-normalized formula.
fn build_gba(nnf: &Ltl) -> GeneralizedBuchiAutomaton {
    let mut builder = GpvwBuilder {
        nodes: Vec::new(),
        counter: INIT,
    };
    let root_id = builder.fresh_id();
    let root = Node {
        id: root_id,
        incoming: {
            let mut set = BTreeSet::new();
            set.insert(INIT);
            set
        },
        new: vec![nnf.clone()],
        old: BTreeSet::new(),
        next: BTreeSet::new(),
    };
    builder.expand(root);

    let nodes = builder.nodes;
    let mut id_to_index: HashMap<usize, usize> = HashMap::new();
    for (index, node) in nodes.iter().enumerate() {
        id_to_index.insert(node.id, index);
    }

    let num_states = nodes.len();
    let mut edges = vec![Vec::new(); num_states];
    let mut initial = Vec::new();
    let mut labels = vec![Vec::new(); num_states];

    for (index, node) in nodes.iter().enumerate() {
        // Extract the literal label of this state.
        let mut lits: Vec<Literal> = Vec::new();
        for f in &node.old {
            match f {
                Ltl::Atom(p) => lits.push((p.clone(), true)),
                Ltl::NotAtom(p) => lits.push((p.clone(), false)),
                _ => {}
            }
        }
        lits.sort();
        lits.dedup();
        labels[index] = lits;

        for pred in &node.incoming {
            if *pred == INIT {
                initial.push(index);
            } else if let Some(&pred_index) = id_to_index.get(pred) {
                edges[pred_index].push(index);
            }
        }
    }
    for succ in &mut edges {
        succ.sort_unstable();
        succ.dedup();
    }
    initial.sort_unstable();
    initial.dedup();

    // Generalized acceptance: one set per `Until` sub-formula in the closure.
    let mut closure = BTreeSet::new();
    collect_subformulas(nnf, &mut closure);
    let untils: Vec<&Ltl> = closure
        .iter()
        .filter(|f| matches!(f, Ltl::Until(_, _)))
        .collect();

    let accepting_sets: Vec<Vec<usize>> = if untils.is_empty() {
        vec![(0..num_states).collect()]
    } else {
        untils
            .iter()
            .map(|until| {
                let Ltl::Until(_, b) = until else {
                    return Vec::new();
                };
                let right = (**b).clone();
                (0..num_states)
                    .filter(|&i| {
                        let old = &nodes[i].old;
                        !old.contains(*until) || old.contains(&right)
                    })
                    .collect()
            })
            .collect()
    };

    GeneralizedBuchiAutomaton {
        num_states,
        initial,
        edges,
        labels,
        accepting_sets,
    }
}

/// Returns `true` iff `state` satisfies every literal in `label`.
fn label_satisfied(label: &[Literal], state: &TemporalState) -> bool {
    label
        .iter()
        .all(|(prop, polarity)| state.satisfies(prop) == *polarity)
}

/// A lasso-shaped infinite path: `prefix` followed by `loop_segment` repeated.
///
/// Each entry is the identifier of a state in the original [`TransitionSystem`].
/// The last state of `loop_segment` transitions back to its first state.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LassoTrace {
    /// Finite prefix leading into the loop (may be empty).
    pub prefix: Vec<String>,
    /// The infinitely repeated loop body (always non-empty).
    pub loop_segment: Vec<String>,
}

/// Result of LTL model checking against a transition system.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LtlModelCheckResult {
    /// Whether every infinite path of the system satisfies the formula.
    pub holds: bool,
    /// A violating lasso path, present exactly when `holds` is `false`.
    pub counterexample: Option<LassoTrace>,
}

/// Synchronous product of a Kripke structure with a GBA, degeneralized on the
/// fly so that emptiness reduces to finding a reachable accepting cycle.
struct Product<'a> {
    system: &'a TransitionSystem,
    gba: &'a GeneralizedBuchiAutomaton,
    acc_sets: Vec<HashSet<usize>>,
    k: usize,
    index: HashMap<(String, usize, usize), usize>,
    nodes: Vec<(String, usize, usize)>,
    adj: Vec<Vec<usize>>,
    accepting: Vec<bool>,
}

impl<'a> Product<'a> {
    fn new(system: &'a TransitionSystem, gba: &'a GeneralizedBuchiAutomaton) -> Self {
        let acc_sets: Vec<HashSet<usize>> = gba
            .accepting_sets
            .iter()
            .map(|set| set.iter().copied().collect())
            .collect();
        let acc_sets = if acc_sets.is_empty() {
            vec![(0..gba.num_states).collect()]
        } else {
            acc_sets
        };
        let k = acc_sets.len().max(1);
        Self {
            system,
            gba,
            acc_sets,
            k,
            index: HashMap::new(),
            nodes: Vec::new(),
            adj: Vec::new(),
            accepting: Vec::new(),
        }
    }

    /// Returns the id of a product state, allocating it on first use.
    fn intern(&mut self, key: (String, usize, usize)) -> usize {
        if let Some(&id) = self.index.get(&key) {
            return id;
        }
        let id = self.nodes.len();
        let accepting = key.2 == 0 && self.acc_sets[0].contains(&key.1);
        self.index.insert(key.clone(), id);
        self.nodes.push(key);
        self.adj.push(Vec::new());
        self.accepting.push(accepting);
        id
    }

    /// Builds the reachable product graph, returning the initial product states.
    fn build(&mut self) -> Vec<usize> {
        let mut initials = Vec::new();
        for s in &self.system.initial_states {
            let Some(state) = self.system.states.get(s) else {
                continue;
            };
            for &q in &self.gba.initial {
                if label_satisfied(&self.gba.labels[q], state) {
                    let id = self.intern((s.clone(), q, 0));
                    initials.push(id);
                }
            }
        }
        initials.sort_unstable();
        initials.dedup();

        let mut work: VecDeque<usize> = initials.iter().copied().collect();
        let mut expanded: HashSet<usize> = HashSet::new();
        while let Some(id) = work.pop_front() {
            if !expanded.insert(id) {
                continue;
            }
            let (s, q, j) = self.nodes[id].clone();

            // Gather successors using only shared borrows, then mutate.
            let succ_kripke: Vec<String> =
                self.system.transitions.get(&s).cloned().unwrap_or_default();
            let q_succs: Vec<usize> = self.gba.edges.get(q).cloned().unwrap_or_default();
            let advance = self.acc_sets[j].contains(&q);
            let jp = if advance { (j + 1) % self.k } else { j };

            let mut targets: Vec<(String, usize, usize)> = Vec::new();
            for sp in &succ_kripke {
                let Some(sp_state) = self.system.states.get(sp) else {
                    continue;
                };
                for &qp in &q_succs {
                    if label_satisfied(&self.gba.labels[qp], sp_state) {
                        targets.push((sp.clone(), qp, jp));
                    }
                }
            }

            for t in targets {
                let tid = self.intern(t);
                self.adj[id].push(tid);
                work.push_back(tid);
            }
        }
        initials
    }
}

/// Outer pass of the nested DFS.
fn dfs_outer(
    u: usize,
    adj: &[Vec<usize>],
    accepting: &[bool],
    visited: &mut [bool],
    flagged: &mut [bool],
) -> Option<usize> {
    visited[u] = true;
    for &v in &adj[u] {
        if !visited[v]
            && let Some(seed) = dfs_outer(v, adj, accepting, visited, flagged)
        {
            return Some(seed);
        }
    }
    if accepting[u] && dfs_inner(u, u, adj, flagged) {
        return Some(u);
    }
    None
}

/// Inner pass of the nested DFS: looks for an edge back to `seed`.
fn dfs_inner(u: usize, seed: usize, adj: &[Vec<usize>], flagged: &mut [bool]) -> bool {
    flagged[u] = true;
    for &v in &adj[u] {
        if v == seed {
            return true;
        }
        if !flagged[v] && dfs_inner(v, seed, adj, flagged) {
            return true;
        }
    }
    false
}

/// Finds a reachable accepting state lying on a cycle, if one exists.
fn find_accepting_seed(
    adj: &[Vec<usize>],
    accepting: &[bool],
    initials: &[usize],
) -> Option<usize> {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut flagged = vec![false; n];
    for &init in initials {
        if !visited[init]
            && let Some(seed) = dfs_outer(init, adj, accepting, &mut visited, &mut flagged)
        {
            return Some(seed);
        }
    }
    None
}

/// Reconstructs the predecessor chain ending at `goal` from a BFS parent map.
fn reconstruct(parent: &HashMap<usize, usize>, goal: usize) -> Vec<usize> {
    let mut path = vec![goal];
    let mut cur = goal;
    while let Some(&p) = parent.get(&cur) {
        if p == cur {
            break;
        }
        cur = p;
        path.push(cur);
    }
    path.reverse();
    path
}

/// Shortest path (in product states) from any of `starts` to `goal`.
fn path_to(adj: &[Vec<usize>], starts: &[usize], goal: usize) -> Option<Vec<usize>> {
    let mut parent: HashMap<usize, usize> = HashMap::new();
    let mut queue = VecDeque::new();
    for &s in starts {
        if parent.insert(s, s).is_none() {
            queue.push_back(s);
        }
    }
    if parent.contains_key(&goal) {
        return Some(vec![goal]);
    }
    while let Some(u) = queue.pop_front() {
        for &v in &adj[u] {
            if let Entry::Vacant(slot) = parent.entry(v) {
                slot.insert(u);
                if v == goal {
                    return Some(reconstruct(&parent, goal));
                }
                queue.push_back(v);
            }
        }
    }
    None
}

/// Finds a cycle (length >= 1) returning to `seed`, as a list starting at `seed`.
fn cycle_through(adj: &[Vec<usize>], seed: usize) -> Option<Vec<usize>> {
    let mut parent: HashMap<usize, usize> = HashMap::new();
    let mut queue = VecDeque::new();
    for &v in &adj[seed] {
        if v == seed {
            return Some(vec![seed]);
        }
        if parent.insert(v, seed).is_none() {
            queue.push_back(v);
        }
    }
    while let Some(u) = queue.pop_front() {
        for &v in &adj[u] {
            if v == seed {
                let mut path = vec![u];
                let mut cur = u;
                while let Some(&p) = parent.get(&cur) {
                    if p == seed {
                        break;
                    }
                    cur = p;
                    path.push(cur);
                }
                path.push(seed);
                path.reverse();
                return Some(path);
            }
            if let Entry::Vacant(slot) = parent.entry(v) {
                slot.insert(u);
                queue.push_back(v);
            }
        }
    }
    None
}

/// Full LTL model checking: does every infinite path of `system` satisfy
/// `formula`?
///
/// Builds the Büchi automaton of `¬formula`, products it with the Kripke
/// structure and checks emptiness via nested DFS. When the property fails the
/// result carries a concrete lasso counterexample.
pub fn check_ltl(system: &TransitionSystem, formula: &LtlFormula) -> LtlModelCheckResult {
    let negated = to_nnf(formula, true);
    let gba = build_gba(&negated);

    let mut product = Product::new(system, &gba);
    let initials = product.build();

    match find_accepting_seed(&product.adj, &product.accepting, &initials) {
        None => LtlModelCheckResult {
            holds: true,
            counterexample: None,
        },
        Some(seed) => {
            let to_kripke = |ids: &[usize]| -> Vec<String> {
                ids.iter().map(|&id| product.nodes[id].0.clone()).collect()
            };
            let prefix_ids = path_to(&product.adj, &initials, seed).unwrap_or_default();
            let loop_ids = cycle_through(&product.adj, seed).unwrap_or_else(|| vec![seed]);

            // Drop the trailing `seed` from the prefix; it begins the loop.
            let prefix_ids = if prefix_ids.is_empty() {
                Vec::new()
            } else {
                prefix_ids[..prefix_ids.len() - 1].to_vec()
            };

            LtlModelCheckResult {
                holds: false,
                counterexample: Some(LassoTrace {
                    prefix: to_kripke(&prefix_ids),
                    loop_segment: to_kripke(&loop_ids),
                }),
            }
        }
    }
}
