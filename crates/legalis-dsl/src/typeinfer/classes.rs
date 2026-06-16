//! Type classes for condition behaviours: a class environment, instance
//! resolution, context reduction, and dictionary (evidence) construction.
//!
//! Classes model the behaviours a condition can demand of a value — equality
//! (`Eq`), ordering (`Ord`), arithmetic (`Numeric`), pattern matchability
//! (`Matchable`). Constraints collected during inference are discharged by
//! [`ClassEnv::reduce`] (THIH-style context reduction) and, where a concrete
//! instance is required, witnessed by an [`Evidence`] derivation that makes the
//! dictionary-passing structure explicit.

use std::collections::HashMap;

use super::error::{InferResult, TypeInferError};
use super::subst::Subst;
use super::types::{MonoType, Pred};

/// An instance declaration `context => head` (e.g. `Eq a => Eq (List a)`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    /// Instance context (constraints on the instance's variables).
    pub context: Vec<Pred>,
    /// The instance head (e.g. `Eq (List a)`).
    pub head: Pred,
}

impl Instance {
    /// A ground instance with no context (e.g. `Ord Int`).
    pub fn ground(class: impl Into<String>, ty: MonoType) -> Self {
        Instance {
            context: Vec::new(),
            head: Pred::new(class, ty),
        }
    }

    /// An instance with a context.
    pub fn new(context: Vec<Pred>, head: Pred) -> Self {
        Instance { context, head }
    }
}

/// Information about a declared class: its superclasses.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClassInfo {
    /// Names of superclasses (e.g. `Eq` is a superclass of `Ord`).
    pub superclasses: Vec<String>,
}

/// An evidence (dictionary) derivation witnessing why a predicate holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Evidence {
    /// Discharged by an assumption already in scope.
    Given(Pred),
    /// Discharged via a superclass of a broader assumption in scope.
    Superclass {
        /// The assumption used.
        from: Pred,
        /// The (sub)class obtained from it.
        derived: Pred,
    },
    /// Discharged by an instance, with sub-evidence for the instance context.
    Instance {
        /// The instance head that fired.
        head: Pred,
        /// Evidence for each context predicate of the instance.
        args: Vec<Evidence>,
    },
}

impl Evidence {
    /// The predicate this evidence proves.
    pub fn predicate(&self) -> &Pred {
        match self {
            Evidence::Given(pred) | Evidence::Superclass { derived: pred, .. } => pred,
            Evidence::Instance { head, .. } => head,
        }
    }
}

/// The class environment: declared classes and their instances.
#[derive(Debug, Clone, Default)]
pub struct ClassEnv {
    classes: HashMap<String, ClassInfo>,
    instances: HashMap<String, Vec<Instance>>,
}

impl ClassEnv {
    /// An empty class environment.
    pub fn new() -> Self {
        ClassEnv::default()
    }

    /// Declares a class with the given superclasses.
    pub fn declare_class(&mut self, name: impl Into<String>, superclasses: Vec<String>) {
        self.classes.insert(name.into(), ClassInfo { superclasses });
    }

    /// Registers an instance.
    pub fn add_instance(&mut self, instance: Instance) {
        self.instances
            .entry(instance.head.class.clone())
            .or_default()
            .push(instance);
    }

    /// `true` when the class is declared.
    pub fn has_class(&self, name: &str) -> bool {
        self.classes.contains_key(name)
    }

    /// Direct superclasses of a class.
    pub fn superclasses(&self, name: &str) -> &[String] {
        self.classes
            .get(name)
            .map(|info| info.superclasses.as_slice())
            .unwrap_or(&[])
    }

    /// All predicates entailed by `pred` through the superclass hierarchy,
    /// including `pred` itself.
    pub fn by_superclass(&self, pred: &Pred) -> Vec<Pred> {
        let mut out = vec![pred.clone()];
        for sup in self.superclasses(&pred.class) {
            let sup_pred = Pred::new(sup.clone(), pred.ty.clone());
            out.extend(self.by_superclass(&sup_pred));
        }
        out
    }

    /// Attempts to discharge `goal` by a matching instance, returning the
    /// instance's (substituted) context predicates as new subgoals.
    pub fn by_instance(&self, goal: &Pred) -> Option<Vec<Pred>> {
        for instance in self.instances.get(&goal.class)? {
            if let Some(subst) = match_pred(&instance.head, goal) {
                return Some(
                    instance
                        .context
                        .iter()
                        .map(|p| subst.apply_pred(p))
                        .collect(),
                );
            }
        }
        None
    }

    /// `true` when `goal` is entailed by the assumptions `given`.
    pub fn entails(&self, given: &[Pred], goal: &Pred) -> bool {
        // By assumption (directly or via superclasses).
        if given.iter().any(|g| self.by_superclass(g).contains(goal)) {
            return true;
        }
        // By instance, with every subgoal entailed in turn.
        match self.by_instance(goal) {
            Some(subgoals) => subgoals.iter().all(|sub| self.entails(given, sub)),
            None => false,
        }
    }

    /// Builds an [`Evidence`] derivation that `goal` holds under `given`, if it
    /// does. This is the dictionary that dictionary-passing would thread.
    pub fn resolve(&self, given: &[Pred], goal: &Pred) -> Option<Evidence> {
        // Directly assumed.
        if given.contains(goal) {
            return Some(Evidence::Given(goal.clone()));
        }
        // Derivable from a superclass of an assumption.
        for g in given {
            if g != goal && self.by_superclass(g).contains(goal) {
                return Some(Evidence::Superclass {
                    from: g.clone(),
                    derived: goal.clone(),
                });
            }
        }
        // Derivable by instance.
        let subgoals = self.by_instance(goal)?;
        let mut args = Vec::with_capacity(subgoals.len());
        for sub in subgoals {
            args.push(self.resolve(given, &sub)?);
        }
        Some(Evidence::Instance {
            head: goal.clone(),
            args,
        })
    }

    /// Reduces a predicate to head normal form, resolving non-HNF predicates by
    /// instance. Fails with [`TypeInferError::NoInstance`] when a concrete
    /// predicate has no instance.
    pub fn to_hnf(&self, pred: &Pred) -> InferResult<Vec<Pred>> {
        if !self.has_class(&pred.class) {
            return Err(TypeInferError::UnknownClass(pred.class.clone()));
        }
        if pred.is_hnf() {
            return Ok(vec![pred.clone()]);
        }
        match self.by_instance(pred) {
            Some(subgoals) => self.to_hnf_all(&subgoals),
            None => Err(TypeInferError::NoInstance {
                predicate: pred.to_string(),
            }),
        }
    }

    /// Reduces several predicates to head normal form.
    fn to_hnf_all(&self, preds: &[Pred]) -> InferResult<Vec<Pred>> {
        let mut out = Vec::new();
        for pred in preds {
            out.extend(self.to_hnf(pred)?);
        }
        Ok(out)
    }

    /// Performs context reduction: applies `subst`, reduces every predicate to
    /// HNF, then drops predicates that are entailed by the others.
    pub fn reduce(&self, subst: &Subst, preds: &[Pred]) -> InferResult<Vec<Pred>> {
        let applied: Vec<Pred> = preds.iter().map(|p| subst.apply_pred(p)).collect();
        let hnf = self.to_hnf_all(&applied)?;
        Ok(self.simplify(&hnf))
    }

    /// Removes predicates entailed by the remaining ones (and removes exact
    /// duplicates), yielding a minimal residual context.
    fn simplify(&self, preds: &[Pred]) -> Vec<Pred> {
        let mut kept: Vec<Pred> = Vec::new();
        for (i, pred) in preds.iter().enumerate() {
            // The context for entailment is every *other* predicate.
            let rest: Vec<Pred> = preds
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, p)| p.clone())
                .chain(kept.iter().cloned())
                .collect();
            if !kept.contains(pred) && !self.entails(&rest, pred) {
                kept.push(pred.clone());
            }
        }
        kept
    }
}

/// One-way matching of an instance head against a goal: finds a substitution of
/// the head's variables (only) that makes it equal to `goal`. Unlike
/// unification, `goal` is treated rigidly.
pub fn match_pred(head: &Pred, goal: &Pred) -> Option<Subst> {
    if head.class != goal.class {
        return None;
    }
    let mut subst = Subst::new();
    if match_type(&head.ty, &goal.ty, &mut subst) {
        Some(subst)
    } else {
        None
    }
}

/// One-way matching of `pat` against `target`, accumulating bindings of `pat`'s
/// variables in `subst`.
fn match_type(pat: &MonoType, target: &MonoType, subst: &mut Subst) -> bool {
    match (pat, target) {
        (MonoType::Var(v), _) => match subst.lookup_type(*v) {
            Some(bound) => bound == target,
            None => {
                subst.insert_type(*v, target.clone());
                true
            }
        },
        (MonoType::Con { name: n1, args: a1 }, MonoType::Con { name: n2, args: a2 }) => {
            n1 == n2 && a1.len() == a2.len() && match_all(a1, a2, subst)
        }
        (MonoType::Fun(f1, t1), MonoType::Fun(f2, t2)) => {
            match_type(f1, f2, subst) && match_type(t1, t2, subst)
        }
        (MonoType::Record(r1), MonoType::Record(r2)) => match_row(r1, r2, subst),
        _ => false,
    }
}

/// Matches argument lists pointwise.
fn match_all(pats: &[MonoType], targets: &[MonoType], subst: &mut Subst) -> bool {
    pats.iter()
        .zip(targets.iter())
        .all(|(p, t)| match_type(p, t, subst))
}

/// Matches record rows (closed rows with identical labels).
fn match_row(pat: &super::types::Row, target: &super::types::Row, subst: &mut Subst) -> bool {
    if pat.tail.is_some() || target.tail.is_some() {
        // Conservative: only match closed rows in instance heads.
        return false;
    }
    if pat.fields.len() != target.fields.len() {
        return false;
    }
    for (label, pty) in &pat.fields {
        match target.fields.get(label) {
            Some(tty) if match_type(pty, tty, subst) => {}
            _ => return false,
        }
    }
    true
}
