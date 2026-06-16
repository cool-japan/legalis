//! The Hindley–Milner inference engine (Algorithm W).
//!
//! [`InferenceEngine`] threads a mutable substitution, a fresh-variable supply,
//! a [`ClassEnv`] and a [`DataEnv`] through inference of the typed [`Term`] IR.
//! It implements:
//!
//! * unification with the occurs-check (delegated to [`crate::typeinfer::unify`]),
//! * `let`-generalization producing [`TypeScheme`]s,
//! * instantiation of schemes with fresh variables,
//! * collection and context-reduction of type-class predicates, and
//! * exhaustiveness-checked `match` over algebraic data types.

use std::collections::{BTreeMap, HashMap};

use super::adt::DataEnv;
use super::classes::ClassEnv;
use super::error::{InferResult, TypeInferError};
use super::subst::Subst;
use super::term::{MatchArm, Pattern, Term};
use super::types::{FreeVars, MonoType, Pred, QualType, Row, TypeScheme, VarId, VarSupply};
use super::unify;

/// A typing environment mapping names to type schemes.
#[derive(Debug, Clone, Default)]
pub struct TypeEnv {
    bindings: HashMap<String, TypeScheme>,
}

impl TypeEnv {
    /// An empty environment.
    pub fn new() -> Self {
        TypeEnv::default()
    }

    /// Inserts a binding in place.
    pub fn insert(&mut self, name: impl Into<String>, scheme: TypeScheme) {
        self.bindings.insert(name.into(), scheme);
    }

    /// Returns a clone of the environment extended with one binding.
    pub fn extended(&self, name: impl Into<String>, scheme: TypeScheme) -> TypeEnv {
        let mut env = self.clone();
        env.bindings.insert(name.into(), scheme);
        env
    }

    /// Returns a clone of the environment extended with several bindings.
    pub fn extended_many(&self, items: Vec<(String, TypeScheme)>) -> TypeEnv {
        let mut env = self.clone();
        for (name, scheme) in items {
            env.bindings.insert(name, scheme);
        }
        env
    }

    /// Looks up a binding.
    pub fn lookup(&self, name: &str) -> Option<&TypeScheme> {
        self.bindings.get(name)
    }

    /// `true` when the environment has no bindings.
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
}

/// The inference engine.
#[derive(Debug, Clone, Default)]
pub struct InferenceEngine {
    supply: VarSupply,
    subst: Subst,
    classes: ClassEnv,
    data: DataEnv,
    prelude: TypeEnv,
    constraints: Vec<Pred>,
}

impl InferenceEngine {
    /// Creates an engine with empty class and data environments.
    pub fn new() -> Self {
        InferenceEngine::default()
    }

    /// Shared access to the class environment.
    pub fn classes(&self) -> &ClassEnv {
        &self.classes
    }

    /// Mutable access to the class environment (to declare classes/instances).
    pub fn classes_mut(&mut self) -> &mut ClassEnv {
        &mut self.classes
    }

    /// Shared access to the data environment.
    pub fn data(&self) -> &DataEnv {
        &self.data
    }

    /// Mutable access to the data environment (to declare data types).
    pub fn data_mut(&mut self) -> &mut DataEnv {
        &mut self.data
    }

    /// Shared access to the prelude environment (the built-in condition
    /// functions installed by [`InferenceEngine::with_prelude`]).
    pub fn prelude_env(&self) -> &TypeEnv {
        &self.prelude
    }

    /// Mutable access to the prelude environment, to register additional
    /// top-level bindings (e.g. user-defined polymorphic condition functions).
    pub fn prelude_mut(&mut self) -> &mut TypeEnv {
        &mut self.prelude
    }

    /// Returns a fresh type variable.
    pub fn fresh_type(&mut self) -> MonoType {
        self.supply.fresh_type()
    }

    /// Returns a fresh row variable id.
    pub fn fresh_row_var(&mut self) -> VarId {
        self.supply.fresh_row_var()
    }

    /// Applies the current substitution to a type.
    pub fn apply(&self, ty: &MonoType) -> MonoType {
        self.subst.apply_type(ty)
    }

    /// Applies the current substitution to a row.
    pub fn apply_row(&self, row: &Row) -> Row {
        self.subst.apply_row(row)
    }

    /// Resets the per-inference state (substitution and pending constraints).
    /// The fresh-variable supply is *not* reset, preserving global uniqueness.
    fn begin(&mut self) {
        self.subst = Subst::new();
        self.constraints.clear();
    }

    /// Unifies two types, folding the result into the ambient substitution.
    pub fn unify(&mut self, lhs: &MonoType, rhs: &MonoType) -> InferResult<()> {
        let a = self.subst.apply_type(lhs);
        let b = self.subst.apply_type(rhs);
        let s = unify::unify(&mut self.supply, &a, &b)?;
        self.subst = s.compose(&self.subst);
        Ok(())
    }

    /// Instantiates a scheme with fresh variables, returning its qualified type;
    /// the instantiated predicates become pending constraints when used during
    /// inference.
    ///
    /// The bound variables are renamed to fresh ids in a single pass; this is a
    /// pure renaming, never a transitively-resolved substitution, so it is
    /// immune to the degenerate `x -> x` binding that would arise when the
    /// fresh-variable supply hands back an id that happens to equal a bound id.
    fn instantiate(&mut self, scheme: &TypeScheme) -> QualType {
        let mut type_map = BTreeMap::new();
        for var in &scheme.type_vars {
            type_map.insert(*var, self.supply.fresh());
        }
        let mut row_map = BTreeMap::new();
        for var in &scheme.row_vars {
            row_map.insert(*var, self.supply.fresh());
        }
        crate::typeinfer::types::rename_qual(&scheme.qual, &type_map, &row_map)
    }

    /// Free variables of the environment under the current substitution.
    fn env_free_vars(&self, env: &TypeEnv) -> FreeVars {
        let mut acc = FreeVars::default();
        for scheme in env.bindings.values() {
            let applied = self.subst.apply_scheme(scheme);
            acc.union_with(&applied.free_vars());
        }
        acc
    }

    /// Generalizes a qualified type with respect to an environment.
    fn generalize(&self, env: &TypeEnv, qual: &QualType) -> TypeScheme {
        let resolved = self.subst.apply_qual(qual);
        let env_ftv = self.env_free_vars(env);
        let body_ftv = resolved.free_vars();
        let type_vars: Vec<VarId> = body_ftv
            .types
            .iter()
            .copied()
            .filter(|v| !env_ftv.contains_type(*v))
            .collect();
        let row_vars: Vec<VarId> = body_ftv
            .rows
            .iter()
            .copied()
            .filter(|v| !env_ftv.contains_row(*v))
            .collect();
        TypeScheme::new(type_vars, row_vars, resolved)
    }

    /// Infers the type of a term, accumulating class constraints internally.
    pub fn infer(&mut self, env: &TypeEnv, term: &Term) -> InferResult<MonoType> {
        match term {
            Term::Lit(lit) => Ok(lit.mono_type()),
            Term::Var(name) => {
                let scheme = env
                    .lookup(name)
                    .ok_or_else(|| TypeInferError::UnboundVariable(name.clone()))?
                    .clone();
                let qual = self.instantiate(&scheme);
                self.constraints.extend(qual.preds);
                Ok(qual.ty)
            }
            Term::Abs(param, body) => {
                let param_ty = self.fresh_type();
                let env2 = env.extended(param.clone(), TypeScheme::mono(param_ty.clone()));
                let body_ty = self.infer(&env2, body)?;
                Ok(MonoType::func(self.apply(&param_ty), body_ty))
            }
            Term::App(func, arg) => {
                let func_ty = self.infer(env, func)?;
                let arg_ty = self.infer(env, arg)?;
                let result = self.fresh_type();
                self.unify(&func_ty, &MonoType::func(arg_ty, result.clone()))?;
                Ok(self.apply(&result))
            }
            Term::Let(name, bound, body) => self.infer_let(env, name, bound, body),
            Term::If(cond, then_branch, else_branch) => {
                let cond_ty = self.infer(env, cond)?;
                self.unify(&cond_ty, &MonoType::boolean())?;
                let then_ty = self.infer(env, then_branch)?;
                let else_ty = self.infer(env, else_branch)?;
                self.unify(&then_ty, &else_ty)?;
                Ok(self.apply(&then_ty))
            }
            Term::Record(fields) => self.infer_record(env, fields),
            Term::RecordExtend(label, value, rest) => {
                let value_ty = self.infer(env, value)?;
                let rest_ty = self.infer(env, rest)?;
                let tail = self.fresh_row_var();
                self.unify(&rest_ty, &MonoType::record(Row::open(tail)))?;
                let row = Row::open(tail).with(label.clone(), self.apply(&value_ty));
                Ok(self.apply(&MonoType::record(row)))
            }
            Term::RecordSelect(label, record) => {
                let record_ty = self.infer(env, record)?;
                let field_ty = self.fresh_type();
                let tail = self.fresh_row_var();
                let expected =
                    MonoType::record(Row::open(tail).with(label.clone(), field_ty.clone()));
                self.unify(&record_ty, &expected)?;
                Ok(self.apply(&field_ty))
            }
            Term::RecordRestrict(label, record) => {
                let record_ty = self.infer(env, record)?;
                let field_ty = self.fresh_type();
                let tail = self.fresh_row_var();
                let expected = MonoType::record(Row::open(tail).with(label.clone(), field_ty));
                self.unify(&record_ty, &expected)?;
                Ok(self.apply(&MonoType::record(Row::open(tail))))
            }
            Term::Construct(name, args) => self.infer_construct(env, name, args),
            Term::Match(scrutinee, arms) => self.infer_match(env, scrutinee, arms),
        }
    }

    /// Inference for `let` with let-generalization.
    fn infer_let(
        &mut self,
        env: &TypeEnv,
        name: &str,
        bound: &Term,
        body: &Term,
    ) -> InferResult<MonoType> {
        let mark = self.constraints.len();
        let bound_ty = self.infer(env, bound)?;
        let bound_preds: Vec<Pred> = self.constraints.split_off(mark);
        let reduced = self.classes.reduce(&self.subst, &bound_preds)?;

        let env_ftv = self.env_free_vars(env);
        let (deferred, retained) = split_predicates(&env_ftv, &self.subst, reduced);
        self.constraints.extend(deferred);

        let scheme = self.generalize(env, &QualType::new(retained, bound_ty));
        let env2 = env.extended(name.to_string(), scheme);
        self.infer(&env2, body)
    }

    /// Inference for a closed record literal.
    fn infer_record(&mut self, env: &TypeEnv, fields: &[(String, Term)]) -> InferResult<MonoType> {
        let mut row_fields = BTreeMap::new();
        for (label, term) in fields {
            if row_fields.contains_key(label) {
                return Err(TypeInferError::DuplicateLabel(label.clone()));
            }
            let ty = self.infer(env, term)?;
            row_fields.insert(label.clone(), self.apply(&ty));
        }
        Ok(self.apply(&MonoType::record(Row::new(row_fields, None))))
    }

    /// Inference for constructor application.
    fn infer_construct(
        &mut self,
        env: &TypeEnv,
        name: &str,
        args: &[Term],
    ) -> InferResult<MonoType> {
        let expected_arity = self
            .data
            .constructor(name)
            .ok_or_else(|| TypeInferError::UnknownConstructor(name.to_string()))?
            .arity();
        if expected_arity != args.len() {
            return Err(TypeInferError::ConstructorArity {
                ctor: name.to_string(),
                expected: expected_arity,
                found: args.len(),
            });
        }
        let scheme = self.data.constructor_scheme(name)?;
        let qual = self.instantiate(&scheme);
        self.constraints.extend(qual.preds);

        let mut current = qual.ty;
        for arg in args {
            let arg_ty = self.infer(env, arg)?;
            let result = self.fresh_type();
            self.unify(&current, &MonoType::func(arg_ty, result.clone()))?;
            current = self.apply(&result);
        }
        Ok(current)
    }

    /// Inference for `match`, including an exhaustiveness check.
    fn infer_match(
        &mut self,
        env: &TypeEnv,
        scrutinee: &Term,
        arms: &[MatchArm],
    ) -> InferResult<MonoType> {
        let scrut_ty = self.infer(env, scrutinee)?;
        let result = self.fresh_type();
        let mut head_patterns = Vec::with_capacity(arms.len());

        for arm in arms {
            let (bindings, pat_ty) = self.infer_pattern(&arm.pattern)?;
            self.unify(&pat_ty, &scrut_ty)?;
            let scheme_bindings = bindings
                .into_iter()
                .map(|(name, ty)| (name, TypeScheme::mono(ty)))
                .collect::<Vec<_>>();
            let env2 = env.extended_many(scheme_bindings);
            let body_ty = self.infer(&env2, &arm.body)?;
            self.unify(&body_ty, &result)?;
            head_patterns.push(arm.pattern.clone());
        }

        let resolved_scrut = self.apply(&scrut_ty);
        match resolved_scrut.head_con() {
            Some(data_name) => {
                let missing = self.data.missing_constructors(data_name, &head_patterns);
                if !missing.is_empty() {
                    return Err(TypeInferError::NonExhaustiveMatch { missing });
                }
            }
            None => {
                if !head_patterns.iter().any(Pattern::is_irrefutable) {
                    return Err(TypeInferError::NonExhaustiveMatch {
                        missing: vec!["_".to_string()],
                    });
                }
            }
        }

        Ok(self.apply(&result))
    }

    /// Infers a pattern, returning the variables it binds and its type.
    fn infer_pattern(
        &mut self,
        pattern: &Pattern,
    ) -> InferResult<(Vec<(String, MonoType)>, MonoType)> {
        match pattern {
            Pattern::Wildcard => Ok((Vec::new(), self.fresh_type())),
            Pattern::Var(name) => {
                let ty = self.fresh_type();
                Ok((vec![(name.clone(), ty.clone())], ty))
            }
            Pattern::Lit(lit) => Ok((Vec::new(), lit.mono_type())),
            Pattern::Constructor { name, args } => {
                let expected_arity = self
                    .data
                    .constructor(name)
                    .ok_or_else(|| TypeInferError::UnknownConstructor(name.to_string()))?
                    .arity();
                if expected_arity != args.len() {
                    return Err(TypeInferError::ConstructorArity {
                        ctor: name.to_string(),
                        expected: expected_arity,
                        found: args.len(),
                    });
                }
                let scheme = self.data.constructor_scheme(name)?;
                let qual = self.instantiate(&scheme);
                self.constraints.extend(qual.preds);

                let mut current = qual.ty;
                let mut bindings = Vec::new();
                for sub in args {
                    let field_ty = self.fresh_type();
                    let rest_ty = self.fresh_type();
                    self.unify(&current, &MonoType::func(field_ty.clone(), rest_ty.clone()))?;
                    let (sub_bindings, sub_ty) = self.infer_pattern(sub)?;
                    self.unify(&sub_ty, &field_ty)?;
                    bindings.extend(sub_bindings);
                    current = self.apply(&rest_ty);
                }
                Ok((bindings, self.apply(&current)))
            }
        }
    }

    /// Top-level inference of a closed term: infers, reduces the residual
    /// constraints, checks for ambiguity, and generalizes into a [`TypeScheme`]
    /// (returned in normalized form for stable display/comparison).
    pub fn infer_scheme(&mut self, env: &TypeEnv, term: &Term) -> InferResult<TypeScheme> {
        self.begin();
        let ty = self.infer(env, term)?;
        let resolved = self.apply(&ty);
        let pending = std::mem::take(&mut self.constraints);
        let reduced = self.classes.reduce(&self.subst, &pending)?;

        let body_ftv = resolved.free_vars();
        for pred in &reduced {
            let pred_vars = pred.free_vars();
            if !pred_vars.types.iter().any(|v| body_ftv.contains_type(*v)) {
                return Err(TypeInferError::AmbiguousConstraint {
                    predicate: pred.to_string(),
                });
            }
        }

        let scheme = self.generalize(env, &QualType::new(reduced, resolved));
        Ok(scheme.normalized())
    }

    /// Infers the type of a term under `env`, returning the resolved monotype
    /// and the reduced residual predicates without generalizing. Useful for
    /// callers (such as condition inference) that own a shared environment.
    pub fn infer_qualified(&mut self, env: &TypeEnv, term: &Term) -> InferResult<QualType> {
        let ty = self.infer(env, term)?;
        let resolved = self.apply(&ty);
        let pending = std::mem::take(&mut self.constraints);
        let reduced = self.classes.reduce(&self.subst, &pending)?;
        Ok(QualType::new(reduced, resolved))
    }

    /// Begins a fresh inference session (clears the substitution and pending
    /// constraints). Exposed for multi-step callers such as statute inference.
    pub fn reset(&mut self) {
        self.begin();
    }

    /// Drains the pending constraints and reduces them through context
    /// reduction, returning the residual predicates.
    pub(crate) fn take_reduced_constraints(&mut self) -> InferResult<Vec<Pred>> {
        let pending = std::mem::take(&mut self.constraints);
        self.classes.reduce(&self.subst, &pending)
    }
}

/// Splits reduced predicates into those deferred to an outer scope (all their
/// type variables are fixed by the environment) and those retained for
/// generalization here.
fn split_predicates(env_ftv: &FreeVars, subst: &Subst, preds: Vec<Pred>) -> (Vec<Pred>, Vec<Pred>) {
    let mut deferred = Vec::new();
    let mut retained = Vec::new();
    for pred in preds {
        let applied = subst.apply_pred(&pred);
        let vars = applied.free_vars();
        let all_fixed = vars.types.iter().all(|v| env_ftv.contains_type(*v))
            && vars.rows.iter().all(|v| env_ftv.contains_row(*v));
        if all_fixed && !vars.is_empty() {
            deferred.push(applied);
        } else {
            retained.push(applied);
        }
    }
    (deferred, retained)
}
